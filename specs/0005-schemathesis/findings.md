# Findings: Property-Based API Testing with Schemathesis

Everything non-obvious learned while implementing this feature: behavior that
looks like a bug unless you know it is deliberate, traps in the libraries and
tools, and what worked. Each entry says what the rule is, how it was caught, and
what it cost.

Status: done, tasks 1 to 7 (pull request 555). Measured with bacon 3.26.0, uv
0.13.8 and Schemathesis 4.33.4 on macOS arm64, in the nono sandbox, on
2026-11-13.

## 1. Behavior that does not survive a naive implementation

### 1.1 A `null` in the two PATCH bodies means "unchanged", on purpose

`PATCH /cities/submissions/{submission_id}` and
`PATCH /pipelines/bna/{pipeline_id}` read `null` as an absent field, while the
specification documents the optional enums and strings of these bodies
non-nullable (`nullable = false`). The bodies deserialize the entity wrappers
(`entity/src/wrappers/submission.rs:40`,
`entity/src/wrappers/bna_pipeline.rs:88`), which serialize an unset field as
`null`, and Lambdas in other repositories use these wrappers: rejecting `null`
could break them. Schemathesis' `negative_data_rejection` is disabled for these
two operations (`lambdas/tests/schemathesis.toml:61`, `:65`).

**Caught by**: cause E (section 4), then the check of the wrappers' `Serialize`
before implementing the fix the team had first chosen (reject `null`).

**Cost**: the fix as first chosen would have answered `422` to every PATCH a
Lambda sends with an unset field. The exclusion loses the check of other invalid
data on these two operations; a type error still fails deserialization, with
`422`.

### 1.2 A body is a JSON object, although serde reads a struct from an array

serde's derived `Deserialize` reads a struct from a JSON array too, matching the
fields by position: `"infrastructure": [null, null]` created a rating, and an
array of eleven elements updated a submission (`PATCH` with
`[null, …, "Approved"]` approved it). `ApiJson` answers `422` to a body that is
valid JSON but not an object (`lambdas/src/core/extract.rs:150`), and the eight
categories of a rating body go through `deserialize_object`
(`lambdas/src/core/resource/schema.rs:684`). A new nested struct in a body needs
it too; `BNAPipelinePost.sqs_message` holds arbitrary JSON, arrays included, so
a body cannot simply refuse every array.

**Caught by**: cause H, which the 76 failures of cause A hid in the first run.

**Cost**: one commit; the check of the body is one byte comparison after
deserialization.

### 1.3 Rating scores are checked before the transaction opens

`post_ratings_adaptor` rejects the first score outside 0 to 100 with `422`,
naming its field path (`lambdas/src/core/resource/ratings/adaptor.rs:68`,
`lambdas/src/core/resource/ratings/schema.rs:265`), before any query: the unit
test checks that the transaction log is empty.

**Caught by**: cause A, 75 of the 86 first failures.

**Cost**: stored out-of-range scores made every later read of the rating violate
the response schema, so one missing check looked like 75 defects.

## 2. Library and tooling traps

### 2.1 bacon restarts the server when a test run writes under `lambdas/tests/`

bacon watches every package's `src`, `tests`, `benches` and `examples`
directories, and `lambdas/tests/` is the `tests` directory of the `lambdas`
package, although it holds the Hurl suites, not Rust tests. With
`on_change_strategy = "kill_then_restart"`, the `axum-debug` job killed the
server whenever a file there changed: `just test` deletes and recreates
`reports/`, and Schemathesis 3 wrote `cassette.yaml` during its run. A
`.gitignore` rule does not help for `reports/`: recreating the ignored directory
itself still killed the server. The job now ignores the folder
(`bacon.toml:121`); a folder pattern covers its content since bacon 3.23.0.

**Caught by**: during the specification, the smoke tests passed and the next
`just test localhost` failed on all 12 files with "Failed to connect" in 0 ms. A
probe polling `/health` and the `axumed` PID every 100 ms showed the server
killed 100 ms after `reports/` was recreated, back after 1.6 s; a new file in
`lambdas/tests/` did the same.

