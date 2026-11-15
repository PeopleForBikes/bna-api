//! Checks that every example of the OpenAPI specification has a JSON type that
//! the `type` of its schema allows (`lambdas/specs/0000-api-request-collection`,
//! Requirement 6). Importers copy the examples into requests: a quoted `"true"`
//! for a boolean made the API reject the bodies they built.

use serde_norway::Value;

/// The OpenAPI specification generated from the annotations (`just generate`).
const SPEC: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../openapi-3.1.yaml"));

/// Returns the JSON Schema type of a value.
fn json_type(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(number) if number.is_f64() => "number",
        Value::Number(_) => "integer",
        Value::String(_) => "string",
        Value::Sequence(_) => "array",
        Value::Mapping(_) => "object",
        Value::Tagged(tagged) => json_type(&tagged.value),
    }
}

/// Returns the types a schema allows: its `type`, a single name or a list.
fn allowed_types(schema: &Value) -> Option<Vec<&str>> {
    match schema.get("type")? {
        Value::String(name) => Some(vec![name.as_str()]),
        Value::Sequence(names) => Some(names.iter().filter_map(Value::as_str).collect()),
        _ => None,
    }
}

/// Tells whether an example fits the allowed types; an integer is a number.
fn allows(types: &[&str], example: &Value) -> bool {
    let actual = json_type(example);
    types.contains(&actual) || (actual == "integer" && types.contains(&"number"))
}

/// Describes an example that does not fit the allowed types.
fn mismatch(path: &str, example: &Value, types: &[&str]) -> String {
    let shown = serde_json::to_string(example).unwrap_or_else(|_| format!("{example:?}"));
    format!(
        "{path}: example {shown} is a {}, the type allows {types:?}",
        json_type(example)
    )
}

/// Collects the examples under `value` whose JSON type their schema does not
/// allow. A mapping with a `type` and a list of `examples` is a schema; a
/// mapping with an `example` and a `schema` that has a `type` is a parameter.
/// Schemas without a `type` (a `$ref`, an `allOf`) are not checked.
fn check(value: &Value, path: &str, errors: &mut Vec<String>) {
    match value {
        Value::Mapping(mapping) => {
            if let (Some(types), Some(Value::Sequence(examples))) =
                (allowed_types(value), value.get("examples"))
            {
                for example in examples.iter().filter(|e| !allows(&types, e)) {
                    errors.push(mismatch(path, example, &types));
                }
            }
            if let (Some(example), Some(types)) = (
                value.get("example"),
                value.get("schema").and_then(allowed_types),
            ) {
                if !allows(&types, example) {
                    errors.push(mismatch(path, example, &types));
                }
            }
            for (key, child) in mapping {
                let key = key
                    .as_str()
                    .map_or_else(|| format!("{key:?}"), str::to_owned);
                check(child, &format!("{path}/{key}"), errors);
            }
        }
        Value::Sequence(items) => {
            for (index, item) in items.iter().enumerate() {
                check(item, &format!("{path}/{index}"), errors);
            }
        }
        _ => {}
    }
}

/// Returns the mismatched examples of a YAML document.
fn mismatches(document: &str) -> Vec<String> {
    let document: Value = serde_norway::from_str(document).expect("the document is valid YAML");
    let mut errors = Vec::new();
    check(&document, "", &mut errors);
    errors
}

#[test]
fn examples_have_the_types_of_their_schemas() {
    let errors = mismatches(SPEC);
    assert!(
        errors.is_empty(),
        "{} example(s) of openapi-3.1.yaml do not have the type of their schema:\n{}",
        errors.len(),
        errors.join("\n")
    );
}

#[test]
fn quoted_numbers_and_booleans_are_reported() {
    let document = r#"
components:
  schemas:
    Body:
      type: object
      properties:
        consent:
          type: boolean
          examples:
          - 'true'
        count:
          type: [integer, 'null']
          examples:
          - 3
        price:
          type: number
          examples:
          - 2
        name:
          type: string
          examples:
          - Antwerp
        status:
          $ref: '#/components/schemas/Status'
          examples:
          - Pending
paths:
  /things:
    get:
      parameters:
      - name: page
        in: query
        example: '5'
        schema:
          type: integer
"#;
    let errors = mismatches(document);
    assert_eq!(errors.len(), 2, "{errors:#?}");
    assert!(errors[0].starts_with("/components/schemas/Body/properties/consent: "));
    assert!(errors[1].starts_with("/paths//things/get/parameters/0: "));
}
