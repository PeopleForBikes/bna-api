# Spike: Tool Manager for Pinned Command-Line Tools

## Status

Concluded — Conditional Go (revised after review, 2026-11-11): adopt mise for
bna-api, for contributors and CI, with explicit `github:` backends, a committed
`mise.lock` and `cargo.binstall = false`; contributors install outside nono,
agents use the installed tools with read access to mise's data and write access
to its state. See §6.

## 1. Intent

Can one tool manager install the command-line tools this repository needs, at
exact versions and with their integrity checked before use, on contributors'
macOS and Linux machines (including inside the nono sandbox), from a single
configuration file that CI can later share?

`specs/0002-rust-tooling` replaced the Node.js Markdown and spelling tools with
oxfmt, rumdl and typos, and left local installation out of scope
(`specs/0002-rust-tooling/requirements.md:37-41`): `just fmt` and `just lint`
expect the three binaries on the `PATH`, at the versions pinned in the `env`
block of `.github/workflows/ci-rust.yml:18-21`, and contributors install them by
hand (`specs/0002-rust-tooling/design.md:395-397`). This spike informs the
choice of a tool manager, or of no tool manager, for this repository and for the
other PeopleForBikes repositories such as brokenspoke.

**Constraints that shaped scope**:

- Exact versions, never ranges or `latest`.
- Integrity checked before use: a published or recorded checksum, or a GitHub
  release or build attestation. A tampered archive must be refused.
- macOS and Linux contributors, and the nono sandbox (the `pfb-claude` profile).
  Extending the profile with domains or paths is acceptable; the additions each
  candidate needs are part of the answer. _Superseded by §1.1: contributors
  install outside nono, agents inside nono only use the installed tools._
- One source of truth for the versions, readable by CI. Running the candidate in
  CI is not tested now: the CI side is assessed structurally only.
- Automated version bumps (Dependabot, Renovate): out of scope for now.

**Success bar**: on a clean macOS account inside nono, one command installs
oxfmt, rumdl and typos at the exact versions from one file, refuses a tampered
or unattested archive, and `just lint` then passes; the same file can drive CI
later; the other tools (just, cargo-progenitor, sea-orm-cli, hurl, bacon,
Node.js) are covered or have a stated reason not to be. Linux is checked in a
container for contributors.

**Success bar after review** (§1.1): outside nono, with the tool manager's
default directories, one command installs the tools at the exact versions from
one file and refuses tampering; inside nono, under `pfb-claude` plus the
smallest stated additions, an agent runs `just lint` with those tools.

## 1.1 Reframe after review

The first review of this spike changed three assumptions. The work before it is
kept: what it found about installing inside nono still applies to an agent that
installs tools itself.

1. **Contributors install the tools outside the sandbox.** nono is for AI agents
   only. The requirement becomes: tools installed outside nono must be usable by
   an agent inside it (`just fmt`, `just lint`), with the smallest profile
   change. The domains nono needed for installing (§5.1 to §5.6c, §5.6e) no
   longer weigh on the choice, and the conditions motivated only by them are
   re-checked with an open network (§5.10).
2. **Scope is bna-api.** The most important other repository is
   brokenspoke-analyzer, not brokenspoke; whether a tool manager fits it may be
   a spike of its own. §5.7 stays as an indication only.
3. **Contributors install the tool manager themselves**, as a prerequisite. How
   they do it is not part of the question.

## 2. Current Baseline

Each tool's version lives in a different place, or nowhere:

| Tool             | Where CI pins it                                                          | How CI installs it                                | Local install |
| ---------------- | ------------------------------------------------------------------------- | ------------------------------------------------- | ------------- |
| oxfmt            | `env` block, `.github/workflows/ci-rust.yml:19`                           | `gh release download` + `gh release verify-asset` | by hand       |
| rumdl            | `.github/workflows/ci-rust.yml:20`                                        | same                                              | by hand       |
| typos            | `.github/workflows/ci-rust.yml:21`                                        | same                                              | by hand       |
| just             | not pinned (`extractions/setup-just`, `.github/workflows/ci-rust.yml:15`) | the action's default (latest)                     | by hand       |
| cargo-progenitor | `0.16.0`, `.github/workflows/bnaclient.yml:31`                            | `taiki-e/install-action`                          | by hand       |
| hurl             | `8.1.0`, `.github/workflows/integration.yml:45`                           | `curl` of the `.deb`, no checksum                 | by hand       |
| sea-orm-cli      | not pinned in CI (not used by a workflow); the crates use `2.0.4`         | none                                              | by hand       |
| bacon            | not used in CI                                                            | none                                              | by hand       |
| Node.js          | not pinned: the runner's preinstalled Node.js runs the `npx` generators   | preinstalled                                      | by hand       |
| cargo-lambda     | not pinned, `.github/workflows/lambdas.yml:48`                            | `pip install cargo-lambda`                        | by hand       |
| uv               | not pinned (`lambdas/tests/justfile:8`, Schemathesis, disabled in CI)     | none                                              | by hand       |

Measured in the nono sandbox (`pfb-claude` profile, nono 0.81.0, macOS arm64),
2026-11-10:

