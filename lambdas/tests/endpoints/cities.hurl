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

# A page of ratings past the last one is empty, but the city is found.
GET {{host}}/cities/{{country}}/{{region}}/{{name}}/ratings?page=999
HTTP 200
[Asserts]
jsonpath "$.city.name" == "{{name}}"
jsonpath "$.ratings" count == 0

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
  "fips_code": "4714000",
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
jsonpath "$.fips_code" == "4714000"
jsonpath "$.latitude" == 35.1
jsonpath "$.longitude" == -85.2
jsonpath "$.residential_speed_limit" == 30

# The latest summaries skip the new city, which has no rating: the newest page
# of one summary is full and belongs to another city.
GET {{host}}/cities/summary/latest?sort_by=created_at&order_direction=desc&page_size=1
HTTP 200
[Asserts]
jsonpath "$" count == 1
jsonpath "$[0].city.id" != "{{new_city_id}}"

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

# Creates a city in a country that needs its name renamed in the API enum, and
# reads it back: every documented country is accepted.
POST {{host}}/cities
Authorization: Bearer {{token}}
content-type: application/json
{
  "country": "The Bahamas",
  "state": "New Providence",
  "name": "hurl-{{newUuid}}"
}
HTTP 201
[Captures]
bahamas_city_name: jsonpath "$.name"
[Asserts]
jsonpath "$.country" == "The Bahamas"

# Reads the new city back.
GET {{host}}/cities/The%20Bahamas/New%20Providence/{{bahamas_city_name}}
HTTP 200
[Asserts]
jsonpath "$.name" == "{{bahamas_city_name}}"

# Creates a city in a country only the API enum used to list.
POST {{host}}/cities
Authorization: Bearer {{token}}
content-type: application/json
{
  "country": "Albania",
  "state": "Tirana",
  "name": "hurl-{{newUuid}}"
}
HTTP 201
[Asserts]
jsonpath "$.country" == "Albania"

# Rejects a city in a country the BNA does not cover.
POST {{host}}/cities
Authorization: Bearer {{token}}
content-type: application/json
{
  "country": "Narnia",
  "state": "Narnia",
  "name": "hurl-{{newUuid}}"
}
HTTP 422
[Asserts]
header "content-type" == "application/json"
jsonpath "$.errors[0].status" == "422"
jsonpath "$.errors[0].source.pointer" == "/cities"
jsonpath "$.errors[0].details" contains "country"

# Rejects a US city in a state that does not exist.
POST {{host}}/cities
Authorization: Bearer {{token}}
content-type: application/json
{
  "country": "United States",
  "state": "Narnia",
  "name": "hurl-{{newUuid}}"
}
HTTP 422
[Asserts]
header "content-type" == "application/json"
jsonpath "$.errors[0].status" == "422"
jsonpath "$.errors[0].source.pointer" == "/cities"
jsonpath "$.errors[0].details" contains "state"

# Rejects a country that does not exist when reading a city.
GET {{host}}/cities/Narnia/Narnia/Cair%20Paravel
HTTP 400
[Asserts]
header "content-type" == "application/json"
jsonpath "$.errors[0].status" == "400"
jsonpath "$.errors[0].source.pointer" == "/cities/Narnia/Narnia/Cair%20Paravel"

# Rejects a country that does not exist when reading the ratings of a city.
GET {{host}}/cities/Narnia/Narnia/Cair%20Paravel/ratings
HTTP 400
[Asserts]
jsonpath "$.errors[0].status" == "400"

# Rejects the top cities for a year before the first BNA.
GET {{host}}/cities/top/2016/5
HTTP 400
[Asserts]
jsonpath "$.errors[0].status" == "400"
jsonpath "$.errors[0].source.pointer" == "/cities/top/2016/5"

# Rejects the top cities for a year after the last supported one.
GET {{host}}/cities/top/2030/5
HTTP 400
[Asserts]
jsonpath "$.errors[0].status" == "400"
jsonpath "$.errors[0].source.pointer" == "/cities/top/2030/5"

# Rejects the top cities for an empty count.
GET {{host}}/cities/top/2025/0
HTTP 400
[Asserts]
jsonpath "$.errors[0].status" == "400"
jsonpath "$.errors[0].source.pointer" == "/cities/top/2025/0"

# Rejects the top cities for a count above the maximum.
GET {{host}}/cities/top/2025/101
HTTP 400
[Asserts]
jsonpath "$.errors[0].status" == "400"
jsonpath "$.errors[0].source.pointer" == "/cities/top/2025/101"

# The top cities of a year without any rating are not found. No rating is
# created in 2029 before that year.
GET {{host}}/cities/top/2029/5
HTTP 404
[Asserts]
header "content-type" == "application/json"
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].status" == "404"
jsonpath "$.errors[0].source.pointer" == "/cities/top/2029/5"

# Accepts the bounds of the top cities parameters.
GET {{host}}/cities/top/2017/100
HTTP 200
[Asserts]
jsonpath "$" count <= 100

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
jsonpath "$.errors[0].source.pointer" == "/cities"

# Rejects a body missing a required field.
POST {{host}}/cities
Authorization: Bearer {{token}}
content-type: application/json
```
{"name": "Y"}
```
HTTP 422
[Asserts]
header "content-type" == "application/json"
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].status" == "422"
jsonpath "$.errors[0].source.pointer" == "/cities"
jsonpath "$.errors[0].details" contains "country"

# Rejects a body sent without the JSON content type.
POST {{host}}/cities
Authorization: Bearer {{token}}
```
{"name": "Y"}
```
HTTP 415
[Asserts]
header "content-type" == "application/json"
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].status" == "415"
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
