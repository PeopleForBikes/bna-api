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

# Queries the first page of the ratings with a page_size too big.
# Ensures at most MAX_PAGE_SIZE is returned.
GET {{host}}/ratings?page_size=150

HTTP 200
[Asserts]
header "X-Per-Page" == "{{max_page_size}}"
jsonpath "$" count <= {{max_page_size}}

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
