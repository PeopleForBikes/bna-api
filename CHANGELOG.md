# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `GET /prices/fargate/latest` returns the latest Fargate price, the most recent
  one, as a single `FargatePrice` object. It replaces
  `GET /prices/fargate?latest=true`.

### Changed

- Requests with an invalid path parameter, query string parameter, or body now
  get a JSON `APIErrors` document, as documented in the OpenAPI specification,
  instead of a plain text message. The error `source` names the failing query
  parameter, the `content-type` header, or otherwise holds the request path in
  `pointer`. `details` names the invalid field; only the first one is reported.
- Pages now start at 1, as documented in the OpenAPI specification. The first
  page used to be page 0, so `page=1` returned the second page. The default page
  is 1, `page=0` is rejected with `400 Bad Request`, and the `x-page`,
  `x-prev-page`, and `x-next-page` headers use the same numbering.
- A `page` above 65536, the maximum documented in the OpenAPI specification, is
  now rejected with `400 Bad Request`.
- A `page_size` outside 1 to 100, the range documented in the OpenAPI
  specification, is now rejected with `400 Bad Request`. It used to be silently
  changed to 1 or 100.
- The submission `status` filter now only accepts `Approved`, `Pending`, or
  `Rejected`, declared as the `ApprovalStatus` enum in the OpenAPI
  specification. Other values are rejected with `400 Bad Request` instead of
  returning an empty list.
- Reading a city, or its ratings, in a country outside the documented `Country`
  enum now returns `400 Bad Request` instead of `404 Not Found`.
- `GET /cities/top/{year}/{count}` now rejects a year outside 2017 to 2029 or a
  count outside 1 to 100, the documented bounds, with `400 Bad Request`. An
  out-of-range year used to return `404 Not Found`, and an out-of-range count
  was silently changed to 1 or 100.
- The `latest` query parameter is no longer documented on `GET /cities` and
  `GET /cities/summary/latest`: it never had any effect there.
- `sort_by` now only accepts the documented fields, declared as enums in the
  OpenAPI specification: `name`, `country`, `state`, `state_abbrev`, `region`,
  `fips_code`, or `created_at` for the city lists, and `per_second` or
  `created_at` for the Fargate prices. Other values are rejected with
  `400 Bad Request` instead of being silently ignored.
- The lists, and `GET /cities/submissions/{submission_id}`, now reject a query
  parameter they do not take with `400 Bad Request`, the error `source` naming
  it in `parameter`. Unknown parameters, misspelled ones included, used to be
  silently ignored.
- `GET /reports/{year}` now rejects a year outside 2017 to 2029, the documented
  bounds, with `400 Bad Request`, now documented. It used to return an empty
  report.
- The pipeline `sqs_message` is now always returned as a JSON object, as the
  OpenAPI specification now documents, so the operator can read it back and
  reproduce a run. It used to be documented as a string and returned as a string
  holding the JSON object. A request may send it as a JSON object or as a string
  holding a JSON object; both are stored as the object, where a string used to
  be stored double-encoded. Any other value is now rejected with
  `422 Unprocessable Entity`. Messages stored double-encoded are returned
  decoded.
- The OpenAPI specification no longer documents `401 Unauthorized` and
  `403 Forbidden` responses. They are produced by API Gateway, not by the
  application, and will be documented separately.
- Each operation in the OpenAPI specification now documents exactly the error
  codes it can return: the lists now document `400 Bad Request` for invalid
  query parameters, and the lists and creations no longer document
  `404 Not Found`. The operations taking a body now document
  `415 Unsupported Media Type` for a missing content type and
  `422 Unprocessable Entity` for rejected content, which they already returned;
  `400 Bad Request` is for a body that is not valid JSON.
- The pipeline steps are now `Setup`, `Analysis`, `Save`, and `Cleanup`, the
  steps the database accepts. `Save` records that the results were saved in the
  database. `SqsMessage` is no longer a step: the database always rejected it.
- The OpenAPI specification now documents every field of the pipeline update
  body (`BnaPipelinePatch`) as optional, as the API always accepted: `status`
  and `step` were documented as required.
- The OpenAPI specification now documents every field of the submission update
  body (`SubmissionPatch`) as optional, as the API always accepted: most fields
  were documented as required.
- The OpenAPI specification now documents the pagination headers of every list
  (`link`, `x-page`, `x-per-page`, `x-total`, `x-total-pages`, `x-prev-page`,
  and `x-next-page`) and their values at the edges. An empty list now reports
  page 1 as its next page instead of page 0, which does not exist.
- The submission creation body (`SubmissionPost`) no longer documents a
  `status`: a new submission is always `Pending`, and a `status` sent in the
  body is ignored, as it always was.
- The OpenAPI specification now documents the `BnaRegion` and `BikeLaneType`
  schemas, so `bnaclient` provides them too. `entity::wrappers::BikeLaneType`
  now uses the names of the `bike_lane_type` table (for example `buffered_lane`
  instead of `BufferedLane`) and can be converted from and to a string.
