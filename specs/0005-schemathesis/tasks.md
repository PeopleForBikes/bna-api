# Implementation Plan: Property-Based API Testing with Schemathesis

## Overview

The work goes from the local setup outwards, one commit per task or sub-task
(signed off, as the team does), so each step can be checked on its own:

1. Commit the bacon change that keeps the Dev_Server up during test runs.
2. Replace the Python project of `lambdas/tests` with the `uvx` recipe and the
   Schemathesis_Config, and check the run itself: reproducible, thorough,
   leaving no files behind.
3. Triage what Schemathesis 4 finds, one commit per Finding, until the run
   passes on the Test_Data.
4. Add the step to the integration workflow, once the run is green, so the pull
   request does not start red.
5. Document the recipe for contributors.
6. Check everything locally, then on the pull request's CI.

Prerequisites on the implementer's machine: uv, Docker with the Compose database
(`just compose-up`), and network access to `pypi.org` and
`files.pythonhosted.org`. In the nono sandbox, the session needs the grants of
`design.md` (Deployment Considerations): the two PyPI domains and write access
to `~/.local/share/uv/tools`. The checks that write through the API run on the
local database after `just db-seed`, which replaces its content. The commands
and their results go in `findings.md`.

## Tasks

- [ ] 1. Keep the Dev_Server up during test runs
  - [ ] 1.1 Commit `ignore = ["lambdas/tests"]` in the `axum-debug` job of
        `bacon.toml` (in the working tree since 2026-11-13).
    - _Requirements: 1.1, 1.2_
  - [ ] 1.2 With `just debug-axum` running on the Test_Data, probe with a script
        in the scratch directory (not committed) that polls `GET /health` and
        the `axumed` PID every 100 ms for 8 s:
    - create and delete a file in `lambdas/tests/`, then delete and recreate
      `lambdas/tests/reports/`: the PID never changes and `/health` stays `200`;
    - touch a file in `lambdas/src/`: the PID changes and `/health` returns
      `200` again.
    - _Requirements: 1.1, 1.2_

- [ ] 2. Run a pinned Schemathesis 4 through `uvx`
  - [ ] 2.1 Check the latest Schemathesis 4.x release on PyPI. Keep 4.33.4 and
        `2026-11-13`, or take the newer release with the day after its
        publication.
    - _Requirements: 2.1_
  - [ ] 2.2 In `lambdas/tests/justfile`, add the `schemathesis` and
        `schemathesis_exclude_newer` variables and the `test-schemathesis`
        recipe of `design.md`; remove the `setup`, `test-localhost-schemathesis`
        and `test-staging-schemathesis` recipes. Delete
        `lambdas/tests/pyproject.toml`, `lambdas/tests/uv.lock` and
        `lambdas/tests/.python-version`. Add `lambdas/tests/schemathesis.toml`
        with the content of `design.md`, and the two `.gitignore` entries. In
        `CLAUDE.md`, replace `just test-localhost-schemathesis` with
        `just test-schemathesis` in the integration test commands. Run
        `just fmt-just`.
    - Verify: `just --list` in `lambdas/tests` shows `test-schemathesis` and
      none of the three removed recipes; `git ls-files lambdas/tests` lists none
      of the three deleted files; `grep -rn 4.33.4` (or the chosen version)
      outside `specs/` finds only `lambdas/tests/justfile`.
    - _Requirements: 2.1, 2.2, 2.4, 2.5, 3.1, 3.2, 3.3, 3.5, 3.6, 6.4_
  - [ ] 2.3 Reproducible resolution: resolve the Pinned_Release twice, each with
        `UV_CACHE_DIR` and `UV_TOOL_DIR` in a new scratch directory, with the
        recipe's `uvx` options and `--from` the release, running `python -c`
        that prints every installed distribution and its version
        (`importlib.metadata.distributions()`), sorted. The two lists are
        identical.
    - _Requirements: 2.3_
  - [ ] 2.4 First run: with the Dev_Server on a fresh `just db-seed`, run
        `just test-schemathesis`.
    - The output lists 26 selected operations, the examples, coverage, fuzzing
      and stateful phases, and every check of `design.md` (Schemathesis 4's list
      of 14).
    - `schemathesis-report/` holds a JUnit file and an HTML report; `git status`
      is unchanged by the run.
    - Record every Finding (operation, check, a reproducing cURL command) and
      the stateful phase's link count (inferred, covered) in `findings.md`.
    - _Requirements: 3.1, 3.2, 3.4, 3.5_
  - [ ] 2.5 Negative checks, none committed:
    - stop the Dev_Server and run the recipe: it exits `1` with a Network Error
      per operation;
    - with `PATH` lacking uv, the recipe fails (`uvx: command not found`);
    - change a `utoipa` annotation so that a documented response field no longer
      matches what the handler returns, run `just generate-oas-31`, and run the
      recipe: it fails on `response_schema_conformance` for that operation.
      Revert with `git restore`.
    - _Requirements: 3.4, 3.7_

