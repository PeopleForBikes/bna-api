---
title: Every error points at its request path, and details names the field
date: 2026-11-07
updated: 2026-11-07
category: conventions
status: active
tags: [errors, jsonapi, source-pointer, http-status, middleware]
modules:
  [
    lambdas/src/core/extract.rs,
    lambdas/src/lib.rs,
    lambdas/src/core/resource/schema.rs,
  ]
spec: specs/0001-hurl-api-coverage
related: [specs/0001-hurl-api-coverage/findings.md]
superseded_by:
---

# Every error points at its request path, and details names the field

## Context

The error responses of the API were made consistent in PR #544 (decisions M-01,
M-02, and M-42 in `specs/0001-hurl-api-coverage/design.md`).

## Symptom

A rejected body field reported its location (`/country`) in `source.pointer`,
while every other error reported the request path (`/ratings/{rating_id}`), and
a 500 reported `Unknown source` with empty details.

## Root Cause

Two readings of `source.pointer` coexisted: the JSON:API one, a pointer into the
request document, and the request path that the 404s used. A database error does
not know the request it belongs to.

## Solution

The team chose one rule set:

- `source.pointer` is always the request path, `source.parameter` a query
  parameter, and `source.header` a header. `details` names the invalid field.
  Only the first invalid field is reported, because deserialization stops there.
- 400: an invalid path or query parameter, or a body that is not valid JSON.
- 415: a body sent without the JSON content type.
- 422: valid JSON whose content is rejected, including values only a check can
  reject, such as an unknown foreign key. An adaptor returns
  `ExecutionError::InvalidAttribute` with `ctx.source()`
  (`lambdas/src/lib.rs:271`, `lambdas/src/core/resource/schema.rs:438`).
- 500: the `complete_internal_error` middleware (`lambdas/src/lib.rs:325`,
  installed in `lambdas/src/main.rs:174`) answers with the request path and a
  generic message; the cause only goes to the logs.

The extractors take the path from `request_path`
(`lambdas/src/core/extract.rs:267`).

## Prevention

Each operation documents its codes with `BadRequestResponse`,
`UnsupportedMediaTypeResponse`, `UnprocessableEntityResponse`, and the others,
and the Hurl suite asserts `source.pointer` and `details` on every rejected
request. `just coverage` fails on any 500 observed. Revisit if the API adopts
JSON:API pointers into the request document, which would need one error object
per field.
