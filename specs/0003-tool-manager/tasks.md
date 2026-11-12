# Implementation Plan: Tool Manager for Pinned Command-Line Tools

## Overview

The work goes from the source of truth outwards, one commit per task or
sub-task, so each step can be checked on its own:

1. Add `mise.toml` and generate `mise.lock`, and check locally that a clean
   Locked_Install works, refuses a broken lockfile and never installs behind a
   command's back.
2. Switch each CI workflow to `jdx/mise-action`, one workflow per commit.
3. Change the Agent_Recipes, then check them once in a real nono session, run by
   the maintainer outside the sandbox.
4. Document the setup in `.github/CONTRIBUTING.md` and point `CLAUDE.md` at
   `mise.toml`.
5. Check everything locally, then on the pull request's CI.

Prerequisites on the implementer's machine: mise 2026.11.5 or newer, a Rust
toolchain (for the Crate_Tools), `gh auth login` or `GITHUB_TOKEN` (GitHub's
anonymous rate limit), and network access to GitHub (API and release downloads)
and crates.io. Every check that installs tools points mise's directories
(`MISE_DATA_DIR`, `MISE_CONFIG_DIR`, `MISE_STATE_DIR`, `MISE_CACHE_DIR`) at
empty folders in the scratch directory, so the contributor's own installs are
never touched. The commands and their results go in `findings.md`.

## Tasks

- [x] 1. Pin the tools in `mise.toml` and `mise.lock`
  - [x] 1.1 Add `mise.toml` at the repository root, with the content of
        `design.md` (header comment, the five `[settings]` with their comments,
        the eight `[tools]`).
    - _Requirements: 1.1, 1.2, 1.3_
  - [x] 1.2 Generate `mise.lock` with
        `mise lock --platform linux-x64,linux-arm64,macos-arm64,macos-x64`.
    - Check it with a script in the scratch directory (not committed): each
      Release_Tool has exactly the four Lockfile_Platforms, each with a
      `checksum`; each `url` names that platform's target (the archives listed
      in `design.md`); oxfmt, rumdl, typos and hurl have
      `provenance = "github-attestations"` on every platform; the Crate_Tools
      have their version only.
    - _Requirements: 1.4, 1.5_
  - [x] 1.3 Clean install, with mise's directories in empty scratch folders:
        `mise trust`, then `mise install` (no `--locked`) in the checkout.
    - Every Pinned_Tool reports the version of `mise.toml` (`--version` of each,
      through `mise exec`).
    - The install log (`mise install --verbose`) shows the checksum check of
      each Release_Tool and the attestation check of oxfmt, rumdl, typos and
      hurl, and the Crate_Tools built by `cargo install --locked`.
    - With the installed tools first on the `PATH` (scratch shims directory),
      `just lint` exits 0.
    - _Requirements: 2.1, 2.2, 2.3, 2.6, 3.2_
  - [x] 1.4 Negative checks, each on a scratch copy of the checkout with empty
        mise directories: an altered `checksum`, a removed `checksum`, and a
        removed platform entry for the contributor's platform, each in
        `mise.lock` for typos.
    - Each makes `mise install` fail (non-zero exit, `Checksum mismatch` for the
      altered checksum, the tool and platform named for the other two), and
      typos is not installed (`mise ls` and the data directory).
    - _Requirements: 2.4, 2.5_
  - [x] 1.5 No silent install: with empty mise directories, install only
        `github:casey/just` and `github:crate-ci/typos`, then, with no other
        rumdl on the `PATH`, run `mise exec -- rumdl --version` and `rumdl` with
        the shims first on the `PATH`.
    - `mise exec` warns naming rumdl and the `mise install` command, then fails;
      the shell finds no `rumdl` (mise has no shim for it, exit 127); a listing
      of the data directory is the same before and after.
    - _Requirements: 1.3, 2.7, 2.8_
  - Commit `mise.toml` and `mise.lock` together.

