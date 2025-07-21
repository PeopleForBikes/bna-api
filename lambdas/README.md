# Lambdas

Repository containing all the lambda endpoints for API Gateway.

## Requirements

- [Bacon]
- [Docker]
- [Hurl]
- [Just]
- [Rust]
- [Sea-ORM-CLI]

### Remark

Once [Rust] is setup, most of the tools can be installed using [cargo binstall]:

```bash
cargo install cargo-binstall
cargo binstall bacon@3.16.0 hurl@7.0.0 just@1.42.4 sea-orm-cli@1.1.1
```

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

- Initialize the database

  ```bash
  just db-init
  ```

- Start the API server in debug mode

  ```bash
  just debug-axum
  ```

### Terminal 2

The second terminal will serve to send requests to the server.

You can send requests using cURL or your favorite REST client, for
instance:

```bash
# Query the first page of City Ratings.
curl http://localhost:3000/ratings
```

Or use the predifined test suites, using Hurl:

```bash
cd lambda/tests
just test-smoke-public localhost
just test localhost
```

[bacon]: https://dystroy.org/bacon/
[cargo binstall]: https://github.com/cargo-bins/cargo-binstall
[docker]: https://www.docker.com/get-started/
[hurl]: http://hurl.dev
[just]: https://github.com/casey/just
[rust]: https://www.rust-lang.org/tools/install
[sea-orm-cli]: https://www.sea-ql.org/SeaORM/docs/generate-entity/sea-orm-cli/
