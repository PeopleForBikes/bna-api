# Query the first page of the bnas.
GET {{host}}/bnas
HTTP 200

# Query a specific bna run.
GET {{host}}/bnas/{{bna_id}}
HTTP 200

# Query a specific bna run and its associated city.
GET {{host}}/bnas/{{bna_id}}/city
HTTP 200

# Query the first page of the cities.
GET {{host}}/cities
HTTP 200

# Query a specific city.
GET {{host}}/cities/{{country}}/{{region}}/{{name}}
HTTP 200

# Query all the BNAs of a specific city.
GET {{host}}/cities/{{country}}/{{region}}/{{name}}/bnas
HTTP 200

# Create a new submission.
POST {{host}}/cities/submissions
content-type: application/json

{
  "city": "santa rosa",
  "country": "United States",
  "email": "jane.dpe@orgllc.com",
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

# Query all the submissions.
GET {{host}}/cities/submissions
HTTP 200

# Query all the pending submissions.
GET {{host}}/cities/submissions?status=Pending
HTTP 200

# Query a specific submission.
GET {{host}}/cities/submissions/{{submission_id}}
HTTP 200
