---
title: A SeaORM update with nothing to change reports the row as not found
date: 2026-11-07
updated: 2026-11-07
category: runtime-errors
status: active
tags: [sea-orm, update, patch, 404, active-model]
modules:
  [
    lambdas/src/core/resource/pipelines/adaptor.rs,
    lambdas/src/core/resource/cities/adaptor.rs,
  ]
spec: specs/0001-hurl-api-coverage
related: [specs/0001-hurl-api-coverage/findings.md]
superseded_by:
---

# A SeaORM update with nothing to change reports the row as not found

## Context

The `PATCH` operations turn a partial body into an `ActiveModel` and call
`update` (SeaORM 2.0.4).

## Symptom

`PATCH` of an unknown pipeline or submission returned 500 "Database error", and
mapping `DbErr::RecordNotUpdated` to 404 still failed for an empty body (`{}`).

## Root Cause

When the active model has no field to set, SeaORM does not run an `UPDATE`: it
looks the row up instead, and reports a missing row as `DbErr::RecordNotFound`
rather than `DbErr::RecordNotUpdated`.

## Solution

Map both errors to 404, as in
`lambdas/src/core/resource/pipelines/adaptor.rs:91` and
`lambdas/src/core/resource/cities/adaptor.rs:184`:

```rust
Err(DbErr::RecordNotUpdated | DbErr::RecordNotFound(_)) => Err(ExecutionError::NotFound(..)),
```

## Prevention

For each `PATCH`, test an unknown identifier both with a body and with `{}`
(`lambdas/tests/endpoints/pipelines.hurl`,
`lambdas/tests/endpoints/submissions.hurl`).
