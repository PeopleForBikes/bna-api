# Requirements: Hurl API Coverage

Feature #0001: hurl-api-coverage

## Summary

A Hurl test suite that exercises the behavior of every operation and every
documented status code in the OpenAPI specification. It runs locally and in CI
on every pull request, provides the evidence for release approval, and fails the
build when an operation or status code has no test. Any disagreement between the
specification and the implementation is treated as a bug and reconciled.

## Problem Frame

The Hurl suite in `lambdas/tests` covers only part of the API and some of its
tests are not valid:

- `scenario/bna-pipeline.hurl` is not run by `just test` nor by the
  `integration` workflow. It also sends `sqs_message` as a JSON object while the
  API and the specification expect a string, and it marks a pipeline complete
  with a `start_time` later than its `end_time`.
- `scenario/pagination.hurl` asserts that the first page is page `0`, while the
  specification declares `page` with a minimum of `1`. Two of its comments
  describe the opposite of what is asserted.
- Several requests only check the status code, have empty assertion sections, or
  capture values that are never used.
- `smoke/public.hurl` creates a submission, so the smoke test writes data.
- Never tested: `POST /cities`, `PATCH /cities/submissions/{submission_id}`,
  `GET /pipelines/bna`, `GET /pipelines/bna/{pipeline_id}`, the `sort_by`,
  `order_direction` and `latest` query parameters, the submission `status`
  filter, and most documented error responses.

The specification also lists the same four error responses (400, 401, 403, 404)
on most operations, whether or not the operation can produce them, and omits
some that the API does return (for instance 400 on an invalid `page` value for
`GET /ratings`).

## Glossary

- **Operation**: One HTTP method on one path of `openapi-3.1.yaml`, identified
  by its `operationId`.
- **Coverage pair**: An operation combined with one of its documented response
  status codes.
- **Coverage check**: An automated comparison of the coverage pairs in the
  specification against the pairs exercised by the Hurl suite.
- **Mismatch**: Any difference between what the specification documents and what
  the API does.
- **Release approval**: The manual decision, by a senior engineer or manager,
  that a version is ready for production.
- **Run-unique data**: Identifiers and values generated per test run so repeated
  runs never collide.
- **Smoke test**: The short, read-only suite (`smoke/public.hurl`) checking that
  a deployed environment is alive and serving data.
- **Ephemeral local API**: The API built from source and started in CI against a
  database seeded from `database-with-test-data.sql`, with auth disabled.

## User Stories

### US-1: Trust the existing tests

As a developer, I want every existing Hurl test to assert what it claims, so
that a passing suite means the API behaves correctly.

Acceptance criteria:

- Every request has assertions matching its comment; no comment contradicts its
  assertions.
- No empty assertion sections and no unused captures remain.
- Every Hurl file in the suite is executed by `just test` and by CI.
- Request bodies conform to the specification's request schemas.

### US-2: Cover the whole specification

As a maintainer, I want every coverage pair exercised by at least one Hurl
request, so that no documented behavior ships untested.

Acceptance criteria:

- Every operation in `openapi-3.1.yaml` has at least one passing request.
- Every documented status code of every operation has at least one request
  producing it.
- Query parameters are tested for their effect: pagination (`page`,
  `page_size`), sorting (`sort_by`, `order_direction`), `latest`, and the
  submission `status` filter.
- Write operations are verified by reading the resource back and comparing the
  values (write-then-read round trip).
- Bound violations on path parameters (year, count) return the documented error.
- Report endpoints are checked for their CSV content type.

### US-3: Keep specification and implementation in sync

As an API consumer, I want the specification to describe exactly what the API
does, so that clients generated from it (including `bnaclient`) are correct.

Acceptance criteria:

- Each mismatch found while writing tests is fixed in either the code or the
  specification, never encoded as an expected-but-wrong behavior in a test.
- Each operation documents exactly the status codes the application can return:
  missing codes are added, impossible codes are removed.
- The specification documents only errors produced by the application. 401 and
  403, which are produced by API Gateway, are removed.
- Each mismatch is resolved case by case (code or specification), and every
  resolution is reviewed and approved before it is implemented.
- Pagination is consistent across the specification, the response headers, and
  the code documentation (first page index, header values, out-of-range
  behavior).
- Any fix that changes behavior visible to API clients is called out as an API
  change in the release that ships it.

### US-4: Catch coverage drift automatically

As a maintainer, I want the build to fail when an operation or status code has
no test, so that new endpoints cannot ship untested.

Acceptance criteria:

- Coverage is measured from the requests the suite actually sent and the
  responses the API actually returned during a run, not from annotations.
- The coverage check lists every uncovered coverage pair and fails when the list
  is not empty.
- The coverage check also fails when the API returned a status code that the
  specification does not document for that operation.
- The coverage check runs locally through a `just` recipe and in CI.

### US-5: Run anywhere, repeatedly

As a developer, I want to run the same suite against localhost or staging, as
many times as needed, without manual cleanup.

Acceptance criteria:

- The same Hurl files run against localhost and staging, selected by the
  variables file.
- All data created by the suite uses run-unique identifiers; a second
  consecutive run passes without resetting the database.