**Cost**: the local Hurl suites looked broken, and Schemathesis looked like it
crashed the server: "Connection aborted. Remote end closed connection without
response" is what it reports when bacon kills the server during a request. That
message is why the CI step was commented out; whether the CI failure had the
same cause is unknown, since CI runs no bacon. With the ignore, task 1.2
measured the PID unchanged and `/health` at `200` for 8 s after each change
under `lambdas/tests/`.

### 2.2 `touch` does not make bacon restart; a content change does

Touching `lambdas/src/lib.rs` (modification time only) left the server untouched
for 8 s. Appending an empty line restarted it: down after 0.2 s, back with a new
PID after 5.9 s (debug build, incremental). The same change to
`entity/src/lib.rs`, after the spec review asked for it: down after 0.2 s, back
with a new PID after 5.5 s.

**Caught by**: task 1.2, whose first version of the probe used `touch` and
concluded no restart.

**Cost**: a probe built on `touch` would wrongly report that bacon no longer
watches `lambdas/src/`. Check a watcher with a real content change, then restore
the file (which restarts it once more).

### 2.8 A new stable Rust can fail `cargo fmt --check` on unchanged code

CI installs the latest stable toolchain (`dtolnay/rust-toolchain@stable`). On
2026-11-13, the day after Rust 1.100.0, the `ci-rust` lint job of pull request
555 failed on an empty block comment (`/*` then `*/`, from July 2025) in
`lambdas/src/core/resource/cities/schema.rs`: its rustfmt wants the closing line
indented, while the rustfmt of Rust 1.99.1, used locally, does not. The comment
held nothing and was removed (`715a2b2`), so every version agrees.

**Caught by**: the first CI run of the pull request; `just ci` passed locally.

**Cost**: a red required `lint` check unrelated to the change. Any pull request
would have hit it, and the toolchain is not pinned, so a later release can do it
again.

### 2.3 progenitor 0.16.0 still panics on a nullable reference

Removing `nullable = false` from `SubmissionPatch.status` turns it into
`oneOf: [{$ref: ApprovalStatus}, {type: 'null'}]`, and `just generate` fails:
progenitor-impl 0.16.0 panics at `to_schema.rs:560`. The learning
`specs/learnings/build-errors/openapi-schemas-that-break-client-generation.md`
names 0.14.0; the limit still holds in 0.16.0.

**Caught by**: an experiment for cause E, reverted.

**Cost**: the specification cannot document "`null` means unchanged", so section
1.1 needs a check excluded.

### 2.4 Schemathesis counts a documented `422` as rejecting valid data

`positive_data_acceptance` expects `2XX`, `3XX`, `401`, `403`, `404`, `409`,
`429` or `5XX` for data valid against the schema. A `422` for a rule the schema
cannot express (an unknown foreign key, a state that is not a US state, a
date-time `format`, which is an annotation in OpenAPI 3.1) fails it. The four
operations concerned list `422` in their expected statuses
(`lambdas/tests/schemathesis.toml:40` to `:53`); the default list must be
repeated with it, and the wildcards are written `"2XX"`.

### 2.5 Schemathesis infers links from nullable identifiers

The stateful phase inferred a link from `fargate_price_id`, which is nullable,
to `GET /prices/fargate/{price_id}`, and sent the `null` as the path segment
`None`. The check is disabled for that operation in the stateful phase only
(`lambdas/tests/schemathesis.toml:73`).

### 2.6 A passing run deletes Schemathesis' crash files

`st replay <id>` replays a failure from `.schemathesis/default/cache/crashes/`,
but the files of failures that no longer occur are gone after a passing run:
`st replay m4mhTV` answered "no crash file found". A random failure that the
latest run did not hit cannot be replayed; see section 3.2.

### 2.7 A generator run during a bacon rebuild can use the old code

`just generate-oas-31`, run while bacon rebuilt the server after a source
change, waited for the build directory lock and left `openapi-3.1.yaml`
unchanged; the negative check of task 2.5 then passed, wrongly. Run again once
bacon's build is done, and check the specification's diff before trusting the
run.

## 3. Method — what actually worked

### 3.1 Poll the health endpoint and the server's PID

A loop printing `/health`'s status and the `axumed` PID every 100 ms, only when
either changes, tells a restart (PID changes) from a crash (no PID) from a slow
answer, and times each. It turned "the tests fail" into "the server is killed
100 ms after this file operation" in one run.

