# Queries the first page of US states.
GET {{host}}/usstates

HTTP 200
[Asserts]
jsonpath "$" count > 0


# Queries a specific US state.
GET {{host}}/usstates/Texas

HTTP 200
[Asserts]
jsonpath "$.name" == "Texas"
jsonpath "$.abbrev" == "TX"
jsonpath "$.fipscode" == "48"
jsonpath "$.speed_limit" == 30

# Rejects an invalid query string parameter with a JSON error.
GET {{host}}/usstates?page=abc
HTTP 400
[Asserts]
header "content-type" == "application/json"
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].status" == "400"
jsonpath "$.errors[0].source.parameter" == "page"

# Counts every US state (50 states, District of Columbia, and Puerto Rico) in
# the pagination headers.
GET {{host}}/usstates?page_size=10
HTTP 200
[Asserts]
header "x-total" == "52"
header "x-total-pages" == "6"
