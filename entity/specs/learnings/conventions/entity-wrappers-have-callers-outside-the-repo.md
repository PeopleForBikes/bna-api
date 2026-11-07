---
title: Entity wrappers without a caller here are not dead code
date: 2026-11-07
updated: 2026-11-07
category: conventions
status: active
tags: [entity, wrappers, dead-code, cleanup, lambda]
modules: [entity/src/wrappers]
spec: specs/0001-hurl-api-coverage
related: [specs/0001-hurl-api-coverage/findings.md]
superseded_by:
---

# Entity wrappers without a caller here are not dead code

## Context

A cleanup in PR #544 removed the types of `entity/src/wrappers` that had no
caller in the workspace, such as `wrappers/bna.rs`, `CityPatch`, `BnaRegion`,
and `BikeLaneType`.

## Symptom

The workspace built and every test passed, yet the team reported that Lambda
functions outside this repository use these types.

## What Did Not Work

- Searching the workspace for callers: the callers are in other repositories, so
  a search here cannot see them.

## Root Cause

`entity` is shared: other repositories depend on it through git, even though it
is not published (`publish = false`).

## Solution

The wrappers were restored at their original paths
(`entity/src/wrappers/mod.rs:80`, `entity/src/wrappers/mod.rs:109`,
`entity/src/wrappers/city.rs:38`), and `CLAUDE.md:69` states the rule: do not
remove a wrapper without checking with the team.

## Prevention

Treat every public item of `entity::wrappers` as public API: a removal or a
rename is a breaking change for those Lambda functions, to agree with the team
and list in the release notes. The `effortless` crate was a different case: the
team confirmed it had no outside user before it was removed.
