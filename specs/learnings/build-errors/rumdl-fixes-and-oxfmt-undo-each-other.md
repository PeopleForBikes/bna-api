---
title: rumdl's automatic fixes and oxfmt can undo each other
date: 2026-11-10
updated: 2026-11-10
category: build-errors
status: active
tags: [rumdl, oxfmt, markdown, lint, formatter]
modules: [.markdownlint.yml, justfile]
spec: specs/0002-rust-tooling
related: [specs/0002-rust-tooling/findings.md]
superseded_by:
---

# rumdl's automatic fixes and oxfmt can undo each other

## Context

Markdown is linted by rumdl and formatted by oxfmt. `just fmt` applies rumdl's
fixes, then formats (the `fmt-md` recipe); `just lint`, which CI runs, checks
the formatting, then lints (the `lint-md` recipe). rumdl reads
`.markdownlint.yml`, but it also has rules markdownlint does not have (MD057 and
MD062 to MD086), and some of them can fix files automatically.

## Symptom

`just lint` fails right after `just fmt`, on a file `just fmt` just wrote:

```text
specs/0002-rust-tooling/spike.md:262:1: [MD076] Unexpected blank line between list items
```

Running `just fmt` again does not help: the file ends the same way each time.

## What Did Not Work

- Letting `rumdl check --fix` repair it: it removes the blank line, then oxfmt
  puts it back, byte for byte.
- Changing the order (oxfmt first, then rumdl's fixes): the file then fails
  `oxfmt --check` instead. Whichever tool runs last wins, and the other one's
  check fails.

## Root Cause

MD076 (`list-item-spacing`) is rumdl-only. In its default `consistent` style,
every gap between the items of a list must be the same: all blank lines or none.
oxfmt, like prettier, writes a blank line after a list item that holds a block
(a fenced code block, a second paragraph) and none between the other items. The
two tools disagree about the same blank line, and both rewrite it.

The same class of conflict appeared with MD034 on a link reference definition
whose URL is on the next line: rumdl's fix wraps the URL in `<…>` and oxfmt
removes the brackets (spike §5.17 in `specs/0002-rust-tooling/spike.md`).

## Solution

Configure the rule so that it accepts what the formatter writes, in
`.markdownlint.yml:11-12`:

```yaml
list-item-spacing:
  allow-loose-continuation: true
```

rumdl then accepts the blank line around an item's continuation block, and still
reports lists that mix spacing for no reason. `rumdl config` confirms the option
is read: `allow-loose-continuation = true [from .markdownlint.yml]`. For the
MD034 case, the content changed instead: shorter reference labels keep each
definition on one line within 80 columns (`code-of-conduct.md`,
`.github/CONTRIBUTING.md`).

## Prevention

- After `just fmt`, run `just lint`: if it fails on a file `just fmt` just
  wrote, look for a rule both tools rewrite, and configure the lint rule to
  accept the formatter's output rather than disabling it.
- When adopting rumdl next to oxfmt or prettier in another repository, run
  `just fmt` then `just lint` on the whole repository before switching CI;
  rumdl-only rules are where the conflicts hide.
- Revisit when upgrading rumdl or oxfmt: new rules or new fixes can conflict in
  the same way.
