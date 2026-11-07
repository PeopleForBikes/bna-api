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

## Continuous integration and releases

On every pull request and push to `main`, the `integration` workflow
(`.github/workflows/integration.yml`) loads `database-with-test-data.sql` in a
fresh Postgres, starts the API with authentication disabled, then runs the smoke
tests, `just test localhost`, and `just coverage`. The coverage output is shown
on the job summary page, and the Hurl report is uploaded as the
`integration-reports` artifact.

Releases are approved manually: pushing the version tag deploys production.
Before pushing it, the person releasing confirms that:

1. the `integration` workflow passed on the commit being tagged, including the
   coverage check (see its job summary);
2. the manual verifications for the release are done, such as the checks against
   staging (`HURL_VARIABLE_token=<token> just test staging`).

## Test data

The tests run against a database loaded from `database-with-test-data.sql`. This
dump is generated, never edited by hand:

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
