# Review: Hurl API Coverage

The findings of the specification review of 2026-11-07 (the `pfb-spec-review`
skill), kept here so they can be worked on. To work on one, take its identifier,
update the specification if the behavior changes, and write the test first (see
[tasks.md](./tasks.md)), then set its status here.

The review only read the code. Where a finding says **observed**, it was also
reproduced against the local API.

## Findings

| ID   | Severity | Finding                                           | Status                  |
| ---- | -------- | ------------------------------------------------- | ----------------------- |
| R-01 | Major    | Client input still produces 500                   | Fixed                   |
| R-02 | Major    | `GET /pipelines/bna` skips the message decoding   | Open                    |
| R-03 | Major    | `Country` enum offers values the API rejects      | Open                    |
| R-04 | Major    | City `state` and `region` fields are confusing    | Moved to its own branch |
| R-05 | Major    | No test run exists for a tagged commit            | Open                    |
| R-06 | Major    | The suite cannot pass on staging                  | Deferred: to discuss    |
| R-07 | Major    | The pipeline scenario reads nothing back          | Open                    |
| R-08 | Major    | Task 4.1 is only partly done                      | Open                    |
| R-09 | Minor    | The specification contradicts itself on codes     | Fixed                   |
| R-10 | Minor    | Bodies accept undocumented fields                 | Open                    |
| R-11 | Minor    | Lists are paged without a stable order            | Open                    |
| R-12 | Minor    | `latest=true` reports misleading headers          | Open                    |
| R-13 | Minor    | `POST /ratings` is not transactional              | Open                    |
| R-14 | Minor    | 500 sources and report failures break the rule    | Open                    |
| R-15 | Minor    | Some requests lack a comment or a value assertion | Open                    |
| R-16 | Minor    | The suite will start failing from 2029            | Open                    |
| R-17 | Minor    | The verification records predate the latest code  | Open                    |
| R-18 | Minor    | Removals outside the scope of the specification   | Fixed                   |

### R-01 Client input still produces 500

**Observed.** A rule of M-03 is that client mistakes never produce 500. These
do:

- `POST` and `PATCH /pipelines/bna` with an unknown `fargate_price_id`
  (`pipelines/adaptor.rs`).
- `PATCH /cities/submissions/{submission_id}` with an unknown `country`
  (`cities/adaptor.rs`).
- A NUL character, which PostgreSQL rejects, in a path (`GET /usstates/%00`) or
  in a body string (`"\u0000"`).

Map the foreign key violations to `InvalidAttribute`, as
`post_cities_submission_adaptor` does, and reject NUL characters.

### R-02 `GET /pipelines/bna` skips the message decoding

The list handler (`pipelines/endpoint.rs`) serializes the raw model, so a
message stored double-encoded comes back as a string from the list and as an
object from `GET /pipelines/bna/{pipeline_id}`, against M-21. Map the models to
`BnaPipeline`, and add a double-encoded row to the fixtures.

### R-03 `Country` enum offers values the API rejects

**Observed.** `CityPost`, `SubmissionPost`, and `SubmissionPatch` document
`country` as an enum of 204 values, but the `country` table holds 27, so
`"Albania"` gets a 422. Decision: add the missing countries through a new
migration.

### R-04 City `state` and `region` fields are confusing

See M-44 in [design.md](./design.md). Moved to a dedicated branch for v2.

### R-05 No test run exists for a tagged commit

`integration` only runs on `pull_request`, and `main` uses squash merges, so the
check documented in `lambdas/tests/README.md` ("passed on the commit being
tagged") cannot be done. Run the workflow on pushes to `main` too.

### R-06 The suite cannot pass on staging

These requests need fixtures staging does not have: submission `1` being
`Pending` and a `Rejected` submission (`submissions.hurl`), 3 prices
(`pricing.hurl`) and 3 pipelines (`pipelines.hurl`), 3 ratings of the city in
the variables (`sorting.hurl`, `pagination.hurl`), and exactly 5 rated cities in
2025 (`cities.hurl`). `staging.vars` also names Austin, which is not in the
dump. How to run the tests on staging is to be discussed; the focus is on
localhost and CI.

### R-07 The pipeline scenario reads nothing back

`scenario/bna-pipeline.hurl` only checks statuses, and skips the `Setup` and
`Save` steps, while `design.md` says "move it through its steps" and "read
everything back".

### R-08 Task 4.1 is only partly done

`page=0`, `page=65537`, `page_size=0`, and `page_size=101` only run on
`/ratings`, and a page past the end only on `/ratings` and the city ratings.
Task 4.1 asks for them on all 8 lists, and `CityListParameters` and
`PriceListParameters` are separate types.

### R-09 The specification contradicts itself on codes

The error handling section and several tasks still said 400. Fixed in `bdd57af`.

### R-10 Bodies accept undocumented fields

`POST /cities` stores `fips_code`, absent from the `CityPost` schema. Document
it. `result_posted` and `torn_down` are parsed and never used; they stay for
now, since the `entity` wrappers stay.

### R-11 Lists are paged without a stable order

The ratings, pipelines, US states, and submissions lists have no `ORDER BY`, and
the sorted city list has no tie-breaker, so items can repeat or be skipped
between pages. Order each list by a unique key.

### R-12 `latest=true` reports misleading headers

`GET /prices/fargate?latest=true` returns one item, but `x-total` counts every
price and `x-page` echoes a `page` that is ignored (`price/db.rs`,
`price/endpoint.rs`).

### R-13 `POST /ratings` is not transactional

Nine separate inserts (`ratings/adaptor.rs`): a failure partway leaves a
`summary` row. Wrap them in a transaction. The code predates this feature.

### R-14 500 sources and report failures break the rule

500 errors report `source.pointer: "Unknown source"` with empty details
(`lib.rs`), against M-42, and `.expect("reports")` panics on a database error
instead of returning `APIErrors` (`reports/endpoint.rs`), against M-01.

### R-15 Some requests lack a comment or a value assertion

- No comment: `pipelines.hurl`, `reports.hurl`, `submissions.hurl`.
- Status only, with a body: `cities.hurl`, `reports.hurl`, `pagination.hurl`,
  `submission-lifecycle.hurl`.

### R-16 The suite will start failing from 2029

`/cities/top/2029/5` expects 404 (`cities.hurl`), which stops holding once
ratings are created in 2029, and `/reports/{{report_year}}` (`reports.hurl`)
returns 400 from 2030. Trigger the 404 another way, and make the year bound
follow the clock.

### R-17 The verification records predate the latest code

Task 12.1 records an older run. Rerun twice in a row on the final commit and
update it, along with the note of task 12.5.

### R-18 Removals outside the scope of the specification

Recorded in the scope of `requirements.md` in `bdd57af`.

## Test Gaps

- Non-US cities on `POST /cities`: the region default and an explicit region,
  and the `state_abbrev` round trip.
- `sqs_message: null` clearing the message.
- `PATCH` of `fargate_price_id`, and of submission fields other than `status`.
- `order_direction=bogus` and `latest=bogus` returning 400.
- `measure` values on `GET /ratings` and `GET /ratings/{rating_id}/city`.
- A city without ratings counting one item in `x-total` (M-37), and the headers
  of an empty list (M-41, only covered by doc tests).
- Fixtures no test reads: Utrecht in a country without regions, and the top
  cities of 2019.
