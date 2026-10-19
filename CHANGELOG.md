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
