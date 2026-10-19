# Tasks: Hurl API Coverage

Feature #0001: hurl-api-coverage. Implements [design.md](./design.md) for
[requirements.md](./requirements.md).

## How to Work Through This List

- Phases run in order; tasks inside a phase can be done in any order unless
  noted.
- **Test first**: for every API fix (phases 3 to 8), write or update the Hurl
  request asserting the specified behavior, see it fail, then fix the code or
  the specification until it passes.
- After any change to an endpoint, schema, or `utoipa` annotation: back up
  `bnaclient/Cargo.toml` if edited, then run `just regenerate-all-no-cargo`.
- Before each commit: `cargo test`, `cargo clippy`, `just lint`, and the Hurl
  suite against localhost (`cd lambdas/tests && just test localhost`).
- Each client-visible change gets an entry under `## [Unreleased]` in
  `CHANGELOG.md` in the same commit.
- **One change per commit**, so each one is easy to review and revert:
  - one commit per mismatch (M-xx), containing its Hurl test, the code or
    specification change, the regenerated `openapi-*.yaml` and `bnaclient` when
    the specification changes, and its `CHANGELOG.md` entry;
  - one commit per test bug (T-xx);
  - one commit per infrastructure step (baseline, fixtures, dump recipe,
    regenerated dump, coverage script, `just` recipes, CI wiring);
  - the test and its fix go in the same commit, so every commit builds and
    passes on its own;
  - never mix a fix with unrelated cleanup.

## Phase 1: Reproducible Test Data

- [x] 1.1 Extract the baseline data.
  - New: `lambdas/tests/baseline.sql`, a data-only export of `city`, `summary`,
    and the seven rating detail tables from the current
    `database-with-test-data.sql`.
- [x] 1.2 Write the fixtures file.
  - New: `lambdas/tests/fixtures.sql`, with fixed identifiers and timestamps.
  - Pin the `created_at` of the Fargate price inserted by the migrations.
  - 3 pipelines (`Pending`, `Processing`, `Completed` with cost and end time).
  - 3 submissions (`Pending`, `Approved`, `Rejected`).
  - 3 Fargate prices with distinct dates and values.
  - Measures on at least 2 ratings.
  - 1 city in a country without regions (region equals the country name), with a
    rating.
  - Ratings for at least 5 cities in one early year (for example 2019).
- [x] 1.3 Replace the `dbdump` recipe with a reproducible one.
  - `lambdas/tests/justfile`: a recipe that, against a dedicated database (never
    the working one), runs the migrations from scratch, then `baseline.sql`,
    then `fixtures.sql`, then `pg_dump` into `database-with-test-data.sql`.
- [x] 1.4 Regenerate `lambdas/tests/database-with-test-data.sql` with the new
      recipe and commit it.
- [x] 1.5 Document the recipe in `lambdas/tests/README.md` (replaces the
      "Preparing the assets" section).

Done when: running the recipe twice produces the same data, and the dump
contains every item of the design's "Test data" section.

## Phase 2: Fix the Existing Tests (no API change)

- [x] 2.1 T-07: point `lambdas/tests/staging.vars` and the
      `test-staging-schemathesis` recipe in `lambdas/tests/justfile` at
      `https://api.staging.bna.peopleforbikes.org`.
- [x] 2.2 T-01: add a plain (not URL-encoded) country variable to
      `localhost.vars` and `staging.vars`; use it in request bodies.
- [x] 2.3 T-06: remove the unused `city_id` from `localhost.vars` and
      `staging.vars`; remove the stale `rating_id` from `staging.vars`.
- [x] 2.4 Add a `token` variable, empty in `localhost.vars`, never committed for
      staging; send `Authorization: Bearer {{token}}` on every write request.
- [x] 2.5 T-02 and run-unique data in `scenario/bna-pipeline.hurl`: use
      `{{newUuid}}` for `state_machine_id`, create its own city, and fix
      `start_time`/`end_time` order.
- [x] 2.6 T-03: rewrite the comments of `scenario/pagination.hurl` to match the
      assertions.
- [x] 2.7 T-04: fix the comment in `endpoints/system.hurl`; assert the `OK` body
      and `text/plain` content type.
- [x] 2.8 T-05: remove the empty `[Asserts]` in `endpoints/ratings.hurl` and the
      unused captures in `endpoints/cities.hurl`; add value assertions to every
      request with a body.
- [x] 2.9 Make `endpoints/ratings.hurl` write run-unique data and read the
      created rating back.
- [x] 2.10 Make `smoke/public.hurl` read-only: move the submission creation to a
      new `scenario/submission-lifecycle.hurl` (completed in phase 5).
