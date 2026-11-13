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
jsonpath "$.errors[0].source.pointer" == "/cities/submissions"
jsonpath "$.errors[0].details" contains "first_name"

# Rejects a NUL character in a body string: the database cannot store it.
POST {{host}}/cities/submissions
Authorization: Bearer {{token}}
content-type: application/json
```
{"first_name": "a\u0000b", "last_name": "Doe", "email": "e@example.com", "country": "Canada", "city": "x", "fips_code": "0", "consent": true}
```
HTTP 422
[Asserts]
header "content-type" == "application/json"
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].status" == "422"
jsonpath "$.errors[0].source.pointer" == "/cities/submissions"
jsonpath "$.errors[0].details" contains "NUL"

# Rejects a body that is not valid JSON.
POST {{host}}/cities/submissions
Authorization: Bearer {{token}}
content-type: application/json
```
not json
```
HTTP 400
[Asserts]
header "content-type" == "application/json"
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].status" == "400"
jsonpath "$.errors[0].source.pointer" == "/cities/submissions"

# Rejects a body sent without the JSON content type.
POST {{host}}/cities/submissions
Authorization: Bearer {{token}}
```
{}
```
HTTP 415
[Asserts]
header "content-type" == "application/json"
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].status" == "415"
jsonpath "$.errors[0].source.header" == "content-type"

# Creates a submission in a country only the API enum used to list.
POST {{host}}/cities/submissions
Authorization: Bearer {{token}}
content-type: application/json
{
  "first_name": "Jane",
  "last_name": "Doe",
  "email": "hurl-{{newUuid}}@example.com",
  "country": "Albania",
  "city": "Tirana",
  "fips_code": "0",
  "consent": true
}
HTTP 201
[Asserts]
jsonpath "$.country" == "Albania"

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
HTTP 422
[Asserts]
header "content-type" == "application/json"
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].status" == "422"
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

# Reads the new submission back: it is still pending.
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
HTTP 422
[Asserts]
header "content-type" == "application/json"
jsonpath "$.errors[0].status" == "422"
jsonpath "$.errors[0].source.pointer" == "/cities/submissions/1"
jsonpath "$.errors[0].details" contains "status"

# Rejects an update with a country the BNA does not cover.
PATCH {{host}}/cities/submissions/1
Authorization: Bearer {{token}}
content-type: application/json
{
  "country": "Narnia"
}
HTTP 422
[Asserts]
header "content-type" == "application/json"
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].status" == "422"
jsonpath "$.errors[0].source.pointer" == "/cities/submissions/1"
jsonpath "$.errors[0].details" contains "country"

# Rejects a body that is not valid JSON.
PATCH {{host}}/cities/submissions/1
Authorization: Bearer {{token}}
content-type: application/json
```
not json
```
HTTP 400
[Asserts]
header "content-type" == "application/json"
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].status" == "400"
jsonpath "$.errors[0].source.pointer" == "/cities/submissions/1"

# Rejects a body sent without the JSON content type.
PATCH {{host}}/cities/submissions/1
Authorization: Bearer {{token}}
```
{}
```
HTTP 415
[Asserts]
header "content-type" == "application/json"
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].status" == "415"
jsonpath "$.errors[0].source.header" == "content-type"

# Rejects a body that is valid JSON but not an object, even an array with one
# element per field, which serde would read field by field.
PATCH {{host}}/cities/submissions/1
Authorization: Bearer {{token}}
content-type: application/json
```
[null, null, null, null, null, null, null, null, null, null, null]
```
HTTP 422
[Asserts]
header "content-type" == "application/json"
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].status" == "422"
jsonpath "$.errors[0].source.pointer" == "/cities/submissions/1"
jsonpath "$.errors[0].details" == "the body must be a JSON object"
