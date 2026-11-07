//! Compares the requests of a Hurl run with the OpenAPI specification.
//!
//! The Hurl JSON report records every request the suite sent and the status the
//! API returned. Each request is matched to a documented operation, and the
//! check reports:
//!
//! - uncovered pairs: documented (operation, status) pairs never observed;
//! - undocumented pairs: observed (operation, status) pairs the specification
//!   does not document, including any 500;
//! - unknown paths: requests matching no documented operation.
//!
//! It exits with 1 when any of these lists is not empty.
//!
//! Usage:
//!
//! ```text
//! cargo run -q -p hurl-coverage -- [--spec PATH] [--report PATH]
//! ```
use serde::Deserialize;
use std::{
    collections::{BTreeMap, BTreeSet},
    env,
    error::Error,
    fs,
    path::{Path, PathBuf},
    process::ExitCode,
};
use url::Url;

const HTTP_METHODS: [&str; 8] = [
    "get", "put", "post", "delete", "options", "head", "patch", "trace",
];

const USAGE: &str = "\
Compares the requests of a Hurl run with the OpenAPI specification.

Usage: hurl-coverage [--spec PATH] [--report PATH]

Options:
  --spec PATH    OpenAPI specification (default: openapi-3.1.yaml at the root)
  --report PATH  Hurl JSON report (default: lambdas/tests/reports/report.json)
  -h, --help     Print this help";

/// A documented operation: its method and its path template.
type Operation = (String, String);

/// A documented or observed (operation, status code) pair.
type Pair = (Operation, String);

/// A request of the Hurl run.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Call {
    method: String,
    path: String,
    status: String,
    /// Hurl file and line of the request.
    location: String,
}

/// The paths of an OpenAPI specification, the only part the check reads.
#[derive(Deserialize)]
struct Spec {
    #[serde(default)]
    paths: BTreeMap<String, BTreeMap<String, serde_norway::Value>>,
}

/// A Hurl JSON report: one item per Hurl file.
#[derive(Deserialize)]
struct HurlFile {
    filename: String,
    entries: Vec<HurlEntry>,
}

#[derive(Deserialize)]
struct HurlEntry {
    line: u64,
    calls: Vec<HurlCall>,
}

#[derive(Deserialize)]
struct HurlCall {
    request: HurlRequest,
    response: HurlResponse,
}

#[derive(Deserialize)]
struct HurlRequest {
    method: String,
    url: String,
}

#[derive(Deserialize)]
struct HurlResponse {
    status: u16,
}

/// The result of the check.
#[derive(Debug, Default, PartialEq)]
struct Check {
    documented: BTreeSet<Pair>,
    uncovered: Vec<Pair>,
    /// Each undocumented pair, with the location it was first seen at.
    undocumented: Vec<(Pair, String)>,
    unknown: Vec<Call>,
}

impl Check {
    fn passed(&self) -> bool {
        self.uncovered.is_empty() && self.undocumented.is_empty() && self.unknown.is_empty()
    }
}

/// Returns the documented status codes of each (method, path template).
fn load_operations(spec: &str) -> Result<BTreeMap<Operation, BTreeSet<String>>, Box<dyn Error>> {
    let spec: Spec = serde_norway::from_str(spec)?;
    let mut operations = BTreeMap::new();
    for (path, item) in spec.paths {
        for (method, operation) in item {
            if !HTTP_METHODS.contains(&method.as_str()) {
                continue;
            }
            let codes = match operation.get("responses") {
                Some(serde_norway::Value::Mapping(responses)) => responses
                    .keys()
                    .filter_map(|code| match code {
                        serde_norway::Value::String(code) => Some(code.clone()),
                        serde_norway::Value::Number(code) => Some(code.to_string()),
                        _ => None,
                    })
                    .collect(),
                _ => BTreeSet::new(),
            };
            operations.insert((method.to_uppercase(), path.clone()), codes);
        }
    }
    Ok(operations)
}

/// Returns every call of a Hurl JSON report.
fn load_calls(report: &str) -> Result<Vec<Call>, Box<dyn Error>> {
    let report: Vec<HurlFile> = serde_json::from_str(report)?;
    let mut calls = Vec::new();
    for hurl_file in report {
        for entry in hurl_file.entries {
            for call in entry.calls {
                calls.push(Call {
                    method: call.request.method,
                    path: Url::parse(&call.request.url)?.path().to_string(),
                    status: call.response.status.to_string(),
                    location: format!("{}:{}", hurl_file.filename, entry.line),
                });
            }
        }
    }
    Ok(calls)
}

