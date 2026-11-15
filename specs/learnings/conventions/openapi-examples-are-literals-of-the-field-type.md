---
title: A utoipa example is a literal of the field's type, never a quoted string
date: 2026-11-15
updated: 2026-11-15
category: conventions
status: active
tags: [utoipa, openapi, examples, schema, annotations]
modules:
  [
    lambdas/src/core/resource,
    lambdas/tests/openapi_examples.rs,
    openapi-3.1.yaml,
    openapi-3.0.yaml,
  ]
spec: lambdas/specs/0000-api-request-collection
related:
  [
    lambdas/specs/0000-api-request-collection/findings.md,
    specs/learnings/build-errors/openapi-schemas-that-break-client-generation.md,
  ]
superseded_by:
---

# A utoipa example is a literal of the field's type, never a quoted string

## Context

The OpenAPI specification is generated from the utoipa annotations of
`lambdas/src/core/resource/**` (utoipa 6.0.0, `Cargo.toml:50`). Its examples are
not decoration: importers such as `slumber import openapi` and
`bru import openapi` copy them into the requests they build, and Schemathesis
has an examples phase. `lambdas/specs/0000-api-request-collection`
(Requirement 6) found and fixed the examples whose type was wrong.

## Symptom

A submission built by `slumber import openapi` from the specification was
rejected:

```text
422 Invalid Request Body
Failed to deserialize the JSON body into the target type: consent: invalid type: string "true", expected a boolean at line 1 column 34
```

The specification had `type: boolean` with `examples: - 'true'` for `consent`,
in three schemas, and 15 more examples quoted where the type is an integer or a
number, among them `pop_size: i32` with the example `"large"`.

## What Did Not Work

- Searching the `schema.rs` files for quoted examples on non-string fields: it
  found 14 of the 18. Path parameters are declared in the `params(...)` of the
  endpoints, with `example = "1"`
  (`lambdas/src/core/resource/cities/endpoint.rs`).

## Root Cause

utoipa writes the value of `examples(...)` or `example = ...` as given: a string
literal becomes a JSON string, whatever the field's type. Nothing checks an
example against its schema, neither utoipa nor `just generate`, so
`#[schema(examples("true"))]` on `consent: bool` produced `'true'` in
`openapi-3.1.yaml`.

## Solution

Write the example as a literal of the field's type (commit `d9fa2d6`):

```rust
#[schema(examples(true))]      // consent: bool, lambdas/src/core/resource/cities/schema.rs:216
#[schema(examples(51.260197))] // latitude: Option<f64>, lambdas/src/core/resource/schema.rs:302
("submission_id" = i32, Path, description = "Submission identifier", example = 1),
```

A field documented as a string keeps a string example: `Decimal` is a `string`
in the specification (utoipa's `decimal` feature, `lambdas/Cargo.toml:32`), so
`#[schema(examples("6.8941"))]` on `cost` stays
(`lambdas/src/core/resource/pipelines/schema.rs:39`). An enum takes the string
of its variant (`example = "Pending"`). Then `just generate`: the two OpenAPI
files changed in exactly the 18 examples, and `bnaclient` did not change.

## Prevention

`lambdas/tests/openapi_examples.rs` reads `openapi-3.1.yaml` and fails when an
example's JSON type is not allowed by its schema's `type` (`integer` also
satisfies `number`), naming each one by its path:

```text
/components/schemas/SubmissionPost/properties/consent: example "true" is a string, the type allows ["boolean"]
```

It checks schemas (`type` with `examples`) and parameters (`example` with a
`schema` that has a `type`); a schema without a `type`, such as a `$ref`, is
skipped. It runs with `cargo test` in `lambdas`, which the `lambdas` CI workflow
runs when the annotations change; after changing one, run `just generate`, then
the test:

```bash
cargo test -p lambdas --test openapi_examples
```
