# ###################
# Submission lifecycle
# ###################
#
# Follows a city submission from its creation. Every run submits with its own
# email address, so the scenario can run repeatedly.

# Create a new submission.
POST {{host}}/cities/submissions
Authorization: Bearer {{token}}
content-type: application/json
{
  "city": "santa rosa",
  "country": "{{country_name}}",
  "email": "hurl-{{newUuid}}@example.com",
  "fips_code": "3570670",
  "first_name": "Jane",
  "last_name": "Doe",
  "organization": "Organization LLC",
  "region": "new mexico",
  "occupation": "CTO",
  "consent": true
}
HTTP 201
[Captures]
submission_id: jsonpath "$.id"
submission_email: jsonpath "$.email"
[Asserts]
jsonpath "$.status" == "Pending"

# Read the new submission back.
GET {{host}}/cities/submissions/{{submission_id}}
HTTP 200
[Asserts]
jsonpath "$.id" == {{submission_id}}
jsonpath "$.email" == "{{submission_email}}"
jsonpath "$.city" == "santa rosa"
jsonpath "$.status" == "Pending"

# Approve the submission.
PATCH {{host}}/cities/submissions/{{submission_id}}
Authorization: Bearer {{token}}
content-type: application/json
{
  "status": "Approved"
}
HTTP 200
[Asserts]
jsonpath "$.id" == {{submission_id}}
jsonpath "$.status" == "Approved"
jsonpath "$.email" == "{{submission_email}}"

# Read the approved submission back.
GET {{host}}/cities/submissions/{{submission_id}}
HTTP 200
[Asserts]
jsonpath "$.status" == "Approved"
