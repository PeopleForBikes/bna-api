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

### Fixed

- `GET /cities` and `GET /usstates` now report the real number of items in the
  `x-total` and `x-total-pages` headers, which were always `1`.
- `GET /pipelines/bna` now returns the pagination headers (`x-page`,
  `x-per-page`, `x-total`, `x-total-pages`, `x-next-page`, `x-prev-page`), like
  every other list.