- The CI install method itself does not work inside the sandbox:
  `gh release download v1.52.1 --repo crate-ci/typos --pattern 'typos-v1.52.1-aarch64-apple-darwin.tar.gz'`
  fails with
  `Forbidden: host release-assets.githubusercontent.com:443 is not in the allowlist`.
  GitHub redirects release downloads to `release-assets.githubusercontent.com`,
  which the profile does not allow (it allows `objects.githubusercontent.com`).
  Every candidate that downloads a GitHub release asset needs this domain.
- `nono why --self --host` also denies `raw.githubusercontent.com`,
  `nodejs.org`, `mise.jdx.dev`, `mise-versions.jdx.dev`, `ghcr.io` and the
  sigstore hosts (`tuf-repo-cdn.sigstore.dev`, `fulcio.sigstore.dev`,
  `rekor.sigstore.dev`), and allows `github.com`, `api.github.com`,
  `codeload.github.com`, `objects.githubusercontent.com`, `index.crates.io`,
  `static.crates.io` and `registry.npmjs.org`.
- The Homebrew-installed mise 2026.11.5 cannot start cleanly in the sandbox:
  `mise --version` warns `failed create_dir_all: ~/.local/state/mise/...` and
  retries `https://mise.jdx.dev/releases.tsv` three times (the version check).
  Its data and state directories (`~/.local/share/mise`, `~/.local/state/mise`)
  are not writable in the profile.

## 3. Candidates Considered

| Candidate                    | Verdict                                  | Why                                                                                                                                                                               |
| ---------------------------- | ---------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| mise (default backends)      | Tested — see §5.1, §5.10                 | Inside nono rumdl fails closed; outside it all install, but typos gets no attestation check and the npm oxfmt needs Node.js.                                                      |
| mise (`github:` backends)    | Tested — see §5.3 to §5.6e, §5.10, §5.12 | Meets the success bar on macOS and Linux, outside and inside nono; release attestations verified; tampering refused; agents need read access to mise's data and a writable state. |
| Plain recipe + versions file | Tested — see §5.6b, §5.6c, §5.11         | Works outside nono (20 s), but needs an authenticated gh and hand-coded platform names, covers release binaries only, and has no lockfile.                                        |
| aqua                         | Desk review only — see §5.8              | Removed release-attestation checks; typos and oxfmt would be trusted on first download; no Node.js, Python or cargo tools.                                                        |
| Nix / devbox                 | Rejected before testing — see §5.9       | System-wide `/nix` with sudo on macOS; nixpkgs lags the pinned versions (typos, oxfmt, hurl) and lacks cargo-progenitor.                                                          |

## 4. Evaluation Method

- Machine: macOS arm64, nono 0.81.0, profile `pfb-claude` extended for the
  experiments with `--allow-domain` flags, never written to the profile: first
  `release-assets.githubusercontent.com` and `nodejs.org` (§5.1 to §5.6b), then
  also `tmaproduction.blob.core.windows.net` (§5.6c onwards). mise 2026.11.5
  (Homebrew), gh 2.103.3, cargo-binstall from Homebrew. Every result in §5.1 to
  §5.6e is a real run, 2026-11-10 and 2026-11-11.
- Experiments run in the scratch directory, never in the repository; each
  candidate's data, cache and state directories point into it, so every run
  starts from a clean state.
- After review (§5.10 to §5.12): a third session allowed every host contacted
  before; the requested path grants did not apply, so the agent's read-only
  access was simulated with `chmod -R a-w` on scratch directories.
- Linux: fresh `ubuntu:24.04` containers on colima (Docker 29.5.2, aarch64 VM;
  x86_64 under `--platform linux/amd64` emulation), outside nono's network
  proxy.
- Desk research reads the upstream documentation through `gh api` (the sandbox
  proxy blocks WebFetch); quoted documentation is labelled as such and was not
  run.

## 5. Findings

### 5.0 Desk research — mise's lockfile and verification (documentation, not run)

From the mise documentation at `jdx/mise` `docs/` (latest release v2026.11.6,
2026-11-10):

- `mise.lock` records, per tool and per platform, the resolved version, the
  download URL and a checksum; `mise lock --platform linux-x64,macos-arm64`
  fills platforms other than the current one; `mise install --locked` fails when
  a tool, platform URL or checksum is missing (`docs/dev-tools/mise-lock.md`).
- The `github` backend checks the digest GitHub reports for the asset or a
  checksum file of the release, and GitHub artifact attestations when the
  release has them; mise 2026.4.14 fixed verification of GitHub _release_
  attestations (`release-notes/2026.4.14.md`), the kind oxfmt and typos publish
  (`specs/0002-rust-tooling/findings.md` §2.4).
- `mise lock` may take checksums from `mise-versions` (the `jdx.dev` metadata
  host) without downloading the asset, marked
  `checksum_source = "mise-versions"`; they are compared with the file after a
  real download.
- When a lockfile entry has both a checksum and provenance, `mise install` may
  skip the provenance check; `locked_verify_provenance` repeats it.
- The aqua registry recipes bundled in mise declare: rumdl, a `.sha256` file and
  a build attestation signed by `rvben/rumdl/.github/workflows/release.yml`;
  hurl, a `.sha256` file; typos, oxfmt and just, no verification at all
  (`aquaproj/aqua-registry` `pkgs/*/registry.yaml`). For those three, only the
  `github` backend's asset digest and attestation checks apply.

### 5.1 mise with the default `aqua` backends — rumdl fails closed, 45 s of retries

