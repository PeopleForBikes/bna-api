# Lambdas

Repository containing all the lambda endpoints for API Gateway.

## Requirements

- [Just]
- [Docker]
- [Hurl]

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

- Seed the database

  ```bash
  just db-seed
  ```

- Start the API server in debug mode

  ```bash
  just debug-axum
  ```

### Terminal 2

The second terminal will serve to send requests to the server.

You can either send requests using cURL (or your favorite REST client), for
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

[just]: https://github.com/casey/just
[docker]: https://www.docker.com/get-started/
[hurl]: http://hurl.dev
