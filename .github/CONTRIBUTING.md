# Contributing

## General guidelines

This project follows the [BNA Mechanics Contributing
Guidelines][contributing guidelines]. Refer to them for general principles.

[contributing guidelines]: https://peopleforbikes.github.io/contributing/

## Local tools

`just fmt`, `just lint`, the API tests and the code generators call command-line
tools that [mise] installs at the versions pinned in `mise.toml`, from the
download URLs and checksums recorded in `mise.lock`. CI installs its tools from
the same two files.

### Prerequisites

- [mise][mise installation], version 2026.11.5 or newer.
- A Rust toolchain: mise builds bacon, sea-orm-cli and cargo-progenitor from
  crates.io.
- On Linux, the `libxml2` library, which hurl needs.
- A GitHub login (`gh auth login`) or a `GITHUB_TOKEN` environment variable, to
  avoid GitHub's rate limit on anonymous requests.
- [uv], for the Schemathesis tests only (`just test-schemathesis` in
  `lambdas/tests`): mise does not install it. The recipe runs a pinned
  Schemathesis release through `uvx`.

### Setup

1. Trust the checkout's `mise.toml`, once per checkout:

   ```bash
   mise trust
   ```

   Or trust every checkout under a folder, in `~/.config/mise/config.toml`:

   ```toml
   [settings]
   trusted_config_paths = ["/path/to/your/projects"]
   ```

2. Install the tools:

   ```bash
   mise install
   ```

3. Put the tools on your `PATH`: either activate mise in your shell (for example
   `eval "$(mise activate zsh)"` in `~/.zshrc`), or add mise's shims directory
   (`~/.local/share/mise/shims` by default) to your `PATH`. The
   `just nono-claude-danger` recipe does the latter for the agent, and lets the
   sandbox read the tools installed outside it.

Run `mise install` again after pulling a change to `mise.toml`. mise never
installs a tool behind a command's back: a tool that is not installed falls back
to any other copy on your `PATH`, and `mise exec` only warns about it.

### What it protects against

- Every downloaded archive must match the checksum `mise.lock` records for your
  platform; the install fails on a mismatch, or when the platform or the
  checksum is missing.
- oxfmt, rumdl, typos and hurl must also pass the check of their GitHub release
  attestation, at every install. just publishes no attestation and is checked by
  its checksum only.
- bacon, sea-orm-cli and cargo-progenitor are built from crates.io with
  `cargo install --locked`, never installed from prebuilt binaries.

### Bumping a tool

1. Change its version in `mise.toml`.
2. Regenerate the lockfile for the four supported platforms:

   ```bash
   mise lock --platform linux-x64,linux-arm64,macos-arm64,macos-x64
   ```

3. Review the diff of `mise.lock`: every `url` names the platform of its entry
   (`linux` with `x86_64` or `aarch64`, or `apple-darwin`), and oxfmt, rumdl,
   typos and hurl keep `provenance = "github-attestations"`.
4. Run `mise install`, then `just lint`.

CI uses the version of mise set by the `version` input of the `jdx/mise-action`
steps in `.github/workflows/`.

[mise]: https://mise.jdx.dev/
[mise installation]: https://mise.jdx.dev/getting-started.html
[uv]: https://docs.astral.sh/uv/
