---
title: mise's shims need the mise executable, so a sandbox must read it too
date: 2026-11-12
updated: 2026-11-17
category: integration-issues
status: active
tags: [mise, shims, nono, sandbox, agents, home]
modules: [justfile, .github/CONTRIBUTING.md]
spec: specs/0003-tool-manager
related:
  [
    specs/0003-tool-manager/findings.md,
    specs/learnings/security-issues/mise-can-skip-its-integrity-checks.md,
    specs/learnings/integration-issues/mise-missing-tool-falls-back-to-path.md,
    specs/learnings/integration-issues/docker-in-the-nono-sandbox.md,
    lambdas/specs/0000-api-request-collection/findings.md,
    specs/learnings/integration-issues/mise-lock-inside-nono-lock-the-tool-by-name.md,
  ]
superseded_by:
---

# mise's shims need the mise executable, so a sandbox must read it too

## Context

The `nono-claude-danger` recipe of the root `justfile` starts Claude in the nono
sandbox with the tools mise installed outside it. Until 2026-11-17 the recipe
put mise's shims directory ahead of the `PATH` and granted read access to mise's
data and configuration directories and write access to its state directory.
Since then (`specs/0003-tool-manager`, task 9) each contributor's nono profile
holds these grants and the recipe passes only the profile.

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

Grant the mise executable as a single file, not the folder around it. Since
2026-11-17 the contributor's nono profile does it, as `.github/CONTRIBUTING.md`
("Agents in the nono sandbox") explains: Homebrew's mise is readable through the
`/opt/homebrew` grant of the base profile, and a mise in `~/.local/bin` needs

```json
"read_file": ["~/.local/bin/mise"]
```

Until then, the recipe granted the `mise` found on the caller's `PATH`, and the
file it resolves to when it is a link, each as a single file:

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

## Also: a different `HOME` makes the shims distrust `mise.toml`

_Added 2026-11-14, from `lambdas/specs/0000-api-request-collection` (findings
§2.2)._ Inside the sandbox, Slumber cannot write its history under
`~/Library/Application Support/slumber/`, so its checks ran it with `HOME` set
to a scratch folder. Through the shim, it failed before Slumber started:

```text
mise ERROR Config files in .../bna-api/mise.toml are not trusted.
Trust them with `mise trust`.
```

mise keeps its trust records in its state directory, by default under the home
folder (`~/.local/state/mise/trusted-configs/`, which holds this checkout's
entry); with another `HOME`, it looks elsewhere and finds the checkout
untrusted. Calling the installed program keeps the override for the tool only:

```bash
HOME=<scratch> "$(mise which slumber)" --file lambdas/slumber.yml collection
```

`mise which slumber` gives
`~/.local/share/mise/installs/cargo-slumber/5.4.4/bin/slumber`; the checks of
that feature passed this way. Pointing `MISE_STATE_DIR` at the real state
directory might also work; not tried.

## Prevention

- When a sandbox must run tools through shims or wrappers (mise, asdf, pyenv),
  grant what the shim executes, not only the directories of the tools: check
  with `ls -l` on a shim.
- Test the sandbox setup with more than one install method of the tool manager.
  The `read_file` entry for a mise in `~/.local/bin` is still untested.
- When a command needs another `HOME` inside the sandbox, run the tool through
  `"$(mise which <tool>)"`, not through its shim.