- Against staging, requests carry a bearer token supplied as a variable; against
  localhost the variable is empty and authentication is not required.
- The smoke test is read-only.

### US-6: Support release approval

As a senior engineer or manager approving a release, I want the suite and
coverage results available as evidence, so that I can approve a version with
confidence.

Acceptance criteria:

- The full Hurl suite and the coverage check run against the ephemeral local API
  in CI on every pull request.
- The release remains a deliberate team decision: a senior engineer or manager
  reviews the results of the suite and the coverage check for the release
  commit, plus any manual verifications, and approves the release.
- A failing suite or coverage check means the release is not approved.
- The test database dump is reproducible: it is generated from a committed
  recipe (schema, seed data, fixtures), so anyone can rebuild the same dump
  without depending on the state of a local database.
- The test database dump contains enough data for every read-only assertion:
  pipelines in several states, one submission per status, several Fargate
  prices, ratings with measures, a city in a country without regions, at least 5
  rated cities for each year used in top-cities tests, and the city referenced
  by the variables files.

## Non-Functional Requirements

- **Repeatability**: the suite produces the same result on consecutive runs
  against the same environment.
- **Independence**: each Hurl file can run on its own; files do not depend on
  data created by another file.
- **Speed**: the full suite against the ephemeral local API completes in a few
  minutes so it stays practical on every pull request.
- **Readability**: each request carries a comment stating the behavior under
  test; the coverage-pair declaration doubles as documentation.
- **Tooling**: uses Hurl (the version already pinned in CI) and existing `just`
  conventions; no new test framework.

## Scope Boundaries

In scope:

- Fixing, extending, and wiring the existing Hurl files in `lambdas/tests`.
- Code or specification changes needed to reconcile mismatches, followed by
  regenerating the OpenAPI files and `bnaclient`.
- The coverage check and its CI wiring.
- Rebuilding the test database dump from a reproducible recipe, extended with
  fixtures.
- Added during the review: removing the `effortless` crate, moving its error
  types to `lambdas`, and removing other unused code (the
  `patch-ratings-analysis` binary, the helpers predating Axum, the `Link` header
  parser, and the event fixtures). The `entity` wrappers stay, since Lambda
  functions outside this repository use them.

Out of scope:

- Fixing Schemathesis. It currently crashes the server and will be handled in a
  separate pull request. Until then, response shapes are not validated against
  the schema by any automated test.
- Errors produced by API Gateway (401, 403, and any other gateway response),
  including authentication tests. They will be handled in a follow-up pull
  request.
- Reorganizing the test layout per specification tag.
- Load, performance, or security testing.

## Key Decisions

- **Specification and implementation must always match**; a disagreement is a
  bug to fix, and tests assert the specification. (session-settled:
  user-directed — chosen over locking in current behavior: a mismatch most
  likely indicates a bug.)
- **Each operation documents exactly the status codes it returns**, adding
  missing ones and removing impossible ones. (session-settled: user-directed —
  chosen over keeping the blanket error lists and testing only reachable codes.)
- **Write tests use run-unique data** so the suite is repeatable on any
  environment, at the cost of the database growing. (session-settled:
  user-directed — chosen over a fresh database per run and over read-only
  tests.)
- **One set of Hurl files for localhost and staging**, with a bearer token on
  staging. (session-settled: user-approved — chosen over localhost-only and over
  no auth tests.)
- **Hurl tests behavior; schema conformance belongs to Schemathesis** once it is
  fixed. (session-settled: user-approved — chosen over Hurl asserting both
  behavior and shape, and over status codes only.)
- **The suite runs in CI against an ephemeral local API** with auth disabled.
  (session-settled: user-directed — chosen over running against staging.)
- **Releases are approved by a person, not gated automatically**: a senior
  engineer or manager approves using the suite and coverage results plus manual
  verifications. (session-settled: user-directed — chosen over automatically
  blocking the production deployment: releasing is a deliberate team decision.)
- **The specification documents only application errors**; API Gateway errors,
  including 401 and 403, are deferred to a follow-up pull request.
  (session-settled: user-directed — chosen over documenting gateway errors and
  testing them on staging.)
- **Mismatches are resolved case by case**, choosing code or specification for
  each one. (session-settled: user-directed — chosen over a blanket rule that
  one side always wins.)
- **Fix and extend the current layout, plus an enforced coverage check**.
  (session-settled: user-approved — chosen over reorganizing per specification
  tag and over unenforced coverage.)
- **The test dump is rebuilt from a committed recipe plus fixtures**.
  (session-settled: user-approved — chosen over patching the existing dump by
  hand and over keeping it as is: today's dump comes from whatever is in a local
  database and cannot be reproduced.)
- **Coverage is measured from the Hurl run report**. (session-settled:
  user-approved — chosen over parsing the `.hurl` files and over explicit
  comment tags: it reflects real traffic, needs no annotations, and also detects
  undocumented status codes.)

## Assumptions

- Authentication is enforced by API Gateway rather than by the Axum application
  (no authentication handling was found in `lambdas/src`).
- The existing `integration` workflow, which already seeds the database and
  starts the API on pull requests, is the base for running the suite in CI.

## Outstanding Questions

None blocking. Each mismatch's resolution is decided case by case in
`design.md`.
