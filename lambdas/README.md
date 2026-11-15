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
- [Slumber], to explore the API
- [uv], for the Schemathesis tests

### Remark

Bacon, Hurl, Just, Sea-ORM-CLI and Slumber are pinned in `mise.toml` at the root
of the repository: install them with `mise install`, as described in the
[Local tools](../.github/CONTRIBUTING.md#local-tools) section of the
contributing guide. Docker, the PostgreSQL client, Rust and uv are installed
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
  just db-seed
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

To explore the API, use [Slumber] and the team's collection of requests,
`slumber.yml` in this folder. Start it from `lambdas/` or any folder below,
where it finds the collection by itself; from anywhere else, pass
`--file lambdas/slumber.yml`:

```bash
cd lambdas
# Browse the requests, send them, and inspect the responses in the TUI.
slumber
# Or send one request, here the list of cities, and print its response.
slumber request get_cities
```

The collection's only profile, `local`, sends the requests to the server started
in the first terminal, `http://localhost:3000`. Slumber keeps the history of the
requests and responses on your machine, never in the repository.

Write requests send `Authorization: Bearer <token>`. Authentication is disabled
locally, so the token is empty unless the `HURL_VARIABLE_token` environment
variable is set, the same variable the Hurl suites read (see
[`tests/README.md`](tests/README.md#authentication)):

```bash
HURL_VARIABLE_token=<token> slumber
```

Or use the predefined test suites, using Hurl, then Schemathesis, which
generates requests from the OpenAPI specification:

```bash
cd lambdas/tests
just test-smoke-public localhost
just test localhost
just test-schemathesis
```

Schemathesis writes random rows: run `just db-seed` again before the next Hurl
run. See [`tests/README.md`](tests/README.md#schemathesis).

### Adding a request to the collection

Add the requests you need to `slumber.yml`, following these conventions:

- A request (a _recipe_) goes in a folder for the `tags` of its operation in
  `openapi-3.1.yaml` (`city`, `pipeline`, `price`, `rating`, `report`, `system`
  or `usstate`): the folder's id and its `name` are both the tag. Create the
  folder with its first recipe.
- A recipe's id is the `operationId` of its operation (`get_city`,
  `post_cities_submission`). A variant of the same operation adds a suffix
  (`get_cities_sorted_by_name`). Slumber requires every recipe and folder id to
  be unique in the whole file, and refuses to load the collection otherwise.
- To reuse a value from another recipe's response, use
  `{{ response('<recipe>', trigger='no_history') | jsonpath('<path>') }}`.
  Slumber sends that recipe first when it has no stored response, and encodes
  the value for the URL.
- Write requests send `Authorization: Bearer {{ token }}`.
- JSON bodies use the types of the specification: numbers and booleans are not
  quoted.
- `slumber import openapi` can give a starting point, not finished recipes: it
  fills every query parameter with an empty value, which the API rejects, and
  quotes numbers and booleans.

For example, the `city` folder with a list of cities, a city taken from that
list, and a submission:

```yaml
requests:
  city:
    name: city
    requests:
      get_cities:
        name: List the cities
        method: GET
        url: "{{ host }}/cities"
        query:
          page_size: 5
      get_city:
        name: Get the first city of the list
        method: GET
        url:
          "{{ host }}/cities/{{ response('get_cities', trigger='no_history') |
          jsonpath('$[0].country') }}/{{ response('get_cities',
          trigger='no_history') | jsonpath('$[0].state') }}/{{
          response('get_cities', trigger='no_history') | jsonpath('$[0].name')
          }}"
      post_cities_submission:
        name: Submit a city
        method: POST
        url: "{{ host }}/cities/submissions"
        headers:
          Authorization: "Bearer {{ token }}"
        body:
          type: json
          data:
            city: Santa Rosa
            country: United States
            region: New Mexico
            fips_code: "3570670"
            first_name: Jane
            last_name: Doe
            email: jane.doe@example.com
            organization: Organization LLC
            consent: true
```

The specification's `fips_code` is a string, hence the quotes; `consent` is a
boolean, hence none.

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
[slumber]: https://slumber.lucaspickering.me/
[uv]: https://docs.astral.sh/uv/
