# Requirements Document: Rust Tooling for Markdown and Spelling

<!-- cspell:words rumdl softwaretechnik -->

## Introduction

The repository formats, lints and spellchecks its Markdown with three JavaScript
tools (prettier, markdownlint-cli2 and cspell), each downloaded by an unpinned
`npx --yes` call, and runs four more JavaScript tools for its DBML and OpenAPI
generated files. The spike in [`spike.md`](spike.md) concluded with a
**Conditional Go**: replace the three Markdown tools with Rust tools (rumdl,
oxfmt and typos), keep the generators on `npx` but pinned, and remove the
`sql2dbml` recipes. Extended twice after review, it found no replacement for the
generators in any language, kept typos alone for spelling, and showed that the
tools' automatic fixes agree once two link definitions are shortened.

This feature implements that conclusion. It removes the unpinned downloads,
makes Markdown work faster, and splits the work between two commands:
`just fmt`, which contributors run to fix everything that can be fixed
automatically, and `just lint`, which only checks and which CI runs on every
pull request. Today CI only lints Markdown: neither spelling nor formatting is
checked.

**In scope**:

- Markdown formatting with oxfmt, linting with rumdl, and spellchecking of the
  whole repository with typos, each pinned to an exact version in CI.
- `just fmt` applying every automatic fix; `just lint` checking formatting, lint
  rules and spelling without modifying files.
- CI running `just lint` on every pull request and stopping at the first
  failure.
- Fixing the real typos found by the spike, and shortening the two link
  reference definitions that rumdl misreports (spike §5.1, §5.17).
- Pinning the `@dbml/cli` and `@softwaretechnik/dbml-renderer` packages.
- Removing the `sql2dbml` recipes and the configuration of the removed tools,
  including the stale `docs/themes` ignores.
- Updating `CLAUDE.md` to describe the new tools and what CI runs.

**Out of scope**:

- How contributors install the tools on their machines: a separate spike will
  evaluate a tool manager (mise, aqua…). Until then the `just` recipes expect
  the tools on the `PATH`, and CI holds the pinned versions.
- Replacing `dbml2sql`, `dbml-renderer` or `openapi-down-convert` (spike §5.10
  to §5.15), and removing Node.js from CI.
- Formatting the `justfile` (`just --fmt`), which is unchanged.
- The other PeopleForBikes repositories.
- Editor integration.

## Glossary

- **Markdown_Files**: every `*.md` file of the repository that git does not
  ignore, including hidden folders such as `.github/`.
- **Repository_Files**: every file of the repository that git does not ignore,
  of any type, including hidden folders, but not the `.git/` folder.
- **Pinned_Tool**: a tool installed at an exact version, from a source whose
  integrity is checked before use (a published checksum, or a GitHub release or
  build attestation).
- **Automatic_Fix**: a change a tool can make without a human choice: oxfmt's
  formatting, the fixes of rumdl's fixable rules, and the typos corrections that
  have a single suggestion.
- **Spelling_Config**: the typos configuration file of the repository, which
  holds the words to accept and the misspellings to add to typos' own list.
- **Known_False_Positives**: the words typos reports that are correct in this
  repository: `ND` and `nd` (North Dakota) and `claus` (a city name).
- **Deliberate_Misspellings**: misspellings a spike quotes on purpose as
  evidence (seeded typos, tool findings), written in its `spike.md` as code
  spans or in `cspell:ignore` comments.

## Requirements

### Requirement 1: Markdown Formatting with oxfmt

**User Story:** As a contributor, I want Markdown formatted exactly as prettier
formats it today, so that switching tools does not reformat the repository.

#### Acceptance Criteria

1. WHEN a contributor runs `just fmt`, the system SHALL format all
   Markdown_Files with oxfmt, wrapping prose at 80 columns.
2. WHEN `just fmt` runs on Markdown_Files already formatted by prettier 3.9.10
   with `--prose-wrap always`, the system SHALL leave every file byte-identical.
3. WHEN a contributor runs `just lint`, the system SHALL report every Markdown
   file that is not formatted, without modifying it.
4. The system SHALL NOT invoke prettier.

### Requirement 2: Markdown Linting with rumdl

**User Story:** As a contributor, I want Markdown linted by a fast single binary
with the same rules as today, so that I get the same feedback without
downloading a JavaScript tool on every run.

#### Acceptance Criteria

1. WHEN a contributor runs `just lint`, the system SHALL lint all Markdown_Files
   with rumdl, applying the rules of `.markdownlint.yml`.
2. IF a Markdown file breaks a rule, THEN `just lint` SHALL exit with a non-zero
   status and report the file, the line and the rule.
3. WHEN a contributor runs `just fmt`, the system SHALL apply rumdl's
   Automatic_Fixes to all Markdown_Files.