### 3.2 Compare the rate of a random failure with and without its fix

Cause G occurs in some runs only. Ten stateful-only runs (`--phases stateful`,
about 3 s each) with a copy of the configuration without the exclusion reported
it 7 times; ten with the exclusion, never. The same idea checked cause B: before
its fix, the `414` appeared in 3 of the first 5 full runs; after it, in none
of 5.

### 3.3 Write the Hurl request first, and run it against the unfixed server

Each fix started with its Hurl request, run against the running server before
the change: `201` for a score of 101, `201` and `200` for the arrays, `404` for
the 256-character names. Requirement 4.2 asks for exactly that evidence, and it
caught that a body array needs eleven elements, not ten, to pass.

## 4. The first run of Schemathesis 4 (tasks 2.4 and 3.1)

`just test-schemathesis` on a fresh `just db-seed`, against the Dev_Server: 26
of 26 operations tested in 24 s (5,335 test cases), exit `1`, 86 unique
failures, one warning. The four phases ran: examples (15 passed, 2 failed),
coverage (21, 5), fuzzing (17, 9), stateful (113 scenarios, 102 passed, 11
failed). The stateful phase inferred 44 links and covered 23, so chained calls
work without OpenAPI links in the specification. The reports landed in
`schemathesis-report/`, the cache in `.schemathesis/`, and `git status` stayed
clean.

| Failure                                 | Count |
| --------------------------------------- | ----- |
| ST108 Response violates schema          | 75    |
| ST202 Schema-valid request rejected     | 8     |
| ST201 Schema-violating request accepted | 2     |
| ST103 Undocumented status code          | 1     |

The 86 failures come from seven causes (an eighth, H, appeared once A was
fixed):

| Cause                                                   | Failures           | Operations                                                                                                                       | Classification                          |
| ------------------------------------------------------- | ------------------ | -------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------- |
| A. Scores outside 0 to 100 are stored                   | 75 ST108, 1 ST201  | `POST /ratings`, then every read of the rating (`GET /ratings`, `/ratings/{rating_id}`, `/ratings/{rating_id}/city`, stateful)   | API and specification defect            |
| B. A URI over 65,534 bytes gets `414`                   | 1 ST103, 1 ST202   | `GET /cities/{country}/{region}/{name}/ratings`                                                                                  | transport limit, outside the API        |
| C. A body valid for the schema breaks a rule of the API | 3 ST202            | `POST /ratings` (unknown `city_id`), `POST /pipelines/bna` (unknown `fargate_price_id`), `POST /cities` (`state` not a US state) | documented `422`                        |
| D. A string with a NUL character gets `422`             | 2 ST202            | `POST /cities/submissions`, `PATCH /cities/submissions/{submission_id}`                                                          | deliberate rejection                    |
| E. `"status": null` is accepted                         | 1 ST201            | `PATCH /cities/submissions/{submission_id}`                                                                                      | API more lenient than its specification |
| F. `"end_time": ""` gets `422`                          | 1 ST202            | `PATCH /pipelines/bna/{pipeline_id}`                                                                                             | documented `422`                        |
| G. An inferred link follows a null identifier           | 1 ST202 (stateful) | `GET /pipelines/bna` then `GET /prices/fargate/None` (`400`)                                                                     | link inference                          |

- **A**: every subscore (`CoreServices`, `Opportunity`, `People`, `Recreation`,
  `Retail`, `Transit`) declares `minimum: 0` and `maximum: 100`, in the request
  and in the response, but `POST /ratings` stores `-1.0`, `101.0` or `-3.4e38`
  and answers `201`; `RatingPost.score` declares no bound at all, while
  `Rating.score`, in every response, declares 0 to 100. The 75 ST108 are the
  stored values read back: 41 below 0, 34 above 100, over 20 score fields.
- **B**: hyper answers `414` between 65,480 and 65,500 characters of path
  (measured with `curl`), before the router: the handler never runs. Behind API
  Gateway, the limit is the gateway's.
- **C**, **F**: `422` is documented for these operations; Schemathesis'
  `positive_data_acceptance` counts a `422` as rejecting valid data. A date-time
  `format` is an annotation in OpenAPI 3.1 (JSON Schema 2020-12), so `""` passes
  the schema of `end_time`.
