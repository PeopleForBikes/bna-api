# ################
# Ratings
# ################
# Query the first ratings page.
GET {{host}}/ratings

HTTP 200
[Asserts]
header "X-Per-Page" == "{{default_page_size}}"
header "X-page" == "0"

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
header "X-page" == "0"
