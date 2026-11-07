---
title: The Country enum and the country table must hold the same values
date: 2026-11-07
updated: 2026-11-07
category: conventions
status: active
tags: [country, enum, migration, lookup-table, openapi]
modules:
  [
    lambdas/src/core/resource/schema.rs,
    migration/src,
    lambdas/src/core/resource/cities,
  ]
spec: specs/0001-hurl-api-coverage
related: [specs/0001-hurl-api-coverage/findings.md]
superseded_by:
---

# The Country enum and the country table must hold the same values

## Context

The API documents the countries it accepts with the `Country` enum
(`lambdas/src/core/resource/schema.rs:18`), while the database checks them
against the `country` lookup table, filled by the migrations
(`migration/src/m20220101_000001_main.rs:48`).

## Symptom

The enum listed 204 countries and the table 27, so `POST /cities` with a
documented country such as `Albania` returned 422
(`lambdas/src/core/resource/cities/adaptor.rs:91`).

## Root Cause

Nothing ties the two lists together: the enum generates the specification and
validates the reads (`lambdas/src/core/resource/cities/mod.rs:25`), and the
table backs the foreign keys and the creation check.

## Solution

A new migration added the 177 missing countries
(`migration/src/m20261107_200000_all_countries.rs:4`), skipping any that a
database already has (`migration/src/m20261107_200000_all_countries.rs:194`).
The lists are now equal.

## Prevention

Adding or renaming a country takes both a new migration and the enum, in the
same change. To compare them, load the dump and diff
`SELECT name FROM public.country` with the serialized names of the enum.