Run (2026-11-11, mise 2026.11.5, extended sandbox:
`release-assets.githubusercontent.com` and `nodejs.org` allowed, `mise.jdx.dev`,
`mise-versions.jdx.dev` and the sigstore hosts still denied), on a scratch copy
of the repository at `d927dea`, every mise directory (`MISE_DATA_DIR`,
`MISE_CACHE_DIR`, `MISE_STATE_DIR`, `MISE_CONFIG_DIR`) in the scratch directory:

```toml
[tools]
"aqua:oxc-project/oxc/oxfmt" = "0.78.0"
"aqua:rvben/rumdl" = "0.3.2"
"aqua:crate-ci/typos" = "1.52.1"
```

`mise install --verbose` exited 1 after 45.9 s:

- Every lookup first tried `mise-versions.jdx.dev` (version lists, release
  metadata, attestations) three times with back-off, then fell back to
  `api.github.com`. Almost all of the 45.9 s is these retries.
- typos and oxfmt installed, checked only against the asset digest GitHub's API
  reports (`using GitHub API digest for checksum verification`): their aqua
  recipes declare no checksum file and no attestation (§5.0).
- rumdl failed closed: its recipe requires a build attestation signed by
  `rvben/rumdl/.github/workflows/release.yml`; the release attestation did not
  match that signer (`signer workflow mismatch`) and the build provenance needs
  the public-good sigstore trust root
  (`could not load the TUF trust root needed to verify the signature`), which
  comes from the denied `tuf-repo-cdn.sigstore.dev`. Nothing was installed for
  rumdl and the lockfile was not written.

The short registry names (`mise registry`) resolve `oxfmt` to `npm:oxfmt` first,
`rumdl`, `typos`, `just`, `hurl` and `uv` to `aqua:`, and have no entry for
`bacon`, `sea-orm-cli`, `cargo-progenitor` or `cargo-lambda`: the configuration
must name each backend explicitly.

### 5.2 The baseline's verification fails in the sandbox, misleadingly

Under the same extended sandbox, with the three macOS archives downloaded by
`gh release download` (gh 2.103.3), this command exits 1 with
`no attestations found for tag v1.52.1`:

```sh
GH_DEBUG=api gh release verify-asset v1.52.1 \
  typos-v1.52.1-aarch64-apple-darwin.tar.gz --repo crate-ci/typos
```

The API call
(`/repos/crate-ci/typos/attestations/sha1:…?predicate_type=release`) returns 200
with a sigstore bundle reference; gh then fetches the bundle from
`tmaproduction.blob.core.windows.net`, which the proxy refuses, and reports the
refusal as "no attestations". The same happens for rumdl and oxfmt. The baseline
is safe (it fails closed) but needs `tmaproduction.blob.core.windows.net` in the
profile, and its error message hides the cause. `gh` and mise both embed
`https://tuf-repo.github.com` and `https://tuf-repo-cdn.sigstore.dev` (`strings`
on both binaries); both hosts are denied.

### 5.3 mise `github` backend — all three verified by release attestation, 3.8 s

Same setup, with `MISE_USE_VERSIONS_HOST=0` and the `github` backend:

```toml
[tools]
"github:crate-ci/typos" = "1.52.1"
"github:oxc-project/oxc" = { version = "0.78.0", version_prefix = "oxfmt_v" }
"github:rvben/rumdl" = "0.3.2"
```

`mise install --verbose` exited 0 in 3.8 s (the first two tools; rumdl added
afterwards, exit 0). For each tool the log shows `checksum <asset>`, then
`verify GitHub artifact attestations` (a call to
`api.github.com/repos/<repo>/attestations/sha256:<digest>`), then
`✓ GitHub artifact attestations verified`. No sigstore or `jdx.dev` host was
contacted: mise verified GitHub's release attestations with a trust root it
ships, while gh (§5.2) could not even fetch the bundle.

`mise lock --platform linux-x64,linux-arm64,macos-arm64,macos-x64` (4.6 s) wrote
`mise.lock` with, per tool and platform, the `url`, the `checksum`, the
`url_api` and `provenance = "github-attestations"`. The three macOS arm64
checksums equal `shasum -a 256` of the archives `gh release download` fetched in
§5.2.

**Trap**: a first version pinned oxfmt with
`version = "oxfmt_v0.78.0", asset_pattern = "oxfmt-aarch64-apple-darwin.tar.gz"`.
`mise lock` then recorded the macOS arm64 archive, with its checksum, for all
four platforms: a Linux contributor would get a Mac binary that passes the
checksum. Without `asset_pattern`, mise picks the right archive per platform
(`oxfmt-x86_64-unknown-linux-gnu.tar.gz`, `oxfmt-aarch64-apple-darwin.tar.gz`,
…). Review the URLs of a new `mise.lock`, not only the checksums.

### 5.4 Success bar on macOS — one command, 2.1 s, `just lint` passes

On the scratch copy of the repository with the `mise.toml` and `mise.lock` of
§5.3 and a new empty mise state:

| Command                         | Exit | Time  | Result                                                                    |
| ------------------------------- | ---- | ----- | ------------------------------------------------------------------------- |
| `mise install --locked`         | 0    | 2.1 s | typos 1.52.1, oxfmt 0.78.0, rumdl 0.3.2 installed                         |
| `mise exec -- just lint`        | 0    | 1.1 s | `Success: No issues found in 29 files`, typos clean                       |
| `mise exec -- <tool> --version` | 0    | —     | `Version: 0.78.0`, `rumdl 0.3.2`, `typos-cli 1.52.1`, from the mise store |

