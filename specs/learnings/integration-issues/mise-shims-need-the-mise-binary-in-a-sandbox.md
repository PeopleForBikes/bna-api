---
title: mise's shims need the mise executable, so a sandbox must read it too
date: 2026-11-12
updated: 2026-11-12
category: integration-issues
status: active
tags: [mise, shims, nono, sandbox, agents]
modules: [justfile]
spec: specs/0003-tool-manager
related:
  [
    specs/0003-tool-manager/findings.md,
    specs/learnings/security-issues/mise-can-skip-its-integrity-checks.md,
    specs/learnings/integration-issues/mise-missing-tool-falls-back-to-path.md,
    specs/learnings/integration-issues/docker-in-the-nono-sandbox.md,
  ]
superseded_by:
---

# mise's shims need the mise executable, so a sandbox must read it too

## Context

The `nono-claude` and `nono-claude-danger` recipes start Claude in the nono
sandbox with the tools mise installed outside it: they put mise's shims
directory ahead of the `PATH` and grant read access to mise's data and
configuration directories and write access to its state directory
(`justfile:199-221`).

## Symptom

None for the maintainer, whose session passed `just lint`. Reviewing PR #553
against its spec found that the recipes would fail for a contributor who
installed mise with its own installer, which puts it in `~/.local/bin/mise`:
every tool run through a shim would be denied in the sandbox, so `just lint`
would fail.

## What Did Not Work

- Granting mise's directories only: they hold the installed tools and the shims,
  but not the program the shims run.

## Root Cause

Each file in mise's shims directory is a symbolic link to the `mise` executable
that created it:

```text
bacon -> /opt/homebrew/bin/mise
```

Running a tool through a shim therefore reads and runs `mise` first. The
maintainer's mise came from Homebrew, and the `pfb-claude` profile already reads
`/opt/homebrew`; `~/.local/bin` is not granted.

## Solution

Both recipes grant the `mise` found on the caller's `PATH`, and the file it
resolves to when it is a link, each as a single file (`justfile:202`,
`justfile:208`, and the same at `justfile:214` and `justfile:220`):

```just
mise_bin="$(command -v mise || true)"; PATH="{{ mise_data }}/shims:$PATH" nono run \
…
${mise_bin:+--read-file "$mise_bin" --read-file "$(realpath "$mise_bin")"} \
```

`--read-file` grants the one program, not `~/.local/bin` and whatever else it
holds. Without `mise` on the `PATH` the recipe adds no grant and still starts
the sandbox. just's `which()` would need `set lists` (just 1.59.0), hence
`command -v` in the shell. Verified with a stand-in `nono` printing its
arguments (Homebrew link and target, a plain file, no mise) and in a real
session with Homebrew's mise; a session with mise in `~/.local/bin` was not run.

## Prevention

- When a sandbox must run tools through shims or wrappers (mise, asdf, pyenv),
  grant what the shim executes, not only the directories of the tools: check
  with `ls -l` on a shim.
- Test sandbox recipes with more than one install method of the tool manager.
