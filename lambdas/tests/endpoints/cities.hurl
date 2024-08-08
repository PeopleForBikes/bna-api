# Queries the first page of the cities.
GET {{host}}/cities

HTTP 200
[Asserts]
jsonpath "$" count > 0

# Queries a specific city.
GET {{host}}/cities/{{city_id}}

HTTP 200

# Queries a non-existing city.
GET {{host}}/cities/123456789

HTTP 404
[Asserts]
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].source.Pointer" == "/cities/123456789"

# Queries all the BNAs of a specific city.
GET {{host}}/cities/{{city_id}}/bnas

HTTP 200
[Asserts]
jsonpath "$" count > 0

# Queries all the BNAs of a non-existing city.
GET {{host}}/cities/0/bnas

HTTP 404
[Asserts]
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].source.Pointer" == "/cities/0/bnas"


