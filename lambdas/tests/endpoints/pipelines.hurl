# BNA pipelines.

# Lists the pipelines with the pagination headers.
GET {{host}}/pipelines/bna?page_size=1
HTTP 200
[Captures]
pipelines_total: header "x-total"
[Asserts]
jsonpath "$" count == 1
header "x-per-page" == "1"
header "x-total" toInt >= 3
header "x-total-pages" == "{{pipelines_total}}"
header "link" exists

# Rejects an invalid query string parameter with a JSON error.
GET {{host}}/pipelines/bna?page=abc
HTTP 400
[Asserts]
header "content-type" == "application/json"
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].status" == "400"
jsonpath "$.errors[0].source.parameter" == "page"

# Rejects an invalid path parameter with a JSON error.
GET {{host}}/pipelines/bna/notauuid
HTTP 400
[Asserts]
header "content-type" == "application/json"
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].status" == "400"
jsonpath "$.errors[0].source.pointer" == "/pipelines/bna/notauuid"

# Rejects a body missing a required field.
POST {{host}}/pipelines/bna
Authorization: Bearer {{token}}
content-type: application/json
```
{}
```
HTTP 400
[Asserts]
header "content-type" == "application/json"
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].status" == "400"
jsonpath "$.errors[0].source.pointer" == "/state_machine_id"
