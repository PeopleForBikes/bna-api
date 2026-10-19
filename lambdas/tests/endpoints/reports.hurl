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

# Rejects an invalid path parameter with a JSON error.
GET {{host}}/reports/abc
HTTP 400
[Asserts]
header "content-type" == "application/json"
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].status" == "400"
jsonpath "$.errors[0].source.pointer" == "/reports/abc"
