# Requirements Document: API Request Collection with Slumber

## Introduction

The team explores the BNA API with a collection of requests kept in
`lambdas/requests.rest`, written for the VS Code "REST Client" extension. The
extension has had no release since 2022, the team now also works in Zed, and the
collection has drifted from the API. The spike in [`spike.md`](spike.md)
compared the replacements and concluded with a Conditional Go for [Slumber], a
terminal HTTP client with a TUI and a CLI whose collection is one YAML file. Its
condition, a hands-on trial of the TUI by a Zed user and a VS Code user, has
passed. Hurl stays the tool that tests the API (`lambdas/tests/`).

This feature makes Slumber the team's way of exploring the API: it pins Slumber
with the other command-line tools, starts a collection in `lambdas/slumber.yml`
with a first request, the list of cities, that the team extends as it needs
requests, retires `lambdas/requests.rest`, and documents how to use and extend
the collection. It also makes `just compose-up` wait until PostgreSQL accepts
connections, so that the local setup can be chained with the database recipes
(raised by the user during the spike, added to this feature after its review).
Last, it gives the examples of the OpenAPI specification the types of their
fields: the importers copy them, and a quoted `"true"` for a boolean made the
seeded bodies fail with `422` (`findings.md` §1.2, fixed in this feature after
its learnings review).

**In scope**:

- Pinning Slumber 5.4.4 in `mise.toml` and `mise.lock`, built from crates.io
  like the other cargo tools.
- `lambdas/slumber.yml` with one profile for the local server, the token
  convention shared with the Hurl suites, and one recipe, `get_cities`, the
  first example of the README's conventions (added at the user's request after
  the first implementation, task 7).
- Deleting `lambdas/requests.rest` and updating the CI path filter that names
  it.
- Documentation in `lambdas/README.md` (use, and conventions for adding recipes)
  and `.github/CONTRIBUTING.md` (the pinned tool).
- A health check on the PostgreSQL service of `docker-compose.yml`, and
  `just compose-up` waiting for it.
- Examples of `openapi-3.1.yaml` whose JSON type matches the `type` of their
  field, and a test that keeps it so. Fields documented as strings keep string
  examples, including the `Decimal` amounts (`cost`, `per_second`).

**Out of scope** (decided with the user; the spike's recommendation differs on
the first point):

- The `hurl-at` wrapper and editor tasks for sending one Hurl entry (spike §6,
  Appendix A): dropped.
- Recipes beyond `get_cities`: the team adds them as it needs them; none are
  ported from `lambdas/requests.rest` or imported from the OpenAPI
  specification.
- Profiles for staging and production.
- Cognito login recipes.
- A check that keeps the collection in step with `openapi-3.1.yaml`.
- Installing Slumber in CI, a `just` recipe to start it, and editor
  configuration files.
- Access for agents in the nono sandbox to Slumber's history folder (spike §7).

## Glossary

- **Slumber**: the HTTP client adopted by the spike, run as a TUI (`slumber`) or
  a CLI (`slumber request <recipe>`).
- **Collection**: the YAML file Slumber loads, here `lambdas/slumber.yml`. When
  no file is given, Slumber looks for `slumber.yml` in the current folder and
  its parents.
- **Recipe**: a named request definition in the collection.
- **Profile**: a named set of values (such as `host`) that recipes use through
  templates; one profile can be the default.
- **Template**: a `{{ ... }}` expression Slumber renders when it builds a
  request, such as `{{ host }}` or `{{ env('NAME') }}`.
- **Token_Variable**: the environment variable `HURL_VARIABLE_token`, which the
  Hurl suites already read for the Bearer token of write requests
  (`lambdas/tests/README.md`).
- **Pinned_Tools**: the command-line tools `mise.toml` pins and `mise.lock`
  records, installed with `mise install`.

## Requirements

### Requirement 1: Slumber is a pinned tool

**User Story:** As a contributor, I want Slumber installed at the team's version
with the other tools, so that everyone explores the API with the same client
without installing it by hand.

#### Acceptance Criteria

1. The Pinned_Tools SHALL include Slumber at version 5.4.4, built from
   crates.io.
2. WHEN a contributor runs `mise install` on a checkout, mise SHALL build
   Slumber 5.4.4 with `cargo install --locked`, never from a prebuilt binary.
3. WHEN a contributor runs `mise install`, `slumber --version` SHALL print
   `slumber 5.4.4`.
4. The CI workflows SHALL NOT install Slumber.

### Requirement 2: A collection for the local server

