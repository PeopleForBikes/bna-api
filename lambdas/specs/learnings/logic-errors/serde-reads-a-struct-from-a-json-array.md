---
title:
  serde reads a struct from a JSON array, so a body must be checked to be an
  object
date: 2026-11-13
updated: 2026-11-13
category: logic-errors
status: active
tags: [serde, deserialize, request-body, json, validation]
modules:
  [
    lambdas/src/core/extract.rs,
    lambdas/src/core/resource/schema.rs,
    lambdas/src/core/resource/ratings/schema.rs,
  ]
spec: specs/0005-schemathesis
related:
  [
    specs/0005-schemathesis/findings.md,
    lambdas/specs/learnings/conventions/error-source-is-the-request-path.md,
  ]
superseded_by:
---

# serde reads a struct from a JSON array, so a body must be checked to be an object

## Context

Schemathesis 4, added in pull request #555, sends invalid data for every
operation, arrays where the specification documents objects included. Every
request body of the API is documented as an object, and so is every nested
struct, such as the eight categories of a rating (`infrastructure`,
`recreation`, …).

## Symptom

Bodies the specification forbids were accepted:

- `POST /ratings` with `"infrastructure": [null, null]` answered `201 Created`
  and stored the rating.
- `PATCH /cities/submissions/1` with
  `[null, null, null, null, null, null, null, null, null, null, "Approved"]` as
  the whole body answered `200 OK` and approved the submission.

An array of the wrong length is rejected, with a confusing message: ten elements
gave "invalid length 10, expected struct SubmissionPatch with 11 elements".

## What Did Not Work

- Rejecting every array in a body, ruled out before trying:
  `BNAPipelinePost.sqs_message` (`entity/src/wrappers/bna_pipeline.rs:61`) holds
  arbitrary JSON, arrays included.
- A generic deserializer wrapping `serde_json`'s, so that no struct at any depth
  can be read from an array: considered, and set aside by the team as more code
  to review than the two targeted checks below.

## Root Cause

serde's derived `Deserialize` for a struct accepts two shapes: a map of its
fields, and a sequence of its fields in declaration order. `serde_json` passes a
JSON array as a sequence, so an array with one element per field, the `Option`
fields included, deserializes into the struct.

## Solution

Two checks, added in pull request #555:

- `ApiJson` answers `422` when the body is valid JSON but not an object
  (`lambdas/src/core/extract.rs:150`, with `is_object` at
  `lambdas/src/core/extract.rs:249`), with the details "the body must be a JSON
  object".
- A struct field of a body goes through `deserialize_object`
  (`lambdas/src/core/resource/schema.rs:684`), which reads a
  `serde_json::Value`, refuses anything but an object with "invalid type: …,
  expected a JSON object", then deserializes the struct from it:

  ```rust
  #[serde(deserialize_with = "deserialize_object")]
  pub(crate) infrastructure: Infrastructure,
  ```

  (`lambdas/src/core/resource/ratings/schema.rs:227` and the seven fields after
  it). The error names the field, as the error convention asks.

## Prevention

- A new request body needs nothing: `ApiJson` checks it. A new struct field in a
  body, at any depth, needs `#[serde(deserialize_with = "deserialize_object")]`;
  `RatingPost` is the only body with nested structs so far.
- The unit tests `test_is_object` (`lambdas/src/core/extract.rs:313`) and
  `test_a_category_must_be_an_object`
  (`lambdas/src/core/resource/ratings/schema.rs:363`), and the Hurl requests in
  `lambdas/tests/endpoints/ratings.hurl:284` and
  `lambdas/tests/endpoints/submissions.hurl:253`, cover the two checks.
- Schemathesis' `negative_data_rejection` check reports a body that should have
  been rejected (`just test-schemathesis` in `lambdas/tests`). It is disabled
  for the two PATCH operations
  (`specs/learnings/build-errors/openapi-schemas-that-break-client-generation.md`),
  so the Hurl request is their only guard.
