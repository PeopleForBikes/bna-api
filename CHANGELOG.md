# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- Requests with an invalid path parameter, query string parameter, or body now
  get a JSON `APIErrors` document, as documented in the OpenAPI specification,
  instead of a plain text message. The error `source` names the failing query
  parameter, the request path, the JSON pointer of the failing body field, or
  the `content-type` header.
- Invalid request bodies now return `400 Bad Request`, as documented in the
  OpenAPI specification. A body with missing or mistyped fields previously
  returned `422 Unprocessable Entity`, and a body sent without the
  `application/json` content type returned `415 Unsupported Media Type`.
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
  `GET /cities/summary/latest`: it never had any effect there. It remains on
  `GET /prices/fargate`.
- `sort_by` now only accepts the documented fields, declared as enums in the
  OpenAPI specification: `name`, `country`, `state`, `state_abbrev`, `region`,
  `fips_code`, or `created_at` for the city lists, and `per_second` or
  `created_at` for the Fargate prices. Other values are rejected with
  `400 Bad Request` instead of being silently ignored.
- `GET /reports/{year}` now rejects a year outside 2017 to 2029, the documented
  bounds, with `400 Bad Request`, now documented. It used to return an empty
  report.
- The pipeline `sqs_message` is now a JSON object in requests and responses, as
  the OpenAPI specification now documents. It used to be documented as a string
  and returned as a string holding the JSON object, and a string sent in a
  request was stored double-encoded. A request with an `sqs_message` that is not
  an object is now rejected with `400 Bad Request`. The pipeline state machine
  must send the message as an object. Messages stored double-encoded are
  returned decoded.
- The OpenAPI specification no longer documents `401 Unauthorized` and
  `403 Forbidden` responses. They are produced by API Gateway, not by the
  application, and will be documented separately.
- Each operation in the OpenAPI specification now documents exactly the error
  codes it can return: the lists now document `400 Bad Request` for invalid
  query parameters, and the lists and creations no longer document
  `404 Not Found`.

### Fixed

- `GET /cities`, `GET /prices/fargate`, and `GET /usstates` now report the real
  number of items in the `x-total` and `x-total-pages` headers, which were
  always `1`.
- `GET /pipelines/bna` now returns the pagination headers (`x-page`,
  `x-per-page`, `x-total`, `x-total-pages`, `x-next-page`, `x-prev-page`), like
  every other list.
- `PATCH /cities/submissions/{submission_id}` works again. It read the
  submission identifier from the query string instead of the path, so every
  request was rejected.
- Updating a submission that does not exist now returns `404 Not Found` instead
  of `500 Internal Server Error`.
- `GET /cities/submissions/{submission_id}` accepts the documented `status`
  filter again. Any query string used to be rejected.
- Updating a submission with an unknown `status` now returns `400 Bad Request`
  instead of `500 Internal Server Error`.
- `POST /cities` now stores the BNA region of a US city (for example `South` for
  Tennessee). It used to store `United States`. Cities created before this fix
  keep the wrong region.
- Creating a city that already exists now returns the documented `409 Conflict`
  instead of `500 Internal Server Error`.
- Creating a city in a country the BNA does not cover now returns
  `400 Bad Request` pointing at `/country`, instead of
  `500 Internal Server Error`.
- Creating a US city in a state that does not exist now returns
  `400 Bad Request` pointing at `/state`, instead of
  `500 Internal Server Error`.
- Creating a city submission in a country outside the documented `Country` enum
  now returns `400 Bad Request` pointing at `/country`, instead of
  `500 Internal Server Error`.
- Creating a rating for a city that does not exist now returns `400 Bad Request`
  pointing at `/city_id`, instead of `500 Internal Server Error`.
- Creating a pipeline whose `state_machine_id` already exists now returns the
  documented `409 Conflict` instead of `500 Internal Server Error`.
- Updating a pipeline that does not exist now returns `404 Not Found` instead of
  `500 Internal Server Error`.
- Updating a pipeline with an unknown `status` or `step` now returns
  `400 Bad Request` instead of `500 Internal Server Error`.
- Updating a pipeline now applies the documented `start_time`, which was
  silently ignored.
- `GET /cities/{country}/{region}/{name}/ratings` now returns the city with an
  empty `ratings` list for a page past the last one, as documented, instead of
  `404 Not Found`.
- `GET /reports` and `GET /reports/{year}` now fill the `coreservices_score`,
  `people`, `retail`, and `transit` columns, which were always empty.
