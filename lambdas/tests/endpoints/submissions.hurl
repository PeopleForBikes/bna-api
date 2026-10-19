# City submissions.

# Rejects an invalid query string parameter with a JSON error.
GET {{host}}/cities/submissions?page=abc
HTTP 400
[Asserts]
header "content-type" == "application/json"
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].status" == "400"
jsonpath "$.errors[0].source.parameter" == "page"

# Rejects an invalid path parameter with a JSON error.
GET {{host}}/cities/submissions/abc
HTTP 400
[Asserts]
header "content-type" == "application/json"
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].status" == "400"
jsonpath "$.errors[0].source.pointer" == "/cities/submissions/abc"

# Rejects a body missing a required field.
POST {{host}}/cities/submissions
Authorization: Bearer {{token}}
content-type: application/json
```
{"city": "x"}
```
HTTP 422
[Asserts]
header "content-type" == "application/json"
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].status" == "422"
jsonpath "$.errors[0].source.pointer" == "/first_name"