- Every country of the documented `Country` enum is now accepted. The `country`
  table held only 27 of the 204, so the others were rejected with
  `422 Unprocessable Entity`. A new migration adds them.
- The cities now return their `fips_code`, and the OpenAPI specification
  documents it in the city creation body (`CityPost`). The API stored it without
  returning or documenting it.
- All the dependencies are updated to their latest version, including utoipa 6,
  utoipa-axum 0.3, and utoipa-swagger-ui 10. The OpenAPI specification is
  unchanged.

### Removed

- The `latest` query parameter of `GET /prices/fargate`: use
  `GET /prices/fargate/latest` instead. A request still sending it is rejected
  with `400 Bad Request`. With `latest=true`, the list returned one price but
  its pagination headers counted every price and echoed the ignored `page` and
  `page_size`, and `sort_by=per_second` made it return the most expensive price
  instead of the latest one.

### Fixed

- `GET /cities`, `GET /prices/fargate`, and `GET /usstates` now report the real
  number of items in the `x-total` and `x-total-pages` headers, which were
  always `1`.
- `GET /pipelines/bna` now returns the pagination headers (`x-page`,
  `x-per-page`, `x-total`, `x-total-pages`, `x-next-page`, `x-prev-page`), like
  every other list.
- Every list now fills the `link` header with the URLs of its first, previous,
  next, and last pages, keeping the other query parameters. It was always empty.
- `PATCH /cities/submissions/{submission_id}` works again. It read the
  submission identifier from the query string instead of the path, so every
  request was rejected.
- Updating a submission that does not exist now returns `404 Not Found` instead
  of `500 Internal Server Error`.
- `GET /cities/submissions/{submission_id}` accepts the documented `status`
  filter again. Any query string used to be rejected.
- Updating a submission with an unknown `status` now returns
  `422 Unprocessable Entity` instead of `500 Internal Server Error`.
- `POST /cities` now stores the BNA region of a US city (for example `South` for
  Tennessee). It used to store `United States`. Cities created before this fix
  keep the wrong region.
- Creating a city that already exists now returns the documented `409 Conflict`
  instead of `500 Internal Server Error`.
- Creating a city in a country the BNA does not cover now returns
  `422 Unprocessable Entity` instead of `500 Internal Server Error`.
- Creating a US city in a state that does not exist now returns
  `422 Unprocessable Entity` instead of `500 Internal Server Error`.
- Creating a city submission in a country outside the documented `Country` enum
  now returns `422 Unprocessable Entity` instead of `500 Internal Server Error`.
- Creating a rating for a city that does not exist now returns
  `422 Unprocessable Entity` instead of `500 Internal Server Error`.
- Creating a pipeline whose `state_machine_id` already exists now returns the
  documented `409 Conflict` instead of `500 Internal Server Error`.
- Updating a pipeline that does not exist now returns `404 Not Found` instead of
  `500 Internal Server Error`.
- Creating or updating a pipeline with a `fargate_price_id` that does not exist,
  and updating a submission with a country the BNA does not cover, now return
  `422 Unprocessable Entity` instead of `500 Internal Server Error`.
- A NUL character, which the database cannot store, is now rejected instead of
  returning `500 Internal Server Error`: with `400 Bad Request` in a path, and
  with `422 Unprocessable Entity` in a body string.
- Every list now has a stable order, so items are no longer repeated or skipped
  between pages. The ratings, pipelines, US states, and submissions had none,
  and the sorted lists had no tie-breaker.
- A `500 Internal Server Error` now holds the request path in `source.pointer`
  and a message in `details`, and the cause goes to the logs. It used to hold
  `Unknown source` and empty details, and logged nothing unless the log level
  was `debug`. `GET /reports` and `GET /reports/{year}` return it on a database
  error instead of panicking.
- Updating a pipeline with an unknown `status` or `step` now returns
  `422 Unprocessable Entity` instead of `500 Internal Server Error`.
- Updating a pipeline now applies the documented `start_time`, which was
  silently ignored.
- `GET /cities/{country}/{region}/{name}/ratings` now returns the city with an
  empty `ratings` list for a page past the last one, as documented, instead of
  `404 Not Found`.
- The rating reads (`GET /ratings` and `GET /ratings/{rating_id}`) now return
  the rating's `measure`. Its fields were always `null`.
- `GET /cities/summary/latest` now counts and pages only the cities with a
  rating. The cities without a rating were counted in `x-total` and
  `x-total-pages`, then dropped from the page, so pages could hold fewer items
  than `page_size`, or none.
- `GET /reports` and `GET /reports/{year}` now fill the `coreservices_score`,
  `people`, `retail`, and `transit` columns, which were always empty.
- `POST /ratings` now writes a rating in a single transaction. A failure partway
  used to leave a partial rating behind, listed by `GET /ratings`.
