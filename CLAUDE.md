# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with
code in this repository.

## What this is

`bna-api` is the API backend for PeopleForBikes' BNA (Bicycle Network Analysis)
project. It is a Rust Cargo workspace deployed as AWS Lambda functions behind
API Gateway, backed by PostgreSQL via SeaORM. The repo bundles the DB
migrations, generated entities, the Axum-based Lambda API, the schema docs, and
a generated Rust REST client (`bnaclient`).

Workspace members: `bnaclient`, `lambdas`, `entity`, `migration`, `effortless`.

## Common commands

Most repo-level tasks are `just` recipes (see `justfile`). Key ones:

```bash
just fmt                       # format markdown + justfile
just lint                      # markdown lint + spellcheck (this is what CI runs)
just compose-up / compose-down # start/stop local Postgres + pgadmin (docker-compose.yml)
just db-migrate                # apply migrations (sea-orm-cli migrate up)
just db-fresh                  # drop all tables and re-apply migrations
just db-generate-models        # regenerate entity/src/entities from the live DB schema
just db-seed                   # seed DB from a City Ratings CSV via `cargo run --example seeder`
just db-reset                  # fresh + generate-models + seed
just debug-axum                # run the Axum server locally in watch mode (bacon)
just generate-oas-31           # regenerate openapi-3.1.yaml from the Axum source (utoipa)
just generate-oas-30           # down-convert 3.1 -> openapi-3.0.yaml
just regenerate-all            # regenerate both OAS files + the bnaclient
```

Standard Cargo commands work as usual for building/testing/checking the Rust
code (`cargo build`, `cargo test`, `cargo test -p lambdas <name>`,
`cargo clippy`). Note that CI (`.github/workflows/ci-rust.yml`) currently only
lints markdown via `just lint-md`; it does not run `cargo test`/`clippy` in CI,
so run these locally before committing.

Local dev DB: `just compose-up` starts Postgres on `localhost:5432` (user/db
`bna`/`bna`, or `postgres`/`postgres`) and pgadmin on `:8484`. `DATABASE_URL` is
read from a `.env` file (`set dotenv-load` in the justfile).

### Integration / API tests

Located in `lambdas/tests`, using [Hurl](https://hurl.dev) and
[Schemathesis](https://github.com/schemathesis/schemathesis), driven by their
own `justfile`:

```bash
cd lambdas/tests
just test localhost            # or `just test staging` — runs the hurl suites in endpoints/ and scenario/
just test-smoke-public localhost
just test-localhost-schemathesis
```

## Architecture

### Workspace layout

- **`entity/`** — SeaORM entities (`entity/src/entities`), generated from the
  live DB schema via `just db-generate-models`. Do not hand-edit generated
  entity files; change migrations and regenerate instead. `entity/src/wrappers`
  holds hand-written DTO/wrapper types (e.g. `wrappers::city::CityPost`,
  `wrappers::submission::{SubmissionPost, SubmissionPatch}`) used at the API
  boundary, distinct from the raw SeaORM models.
- **`migration/`** — SeaORM migrations (`m20220101_000001_main.rs`, etc.),
  applied with `sea-orm-cli migrate`. Schema docs (`docs/database.dbml`,
  `docs/database.sql`, `docs/database.svg`) are derived from the DB via the
  `db-dump`/`dbml-*` just recipes.
- **`effortless/`** — small shared helper crate for the Lambda API: `api`,
  `error` (`APIError`/ `APIErrors`), `fragment` (request extension helpers like
  `BnaRequestExt`), and `response` types. Import these instead of re-inventing
  error/response shapes in `lambdas`.
- **`lambdas/`** — the actual API. `src/main.rs` builds the Axum app + `utoipa`
  OpenAPI spec and runs it as a Lambda (via `lambda_http::run`);
  `src/bin/ratings` is a separate binary. `src/lib.rs` exposes shared
  infrastructure: the global `DB_CONN: OnceCell<DatabaseConnection>`,
  `database_connect*` helpers, pagination constants (`MAX_PAGE_SIZE`,
  `DEFAULT_PAGE_SIZE`), and the `APIResult<T>` type.
- **`bnaclient/`** — a Rust client generated from `openapi-3.0.yaml` via
  `cargo progenitor` (`just generate-client`). Never hand-edit; regenerate via
  `just regenerate-all` (or `regenerate-all-no-cargo` to keep
  `bnaclient/Cargo.toml` from being auto-updated).

### Resource module pattern (`lambdas/src/core/resource/<name>/`)

Each API resource (`cities`, `pipelines`, `price`, `ratings`, `reports`,
`system`, `usstates`) follows the same three-layer split, and new
resources/endpoints should follow it too:

- `db.rs` — raw SeaORM queries against `entity::*` (fetch/filter/paginate). Free
  functions, no HTTP concerns.
- `adaptor.rs` — business logic sitting between DB and HTTP: calls `db.rs`
  functions, converts between entity models and `entity::wrappers` DTOs, does
  validation, returns `Context`/`ExecutionError`.
- `endpoint.rs` — Axum handlers (`Path`/`Query`/`Json` extractors), calls the
  adaptor, maps results to `APIResult<T>` / `ErrorResponses`, and carries the
  `utoipa` OpenAPI annotations for that route.
- `schema.rs` — resource-local request/response types (`ToSchema`, `IntoParams`,
  `IntoResponses`) distinct from the cross-resource types in
  `core/resource/schema.rs` (pagination params, `Country` enum, generic
  `ErrorResponses`, etc.).

Pagination follows a shared convention across resources:
`ListParameters`/`PaginationParameters` from `core/resource/schema.rs`,
`MAX_PAGE_SIZE`/`DEFAULT_PAGE_SIZE` from `lambdas/src/lib.rs`, and
`Paginatron`/`PageFlow` helpers for building paginated responses with link
headers (`core/link_header.rs`).

### OpenAPI spec flow

The OpenAPI spec is generated from code, not hand-written: `utoipa` annotations
on the Axum handlers/schemas in `lambdas/src/core/resource/**` are the source of
truth. The flow is `generate-oas-31` (runs the Axum binary with
`BNA_API_GENERATE_ONLY=1` to emit `openapi-3.1.yaml`) → `generate-oas-30`
(down-converts to `openapi-3.0.yaml`) → `generate-client` (progenitor generates
`bnaclient` from the 3.0 spec). Run `just regenerate-all` after changing any
endpoint/schema shape.
