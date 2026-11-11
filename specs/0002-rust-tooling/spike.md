# Spike: Rust Alternatives to the JavaScript Tooling

<!-- cspell:words apskhem brokenspoke chrono codebook mado mmdflux pgrouting -->
<!-- cspell:words quickmark roas rumdl rvben serde softwaretechnik sqlx taiki -->
<!-- cspell:words unpinnable venv -->
<!-- cspell:words checksummed conv duythinht erraggy fkey goccy graphviz -->
<!-- cspell:words libopenapi mevdschee oastools pkey rsvg shmget todot tosql -->
<!-- cspell:words bikeable butschster downgrader downgraders hidi sharrows -->
<!-- cspell:words subscore Tsvg varchar wazero -->
<!-- Misspellings quoted on purpose (seeded typos and tool findings): -->
<!-- cspell:ignore accomodate accross configuraiton databse defaul definately -->
<!-- cspell:ignore dependancy endpiont enpoint entites enviroment existance -->
<!-- cspell:ignore Lmabda markown Massachussets migraiton occured pagniation -->
<!-- cspell:ignore paramaters paramter recieve reponse requets schmea seperate -->
<!-- cspell:ignore sumission untill wich -->

## Status

Concluded — Conditional Go: replace markdownlint-cli2, prettier and cspell with
rumdl, oxfmt and typos; keep the DBML and OpenAPI generators on pinned `npx`.
See §6. Extended twice after review: Go candidates (§5.12 to §5.14), then
generators in any language, spellchecking with a word list and auto-fixing in
`just fmt` (§5.15 to §5.17). The verdict is unchanged; `just fmt` needs the two
link definitions shortened.

## 1. Intent

The repository runs seven JavaScript tools through `npx`. This spike asks, for
each one, whether a Rust tool can replace it while still meeting our needs.

Why now:

- **Supply-chain risk.** All but one `npx` call is unpinned (`npx --yes <pkg>`).
  Each run, locally and in CI, downloads whatever version npm serves that day,
  with its dependency tree.
- **Speed and simpler setup.** Contributors and CI runners would not need
  Node.js, only single static binaries.
- **Reuse.** The verdict should also apply to the other PeopleForBikes
  repositories that use the same tooling (for example `brokenspoke`).

The decision it informs: which tools to replace (and with what), and which to
keep on `npx`. Each tool is judged on its own. Removing Node.js entirely is a
welcome outcome, not a requirement.

**Constraints that shaped scope**:

- In scope: all seven tools from the inventory in §2.
- A candidate must be **installable at an exact version in CI**, for example
  through `taiki-e/install-action`, `cargo-binstall` or a release binary.
- A candidate must be **actively maintained**: recent releases and a permissive
  license.
- Experiments: web research, installing Rust candidates and running the current
  npm tools, all on a copy of the repository in a scratch directory. No
  prototypes are written in this spike.

**Extension after review**: a reviewer asked that, for the tools with no Rust
candidate (`dbml2sql`, `dbml-renderer` and `openapi-down-convert`), Go tools be
evaluated too, since they share the qualities that motivated the spike: speed,
memory safety and a single binary that can be installed at an exact version and
verified. The same success bar and constraints apply. `sql2dbml` is left out
because its recipes are being removed (§5.11).

**Second extension after review**: reviewing the first specification draft, the
team reframed three points.

- **Generators in any language**: for the same three generators, any language
  qualifies if the tool is fast, secure and a single binary. Compiling the
  current JavaScript tools into a binary (`bun build --compile`, `deno compile`)
  was ruled out by the team as overkill.
- **A word list is acceptable**: maintaining a spelling word list is not a cost.
  This reopens the spellcheck choice of §5.7 to §5.9, where avoiding a word list
  counted in favor of typos.
- **`just fmt` auto-fixes, `just lint` only checks**: contributors run
  `just fmt`, which applies every automatic fix (formatting, lint fixes,
  unambiguous spelling corrections); CI runs `just lint`, which must pass after
  `just fmt`. Whether the tools' fixes agree with each other needed testing
  (§5.17).

**Success bar**: for each tool, a replacement is a **Go** when it covers the
same intent (the same formatting style, the same rules, the same checks, the
same generated artifact) on the repository's real files. A one-time reformatting
or configuration translation is fine if the result is stable (running the tool
twice changes nothing) and reasonable. A replacement must pass on the current
files once that translation is done, or flag only real issues the current tool
misses.

## 2. Current Baseline

No `package.json`, lock file, pre-commit hook or Node version file exists. Every
JavaScript tool runs through `npx` in the root `justfile`; the ubuntu runners
already ship Node.js.

| #   | Tool (npm package)               | Recipe (`justfile`)                      | Purpose                                                     | In CI                                        | Pinned |
| --- | -------------------------------- | ---------------------------------------- | ----------------------------------------------------------- | -------------------------------------------- | ------ |
| 1   | `prettier`                       | `fmt-md` (`justfile:25`)                 | Formats Markdown, `--prose-wrap always` (`.prettierignore`) | No                                           | No     |
| 2   | `markdownlint-cli2`              | `lint-md` (`justfile:32`)                | Lints Markdown (`.markdownlint.yml`)                        | Yes, `.github/workflows/ci-rust.yml:17`      | No     |
| 3   | `cspell`                         | `lint-spellcheck` (`justfile:36`)        | Spellchecks Markdown (`cspell.config.yaml`, `words.txt`)    | No                                           | No     |
| 4   | `@dbml/cli` (`dbml2sql`)         | `dbml-sql`, `dbml-check` (`justfile:91`) | Generates `docs/database.sql`, checks it against migrations | Yes, `.github/workflows/integration.yml:148` | No     |
| 5   | `@dbml/cli` (`sql2dbml`)         | `dbml-from-sql` (`justfile:140`)         | SQL dump to DBML; `CLAUDE.md` says never to use it          | No                                           | No     |
| 6   | `@softwaretechnik/dbml-renderer` | `dbml-svg` (`justfile:144`)              | Generates `docs/database.svg`                               | No                                           | No     |
| 7   | `@apiture/openapi-down-convert`  | `generate-oas-30` (`justfile:180`)       | Converts `openapi-3.1.yaml` to 3.0 for `progenitor`         | Yes, `.github/workflows/bnaclient.yml:33`    | 0.14.2 |

Other observations:

- `CLAUDE.md:23` says `just lint` (Markdown lint and spellcheck) is what CI
  runs, but CI runs only `just lint-md`. Spellchecking and formatting are never
  enforced, so they drift (see below).
- `.prettierignore`, `.markdownlint.yml`'s CLI globs and `cspell.config.yaml`
  ignore `docs/themes/`, which no longer exists.
- oxc's linter (`oxlint`) targets JavaScript and TypeScript, which this
  repository does not contain, so it is out of scope.

### 2.1 Measured baseline

Run on 2026-11-10 on a copy of the repository at `b9ea034` (`git archive HEAD`),
macOS (Apple Silicon), Node.js v24.21.0, npm 12.2.2, with a warm `npx` cache.
Times are the wall-clock time of one `time` run of the `justfile` command.