### 5.5 Tampering is refused — three negative checks

Each on a new empty mise state, with a one-tool `mise.toml` (typos 1.52.1) and a
`mise.lock` written by `mise lock --platform macos-arm64`, then
`mise install --locked`:

| Change to the lockfile or the download                                                                                 | Exit | Installed | Message                                                                                     |
| ---------------------------------------------------------------------------------------------------------------------- | ---- | --------- | ------------------------------------------------------------------------------------------- |
| Checksum altered (`e342c1ec…` → `00000000…`)                                                                           | 1    | nothing   | `Checksum mismatch for file`, with a hint that GitHub's current digest matches the download |
| URL pointed at a copy of the genuine archive with one byte appended, served on `127.0.0.1:3000`, genuine checksum kept | 1    | nothing   | `lockfile entry … locks http://127.0.0.1:3000/t.tar.gz: Checksum mismatch for file`         |
| `checksum` lines removed                                                                                               | 1    | nothing   | `No lockfile checksum found … (--locked mode)`, hint `Run mise lock to record checksums`    |
| Control: the unchanged lockfile                                                                                        | 0    | typos     | —                                                                                           |

The local server had to use port 3000: the profile opens only ports 3000 and
5432 to local connections.

### 5.6 The other tools — all install, with uneven integrity

Same setup, `MISE_USE_VERSIONS_HOST=0`, mise's defaults otherwise
(cargo-binstall 1.x from Homebrew on the `PATH`):

```toml
[tools]
"github:casey/just" = "1.59.0"
"github:Orange-OpenSource/hurl" = "8.1.0"
node = "24.21.0"
"cargo:bacon" = "3.26.0"
"cargo:sea-orm-cli" = "2.0.4"
"cargo:cargo-progenitor" = "0.16.0"
```

`mise install` exited 0 in 65 s (wall; 4 min CPU), every tool at the pinned
version. `mise lock` for the four platforms took 4.8 s.

| Tool                    | How mise installed it                                                                  | Integrity checked                                                                                     | `mise.lock` records       |
| ----------------------- | -------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------- | ------------------------- |
| hurl 8.1.0              | GitHub release archive (immutable release)                                             | API digest + release attestation (`✓ GitHub artifact attestations verified`)                          | URL, checksum, provenance |
| just 1.59.0             | GitHub release archive (not an immutable release)                                      | API digest only: `No GitHub artifact attestations found … trying SLSA`, then installed                | URL, checksum             |
| Node.js 24.21.0         | `nodejs.org` archive                                                                   | `SHASUMS256.txt` and its OpenPGP signature (`.sig` downloaded; documented in `docs/lang/node.md:204`) | URL, checksum             |
| sea-orm-cli 2.0.4       | cargo-binstall: prebuilt archive of the `sea-orm-cli@2.0.4` release of `SeaQL/sea-orm` | none: the release is not immutable and the crate's binstall metadata has `signing: None`              | version only              |
| bacon 3.26.0            | cargo-binstall found no prebuilt archive, fell back to `cargo install --locked`        | crates.io index checksums (cargo)                                                                     | version only              |
| cargo-progenitor 0.16.0 | same fallback                                                                          | crates.io index checksums (cargo)                                                                     | version only              |

hurl's `--version` prints `x86_64-apple-darwin26.0`, but `file` shows a
`Mach-O 64-bit executable arm64`: hurl misreports its own target, mise picked
the right archive.

With `cargo.binstall = false` in `[settings]`, the three cargo tools build from
crates.io with `cargo install --locked` in 1 min 31 s (wall, 5 min 48 s CPU,
`~/.cargo/registry` already warm), so no unverified prebuilt binary is used.

Two consequences. A release without attestations installs on the API digest
alone: the settings reference (`jdx/mise` `settings.toml`) has no option that
makes an attestation mandatory for a `github:` tool, only the aqua registry's
`attestations_since` does for `aqua:` tools. Once `mise.lock` records
`provenance = "github-attestations"` for a tool, though, later installs require
it (`docs/dev-tools/backends/github.md`, `github_attestations`), so the lockfile
pins the requirement for every tool that had an attestation when it was locked.
And cargo tools are never checksummed in the lockfile, so their integrity rests
on crates.io and `--locked`.

### 5.6b Plain recipe + versions file — blocked in the sandbox like the baseline

A scratch `tools.txt` (one line per tool: name, repository, tag, asset template,
archive member) and a 27-line `install-tools.sh` that maps `uname -s`/`uname -m`
to a Rust target triple, then runs `gh release download`,
`gh release verify-asset`, `tar -xzf` and `install` for each tool, as the CI
step does (`.github/workflows/ci-rust.yml:23-34`).

