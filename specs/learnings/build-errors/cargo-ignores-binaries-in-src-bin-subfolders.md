---
title: Cargo ignores a binary in a subfolder of src/bin
date: 2026-11-07
updated: 2026-11-07
category: build-errors
status: active
tags: [cargo, binaries, targets, dead-code, lambda]
modules: [lambdas]
spec: specs/0001-hurl-api-coverage
related: [specs/0001-hurl-api-coverage/findings.md]
superseded_by:
---

# Cargo ignores a binary in a subfolder of src/bin

## Context

`lambdas/src/bin/ratings/patch-ratings-analysis.rs` was meant to be a Lambda
binary. It was removed in PR #544, since it was no longer used.

## Symptom

The file called an adaptor that no longer existed, yet every build and test
passed: it was never compiled.

## Root Cause

Cargo only discovers `src/bin/*.rs` and `src/bin/<name>/main.rs` as binaries.
Any other file under `src/bin` is ignored, unless a `[[bin]]` section names it.

## Solution

List the targets Cargo sees before trusting that a file builds:

```bash
cargo read-manifest --manifest-path lambdas/Cargo.toml
```

The `targets` of the output had only `lambdas`, `axumed`, and the example.

## Prevention

Put a binary in `src/bin/<name>.rs` or `src/bin/<name>/main.rs`, or declare it
with `[[bin]]`. Note that `cargo lambda build` builds every binary of the crate.
