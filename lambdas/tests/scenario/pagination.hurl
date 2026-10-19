# ################
# Ratings
# ################
# Query the first ratings page: pages start at 1.
GET {{host}}/ratings

HTTP 200
[Captures]
first_rating_id: jsonpath "$[0].id"
[Asserts]
header "X-Per-Page" == "{{default_page_size}}"
header "X-page" == "1"
header "x-prev-page" == "1"
header "x-next-page" == "2"

# Page 1 is the default page.
GET {{host}}/ratings?page=1

HTTP 200
[Asserts]
header "X-page" == "1"
jsonpath "$[0].id" == "{{first_rating_id}}"

# Page 2 starts after page 1.
GET {{host}}/ratings?page=2&page_size=1

HTTP 200
[Asserts]
header "X-page" == "2"
header "x-prev-page" == "1"
jsonpath "$" count == 1
jsonpath "$[0].id" != "{{first_rating_id}}"

# Page 0 does not exist.
GET {{host}}/ratings?page=0

HTTP 400
[Asserts]
jsonpath "$.errors[0].source.parameter" == "page"

# The last allowed page is 65536.
GET {{host}}/ratings?page=65536

HTTP 200

# Pages past 65536 are rejected.
GET {{host}}/ratings?page=65537

HTTP 400
[Asserts]
jsonpath "$.errors[0].source.parameter" == "page"

# Queries the first page of the ratings with the largest page_size.
GET {{host}}/ratings?page_size={{max_page_size}}

HTTP 200
[Asserts]
header "X-Per-Page" == "{{max_page_size}}"
jsonpath "$" count <= {{max_page_size}}

# Rejects a page_size above the maximum.
GET {{host}}/ratings?page_size=101

HTTP 400
[Asserts]
jsonpath "$.errors[0].source.parameter" == "page_size"

# Rejects an empty page_size.
GET {{host}}/ratings?page_size=0

HTTP 400
[Asserts]
jsonpath "$.errors[0].source.parameter" == "page_size"

# Queries the ratings with a negative page number.
# Should be rejected as a bad request.
GET {{host}}/ratings?page=-5

HTTP 400

# Queries the ratings with a page number past the last page.
# Should return an empty list.
GET {{host}}/ratings?page=5000

HTTP 200
[Asserts]
jsonpath "$" count == 0

# ################
# Cities
# ################
# Query the first cities page.
GET {{host}}/cities

HTTP 200
[Asserts]
header "X-Per-Page" == "{{default_page_size}}"
header "X-page" == "1"

# ################
# Every list
# ################
# Each list has at least 3 items in the test data, so page 2 of size 1 has a
# previous and a next page.
# Second page of the cities.
GET {{host}}/cities?page=2&page_size=1

HTTP 200
[Captures]
total: header "x-total"
[Asserts]
jsonpath "$" count == 1
header "x-page" == "2"
header "x-per-page" == "1"
header "x-prev-page" == "1"
header "x-next-page" == "3"
header "x-total-pages" == "{{total}}"

# Second page of the city submissions.
GET {{host}}/cities/submissions?page=2&page_size=1

HTTP 200
[Captures]
total: header "x-total"
[Asserts]
jsonpath "$" count == 1
header "x-page" == "2"
header "x-per-page" == "1"
header "x-prev-page" == "1"
header "x-next-page" == "3"
header "x-total-pages" == "{{total}}"

# Second page of the latest city summaries.
GET {{host}}/cities/summary/latest?page=2&page_size=1

HTTP 200
[Captures]
total: header "x-total"
[Asserts]
jsonpath "$" count == 1
header "x-page" == "2"
header "x-per-page" == "1"
header "x-prev-page" == "1"
header "x-next-page" == "3"
header "x-total-pages" == "{{total}}"

# Second page of the ratings of a city.
GET {{host}}/cities/{{country}}/{{region}}/{{name}}/ratings?page=2&page_size=1

HTTP 200
[Captures]
total: header "x-total"
[Asserts]
jsonpath "$.ratings" count == 1
header "x-page" == "2"
header "x-per-page" == "1"
header "x-prev-page" == "1"
header "x-next-page" == "3"
header "x-total-pages" == "{{total}}"

# Second page of the BNA pipelines.
GET {{host}}/pipelines/bna?page=2&page_size=1

HTTP 200
[Captures]
total: header "x-total"
[Asserts]
jsonpath "$" count == 1
header "x-page" == "2"
header "x-per-page" == "1"
header "x-prev-page" == "1"
header "x-next-page" == "3"
header "x-total-pages" == "{{total}}"

# Second page of the Fargate prices.
GET {{host}}/prices/fargate?page=2&page_size=1

HTTP 200
[Captures]
total: header "x-total"
[Asserts]
jsonpath "$" count == 1
header "x-page" == "2"
header "x-per-page" == "1"
header "x-prev-page" == "1"
header "x-next-page" == "3"
header "x-total-pages" == "{{total}}"

# Second page of the ratings.
GET {{host}}/ratings?page=2&page_size=1

HTTP 200
[Captures]
total: header "x-total"
[Asserts]
jsonpath "$" count == 1
header "x-page" == "2"
header "x-per-page" == "1"
header "x-prev-page" == "1"
header "x-next-page" == "3"
header "x-total-pages" == "{{total}}"

# Second page of the US states.
GET {{host}}/usstates?page=2&page_size=1

HTTP 200
[Captures]
total: header "x-total"
[Asserts]
jsonpath "$" count == 1
header "x-page" == "2"
header "x-per-page" == "1"
header "x-prev-page" == "1"
header "x-next-page" == "3"
header "x-total-pages" == "{{total}}"
