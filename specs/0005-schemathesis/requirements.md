# Requirements Document: Property-Based API Testing with Schemathesis

## Introduction

The Hurl suites check the responses the API is expected to return for chosen
requests, and `just coverage` checks that every documented status code is
observed. Nothing checks the API against inputs nobody wrote down: since
`specs/0001-hurl-api-coverage`, which left "Fixing Schemathesis" out of scope,
response shapes can drift from the specification on any request outside the
suites without a test failing.

Schemathesis generates requests from the OpenAPI specification and checks every
response against it. The repository already has two recipes for it, but they
were believed broken: the CI step was commented out after "Connection aborted.
Remote end closed connection without response". Investigating on 2026-11-13
showed that, locally, the server was not crashing: bacon, behind
`just debug-axum`, restarted it whenever a file changed under `lambdas/tests/`,
which Schemathesis' cassette and Hurl's reports both do. With the server left
alone, the pinned Schemathesis 3.39.5 passes, but it tests little: only the
`not_a_server_error` check, only GET operations (20 of 26), only valid inputs,
no chained calls, and against the down-converted `openapi-3.0.yaml`. With every
check enabled, the 20 GET operations (1,628 requests) and the 6 POST and PATCH
operations (573 requests) all pass against a database loaded with the test data.

This feature keeps the dev server up during test runs, moves to Schemathesis 4,
whose defaults run every check, invalid inputs, boundary values and chained
calls, tests every operation of the OpenAPI 3.1 specification, fixes what it
finds, and runs it on every pull request in the integration workflow. Running it
locally takes one recipe and uv.

**In scope**:

- The bacon job behind `just debug-axum` no longer restarting the server when a
  test run writes under `lambdas/tests/`.
- One recipe running a pinned Schemathesis 4 release through `uvx` against the
  local server, every operation included, with its settings in one configuration
  file and its reports in a git-ignored directory.
- Removing the Python project of `lambdas/tests` (`pyproject.toml`, `uv.lock`,
  `.python-version`), the `setup` recipe and the two current Schemathesis
  recipes.
- Fixing every defect Schemathesis 4 reports against the test data, each with a
  Hurl request reproducing it.
- A Schemathesis step in the `test` job of `.github/workflows/integration.yml`,
  replacing the commented-out one, with its reports uploaded.
- Contributor documentation: `lambdas/tests/README.md`, `lambdas/README.md`,
  `.github/CONTRIBUTING.md` and `CLAUDE.md`.

**Out of scope**:

- Running Schemathesis against staging or production: it runs against the local
  server only.
- Authentication: the local and CI servers run with authentication disabled, and
  errors produced by API Gateway stay out of scope, as in
  `specs/0001-hurl-api-coverage`.
- Installing uv: a contributor prerequisite, like Docker. uv stays out of
  `mise.toml`, as `specs/0003-tool-manager` decided.
- Capping the run time up front: the CI step runs with Schemathesis' default
  number of examples, and its duration is measured before any cap is considered.
- Replacing or reducing the Hurl suites and the coverage check.
- Load or performance testing.

## Glossary

- **Dev_Server**: the API server started by `just debug-axum`, which runs the
  `axum-debug` job of `bacon.toml` and restarts the server when a watched file
  changes.
- **Test_Data**: the database state `just db-seed` loads from
  `lambdas/tests/database-with-test-data.sql`, the same the integration workflow
  loads.
- **Local_Server**: the API in standalone mode on `http://localhost:3000` with
  authentication disabled, as the Dev_Server or as the release build the
  integration workflow starts.
- **Schemathesis_Recipe**: the `lambdas/tests/justfile` recipe that runs
  Schemathesis against the Local_Server.
- **Pinned_Release**: one exact Schemathesis 4 version, with the upload date
  after which no dependency is considered when resolving it, both recorded once
  in `lambdas/tests/justfile`.
- **Schemathesis_Config**: the Schemathesis configuration file of
  `lambdas/tests`.
- **Schemathesis_Reports**: the JUnit and HTML reports a run writes.
- **Finding**: a failure Schemathesis reports, identified by its operation and
  its check.
- **Excluded_Finding**: a Finding judged a false positive, excluded from the
  runs in the Schemathesis_Config.

## Requirements

### Requirement 1: A Dev Server That Survives Test Runs

**User Story:** As a contributor, I want the server started by `just debug-axum`
to keep running while I run the test suites, so that the tests reach it instead
of failing on a server that is restarting.

#### Acceptance Criteria

1. WHILE the Dev_Server runs, WHEN a file or directory under `lambdas/tests/` is
   created, modified or deleted, the Dev_Server SHALL keep the same server
   process, which keeps answering `GET /health` with `200`.
2. WHILE the Dev_Server runs, WHEN a file under `lambdas/src/` or `entity/src/`
   changes, the Dev_Server SHALL restart the server, as it does today.
3. WHILE the Dev_Server runs on the Test_Data, WHEN a contributor runs
   `just test-smoke-public localhost`, `just test localhost` and the
   Schemathesis_Recipe one after the other, each run SHALL reach the server.

