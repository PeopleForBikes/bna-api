# Design Document: API Request Collection with Slumber

## Overview

The feature changes configuration and documentation only; no Rust code, no
endpoint, no schema. It also gives the local PostgreSQL service a health check
that `just compose-up` waits for (Requirement 5). Slumber 5.4.4 joins the cargo
tools of `mise.toml`, built from crates.io exactly like `cargo:bacon`,
`cargo:sea-orm-cli` and `cargo:cargo-progenitor`, and stays out of CI because
every workflow names the tools it installs. A new `lambdas/slumber.yml` holds
one profile for the local server and one recipe, `get_cities`, the first example
of the README's conventions; its token comes from `HURL_VARIABLE_token`, the
variable the Hurl suites already read, so one variable serves both tools.

`lambdas/requests.rest` is deleted, and the `lambdas` workflow's path filter
excludes the new collection instead of the old one. `lambdas/README.md` gains
Slumber in its tool lists, a way to start it next to the existing ways of
sending requests, and the conventions the team follows when adding recipes;
`.github/CONTRIBUTING.md` lists Slumber with the other tools built from
crates.io.

Every behaviour of Slumber this design relies on was checked with Slumber 5.4.4
during the spike or while writing this document (built from crates.io in the
agent's scratch folder, `HOME` pointed at a scratch folder): see Testing
Strategy.

## Architecture

```text
mise.toml / mise.lock ──mise install──> slumber 5.4.4 (built from crates.io)
                                             │
contributor: cd lambdas && slumber ──────────┤ finds lambdas/slumber.yml
                                             │ (current folder, then parents)
lambdas/slumber.yml                          │
  profile "local" (default)                  │
    host  = http://localhost:3000            ▼
    token = {{ env('HURL_VARIABLE_token') }} ──> requests to the local API
  city/get_cities (more added by the team)        (just debug-axum)

history: ~/Library/Application Support/slumber/state.sqlite (per user, not in git)
```

Hurl (`lambdas/tests/`) is untouched and stays the testing tool; the two share
only the token variable.

## Components and Interfaces

### `mise.toml` and `mise.lock`

- `mise.toml`, `[tools]`: add `"cargo:slumber" = "5.4.4"` after
  `"cargo:cargo-progenitor"` (`mise.toml:27`), the last cargo tool.
- `mise.lock`: regenerated with the command of `.github/CONTRIBUTING.md`
  ("Bumping a tool"):
  `mise lock --platform linux-x64,linux-arm64,macos-arm64,macos-x64`. The new
  entry has the shape of the other cargo tools (`mise.lock:5-8`):
  `[[tools."cargo:slumber"]]`, `version = "5.4.4"`, `backend = "cargo:slumber"`,
  `specifiers = ["5.4.4"]`, with no URL or checksum, since the tool is built
  from source.
- The existing settings already give R1.2: `cargo.binstall = false`
  (`mise.toml:9`) builds from crates.io, and the cargo tools are built with
  `cargo install --locked` (`.github/CONTRIBUTING.md:68-69`).
- No workflow changes: `jdx/mise-action` installs only its `install_args` in
  `ci-rust.yml`, `bnaclient.yml` and `integration.yml`, none of which will name
  Slumber (R1.4).

### `lambdas/slumber.yml` (new)

```yaml
# yaml-language-server: $schema=https://raw.githubusercontent.com/LucasPickering/slumber/refs/tags/v5.4.4/schemas/collection.json
# Keep the tag above equal to the Slumber version pinned in mise.toml.
#
# Requests to explore the BNA API with Slumber: see lambdas/README.md.
name: BNA API

profiles:
  local:
    name: Local development API
    default: true
    data:
      host: http://localhost:3000
      # Empty unless HURL_VARIABLE_token is set; the Hurl suites read it too.
      token: "{{ env('HURL_VARIABLE_token') }}"

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
```

- The schema URL points at the tag of the pinned version; the file exists there
  (`schemas/collection.json`, 16,109 bytes at `v5.4.4`) (R2.8).
- `name: Local development API` is the description of the
  `http://localhost:3000` server in `openapi-3.1.yaml:10-11` (R2.3).
- `requests` holds one folder, `city` (the tag of `GET /cities`), with one
  recipe, `get_cities` (its `operationId`), identical to the first recipe of the
  README example (R2.4, R2.10). It was added at the user's request after the
  first implementation, which had `requests: {}` (task 7).
- The token is a template, rendered when a request is built, so no value is ever
  stored in the file (R2.5–R2.7).
- Discovery (R2.2): started without `--file`, Slumber looks for `slumber.yml` in
  the current folder and its parents, so any folder under `lambdas/` finds this
  file. From the repository root it finds nothing; the README says to start it
  from `lambdas/` or with `--file lambdas/slumber.yml`.

### `lambdas/requests.rest` and `.github/workflows/lambdas.yml`

- Delete `lambdas/requests.rest` (R3.1). The only mention outside `specs/` is
  `.github/workflows/lambdas.yml:12` (`git grep -n 'requests\.rest'`).
- Replace that line, `- "!lambdas/requests.rest"`, with
  `- "!lambdas/slumber.yml"` (R3.2–R3.4). GitHub evaluates `paths` in order: a
  file matching `lambdas/**` and then the negative pattern is excluded, and the
  workflow runs when at least one changed file remains. A pull request changing
  only `lambdas/slumber.yml` therefore skips the workflow; one changing any
  other file under `lambdas/` still runs it, as `requests.rest` behaved until
  now.

### `lambdas/README.md`

- **Requirements** (`lambdas/README.md:5-13`): add `[Slumber]` to the list, in
  alphabetical order after `[Sea-ORM-CLI]`, and its link definition with the
  others (`lambdas/README.md:93-100`) (R4.1).
- **Remark** (`lambdas/README.md:15-21`): "Bacon, Hurl, Just, Sea-ORM-CLI and
  Slumber are pinned in `mise.toml`…" (R4.1).
- **Terminal 2** (`lambdas/README.md:58-79`), after the cURL example: a
  paragraph and example on exploring with Slumber (R4.2, R4.3):

  ```bash
  cd lambdas
  slumber                                   # the TUI
  slumber request get_cities                # one request, printed
  HURL_VARIABLE_token=<token> slumber       # send a Bearer token
  ```

  with the facts a newcomer needs: it loads `lambdas/slumber.yml` from
  `lambdas/` or any folder below (otherwise `--file lambdas/slumber.yml`); the
  `local` profile targets `http://localhost:3000`; authentication is disabled
  locally, so the token is empty unless `HURL_VARIABLE_token` is set, the same
  variable as for the Hurl suites (`tests/README.md`); history stays on the
  contributor's machine.

- **New subsection "Adding a request to the collection"**, after Terminal 2,
  stating the conventions (R4.4). They were agreed as a starting point: if the
  team does not like them, it changes this subsection and the recipes, nothing
  else.
  - Recipes go in a folder named after the operation's OpenAPI tag (`city`,
    `pipeline`, `price`, `rating`, `report`, `system`, `usstate`), created when
    its first recipe is added; the folder's `name` is the tag.
  - A recipe's id is the operation's `operationId` (`get_city`,
    `post_cities_submission`); a variant of the same operation adds a suffix
    (`get_cities_sorted_by_name`). Slumber requires every recipe and folder id
    to be unique across the whole file, and refuses to load the collection
    otherwise (`Duplicate recipe/folder ID`); operation ids are unique in the
    specification, and no tag equals an operation id.
  - A value from another recipe's response is reused with
    `{{ response('<recipe>', trigger='no_history') | jsonpath('<path>') }}`:
    Slumber sends that recipe first if it has no stored response, and encodes
    the value for the URL.
  - A write request sends `Authorization: Bearer {{ token }}`.
  - Bodies use `body: { type: json, data: ... }` with the types of the
    specification (numbers and booleans unquoted); `slumber import openapi` is a
    starting point only, since its output sends empty query parameters and
    quotes numbers and booleans (spike §5.3 S2).
  - One example, a folder with a list recipe and a recipe chained on it.
