# Findings: Hurl API Coverage

Everything non-obvious learned while implementing this feature: behavior that
looks like a bug unless you know it is deliberate, traps in the libraries and
tools, and what worked. The findings of the specification review, which are work
to do, are in [review.md](./review.md).

Status: in progress; keep it current as the code changes.

## 1. Behavior that does not survive a naive implementation

### 1.1 An `entity` wrapper without a caller here is not dead code

Lambda functions outside this repository use `entity::wrappers`. `BnaRegion`,
`BikeLaneType`, `CityPatch`, and `wrappers/bna.rs` have no caller in this
workspace, and stay.

**Caught by**: the team, after a cleanup removed them because a search found no
caller.

**Cost**: 3 commits to restore them. `CLAUDE.md` now states the rule.

### 1.2 The region of a city is not the BNA region

The region of a city is in the `state` column and the `{region}` path parameter.
The `region` column holds the BNA region of a US city, and otherwise the value
sent or the country name. Pending a dedicated branch (M-44).

**Caught by**: the team, reviewing an explanation that mixed the two up.

**Cost**: every description of these fields in the specification is misleading
until then.

### 1.3 A rejected body keeps the request path in `source.pointer`

`source.pointer` is always the request path, even for a rejected body field, and
`details` names the field (M-42). This differs from JSON:API on purpose. Only
the first invalid field is reported, because deserialization stops there.

**Caught by**: the team, reviewing the error responses.

### 1.4 The `Country` enum and the `country` table must hold the same values

The API documents the enum (`core/resource/schema.rs`), and the foreign key and
`POST /cities` check the table, so nothing else keeps them equal: adding a
country takes both a migration and the enum. They differ by nothing since
`m20261107_200000_all_countries`.

**Caught by**: the specification review (R-03), and a comparison of the two
lists.

**Cost**: 177 documented countries were rejected with a 422.

## 2. Library and tooling traps

### 2.1 Updating a row with no changes reports it as not found

SeaORM reports `RecordNotFound` rather than `RecordNotUpdated` when an update
has nothing to change, so an empty `PATCH` body needs both mapped to 404
(`pipelines/adaptor.rs`, `cities/adaptor.rs`).

### 2.2 utoipa drops the examples of a `value_type = Object` field

`sqs_message` uses `schema_with` instead (`pipelines/schema.rs`).

### 2.3 Type arrays and nullable references break the client generation

A schema of type `[object, string, null]` converts to invalid 3.0, so
`sqs_message` uses `oneOf`. An optional enum reference (`oneOf` with `null`)
makes progenitor panic, so optional enums use `#[schema(nullable = false)]`.

### 2.4 Cargo ignores a binary in a subdirectory of `src/bin`

Only `src/bin/*.rs` and `src/bin/*/main.rs` are targets, so
`src/bin/ratings/patch-ratings-analysis.rs` was never compiled, and could rot
unnoticed.

### 2.5 An unused dependency can still change production behavior

`tracing-test`, a normal dependency of `lambdas`, was what enabled the `ansi`
and `tracing-log` features of `tracing-subscriber`. Removing it would have
silently lost the log records of crates using `log`, so `lambdas` now requests
`tracing-log`. Compare `cargo tree -e normal -f '{p} [{f}]'` before and after
removing a dependency.

### 2.6 `hurl --test` runs files in parallel

A count compared between two requests is racy when other files create rows. Test
what cannot change instead: a full page rather than a total (`cities.hurl`).

## 3. Method — what actually worked

### 3.1 Prove a refactoring with a byte comparison

Moving the error types out of `effortless`, and each cleanup, left the
regenerated specification and `bnaclient` byte-identical, which showed nothing
changed for clients. The conversion of `baseline.sql` left the generated dump
byte-identical.

### 3.2 Compare a port against the original on the same input

The Rust coverage check was compared with the Python script on one real report
and one report built to fail every check.

### 3.3 Reproduce a review finding before fixing it

Review findings come from reading the code. Running them against the local API
confirmed them, and showed which responses to expect in the tests.