- [x] 2.11 Make `just test` run every Hurl file (including
      `scenario/bna-pipeline.hurl`) from one list, and add a `--report-json`
      output directory.

Done when: the whole suite passes twice in a row against localhost without
resetting the database.

## Phase 3: Error Contract (M-01, M-02, M-03)

- [x] 3.1 Hurl: add requests for malformed path, query, and body inputs on each
      resource, asserting 400 with an `APIErrors` JSON body and the right
      `source`.
- [x] 3.2 Code: map every Axum extractor rejection (path, query, JSON) to an
      `APIErrors` response with status 400.
  - `effortless/src/error.rs`, `lambdas/src/lib.rs`
    (`impl IntoResponse for ExecutionError`), and the extractors used in
    `lambdas/src/core/resource/*/endpoint.rs`.
- [ ] 3.3 Code: database errors caused by client input must never surface as 500
      (enforced resource by resource in phases 5 to 7).
- [x] 3.4 `CHANGELOG.md`: JSON error bodies everywhere; 400 replaces 422.

## Phase 4: Pagination (M-05 to M-10)

- [x] 4.1 Hurl: extend `scenario/pagination.hurl` to cover every list operation
      (`/cities`, `/cities/submissions`, `/cities/summary/latest`,
      `/cities/{country}/{region}/{name}/ratings`, `/pipelines/bna`,
      `/prices/fargate`, `/ratings`, `/usstates`): default page, explicit page,
      header values (`x-page`, `x-per-page`, `x-total`, `x-total-pages`,
      `x-next-page`, `x-prev-page`, `link`), out-of-range values, and a page
      past the end.
- [x] 4.2 M-05: make pages 1-based, default 1, reject `page=0`.
  - `lambdas/src/core/resource/schema.rs` (`page()`), the paginated queries in
    each `db.rs`, and `lambdas/src/lib.rs` (`Paginatron`, docs).
- [x] 4.3 M-06, M-07: reject `page` above 65536 and `page_size` outside 1–100
      with 400.
- [x] 4.4 M-08: fix the total counts of `GET /cities` and `GET /usstates`.
  - `lambdas/src/core/resource/cities/db.rs`,
    `lambdas/src/core/resource/usstates/db.rs`.
- [x] 4.5 M-09: return pagination headers from `GET /pipelines/bna`.
  - `lambdas/src/core/resource/pipelines/endpoint.rs`.
- [x] 4.6 M-10: document that a page past the end returns an empty list.
- [x] 4.7 `CHANGELOG.md`: 1-based pages; out-of-range `page`/`page_size` now
      rejected.

## Phase 5: Query Parameters, Cities, and Submissions (M-11 to M-19)

- [ ] 5.1 Hurl: new `scenario/sorting.hurl` covering `sort_by` and
      `order_direction` on every operation that accepts them, using fixture data
      with known order.
- [ ] 5.2 M-11: declare the allowed `sort_by` values per operation as an enum;
      reject others with 400.
- [ ] 5.3 M-12: remove `latest` from `GET /cities` and
      `GET /cities/summary/latest`; keep and test it on `GET /prices/fargate`.
- [ ] 5.4 M-13: make the submission `status` filter an enum (`Pending`,
      `Approved`, `Rejected`); reject unknown values with 400.
- [ ] 5.5 Hurl: complete `scenario/submission-lifecycle.hurl`: create, read,
      filter by status, update the status, check the filter results change.
- [x] 5.6 M-17: read `submission_id` from the path in `patch_cities_submission`.
  - `lambdas/src/core/resource/cities/endpoint.rs`.
- [x] 5.7 M-18: fix the `status` query extractor of `get_cities_submission`.
- [ ] 5.8 Hurl: `endpoints/cities.hurl` covers `POST /cities` (201 with
      read-back, 400, 409) and unknown countries on read.
- [ ] 5.9 M-14: return and document 409 for a duplicate city.
- [ ] 5.10 M-15, M-16: return 400 for an unknown country, on create and on read.
- [ ] 5.11 M-19: return 400 for out-of-range `year` and `count` on
      `/cities/top/{year}/{count}`.
- [ ] 5.12 `CHANGELOG.md`: submission update fixed; `latest` removed from city
      lists; stricter `sort_by`, `status`, country, year, and count validation.

## Phase 6: Ratings (M-20, M-29)

- [ ] 6.1 Hurl: `POST /ratings` with an unknown `city_id` returns 400 with
      `source.pointer` `/city_id`.
- [ ] 6.2 Code: validate the city before inserting the rating.
  - `lambdas/src/core/resource/ratings/adaptor.rs`.
