---
title: oxfmt can corrupt wrapped code spans and reflows YAML in Markdown
date: 2026-11-14
updated: 2026-11-14
category: build-errors
status: active
tags: [oxfmt, markdown, formatter, code-span, yaml, documentation]
modules: [justfile, lambdas/README.md]
spec: lambdas/specs/0000-api-request-collection
related:
  [
    lambdas/specs/0000-api-request-collection/findings.md,
    specs/learnings/build-errors/rumdl-fixes-and-oxfmt-undo-each-other.md,
  ]
superseded_by:
---

# oxfmt can corrupt wrapped code spans and reflows YAML in Markdown

## Context

`just fmt` formats every Markdown file with oxfmt and `just lint` checks it
(`justfile:36`, `justfile:48`; oxfmt 0.78.0, `mise.toml:21`).
`lambdas/specs/0000-api-request-collection` wrote long task lists with commands
in code spans, and a README example with a long YAML value.

## Symptom

Two changes after formatting, both passing `just lint`:

- A code span that crossed a line break inside an indented list item came out
  joined with the indentation kept inside it, twice in the feature's `tasks.md`:
  `` `slumber --file lambdas/slumber.yml     collection` `` and
  `` `just compose-down && just compose-up &&     just db-seed` ``. The command
  readers copy now holds a run of spaces.
- In the README's fenced `yaml` example, the one-line `url:` of a chained recipe
  was folded into a double-quoted scalar over five lines.

## What Did Not Work

- Relying on the linters: rumdl 0.3.2 (MD038 only reports spaces at the edges of
  a span) and `oxfmt --check` both pass the corrupted span (checked on a
  reproduction file).

## Root Cause

Reproduced with oxfmt 0.78.0 on a scratch file:

- A span split over two lines of a list item whose continuation lines are
  indented deeper than its content loses its line break, but not the
  indentation:

  ```text
  before: - [ ] 2.2 Check loading and discovery: `slumber --file lambdas/slumber.yml
                  collection` exits 0 and shows one profile `local`, default, with the
  after:  - [ ] 2.2 Check loading and discovery:
                `slumber --file lambdas/slumber.yml       collection` exits 0 and shows
  ```

  The same split span in a plain paragraph is joined with one space, and a span
  written on one line is moved to the next line whole: both are correct.

- oxfmt formats the YAML of fenced `yaml` blocks like any YAML file, so long
  values are folded. A double-quoted scalar folded at existing spaces keeps its
  value: YAML joins the lines with one space. Slumber parsed the README's folded
  `url` to the original one-line string exactly.

## Solution

- The two spans were rewritten as short spans on one line each (the feature's
  `tasks.md`, tasks 2.2 and 9.3).
- The README example was kept as oxfmt formats it, after re-testing it: a script
  took the `yaml` block from `lambdas/README.md`, put it in place of the whole
  `requests:` mapping of a scratch copy of `lambdas/slumber.yml`, and Slumber
  loaded it and sent its requests (`200`, `201`). Extracting the block tests
  what readers copy; retyping it would have tested something else.

## Prevention

- Keep each code span on one line in the source; break the sentence around it,
  not inside it.
- After `just fmt`, look for runs of spaces inside code spans, which no linter
  reports:

  ```bash
  grep -nE '`[^`]*[^ |]  +[^ |][^`]*`' path/to/*.md
  ```

  (table rows can match it harmlessly through their padding).

- Test configuration examples kept in documentation after formatting, by
  extracting them from the file rather than retyping them.