**User Story:** As a contributor, I want a collection that is ready to receive
requests against my local server, so that I can start exploring the API and add
the requests I need.

#### Acceptance Criteria

1. The collection SHALL be the file `lambdas/slumber.yml`.
2. WHEN a contributor starts Slumber in `lambdas/` or one of its subfolders
   without naming a file, Slumber SHALL load `lambdas/slumber.yml`.
3. The collection SHALL define exactly one profile, the default one, for the
   local server at `http://localhost:3000`, named after the
   `Local development API` server of `openapi-3.1.yaml`.
4. The collection SHALL define exactly one recipe, `get_cities`, in a folder
   `city`, sending `GET /cities` with the query parameter `page_size=5` to the
   profile's host.
5. WHILE the Token_Variable is not set, the profile's token SHALL render as an
   empty string.
6. WHILE the Token_Variable is set, the profile's token SHALL render as its
   value.
7. The collection SHALL hold no token, password or other secret.
8. The collection SHALL declare the JSON schema of the pinned Slumber version,
   so that editors using `yaml-language-server` validate and complete it.
9. WHEN `slumber --file lambdas/slumber.yml collection` runs, it SHALL exit with
   status 0.
10. WHEN a contributor sends `get_cities` while the local API runs with the test
    data, the API SHALL answer with status 200 and at most five cities.

### Requirement 3: `lambdas/requests.rest` is retired

**User Story:** As a contributor, I want one collection only, so that nobody
keeps extending the old one or wonders which one is current.

#### Acceptance Criteria

1. The repository SHALL NOT contain `lambdas/requests.rest`.
2. Outside the `specs/` folders, the repository SHALL NOT mention
   `requests.rest`.
3. WHEN a pull request changes only `lambdas/slumber.yml`, the `lambdas` CI
   workflow SHALL NOT run.
4. WHEN a pull request changes any other file under `lambdas/`, the `lambdas` CI
   workflow SHALL run as before.

### Requirement 4: Documentation

**User Story:** As a contributor, I want to read how to install, start and
extend the collection, so that I can explore the API and add requests the same
way as the rest of the team.

#### Acceptance Criteria

1. `lambdas/README.md` SHALL list Slumber among the requirements, and among the
   tools pinned in `mise.toml`.
2. `lambdas/README.md` SHALL explain how to start Slumber on the collection and
   send requests to the local server, next to the existing ways of sending
   requests (cURL, the Hurl suites, Schemathesis).
3. `lambdas/README.md` SHALL explain how to provide a token through the
   Token_Variable.
4. `lambdas/README.md` SHALL state the conventions for adding a recipe: where it
   goes in the collection, how it reuses a value from another recipe's response,
   and how a write request sends the token.
5. `.github/CONTRIBUTING.md` SHALL list Slumber among the tools mise builds from
   crates.io with `cargo install --locked`.
6. WHEN `just lint` runs, it SHALL pass on the changed Markdown files.

### Requirement 5: `just compose-up` waits for PostgreSQL

**User Story:** As a contributor, I want `just compose-up` to return only once
the database accepts connections, so that I can chain it with the database
recipes (`just compose-up && just db-seed`) without waiting by hand.

#### Acceptance Criteria

1. The PostgreSQL service of `docker-compose.yml` SHALL report itself healthy
   only when `pg_isready` succeeds against it.
2. WHEN a contributor runs `just compose-up`, the recipe SHALL return only after
   the PostgreSQL service reports itself healthy.
3. WHEN a contributor runs `just compose-up && just db-seed` with the services
   stopped, `just db-seed` SHALL succeed.
4. IF the PostgreSQL service becomes unhealthy, THEN `just compose-up` SHALL
   exit with a non-zero status.

### Requirement 6: OpenAPI examples have the types of their fields

**User Story:** As a contributor seeding requests from the OpenAPI
specification, I want its examples to have the types of their fields, so that a
request built from them is accepted by the API.

#### Acceptance Criteria

1. Every example of a schema or parameter in `openapi-3.1.yaml` SHALL have a
   JSON type that the `type` of its schema allows: a number for `integer` or
   `number`, `true` or `false` for `boolean`, a string for `string`.
2. The example of `pop_size`, a population size category stored as an integer,
   SHALL be the integer of the category its example population belongs to.
3. IF an example of `openapi-3.1.yaml` has a JSON type that the `type` of its
   schema does not allow, THEN `cargo test` in `lambdas` SHALL fail and name the
   location of that example.
4. WHEN a submission is created from the examples of `SubmissionPost`, the API
   SHALL answer with status 201.

[Slumber]: https://github.com/LucasPickering/slumber
