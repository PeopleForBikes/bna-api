# Fetch all city reports.
GET {{host}}/reports

HTTP 200

# Fetch 2024 city reports.
GET {{host}}/reports/2024

HTTP 200

# Rejects an invalid path parameter with a JSON error.
GET {{host}}/reports/abc
HTTP 400
[Asserts]
header "content-type" == "application/json"
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].status" == "400"
jsonpath "$.errors[0].source.pointer" == "/reports/abc"
