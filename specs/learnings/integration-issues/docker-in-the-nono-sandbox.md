---
title: Docker in the nono sandbox needs a socket grant, own config, mise grants
date: 2026-11-12
updated: 2026-11-12
category: integration-issues
status: active
tags: [docker, nono, sandbox, agents, compose]
modules: [justfile, docker-compose.yml]
spec: specs/0004-local-connection-pooler-yanked
related:
  [
    specs/0004-local-connection-pooler-yanked/spike.md,
    specs/learnings/integration-issues/mise-shims-need-the-mise-binary-in-a-sandbox.md,
  ]
superseded_by:
---

# Docker in the nono sandbox needs a socket grant, own config, mise grants

## Context

The spike `specs/0004-local-connection-pooler-yanked` had an agent run a Docker
Compose stack (PostgreSQL and three poolers) from a session started in the
`pfb-claude` nono profile (nono 0.81.0, Docker Desktop 29.9.3, Compose 5.7.0, on
macOS). Three separate problems stood in the way, one after the other.

## Symptom

1. With the daemon running, every `docker` command failed:

   ```text
   permission denied while trying to connect to the docker API at unix:///var/run/docker.sock
   ```

   (With the daemon stopped, the error is
   `dial unix /var/run/docker.sock: connect: no such file or directory` instead:
   starting the daemon does not fix the first one.)
   `nono why --self --path ~/.docker/run/docker.sock --op readwrite` reported
   `filesystem_deny: Path is covered by filesystem.deny rule: ~/.docker`.

2. Once the socket was granted, `docker compose up` started the services whose
   images were cached and failed to pull the others:

   ```text
   [docker-credential-desktop.paths][F] creating <HOME>/Library/Containers/com.docker.docker/Data: mkdir <HOME>/Library/Containers: file exists
   error getting credentials - err: exit status 1, out: ``
   ```

   The stack came up half-started, and the tests ran against services that were
   not there.

3. In the restarted session, `rumdl`, `oxfmt` and `typos` were missing
   (`command not found`), and `mise exec` failed with `Operation not permitted`
   on `ln -sf` into `~/.local/state/mise/trusted-configs/`.

## What Did Not Work

- **Granting the socket alone** (`--allow-unix-socket`): the profile's deny rule
  on `~/.docker` wins over a grant.
- **An empty `DOCKER_CONFIG`** (`{}`) to skip the credential helper: pulls are
  fine, but the CLI no longer finds its plugins, so `docker compose` is not a
  command (`unknown shorthand flag: 'f' in -f`). Docker Desktop installs them
  under `~/.docker/cli-plugins`, which stays denied.
- **A bare `nono run --profile pfb-claude ...`** with the Docker flags: it drops
  the mise grants that the `nono-claude` recipe adds, so the tools installed by
  mise disappear.

## Root Cause

- The `pfb-claude` profile denies `~/.docker`, which holds both the daemon's
  socket (`/var/run/docker.sock` is a symbolic link to
  `~/.docker/run/docker.sock`) and `config.json`, where registry credentials can
  live. nono's `--bypass-protection` overrides a deny rule only when paired with
  a grant (`nono run --help`).
- Docker Desktop sets `"credsStore": "desktop"` in `~/.docker/config.json`, so
  every pull runs `docker-credential-desktop`, which needs
  `~/Library/Containers/com.docker.docker`, outside the sandbox.
- The mise grants are not in the profile but in the recipe: `justfile:200`
  (`nono-claude`) adds `--read` on mise's data and configuration directories,
  `--allow` on its state directory (defined at `justfile:14` to `justfile:16`)
  and `--read-file` on the mise executable (`justfile:208`).

## Solution

1. Start the session from the `nono-claude` recipe's command line, plus the two
   Docker flags (and any `--allow-domain` / `--open-port` the work needs). The
   recipe takes no extra arguments, so copy its command:

   ```sh
   mise_data=~/.local/share/mise mise_config=~/.config/mise mise_state=~/.local/state/mise
   mise_bin="$(command -v mise)"
   PATH="$mise_data/shims:$PATH" nono run --allow-cwd --profile pfb-claude \
     --read "$mise_data" --read "$mise_config" --allow "$mise_state" \
     --read-file "$mise_bin" --read-file "$(realpath "$mise_bin")" \
     --bypass-protection ~/.docker \
     --allow-unix-socket ~/.docker/run/docker.sock \
     -- claude
   ```

   The two Docker flags were verified (`docker version` reaches the daemon);
   this combined command line was not run as such: the spike's session used them
   without the mise grants.

2. Give Docker a configuration of its own, without a credential store, that
   still finds Docker Desktop's plugins:

   ```sh
   mkdir -p "$SCRATCH/docker-config"
   echo '{"cliPluginsExtraDirs": ["/Applications/Docker.app/Contents/Resources/cli-plugins"]}' \
     > "$SCRATCH/docker-config/config.json"
   export DOCKER_CONFIG="$SCRATCH/docker-config"
   ```

   With it, `docker compose version` and pulls of public images work, and the
   agent never reads the registry credentials. Images from private registries
   (such as Amazon ECR) cannot be pulled this way.

Access to the Docker socket gives the agent whatever Docker Desktop shares
(normally all of `/Users`, through a bind mount), which is what the deny rule
protects. Grant it for one session, not in the profile.

## Prevention

- Before testing against a Compose stack, check that every service runs, for
  example `docker compose ps --status running --services | wc -l` against the
  expected count, and stop otherwise: a failed pull leaves a half-started stack
  that looks like a test failure.
- When proposing extra sandbox flags, start from the `nono-claude` recipe
  (`justfile:200`), never from a bare `nono run`. A recipe parameter for extra
  flags would make this one command; it does not exist yet.
