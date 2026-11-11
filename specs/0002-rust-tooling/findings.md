# Findings: Rust Tooling for Markdown and Spelling

<!-- cspell:words oxfmt rumdl -->

Everything non-obvious learned while implementing this feature: behavior that
looks like a bug unless you know it is deliberate, traps in the libraries and
tools, and what worked. Each entry says what the rule is, how it was caught, and
what it cost.

Status: complete for tasks 1 to 10 and 9.1; tasks 9.2, 9.3 and 11 wait for the
pull request's CI run. Keep it current as the code changes.

## 1. Behavior that does not survive a naive implementation

None found: the behavior matched the specification once the tools were
configured (section 2).

## 2. Library and tooling traps

### 2.1 rumdl's MD076 and oxfmt fight over the blank line after a block item

rumdl adds rules markdownlint does not have (MD057 and MD062 to MD086). One of
them, MD076 (`list-item-spacing`), asks in its default `consistent` style that
every gap between the items of a list be the same: all blank lines or none.
oxfmt, like prettier, writes a blank line after an item that holds a block (a
fenced code block, a second paragraph) and none between the other items. On such
a list, `rumdl check --fix` removes the blank line, oxfmt puts it back, and
`just fmt` always ends in a state `just lint` rejects. The fix is one option in
`.markdownlint.yml:11-12` (`list-item-spacing: allow-loose-continuation: true`),
which rumdl reads like the rest of the file (`rumdl config` shows
`allow-loose-continuation = true [from .markdownlint.yml]`).

**Caught by**: the first `just lint-md` with rumdl (task 5.2):

```text
specs/0002-rust-tooling/spike.md:262:1: [MD076] Unexpected blank line between list items
```

On a scratch copy, `rumdl check --fix` removed the line and `oxfmt` restored it,
byte for byte.

**Cost**: none in the end, but the spike missed it: its auto-fix test (spike
§5.17) ran on the repository at `b9ea034`, where no list had this shape. The
first such list (in `spike.md` itself) broke `just lint`. Any rumdl-only rule
with an automatic fix can conflict with the formatter in the same way; when
`just lint` fails right after `just fmt`, look for a rule both tools rewrite.

### 2.2 A fixer must not hide a missing tool or a broken configuration

`just fmt` has to keep going when a fixer leaves work for a person, but it must
stop on a real error. The first version used just's `-` prefix on
`typos --write-changes` and `rumdl check --fix`, which ignores every non-zero
exit: without typos on the `PATH`, `just fmt` printed `command not found` and
still exited 0. The recipes now accept only the documented code (`justfile:26`,
`justfile:31`): `|| [ $? -eq 2 ]` for typos, `|| [ $? -eq 1 ]` for rumdl.

A second trap sat behind the first: when rumdl finds `.markdownlint.yml` by
itself and cannot parse it, it ignores it silently, uses its defaults and exits
0 (`rumdl config` then shows every setting `[from default]`). Given the file
with `--config .markdownlint.yml`, it fails with
`Config error: Failed to parse config` and exits 2, so both recipes pass it
explicitly. oxfmt (exit 1) and typos (exit 78) already fail on a broken
configuration file.

**Caught by**: the spec review, then exit codes measured on scratch files with
the pinned binaries: typos 0 clean, 2 misspellings left, 78 broken config; rumdl
0 clean, 1 findings left, 2 broken config given with `--config`, 0 broken config
found by itself; a missing command 127.

**Cost**: before the fix, a contributor without typos or with a YAML mistake in
`.markdownlint.yml` would get a green `just fmt` that fixed nothing (or linted
with the wrong rules); only `just lint` or CI would show it. Verified after the
fix on a scratch copy: `just fmt` exits 1 without typos, 1 with a broken
`.markdownlint.yml` or `_typos.toml`; with only unfixable findings left it exits
0 and `just lint` exits 1.

## 3. Method — what actually worked

### 3.1 Compare the new formatter with the old one before switching

Before running oxfmt on the repository, `prettier --check` listed the only file
prettier would change (`specs/0001-hurl-api-coverage/review.md`). After
`just fmt-md`, that file matched prettier's own output byte for byte (`cmp`
against a scratch copy formatted by prettier 3.9.10), and `prettier --check`
passed on every Markdown file. This proves the switch reformats nothing beyond
what the old tool would have.

### 3.2 List the CI archives before the first CI run

The design took the binary names inside the Linux archives from the macOS
archives of the same releases, leaving the first CI run to confirm them.
Downloading the three Linux archives with `gh release download` and listing them
with `tar -tzf` settled it locally in seconds (`oxfmt-x86_64-unknown-linux-gnu`,
`rumdl`, `./typos`), and their SHA-256 digests matched the attested ones. Do the
same after a version bump, before pushing the workflow change.