On macOS in the extended sandbox, `install-tools.sh tools.txt .tools` exits 1
after 3.3 s with `no attestations found for tag oxfmt_v0.78.0`, for the reason
found in §5.2 (`tmaproduction.blob.core.windows.net` denied); nothing is
installed. Already visible: each tool's platform naming must be hand-coded
(typos publishes musl-only Linux archives, oxfmt's archive member carries the
target, rumdl's does not), the script installs only GitHub release binaries, and
every contributor needs an authenticated `gh`.

### 5.6c Plain recipe — needs three extra hosts in the sandbox

After a second restart that also allowed `tmaproduction.blob.core.windows.net`,
the same run exits 1 after 4.5 s with:

```text
failed to verify attestation for tag oxfmt_v0.78.0: no valid Sigstore verifiers could be initialized
```

`GH_DEBUG=api` shows the next denied host: gh downloads GitHub's sigstore trust
root from `tuf-repo.github.com` on every verification
(`Forbidden: host tuf-repo.github.com:443 is not in the allowlist`, three
attempts). The recipe therefore needs `release-assets.githubusercontent.com`,
`tmaproduction.blob.core.windows.net` and `tuf-repo.github.com` in the profile;
mise's `github` backend needed only the first (§5.3).

Integrity still holds: `gh release verify-asset` on the typos archive with one
byte appended exits 1 with
`attestation for v1.52.1 does not contain subject sha256:23923487…`, before any
signature check. The mechanism itself is the one CI already runs successfully
(`specs/0002-rust-tooling/findings.md` §3.2), so the run was not repeated with a
third grant.

### 5.6d Linux — mise passes in fresh Ubuntu containers

On colima (Docker 29.5.2, Ubuntu 24.04.4 VM, aarch64), outside nono's proxy:
`docker run --rm ubuntu:24.04` with the repository copy mounted read-only,
`apt-get install curl ca-certificates`, mise 2026.11.5 downloaded from its
GitHub release and checked with `sha256sum -c` against the digest GitHub's API
reports, then `mise install --locked` with this configuration and the
`mise.lock` written on macOS for the four platforms:

```toml
[tools]
"github:casey/just" = "1.59.0"
"github:crate-ci/typos" = "1.52.1"
"github:oxc-project/oxc" = { version = "0.78.0", version_prefix = "oxfmt_v" }
"github:rvben/rumdl" = "0.3.2"
"github:Orange-OpenSource/hurl" = "8.1.0"
node = "24.21.0"
```

| Container                                           | `mise install --locked` | Archives chosen                                                                    | `mise exec -- just lint` |
| --------------------------------------------------- | ----------------------- | ---------------------------------------------------------------------------------- | ------------------------ |
| `ubuntu:24.04`, aarch64                             | exit 0, 4 s, 6 tools    | `*-aarch64-unknown-linux-musl` (just, typos), `*-gnu` (others), Node `linux-arm64` | exit 0                   |
| `ubuntu:24.04`, `--platform linux/amd64` (emulated) | exit 0, 9 s, 6 tools    | `*-x86_64-unknown-linux-musl` (just, typos), `*-gnu` (others), Node `linux-x64`    | exit 0                   |

Every tool reported the pinned version. One gap: the Linux hurl binary is linked
against the system `libxml2` (`libxml2.so.2: cannot open shared object file`);
after `apt-get install libxml2` it prints
`hurl 8.1.0 (aarch64-unknown-linux-gnu)`. mise does not manage system libraries,
so this stays a documented prerequisite. The cargo tools were not built in the
containers (no Rust toolchain there; contributors already have one).

### 5.6e Project settings and the sandbox profile

`use_versions_host = false` in the `[settings]` table of the project's
`mise.toml` has the same effect as the environment variable: with no
`MISE_USE_VERSIONS_HOST` set, `mise install --verbose` contacted no `jdx.dev`
host (0 matches in the log) and took 3.3 s;
`mise settings get use_versions_host` prints `false`. Contributors need no
personal setting for the sandbox.

What the `pfb-claude` profile needs for an agent that _installs_ tools with mise
inside nono, from §5.1 to §5.6d (after review, contributors install outside
nono; the agents' needs are in §5.12):

- Domains: `release-assets.githubusercontent.com` (every GitHub release
  download, also needed by the current CI method), `nodejs.org` (Node.js). Not
  needed: `mise.jdx.dev`, `mise-versions.jdx.dev`, the sigstore hosts.
- Paths: mise's data and state directories (`~/.local/share/mise`,
  `~/.local/state/mise`) writable, its configuration (`~/.config/mise`)
  readable; the cache (`~/Library/Caches/mise` on macOS) is already writable.
  The experiments redirected all of them into the scratch directory instead.

CI was not run (out of scope). Structurally, `jdx/mise-action` (v5.2.7,
2026-11-06) installs the tools of `mise.toml` and adds `--locked` itself when
`mise.lock` exists (`docs/dev-tools/mise-lock.md:408`), so the same two files
can drive CI.

### 5.7 brokenspoke needs the same kind of tools

_Indication only after review (§1.1): the repository that matters most is
brokenspoke-analyzer, which this spike did not survey._

Read-only survey of `~/projects/PeopleForBikes/brokenspoke` (its `Justfile`
files and `.github/workflows/*.yml`), 2026-11-11:

- Markdown and spelling still run through unpinned `npx` (`markdownlint-cli2`,
  `cspell`, `prettier`, `Justfile:16-31`), as bna-api did before
  `specs/0002-rust-tooling`.
- Python through uv (`.python-versions`: `3.14`, `astral-sh/setup-uv` in CI) and
  poetry (`figures/Justfile`); kubectl and minikube
  (`infra/kubernetes/Justfile`); just; the stable Rust toolchain
  (`dtolnay/rust-toolchain@stable`).

Every one of these is a GitHub release binary, a cargo crate, or a mise core
tool (Python), so one polyglot manager could serve both repositories; a
binary-only manager could not cover Python, Node.js or the cargo tools.

### 5.8 aqua — desk review (documentation, not run)

From `aquaproj/aqua` `website/docs/reference/security/` (aqua v2.66.1,
2026-11-04):

- Checksums live in `aqua-checksums.json`. The expected value comes from that
  file, else from a checksum file the tool publishes, else **from the downloaded
  asset itself** (`checksum.md`, "How it works"): for typos and oxfmt, which
  publish no checksum file and whose recipes declare nothing (§5.0), the first
  download is trusted as is.
- aqua verified GitHub release attestations from v2.55.0 and removed the feature
  in v2.60.1 ("This feature was abandoned", `github-immutable-release.md`; issue
  #4862, closed). Build attestations (`github-artifact-attestations.md`) are
  verified with a gh CLI aqua installs itself, only when the recipe declares
  them (rumdl).
- aqua installs prebuilt binaries from its registry; Node.js, Python and the
  cargo tools would stay with another installer.

**Verdict**: not tested further. It is weaker than mise's `github` backend on
the exact tools of this repository (typos and oxfmt would be trusted on first
download), and it covers fewer of the tools of either repository. mise's `aqua:`
backend already gives access to the same registry when a recipe is useful.

### 5.9 Nix / devbox — desk review (not run)

- Installation: a multi-user Nix on macOS creates an APFS volume for `/nix` and
  a daemon, with `sudo`; it cannot be installed from the sandbox, and the
  profile would need `/nix` and the daemon socket.
- Exact versions: `nixpkgs` at `nixos-unstable` (`f4a03c008967`, 2026-11-07),
  read through `gh api`, has typos **1.51.1** (pinned 1.52.1), oxfmt **0.75.0**
  (0.78.0), hurl **8.0.1** (8.1.0), rumdl 0.3.2, just 1.59.0, bacon 3.26.0,
  sea-orm-cli 2.0.4, and no cargo-progenitor package. Matching the CI versions
  would mean maintaining our own derivations or overlays, and waiting for
  nixpkgs after each release.
- Integrity is strong (content-addressed store, signed binary cache,
  `flake.lock`), but the cost for contributors and the version lag do not fit
  the constraints.

**Verdict**: rejected without a hands-on test.

### 5.10 After review: mise defaults with an open network

Run 2026-11-11 in a session that allowed every host mise and gh contacted
earlier (`mise.jdx.dev`, `mise-versions.jdx.dev`, `tuf-repo.github.com`,
`tuf-repo-cdn.sigstore.dev`, `tmaproduction.blob.core.windows.net`,
`release-assets.githubusercontent.com`, `nodejs.org`), the closest this spike
gets to a contributor's machine outside nono. mise 2026.11.5, empty scratch
directories, no `MISE_USE_VERSIONS_HOST`.

**Short registry names** (`oxfmt = "0.78.0"`, `rumdl = "0.3.2"`,
`typos = "1.52.1"`): `mise install --verbose` exited 0 in 3.9 s.

| Tool  | Backend chosen        | Check before use                                                                                         |
| ----- | --------------------- | -------------------------------------------------------------------------------------------------------- |
| oxfmt | `npm:oxfmt`           | npm provenance (`provenance verified, built from github.com/oxc-project/oxc`)                            |
| rumdl | `aqua:rvben/rumdl`    | Build attestation: the release attestation fails the recipe's signer, the build provenance then verifies |
| typos | `aqua:crate-ci/typos` | GitHub API digest only (`using GitHub API digest for checksum verification`)                             |

With the sigstore hosts reachable, the rumdl failure of §5.1 disappears. Two
reasons for explicit `github:` backends remain: typos gets no attestation check
under `aqua:`, and the npm oxfmt is a shell wrapper that runs Node.js: with
`PATH=/usr/bin:/bin` it fails with `exec: node: not found`, which would bring
back the Node.js dependency for Markdown work that `specs/0002-rust-tooling`
removed.

**`github:` backends with the versions host on** (the configuration of §5.3, no
`use_versions_host` setting): `mise install` exited 0 in 4.4 s (3.8 s with it
off, §5.3); the three tools show `✓ GitHub artifact attestations verified`. The
log has 24 requests to `mise-versions.jdx.dev/api/github/…` (release metadata
and attestation bundles) instead of `api.github.com`; the signatures are still
verified locally. `mise lock` for the four platforms (4.1 s) wrote the same URLs
and checksums as the lockfile of §5.3 (`diff` of the sorted `url` and `checksum`
lines is empty), with 12 `provenance = "github-attestations"` entries and no
`checksum_source = "mise-versions"` marker.

So `use_versions_host = false` is no longer needed for speed outside nono. It
remains a choice between one more third-party service (`jdx.dev`) in the install
path and GitHub's API rate limits for unauthenticated contributors.

### 5.11 After review: the plain recipe end to end

Same open session, the script and `tools.txt` of §5.6b:
`install-tools.sh tools.txt .tools` exited 0 in 20.1 s. For each tool gh printed
`Loaded attestation from GitHub API` and
`✓ Verification succeeded! <archive> is present in release <tag>`; the installed
binaries report oxfmt 0.78.0, rumdl 0.3.2 and typos 1.52.1. The recipe works for
contributors outside nono; its limits are those of §5.6b (hand-coded platform
names, release binaries only, an authenticated gh, no lockfile), and it was five
times slower than mise here.

### 5.12 After review: an agent in nono using tools installed outside it

The path grants requested for this session did not reach it
(`nono why --self --path ~/.local/state/nono --op write` → `path_not_granted`),
so mise could not be installed into its default directories, and a strict
`pfb-claude` sandbox could not be nested inside the session
(`Failed to write config to ~/.local/state/nono/sessions/…`). The agent's view
was **simulated** instead: tools installed with the `github:` configuration and
`mise.lock` into scratch directories (the "contributor" step, network open),
then the directories made read-only with `chmod -R a-w` (the "agent" step).
These are file-permission errors, not nono denials; the real profile should be
checked once (§7).

| Agent setup (data, config, state; cache)                                                                                | Checkout                                  | `mise exec -- just lint`                       | Shims first on `PATH`, `just lint`          |
| ----------------------------------------------------------------------------------------------------------------------- | ----------------------------------------- | ---------------------------------------------- | ------------------------------------------- |
| all read-only; cache writable                                                                                           | trusted by the contributor (`mise trust`) | exit 0, 0.58 s                                 | exit 0, 0.14 s                              |
| all read-only, cache too                                                                                                | trusted                                   | exit 0                                         | —                                           |
| all read-only                                                                                                           | new copy, not trusted                     | exit 1: `failed to ln -sf … Permission denied` | exit 1: `Config files in … are not trusted` |
| all read-only, `MISE_TRUSTED_CONFIG_PATHS` set to the copy                                                              | new copy                                  | exit 1: same `ln -sf` error                    | —                                           |
| data and config read-only, **state writable**; global config has `trusted_config_paths = ["<parent of the checkouts>"]` | new copy under that parent                | exit 0                                         | exit 0                                      |

`mise exec --verbose` logged no HTTP request and no write when the checkout was
trusted. mise writes a link into `state/tracked-configs` the first time it meets
a checkout, so an agent working in new worktrees or clones needs the state
directory writable, and the checkout must already be trusted: by the
contributor's `mise trust`, or by a parent folder listed once in
`trusted_config_paths`, so that the agent never has to trust a configuration
itself.

The resulting additions for the agents' profile, with no extra domain:

- read: `~/.local/share/mise` (installs and shims), `~/.config/mise`;
- write: `~/.local/state/mise`;
- the cache (`~/Library/Caches/mise` on macOS) is already writable, and the run
  above shows it can also be read-only.

### 5.13 After review: mise in CI — desk review (documentation and source, not run)

From `jdx/mise-action` (v5.2.7, 2026-11-06), its `README.md` and `src/index.ts`,
read through `gh api`:

- Inputs relevant here: `version` (the mise version to install; default "newest
  release at least 24h old"), `install_args` ("additional arguments to
  `mise install`", so a job can name only the tools it needs), `cache` (on by
  default) and `github_token` (default `github.token`, for GitHub API calls).
- The action verifies the mise binary it installs, or restores from its cache,
  against mise's signed checksums: the source holds a minisign public key
  (`MISE_MINISIGN_PUBLIC_KEY`) and checks "against its own version's signed
  checksums".
- It runs `mise install` with `--locked` when the repository has a `mise.lock`
  (`docs/dev-tools/mise-lock.md:408`, §5.6e).

Compared with today's CI (§2): one step per job would replace the 20-line
`gh release download` and `gh release verify-asset` function of `ci-rust.yml`,
`taiki-e/install-action` for cargo-progenitor in `bnaclient.yml`, and the
unchecked `curl` of the hurl package in `integration.yml`; CI would check the
same lockfile checksums and release attestations as contributors (§5.3, §5.5).
Without `install_args`, a job would install every tool of `mise.toml`, and with
`cargo.binstall = false` the lint job would compile the three cargo tools (about
1.5 min, §5.6) for nothing.

## 6. Conclusion & Recommendation

**Conditional Go: adopt mise for bna-api**, with explicit `github:` backends and
a committed `mise.lock`. The first conclusion, written before the review of
§1.1, is replaced by this one; the review did not change the verdict, it changed
which conditions matter and why.

The evidence against the success bar after review (§1):

- **Contributors, outside nono**: one command, `mise install --locked`, installs
  the tools at the exact versions of `mise.toml` and `mise.lock`, in 2 to 4 s
  for the release binaries on macOS (§5.4, §5.10) and 4 to 9 s with Node.js and
  hurl on Ubuntu aarch64 and x86_64 (§5.6d); `just lint` passes each time. The
  `github:` backend verifies the GitHub release attestation of oxfmt, rumdl,
  typos and hurl, and a wrong checksum, a tampered archive or a missing checksum
  is refused with nothing installed (§5.5).
- **Agents, inside nono**: tools installed outside run `just lint` through
  `mise exec` or shims with mise's data and configuration read-only and its
  state writable, no network, as long as the checkout is trusted beforehand
  (§5.12, simulated with file permissions).
- **CI can share the files**: `jdx/mise-action` reads `mise.toml` and installs
  `--locked` when `mise.lock` exists (documented, not run, §5.6e).
- **Against the alternatives**: the plain recipe works outside nono (§5.11) but
  needs hand-coded platform names and an authenticated gh, covers release
  binaries only, has no lockfile, and took 20 s against 4 s; aqua dropped
  release-attestation checks (§5.8); nixpkgs lags the pinned versions (§5.9).

**Conditions**:

1. Name every backend explicitly as `github:<owner>/<repo>` (`node` for Node.js,
   `cargo:` for crates). The short names install typos without an attestation
   check and oxfmt as an npm package that needs Node.js (§5.10).
2. Set `cargo.binstall = false`, or sea-orm-cli is installed from a prebuilt
   archive with no check (§5.6).
3. Never pin an `asset_pattern` to one platform's file name, and review the URLs
   of every `mise.lock` change (§5.3).
4. Commit `mise.lock` for `linux-x64`, `linux-arm64`, `macos-arm64` and
   `macos-x64`; contributors and CI install with `--locked`.
5. For agents: extend `pfb-claude` with read access to `~/.local/share/mise` and
   `~/.config/mise` and write access to `~/.local/state/mise`, and have
   contributors trust their checkouts' parent folder once
   (`trusted_config_paths` in their global mise configuration), so an agent
   never trusts a configuration itself (§5.12).

`use_versions_host` is no longer a condition: outside nono both settings give
the same lockfile and nearly the same time (§5.10). An agent that installs tools
inside nono needs it off, or `mise-versions.jdx.dev` allowed (§5.1, §5.6e).

**Integrity level per tool, accepted as stated**:

| Tool                                 | Check before use                                                                           |
| ------------------------------------ | ------------------------------------------------------------------------------------------ |
| oxfmt, rumdl, typos, hurl            | Lockfile checksum and GitHub release attestation                                           |
| just                                 | Lockfile checksum only (from GitHub's API digest; release not immutable)                   |
| Node.js                              | Lockfile checksum, from the OpenPGP-signed `SHASUMS256.txt`                                |
| bacon, sea-orm-cli, cargo-progenitor | Built from crates.io with `cargo install --locked` (index checksums); no lockfile checksum |

**CI** (decided with the reviewer while specifying the feature, 2026-11-11): CI
uses mise too, so that `mise.toml` and `mise.lock` are the one source of truth
for contributors and CI (§1, constraints). `jdx/mise-action`, pinned by commit
and with a pinned mise version, installs in each job only the tools that job
needs (§5.13): oxfmt, rumdl, typos and just in the lint job of `ci-rust.yml`,
cargo-progenitor in `bnaclient.yml`, hurl in `integration.yml`. The Rust
toolchain stays on `dtolnay/rust-toolchain`. The action was not run by this
spike; the feature's first CI run is its test.

**Scope**: this verdict is for bna-api. brokenspoke's tools are of the same
kinds (§5.7), but the repository that matters most is brokenspoke-analyzer,
whose fit deserves its own spike.

## 7. Still Open

- **The real nono profile**: §5.12 simulated the agent's access with file
  permissions. The grants of condition 5 should be checked once in a real
  `pfb-claude` session, with mise installed in its default directories.
- **CI**: decided (§6, CI), but `jdx/mise-action` was not run. The feature's
  first CI run must show each job installing only its tools, `--locked`, and a
  wrong checksum failing the job; whether hurl's `libxml2` is present on
  `ubuntu-latest` is to be confirmed there.
- **Version bumps**: Dependabot and Renovate were out of scope. Whether one of
  them updates `mise.toml` and `mise.lock` decides how much of the manual bump
  work of `specs/0002-rust-tooling/design.md:398-400` remains.
- **The versions host**: whether contributors keep `mise-versions.jdx.dev` in
  their install path or rely on GitHub's API (and its rate limits).
- **How the `just` recipes reach the tools**: shell activation, shims, or
  recipes that call `mise exec` themselves. All worked (§5.4, §5.12); the choice
  affects contributors and agents differently.
- **Tools outside the test**: cargo-lambda, uv and the Rust toolchain itself;
  hurl's `libxml2` on Linux, a system package mise does not manage.
- **brokenspoke-analyzer**: a separate spike (§6, Scope).
- **gh's misleading message**: `gh release verify-asset` reports
  `no attestations found` when the bundle host is blocked (§5.2), which could
  mislead anyone debugging the current CI step in a restricted network.

## 8. Remaining Work

In priority order, to settle while specifying the feature (`pfb-spec-new` on
this folder):

1. The `mise.toml` and `mise.lock` for bna-api: which tools (the six of §5.6d
   plus the cargo tools), the settings of the conditions, the four lockfile
   platforms, and the versions-host choice.
2. How the `just` recipes and the agents reach the tools (activation, shims or
   `mise exec`), and the `pfb-claude` profile change of condition 5, checked in
   a real session.
3. The contributor documentation: mise as a prerequisite, `mise trust` or
   `trusted_config_paths`, `mise install --locked`, `libxml2` on Linux.
4. The CI migration (§6, CI): `jdx/mise-action` pinned by commit with a pinned
   mise version, one tool set per job, in `ci-rust.yml` (replacing the `env`
   block and the `gh` install step), `bnaclient.yml` (replacing
   `taiki-e/install-action`) and `integration.yml` (replacing the `curl` and
   `apt` install of hurl).
5. Updating `CLAUDE.md`, which today points at the `env` block for the versions.
