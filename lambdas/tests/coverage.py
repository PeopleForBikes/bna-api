"""Compare the requests of a Hurl run with the OpenAPI specification.

The Hurl JSON report records every request the suite sent and the status the
API returned. Each request is matched to a documented operation, and the check
reports:

- uncovered pairs: documented (operation, status) pairs never observed;
- undocumented pairs: observed (operation, status) pairs the specification
  does not document, including any 500;
- unknown paths: requests matching no documented operation.

It exits with 1 when any of these lists is not empty.

Usage:
    uv run python coverage.py [--spec PATH] [--report PATH]

The matching helpers are tested with `python -m doctest coverage.py`.
"""

import argparse
import json
import sys
from pathlib import Path
from urllib.parse import unquote, urlsplit

import yaml

HTTP_METHODS = {"get", "put", "post", "delete", "options", "head", "patch", "trace"}
TESTS_DIR = Path(__file__).resolve().parent


def load_operations(spec_path):
    """Return the documented status codes of each (method, path template)."""
    spec = yaml.safe_load(Path(spec_path).read_text())
    operations = {}
    for path, item in spec.get("paths", {}).items():
        for method, operation in item.items():
            if method not in HTTP_METHODS:
                continue
            codes = {str(code) for code in operation.get("responses", {})}
            operations[(method.upper(), path)] = codes
    return operations


def load_calls(report_path):
    """Return the (method, path, status) of every call in a Hurl JSON report."""
    report = json.loads(Path(report_path).read_text())
    calls = []
    for hurl_file in report:
        for entry in hurl_file["entries"]:
            for call in entry["calls"]:
                request = call["request"]
                calls.append(
                    (
                        request["method"],
                        urlsplit(request["url"]).path,
                        str(call["response"]["status"]),
                        f"{hurl_file['filename']}:{entry['line']}",
                    )
                )
    return calls


def segments(path):
    """Split a path into its decoded segments.

    >>> segments("/cities/United%20States/Tennessee/Chattanooga")
    ['cities', 'United States', 'Tennessee', 'Chattanooga']
    >>> segments("/")
    []
    """
    return [unquote(s) for s in path.strip("/").split("/") if s]


def is_parameter(segment):
    return segment.startswith("{") and segment.endswith("}")


def match(method, path, templates):
    """Return the template matching a request, or None.

    A literal segment must match exactly and a parameter matches anything. When
    several templates match, the one with the most literal segments wins.

    >>> templates = [
    ...     ("GET", "/cities/{country}/{region}/{name}"),
    ...     ("GET", "/cities/top/{year}/{count}"),
    ...     ("GET", "/cities/summary/latest"),
    ...     ("POST", "/cities"),
    ... ]
    >>> match("GET", "/cities/top/2025/5", templates)
    ('GET', '/cities/top/{year}/{count}')
    >>> match("GET", "/cities/United%20States/Texas/Austin", templates)
    ('GET', '/cities/{country}/{region}/{name}')
    >>> match("GET", "/cities/summary/latest", templates)
    ('GET', '/cities/summary/latest')
    >>> match("GET", "/cities", templates) is None
    True
    """
    request = segments(path)
    best, best_literals = None, -1
    for template in templates:
        template_method, template_path = template
        if template_method != method:
            continue
        expected = segments(template_path)
        if len(expected) != len(request):
            continue
        if all(is_parameter(e) or e == r for e, r in zip(expected, request)):
            literals = sum(not is_parameter(e) for e in expected)
            if literals > best_literals:
                best, best_literals = template, literals
    return best


def check(operations, calls):
    """Return the uncovered pairs, the undocumented pairs, and the unknown paths."""
    observed = {}
    unknown = []
    for method, path, status, location in calls:
        operation = match(method, path, operations)
        if operation is None:
            unknown.append((method, path, status, location))
            continue
        observed.setdefault((operation, status), location)

    documented = {(op, code) for op, codes in operations.items() for code in codes}
    uncovered = sorted(documented - observed.keys())
    undocumented = sorted(
        (pair, location)
        for pair, location in observed.items()
        if pair not in documented
    )
    return documented, uncovered, undocumented, unknown


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument(
        "--spec",
        default=TESTS_DIR.parent.parent / "openapi-3.1.yaml",
        help="OpenAPI specification (default: openapi-3.1.yaml at the root)",
    )
    parser.add_argument(
        "--report",
        default=TESTS_DIR / "reports" / "report.json",
        help="Hurl JSON report (default: reports/report.json)",
    )
    args = parser.parse_args()

    operations = load_operations(args.spec)
    calls = load_calls(args.report)
    documented, uncovered, undocumented, unknown = check(operations, calls)

    print(f"Uncovered pairs ({len(uncovered)}):")
    for (method, path), status in uncovered:
        print(f"  {method} {path} {status}")
    print(f"Undocumented pairs ({len(undocumented)}):")
    for ((method, path), status), location in undocumented:
        print(f"  {method} {path} {status} (first seen at {location})")
    print(f"Unknown paths ({len(unknown)}):")
    for method, path, status, location in unknown:
        print(f"  {method} {path} {status} (at {location})")

    covered = len(documented) - len(uncovered)
    print(
        f"Coverage: {covered}/{len(documented)} documented pairs, "
        f"{len(calls)} requests"
    )
    return 1 if uncovered or undocumented or unknown else 0


if __name__ == "__main__":
    sys.exit(main())