- [ ] 3. Triage the Findings until the run passes
  - [ ] 3.1 Classify each Finding of 2.4 as a defect of the API, a defect of the
        specification, or a false positive, with the reason, in `findings.md`.
        Group the Findings that share one cause.
    - _Requirements: 4.1_
  - [ ] 3.2 One commit per cause, following `design.md` (Integration):
    - API defect: the fix in `lambdas/src/core/resource/**`, a Hurl request in
      `lambdas/tests/endpoints/` or `lambdas/tests/scenario/` that fails without
      the fix and passes with it, and a `CHANGELOG.md` entry under
      `[Unreleased]` when a client can see the change;
    - specification defect: the `utoipa` annotation fix, `just generate`, and
      the Hurl request;
    - false positive: an `[[operations]]` block in
      `lambdas/tests/schemathesis.toml` naming the operation and the check, with
      the reason in a comment.
    - After each commit: `just db-seed`, `just test localhost`, `just coverage`,
      then `just db-seed` and `just test-schemathesis` show the Finding gone and
      no new one.
    - _Requirements: 3.6, 4.1, 4.2, 4.3, 4.4_

- [ ] 4. Checkpoint: the local run is green
  - On a fresh `just db-seed` with the Dev_Server:
    `just test-smoke-public localhost`, `just test localhost` and
    `just coverage` pass, then `just test-schemathesis` passes, all against the
    same running server.
  - `just ci` passes at the root, and `git status` is clean.
  - _Requirements: 1.3, 4.1, 4.4_

- [ ] 5. Run Schemathesis on every pull request
  - [ ] 5.1 In `.github/workflows/integration.yml`, in the `test` job: add the
        pinned `astral-sh/setup-uv` step with uv's version, the
        `Run Schemathesis` step and the `Upload the Schemathesis reports` step
        of `design.md` after the coverage summary, and remove the commented-out
        Schemathesis step.
    - Verify the workflow parses (a YAML parser in the scratch directory) and
      that the new steps come after the Hurl suites and the coverage check, each
      with `if: ${{ !cancelled() }}`.
    - _Requirements: 2.2, 5.1, 5.2, 5.3, 5.4, 5.5_

- [ ] 6. Document the recipe
  - [ ] 6.1 `lambdas/tests/README.md`: a "Schemathesis" section with the
        prerequisites, the recipe, what it checks, the reports and crash files,
        `st replay` through the recipe's `uvx` command, the CI step, and the
        `just db-seed` needed before the Hurl suites after a run. Mention the
        nono grants for agents.
    - _Requirements: 6.1, 6.2_
  - [ ] 6.2 `lambdas/README.md`: uv among the requirements (installed
        separately, like Docker), and `just test-schemathesis` with the Hurl
        suites in "Terminal 2". `.github/CONTRIBUTING.md`: uv as the
        prerequisite of Schemathesis, outside the tools mise installs.
        `CLAUDE.md`: uv as the prerequisite of `just test-schemathesis`.
    - _Requirements: 6.3, 6.4_
  - [ ] 6.3 `just lint` passes.
    - _Requirements: 6.1, 6.2, 6.3, 6.4_

- [ ] 7. Checkpoint: the pull request
  - The pull request's integration run passes, with the Schemathesis step and
    the `schemathesis-reports` artifact (JUnit and HTML). Record the step's
    duration in `findings.md`.
  - The `ci-rust`, `lambdas`, `migration` and `bnaclient` workflows pass where
    they run.
  - _Requirements: 5.1, 5.2, 5.4_

## Notes

- The number of commits in task 3 depends on what Schemathesis 4 finds; none
  could be observed before implementation. A Finding whose fix turns out larger
  than this feature (a new endpoint behavior, a migration) is recorded in
  `findings.md` and raised with the team before going further; it is not
  excluded silently.
- A Schemathesis run writes random rows. Always `just db-seed` before the Hurl
  suites, and before a Schemathesis run whose result is compared with an earlier
  one.
- If the stateful phase infers no links, record it in `findings.md`; adding
  OpenAPI links to the `utoipa` annotations is a follow-up (`design.md`, Open
  Questions).
- `lambdas/tests/.venv`, left by the old `setup` recipe, stays git-ignored;
  contributors can delete it.
- `lambdas/tests/cassette.yaml`, written by an earlier run of the old recipe, is
  untracked and not ignored: delete it before committing.
