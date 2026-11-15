---
title: A full mise lock fails inside nono; lock the changed tool by name
date: 2026-11-14
updated: 2026-11-14
category: integration-issues
status: active
tags: [mise, lockfile, nono, sandbox, attestation, agents]
modules: [mise.toml, mise.lock, .github/CONTRIBUTING.md]
spec: lambdas/specs/0000-api-request-collection
related:
  [
    lambdas/specs/0000-api-request-collection/findings.md,
    specs/learnings/security-issues/mise-can-skip-its-integrity-checks.md,
    specs/learnings/integration-issues/mise-shims-need-the-mise-binary-in-a-sandbox.md,
  ]
superseded_by:
---

# A full mise lock fails inside nono; lock the changed tool by name

## Context

Adding or bumping a tool follows "Bumping a tool" in `.github/CONTRIBUTING.md`:
change `mise.toml`, then regenerate the lockfile for the four platforms
(`.github/CONTRIBUTING.md:71-77`). `lambdas/specs/0000-api-request-collection`
did it from an agent session inside the nono sandbox, to add
`"cargo:slumber" = "5.4.4"`.

## Symptom

`mise lock --platform linux-x64,linux-arm64,macos-arm64,macos-x64` (mise
2026.11.8) re-resolved every tool of `mise.toml` and stopped on the first GitHub
tool whose attestation it verifies:

```text
mise ERROR failed to resolve github:crate-ci/typos for linux-arm64: lock-time provenance verification failed
```

Exit 1, and `mise.lock` was left unchanged.

## What Did Not Work

- The documented command as is: it re-verifies the attestations of all the
  GitHub tools (`locked_verify_provenance = true`, `mise.toml:16`), and that
  check fails inside the sandbox.

## Root Cause

Most likely the Sigstore verification mise cannot set up inside nono (not
confirmed: the run was not repeated with `--verbose`). The tool manager spike
hit that wall at install time (`specs/0003-tool-manager/spike.md:353`), which is
why contributors install outside the sandbox:

```text
failed to verify attestation for tag oxfmt_v0.78.0: no valid Sigstore verifiers could be initialized
```

`mise lock` without arguments locks every configured tool, so one unverifiable
tool fails the whole run.

## Solution

Name the tool, so that mise resolves only it:

```bash
mise lock cargo:slumber --platform linux-x64,linux-arm64,macos-arm64,macos-x64
```

It checked the four targets, exited 0, and added only the new entry
(`mise.lock:20-23`), shaped like the other cargo tools; no other entry changed
(reviewed with `git diff mise.lock`). A cargo tool needs no attestation, since
mise builds it from crates.io.

## Prevention

- Inside the sandbox, lock the changed tool by name (`mise lock <tool>`) and
  review the diff of `mise.lock`. Outside the sandbox, the documented command
  remains the one to run.
- A GitHub tool cannot be locked from inside the sandbox this way either: its
  entry needs the attestation check. Not tried; run `mise lock` outside nono for
  those.