- Link definition: `[slumber]: https://slumber.lucaspickering.me/`, the homepage
  `slumber --help` prints.

### `docker-compose.yml` and the `compose-up` recipe

- `docker-compose.yml`, service `postgres` (image `postgres:17.9`): a
  `healthcheck` running `pg_isready -d postgres -U postgres`, the user and
  database of the local `DATABASE_URL`, with `interval: 10s`, `timeout: 60s`,
  `retries: 6` and `start_period: 10s` (R5.1).
- `justfile`, `compose-up`: `docker compose up -d --wait` instead of
  `docker compose up -d`. `--wait` makes Compose wait for the services to be
  "running|healthy" and implies detached mode (Compose's `compose_up.md`); a
  service with a health check counts once it is `healthy` (R5.2).
- Timing, from the Dockerfile `HEALTHCHECK` reference (the same semantics as
  Compose's `healthcheck`): during the start period, checks run every start
  interval (5 s by default) and their failures do not count; the first success
  makes the service `healthy`, so `compose-up` normally returns a few seconds
  after PostgreSQL accepts connections. Afterwards a check runs `interval`
  seconds after the previous one completes, so checks never overlap, whatever
  the `timeout`; a check that runs longer than `timeout` fails. Six consecutive
  failures make the service `unhealthy` (R5.4).
- `compose` (`compose-down` then `compose-up`) inherits the wait.

### `.github/CONTRIBUTING.md`

- Prerequisites (`.github/CONTRIBUTING.md:20-21`): "mise builds bacon,
  sea-orm-cli, cargo-progenitor and slumber from crates.io" (R4.5).
- What it protects against (`.github/CONTRIBUTING.md:68-69`): "bacon,
  sea-orm-cli, cargo-progenitor and slumber are built from crates.io with
  `cargo install --locked`…" (R4.5).

## Data Models

`lambdas/slumber.yml` follows Slumber's collection format, version 5.4.4,
described by its JSON schema (`schemas/collection.json` at `v5.4.4`). The keys
this feature uses:

| Key                         | Value                                                   |
| --------------------------- | ------------------------------------------------------- |
| `name`                      | `BNA API`                                               |
| `profiles.local.name`       | `Local development API`                                 |
| `profiles.local.default`    | `true`                                                  |
| `profiles.local.data.host`  | `http://localhost:3000`                                 |
| `profiles.local.data.token` | `{{ env('HURL_VARIABLE_token') }}`                      |
| `requests`                  | folder `city` (`name`, `requests`) holding `get_cities` |

No other data is added. Slumber's history lives outside the repository.

## Correctness Properties

1. **One version**: the Slumber version in `mise.toml`, in `mise.lock` and in
   the schema URL of `lambdas/slumber.yml` are the same (5.4.4).
2. **No secret in git**: `lambdas/slumber.yml` contains no literal token,
   password or key; the only credential is the `env()` template.
3. **Local by default**: the collection has exactly one profile, `local`, and it
   is the default, so a request cannot reach staging or production without
   someone adding a profile.
4. **One collection**: `lambdas/requests.rest` no longer exists, and nothing
   outside `specs/` refers to it.
5. **CI unchanged**: no workflow installs Slumber, and the `lambdas` workflow
   triggers on exactly the files it triggered on before, with the collection in
   place of `requests.rest`.
6. **A ready database**: when `just compose-up` exits 0, PostgreSQL accepts
   connections.

## Error Handling

| Condition                                  | Behaviour                                                                                                                                                                                                                                                       |
| ------------------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `HURL_VARIABLE_token` not set              | `token` renders empty, so the header is `authorization: Bearer` with no value (checked, `--dry-run`). Harmless locally, where authentication is disabled, as with `localhost.vars`.                                                                             |
| Slumber started outside `lambdas/`         | `No collection file found in … or its ancestors`, exit 1. The README gives `--file lambdas/slumber.yml`.                                                                                                                                                        |
| Two recipes or folders with the same id    | The collection does not load (`Duplicate recipe/folder ID`, exit 1); the README's naming convention avoids it.                                                                                                                                                  |
| Invalid YAML or an unknown key             | Slumber reports the error with its line on load; editors flag it beforehand through the schema.                                                                                                                                                                 |
| `mise install` not run                     | `auto_install = false`; mise does not build Slumber behind the contributor's back, and a `slumber` elsewhere on the `PATH` may run instead (`specs/learnings/integration-issues/mise-missing-tool-falls-back-to-path.md`). The README points to `mise install`. |
| PostgreSQL never accepts connections       | The service becomes `unhealthy` after six consecutive failed checks; `--wait` makes `just compose-up` fail instead of returning (R5.4).                                                                                                                         |
| Upstream recipe of a `response(...)` fails | Slumber reports the failed upstream request (`Error executing request for 'upstream'`), exits 1 and does not send the dependent one (checked).                                                                                                                  |

## Integration

- **mise and the contributor setup** (`specs/0003-tool-manager`): one more cargo
  tool; `mise install` builds it once per contributor (47.7 s for 5.4.4 in the
  spike). Bumps follow "Bumping a tool" in `.github/CONTRIBUTING.md`, and the
  schema tag in `lambdas/slumber.yml` changes with the version (the comment next
  to it says so).
- **Hurl suites** (`lambdas/tests/`): unchanged; the token variable is shared on
  purpose.
- **CI**: `ci-rust.yml` runs `just lint`, which spell-checks the new file with
  typos and checks the Markdown; no other workflow is affected besides the path
  filter of `lambdas.yml`.
- **Local database setup** (`docker-compose.yml`, `justfile`): `compose-up`, and
  `compose` through it, now return once PostgreSQL is ready, so the Quickstart
  of `lambdas/README.md` and `just compose-up && just db-seed` work without a
  pause. The nono sandbox denies Docker
  (`specs/learnings/integration-issues/docker-in-the-nono-sandbox.md`): the user
  runs these recipes.
- **nono sandbox**: agents can read and edit the collection; running Slumber
  inside the sandbox needs its history folder or a `HOME` override, which is out
  of scope (requirements, "Out of scope").

## Testing Strategy

No Rust code changes, so no new Rust tests; `just ci` must still pass. Each
criterion is checked by a command, run on the implementation branch:

| Criteria         | Check                                                                                                                                                                                                                                                          |
| ---------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| R1.1             | `grep '"cargo:slumber" = "5.4.4"' mise.toml`, and the `cargo:slumber` entry in `mise.lock`.                                                                                                                                                                    |
| R1.2, R1.3       | `mise install` (run by the user, outside the sandbox, as for every mise install), then `slumber --version` → `slumber 5.4.4`.                                                                                                                                  |
| R1.4             | `grep -n 'slumber' .github/workflows/*.yml` finds nothing.                                                                                                                                                                                                     |
| R2.1, R2.2, R2.9 | `slumber --file lambdas/slumber.yml collection` exits 0; from `lambdas/tests/endpoints`, `slumber collection` prints the same collection.                                                                                                                      |
| R2.3, R2.4       | The output of `slumber collection` shows one profile `local`, `default: true`, `host: http://localhost:3000`, and one recipe, `city/get_cities`.                                                                                                               |
| R2.5, R2.6       | With a temporary recipe in a scratch copy of the collection (never committed): `slumber request <recipe> --dry-run` prints `authorization: Bearer` with no value when the variable is unset, and `Bearer abc123` with `HURL_VARIABLE_token=abc123`.            |
| R2.7             | Review of the file: only the `env()` template, no literal credential.                                                                                                                                                                                          |
| R2.8             | The schema URL's tag equals `mise.toml`'s version, and the URL answers (`gh api repos/LucasPickering/slumber/contents/schemas/collection.json?ref=v5.4.4`).                                                                                                    |
| R2.10            | With the local API running: `slumber request get_cities -v` sends `GET http://localhost:3000/cities?page_size=5` and receives `200` with at most five cities.                                                                                                  |
| R3.1, R3.2       | `test ! -e lambdas/requests.rest`; `git grep -n 'requests\.rest' -- ':!*specs/*'` finds nothing.                                                                                                                                                               |
| R3.3, R3.4       | Review of `lambdas.yml` against GitHub's `paths` semantics. Observable only after merging: the implementation pull request touches other `lambdas/` files and runs the workflow (R3.4); the first pull request that only adds a recipe must not run it (R3.3). |
| R4.1–R4.5        | Review of `lambdas/README.md` and `.github/CONTRIBUTING.md` against the criteria.                                                                                                                                                                              |
| R4.6             | `just lint` passes.                                                                                                                                                                                                                                            |
| R5.1, R5.2       | Review of `docker-compose.yml` and `justfile`; then, run by the user, `just compose-down` and `just compose-up`: `docker compose ps` shows the `postgres` service `(healthy)` as soon as the recipe returns.                                                   |
| R5.3             | Run by the user: `just compose-down && just compose-up && just db-seed` exits 0.                                                                                                                                                                               |
| R5.4             | Run by the user: `docker compose` with a scratch override file setting the health check to `["CMD", "false"]` and `--wait` exits non-zero once the service is `unhealthy`; then `just compose`.                                                                |

Every Slumber behaviour above was already observed with 5.4.4: an empty
`requests: {}` loads; `env()` renders an unset variable as empty and a set one
as its value; the collection is found from `lambdas/tests/endpoints` and not
from the repository root; a duplicated id refuses to load; `response(...)` with
`trigger='no_history'` sends the upstream recipe and URL-encodes the value
(spike §5.3 S3).

## Deployment Considerations

Nothing is deployed: the Lambda functions, the API and the OpenAPI specification
do not change, so `just generate` has nothing to regenerate. After merging, each
contributor runs `mise install` once to build Slumber.

## Dependencies

- **Slumber 5.4.4** (MIT, `LucasPickering/slumber`, released 2026-10-27), built
  from crates.io by mise with the Rust toolchain contributors already need.
- No new library, service or CI action.
