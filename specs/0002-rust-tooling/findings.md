# Findings: Rust Tooling for Markdown and Spelling

Everything non-obvious learned while implementing this feature: behavior that
looks like a bug unless you know it is deliberate, traps in the libraries and
tools, and what worked. Each entry says what the rule is, how it was caught, and
what it cost.

Status: complete for tasks 1 to 10 and 9.1; tasks 9.2, 9.3 and 11 wait for the
pull request's CI run. Keep it current as the code changes.

## 1. Behavior that does not survive a naive implementation

### 1.1 `just fmt` can correct a generated file without its source

`typos --write-changes` corrects every file it reads, generated ones included
(`openapi-3.1.yaml`, `openapi-3.0.yaml`, `bnaclient/src/lib.rs`,
`docs/database.sql`). When the misspelling also sits in the source (a utoipa
description, `docs/database.dbml`), both are corrected in the same run and the
generated file still matches; this is what happened to the description at
`lambdas/src/core/resource/cities/endpoint.rs:225`, and
`just regenerate-all-no-cargo` reproduced all four copies. When a misspelling
exists only in a generated file, `just fmt` creates a drift that only the
`bnaclient` and `integration` CI jobs reveal. Nothing in this feature guards
against it: after `just fmt`, regenerate when a generated file changed.

**Caught by**: the spec review (test gaps).

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

### 2.3 rumdl writes a cache into the working tree unless told not to

`rumdl check` creates and updates `.rumdl_cache/` at the repository root (85
entries after a few runs). It stays out of `git status` only because rumdl
writes its own `.rumdl_cache/.gitignore` (`*`), so `just lint` looked clean
while it modified files, against Requirement 4.4. Both recipes now pass
`--no-cache` (`justfile:31`, `justfile:40`); without a cache rumdl still lints
the whole repository in about 10 ms.

**Caught by**: the spec review, which found the folder in the working tree.

### 2.4 `gh attestation verify` checks build provenance, not release attestations

The CI lint job of the pull request failed in its install step (PR #552, run
42965301458, exit code 1, before `just lint`). The step log could not be read
from the sandbox; the cause follows from the attestations themselves.
`gh attestation verify` looks for SLSA build provenance
(`https://slsa.dev/provenance/v1`) by default. oxfmt and typos publish only the
release attestation GitHub signs for immutable releases
(`https://in-toto.io/attestation/release/v0.2`, whose subjects are the tag and
every asset digest); rumdl publishes both. The spike and the design had called
all three "build attestations" after counting them through the attestations API
without reading their predicate type. The step now runs
`gh release verify-asset "$tag" "$archive" --repo "$repo"`
(`.github/workflows/ci-rust.yml:31`), which checks the release attestation and
works for all three: the attestations API returns one release attestation for
each of the three tags. The fix is confirmed only once the next CI run passes
(task 9.2).

**Caught by**: the failed CI step (the log itself was out of reach from the
sandbox), then the predicate type of each attestation, decoded from
`gh api repos/<repo>/attestations/sha256:<digest>`.

**Cost**: one failed CI run. When a project's attestations matter, read their
`predicateType` before choosing the verification command: a release attestation
proves the archive is the published one, while build provenance also says which
workflow built it.

## 3. Method — what actually worked

### 3.0 Negative checks and their output

Each check ran on a scratch copy of the repository (never on the branch) with
the pinned binaries; `just lint` left the copy unchanged every time. The
misspelling used was a common one typos knows, with a single correction; it is
not quoted here, since `just fmt` would correct the quote.

| Change made to the copy                            | `just lint` exit | Reported                                                                   |
| -------------------------------------------------- | ---------------- | -------------------------------------------------------------------------- |
| Heading level skip in `code-of-conduct.md`         | 1                | `code-of-conduct.md:7:1: [MD001] Expected heading level 2…`                |
| Long unformatted paragraph in `code-of-conduct.md` | 1                | oxfmt: `Format issues found in above 1 files`; rumdl and typos did not run |
| Known misspelling in `.github/CONTRIBUTING.md`     | 2 (typos)        | typos names the word, its correction and `.github/CONTRIBUTING.md:10:8`    |
| Long paragraph in `.github/CONTRIBUTING.md`        | 1                | oxfmt lists `.github/CONTRIBUTING.md`                                      |
| Heading level skip in `.github/CONTRIBUTING.md`    | 1                | `.github/CONTRIBUTING.md:10:1: [MD001] …`                                  |
| Misspelling in a code span, ordinary Markdown file | 2                | typos names the word and its correction                                    |
| Misspelling in a code span of a `spike.md`         | 0                | nothing (skipped by the `spike` file type)                                 |
| Misspelling in the prose of a `spike.md`           | 2                | typos names the word and its correction                                    |

After `just fmt` on a copy with a long paragraph and a known misspelling, the
paragraph was rewrapped, the word corrected, and `just lint` exited 0. The exit
codes of `just fmt` with a missing tool or a broken configuration are in §2.2.

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