4. The system SHALL NOT invoke markdownlint-cli2.

### Requirement 3: Spellchecking the Whole Repository with typos

**User Story:** As a contributor, I want the whole repository checked for
misspellings, so that typos in documentation, and in code as a bonus, are caught
and project jargon is never flagged.

#### Acceptance Criteria

1. WHEN a contributor runs `just lint`, the system SHALL check all
   Repository_Files with typos.
2. IF a file contains a misspelling typos knows, THEN `just lint` SHALL exit
   with a non-zero status and report the file, the line, the word and the
   suggested correction.
3. WHEN a contributor runs `just fmt`, the system SHALL apply the typos
   corrections that have a single suggestion, and leave the others reported.
4. The system SHALL accept the Known_False_Positives, and SHALL read any further
   accepted word or added misspelling from the Spelling_Config.
5. WHERE a file is named `spike.md`, the system SHALL ignore the
   Deliberate_Misspellings and SHALL check the rest of the file.
6. The system SHALL NOT invoke cspell.

### Requirement 4: Auto-Fix and Check Commands

**User Story:** As a contributor, I want one command that fixes everything that
can be fixed automatically and one that checks everything, so that I fix my
changes locally and CI only has to verify them.

#### Acceptance Criteria

1. WHEN a contributor runs `just fmt`, the system SHALL apply every
   Automatic_Fix of oxfmt, rumdl and typos.
2. WHEN `just fmt` runs twice in a row, the second run SHALL change no file.
3. WHEN `just fmt` has run on a repository whose remaining problems all have an
   Automatic_Fix, `just lint` SHALL pass.
4. WHEN a contributor runs `just lint`, the system SHALL check formatting, lint
   rules and spelling, and SHALL NOT modify any file.
5. IF any check fails, THEN `just lint` SHALL stop and exit with a non-zero
   status.
6. WHEN `just lint` runs on the repository once this feature is complete, the
   system SHALL report no problem.

### Requirement 5: Checks in Continuous Integration

**User Story:** As a maintainer, I want every pull request checked with pinned
tools, so that the rules are enforced and no unpinned code runs in CI.

#### Acceptance Criteria

1. WHEN a pull request is opened, synchronized or reopened, CI SHALL run
   `just lint`.
2. CI SHALL NOT run `just fmt` or modify any file.
3. IF a check fails, THEN the CI job SHALL fail without running the remaining
   checks.
4. CI SHALL install rumdl, oxfmt and typos as Pinned_Tools.
5. IF a tool's checksum or attestation does not match, THEN CI SHALL fail before
   running the tool.
6. CI SHALL NOT need Node.js to run `just lint`.

### Requirement 6: Pinned Generators

**User Story:** As a maintainer, I want every remaining `npx` package pinned to
an exact version, so that the generated files do not change with whatever npm
serves on a given day.

#### Acceptance Criteria

1. The system SHALL name an exact package version in every `npx` call of the
   `justfile`.
2. WHEN `just dbml-sql` runs, the system SHALL produce a `docs/database.sql`
   byte-identical to the committed one.
3. WHEN `just dbml-svg` runs, the system SHALL produce a `docs/database.svg`
   byte-identical to the committed one.
4. WHEN CI runs `just dbml-check` and `just regenerate-all-no-cargo`, they SHALL
   pass as they do before this feature.

### Requirement 7: Removal of Unused Tools and Configuration

**User Story:** As a maintainer, I want unused recipes and the configuration of
removed tools deleted, so that nobody runs a forbidden recipe or edits a dead
configuration file.

#### Acceptance Criteria

1. The `justfile` SHALL NOT contain the `dbml-from-sql` and `db-to-dbml`
   recipes, nor any call to `sql2dbml`.
2. The repository SHALL NOT contain `.prettierignore`, `cspell.config.yaml` or
   `words.txt`.
3. No configuration file or recipe SHALL reference `docs/themes`.

### Requirement 8: Accurate Contributor Instructions

**User Story:** As a contributor (human or agent), I want `CLAUDE.md` to name
the tools and what each command does, so that I run the same checks locally
before pushing.

#### Acceptance Criteria

1. `CLAUDE.md` SHALL name oxfmt, rumdl and typos, and SHALL state that they must
   be on the `PATH` at the versions CI pins.
2. `CLAUDE.md` SHALL state that `just fmt` applies the automatic fixes and that
   CI runs `just lint` (formatting check, Markdown lint, spellcheck of the whole
   repository).
3. `CLAUDE.md` SHALL state that the changes `just fmt` makes to code (typos
   corrections) must be reviewed and compiled before committing.
4. `CLAUDE.md` SHALL NOT mention prettier, markdownlint-cli2, cspell or
   `words.txt`.
