# Query the first page of the bnas.
GET {{host}}/ratings
HTTP 200

[Captures]
rating_id: jsonpath "$.[0].id"

# Query a specific bna run.
GET {{host}}/ratings/{{rating_id}}
HTTP 200

# Query a specific bna run and its associated city.
GET {{host}}/ratings/{{rating_id}}/city
HTTP 200

# Query the first page of the cities.
GET {{host}}/cities
HTTP 200

# Query a specific city.
GET {{host}}/cities/{{country}}/{{region}}/{{name}}
HTTP 200

# Query all the BNAs of a specific city.
GET {{host}}/cities/{{country}}/{{region}}/{{name}}/ratings
HTTP 200

# Query the top 10 cities for 2024
GET {{host}}/cities/top/2024/10

HTTP 200
[Asserts]
jsonpath "$" count <= 10

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

# Query Fargate prices.
GET {{host}}/prices/fargate

HTTP 200
[Asserts]
jsonpath "$" count >= 1
[Captures]
fargate_price_id: jsonpath "$.[0].id"

# Query a specific Fargate price.
GET {{host}}/prices/fargate/{{fargate_price_id}}

HTTP 200
[Asserts]
jsonpath "$.id" == {{fargate_price_id}}

# Query the latest Fargate price.
GET {{host}}/prices/fargate?latest=true

HTTP 200
[Asserts]
jsonpath "$" count == 1

# Fetch all city reports.
GET {{host}}/reports

HTTP 200

# Fetch 2024 city reports.
GET {{host}}/reports/2024

HTTP 200
