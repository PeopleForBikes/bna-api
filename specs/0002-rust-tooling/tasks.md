# Implementation Plan: Rust Tooling for Markdown and Spelling

<!-- cspell:words brokenspoke oxfmt oxfmtrc rumdl softwaretechnik -->

## Overview

The work goes from the least to the most visible change, one commit per task, so
each step can be checked on its own:

1. Pin the generators and remove the `sql2dbml` recipes (no tool change).
2. Prepare the content the new tools would otherwise reject (two link labels).
3. Switch formatting to oxfmt, then linting to rumdl, then spelling to typos,
   each with its configuration, its `justfile` recipes and its clean-up.
4. Fix the typos (mostly with `just fmt` itself).
5. Wire the `fmt` and `lint` meta recipes, then CI, then `CLAUDE.md`.

Prerequisites on the implementer's machine: oxfmt 0.78.0, rumdl 0.3.2 and typos
1.52.1 on the `PATH` (from their GitHub releases), Node.js for the remaining
`npx` generators, a Rust toolchain and `cargo progenitor` 0.16.0 for
`just regenerate-all-no-cargo`. The spec documents never spell out the
misspellings they fix (see `design.md`); tasks name the file and line instead.

## Tasks

- [x] 1. Pin the remaining `npx` packages
  - In `justfile` (`dbml-sql-print`, line 91), call
    `npx -y --package=@dbml/cli@12.0.0 dbml2sql`.
  - In `justfile` (`dbml-svg`, line 144), call
    `npx -y --package=@softwaretechnik/dbml-renderer@1.0.31 dbml-renderer`.
  - Verify: `just dbml-sql` and `just dbml-svg` leave `git status` unchanged
    (`docs/database.sql` and `docs/database.svg` byte-identical).
  - _Requirements: 6.1, 6.2, 6.3_

- [x] 2. Remove the `sql2dbml` recipes
  - In `justfile`, delete `db-to-dbml` (line 47) and `dbml-from-sql` (line 139),
    with their comments. Keep `db-dump`.
  - In `CLAUDE.md` (line 79), keep the rule "do not regenerate the `.dbml` from
    the database" but drop the reference to the removed `db-to-dbml` recipe.
  - Verify: `just --list` shows neither recipe; `git grep -n sql2dbml` finds
    only `specs/`.
  - _Requirements: 7.1_

- [x] 3. Shorten the two long link reference labels
  - `code-of-conduct.md`: `[BNA Mechanics Code of Conduct][code of conduct]` and
    `[code of conduct]: https://peopleforbikes.github.io/code-of-conduct/` on
    one line.
  - `.github/CONTRIBUTING.md`:
    `[BNA Mechanics Contributing Guidelines][contributing guidelines]` and
    `[contributing guidelines]: https://peopleforbikes.github.io/contributing/`
    on one line.
  - Verify: `rumdl check code-of-conduct.md .github/CONTRIBUTING.md` reports no
    issue, and both links still render to the same URLs.
  - _Requirements: 2.1, 4.3_

- [x] 4. Format Markdown with oxfmt
  - [x] 4.1 Add `.oxfmtrc.json` at the root with `printWidth: 80` and
        `proseWrap: "always"` (content in `design.md`).
    - _Requirements: 1.1_
  - [x] 4.2 In `justfile`, make `fmt-md` run `oxfmt "**/*.md"`, and make
        `lint-md` start with `oxfmt --check "**/*.md"`.
    - _Requirements: 1.1, 1.3, 1.4_
  - [x] 4.3 Delete `.prettierignore`.
    - _Requirements: 7.2, 7.3_
  - [x] 4.4 Check parity, then format.
    - Before formatting, run
      `npx prettier@3.9.10 --check --prose-wrap always "**/*.md"` to list the
      files prettier would change today
      (`specs/0001-hurl-api-coverage/review.md` at the time of the spike).
    - Run `just fmt-md`: only those files may change, exactly as prettier would
      change them (compare with `npx prettier@3.9.10 --write` on a scratch
      copy).
    - Run `just fmt-md` again: nothing changes. `oxfmt --check "**/*.md"`
      exits 0.
    - _Requirements: 1.2, 4.2_

