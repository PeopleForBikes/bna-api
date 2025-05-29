# Queries the first page of the bnas.
GET {{host}}/ratings
HTTP 200
[Asserts]
jsonpath "$" count > 0
[Captures]
rating_id: jsonpath "$.[0].id"


# Queries a specific bna run.
GET {{host}}/ratings/{{rating_id}}
HTTP 200

# Queries a specific bna with an invalid id.
GET {{host}}/ratings/1
HTTP 400

# Queries a non-existing bna run
GET {{host}}/ratings/{{fake_rating_id}}
HTTP 404
[Asserts]
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].source.pointer" == "/ratings/{{fake_rating_id}}"

# Queries a specific bna run and its associated city.
GET {{host}}/ratings/{{rating_id}}/city
HTTP 200
[Asserts]

# Queries a non-existing bna run and its associated city.
GET {{host}}/ratings/{{fake_rating_id}}/city
HTTP 404
[Asserts]
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].source.pointer" == "/ratings/{{fake_rating_id}}/city"

# Post a new city rating
POST {{host}}/ratings
{
  "city_id": "09c049b6-213b-405c-bc4e-178346ff814d",
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
  "measure": {},
  "pop_size": 2,
  "population": 123456,
  "version": "24.12",
  "score": 8.93
}
HTTP 201
