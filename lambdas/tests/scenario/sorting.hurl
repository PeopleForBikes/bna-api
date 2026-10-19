# #######
# Sorting
# #######
#
# Checks that `sort_by` and `order_direction` order the lists. Each list has at
# least three items in the test data.

# Cities by ascending name.
GET {{host}}/cities?sort_by=name&order_direction=asc&page_size=3
HTTP 200
[Captures]
city_name_1: jsonpath "$[1].name"
city_name_2: jsonpath "$[2].name"
[Asserts]
jsonpath "$[0].name" <= "{{city_name_1}}"
jsonpath "$[1].name" <= "{{city_name_2}}"

# Cities by descending name.
GET {{host}}/cities?sort_by=name&order_direction=desc&page_size=3
HTTP 200
[Captures]
city_name_1: jsonpath "$[1].name"
city_name_2: jsonpath "$[2].name"
[Asserts]
jsonpath "$[0].name" >= "{{city_name_1}}"
jsonpath "$[1].name" >= "{{city_name_2}}"

# Latest city summaries by descending city name.
GET {{host}}/cities/summary/latest?sort_by=name&order_direction=desc&page_size=3
HTTP 200
[Captures]
summary_city_name_1: jsonpath "$[1].city.name"
[Asserts]
jsonpath "$[0].city.name" >= "{{summary_city_name_1}}"

# Fargate prices by ascending price.
GET {{host}}/prices/fargate?sort_by=per_second&order_direction=asc
HTTP 200
[Captures]
price_1: jsonpath "$[1].per_second" toFloat
price_2: jsonpath "$[2].per_second" toFloat
[Asserts]
jsonpath "$[0].per_second" toFloat <= {{price_1}}
jsonpath "$[1].per_second" toFloat <= {{price_2}}

# Fargate prices by descending price.
GET {{host}}/prices/fargate?sort_by=per_second&order_direction=desc
HTTP 200
[Captures]
price_1: jsonpath "$[1].per_second" toFloat
[Asserts]
jsonpath "$[0].per_second" toFloat >= {{price_1}}

# Fargate prices from the newest one.
GET {{host}}/prices/fargate?sort_by=created_at&order_direction=desc
HTTP 200
[Captures]
newest_price_id: jsonpath "$[0].id"
newest_price_date: jsonpath "$[0].created_at"
[Asserts]
jsonpath "$[1].created_at" <= "{{newest_price_date}}"

# The latest Fargate price is the newest one.
GET {{host}}/prices/fargate?latest=true
HTTP 200
[Asserts]
jsonpath "$" count == 1
jsonpath "$[0].id" == {{newest_price_id}}
