# Queries the first page of the ratings.
GET {{host}}/ratings
HTTP 200
[Asserts]
jsonpath "$" count > 0
[Captures]
rating_id: jsonpath "$[0].id"
rating_city_id: jsonpath "$[0].city_id"


# Queries a specific rating.
GET {{host}}/ratings/{{rating_id}}
HTTP 200
[Asserts]
jsonpath "$.id" == "{{rating_id}}"
jsonpath "$.city_id" == "{{rating_city_id}}"

# Queries a specific rating with an invalid id.
GET {{host}}/ratings/1
HTTP 400
[Asserts]
header "content-type" == "application/json"
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].status" == "400"
jsonpath "$.errors[0].source.pointer" == "/ratings/1"

# Queries a non-existing rating.
GET {{host}}/ratings/{{fake_rating_id}}
HTTP 404
[Asserts]
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].source.pointer" == "/ratings/{{fake_rating_id}}"

# Queries a specific rating and its associated city.
GET {{host}}/ratings/{{rating_id}}/city
HTTP 200
[Asserts]
jsonpath "$.rating.id" == "{{rating_id}}"
jsonpath "$.city.id" == "{{rating_city_id}}"

# Queries a non-existing rating and its associated city.
GET {{host}}/ratings/{{fake_rating_id}}/city
HTTP 404
[Asserts]
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].source.pointer" == "/ratings/{{fake_rating_id}}/city"

# Create a city for this run, so the new rating does not alter shared data.
POST {{host}}/cities
Authorization: Bearer {{token}}
content-type: application/json
{
  "country": "{{country_name}}",
  "state": "{{region}}",
  "name": "hurl-{{newUuid}}"
}
HTTP 201
[Captures]
city_id: jsonpath "$.id"
[Asserts]
jsonpath "$.id" exists

# Post a new city rating.
POST {{host}}/ratings
Authorization: Bearer {{token}}
content-type: application/json
{
  "city_id": "{{city_id}}",
  "core_services": {
    "dentists": 0,
    "doctors": 0,
    "grocery": 1.69,
    "hospitals": 5.18,
    "pharmacies": 0,
    "score": 3.24
  },
  "infrastructure": {
    "high_stress_miles": 64.5,
    "low_stress_miles": 9.3
  },
  "opportunity": {
    "employment": 8.26,
    "higher_education": 0,
    "k12_education": 8.31,
    "score": 8.29,
    "technical_vocational_college": 0
  },
  "people": {
    "people": 19.17
  },
  "recreation": {
    "community_centers": 0,
    "parks": 7.13,
    "score": 7.13,
    "trails": 0
  },
  "retail": {
    "retail": 0
  },
  "transit": {
    "transit": 0
  },
  "measure": {
    "buffered_lane": 1.5,
    "lane": 2.5,
    "path": 3.5,
    "sharrow": 4.5,
    "track": 5.5
  },
  "pop_size": 2,
  "population": 123456,
  "version": "24.12",
  "score": 8.93
}
HTTP 201
[Captures]
new_rating_id: jsonpath "$.id"
[Asserts]
jsonpath "$.city_id" == "{{city_id}}"

# Read the new rating back: the stored values match the posted ones.
GET {{host}}/ratings/{{new_rating_id}}
HTTP 200
[Asserts]
jsonpath "$.city_id" == "{{city_id}}"
jsonpath "$.score" == 8.93
jsonpath "$.version" == "24.12"
jsonpath "$.core_services.grocery" == 1.69
jsonpath "$.core_services.score" == 3.24
jsonpath "$.infrastructure.low_stress_miles" == 9.3
jsonpath "$.infrastructure.high_stress_miles" == 64.5
jsonpath "$.opportunity.k12_education" == 8.31
jsonpath "$.opportunity.score" == 8.29
jsonpath "$.people.people" == 19.17
jsonpath "$.recreation.parks" == 7.13
jsonpath "$.recreation.score" == 7.13
jsonpath "$.retail.retail" == 0
jsonpath "$.transit.transit" == 0
jsonpath "$.measure.buffered_lane" == 1.5
jsonpath "$.measure.lane" == 2.5
jsonpath "$.measure.path" == 3.5
jsonpath "$.measure.sharrow" == 4.5
jsonpath "$.measure.track" == 5.5

# The new rating is listed with its city.
GET {{host}}/ratings/{{new_rating_id}}/city
HTTP 200
[Asserts]
jsonpath "$.rating.id" == "{{new_rating_id}}"
jsonpath "$.city.id" == "{{city_id}}"

# Rejects an invalid query string parameter with a JSON error.
GET {{host}}/ratings?page_size=abc
HTTP 400
[Asserts]
header "content-type" == "application/json"
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].status" == "400"
jsonpath "$.errors[0].source.parameter" == "page_size"

# Rejects a body with a field of the wrong type.
POST {{host}}/ratings
Authorization: Bearer {{token}}
content-type: application/json
```
{"city_id": "11111111-2222-3333-4444-555555555555", "score": "high"}
```
HTTP 422
[Asserts]
header "content-type" == "application/json"
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].status" == "422"
jsonpath "$.errors[0].source.pointer" == "/ratings"
jsonpath "$.errors[0].details" contains "score"

# Rejects a body that is not valid JSON.
POST {{host}}/ratings
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
jsonpath "$.errors[0].source.pointer" == "/ratings"

# Rejects a body sent without the JSON content type.
POST {{host}}/ratings
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

# Rejects a rating for a city that does not exist.
POST {{host}}/ratings
Authorization: Bearer {{token}}
content-type: application/json
{
  "city_id": "00000000-0000-4000-8000-000000000000",
  "core_services": {},
  "infrastructure": {},
  "opportunity": {},
  "people": {},
  "recreation": {},
  "retail": {},
  "transit": {},
  "measure": {},
  "pop_size": 2,
  "population": 123456,
  "version": "24.12",
  "score": 8.93
}
HTTP 422
[Asserts]
header "content-type" == "application/json"
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].status" == "422"
jsonpath "$.errors[0].source.pointer" == "/ratings"
jsonpath "$.errors[0].details" contains "city_id"

# Queries the city of a rating with an invalid id.
GET {{host}}/ratings/1/city
HTTP 400
[Asserts]
header "content-type" == "application/json"
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].status" == "400"
jsonpath "$.errors[0].source.pointer" == "/ratings/1/city"
