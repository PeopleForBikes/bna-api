---
title: Hurl runs files in parallel, and list order needs an updated row to test
date: 2026-11-07
updated: 2026-11-07
category: test-failures
status: active
tags: [hurl, parallel, flaky, pagination, order-by]
modules: [lambdas/tests, lambdas/src/core/resource]
spec: specs/0001-hurl-api-coverage
related: [specs/0001-hurl-api-coverage/findings.md]
superseded_by:
---

# Hurl runs files in parallel, and list order needs an updated row to test

## Context

`just test` runs every Hurl file with `hurl --test` (the `test` recipe of
`lambdas/tests/justfile`), which runs the files in parallel against one
database.

## Symptom

- A count compared between two requests is racy: another file may create a row
  in between.
- A list without `ORDER BY` returns the same order run after run, so a test of
  its order passes, until a row is updated.

## Root Cause

The files share the database. PostgreSQL returns rows in physical order when no
order is given, and an `UPDATE` writes a new version of the row elsewhere.

## Solution

- Assert what other files cannot change: a full page instead of a total
  (`lambdas/tests/endpoints/cities.hurl:116`), and run-unique data
  (`{{newUuid}}`).
- To test the order of a list, update a row first, even with the same value,
  then check its place (`lambdas/tests/scenario/pagination.hurl:250`). Every
  list orders by a unique key, or uses one as a tie-breaker
  (`lambdas/src/core/resource/cities/db.rs:45`).

## Prevention

Run each file alone and the whole suite twice in a row before committing. Avoid
assertions on values another file can change.
