---
title:
  Docker health checks never overlap; compose up --wait fails when unhealthy
date: 2026-11-14
updated: 2026-11-14
category: integration-issues
status: active
tags: [docker, compose, healthcheck, postgres, wait]
modules: [docker-compose.yml, justfile]
spec: lambdas/specs/0000-api-request-collection
related:
  [
    lambdas/specs/0000-api-request-collection/findings.md,
    specs/learnings/integration-issues/docker-in-the-nono-sandbox.md,
  ]
superseded_by:
---

# Docker health checks never overlap; compose up --wait fails when unhealthy

## Context

`just compose-up` returned as soon as the containers started, so
`just compose-up && just db-seed` failed while PostgreSQL was still starting.
`lambdas/specs/0000-api-request-collection` (Requirement 5) gave the `postgres`
service a health check and made the recipe wait for it:

- `docker-compose.yml:10-15`: `pg_isready -d postgres -U postgres`, with
  `interval: 10s`, `timeout: 60s`, `retries: 6`, `start_period: 10s`.
- `justfile:173-174`: `compose-up` runs `docker compose up -d --wait`.

## Symptom

The spec review flagged `timeout: 60s` above `interval: 10s` as "a probe that
hangs can overlap the next one", and the implementation predicted the failure
case at "about 70 s"; both were wrong.

## What Did Not Work

- Reading `interval` as a fixed period between check starts: it would make a
  long `timeout` overlap checks, which Docker does not do.
- Counting the first counted failure one `interval` after the start period: it
  over-estimated the time to `unhealthy` by one interval (70 s instead of 60 s).

## Root Cause

The Dockerfile `HEALTHCHECK` reference (the same semantics as Compose's
`healthcheck`; in `docker/docs`,
`_vendor/github.com/moby/buildkit/frontend/dockerfile/docs/reference.md`):

- A check runs `interval` seconds after the previous check _completes_, so
  checks never overlap; a check running longer than `timeout` counts as failed.
- During `start_period`, checks run every `start_interval` (default 5 s) and
  their failures do not count; one success makes the container `healthy`.
- `retries` consecutive failures make it `unhealthy`.

`docker compose up --wait` waits for the services to be "running|healthy" and
implies detached mode
(`_vendor/github.com/docker/compose/v5/docs/reference/compose_up.md:67`).

## Solution

The values were kept, and measured with the user's runs (2026-11-14):

- Fresh volume: `just compose db-seed` showed
  `Container bna-api-postgres-1 Healthy 5.2s`, and `db-seed` connected at once.
- Always-failing check, with a Compose override file setting
  `test: ["CMD", "false"]`:

  ```bash
  docker compose -f docker-compose.yml -f <override> up -d --wait --wait-timeout 180
  ```

  It printed `container bna-api-postgres-1 is unhealthy` and exited 1 after 60.9
  s: the start period ends at 10 s, then six counted failures 10 s apart.

So a ready database is about one start interval away, and a database that never
starts makes `compose-up` fail after roughly `start_period + retries × interval`
(plus the duration of each failed check, up to `timeout`).

## Prevention

- Estimate a health check with that formula, not
  `retries × (interval + timeout)`; a long `timeout` only slows the detection of
  a hung check.
- Test the failure path with an override file whose check always fails, and
  `--wait-timeout` as a guard, so that a wait that never ends shows up as the
  guard's duration instead of a hang.
- The nono sandbox denies Docker: the user runs these recipes
  (`specs/learnings/integration-issues/docker-in-the-nono-sandbox.md`).
