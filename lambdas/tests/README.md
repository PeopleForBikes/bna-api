# Integration tests

The integration tests use [Hurl] and [Schemathesis].

## Authentication

Write requests send `Authorization: Bearer {{token}}`. Locally, authentication
is disabled and `localhost.vars` sets an empty token. Against staging, provide a
token through the environment; it is never committed:

```bash
HURL_VARIABLE_token=<token> just test staging
```

Without it, Hurl stops with an "Undefined variable" error.

## Coverage

`just test` writes a JSON report of the run in `reports/`. `just coverage`
compares it with `openapi-3.1.yaml`:

```bash
just test localhost
just coverage
```

Every operation documents the status codes it can return. The check matches each
request of the run to its operation and fails when:

- a documented status code of an operation was never observed (uncovered);
- an observed status code is not documented for its operation, including any
  `500` (undocumented);
- a request matches no operation (unknown path).

A new endpoint or status code therefore needs a Hurl request before the check
passes again.

The check is the `hurl-coverage` binary in `coverage/`, a member of the Cargo
workspace; `just coverage` runs it through `cargo run`. Its options set other
paths for the specification or the report:

```bash
cargo run -q -p hurl-coverage -- --spec ../../openapi-3.1.yaml --report reports/report.json
```

## Schemathesis

[Schemathesis] generates requests from `openapi-3.1.yaml` and checks every
response against it. It needs [uv] on your `PATH`, nothing else: the recipe runs
a pinned Schemathesis release through `uvx`, which downloads it with its
dependencies, and Python if needed. Run it against the local server loaded with
the test data, as for the Hurl suites:

```bash
just db-seed           # at the root of the repository
just debug-axum        # at the root, in another terminal
just test-schemathesis # here, in lambdas/tests
```

The run tests every operation, `POST` and `PATCH` included, with every check
Schemathesis provides (server errors, status codes, content types, headers,
response schemas, valid data accepted, invalid data rejected, and more), with
valid and invalid inputs, in four phases: the examples of the specification,
boundary values of every constraint, random inputs, and chains of calls that
pass an identifier from one response to the next request. It reports every
failure, with a `curl` command to reproduce it, and exits non-zero on any.

Its settings are in `schemathesis.toml`, including the few checks excluded for
some operations, each with the reason it is a false positive. The pinned release
and the date that fixes its dependencies are in `justfile`; they change
together.

A run writes its reports in `schemathesis-report/` (a JUnit file and an HTML
page, `html-<timestamp>/index.html`) and its cache and crash files in
`.schemathesis/`; both are git-ignored. To replay a failure by the ID the run
printed, use the `uvx` command of the recipe (`just --dry-run test-schemathesis`
prints it) with `replay <ID>` in place of `run …`. A passing run deletes the
crash files of the failures it no longer finds.

Schemathesis writes random rows into the database. Run `just db-seed` again
before the Hurl suites, which expect the test data as loaded.

In the nono sandbox, an agent needs PyPI and a writable uv tool directory to run
the recipe: `--allow-domain pypi.org --allow-domain files.pythonhosted.org` and
`--allow ~/.local/share/uv/tools`, on top of the `nono-claude-danger` recipe.

## Continuous integration and releases

On every pull request and push to `main`, the `integration` workflow
(`.github/workflows/integration.yml`) loads `database-with-test-data.sql` in a
fresh Postgres, starts the API with authentication disabled, then runs the smoke
tests, `just test localhost`, `just coverage`, and `just test-schemathesis`. The
coverage output is shown on the job summary page, the Hurl report is uploaded as
the `integration-reports` artifact, and the Schemathesis reports as the
`schemathesis-reports` artifact. Schemathesis runs last, because it writes
random rows, and also runs when an earlier step failed.

Releases are approved manually: pushing the version tag deploys production.
Before pushing it, the person releasing confirms that:

1. the `integration` workflow passed on the commit being tagged, including the
   coverage check (see its job summary);
2. the manual verifications for the release are done, such as the checks against
   staging (`HURL_VARIABLE_token=<token> just test staging`).

## Test data

The tests run against a database loaded from `database-with-test-data.sql`.
Locally, `just db-seed` at the root of the repository loads it, replacing
whatever the database holds, then applies any migration newer than the dump.
This dump is generated, never edited by hand:

```bash
just dump-test-data
```

The recipe recreates a dedicated database (`bna_test_data` by default), then:

1. applies the migrations, which fill the lookup tables (countries, regions, US
   states, statuses, steps, bike lane types) and the first Fargate price;
2. loads `baseline.sql`, the cities and their rating history;
3. loads `fixtures.sql`, the extra rows the read-only tests rely on (prices,
   pipelines, submissions, measures, and a few more cities and ratings);
4. dumps the result into `database-with-test-data.sql`, then drops the database.

Two runs produce identical output. To change the test data, edit `baseline.sql`
or `fixtures.sql` with fixed identifiers and timestamps, run the recipe, and
commit both the SQL file and the regenerated dump.

The recipe connects to `postgresql://postgres:postgres@localhost:5432/postgres`
by default (the local `docker compose` database). Pass another server with
`just dump-test-data <admin_url>`. It requires `sea-orm-cli`, `psql`, and
`pg_dump` 17.6 or later.

[Hurl]: https://hurl.dev
[Schemathesis]: https://github.com/schemathesis/schemathesis
[uv]: https://docs.astral.sh/uv/
