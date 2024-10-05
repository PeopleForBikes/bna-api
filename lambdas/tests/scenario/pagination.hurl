# ################
# Ratings
# ################
# Query the first ratings page.
GET {{host}}/ratings

HTTP 200
[Asserts]
header "X-Per-Page" == "{{max_page_size}}"
header "X-page" == "0"

# Queries a the first page of the BNA with a page_size too big.
# Ensures at most MAX_PAGE_SIZE is returned.
GET {{host}}/ratings?page_size=150

HTTP 200
[Asserts]
header "X-Per-Page" == "{{max_page_size}}"
jsonpath "$" count <= {{max_page_size}}

# Queries the BNA with an invalid page number.
# Should return the first page of results.
GET {{host}}/ratings?page=-5

HTTP 200
[Asserts]
header "X-page" == "0"

# Queries the BNA with a valid page number but which does not exist.
# Should return the last page of results.
GET {{host}}/ratings?page=5000

HTTP 200
[Asserts]
jsonpath "$" count == 0

# ################
# Cities
# ################
# Query the first ratings page.
GET {{host}}/cities

HTTP 200
[Asserts]
header "X-Per-Page" == "{{max_page_size}}"
header "X-page" == "0"
