# Findings: Tool Manager for Pinned Command-Line Tools

Everything non-obvious learned while implementing this feature: behavior that
looks like a bug unless you know it is deliberate, traps in the libraries and
tools, and what worked. Each entry says what the rule is, how it was caught, and
what it cost.

Status: in progress, tasks 1 to 6 done; task 7 (CI on the pull request) waits
for the maintainer. Measured with mise 2026.11.6 on macOS arm64, in the nono
sandbox, on 2026-11-11.

## 1. Behavior that does not survive a naive implementation

### 1.1 A locked install skips the attestation unless asked to verify it again

`mise.lock` records `provenance = "github-attestations"` for oxfmt, rumdl, typos
and hurl, but when the entry also has a checksum, `mise install` only compares
the download with that checksum: the verbose log of a clean install shows the
`checksum` step for the five release archives and no attestation step at all.
`locked_verify_provenance = true` (`mise.toml:16`) makes every install verify
the recorded provenance again: the log then shows
`✓ GitHub artifact attestations verified` for the four attested tools, and just,
with no recorded provenance, installs on its checksum
(`No GitHub artifact attestations found … trying SLSA`, then installed).

**Caught by**: task 1.3, reading the `mise install --verbose` log for the
attestation check the task asked for. The spike had noted that the check "may"
be skipped (`spike.md` §5.0) but the design did not act on it.

**Cost**: without the setting, Requirement 2.3 is not met: the attestation is
checked once, by the maintainer who runs `mise lock`, and never by contributors
or CI. With it, a clean install of the eight tools took 1 min 31 s (the cargo
builds dominate; 1 min 40 s without it).

### 1.2 A pinned tool that is not installed falls back to the `PATH`

With `auto_install = false`, mise never installs a missing tool, but it does not
fail either:

- `mise exec -- rumdl --version`, with rumdl configured but not installed, warns
  (the message below), then runs the first `rumdl` on the `PATH`. With none, it
  fails (`"rumdl" couldn't exec process`, exit 1); with a stand-in script
  printing `unpinned rumdl 9.9.9` on the `PATH`, it runs it and exits 0.

  ```text
  mise WARN  github:rvben/rumdl@0.3.2 is not installed and auto-install is disabled, so mise looks for rumdl on PATH instead. Install it with: mise install github:rvben/rumdl@0.3.2
  ```

- mise creates shims only for installed tools (the shims directory held `just`
  and `typos` only), so there is no `rumdl` shim: with the shims first on the
  `PATH`, the shell runs the next `rumdl` on the `PATH` silently, or fails with
  `command not found` (127) when there is none.
- No setting makes this strict: `mise settings ls --all` lists
  `exec_auto_install` and `not_found_auto_install`, which only control
  installing.

In both cases the data directory listing is the same before and after (nothing
installed).

**Caught by**: task 1.5.

