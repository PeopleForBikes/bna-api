---
title: A pinned tool mise has not installed runs whatever copy is on the PATH
date: 2026-11-12
updated: 2026-11-12
category: integration-issues
status: active
tags: [mise, auto-install, shims, path, version-drift]
modules: [mise.toml, .github/CONTRIBUTING.md]
spec: specs/0003-tool-manager
related:
  [
    specs/0003-tool-manager/findings.md,
    specs/learnings/security-issues/mise-can-skip-its-integrity-checks.md,
    specs/learnings/integration-issues/mise-shims-need-the-mise-binary-in-a-sandbox.md,
  ]
superseded_by:
---

# A pinned tool mise has not installed runs whatever copy is on the PATH

## Context

`mise.toml` pins the command-line tools and sets `auto_install = false`
(`mise.toml:12`), so that no command installs a tool behind the contributor's
back (by default, `mise exec` installs every missing tool of the configuration,
compiling the cargo ones). The expectation was that a pinned tool that is not
installed makes the command fail.

## Symptom

After a pull that adds or bumps a tool in `mise.toml`, the command still works,
with another version:

```text
mise WARN  github:rvben/rumdl@0.3.2 is not installed and auto-install is disabled, so mise looks for rumdl on PATH instead. Install it with: mise install github:rvben/rumdl@0.3.2
```

`mise exec -- rumdl --version` then runs the first `rumdl` on the `PATH` and
exits 0. Through the shims there is not even a warning: the shell runs the next
`rumdl` on the `PATH` silently.

## What Did Not Work

- Looking for a strict setting: `mise settings ls --all` lists
  `exec_auto_install` and `not_found_auto_install`, which only control
  installing, not failing.

## Root Cause

- With `auto_install = false`, mise falls back to the `PATH` instead of failing;
  it fails only when there is no other copy (`"rumdl" couldn't exec process`,
  exit 1).
- mise creates shims for installed tools only, so a tool that is not installed
  has no shim, and the shims directory, even first on the `PATH`, does not stop
  the shell from finding another copy (`command not found`, 127, only when there
  is none).

## Solution

The team accepted mise's behavior rather than adding a guard (requirements 2.7
and 2.8 of `specs/0003-tool-manager/requirements.md` describe it), and
documented the habit that avoids it: `.github/CONTRIBUTING.md:53-55` says to run
`mise install` again after pulling a change to `mise.toml`.

## Prevention

- After pulling a change to `mise.toml`, run `mise install`. `mise ls --missing`
  lists what is not installed.
- When a lint or test result differs from CI, check the version first:
  `mise ls --current` shows the pinned versions and whether each is installed.
- Revisit if mise adds a setting that fails on a missing pinned tool: a guard
  would then be one line in `mise.toml`.
