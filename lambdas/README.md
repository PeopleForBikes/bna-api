# Lambdas

Repository containing all the lambda endpoints for API Gateway.

## Requirements

- [Bacon]
- [Docker]
- [Hurl]
- [Just]
- [PostgreSQL] client, `psql` 17.6 or later
- [Rust]
- [Sea-ORM-CLI]

### Remark

Bacon, Hurl, Just and Sea-ORM-CLI are pinned in `mise.toml` at the root of the
repository: install them with `mise install`, as described in the
[Local tools](../.github/CONTRIBUTING.md#local-tools) section of the
contributing guide. Docker, the PostgreSQL client and Rust are installed
separately.

## Quickstart

For this part, the Docker daemon must be running and 2 terminals will be needed.

### Terminal 1

The first terminal is used to watch the API server output in debug mode.

- Start Docker Compose

  ```bash
  just compose-up
  ```

- Export the database URL

  ```bash
  export DATABASE_URL=postgresql://postgres:postgres@localhost:5432/postgres
  ```

- Initialize the database with the test data, the same data the CI tests run
  against. It replaces whatever the database holds, so run it again at any time
  to start over.

  ```bash
  just db-init
  ```

- Start the API server in debug mode

  ```bash
  just debug-axum
  ```

### Terminal 2

The second terminal will serve to send requests to the server.

You can send requests using cURL or your favorite REST client, for instance:

```bash
# Query the first page of City Ratings.
curl http://localhost:3000/ratings
```

Or use the predefined test suites, using Hurl:

```bash
cd lambdas/tests
just test-smoke-public localhost
just test localhost
```

## Extras

### Generate the samples

Here is a one liner to generate a new `samples.csv` file from the 2025
historical results:

```bash
for id in `xan sample 10 city-ratings-all-historical-results-v25.09.csv | xan select bna_id`; do xan grep "${id}" "city-ratings-all-historical-results-v25.09.csv" ; done
```

[bacon]: https://dystroy.org/bacon/
[docker]: https://www.docker.com/get-started/
[hurl]: http://hurl.dev
[just]: https://github.com/casey/just
[postgresql]: https://www.postgresql.org/download/
[rust]: https://www.rust-lang.org/tools/install
[sea-orm-cli]: https://www.sea-ql.org/SeaORM/docs/generate-entity/sea-orm-cli/