- [ ] 6.3 M-29 (pending approval): Hurl asserts the fixture measures on
      `GET /ratings/{rating_id}`; include the measure in every rating read.
  - `lambdas/src/core/resource/ratings/db.rs`.

## Phase 7: Pipelines (M-21 to M-25, M-28)

- [ ] 7.1 Hurl: new `endpoints/pipelines.hurl` covering list, read (200, 404,
      400 on an invalid UUID), create (201, 400, 409), and update (200, 400,
      404), with read-back of every written field.
- [ ] 7.2 M-21: declare and return `sqs_message` as a JSON object.
  - `lambdas/src/core/resource/pipelines/schema.rs`,
    `entity/src/wrappers/bna_pipeline.rs`.
- [ ] 7.3 M-22: return and document 409 for a duplicate `state_machine_id`.
- [ ] 7.4 M-23: return 404 when updating an unknown pipeline.
- [ ] 7.5 M-24: return 400 for an invalid `status` or `step` in the update body.
- [ ] 7.6 M-25: apply `start_time` on update.
  - `entity/src/wrappers/bna_pipeline.rs` (`start_time: ActiveValue::NotSet`).
- [ ] 7.7 Tell the owners of the pipeline state machine that `sqs_message` must
      be sent as an object; note it in `CHANGELOG.md`.
- [ ] 7.8 M-28 (pending approval): align the `bna_pipeline_step` table with the
      API's steps through a new migration, then update the fixtures pipeline
      using `Setup` back to `SqsMessage` and regenerate the dump.
  - `migration/src/`, `lambdas/tests/fixtures.sql`.

## Phase 8: Reports and US States (M-26, M-27)

- [ ] 8.1 Hurl: `endpoints/reports.hurl` asserts `text/csv` and the header line,
      and 400 for years outside 2017–2029.
- [ ] 8.2 M-26: validate the year range in
      `lambdas/src/core/resource/reports/endpoint.rs`.
- [ ] 8.3 Hurl: `endpoints/usstates.hurl` covers an unknown state (404) and the
      case sensitivity.
- [ ] 8.4 M-27: document in the specification that the state name is case
      sensitive.

## Phase 9: Documented Status Codes (M-04)

- [ ] 9.1 Remove 401 and 403 from every operation.
  - The shared `ErrorResponses` in `lambdas/src/core/resource/schema.rs` and
    each endpoint's `utoipa` `responses`.
- [ ] 9.2 For each operation, keep exactly the codes it can return after phases
      3 to 8 (adding 409 where created, removing unreachable 400 or 404).
- [ ] 9.3 Regenerate `openapi-3.1.yaml`, `openapi-3.0.yaml`, and `bnaclient`
      (`just regenerate-all-no-cargo`).

## Phase 10: Coverage Check (US-4)

- [ ] 10.1 New script `lambdas/tests/coverage.py`: read `openapi-3.1.yaml` and
      the Hurl JSON report; match requests to operations (literal segments win
      over parameters); print uncovered pairs, undocumented pairs (including any
      500), unknown paths, and a summary; exit non-zero if any list is not
      empty.
- [ ] 10.2 Add the YAML parser to `lambdas/tests/pyproject.toml`; update
      `uv.lock`.
- [ ] 10.3 New `just coverage` recipe in `lambdas/tests/justfile`.
- [ ] 10.4 Fill every remaining uncovered pair with a Hurl request until
      `just coverage` passes.

## Phase 11: CI (US-6)

- [ ] 11.1 `.github/workflows/integration.yml`: install `just` and `uv`; load
      the regenerated dump; run the suite through `just test localhost` instead
      of listing files; run `just coverage`.
- [ ] 11.2 Upload the Hurl report and coverage output as artifacts; write the
      coverage summary to the job summary.
- [ ] 11.3 Document the release check in `lambdas/tests/README.md`: before
      pushing a version tag, confirm the `integration` workflow passed on that
      commit and complete any manual verifications.

## Phase 12: Final Verification

- [ ] 12.1 Fresh database from the new dump: `just test localhost` passes twice
      in a row; `just coverage` passes.
- [ ] 12.2 `just test staging` passes against
      `https://api.staging.bna.peopleforbikes.org` with a token (after staging
      is deployed with the changes).
- [ ] 12.3 `cargo test`, `cargo clippy`, `just lint` pass; `bnaclient` CI job
      shows no diff.
- [ ] 12.4 `CHANGELOG.md` lists every client-visible change (M-01, M-02, M-05,
      M-07, M-12, M-13, M-15, M-16, M-19, M-21, M-26).
- [ ] 12.5 Every acceptance criterion in `requirements.md` is checked off.