- [x] 2. Install the CI tools with `jdx/mise-action`
  - Line numbers are those of commit `d927dea`. Every step uses
    `jdx/mise-action@b44d12e7d0d8ee7461bb893e1b1993500433185b # v5.2.7` with
    `version: 2026.11.5`, `cache: false` (added after the first CI run,
    `findings.md` §2.5) and the default `github_token` (steps in `design.md`).
  - Local check for each sub-task, with empty scratch mise directories:
    `mise install <the job's install_args>` installs exactly those tools from
    `mise.lock` and nothing else (`mise ls --installed`).
  - [x] 2.1 `.github/workflows/ci-rust.yml`, `lint` job: replace
        `extractions/setup-just` (line 15) and the "Install oxfmt, rumdl and
        typos" step with its `env` block (lines 16-41) with the action, whose
        `install_args` are `github:casey/just`, `github:crate-ci/typos`,
        `github:oxc-project/oxc` and `github:rvben/rumdl`. `just lint` stays the
        next step; the `test` and `build` jobs are unchanged.
    - _Requirements: 5.1, 5.2, 5.3, 5.4, 5.6, 5.7_
  - [x] 2.2 `.github/workflows/bnaclient.yml`, `test` job: remove
        `extractions/setup-just` (line 24) and `taiki-e/install-action` (lines
        28-31), and add the action after `dtolnay/rust-toolchain@nightly`
        (`install_args: github:casey/just cargo:cargo-progenitor`). The
        remaining steps are unchanged.
    - _Requirements: 5.1, 5.2, 5.3, 5.4, 5.6, 5.7_
  - [x] 2.3 `.github/workflows/integration.yml`:
    - `test` job: replace `extractions/setup-just` (line 42) and the "Install
      HURL" step (lines 43-47) with the action
      (`install_args: github:casey/just github:Orange-OpenSource/hurl`); keep
      `runs-on: ubuntu-24.04` and change its comment (line 17) to "Hurl needs
      libxml2, which Ubuntu 26.04 no longer provides".
    - `schema` job: replace `extractions/setup-just` (line 138) with the action
      (`install_args: github:casey/just`). The PostgreSQL client install
      (line 145) stays.
    - _Requirements: 5.1, 5.2, 5.3, 5.4, 5.6, 5.7_
  - [x] 2.4 Verify the three workflows: this command finds nothing, and
        `git diff d927dea -- .github/workflows` changes only the install steps
        and the comment of task 2.3.

    ```bash
    git grep -n -E 'setup-just|install-action|gh release download|_VERSION' .github/
    ```

    - _Requirements: 5.6, 5.7_

- [x] 3. Give agents the tools in the Agent_Recipes
  - In `justfile`, add the three variables `mise_data`, `mise_config` and
    `mise_state` with their comment, and replace `nono-claude` and
    `nono-claude-danger` (lines 194-204) with the recipes of `design.md`.
    `nono-claude-login` is unchanged.
  - Verify:
    - `just --fmt --check --unstable` passes (the formatting of `design.md` is
      kept).
    - `just --evaluate mise_data` (and the two others) gives the default paths;
      with `MISE_STATE_DIR`, then `XDG_STATE_HOME`, set to a scratch path,
      `mise_state` follows it.
    - `just --dry-run nono-claude` and `just --dry-run nono-claude-danger` show
      the `mkdir -p`, the shims first on the `PATH`, `--allow-cwd`,
      `--profile pfb-claude`, two `--read`, one `--allow` and the same `claude`
      arguments as before.
    - `git diff d927dea -- justfile` changes nothing but the variables and the
      two recipes.
  - _Requirements: 3.1, 4.1, 4.2, 4.4, 4.5_
  - [x] 3.1 Added after the spec review: in both recipes, grant `--read-file` to
        the `mise` on the caller's `PATH` and to the file it resolves to, only
        when there is one, as in `design.md`. Verify with a stand-in `nono` that
        prints its arguments: Homebrew's mise gives the link and its target, a
        plain `mise` file gives that file (twice), no `mise` gives no
        `--read-file` and the recipe still starts nono;
        `just --fmt --check --unstable` passes.
    - _Requirements: 4.1, 4.3_

- [ ] 4. Check the Agent_Recipes in a real nono session
  - Run by the maintainer, outside the sandbox (an agent cannot start nono
    inside nono): in the checkout, `mise trust` and `mise install` with mise's
    default directories, then `just nono-claude`.
  - Inside the session:
    - In `echo "$PATH"`, mise's shims directory comes before every directory of
      the contributor's `PATH` (nono's own helper directory may come first);
      `command -v typos` names the shim.
    - `just lint` exits 0.
    - Writing a file in mise's data directory fails; mise can write its state
      directory (no `tracked-configs` error).
  - Then start `MISE_STATE_DIR=<a missing scratch path> just nono-claude`: the
    recipe creates the folder and the session starts.
  - After task 3.1, start `just nono-claude` again: the session starts (nono
    accepts the `--read-file` flags) and `just lint` exits 0. If possible, also
    with mise installed by its own installer in `~/.local/bin` (shims pointing
    there, after `mise reshim`).
  - Record whether `nono run` accepted the repeated `--read` flags and passed
    the `PATH`. If either fails, stop and update the specification
    (`pfb-spec-update`) before going on.
  - _Requirements: 4.1, 4.2, 4.3, 4.5_

