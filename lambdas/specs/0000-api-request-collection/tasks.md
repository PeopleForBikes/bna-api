# Implementation Plan: API Request Collection with Slumber

## Overview

Four changes, each a commit of its own, then a final checkpoint. The tool comes
first, so that the collection can be loaded with the pinned version; the
collection comes before the retirement of `lambdas/requests.rest`, so the
repository always has one; the README comes last, because its example is checked
against the collection. There is no Rust code: the checks are the commands of
the Testing Strategy in `design.md`, and `just ci` at each checkpoint.

## Tasks

- [x] 1. Pin Slumber with the other cargo tools
  - [x] 1.1 Add `"cargo:slumber" = "5.4.4"` to `[tools]` in `mise.toml`, after
        `"cargo:cargo-progenitor"`.
    - _Requirements: 1.1_
  - [x] 1.2 Regenerate `mise.lock` with
        `mise lock --platform linux-x64,linux-arm64,macos-arm64,macos-x64`, and
        review its diff: only a new `[[tools."cargo:slumber"]]` entry
        (`version = "5.4.4"`, `backend = "cargo:slumber"`,
        `specifiers = ["5.4.4"]`), shaped like `cargo:bacon`. If the sandbox
        blocks the command (crates.io's API is not reachable from it), ask the
        user to run it.
    - _Requirements: 1.1_
  - [x] 1.3 In `.github/CONTRIBUTING.md`, add slumber to the tools mise builds
        from crates.io, in "Prerequisites" and in "What it protects against".
    - _Requirements: 4.5_
  - [x] 1.4 Ask the user to run `mise install` outside the sandbox, then check
        that `slumber --version` prints `slumber 5.4.4`.
    - _Requirements: 1.2, 1.3_
  - [x] 1.5 Check that `grep -n slumber .github/workflows/*.yml` finds nothing.
    - _Requirements: 1.4_

- [x] 2. Add the collection `lambdas/slumber.yml`
  - [x] 2.1 Create `lambdas/slumber.yml` with the content given in `design.md`
        (Components and Interfaces): the schema comment pinned to `v5.4.4` and
        the note to keep it equal to `mise.toml`, `name: BNA API`, the profile
        `local` (`name: Local development API`, `default: true`,
        `host: http://localhost:3000`,
        `token: "{{ env('HURL_VARIABLE_token') }}"`), and `requests: {}`.
    - _Requirements: 2.1, 2.3, 2.4, 2.7, 2.8_
  - [x] 2.2 Check loading and discovery:
        `slumber --file lambdas/slumber.yml collection` exits 0 and shows one
        profile `local`, default, with the local host, and `requests: {}`;
        `slumber collection` run from `lambdas/tests/endpoints` prints the same
        collection. Inside the sandbox, set `HOME` to a scratch folder for these
        runs.
    - _Requirements: 2.2, 2.3, 2.4, 2.9_
  - [x] 2.3 Check the token with a scratch copy of the collection holding one
        temporary recipe that sends `Authorization: Bearer {{ token }}` (never
        committed): `slumber request <recipe> --dry-run` prints
        `authorization: Bearer` with no value when `HURL_VARIABLE_token` is
        unset, and `Bearer abc123` with `HURL_VARIABLE_token=abc123`.
    - _Requirements: 2.5, 2.6_
  - [x] 2.4 Check the schema: the tag in the URL equals the version in
        `mise.toml`, and
        `gh api repos/LucasPickering/slumber/contents/schemas/collection.json?ref=v5.4.4`
        answers.
    - _Requirements: 2.8_

- [x] 3. Checkpoint - tool and collection
  - Run `just ci`; it passes (typos checks `lambdas/slumber.yml`).
  - Review `lambdas/slumber.yml`: no literal token, password or key.
  - _Requirements: 2.7_

- [x] 4. Retire `lambdas/requests.rest`
  - [x] 4.1 Delete `lambdas/requests.rest`.
    - _Requirements: 3.1_
  - [x] 4.2 In `.github/workflows/lambdas.yml`, replace
        `- "!lambdas/requests.rest"` with `- "!lambdas/slumber.yml"`, keeping it
        after `- "lambdas/**"`.
    - _Requirements: 3.3, 3.4_
  - [x] 4.3 Check that `test ! -e lambdas/requests.rest` succeeds and
        `git grep -n 'requests\.rest' -- ':!*specs/*'` finds nothing.
    - _Requirements: 3.1, 3.2_

- [x] 5. Document Slumber in `lambdas/README.md`
  - [x] 5.1 Add `[Slumber]` to "Requirements" after `[Sea-ORM-CLI]`, its link
        definition `[slumber]: https://slumber.lucaspickering.me/` with the
        others, and Slumber to the tools of the "Remark" pinned in `mise.toml`.
    - _Requirements: 4.1_
  - [x] 5.2 In "Terminal 2", after the cURL example, explain exploring with
        Slumber: start it from `lambdas/` or a subfolder (otherwise
        `--file lambdas/slumber.yml`), the TUI (`slumber`) and one request
        (`slumber request <recipe>`), the `local` profile, and the history kept
        on the contributor's machine.
    - _Requirements: 4.2_
  - [x] 5.3 In the same place, explain the token: authentication is disabled
        locally, so it is empty unless `HURL_VARIABLE_token` is set, the
        variable of the Hurl suites (`tests/README.md`), e.g.
        `HURL_VARIABLE_token=<token> slumber`.
    - _Requirements: 4.3_
  - [x] 5.4 Add the subsection "Adding a request to the collection" with the
        conventions of `design.md`: one folder per OpenAPI tag, recipe ids from
        the `operationId` with a suffix for variants and unique across the file,
        chaining with
        `{{ response('<recipe>', trigger='no_history') | jsonpath('<path>') }}`,
        `Authorization: Bearer {{ token }}` on write requests, JSON bodies with
        the types of the specification, `slumber import openapi` as a starting
        point only; and one example: a folder with a list recipe and a recipe
        chained on it.
    - _Requirements: 4.4_
  - [x] 5.5 Check the example: paste it into a scratch copy of
        `lambdas/slumber.yml` (never committed); `slumber collection` loads it,
        and, with the local API running (`just compose-up`, `just db-seed`,
        `just debug-axum`, started by the user), `slumber request` on the
        chained recipe sends the list recipe first and answers `200`.
    - _Requirements: 4.4_

- [x] 6. Checkpoint - final
  - Run `just ci`; it passes, including the Markdown checks of `just lint` on
    `lambdas/README.md` and `.github/CONTRIBUTING.md`.
  - Review `lambdas/README.md` and `.github/CONTRIBUTING.md` against
    Requirement 4.
  - Check the correctness properties of `design.md`: one Slumber version in
    `mise.toml`, `mise.lock` and the schema URL; no secret in the collection;
    one default profile, `local`; no `requests.rest` and no mention of it
    outside `specs/`; no workflow installs Slumber.
  - _Requirements: 1.1, 1.4, 2.7, 3.2, 4.1, 4.2, 4.3, 4.4, 4.5, 4.6_

- [x] 7. Add the first recipe, `get_cities` (requested after task 6)
  - [x] 7.1 In `lambdas/slumber.yml`, replace `requests: {}` with the folder
        `city` holding `get_cities`, as given in `design.md`: the first recipe
        of the README example, unchanged.
    - _Requirements: 2.4_
  - [x] 7.2 Check: `slumber --file lambdas/slumber.yml collection` exits 0 and
        shows one recipe, `city/get_cities`; with the local API running,
        `slumber request get_cities -v` sends
        `GET http://localhost:3000/cities?page_size=5` and receives `200` with
        at most five cities.
    - _Requirements: 2.4, 2.9, 2.10_
  - [x] 7.3 In `lambdas/README.md` ("Terminal 2"), show
        `slumber request get_cities` instead of `slumber request <recipe>`, so
        the example works as is.
    - _Requirements: 4.2_

- [x] 8. Checkpoint - first recipe
  - Run `just ci`; it passes.
  - Check the correctness properties of `design.md` again (no secret, one
    default profile `local`, one version).
  - _Requirements: 2.7, 2.9_

- [x] 9. Wait for PostgreSQL in `just compose-up` (added after the review)
  - [x] 9.1 In `docker-compose.yml`, give the `postgres` service a `healthcheck`
        running `pg_isready -d postgres -U postgres`, as given in `design.md`.
    - _Requirements: 5.1_
  - [x] 9.2 In `justfile`, make `compose-up` run `docker compose up -d --wait`.
    - _Requirements: 5.2_
  - [x] 9.3 Ask the user to run `just compose-down`, `just compose-up` and
        `docker compose ps` (the `postgres` service is `(healthy)` when the
        recipe returns); then `just compose-down` and, chained with `&&`,
        `just compose-up` and `just db-seed` (exit 0).
    - Run by the user as `just compose db-seed` (2026-11-14), which chains
      `compose-down`, `compose-up` and `db-seed`: `docker compose up -d --wait`
      printed `Container bna-api-postgres-1 Healthy 5.2s` before returning, and
      `db-seed` connected at once (`No pending migrations`).
    - _Requirements: 5.2, 5.3_
  - [x] 9.4 Ask the user to check the failure case: with a scratch override file
        setting the health check to `["CMD", "false"]`,
        `docker compose -f docker-compose.yml -f <override> up -d --wait` exits
        non-zero once the service is `unhealthy` (about 60 s;
        `--wait-timeout 180` guards against a hang); then restore with
        `just compose`.
    - Run by the user (2026-11-14): `up -d --wait --wait-timeout 180` printed
      `container bna-api-postgres-1 is unhealthy` and exited 1 after 60.9 s;
      `just compose db-seed` then restored the database (`Healthy 5.2s`). The
      task first said "about 70 s", one interval too many (`findings.md` §1.3).
    - _Requirements: 5.4_

- [x] 10. Checkpoint - database wait
  - Run `just ci`; it passes.
  - _Requirements: 5.1, 5.2_

## Notes

- **Commits**: one per task 1, 2, 4, 5 and 9, each with its checks; task 7 goes
  with the collection and the README.
- **Sandbox**: `mise install` runs outside the nono sandbox, as for every mise
  install; Docker and the local API too. Inside the sandbox, run Slumber with
  `HOME` set to a scratch folder, so that it does not need its history folder,
  and call the installed binary (`"$(mise which slumber)"`) rather than the mise
  shim, which stops trusting `mise.toml` when `HOME` changes (`findings.md`
  §2.2).
- **Requirement 3.3** (a pull request changing only `lambdas/slumber.yml` does
  not run the `lambdas` workflow) cannot be observed on this feature's pull
  request, which changes other files under `lambdas/`; check it on the first
  pull request that only adds a recipe, and record the result in `findings.md`.
- **Conventions**: agreed as a starting point; if the team reverts them, only
  the README subsection and the recipes change.
