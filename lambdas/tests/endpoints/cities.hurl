# Queries the first page of the cities.
GET {{host}}/cities

HTTP 200
[Asserts]
jsonpath "$" count > 0
jsonpath "$[0].id" exists
jsonpath "$[0].name" exists


# Queries a specific city.
GET {{host}}/cities/{{country}}/{{region}}/{{name}}

HTTP 200
[Captures]
reference_bna_region: jsonpath "$.region"
[Asserts]
jsonpath "$.country" == "{{country_name}}"
jsonpath "$.state" == "{{region}}"
jsonpath "$.name" == "{{name}}"
jsonpath "$.id" exists

# Queries a non-existing city.
GET {{host}}/cities/{{country}}/{{region}}/unknown_city

HTTP 404
[Asserts]
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].source.pointer" == "/cities/United%20States/{{region}}/unknown_city"

# Queries all the BNAs of a specific city.
GET {{host}}/cities/{{country}}/{{region}}/{{name}}/ratings

HTTP 200
[Captures]
city_id: jsonpath "$.city.id"
[Asserts]
jsonpath "$.city.name" == "{{name}}"
jsonpath "$.ratings" count > 0
jsonpath "$.ratings[0].city_id" == "{{city_id}}"

# Queries all the BNAs of a non-existing city.
GET {{host}}/cities/{{country}}/{{region}}/unknown_city/ratings

HTTP 404
[Asserts]
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].source.pointer" == "/cities/United%20States/{{region}}/unknown_city/ratings"

# Query the top 5 cities for 2025, ordered by descending score.
GET {{host}}/cities/top/2025/5

HTTP 200
[Captures]
score_1: jsonpath "$[1].summary.score"
score_2: jsonpath "$[2].summary.score"
score_3: jsonpath "$[3].summary.score"
score_4: jsonpath "$[4].summary.score"
[Asserts]
jsonpath "$" count == 5
jsonpath "$[0].summary.score" >= {{score_1}}
jsonpath "$[1].summary.score" >= {{score_2}}
jsonpath "$[2].summary.score" >= {{score_3}}
jsonpath "$[3].summary.score" >= {{score_4}}

# Queries the latest city summaries; each summary belongs to its city.
GET {{host}}/cities/summary/latest

HTTP 200
[Captures]
summary_city_id: jsonpath "$[0].city.id"
[Asserts]
jsonpath "$" count > 0
jsonpath "$[0].summary.city_id" == "{{summary_city_id}}"

# Creates a city.
POST {{host}}/cities
Authorization: Bearer {{token}}
content-type: application/json
{
  "country": "{{country_name}}",
  "state": "{{region}}",
  "name": "hurl-{{newUuid}}",
  "latitude": 35.1,
  "longitude": -85.2,
  "residential_speed_limit": 30
}
HTTP 201
[Captures]
new_city_id: jsonpath "$.id"
new_city_name: jsonpath "$.name"
[Asserts]
jsonpath "$.country" == "{{country_name}}"
jsonpath "$.state" == "{{region}}"
jsonpath "$.region" == "{{reference_bna_region}}"
jsonpath "$.created_at" exists

# Reads the new city back.
GET {{host}}/cities/{{country}}/{{region}}/{{new_city_name}}
HTTP 200
[Asserts]
jsonpath "$.id" == "{{new_city_id}}"
jsonpath "$.latitude" == 35.1
jsonpath "$.longitude" == -85.2
jsonpath "$.residential_speed_limit" == 30

# Creating a city that already exists is a conflict.
POST {{host}}/cities
Authorization: Bearer {{token}}
content-type: application/json
{
  "country": "{{country_name}}",
  "state": "{{region}}",
  "name": "{{new_city_name}}"
}
HTTP 409
[Asserts]
header "content-type" == "application/json"
jsonpath "$.errors[0].status" == "409"
jsonpath "$.errors[0].source.pointer" == "/cities"

# Rejects a city in a country the BNA does not cover.
POST {{host}}/cities
Authorization: Bearer {{token}}
content-type: application/json
{
  "country": "Narnia",
  "state": "Narnia",
  "name": "hurl-{{newUuid}}"
}
HTTP 400
[Asserts]
header "content-type" == "application/json"
jsonpath "$.errors[0].status" == "400"
jsonpath "$.errors[0].source.pointer" == "/country"

# Rejects a US city in a state that does not exist.
POST {{host}}/cities
Authorization: Bearer {{token}}
content-type: application/json
{
  "country": "United States",
  "state": "Narnia",
  "name": "hurl-{{newUuid}}"
}
HTTP 400
[Asserts]
header "content-type" == "application/json"
jsonpath "$.errors[0].status" == "400"
jsonpath "$.errors[0].source.pointer" == "/state"

# Rejects an invalid query string parameter with a JSON error.
GET {{host}}/cities?page=abc
HTTP 400
[Asserts]
header "content-type" == "application/json"
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].status" == "400"
jsonpath "$.errors[0].source.parameter" == "page"

# Rejects an invalid path parameter with a JSON error.
GET {{host}}/cities/top/abc/5
HTTP 400
[Asserts]
header "content-type" == "application/json"
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].status" == "400"
jsonpath "$.errors[0].source.pointer" == "/cities/top/abc/5"

# Rejects a body that is not valid JSON.
POST {{host}}/cities
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
jsonpath "$.errors[0].source.pointer" == "/"

# Rejects a body missing a required field.
POST {{host}}/cities
Authorization: Bearer {{token}}
content-type: application/json
```
{"name": "Y"}
```
HTTP 400
[Asserts]
header "content-type" == "application/json"
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].status" == "400"
jsonpath "$.errors[0].source.pointer" == "/country"

# Rejects a body sent without the JSON content type.
POST {{host}}/cities
Authorization: Bearer {{token}}
```
{"name": "Y"}
```
HTTP 400
[Asserts]
header "content-type" == "application/json"
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].status" == "400"
jsonpath "$.errors[0].source.header" == "content-type"

# Counts every city in the pagination headers: with one city per page, there
# are as many pages as cities.
GET {{host}}/cities?page_size=1
HTTP 200
[Captures]
cities_total: header "x-total"
[Asserts]
header "x-total" toInt > 1
header "x-total-pages" == "{{cities_total}}"