- **D**: PostgreSQL text cannot store NUL; the API answers `422` with "a string
  of the body must not contain a NUL character".
- **E**: `SubmissionPatch.status` is an optional enum, marked `nullable = false`
  on purpose (progenitor panics on a nullable reference:
  `specs/learnings/build-errors/openapi-schemas-that-break-client-generation.md`);
  serde reads `null` as an absent field and the PATCH succeeds.
- **G**: the link from `GET /pipelines/bna` to `GET /prices/fargate/{price_id}`
  was inferred from `fargate_price_id`, which is nullable; the `null` became the
  path segment `None`, which the API rightly rejects with `400`.

**H**, found by the second run (after fix A, which hid it): `POST /ratings`
accepted `"infrastructure": [null, null]` with `201` (ST201). serde's derived
`Deserialize` reads a struct from a JSON array too, matching the fields by
position, so any array of the right length passes for a struct, nested or at the
top of a body. `RatingPost` is the only body with nested structs (eight); the
other bodies are flat, and `BNAPipelinePost.sqs_message` holds arbitrary JSON,
arrays included, so rejecting every array is not an option.

Warning: `GET /cities/{country}/{region}/{name}` and its `/ratings` returned
`404` to nearly every request: Schemathesis cannot guess an existing country,
region and name, so these two operations barely reach their logic.

### 4.1 Outcome of each cause (task 3.2)

| Cause | Decision                                                                          | Commit    |
| ----- | --------------------------------------------------------------------------------- | --------- |
| A     | API fix: scores checked, `RatingPost.score` documents 0 to 100                    | `4621cae` |
| H     | API fix: a body and a rating category must be objects                             | `99ac039` |
| C, F  | excluded: `422` accepted for valid data on four operations                        | `7bac4a7` |
| E     | excluded: `null` keeps meaning unchanged (section 1.1)                            | `5331742` |
| D     | `generation.allow-x00 = false`: the API rejects NUL on purpose                    | `4e45960` |
| G     | excluded in the stateful phase for one operation                                  | `b91ce85` |
| B     | API fix: region, city and US state names of at most 255 characters, `400` past it | `e6ae4de` |

The team chose each decision that changes the API's behavior (A, B, E, H, and
the 255-character limit). After the last fix, `just test-schemathesis` passed 5
runs out of 5 on fresh test data (21 to 23 s each, about 5,200 test cases), with
`just test localhost` (12 of 12 files) and `just coverage` (74 of 74 pairs)
passing too. Two warnings remain, neither failing the run:

- `GET /cities/{country}/{region}/{name}` and its `/ratings` reach their logic
  only with an existing city, which Schemathesis cannot guess;
- `POST /pipelines/bna` mostly gets `422`, since a random `fargate_price_id`
  rarely matches a price.

### 4.2 The CI runs (task 7)

Pull request #555, head `5146ccd`, integration run `43485935034`: the `test` job
passed in 2 min 27 s. The Schemathesis step took 24 s: about 1 s to download
CPython 3.13.18 and install the 27 packages of the Pinned_Release, then 22 s of
testing. It tested 26 of 26 operations, 5,600 test cases, all passing, with 153
stateful scenarios (20 of 44 inferred links covered), and the same two warnings
as locally. The `schemathesis-reports` artifact (JUnit and HTML, 17.8 kB) and
`integration-reports` were uploaded. The `lambdas` and `migration` workflows
passed; `ci-rust` failed on rustfmt (section 2.8).

The second run, on head `4d9b46e` after the rustfmt fix (integration run
`43487781516`), passed every workflow: `ci-rust`, `integration`, `bnaclient`,
`lambdas` and `migration`. Its Schemathesis step took 32 s: 5,095 test cases,
all passing, 31 stateful scenarios with 10 of the 44 inferred links covered. The
number of scenarios and covered links varies from run to run (153 and 20 in the
first), since the stateful phase is random.

Follow-ups, outside this feature: identifiers or OpenAPI links for these two
warnings; a length limit on the body fields matching the 255 characters of the
path parameters (`POST /cities`, submissions), so that every stored city stays
reachable; and the nullable fields of section 1.1, if progenitor learns nullable
references.
