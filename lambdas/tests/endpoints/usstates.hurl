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
jsonpath "$.speed_limit" == "30"