| Tool                   | Version                      | Files                | Result                                                                                                      | Time   |
| ---------------------- | ---------------------------- | -------------------- | ----------------------------------------------------------------------------------------------------------- | ------ |
| prettier               | 3.9.10                       | 24 `.md`             | Rewraps one paragraph of `specs/0001-hurl-api-coverage/review.md` (4 lines), all other files unchanged      | 0.85 s |
| markdownlint-cli2      | 0.24.0 (markdownlint 0.42.0) | 24 `.md`             | `0 issues in 0 files`                                                                                       | 0.74 s |
| cspell                 | 10.4.1                       | 21 of 24 `.md`       | `Issues found: 0`; skips `.github/*.md` (dot folder, no `--dot`) and the `AGENTS.md` symlink to `CLAUDE.md` | 1.12 s |
| `@dbml/cli` `dbml2sql` | 12.0.0                       | `docs/database.dbml` | `just dbml-sql` reproduces `docs/database.sql` byte for byte                                                | 2.21 s |
| dbml-renderer          | 1.0.31                       | `docs/database.dbml` | Reproduces `docs/database.svg` byte for byte                                                                | 0.77 s |
| openapi-down-convert   | 0.14.2                       | `openapi-3.1.yaml`   | Reproduces `openapi-3.0.yaml` byte for byte                                                                 | 0.78 s |

The Markdown corpus: 24 files, 2,692 lines. The DBML file has 22 tables, 17
named `Ref`s (9 with `delete:` actions), 4 `Indexes` blocks, `default:` values
and composite primary keys. The OpenAPI 3.1 file (3,133 lines) uses
`type: [T, 'null']` 71 times, `examples:` 82 times and `oneOf` 3 times; these
are the 3.1 features the down-conversion must rewrite.

## 3. Candidates Considered

