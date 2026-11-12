---
title: mise can skip its integrity checks, locally and in CI
date: 2026-11-12
updated: 2026-11-12
category: security-issues
status: active
tags: [mise, attestation, checksum, mise-action, cache, supply-chain]
modules: [mise.toml, mise.lock, .github/workflows]
spec: specs/0003-tool-manager
related:
  [
    specs/0003-tool-manager/findings.md,
    specs/learnings/integration-issues/mise-missing-tool-falls-back-to-path.md,
    specs/learnings/integration-issues/mise-shims-need-the-mise-binary-in-a-sandbox.md,
  ]
superseded_by:
---

# mise can skip its integrity checks, locally and in CI

## Context

mise installs the pinned command-line tools from `mise.toml` and `mise.lock`,
for contributors and in CI through `jdx/mise-action` (PR #553). `mise lock`
records, for each release archive, its URL, its checksum and, for the tools
whose GitHub releases carry one, `provenance = "github-attestations"`. The point
of the setup is that every install is checked against those records.

## Symptom

Nothing fails, which is the problem. Two installs that look verified are not:

- **Locally**: `mise install --verbose` shows the `checksum` step for each
  release archive and no attestation step at all, even though `mise.lock`
  records the provenance.
- **In CI**: mise-action's log shows `Restoring mise cache` and, on a later run
  with the same key, `mise install --locked` finds the tools installed and
  downloads nothing. The first run of the pull request shows
  `mise cache not found for mise-v1-linux-x64-…` then
  `Cache saved from /home/runner/.local/share/mise`.

## What Did Not Work

- Relying on the lockfile alone: recording `provenance` in `mise.lock` does not
  make an install check it again.
- Relying on `--locked` alone: it guarantees mise uses the recorded URL and
  checksum, not that it verifies anything when the tool is already installed.

## Root Cause

- When a `mise.lock` entry has a checksum, `mise install` compares the download
  with it and skips the attestation it verified at lock time. Only
  `locked_verify_provenance` makes it verify the recorded provenance again. With
  it, the verbose log shows `✓ GitHub artifact attestations verified` for each
  attested tool.
- `jdx/mise-action` has `cache: true` by default: it saves mise's data directory
  to the GitHub Actions cache, under a key that changes with `mise.toml` and
  `mise.lock`, and restores it on later runs. A restored tool is already
  installed, so nothing is downloaded or checked: the checksums and attestations
  are verified only by the run that wrote the cache entry, and an entry written
  by a compromised step would be trusted.

## Solution

- `mise.toml:16`: `locked_verify_provenance = true`, next to `locked = true`
  (`mise.toml:14`).
- Every mise-action step sets `cache: false`
  (`.github/workflows/ci-rust.yml:19`, `.github/workflows/bnaclient.yml:31`,
  `.github/workflows/integration.yml:46`,
  `.github/workflows/integration.yml:142`). The cost, measured on PR #553: about
  1 s per job for the release tools, and 72.5 to 99.7 s in the bnaclient job,
  which builds cargo-progenitor from crates.io at every run.

## Prevention

- Read the verbose install log once when setting up a tool manager: look for
  each check you expect, not only for success.
- When adopting mise in another repository (brokenspoke-analyzer may follow),
  set `locked_verify_provenance = true` and `cache: false` from the start.
- Revisit when upgrading mise or mise-action: if mise starts verifying
  provenance by default, or the action verifies restored tools, the settings may
  become unnecessary; check the verbose log again before removing them.