- [x] 5. Document the tools
  - [x] 5.1 In `.github/CONTRIBUTING.md`, add the "Local tools" section after
        "General guidelines", as outlined in `design.md`: prerequisites (mise
        2026.11.5 or newer, a Rust toolchain, `libxml2` on Linux, `gh` login or
        `GITHUB_TOKEN`), setup (`mise trust` or `trusted_config_paths`,
        `mise install`, `mise activate` or the shims, `just nono-claude` for
        agents, `mise install` again after pulling a change to `mise.toml`),
        what it protects against, and how to bump a tool. The section names no
        tool version.
    - _Requirements: 6.1, 6.2, 6.3_
  - [x] 5.2 In `CLAUDE.md` (lines 44-47), replace "at the versions pinned in the
        `env` block of `.github/workflows/ci-rust.yml`" with "at the versions
        pinned in `mise.toml` (installed with `mise install`, see
        `.github/CONTRIBUTING.md`)".
    - _Requirements: 6.4_
  - [x] 5.3 In `lambdas/README.md`, replace the "Remark" (a `cargo binstall` of
        other versions of bacon, hurl, just and sea-orm-cli) with a pointer to
        the "Local tools" section of `.github/CONTRIBUTING.md`, and drop the
        `[cargo binstall]` link. Added after the spec review.
    - _Requirements: 1.6_
  - Verify: `just lint` passes on the three files.

- [x] 6. Checkpoint - Local quality
  - `just fmt` changes nothing; `just lint` passes with the tools of task 1.3 on
    the `PATH`.
  - With a `PATH` without oxfmt, rumdl and typos (`just` called by its full
    path), `just lint` fails with a non-zero status (127).
  - One source of versions: this command, which matches a tool's name followed
    by any version (not only the pinned one), finds only
    `bnaclient/Cargo.toml:11` (`progenitor-client`, a library the client depends
    on, not the tool) and the four `s3_bucket` paths of
    `lambdas/tests/endpoints/pipelines.hurl` (test data such as
    `tennessee/hurl/25.06`).

    ```bash
    git grep -n -i -E \
      '(oxfmt|rumdl|typos|just|hurl|bacon|sea-orm-cli|progenitor)[^0-9]{0,20}[0-9]+\.[0-9]+' \
      -- ':!specs/' ':!CHANGELOG.md' ':!Cargo.lock' ':!mise.toml' ':!mise.lock'
    ```

  - `git diff d927dea --stat` lists only `mise.toml`, `mise.lock`, the three
    workflows, `justfile`, `.github/CONTRIBUTING.md`, `CLAUDE.md`,
    `lambdas/README.md` and `specs/0003-tool-manager/`.
  - _Requirements: 1.6, 3.1, 3.3_

- [ ] 7. Run the checks in CI
  - Ask the maintainer before pushing; push over SSH only.
  - [x] 7.1 On the pull request, `ci-rust`, `bnaclient` and `integration` pass.
        Each job's log shows the action installing mise 2026.11.5, running
        `mise install --locked` with that job's tools only, and the next steps
        using them; the Hurl tests pass on `ubuntu-24.04` (`libxml2` present).
    - _Requirements: 5.1, 5.2, 5.3, 5.4, 5.7_
  - [ ] 7.2 Optional, only with the maintainer's approval before the push: a
        commit altering one checksum in `mise.lock` makes the `ci-rust` `lint`
        job fail at the action, and `just lint` does not run. Remove the commit.
    - _Requirements: 5.5_

- [ ] 8. Checkpoint - Final
  - Run the checks of task 6 again.
  - The pull request's CI is green.
  - Every acceptance criterion of `requirements.md` is checked off against the
    task that verified it, in `findings.md`.

## Notes

- **One commit per task or sub-task**, each signed off: task 1 (both files),
  2.1, 2.2, 2.3, 3, 5.1, 5.2. Tasks 4, 6, 7 and 8 only check, and record their
  results in `findings.md`.
- **Line numbers** are those of commit `d927dea`; no earlier task edits the same
  file, so they hold.
- **Scratch directories**: never run a check that installs tools with mise's
  default directories, except task 4, which the maintainer runs on purpose.
  Scratch copies of the checkout (task 1.4) stay in the scratch directory.
- **Without task 4**, Requirement 4.3 is not met: the dry runs of task 3 show
  the flags, not that nono honors them.
- **Task 7.2** pushes a deliberately broken commit; skip it unless the
  maintainer asks for it. Task 1.4 already shows that the install fails, and the
  action fails its step when `mise install` fails.
- **Bumping later**: edit `mise.toml`, run the `mise lock` command of its
  header, review every `url` in the diff, then `mise install` and `just lint`
  (documented in task 5.1). Dependabot updates the action's commit, not the
  `version:` input or the tools.
