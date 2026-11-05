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
HTTP 400
[Asserts]
header "content-type" == "application/json"
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].status" == "400"
jsonpath "$.errors[0].source.pointer" == "/cities/submissions"
jsonpath "$.errors[0].details" contains "first_name"

# Rejects a submission in a country the BNA does not cover.
POST {{host}}/cities/submissions
Authorization: Bearer {{token}}
content-type: application/json
{
  "first_name": "Jane",
  "last_name": "Doe",
  "email": "jane.doe@example.com",
  "country": "Narnia",
  "city": "Cair Paravel",
  "fips_code": "0",
  "consent": true
}
HTTP 400
[Asserts]
header "content-type" == "application/json"
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].status" == "400"
jsonpath "$.errors[0].source.pointer" == "/cities/submissions"
jsonpath "$.errors[0].details" contains "country"

# A new submission is always pending, whatever status the body sends.
POST {{host}}/cities/submissions
Authorization: Bearer {{token}}
content-type: application/json
{
  "first_name": "Jane",
  "last_name": "Doe",
  "email": "hurl-{{newUuid}}@example.com",
  "country": "{{country_name}}",
  "city": "Santa Rosa",
  "region": "New Mexico",
  "fips_code": "3570670",
  "consent": true,
  "status": "Approved"
}
HTTP 201
[Captures]
approved_submission_id: jsonpath "$.id"
[Asserts]
jsonpath "$.status" == "Pending"

GET {{host}}/cities/submissions/{{approved_submission_id}}
HTTP 200
[Asserts]
jsonpath "$.status" == "Pending"

# Updating a submission that does not exist is not found.
PATCH {{host}}/cities/submissions/999999
Authorization: Bearer {{token}}
content-type: application/json
{
  "status": "Approved"
}
HTTP 404
[Asserts]
header "content-type" == "application/json"
jsonpath "$.errors[0].status" == "404"
jsonpath "$.errors[0].source.pointer" == "/cities/submissions/999999"

# Updating a submission that does not exist is not found, even without changes.
PATCH {{host}}/cities/submissions/999999
Authorization: Bearer {{token}}
content-type: application/json
{}
HTTP 404
[Asserts]
jsonpath "$.errors[0].status" == "404"

# Reads a submission filtered by its own status.
GET {{host}}/cities/submissions/1?status=Pending
HTTP 200
[Asserts]
jsonpath "$.id" == 1
jsonpath "$.status" == "Pending"

# Filtering a submission by another status finds nothing.
GET {{host}}/cities/submissions/1?status=Approved
HTTP 404
[Asserts]
jsonpath "$.errors[0].status" == "404"

# Lists only the submissions with the requested status.
GET {{host}}/cities/submissions?status=Rejected
HTTP 200
[Asserts]
jsonpath "$" count > 0
jsonpath "$[?(@.status != 'Rejected')]" not exists

# Rejects an unknown status when listing submissions.
GET {{host}}/cities/submissions?status=Bogus
HTTP 400
[Asserts]
jsonpath "$.errors[0].source.parameter" == "status"

# Rejects an unknown status when reading a submission.
GET {{host}}/cities/submissions/1?status=Bogus
HTTP 400
[Asserts]
jsonpath "$.errors[0].source.parameter" == "status"

# Rejects an unknown status when updating a submission.
PATCH {{host}}/cities/submissions/1
Authorization: Bearer {{token}}
content-type: application/json
{
  "status": "Bogus"
}
HTTP 400
[Asserts]
header "content-type" == "application/json"
jsonpath "$.errors[0].status" == "400"
jsonpath "$.errors[0].source.pointer" == "/cities/submissions/1"
jsonpath "$.errors[0].details" contains "status"