| Tool replaced        | Candidate                                                   | Verdict                        | Why                                                                                                                                                                     |
| -------------------- | ----------------------------------------------------------- | ------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| markdownlint-cli2    | rumdl 0.3.2                                                 | Tested, see §5.1, §5.2         | Reads `.markdownlint.yml` as is; 0 real issues on the repo (2 false positives); 22 of 25 seeded defects found identically; 15x faster                                   |
| markdownlint-cli2    | mado 0.4.0                                                  | Tested and rejected, see §5.3  | Implements the Ruby `mdl` rules (38 rules, nothing after MD047), needs its own config, 20 false positives on the repo, two unexplained hangs                            |
| markdownlint-cli2    | quickmark-cli 1.1.0                                         | Rejected before testing        | Last release 2025-09-05 and last push 2025-09-11: fails "actively maintained"                                                                                           |
| prettier             | oxfmt 0.78.0 (standalone binary)                            | Tested, see §5.4               | Byte-identical to prettier on all 23 repository files with `printWidth: 80`; 426 of 435 corpus files identical; 70x faster; stable                                      |
| prettier             | rumdl 0.3.2 (`rumdl fmt`)                                   | Tested and rejected, see §5.5  | Without reflow it only applies lint fixes; with reflow it rewraps differently and writes lines that fail MD013                                                          |
| prettier             | dprint 0.64.3 + markdown 0.29.0                             | Tested, works but second, §5.6 | Stable and lint-clean, but differs on 9 files (wraps inside code spans) and downloads its plugin from a second host at run time                                         |
| cspell               | typos 1.52.1                                                | Tested, see §5.7               | 0 false positives on the repository and on jargon; 18 of 20 seeded typos; found a real typo cspell misses; different model (known misspellings)                         |
| cspell               | harper-cli 3.1.0                                            | Tested and rejected, see §5.8  | 20 of 20 seeded typos, but 126 false positives over 29 jargon words on the repository; a grammar checker whose CLI is labelled a debugging tool                         |
| cspell               | codebook 0.3.42                                             | Tested and rejected, see §5.9  | Downloads its dictionaries at run time from cspell-dicts' `main` branch (unpinnable); 15 false positives on the repository even with `words.txt`                        |
| cspell               | cargo-spellcheck 0.15.7                                     | Rejected before testing        | Checks Rust doc comments, not Markdown                                                                                                                                  |
| dbml2sql             | (none)                                                      | No candidate, see §5.11        | No Rust tool generates SQL from DBML; `dbml-rs` 1.0.1 is a parser library only, `sea-orm-dbml` generates entities                                                       |
| dbml-renderer        | (none)                                                      | No candidate, see §5.11        | No Rust tool renders DBML; the Rust ER renderers (mmdflux, merman) take Mermaid, and `dbml2mermaid` has 0 stars                                                         |
| sql2dbml             | remus                                                       | Rejected before testing        | 3 stars; and `CLAUDE.md` already forbids the recipe, which can be removed instead (§5.11)                                                                               |
| openapi-down-convert | openapi31to30 0.0.1                                         | Tested and rejected, see §5.10 | Leaves the 82 schema `examples` unconverted (invalid 3.0) and drops `servers`; one release (2025-03), 2 stars                                                           |
| openapi-down-convert | roas-cli 1.1.0                                              | Rejected before testing        | Converts only upward (2.0 to 3.0 to 3.1 to 3.2); no 3.1 to 3.0                                                                                                          |
| dbml2sql             | dbml-tools 0.9.11 (Go)                                      | Tested and rejected, see §5.12 | Renames the 17 named foreign keys and writes `SERIAL` for `IDENTITY`, so `dbml-check` would fail; needs edits to the DBML; 0 stars, one author                          |
| dbml2sql             | go-dbml 0.1.1 (Go)                                          | Rejected before testing        | A library with no CLI: using it means writing a Go program (no prototypes in this spike); first commit October 2026, 0 stars, one author                                |
| dbml2sql             | dbml-go (Go, `duythinht`)                                   | Rejected before testing        | Generates Go model structs, not SQL; last push June 2023                                                                                                                |
| dbml-renderer        | dbml-toolkit 0.0.19 (Go)                                    | Tested and rejected, see §5.13 | Single binary with Graphviz embedded, readable and 27x faster, but drops the composite primary keys of 2 tables; 1 star, one author                                     |
| dbml-renderer        | dbml-tools 0.9.11 `todot` (Go)                              | Rejected before testing        | Writes Graphviz DOT only; producing the SVG needs the Graphviz `dot` program, which is written in C, not Go                                                             |
| openapi-down-convert | oastools 1.66.0 (Go)                                        | Tested and rejected, see §5.14 | From 3.1 to 3.0 it only changes the version string; progenitor rejects the output                                                                                       |
| openapi-down-convert | kin-openapi, libopenapi, Speakeasy `openapi` (Go)           | Rejected before testing        | No 3.1 to 3.0 conversion: kin-openapi's `openapi3conv` lists "any → 3.0" as out of scope, Speakeasy's CLI only upgrades, libopenapi is a parser library                 |
| dbml2sql             | PyDBML (Python), DBML_SQLite (Python), PHP and Java parsers | Rejected before testing, §5.15 | Libraries, not single binaries; DBML_SQLite writes SQLite only                                                                                                          |
| dbml-renderer        | dbml-tools `todot` + Graphviz `dot` (Go + C)                | Tested and rejected, see §5.15 | Shows composite primary keys, but rewrites the column types (`timestamp with time zone` becomes `timestamp`); Graphviz is a system package, not a single binary         |
| dbml-renderer        | dbml2dot (Python)                                           | Rejected before testing        | Not a single binary; 8 stars, last push March 2021                                                                                                                      |
| openapi-down-convert | Microsoft `hidi` (C#)                                       | Rejected before testing, §5.15 | A .NET tool: needs the .NET runtime, a single executable is published for Windows only                                                                                  |
| openapi-down-convert | openapi-downgrade (Python)                                  | Rejected before testing        | A Python package, not a single binary; 10 stars, one author with 3 commits                                                                                              |
| openapi-down-convert | openapi-downgrader (C++)                                    | Rejected before testing        | Converts 3.0 to Swagger 2.0, not 3.1 to 3.0; Windows executable only                                                                                                    |
| openapi-down-convert | progenitor reading 3.1 directly                             | Not available                  | Issues #762 and #1268 (OpenAPI 3.1 support) are still open                                                                                                              |
| cspell               | Vale 3.26.0 (Go)                                            | Tested, not retained, §5.16    | Dictionary checker for Markdown and Rust comments; 5 real typos, about 26 words to add; 20 of 20 seeded typos; a second tool next to typos, judged overkill by the team |
| cspell               | harper-cli 3.1.0, with a word list                          | Re-tested, see §5.16           | The word-list argument against it no longer holds, but it needs about 70 words and its CLI is labelled a debugging tool                                                 |

## 4. Evaluation Method

All runs were made on 2026-11-10 on macOS (Apple Silicon), in a scratch
directory outside the repository, on a copy of the repository at `b9ea034`
(`git archive HEAD`). Tools ran from their official release archives for
`aarch64-apple-darwin`, downloaded from GitHub Releases (the same artifacts a CI
install would use):

| Tool       | Version | Archive SHA-256 (first 16) | Notes                                       |
| ---------- | ------- | -------------------------- | ------------------------------------------- |
| rumdl      | 0.3.2   | `a1eefb1cd2e0cafa`         | Matches the published `.sha256`             |
| mado       | 0.4.0   | `8a646da7c4381580`         | Matches the published `.sha256`             |
| oxfmt      | 0.78.0  | `ba7e010e627377d1`         | Standalone binary (`oxfmt_v0.78.0` release) |
| dprint     | 0.64.3  | `94b48820945602fb`         |                                             |
| typos      | 1.52.1  | `e342c1ec32ea6f60`         |                                             |
| codebook   | 0.3.42  | `d0a17c11eaccb7d0`         | `codebook-lsp`, which has a CLI mode        |
| harper-cli | 3.1.0   | `3551a6e32cf1c875`         |                                             |

The npm tools ran through `npx --yes`, at the versions in §2.1.

The Go candidates (§5.12 to §5.14) ran the same way, from their release archives
for `darwin_arm64`:

| Tool         | Version | Archive SHA-256 (first 16) | Notes                                                             |
| ------------ | ------- | -------------------------- | ----------------------------------------------------------------- |
| dbml-tools   | 0.9.11  | `ac9f0bab82468816`         | Matches the published `SHA256SUMS`; no build attestation          |
| dbml-toolkit | 0.0.19  | `843e0a4242316404`         | Matches the published `checksums.txt`; no build attestation       |
| oastools     | 1.66.0  | `f375d2a7651bfd82`         | Matches the published checksums; has a GitHub attestation         |
| Vale         | 3.26.0  | `a6c3370f53bad930`         | Matches the published checksums; has a GitHub attestation (§5.16) |

The second extension (§5.15 to §5.17) also used Graphviz 16.1.1 (`dot`, from
Homebrew) and ran cspell 10.4.1 on the whole repository for comparison.

Their output was compared with the committed generated files as follows:

- **SQL**: a text diff after normalizing whitespace and the equivalent spellings
  (`PRIMARY KEY NOT NULL` against `PRIMARY KEY`, `DEFAULT (CURRENT_TIMESTAMP)`
  against `DEFAULT CURRENT_TIMESTAMP`). Loading both files into PostgreSQL and
  comparing `pg_dump`s, as `just dbml-check` does, was not possible: the sandbox
  forbids the shared memory PostgreSQL 18 needs (`shmget`:
  `Operation not permitted`), and Docker is not reachable.
- **SVG**: each file rasterized with `rsvg-convert` 2.63.4 and compared by eye
  with the committed `docs/database.svg`.
- **OpenAPI**: key-sorted JSON (`yq` v4.54.3, `jq` 1.8.2) as in §5.10, and the
  real consumer: `cargo progenitor` 0.16.0 with the `generate-client` options.

Lint comparisons normalize each tool's output to a set of `(file, line, rule)`
findings and compare the sets per rule (script: `compare_lint.py`, kept in the
scratch directory). Two corpora were used:

- **markdownlint's own test corpus**: the 435 `test/*.md` files and the
  `.markdownlint.json` (`{}`) of markdownlint v0.42.0 (the version
  markdownlint-cli2 0.24.0 bundles), from the `v0.42.0` tag on GitHub. These
  files are deliberate edge cases and many configure rules inline, so they
  stress compatibility harder than our documents do.
- **A seeded document**: `# Seeded defects` followed by 25 realistic mistakes,
  one or two per rule (skipped heading level, trailing spaces, hard tab, double
  blank line, missing blank lines, mixed list markers, wrong ordered list
  numbering, fence without a language, bare URL, long line, duplicate sibling
  heading, emphasis as a heading, spaced emphasis, empty link, image without alt
  text, broken fragment, undefined and unused references, extra table cell,
  indented code block, `#Heading`, blank line inside a quote, no final newline),
  linted with the repository's `.markdownlint.yml`.

## 5. Findings

### 5.1 rumdl on the repository — passes, 2 false positives, 15x faster

```bash
rumdl check .          # from the repository root, no extra configuration
rumdl config           # shows where each setting comes from
```

- rumdl finds and reads `.markdownlint.yml` by itself: `rumdl config` lists
  `disable = ["no-inline-html", "single-h1"]`, `MD013.code-blocks = false`,
  `MD013.tables = false` and `MD024.siblings-only = true`, each marked
  `[from .markdownlint.yml]`. No configuration translation is needed.
- Result: `Found 2 issues in 2/23 files (16ms)`; 0.049 s wall clock against 0.74
  s for markdownlint-cli2. The 23 files are the 24 minus the `AGENTS.md`
  symlink.
- Both issues are **false positives** of MD034 (bare URL) on a link reference
  definition whose URL is on the next line (`code-of-conduct.md:6`,
  `.github/CONTRIBUTING.md:9`):

  ```markdown
  [bna mechanics code of conduct]:
    https://peopleforbikes.github.io/code-of-conduct/
  ```

  This is valid CommonMark, and markdownlint accepts it. Adopting rumdl means
  either joining these two definitions on one line or reporting the bug
  upstream.

- YAML front matter (the learnings files) is handled correctly.

### 5.2 rumdl against markdownlint — equivalent on realistic content

**Seeded document** (`npx markdownlint-cli2@0.24.0 seeded.md`,
`rumdl check --no-cache seeded.md`): markdownlint reports 25 findings and rumdl
24; 22 are identical.

- MD004 (mixed `*` and `-` markers): both flag it, on different lines. rumdl
  takes the most frequent marker as the expected one, markdownlint the first
  one; a documented, intentional difference.
- MD032: rumdl misses both findings for two adjacent lists that differ only by
  their marker (`*` then `-`), which CommonMark parses as two lists.
- MD057 (rumdl only): a relative link to a missing file, a useful extra check.

**markdownlint test corpus** (435 files): markdownlint reports 2,685 findings,
rumdl 4,660; 1,825 are shared (68% of markdownlint's). The gap is mostly not
missed problems:

- MD060 (254 findings) is markdownlint 0.42's new `table-column-style`, on by
  default. rumdl's MD060 (`table-format`) covers the same alignment but is
  opt-in. Our tables are aligned by prettier, so enabling it is a one-line
  configuration choice.
- MD043 (2,071 rumdl findings against 23) reports every mismatched heading
  instead of once per file; MD005 and MD029 differ in granularity too.
- rules numbered above MD059 are rumdl's own (MD057 and MD062 to MD086: link
  targets, link and code-span whitespace, etc.); they do not exist in
  markdownlint. The two rule sets share their numbering only up to MD059.
- the rest is concentrated in emphasis, code-span and table edge cases (MD037,
  MD038, MD055), which the corpus targets on purpose.

**Maintenance and installation**: MIT, 393 releases (latest 0.3.2 on 2026-10-28,
still `0.x`), 1,637 stars, pushed daily. 63 contributors, but one author holds
3,778 of the commits (bus factor of one). The repository ships a GitHub Action
(`rvben/rumdl`, `version` input) and release binaries with checksums;
`taiki-e/install-action` has no rumdl manifest but falls back to
`cargo-binstall`.

### 5.3 mado — rejected: different rule set and false positives

```bash
mado check .                                  # default configuration
mado --config mado.toml check .               # translated configuration
```

- mado does not read `.markdownlint.yml`. With its defaults it reports 105
  errors (MD007 45, MD013 31, MD029 13, MD041 8, MD005 6, MD024 2).
- It implements the rules of the Ruby `mdl` (markdownlint/markdownlint), not
  those of markdownlint-cli2: 38 rules up to MD047 (none of MD042 to MD059:
  empty links, alt text, link fragments, reference definitions, tables…), and
  `mdl` defaults (list indent 4, ordered lists numbered `1.`). MD024 has
  `allow-different-nesting`, not `siblings_only`.
- With a translated `mado.toml` (MD013 tables and code blocks off, MD007 indent
  2, MD029 `ordered`, MD024 `allow-different-nesting`, MD025 and MD033 left out)
  20 errors remain, all false positives: 8 × MD041 because it does not recognize
  YAML front matter, and 12 × MD005/MD007 on a bullet list nested under an
  ordered item with 3 spaces of indentation
  (`specs/0001-hurl-api-coverage/design.md:433`), which is correct.
- Twice, `mado --config mado.toml check .` never finished (killed after 120 s
  and 15 s, where a normal run takes 8 ms). It did not happen again in 40
  further runs, with or without the configuration; the cause is unknown.
- 8 contributors, one author holding nearly all commits.

### 5.4 oxfmt — identical to prettier on our files

Method: the repository copy was first formatted with
`npx prettier@3.9.10 --write --prose-wrap always "**/*.md"` and committed as the
reference. Each formatter then ran on that reference; any diff is a difference
from prettier.

```json
{
  "printWidth": 80,
  "proseWrap": "always",
  "ignorePatterns": ["target/**", ".venv/**"]
}
```

```bash
oxfmt -c .oxfmtrc.json "**/*.md"           # format
oxfmt -c .oxfmtrc.json --check "**/*.md"   # CI check
```

- **Repository**: 0 changes on all 23 files (`git status` clean): oxfmt's output
  is byte-identical to prettier's. 0.013 s against 0.85 s.
- **The drift fix**: on the original, unformatted
  `specs/0001-hurl-api-coverage/review.md`, `--check` exits 1, and formatting it
  gives exactly prettier's result (no diff against the reference). On formatted
  files `--check` exits 0.
- **Stress corpus** (markdownlint's 435 test files, formatted by prettier first,
  which changed 308 of them): oxfmt leaves 426 identical and changes 9 (29 lines
  added, 18 removed). A second oxfmt pass changes nothing. Seven are edge cases
  of the Markdown itself: TOML (`+++`) front matter (3 files), a blank line
  added after an HTML comment glued to a paragraph, a malformed table,
  consecutive `$$` math delimiters, and escaped `*` in nested emphasis. The
  other two matter more: inside a ` ```css ` fence holding invalid CSS
  (`body {} {MD040:11}`), prettier leaves the block alone while oxfmt reformats
  it into different code. Like prettier, oxfmt formats the code in fences it
  recognizes, but its parsers accept invalid input that prettier's reject. With
  `"embeddedLanguageFormatting": "off"` both files are left untouched (4 files
  then differ from prettier). Our fences are `bash` (13), `rust`, `mermaid`, and
  `json` and `markdown` in this document; none changed. oxc lists its
  intentional Markdown differences from prettier in
  `crates/oxc_formatter_markdown/DIVERGENCES.md` (25 entries).
- **Pitfalls**:
  - `printWidth` must be set: oxfmt defaults to 100, and with only
    `proseWrap: "always"` it rewrapped 21 files (1,084 lines added, 1,362
    removed).
  - Unknown configuration keys are silently ignored (`{"proseWrapX": ...}` ran
    without a warning), so a typo goes unnoticed.
  - oxfmt reads `.prettierignore` and `.gitignore` by default.
- **Markdown is native Rust**: oxc's language-support page lists Markdown under
  "Native" ("no Node.js round-trip"); only Angular, Svelte, Astro, MDX,
  Handlebars and MJML go through the bundled prettier, and the standalone binary
  skips those.
- **Distribution**: the npm package is oxc's recommended install; the standalone
  binary is published on GitHub Releases for 16 targets (`oxfmt_v0.78.0`,
  2026-11-09), without checksum files but with a GitHub build attestation
  (`gh api repos/oxc-project/oxc/attestations/sha256:<digest>` returns 1).
  `taiki-e/install-action`'s `oxfmt` manifest is stale: it still follows the
  combined `apps_v*` tags (latest `1.84.0`, which shipped oxfmt 0.69.0) and does
  not know the `oxfmt_v*` tags, so pinning `oxfmt@0.78.0` through it does not
  work today. The `oxfmt` crate on crates.io (1.0.0) is not oxc's: oxc's crate
  has `publish = false`.
- **Maintenance**: MIT, oxc has 23,405 stars and more than 100 contributors,
  backed by VoidZero. oxfmt is still `0.x` and releases about twice a week
  (0.76.0, 0.77.0 and 0.78.0 within 6 days), so a pinned version needs regular
  bumps.

### 5.5 rumdl fmt — rejected as a formatter

```bash
rumdl fmt .
rumdl fmt --config 'MD013.reflow = true' --config 'MD013.reflow-mode = "normalize"' .
```

- Without reflow (the default, `MD013.reflow = false`), `rumdl fmt` only applies
  lint fixes: on the reference it changed the two link definitions of §5.1 to
  `<https://…>`. It does not wrap prose.
- With `reflow-mode = "default"` it changes nothing more (it only shortens lines
  over the limit). With `reflow-mode = "normalize"` (the equivalent of
  `--prose-wrap always`), it rewraps 4 files (31 lines added, 32 removed): it
  fills lines differently from prettier, changes the continuation indent of
  task-list items from 6 to 2 spaces, and writes two 84-character lines
  (`specs/0001-hurl-api-coverage/tasks.md:104` and `:268`) that both rumdl and
  markdownlint-cli2 then report as MD013 errors. It is stable (a second pass
  changes nothing) but its output does not pass its own linter.

### 5.6 dprint with the Markdown plugin — works, second choice

```json
{
  "lineWidth": 80,
  "markdown": { "textWrap": "always" },
  "excludes": ["target", ".venv"],
  "plugins": ["https://plugins.dprint.dev/markdown-0.29.0.wasm"]
}
```

- `dprint fmt "**/*.md"` changes 9 files (86 lines added, 89 removed). The
  output passes markdownlint-cli2 (`0 issues`), and a second run changes nothing
  (0.010 s once the plugin is compiled; the first run, which downloads and
  compiles it, takes 1.0 s).
- The main difference from prettier: dprint breaks lines inside inline code
  spans (`` `GET `` / `` /prices/fargate?latest=true` ``), which still renders
  the same but splits commands and paths across lines, hurting search. It also
  joins link reference definitions onto one line.
- The plugin is a Wasm file fetched from `plugins.dprint.dev` at run time: a
  second download host on every fresh machine. It can be pinned with a checksum
  (`…/markdown-0.29.0.wasm@<sha256>`), and the dprint binary is in
  `taiki-e/install-action`. dprint is MIT, 4,135 stars, mostly one author; the
  Markdown plugin has 60 stars.

### 5.7 typos — no false positives, a different model than cspell

Two tests, each run with cspell 10.4.1 (`words.txt` and `cspell.config.yaml`
copied next to the file), typos 1.52.1 and harper-cli 3.1.0 (§5.8):

- **The repository's 24 Markdown files**: `typos $(find . -name '*.md')`.
- **A seeded document** with 20 typos, 10 common English misspellings
  (`recieve`, `seperate`, `occured`, `dependancy`, `enviroment`, `existance`,
  `accomodate`, `definately`, `untill`, `wich`) and 10 transposed domain words
  (`paramter`, `reponse`, `databse`, `migraiton`, `schmea`, `endpiont`,
  `pagniation`, `Lmabda`, `configuraiton`, `requets`), plus 24 correct jargon
  words (SeaORM, utoipa, Axum, Fargate, OpenAPI, progenitor, Hurl, Schemathesis,
  pgrouting, PostGIS, bnaclient, Paginatron, justfile, Postgres, PeopleForBikes,
  serde, chrono, tokio, clippy, rustfmt, dbml, pgadmin, usstates, axumed).

| Tool                    | Repository findings    | Seeded typos found | False positives on jargon    |
| ----------------------- | ---------------------- | ------------------ | ---------------------------- |
| cspell 10.4.1           | 0                      | 20 / 20            | 3 (pgrouting, serde, chrono) |
| typos 1.52.1            | 1, real                | 18 / 20            | 0                            |
| harper-cli (SpellCheck) | 126 (30 words), 1 real | 20 / 20            | 10                           |

- typos found a real typo that cspell misses: "from `tha` API schema"
  (`bnaclient/CHANGELOG.md:15`). cspell accepts `tha` because it is in one of
  its dictionaries.
- typos does not check words against a dictionary: it looks them up in a list of
  known misspellings, so it never flags jargon (no `words.txt` needed) but
  misses invented typos it has never seen (`pagniation`, `Lmabda`). cspell flags
  any unknown word, at the cost of maintaining `words.txt`.
- Bonus: typos checks code too. `typos` on the whole repository (0.028 s)
  reports 19 findings, most of them real: `enpoint` (4, in `mod.rs` doc
  comments), `entites` (3, `justfile`), `defaul` (2), `paramaters`, `markown`,
  `accross`, `Massachussets` (`examples/query.rs:82`) and "at `onces`"
  (`justfile:13`). The false positives are `ND`/`nd` (North Dakota) and `claus`
  (a city name in a CSV), which a `_typos.toml` would allow.
- Distribution: Apache-2.0 or MIT, 4,216 stars, 94 contributors, maintained
  under the `crate-ci` organization; in `taiki-e/install-action`, with an
  official GitHub Action (`crate-ci/typos`).
- Time: 0.064 s for the Markdown files against 1.12 s for cspell.

### 5.8 harper — rejected: too many false positives

```bash
harper-cli lint --only SpellCheck -u words.txt <files>
```

- With all its rules, harper also reports grammar and style (title case, word
  choice, Oxford comma…), which is outside cspell's scope; the spelling rule
  alone was compared.
- On the repository it reports 126 spelling findings over 30 words, with
  `words.txt` as its user dictionary. Only `tha` is a real typo; the 29 others
  are project jargon (OpenAPI 30 times, SeaORM 15, BNA 14, Fargate 13, `md`,
  adaptor, DTO, justfile, semver…) that cspell's software dictionaries already
  know. Adopting it means growing `words.txt` from 18 to about 47 words, and
  more with each new term.
- `harper-cli --help` describes it as "a debugging tool for the Harper grammar
  checker": the language server is the product. Apache-2.0, 17,556 stars.

### 5.9 codebook — rejected: unpinned dictionaries, more false positives

```bash
XDG_DATA_HOME=<scratch> codebook-lsp -r . lint -u <files>
```

with a `codebook.toml` holding the 18 words of `words.txt`
(`words = ["apiture", …, "utoipa"]`; codebook cannot read a word file).

- **Dictionaries are downloaded at run time** from
  `https://raw.githubusercontent.com/streetsidesoftware/cspell-dicts/refs/heads/main/…`
  (`en_US-large.dic`/`.aff`, `softwareTerms.txt`, `computing-acronyms.txt`,
  `rust.txt`), then cached (`last_checked` and a content hash in
  `_metadata.json`). The binary can be pinned, but its dictionaries follow the
  `main` branch of another project, so the same commit can pass one day and fail
  the next. The cache defaults to `~/.local/share/codebook`. The first attempt
  failed because the sandbox blocked `raw.githubusercontent.com`; the results
  below come from a second session that allowed it.
- **Repository**: without configuration, 33 unique words; with the `words.txt`
  list, 16 unique words in 8 files (1.6 s). Only `tha` is real; the 15 others
  are false positives, including ordinary English and possessives: `lifecycle`,
  `learnings`, `cli`, `dotenv`, `psql`, `API's`, `ORM's`, BNA, DTO, Fargate,
  NFR, NUL, `overcount`, `adaptor`, `sqs`.
- **Seeded document**: 20 of 20 typos, 6 false positives on jargon (Fargate,
  pgrouting, `GIS` from PostGIS, serde, tokio, rustfmt), against cspell's 3.
- MIT, 928 stars, 48 contributors with one author holding nearly all commits;
  not in `taiki-e/install-action`.

### 5.10 openapi31to30 — rejected: invalid output, data loss

```bash
cargo install --locked openapi31to30@0.0.1        # builds in 8 s, no release binaries
openapi31to30 --input openapi-3.1.yaml --output rust-3.0.yaml
```

Both 3.0 documents were converted to key-sorted JSON (`yq -o json | jq -S`) and
compared with the committed `openapi-3.0.yaml` (byte-identical to
`openapi-down-convert` 0.14.2's output, §2.1):

- It rewrites the 71 `type: [T, 'null']` to `nullable: true` correctly.
- It leaves the 82 schema-level `examples: [x]` (JSON Schema, 3.1) as they are,
  where 3.0 needs `example: x`: the output is not valid OpenAPI 3.0.
- It silently drops the whole `servers` section (local, staging and production
  URLs).
- After normalizing `examples` to `example`, the only other difference is
  `example: 0` written as `0.0`.
- One release (0.0.1, March 2025), 2 stars, 3 contributors; its CLI prints usage
  as an error with the source location. roas, the only other Rust OpenAPI
  converter, converts only upward.

### 5.11 DBML tools — no Rust replacement

Searches on GitHub (`dbml language:rust`), crates.io and the web found:

- `dbml-rs` 1.0.1 (`apskhem/dbml-rs`, 10 stars, last push 2026-03-31): a DBML
  parser library, with no SQL or diagram output and no CLI.
- `sea-orm-dbml` (0.1.0-beta.2) and `sqlx-dbml`: DBML to SeaORM or SQLx Rust
  code, not SQL; last pushed in 2023.
- `dbml2mermaid` (0 stars): DBML to Mermaid, which Rust renderers such as
  `mmdflux` could draw, but the diagram would no longer be `dbml-renderer`'s.
- `remus` (3 stars): introspects a live PostgreSQL database into DBML and SQL,
  the direction of `sql2dbml`.

So `dbml2sql` and `dbml-renderer` stay on `npx`. Two observations for the
follow-up:

- `dbml2sql` matters most: its output is committed (`docs/database.sql`) and
  `just dbml-check` runs it in CI. Pinning it
  (`npx -y --package=@dbml/cli@12.0.0`) removes most of the supply-chain risk
  without changing the output.
- `sql2dbml` (`just dbml-from-sql`, `just db-to-dbml`) is a recipe `CLAUDE.md`
  says never to run, because it loses the column types; removing the two recipes
  removes the tool.

### 5.12 dbml-tools (Go) — rejected: renames foreign keys, changes identity

Searches on GitHub (`dbml language:go`, sorted by stars) and the web found one
Go CLI that writes SQL from DBML: `mevdschee/dbml-tools`.

```bash
dbml-tools check database.dbml
dbml-tools tosql adapted.dbml > go-tools-pg.sql
```

- **It does not read our file as is.** `check` exits 1 with
  `column "Indexes" is missing a type` 4 times (lines 48, 126, 159 and 175): it
  only accepts the lowercase `indexes` keyword, where `@dbml/cli` accepts both.
  It has no dialect option: the dialect comes from a `Project` block's
  `database_type`, and defaults to MariaDB (backtick quoting). The test used an
  adapted copy with `Project bna { database_type: 'PostgreSQL' }` and lowercase
  `indexes`; `check` then passes.
- **Output** (0.008 s against 2.21 s): 186 lines against the committed 209.
  After normalization (§4) the differences are:
  - All 17 foreign keys are named `fk_<table>_<column>`
    (`fk_bna_pipeline_fargate_price_id`) instead of the names given in the
    `Ref`s (`bna_pipeline_fargate_price_id_fkey`), which are the names the
    migrations create.
  - The two `[pk, increment]` columns (`fargate_price.id`, `submission.id`)
    become `SERIAL` instead of `INTEGER GENERATED BY DEFAULT AS IDENTITY`.
  - Equivalent: the composite primary keys are named (`city_pkey`, the name
    PostgreSQL would give them anyway), their columns lose an explicit
    `NOT NULL` that the primary key already implies, indexes are written after
    their table, and foreign keys lose `DEFERRABLE INITIALLY IMMEDIATE`.
- **Consequence**: PostgreSQL keeps constraint names, and `pg_dump` writes an
  identity column differently from a `serial` one, so `just dbml-check` would
  report both differences against the migrations. This follows from the SQL
  text; the database comparison itself could not be run (§4).
- MIT, first commit April 2026, 0 stars, 62 commits by one author. Release
  binaries for 6 targets with `SHA256SUMS` (0.9.11 on 2026-11-08), no build
  attestation.

### 5.13 dbml-toolkit (Go) — closest, rejected: drops composite primary keys

`jason-cairns/dbml-toolkit` renders DBML with D2 (ELK layout, the default) or
Graphviz.

```bash
dbml render database.dbml -o toolkit-d2.svg                    # 0.42 s
dbml render --engine graphviz database.dbml -o toolkit-gv.svg  # 0.028 s
```

- Both engines read `docs/database.dbml` unchanged, and two runs of each produce
  byte-identical files.
- **Graphviz is built in**: the binary embeds Graphviz compiled to WebAssembly
  (`goccy/go-graphviz` on `wazero`), as `dbml-renderer` embeds `viz.js`. With
  `PATH=/usr/bin:/bin` (no Homebrew `dot`) the output is identical. 0.028 s
  against 0.77 s for `dbml-renderer`.
- **Graphviz engine**: a layout close to the current diagram (the same
  left-to-right ranking), PK, FK, NN and U badges on columns, crow's-foot ends,
  a transparent background like the current file. Readable.
- **D2 engine**: a different, more compact layout on a white background, PK, FK
  and UNQ markers but no NOT NULL, and edges animated by default (`--animate`).
- **Missing information**: neither engine shows the composite primary keys
  declared in `Indexes { (…) [pk] }`: `city` (`country`, `state`, `name`) and
  `state_region_crosswalk` (`state`, `region`) have no primary key in the
  diagram (`--format d2` shows only `foreign_key` constraints on those columns).
  `dbml-renderer` shows them in bold. This fails the "same generated artifact"
  bar.
- MIT, first commit July 2026, 1 star, 34 commits by one author, 19 `0.0.x`
  releases in one month and none since 2026-08-25. Release binaries with
  `checksums.txt`, no build attestation.

### 5.14 oastools (Go) — rejected: no actual down-conversion

`erraggy/oastools` is the only Go CLI found that claims a 3.1 to 3.0 conversion
(its documentation calls it lossy).

```bash
oastools convert -t 3.0.3 -o go-3.0.yaml openapi-3.1.yaml   # flags before the file
```

- It runs in 0.025 s and reports `1 info, 0 warnings`: the only conversion issue
  is `openapi: Updated version from 3.1.0 to 3.0.3`. `--strict` also exits 0.
- The output keeps the 71 `type: [T, 'null']` and the 82 schema `examples`
  unchanged: only the version string changed. Against the committed
  `openapi-3.0.yaml` the key-sorted JSON differs on 820 lines.
- `oastools validate go-3.0.yaml` passes, so its own validator does not notice.
- **progenitor rejects it**: `cargo progenitor -i go-3.0.yaml …` exits 1 with
  the error below, where the committed file generates the client (exit 0).

  ```text
  Error: components.schemas: invalid type: sequence, expected a string at line 1652 column 9
  ```

- MIT, 6 stars, 437 of the commits by one author; frequent releases with
  checksums and a GitHub build attestation.

### 5.15 Generators in any language — still no replacement

GitHub searches (`dbml` and OpenAPI down-conversion repositories in any
language, sorted by stars) and web searches found no new candidate that is a
single binary and reproduces our files:

- **dbml2sql**: outside JavaScript, Go and Rust, the DBML projects are parsers
  or libraries: PyDBML (Python, 142 stars, can print SQL from Python code),
  DBML_SQLite (Python, SQLite only), `butschster/dbml-parser` (PHP), `dbml-java`
  (Java). None is a command-line binary.
- **dbml-renderer**: the remaining route is a pipeline, a DBML-to-DOT converter
  followed by Graphviz. Tested with `dbml-tools todot` (§5.12, after lowering
  `Indexes` to `indexes`) and `dot -Tsvg` from Graphviz 16.1.1 (0.015 s and 0.40
  s). Unlike `dbml-toolkit`, it marks the composite primary keys of `city` and
  `state_region_crosswalk`. But it rewrites every column type into a short form
  (`character varying` becomes `varchar`, `timestamp with time zone` becomes
  `timestamp`, `double precision` becomes `double`), which loses the time-zone
  information; on columns that are both, it shows PK but not FK; and Graphviz is
  a system package with plugins and shared libraries, not a single binary.
  `dbml-toolkit` already embeds Graphviz (§5.13), so its DOT output would not
  change its missing primary keys.
- **openapi-down-convert**: Microsoft's `hidi` (OpenAPI.NET, C#) is the most
  established engine that reads 3.1 and writes 3.0, but it is a .NET tool that
  needs the .NET runtime; its releases publish a single executable for Windows
  only. Whether it rewrites `examples` and type arrays was not tested. The
  Python `openapi-downgrade` is a package, not a binary. The C++
  `openapi-downgrader` converts 3.0 to Swagger 2.0.
- **progenitor reading 3.1**, which would remove the conversion, is still
  pending: issues #762 (2024) and #1268 (2025) are open.

### 5.16 Spellchecking with a word list — Vale reopens the choice

With a word list no longer a cost, the question becomes: which tool finds the
most real typos with an acceptable list? Measured on the repository copy (23
Markdown files, 81 Rust files) and the seeded document of §5.7:

```bash
vale --config vale.ini --output line <files>   # Vale.Spelling only, Vocab = words.txt
harper-cli lint --only SpellCheck -u words.txt <files>
git ls-files | npx cspell@10.4.1 --dot --words-only --unique --file-list stdin
```

Vale's configuration enables only `Vale.Spelling` (its built-in English
dictionary; `Vale.Terms`, `Vale.Repetition` and `Vale.Avoid` off), with the 18
words of `words.txt` as the vocabulary. No style package was downloaded
(`vale sync` was not run).

| Tool                  | Files checked           | Misspelled words found | Words to add for 0 false positives    | Seeded typos | Time   |
| --------------------- | ----------------------- | ---------------------- | ------------------------------------- | ------------ | ------ |
| typos 1.52.1          | every file type         | 9 (15 occurrences)     | none (3 allowed: `ND`, `nd`, `claus`) | 18 / 20      | 0.03 s |
| Vale 3.26.0           | Markdown, Rust comments | 5                      | about 26                              | 20 / 20      | 0.20 s |
| harper-cli 3.1.0      | Markdown, Rust comments | 5                      | about 70                              | 20 / 20      | 1.2 s  |
| cspell 10.4.1 (today) | every file type         | 8                      | about 121                             | 20 / 20      | 1.7 s  |

- **A typo only dictionary checkers find**: `sumission` (for "submission") in
  the description of the city-submission endpoint
  (`lambdas/src/core/resource/cities/endpoint.rs:225`), copied into both OpenAPI
  files and the `bnaclient` doc comments. typos does not know this misspelling.
  Vale and harper find it in `bnaclient/src/lib.rs`'s doc comment, cspell finds
  it too. The other four typos Vale and harper find are among typos' 15.
- **What only typos finds**: 5 misspelled words (8 occurrences) in identifiers,
  string literals and the `justfile` (a variable, a parameter, a region name in
  `examples/query.rs`, two comments of the `justfile`), which neither Vale nor
  harper reads.
- **Vale**: its words to add are project terms (`justfile`, `DTOs`, `subscore`,
  `sharrows`, `bikeable`…) and `snake_case` identifiers quoted in comments
  (`city_id`, `page_size`), which a `TokenIgnores` pattern could skip instead.
  It also suggests capitalizations (`Uuid` → `UUID`, 3 times). Seeded document:
  20 of 20 typos, and 6 jargon words flagged (`pgrouting`, `justfile`, `serde`,
  `chrono`, `tokio`, `rustfmt`), against cspell's 3. MIT, 6,371 stars,
  maintained since 2017, monthly releases, but one author holds 2,120 of the
  commits. Release binaries (55 MB unpacked) with checksums and a GitHub
  attestation; an official `errata-ai/vale-action`; not in
  `taiki-e/install-action`.
- **harper**: the §5.8 rejection mostly stands. The word list it needs is almost
  three times Vale's, and its CLI is still described as a debugging tool.
- **cspell** checks everything and finds 8 typos, but would need about 121 words
  for the whole repository (many are hexadecimal identifiers and test data), and
  it remains an `npx` package.

So no single tool covers both kinds of typo: typos covers code and known
misspellings, while a dictionary checker (Vale) covers unknown misspellings in
prose. Running both costs about 0.2 s.

### 5.17 Auto-fixing in `just fmt` — works once the two link definitions are shortened

A candidate `just fmt` (oxfmt, then `rumdl check --fix`, then
`typos --write-changes`), followed by the checks `just lint` would run
(`rumdl check`, `oxfmt --check`, `typos`), on the original repository copy
(script `autofix.sh` in the scratch directory, both orders of oxfmt and rumdl):

- **The fixes conflict on the two long link definitions** of §5.1. rumdl's MD034
  fix wraps the URL in angle brackets (`<https://…>`), and oxfmt removes them
  again. Whatever the order, `just fmt` ends in a state that `just lint`
  rejects: with oxfmt first, `oxfmt --check` fails on both files; with rumdl
  first, rumdl reports both MD034 findings.
- **Shortening the reference labels removes the conflict**: with
  `[BNA Mechanics Code of Conduct][code of conduct]` and
  `[code of conduct]: https://peopleforbikes.github.io/code-of-conduct/` (69
  characters, so the formatter keeps it on one line), and the same for the
  contributing guidelines, both orders give a clean `rumdl check` and
  `oxfmt --check`, and a second `just fmt` changes nothing.
- **`typos --write-changes`** applies only corrections with a single suggestion.
  On the repository it fixed 13 of the 15 findings, including renames of a
  `justfile` variable (3 uses) and of a function parameter in
  `lambdas/src/lib.rs`; the renames were consistent, but code changes still need
  a compile check and a review. The 2 findings with several suggestions are left
  for a person.
- **rumdl's fixes** cover formatting-like rules; on the seeded document, 11
  findings (duplicate headings, empty links, missing alt text, broken
  fragments…) are not fixable and still fail `just lint`, as expected.

## 6. Conclusion & Recommendation

**Conditional Go**: replace the three Markdown tools with Rust tools, keep the
DBML and OpenAPI generators on `npx` but pinned, and remove the `sql2dbml`
recipes.

| Tool                 | Decision                             | Evidence                                                                                                                        |
| -------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------- |
| markdownlint-cli2    | Replace with **rumdl**               | Reads `.markdownlint.yml` unchanged; same findings on realistic defects but one edge case; 0.05 s against 0.74 s (§5.1, §5.2)   |
| prettier             | Replace with **oxfmt** (binary)      | Byte-identical to prettier on all our files with `printWidth: 80`; stable; 0.013 s against 0.85 s (§5.4)                        |
| cspell               | Replace with **typos**, whole repo   | No false positives in Markdown; catches a typo cspell misses (`tha`) and 14 more in code; misses typos it has never seen (§5.7) |
| dbml2sql             | Keep, pin `@dbml/cli@12.0.0`         | No Rust tool generates SQL from DBML (§5.11); the Go `dbml-tools` renames the foreign keys (§5.12)                              |
| dbml-renderer        | Keep, pin `@1.0.31`                  | No Rust tool renders DBML (§5.11); the Go `dbml-toolkit` drops composite primary keys (§5.13)                                   |
| openapi-down-convert | Keep, already pinned to `0.14.2`     | The Rust port produces invalid 3.0 and drops `servers` (§5.10); the Go `oastools` only changes the version (§5.14)              |
| sql2dbml             | Remove `dbml-from-sql`, `db-to-dbml` | `CLAUDE.md` already says never to use them                                                                                      |

Conditions:

- **rumdl**: shorten the reference labels of the two link definitions that
  trigger its MD034 false positive (`code-of-conduct.md`,
  `.github/CONTRIBUTING.md`) so each fits on one line. Joining them, as first
  proposed, does not hold: the formatter splits them again, and rumdl's fix then
  conflicts with oxfmt (§5.17); accept its own numbering above MD059 and its
  most-common marker rule for MD004. One maintainer holds nearly all commits, so
  pin the version and keep markdownlint-cli2 as the fallback.
- **oxfmt**: set `printWidth: 80` explicitly (the default is 100), and install
  the standalone binary by downloading the release archive and checking it with
  `gh attestation verify`, because `taiki-e/install-action`'s manifest does not
  know the `oxfmt_v*` tags. Expect frequent version bumps while it is `0.x`.
- **typos**: the team accepts its model. It reports only known misspellings, so
  it never flags jargon, but it found 18 of 20 seeded typos where cspell found
  20, and misses never-seen misspellings such as the one in the city-submission
  endpoint's description (§5.16). In exchange it checks the code too.
  Re-examined once a word list became acceptable (§5.16), the team kept typos
  alone: Markdown is the main use case and a second tool (Vale) was judged
  overkill. `words.txt` and `cspell.config.yaml` go away; `_typos.toml` allows
  `ND`, `nd` and `claus`, and can teach typos a misspelling once spotted
  (`<typo> = "<correction>"`).
- **`just fmt` and `just lint`**: `just fmt` applies every automatic fix (oxfmt,
  `rumdl check --fix`, `typos --write-changes`) and `just lint` only checks;
  once the two link definitions are shortened, `just lint` passes after
  `just fmt` and a second `just fmt` changes nothing (§5.17). Renames made by
  `typos --write-changes` in code need a review and a compile check.

Outcome for Node.js: it leaves the Markdown jobs (`ci-rust.yml`) and the local
`fmt`/`lint` recipes, and stays in the two CI jobs that generate committed files
(`integration.yml`'s `dbml-check`, `bnaclient.yml`), where every package is then
pinned. Removing Node.js entirely is not possible today.

Rejected: mado (Ruby `mdl` rule set, 20 false positives), `rumdl fmt` (output
fails MD013), harper (126 false positives), codebook (dictionaries downloaded
from another project's `main` branch, 15 false positives), openapi31to30
(invalid 3.0). dprint works but differs from prettier and needs a second
download host; it is the fallback if oxfmt stalls.

Go candidates (review extension): none replaces the three generators. They have
the qualities asked for (single binaries, 27 to 275 times faster, checksummed
releases), but `dbml-tools` changes the generated schema, `dbml-toolkit` loses
information in the diagram, and `oastools` does not convert at all. All three
are young projects with one author. The decisions above are unchanged.

Any language (second review extension): still none (§5.15). The DBML projects
outside JavaScript are libraries, the Graphviz pipeline rewrites the column
types, and the OpenAPI downgraders are a .NET tool, a Python package or a
Swagger 2.0 converter. Compiling the JavaScript tools into binaries was ruled
out by the team.

## 7. Still Open

- **Linux was not run.** Every run was on macOS arm64. The Linux x86_64 archives
  of rumdl, oxfmt and typos exist but their output on the runners is unverified;
  the first CI run of the feature settles it.
- **The rumdl MD034 false positive** (a link reference definition with its URL
  on the next line) is not reported upstream yet.
- **typos' recall** was measured on 20 seeded typos only. Whether missing
  never-seen typos matters in practice will show over time; Vale (§5.16) is the
  candidate to add if it does.
- **CI scope** (settled by the team): CI runs `just lint` (lint, formatting
  check and spellcheck), stops at the first failure, and never formats.
- **The other PeopleForBikes repositories** (for example `brokenspoke`) were not
  tested; their Markdown may use features ours does not (MDX, TOML front matter,
  math), where oxfmt differs from prettier (§5.4).
- **Generators**: Go candidates were tested (§5.12 to §5.14) and other languages
  searched (§5.15). Microsoft's `hidi` was not run, since it is not a single
  binary; whether it converts our file correctly is unknown.
- **dbml-toolkit is worth a second look** if it learns to show composite primary
  keys and keeps releasing: it would remove Node.js from `dbml-svg`, which no CI
  job runs. The missing primary keys are not reported upstream.
- **The PostgreSQL comparison for dbml-tools** was not run (the sandbox forbids
  PostgreSQL's shared memory); its rejection rests on the generated SQL text
  (§5.12).
- **progenitor and OpenAPI 3.1**: if progenitor ever reads 3.1, the
  down-conversion and its Node dependency disappear.
- **mado's two hangs** stay unexplained; moot since it is rejected.
- **Editor integration** was not a constraint and was not evaluated.

## 8. Remaining Work

In priority order:

1. Pin the remaining `npx` packages: `@dbml/cli@12.0.0` (`justfile:91`,
   `justfile:140`) and `@softwaretechnik/dbml-renderer@1.0.31` (`justfile:144`).
   Independent of the rest, and it removes the unpinned downloads from CI at
   once.
2. Replace `lint-md` with rumdl in the `justfile` and `ci-rust.yml` (pinned,
   through the `rvben/rumdl` action or `cargo-binstall`), after shortening the
   two link reference labels (§5.17).
3. Replace `fmt-md` with oxfmt: `.oxfmtrc.json` (`printWidth: 80`,
   `proseWrap: "always"`), a pinned and attested install, and its `--check` in
   `just lint`. Remove `.prettierignore`.
4. Replace cspell with typos on the whole repository: fix the 15 findings it
   reports and the never-seen misspelling of §5.16 (`enpoint`, `entites`,
   `defaul`, `paramaters`, `markown`, `accross`, `Massachussets`, `onces`,
   `tha`, `sumission`), add `_typos.toml`, remove `cspell.config.yaml` and
   `words.txt`.
5. Make `just fmt` apply every automatic fix (oxfmt, `rumdl check --fix`,
   `typos --write-changes`) and `just lint` check only; CI runs `just lint` and
   stops at the first failure.
6. Remove the `dbml-from-sql` and `db-to-dbml` recipes and the stale
   `docs/themes` ignores.
7. Update `CLAUDE.md`, `.github/CONTRIBUTING.md` and the contributor setup (no
   Node.js needed for Markdown work).
8. Carry the decision to the other PeopleForBikes repositories, checking their
   Markdown against §5.4's divergences first.
