# Queries the first page of the cities.
GET {{host}}/cities

HTTP 200
[Asserts]
jsonpath "$" count > 0


# Queries a specific city.
GET {{host}}/cities/{{country}}/{{region}}/{{name}}

HTTP 200
[Captures]
country: jsonpath "$.country" urlEncode
region: jsonpath "$.state"
name: jsonpath "$.name"

# Queries a non-existing city.
GET {{host}}/cities/{{country}}/{{region}}/unknown_city

HTTP 404
[Asserts]
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].source.pointer" == "/cities/United%20States/{{region}}/unknown_city"

# Queries all the BNAs of a specific city.
GET {{host}}/cities/{{country}}/{{region}}/{{name}}/ratings

HTTP 200
[Asserts]
jsonpath "$.ratings" count > 0

# Queries all the BNAs of a non-existing city.
GET {{host}}/cities/{{country}}/{{region}}/unknown_city/ratings

HTTP 404
[Asserts]
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].source.pointer" == "/cities/United%20States/{{region}}/unknown_city/ratings"

# Query the top 10 cities for 2024
GET {{host}}/cities/top/2024/10

HTTP 200
[Asserts]
jsonpath "$" count <= 10
