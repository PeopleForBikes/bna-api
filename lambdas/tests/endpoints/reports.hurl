# Fetch all city reports, as CSV with a header line.
GET {{host}}/reports
HTTP 200
[Asserts]
header "content-type" == "text/csv"
body startsWith "id,score,version,low_stress_miles,high_stress_miles,community_centers,parks,recreation_trails,recreation_score,employment,higher_education,k12_education,opportunity_score,technical_vocational_college,dentists,doctors,grocery,hospitals,pharmacies,coreservices_score,social_services,people,retail,transit\n"

# Fetch 2024 city reports, as CSV with a header line.
GET {{host}}/reports/2024
HTTP 200
[Asserts]
header "content-type" == "text/csv"
body startsWith "id,score,version,low_stress_miles,high_stress_miles,community_centers,parks,recreation_trails,recreation_score,employment,higher_education,k12_education,opportunity_score,technical_vocational_college,dentists,doctors,grocery,hospitals,pharmacies,coreservices_score,social_services,people,retail,transit\n"

# Rejects a year before the first BNA.
GET {{host}}/reports/2016
HTTP 400
[Asserts]
header "content-type" == "application/json"
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].status" == "400"
jsonpath "$.errors[0].source.pointer" == "/reports/2016"

# Rejects a year after the last supported one.
GET {{host}}/reports/2030
HTTP 400
[Asserts]
header "content-type" == "application/json"
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].status" == "400"
jsonpath "$.errors[0].source.pointer" == "/reports/2030"

# Accepts the bounds of the years.
GET {{host}}/reports/2017
HTTP 200
[Asserts]
header "content-type" contains "text/csv"

# Accepts the last year of the bounds.
GET {{host}}/reports/2029
HTTP 200
[Asserts]
header "content-type" contains "text/csv"

# Rejects an invalid path parameter with a JSON error.
GET {{host}}/reports/abc
HTTP 400
[Asserts]
header "content-type" == "application/json"
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].status" == "400"
jsonpath "$.errors[0].source.pointer" == "/reports/abc"

# Create a city and rate it, so the report of the current year has a known row.
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
report_city_id: jsonpath "$.id"
report_year: jsonpath "$.created_at" regex "^(\\d{4})-"
[Asserts]
jsonpath "$.id" exists

POST {{host}}/ratings
Authorization: Bearer {{token}}
content-type: application/json
{
  "city_id": "{{report_city_id}}",
  "core_services": {
    "dentists": 1.1,
    "doctors": 1.2,
    "grocery": 1.3,
    "hospitals": 1.4,
    "pharmacies": 1.5,
    "score": 1.6,
    "social_services": 1.7
  },
  "infrastructure": {
    "high_stress_miles": 2.2,
    "low_stress_miles": 2.1
  },
  "opportunity": {
    "employment": 3.1,
    "higher_education": 3.2,
    "k12_education": 3.3,
    "score": 3.4,
    "technical_vocational_college": 3.5
  },
  "people": {
    "people": 4.1
  },
  "recreation": {
    "community_centers": 5.1,
    "parks": 5.2,
    "score": 5.4,
    "trails": 5.3
  },
  "retail": {
    "retail": 6.1
  },
  "transit": {
    "transit": 7.1
  },
  "measure": {},
  "pop_size": 2,
  "population": 123456,
  "version": "24.12",
  "score": 8.1
}
HTTP 201
[Asserts]
jsonpath "$.id" exists

# The report of the current year has every value of the rating.
GET {{host}}/reports/{{report_year}}
HTTP 200
[Asserts]
body contains "\n{{report_city_id}},8.1,24.12,2.1,2.2,5.1,5.2,5.3,5.4,3.1,3.2,3.3,3.4,3.5,1.1,1.2,1.3,1.4,1.5,1.6,1.7,4.1,6.1,7.1\n"
