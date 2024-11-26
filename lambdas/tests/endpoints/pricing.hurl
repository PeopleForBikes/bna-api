# Queries the first page of the fargate prices.
GET {{host}}/prices/fargate

HTTP 200
[Asserts]
jsonpath "$" count > 0
[Captures]
price_id: jsonpath "$.[0].id"


# Queries a specific fargate price.
GET {{host}}/prices/fargate/{{price_id}}

HTTP 200