/// Splits a path into its decoded segments.
fn segments(path: &str) -> Vec<String> {
    path.split('/')
        .filter(|s| !s.is_empty())
        .map(|s| String::from_utf8_lossy(&urlencoding::decode_binary(s.as_bytes())).into_owned())
        .collect()
}

fn is_parameter(segment: &str) -> bool {
    segment.starts_with('{') && segment.ends_with('}')
}

/// Returns the operation matching a request, or `None`.
///
/// A literal segment must match exactly and a parameter matches anything. When
/// several operations match, the one with the most literal segments wins.
fn find_operation<'a>(
    method: &str,
    path: &str,
    operations: impl IntoIterator<Item = &'a Operation>,
) -> Option<&'a Operation> {
    let request = segments(path);
    let mut best: Option<(&Operation, usize)> = None;
    for operation in operations {
        let (operation_method, template) = operation;
        if operation_method != method {
            continue;
        }
        let expected = segments(template);
        if expected.len() != request.len() {
            continue;
        }
        if expected
            .iter()
            .zip(&request)
            .all(|(e, r)| is_parameter(e) || e == r)
        {
            let literals = expected.iter().filter(|e| !is_parameter(e)).count();
            if best.is_none_or(|(_, best_literals)| literals > best_literals) {
                best = Some((operation, literals));
            }
        }
    }
    best.map(|(operation, _)| operation)
}

/// Compares the calls of a run with the documented operations.
fn check(operations: &BTreeMap<Operation, BTreeSet<String>>, calls: &[Call]) -> Check {
    let mut observed: BTreeMap<Pair, String> = BTreeMap::new();
    let mut unknown = Vec::new();
    for call in calls {
        match find_operation(&call.method, &call.path, operations.keys()) {
            Some(operation) => {
                observed
                    .entry((operation.clone(), call.status.clone()))
                    .or_insert_with(|| call.location.clone());
            }
            None => unknown.push(call.clone()),
        }
    }

    let documented: BTreeSet<Pair> = operations
        .iter()
        .flat_map(|(operation, codes)| codes.iter().map(|code| (operation.clone(), code.clone())))
        .collect();
    let uncovered = documented
        .iter()
        .filter(|pair| !observed.contains_key(*pair))
        .cloned()
        .collect();
    let undocumented = observed
        .into_iter()
        .filter(|(pair, _)| !documented.contains(pair))
        .collect();
    Check {
        documented,
        uncovered,
        undocumented,
        unknown,
    }
}

/// Prints the result of the check.
fn print(check: &Check, requests: usize) {
    println!("Uncovered pairs ({}):", check.uncovered.len());
    for ((method, path), status) in &check.uncovered {
        println!("  {method} {path} {status}");
    }
    println!("Undocumented pairs ({}):", check.undocumented.len());
    for (((method, path), status), location) in &check.undocumented {
        println!("  {method} {path} {status} (first seen at {location})");
    }
    println!("Unknown paths ({}):", check.unknown.len());
    for call in &check.unknown {
        println!(
            "  {} {} {} (at {})",
            call.method, call.path, call.status, call.location
        );
    }
    let covered = check.documented.len() - check.uncovered.len();
    println!(
        "Coverage: {covered}/{} documented pairs, {requests} requests",
        check.documented.len()
    );
}

/// Reads a file, naming it in the error.
fn read(path: &Path) -> Result<String, Box<dyn Error>> {
    fs::read_to_string(path).map_err(|e| format!("cannot read {}: {e}", path.display()).into())
}

fn run() -> Result<ExitCode, Box<dyn Error>> {
    // The defaults do not depend on the working directory.
    let tests_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let mut spec = tests_dir.join("../../openapi-3.1.yaml");
    let mut report = tests_dir.join("reports/report.json");

    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        let target: &mut PathBuf = match arg.as_str() {
            "--spec" => &mut spec,
            "--report" => &mut report,
            "-h" | "--help" => {
                println!("{USAGE}");
                return Ok(ExitCode::SUCCESS);
            }
            _ => return Err(format!("unexpected argument {arg}\n\n{USAGE}").into()),
        };
        *target = args
            .next()
            .ok_or_else(|| format!("{arg} needs a path\n\n{USAGE}"))?
            .into();
    }

    let operations = load_operations(&read(&spec)?)?;
    let calls = load_calls(&read(&report)?)?;
    let result = check(&operations, &calls);
    print(&result, calls.len());
    Ok(if result.passed() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    })
}

