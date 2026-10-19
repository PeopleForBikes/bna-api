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
