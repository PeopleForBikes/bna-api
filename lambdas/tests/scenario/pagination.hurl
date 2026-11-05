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
# Should return an empty list, navigating like the last page.
GET {{host}}/ratings?page=65536&page_size=1

HTTP 200
[Captures]
last_page: header "x-total-pages"
[Asserts]
jsonpath "$" count == 0
header "x-page" == "65536"
header "x-next-page" == "{{last_page}}"
header "link" contains "<{{host}}/ratings?page_size=1&page={{last_page}}>; rel=\"next\""

# The first page has no page before it: its previous page is itself.
GET {{host}}/ratings?page_size=1

HTTP 200
[Asserts]
header "x-prev-page" == "1"
header "link" startsWith "<{{host}}/ratings?page_size=1&page=1>; rel=\"first\", <{{host}}/ratings?page_size=1&page=1>; rel=\"prev\""

# The links keep the other query parameters, and omit the default page size.
GET {{host}}/cities/submissions?status=Pending&page=2

HTTP 200
[Asserts]
header "link" startsWith "<{{host}}/cities/submissions?status=Pending&page=1>; rel=\"first\""

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
header "link" == "<{{host}}/cities?page_size=1&page=1>; rel=\"first\", <{{host}}/cities?page_size=1&page=1>; rel=\"prev\", <{{host}}/cities?page_size=1&page=3>; rel=\"next\", <{{host}}/cities?page_size=1&page={{total}}>; rel=\"last\""

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
header "link" == "<{{host}}/cities/submissions?page_size=1&page=1>; rel=\"first\", <{{host}}/cities/submissions?page_size=1&page=1>; rel=\"prev\", <{{host}}/cities/submissions?page_size=1&page=3>; rel=\"next\", <{{host}}/cities/submissions?page_size=1&page={{total}}>; rel=\"last\""

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
header "link" == "<{{host}}/cities/summary/latest?page_size=1&page=1>; rel=\"first\", <{{host}}/cities/summary/latest?page_size=1&page=1>; rel=\"prev\", <{{host}}/cities/summary/latest?page_size=1&page=3>; rel=\"next\", <{{host}}/cities/summary/latest?page_size=1&page={{total}}>; rel=\"last\""

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
header "link" == "<{{host}}/cities/{{country}}/{{region}}/{{name}}/ratings?page_size=1&page=1>; rel=\"first\", <{{host}}/cities/{{country}}/{{region}}/{{name}}/ratings?page_size=1&page=1>; rel=\"prev\", <{{host}}/cities/{{country}}/{{region}}/{{name}}/ratings?page_size=1&page=3>; rel=\"next\", <{{host}}/cities/{{country}}/{{region}}/{{name}}/ratings?page_size=1&page={{total}}>; rel=\"last\""

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
header "link" == "<{{host}}/pipelines/bna?page_size=1&page=1>; rel=\"first\", <{{host}}/pipelines/bna?page_size=1&page=1>; rel=\"prev\", <{{host}}/pipelines/bna?page_size=1&page=3>; rel=\"next\", <{{host}}/pipelines/bna?page_size=1&page={{total}}>; rel=\"last\""

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
header "link" == "<{{host}}/prices/fargate?page_size=1&page=1>; rel=\"first\", <{{host}}/prices/fargate?page_size=1&page=1>; rel=\"prev\", <{{host}}/prices/fargate?page_size=1&page=3>; rel=\"next\", <{{host}}/prices/fargate?page_size=1&page={{total}}>; rel=\"last\""

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
header "link" == "<{{host}}/ratings?page_size=1&page=1>; rel=\"first\", <{{host}}/ratings?page_size=1&page=1>; rel=\"prev\", <{{host}}/ratings?page_size=1&page=3>; rel=\"next\", <{{host}}/ratings?page_size=1&page={{total}}>; rel=\"last\""

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
header "link" == "<{{host}}/usstates?page_size=1&page=1>; rel=\"first\", <{{host}}/usstates?page_size=1&page=1>; rel=\"prev\", <{{host}}/usstates?page_size=1&page=3>; rel=\"next\", <{{host}}/usstates?page_size=1&page={{total}}>; rel=\"last\""