### Requirement 2: A Pinned Schemathesis Without a Python Project

**User Story:** As a contributor, I want to run Schemathesis with one recipe and
only uv installed, so that I need no Python environment, and every run uses the
same Schemathesis.

#### Acceptance Criteria

1. The Schemathesis_Recipe SHALL run the Pinned_Release through `uvx`.
2. The Pinned_Release SHALL be recorded in one place, `lambdas/tests/justfile`,
   and the integration workflow SHALL run the Schemathesis_Recipe instead of
   naming a version of its own.
3. WHEN two runs resolve the same Pinned_Release, they SHALL use the same
   versions of Schemathesis and of its dependencies.
4. WHEN a contributor with uv on the `PATH` runs the Schemathesis_Recipe, it
   SHALL run without any other Python setup.
5. The repository SHALL no longer contain `lambdas/tests/pyproject.toml`,
   `lambdas/tests/uv.lock` and `lambdas/tests/.python-version`, nor the `setup`,
   `test-localhost-schemathesis` and `test-staging-schemathesis` recipes.

### Requirement 3: A Thorough Test Configuration

**User Story:** As a maintainer, I want Schemathesis to test every operation
with every check, valid and invalid inputs, boundary values and chained calls,
so that a response that drifts from the specification fails a test.

#### Acceptance Criteria

1. The Schemathesis_Recipe SHALL test every operation of `openapi-3.1.yaml`,
   POST and PATCH included, against the Local_Server.
2. The Schemathesis_Recipe SHALL run every check Schemathesis provides, with
   valid and invalid inputs, in the examples, coverage, fuzzing and stateful
   phases.
3. The settings of the run SHALL live in the Schemathesis_Config, which the
   Schemathesis_Recipe and the integration workflow both use.
4. WHEN a check fails, the run SHALL go on testing the other operations, report
   every Finding, and exit with a non-zero status.
5. The run SHALL write the Schemathesis_Reports in a git-ignored directory under
   `lambdas/tests/`, and leave `git status` unchanged.
6. WHERE an Excluded_Finding exists, the Schemathesis_Config SHALL name its
   operation and its check, with a comment giving the reason it is a false
   positive.
7. IF the Local_Server is not reachable, THEN the Schemathesis_Recipe SHALL fail
   with a non-zero status.

### Requirement 4: An API That Passes

**User Story:** As a maintainer, I want every real defect Schemathesis finds
fixed, so that the CI step starts green and any new failure points at a
regression.

#### Acceptance Criteria

1. WHEN the Schemathesis_Recipe runs against the Local_Server on the Test_Data,
   every check SHALL pass, apart from the Excluded_Findings.
2. WHEN a Finding is a defect of the API or of its specification, the fix SHALL
   come with a Hurl request in `lambdas/tests/` that fails without the fix and
   passes with it.
3. WHEN a fix changes the specification, `openapi-3.1.yaml`, `openapi-3.0.yaml`
   and `bnaclient` SHALL be regenerated with `just generate`.
4. WHEN the Findings are fixed, `just test localhost` and `just coverage` SHALL
   still pass.

### Requirement 5: Schemathesis on Every Pull Request

**User Story:** As a maintainer, I want Schemathesis to run on every pull
request, so that a change that breaks the API's contract cannot be merged
unnoticed.

#### Acceptance Criteria

1. WHEN the integration workflow runs, its `test` job SHALL run the
   Schemathesis_Recipe against the release server it starts, after the Hurl
   suites and the coverage check.
2. IF the Schemathesis_Recipe fails, THEN the `test` job SHALL fail.
3. WHEN an earlier step of the `test` job fails, the Schemathesis_Recipe SHALL
   still run, unless the run was cancelled.
4. WHEN the Schemathesis_Recipe has run, the Schemathesis_Reports SHALL be
   uploaded as an artifact of the run, whether it passed or failed.
5. The integration workflow SHALL no longer contain the commented-out
   Schemathesis step.

### Requirement 6: Documented for Contributors

**User Story:** As a contributor, I want to find how to run Schemathesis and
what it checks, so that I can reproduce a CI failure locally.

#### Acceptance Criteria

1. `lambdas/tests/README.md` SHALL describe the prerequisites (uv, the
   Local_Server on the Test_Data), the Schemathesis_Recipe, what it checks,
   where the Schemathesis_Reports are written, and that it runs in the
   integration workflow.
2. `lambdas/tests/README.md` SHALL state that a Schemathesis run writes random
   rows, so the database must be reloaded with `just db-seed` before running the
   Hurl suites again.
3. `lambdas/README.md` SHALL list the Schemathesis_Recipe with the other test
   suites, and uv among the requirements.
4. `.github/CONTRIBUTING.md` and `CLAUDE.md` SHALL name uv as the one
   prerequisite of the Schemathesis_Recipe, and `CLAUDE.md` SHALL list the
   recipe with the other test commands.