- [x] 5. Lint Markdown with rumdl
  - [x] 5.1 In `justfile`, make `lint-md` run `rumdl check .` after the format
        check, and make `fmt-md` run `rumdl check --fix .` (accepting exit 1)
        before `oxfmt "**/*.md"` (recipes in `design.md`).
    - Add the MD076 option to `.markdownlint.yml` (`list-item-spacing`,
      `allow-loose-continuation: true`, content in `design.md`); added during
      implementation, see `findings.md` §2.1.
    - _Requirements: 2.1, 2.3, 2.4, 4.3_
  - [x] 5.2 Verify on the repository: `just lint-md` passes and reads
        `.markdownlint.yml` (`rumdl config` lists its settings as
        `[from .markdownlint.yml]`).
    - _Requirements: 2.1_
  - [x] 5.3 Negative check on a scratch branch: a heading level skip in any
        `.md` file makes `just lint-md` fail with the file, the line and the
        rule; revert.
    - _Requirements: 2.2_

- [x] 6. Spellcheck the whole repository with typos
  - [x] 6.1 Add `_typos.toml` at the root (content in `design.md`): hidden
        folders checked, `.git/` excluded, `ND`, `nd` and `claus` accepted, the
        misspelling of "submission" found at
        `lambdas/src/core/resource/cities/endpoint.rs:225` added with its
        correction, and the `spike` file type ignoring code spans and
        `cspell:ignore` comments in files named `spike.md`.
    - _Requirements: 3.1, 3.4, 3.5_
  - [x] 6.2 In `justfile`, add `fmt-spellcheck` (`typos --write-changes`,
        accepting exit 2) and make `lint-spellcheck` run `typos`.
    - _Requirements: 3.1, 3.2, 3.3, 3.6_
  - [x] 6.3 Delete `cspell.config.yaml` and `words.txt`.
    - _Requirements: 3.6, 7.2, 7.3_
  - [x] 6.4 In `specs/0002-rust-tooling/spike.md:457`, turn the bold quote of
        the typo finding into a code span (formatting only).
    - _Requirements: 3.5_
  - [x] 6.5 Run `just fmt-spellcheck` and review the diff: it must fix the 13
        single-suggestion findings (the `justfile` variable and its comment at
        line 30, `lambdas/src/lib.rs:188` and `:195`, the doc comments of
        `lambdas/src/core/resource/{cities,pipelines,usstates}/mod.rs` and
        `lambdas/src/core/resource/schema.rs:1`, the comment at
        `migration/src/m20240202_004130_brokenspoke_analyzer_pipeline.rs:169`,
        the region name at `examples/query.rs:82`; 12 in the end, since task 5
        already rewrote the comment at line 30) and the four copies of
        "submission" (`endpoint.rs:225`, `openapi-3.1.yaml:338`,
        `openapi-3.0.yaml:376`, `bnaclient/src/lib.rs:5629`).
    - _Requirements: 3.3_
  - [x] 6.6 Fix the two findings with several suggestions by hand: the comment
        at `justfile:13` ("at once") and `bnaclient/CHANGELOG.md:15` ("from the
        API schema").
    - _Requirements: 3.2_
  - [x] 6.7 Verify the code and the generated files.
    - `cargo check --workspace` and `cargo test` pass.
    - `just regenerate-all-no-cargo` leaves `git status` unchanged (the OpenAPI
      files and `bnaclient` match their fixed source).
    - `just lint-spellcheck` passes; it reports `.github/` files when given a
      misspelling there (negative check on a scratch branch, then revert).
    - _Requirements: 3.1, 3.2, 3.5, 6.4_

- [x] 7. Wire the meta recipes
  - [x] 7.1 In `justfile`, set `fmt: fmt-spellcheck fmt-md fmt-just` and
        `lint: lint-md lint-spellcheck`; keep `ci: lint`. Update the recipe
        comments as in `design.md`.
    - _Requirements: 4.1, 4.4_
  - [x] 7.2 Verify the properties of `design.md`.
    - `just fmt` twice: the second run changes nothing.
    - `just lint` passes on the repository.
    - On a scratch branch, add in turn an unformatted paragraph, a lint error
      and a known misspelling: `just lint` fails on each, stops at the first
      failing check, and `git status` is the same before and after. Then run
      `just fmt`: the formatting and the misspelling are fixed. Revert.
    - _Requirements: 1.3, 4.2, 4.3, 4.4, 4.5, 4.6_

- [x] 8. Checkpoint - Local quality
  - `just fmt` changes nothing; `just lint` passes.
  - `cargo check --workspace`, `cargo test` and `cargo clippy` pass.
  - `just dbml-sql`, `just dbml-svg` and `just regenerate-all-no-cargo` leave
    `git status` unchanged.
  - `git grep -n -E 'prettier|markdownlint-cli2|cspell|words\.txt|docs/themes'`
    finds nothing outside `specs/` except the two `cspell:ignore` lines of
    `_typos.toml` (lines 16 and 20), which the design requires: typos skips the
    misspellings spikes quote in `cspell:ignore` comments.

- [ ] 9. Run the checks in CI
  - [x] 9.1 In `.github/workflows/ci-rust.yml`, `lint` job: add the step that
        installs oxfmt 0.78.0, rumdl 0.3.2 and typos 1.52.1 with
        `gh release download` and `gh release verify-asset`, then replace
        `just lint-md` with `just lint` (step in `design.md`).
    - _Requirements: 5.1, 5.2, 5.4, 5.5, 5.6_
  - [ ] 9.2 Verify on the pull request: the install step logs a verified release
        attestation for each archive (and confirms the binary names inside the
        Linux archives), and the job passes without setting up Node.js.
    - _Requirements: 5.4, 5.5, 5.6_
  - [ ] 9.3 Push a commit with an unformatted paragraph and a known misspelling
        in a Markdown file: the job fails at the format check of `lint-md` and
        its log shows that neither rumdl nor typos ran. Remove the commit.
    - _Requirements: 5.3_

- [x] 10. Update `CLAUDE.md`
  - In the command block (lines 22-23), describe `just fmt` (every automatic
    fix: spelling, Markdown lint, Markdown format, `justfile` format) and
    `just lint` (formatting check, Markdown lint, spelling of the whole
    repository; what CI runs).
  - In the CI note (lines 40-42), state that CI runs `just lint`.
  - Add a short paragraph: oxfmt, rumdl and typos must be on the `PATH` at the
    versions pinned in `.github/workflows/ci-rust.yml`; accepted words and newly
    found misspellings go in `_typos.toml`; changes `just fmt` makes to code
    must be reviewed and compiled before committing.
  - Remove every mention of prettier, markdownlint-cli2, cspell and `words.txt`.
  - Verify: `just lint` passes on the new `CLAUDE.md`.
  - _Requirements: 8.1, 8.2, 8.3, 8.4_

- [ ] 11. Checkpoint - Final
  - Run the checks of task 8 again.
  - The pull request's CI is green: `ci-rust` (`just lint`), `integration`
    (`just dbml-check`) and `bnaclient` (`just regenerate-all-no-cargo`).
  - Every acceptance criterion of `requirements.md` is checked off against the
    task that verified it.

## Notes

- **One commit per task** (or per sub-task for task 6), each signed off. Task 6
  separates the configuration (6.1 to 6.4), the automatic fixes (6.5) and the
  manual fixes (6.6), so a reviewer can read the automatic renames on their own.
- **Order matters**: the link labels (task 3) must be shortened before rumdl's
  fixes run (task 5), otherwise rumdl and oxfmt undo each other (spike §5.17).
  oxfmt (task 4) comes before rumdl so the parity check runs on unchanged files.
- **Line numbers** are those of commit `b9ea034`; earlier tasks shift some of
  them in `justfile` (tasks 1, 2 and 6 all edit it).
- **Accepted exit codes**: `typos --write-changes` (2) and `rumdl check --fix`
  (1) exit non-zero when they leave something to fix by hand, and `just fmt`
  must still run the next fixer; any other code must stop it. The first version
  used just's `-` prefix, which also hid a missing tool (`findings.md` §2.2).
- **Linux archives**: the binary names inside them were confirmed with
  `tar -tzf` during task 9.1; after a version bump, list the new archives the
  same way if the install step fails on `install`.
- **Version bumps** of the three tools are manual (the `env` block of the CI
  job); Dependabot does not update them.
- **Earlier specs** keep their `<!-- cspell:… -->` comments; they are inert once
  cspell is removed and need no clean-up in this feature.
