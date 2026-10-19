# Queries the first page of the fargate prices.
GET {{host}}/prices/fargate

HTTP 200
[Asserts]
jsonpath "$" count > 0
[Captures]
price_id: jsonpath "$[0].id"
price_per_second: jsonpath "$[0].per_second"


# Queries a specific fargate price; it matches the listed one.
GET {{host}}/prices/fargate/{{price_id}}

HTTP 200
[Asserts]
jsonpath "$.id" == {{price_id}}
jsonpath "$.per_second" == "{{price_per_second}}"
jsonpath "$.created_at" exists
