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

# Queries a US state whose name has a space.
GET {{host}}/usstates/New%20Mexico
HTTP 200
[Asserts]
jsonpath "$.name" == "New Mexico"
jsonpath "$.abbrev" == "NM"

# Queries a US state that does not exist.
GET {{host}}/usstates/Narnia
HTTP 404
[Asserts]
header "content-type" == "application/json"
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].status" == "404"
jsonpath "$.errors[0].source.pointer" == "/usstates/Narnia"

# The state name is case sensitive.
GET {{host}}/usstates/texas
HTTP 404
[Asserts]
jsonpath "$.errors[0].status" == "404"
jsonpath "$.errors[0].source.pointer" == "/usstates/texas"

# Rejects a state name that is not valid UTF-8.
GET {{host}}/usstates/%FF
HTTP 400
[Asserts]
header "content-type" == "application/json"
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].status" == "400"
jsonpath "$.errors[0].source.pointer" == "/usstates/%FF"

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