fn main() -> ExitCode {
    run().unwrap_or_else(|e| {
        eprintln!("error: {e}");
        ExitCode::FAILURE
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn operation(method: &str, path: &str) -> Operation {
        (method.to_string(), path.to_string())
    }

    fn call(method: &str, path: &str, status: &str) -> Call {
        Call {
            method: method.to_string(),
            path: path.to_string(),
            status: status.to_string(),
            location: format!("test.hurl:{status}"),
        }
    }

    fn templates() -> Vec<Operation> {
        vec![
            operation("GET", "/cities/{country}/{region}/{name}"),
            operation("GET", "/cities/top/{year}/{count}"),
            operation("GET", "/cities/summary/latest"),
            operation("POST", "/cities"),
        ]
    }

    #[test]
    fn test_segments_decoded() {
        assert_eq!(
            segments("/cities/United%20States/Tennessee/Chattanooga"),
            vec!["cities", "United States", "Tennessee", "Chattanooga"]
        );
    }

    #[test]
    fn test_segments_root() {
        assert!(segments("/").is_empty());
    }

    #[test]
    fn test_find_operation_literals_win() {
        let templates = templates();
        assert_eq!(
            find_operation("GET", "/cities/top/2025/5", &templates),
            Some(&operation("GET", "/cities/top/{year}/{count}"))
        );
        assert_eq!(
            find_operation("GET", "/cities/summary/latest", &templates),
            Some(&operation("GET", "/cities/summary/latest"))
        );
    }

    #[test]
    fn test_find_operation_parameters() {
        assert_eq!(
            find_operation("GET", "/cities/United%20States/Texas/Austin", &templates()),
            Some(&operation("GET", "/cities/{country}/{region}/{name}"))
        );
    }

    #[test]
    fn test_find_operation_none() {
        assert_eq!(find_operation("GET", "/cities", &templates()), None);
        assert_eq!(find_operation("DELETE", "/cities", &templates()), None);
    }

    #[test]
    fn test_load_operations() {
        let spec = r#"
paths:
  /cities:
    parameters: []
    get:
      responses:
        '200':
          description: ok
        400:
          description: bad request
    post:
      responses: {}
"#;
        let operations = load_operations(spec).unwrap();
        assert_eq!(operations.len(), 2);
        assert_eq!(
            operations[&operation("GET", "/cities")],
            BTreeSet::from(["200".to_string(), "400".to_string()])
        );
        assert!(operations[&operation("POST", "/cities")].is_empty());
    }

    #[test]
    fn test_load_calls() {
        let report = r#"[{
            "filename": "endpoints/cities.hurl",
            "entries": [{
                "line": 12,
                "calls": [{
                    "request": {"method": "GET", "url": "http://localhost:3000/cities/United%20States/Texas/Austin?page=2"},
                    "response": {"status": 200}
                }]
            }]
        }]"#;
        assert_eq!(
            load_calls(report).unwrap(),
            vec![Call {
                method: "GET".to_string(),
                path: "/cities/United%20States/Texas/Austin".to_string(),
                status: "200".to_string(),
                location: "endpoints/cities.hurl:12".to_string(),
            }]
        );
    }

    #[test]
    fn test_check() {
        let operations = BTreeMap::from([
            (
                operation("GET", "/cities"),
                BTreeSet::from(["200".to_string(), "400".to_string()]),
            ),
            (
                operation("POST", "/cities"),
                BTreeSet::from(["201".to_string()]),
            ),
        ]);
        let calls = [
            call("GET", "/cities", "200"),
            call("GET", "/cities", "500"),
            call("POST", "/cities", "201"),
            call("GET", "/nowhere", "404"),
        ];
        let result = check(&operations, &calls);
        assert_eq!(result.documented.len(), 3);
        assert_eq!(
            result.uncovered,
            vec![(operation("GET", "/cities"), "400".to_string())]
        );
        assert_eq!(
            result.undocumented,
            vec![(
                (operation("GET", "/cities"), "500".to_string()),
                "test.hurl:500".to_string()
            )]
        );
        assert_eq!(result.unknown, vec![call("GET", "/nowhere", "404")]);
        assert!(!result.passed());
    }

    #[test]
    fn test_check_passed() {
        let operations = BTreeMap::from([(
            operation("GET", "/cities"),
            BTreeSet::from(["200".to_string()]),
        )]);
        let result = check(&operations, &[call("GET", "/cities", "200")]);
        assert!(result.passed());
    }
}
