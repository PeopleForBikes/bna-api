# Queries the first page of the cities.
GET {{host}}/cities

HTTP 200
[Asserts]
jsonpath "$" count > 0

# Queries a specific city.
GET {{host}}/cities/{{city_id}}

HTTP 200

# Queries all the BNAs of a specific city.
GET {{host}}/cities/{{city_id}}/bnas

HTTP 200
[Asserts]
jsonpath "$" count > 0