**Cost**: Requirement 2.7 as first written ("SHALL fail naming the missing
tool") held only when no other copy of the tool was on the `PATH`. The
maintainer chose to describe mise's behavior instead of adding a guard: 2.7 now
requires no install and a warning from `mise exec`, the new 2.8 requires the
failure when there is no other copy, and 6.1 asks `.github/CONTRIBUTING.md` to
say to run `mise install` again after pulling a change to `mise.toml`.

## 2. Library and tooling traps

### 2.1 The first `mise lock` warns about every tool when `locked = true`

Generating `mise.lock` for the first time, with `locked = true` already in
`mise.toml`, prints one
`Failed to resolve tool version list for … is not in the lockfile` warning per
tool (eight), then locks the 32 targets and exits 0 in 8.6 s. The warnings are
harmless: `mise lock` is the command that fills the lockfile.

### 2.2 `hurl --version` names the wrong architecture on Apple silicon

The hurl installed from `hurl-8.1.0-aarch64-apple-darwin.tar.gz` prints
`hurl 8.1.0 (x86_64-apple-darwin26.0) …`, but `file` reports
`Mach-O 64-bit executable arm64`: the label comes from hurl's build, not from
the archive mise chose. Check the binary with `file`, not the version string.

### 2.3 cargo-progenitor has no `--version`

`cargo progenitor --version` fails
(`a value is required for '--version <VERSION>'`: the flag sets the generated
crate's version). `mise ls --installed` shows the installed version of every
tool instead, including the Crate_Tools.

### 2.4 nono puts its own helper directory ahead of mise's shims

Inside a session started by `just nono-claude`, the first `PATH` entry is a
temporary directory nono adds (`/var/folders/…/T/nono-shim-<random>`), and
mise's shims directory comes second, right after it. nono's directory holds only
`open` and `nono-open-url-helper`, which open URLs from inside the sandbox, so
it cannot hide a Pinned_Tool: `command -v typos` still gives
`~/.local/share/mise/shims/typos`. The recipe cannot change this; it controls
only the `PATH` it passes to `nono run`.

**Caught by**: task 4, `echo "$PATH"` in a real session run by the maintainer.

**Cost**: Requirement 4.2 first said "first on the `PATH`", which no recipe can
guarantee. The maintainer reworded it: the shims come ahead of every directory
of the contributor's `PATH`.

## 3. Method — what actually worked

### 3.1 Scratch mise directories for every install

Every check sets `MISE_DATA_DIR`, `MISE_CONFIG_DIR`, `MISE_STATE_DIR` and
`MISE_CACHE_DIR` to empty folders in the scratch directory, then runs
`mise trust mise.toml` there. The contributor's own installs are never touched,
and each run starts clean, so "nothing installed" can be shown by listing the
data directory. Inside nono this is also required: mise's default state
directory is read-only in the sandbox
(`failed create_dir_all: ~/.local/state/mise/incomplete-installs`).

### 3.2 Verification record

- **1.2**: `mise lock --platform linux-x64,linux-arm64,macos-arm64,macos-x64`,
  exit 0, 162 lines. A scratch script (`check_lock.py`, reading the lockfile
  with `tomllib`) checks properties 2 and 3: the three Crate_Tools have their
  version only; each Release_Tool has the four platforms with a `sha256:`
  checksum and a URL naming that platform's target; the four Attested_Tools have
  `provenance = "github-attestations"` on every platform; just has none. Every
  archive is the one `design.md` lists, and the oxfmt URLs point to the
  `oxfmt_v0.78.0` tag. Note: the platform tables are quoted keys
  (`[tools."github:crate-ci/typos"."platforms.macos-arm64"]`), not nested
  tables.
- **1.3**: plain `mise install --verbose` (no `--locked`), exit 0 in 1 min 31 s.
  `mise exec -- <tool> --version` gives just 1.59.0, typos-cli 1.52.1, oxfmt
  0.78.0, rumdl 0.3.2, hurl 8.1.0, bacon 3.26.0, sea-orm-cli 2.0.4;
  `mise ls --installed` gives cargo-progenitor 0.16.0. The log shows
  `cargo install <crate>@<version> --locked` for the three crates. The install
  left `mise.lock` byte-identical. With the scratch shims directory first on the
  `PATH`, `just lint` exits 0.
- **1.4**: on scratch copies, `mise install github:crate-ci/typos` exits 1 in
  each case, typos is not in `mise ls --installed`, and the data directory holds
  no file:
  - checksum altered (first hex digit): `Checksum mismatch for file …`, with the
    expected and actual digests;
  - checksum line removed:
    `No lockfile checksum found for github:crate-ci/typos@1.52.1 on platform macos-arm64`;
  - `platforms.macos-arm64` table removed: `No lockfile URL found for …`, with
    the same tool and platform, and `(--locked mode)`.
- **1.5**: after `mise install github:casey/just github:crate-ci/typos`, the
  shims directory holds `just` and `typos` only. With no other rumdl on the
  `PATH`, `mise exec -- rumdl --version` warns (§1.2) and exits 1, and `rumdl`
  with the shims first on the `PATH` gives `command not found` (127); the
  `typos` shim runs typos-cli 1.52.1. A `find` listing of the data directory is
  identical before and after.
- **2.1 to 2.3**: the four `jdx/mise-action` steps parse (Ruby's YAML loader)
  with `version: 2026.11.5` and the job's `install_args`; the folded
  `install_args: >-` of the lint job reads as one space-separated line. With
  empty scratch directories, `mise install <install_args>` of each job exits 0
  and `mise ls --installed` lists exactly that job's tools (lint: just, typos,
  oxfmt, rumdl; bnaclient: just, cargo-progenitor; integration test: just, hurl;
  schema: just). Installing only the lint job's tools still verifies the
  attestations of oxfmt, rumdl and typos (`--verbose`). `mise.lock` stays
  byte-identical.
- **2.4**:
  `git grep -n -E 'setup-just|install-action|gh release download|_VERSION' .github/`
  finds nothing; `git diff --stat d927dea -- .github/workflows` shows 18
  insertions and 39 deletions in the three workflows, all in the install steps
  and the comment of the integration `test` job.
- **3**: with just 1.59.0, `just --fmt --check --unstable` exits 0.
  `just --evaluate` gives `~/.local/share/mise`, `~/.config/mise` and
  `~/.local/state/mise`; `mise_state` follows `XDG_STATE_HOME` (adding `/mise`)
  and, when both are set, `MISE_STATE_DIR`. The dry runs of both recipes show
  the `mkdir -p`, the shims first on the `PATH`, `--allow-cwd`,
  `--profile pfb-claude` (or the profile given), two `--read`, one `--allow`,
  and `-- claude` or `-- claude --dangerously-skip-permissions`. Running
  `just nono-claude` with a stand-in `nono` script on the `PATH` and the three
  `MISE_*_DIR` pointing to missing scratch folders: exit 0, the folders are
  created, the stand-in receives the flags, and the first entry of its `PATH` is
  the shims directory. `git diff -- justfile` changes only the variables and the
  two recipes.
- **5**: `.github/CONTRIBUTING.md` gets a "Local tools" section (prerequisites,
  setup, what it protects against, bumping a tool) that names no tool version,
  and `CLAUDE.md` points to `mise.toml`. oxfmt only rewrapped three paragraphs;
  `just lint` passes.
- **6**: with the scratch shims first on the `PATH`, `just fmt` exits 0 and
  leaves every changed file byte-identical (MD5 before and after), and
  `just lint` exits 0. With `PATH=/usr/bin:/bin` and just called by its full
  path, `just lint` exits 127 (`lint-md: oxfmt: command not found`). The version
  `git grep` of task 6 (with `--untracked`) finds only `bnaclient/Cargo.toml:11`
  (`progenitor-client`). `git diff --stat d927dea` lists the three workflows,
  `.github/CONTRIBUTING.md`, `CLAUDE.md` and `justfile`; `git status` adds
  `mise.toml`, `mise.lock` and `specs/0003-tool-manager/` as new files.
- **4** (real session, run by the maintainer with mise's default directories,
  mise 2026.11.6): `just nono-claude` starts the session with the two `--read`
  flags and the `--allow` flag. Inside it: the `PATH` starts with nono's helper
  directory, then `~/.local/share/mise/shims` (§2.4); `command -v typos` gives
  the shim, which runs typos-cli 1.52.1; `just lint` exits 0 (oxfmt and rumdl on
  34 files, typos silent); `touch ~/.local/share/mise/x` exits 1 with
  `Operation not permitted`, and `nono why` reports that mise's data directory
  grants read access only (`insufficient_access`). With
  `MISE_STATE_DIR=/tmp/mise-state-test` (missing beforehand), the recipe created
  the folder and the session started; inside it, `mise ls --current` stopped
  with `Config files in … mise.toml are not trusted`, as expected since mise
  keeps its trust records in the state directory, and mise wrote
  `incomplete-installs/.migrated` there without a denial (that folder is under
  `/tmp`, which the profile already grants, so this does not test the `--allow`
  flag). In a session with the default state directory, `mise ls --current`
  lists the eight tools from `mise.toml` at their pinned versions, none missing,
  with no trust error and no denied write.
