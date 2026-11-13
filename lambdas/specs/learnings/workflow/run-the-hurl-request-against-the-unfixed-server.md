---
title: Write a fix's Hurl request first, and watch it fail on the unfixed server
date: 2026-11-13
updated: 2026-11-13
category: workflow
status: active
tags: [hurl, regression-test, bugfix, bacon, test-first]
modules: [lambdas/tests, lambdas/src/core/resource]
spec: specs/0005-schemathesis
related:
  [
    specs/0005-schemathesis/findings.md,
    lambdas/specs/learnings/test-failures/hurl-parallel-files-and-list-order.md,
  ]
superseded_by:
---

# Write a fix's Hurl request first, and watch it fail on the unfixed server

## Context

`specs/0005-schemathesis` fixed four API defects that Schemathesis found, and
its Requirement 4.2 asks each fix to come with a Hurl request "that fails
without the fix and passes with it". A request written after the fix only shows
the second half.

## Symptom

A request written after the fix can pass for the wrong reason: a body the
unfixed API already rejected on another ground, or a request that never reaches
the code path of the defect. Nothing in the green run tells the difference.

## What Did Not Work

Guessing the reproducing request: the first top-level array sent to
`PATCH /cities/submissions/1` had ten elements and got `422` with "invalid
length 10, expected struct SubmissionPatch with 11 elements", a rejection for
the wrong reason. Only eleven elements reproduced the defect (`200 OK`).

## Root Cause

A regression test proves something only if it is seen to fail on the code that
has the defect, with the status the defect produces.

## Solution

For each fix, before changing the code:

1. Load the test data and keep `just debug-axum` running (bacon rebuilds and
   restarts the server when `lambdas/src/` changes).
2. Add the request to the Hurl file of the operation, with the status and the
   `details` the fixed API must return.
3. Run that file alone against the unfixed server, and check that it fails with
   the defect's status:

   ```bash
   just db-seed            # at the root of the repository
   cd lambdas/tests
   hurl --test --variables-file localhost.vars endpoints/ratings.hurl
   ```

4. Change the code, let bacon restart the server, and run the file again: it
   passes.

The requests written this way: `lambdas/tests/endpoints/ratings.hurl:230`,
`:257` and `:284` (`201` before the fix), `submissions.hurl:253` (`200`),
`cities.hurl:340` and `:350`, and `usstates.hurl:84` (`404`).

## Prevention

Write the failing status in the commit message or the findings next to the fix,
as the team's one commit per fix keeps the request and the fix together. Pick a
request that changes no data shared with other files, even before the fix
(eleven `null`s, rather than a status change): Hurl runs the files in parallel
against one database (`hurl-parallel-files-and-list-order.md`).
