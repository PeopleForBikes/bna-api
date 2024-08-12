# Queries the first page of the cities.
GET {{host}}/cities

HTTP 200
[Asserts]
jsonpath "$" count > 0
[Captures]
country: jsonpath "$.[0].country" urlEncode
region: jsonpath "$.[0].state"
name: jsonpath "$.[0].name"

# Queries a specific city.
GET {{host}}/cities/{{country}}/{{region}}/{{name}}

HTTP 200

# Queries a non-existing city.
GET {{host}}/cities/{{country}}/{{region}}/unknown_city

HTTP 404
[Asserts]
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].source.Parameter" == "/cities/United%20States/Mississippi/unknown_city?"

# Queries all the BNAs of a specific city.
GET {{host}}/cities/{{country}}/{{region}}/{{name}}/bnas

HTTP 200
[Asserts]
jsonpath "$" count > 0

# Queries all the BNAs of a non-existing city.
GET {{host}}/cities/{{country}}/{{region}}/unknown_city/bnas

HTTP 404
[Asserts]
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].source.Parameter" == "/cities/United%20States/Mississippi/unknown_city/bnas?"


