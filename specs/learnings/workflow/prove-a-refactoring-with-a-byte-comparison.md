---
title: Prove a refactoring changed nothing with a byte comparison
date: 2026-11-07
updated: 2026-11-07
category: workflow
status: active
tags: [refactoring, openapi, bnaclient, test-dump, verification]
modules:
  [
    openapi-3.1.yaml,
    openapi-3.0.yaml,
    bnaclient,
    lambdas/tests/database-with-test-data.sql,
  ]
spec: specs/0001-hurl-api-coverage
related: [specs/0001-hurl-api-coverage/findings.md]
superseded_by:
---

# Prove a refactoring changed nothing with a byte comparison

## Context

PR #544 moved the API error types out of the `effortless` crate, removed unused
code and dependencies, and converted the test baseline from `COPY` to `INSERT`.

## Symptom

The question for each change: did anything change for clients or for the test
data?

## Root Cause

The specification, `bnaclient`, and the test dump are generated, deterministic
artifacts. If a change leaves them byte-identical, it cannot have changed what
they describe.

## Solution

Copy the artifacts, regenerate them, and compare:

```bash
cp openapi-3.1.yaml /tmp/31.yaml
just generate
cmp openapi-3.1.yaml /tmp/31.yaml && git diff --quiet bnaclient openapi-3.0.yaml
```

For the test data, regenerate the dump (`just dump-test-data` in
`lambdas/tests`) and check that `git status database-with-test-data.sql` shows
no change.

## Prevention

Separate a pure move from any behavior change: commit the move when the
comparison passes, then the change, with its own diff of the artifacts. It only
covers what the artifacts describe, so still run the tests.
