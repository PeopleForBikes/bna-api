# Queries a the first page of the BNA with a page_size too big.
# Ensures at most MAX_PAGE_SIZE is returned.
GET {{host}}/bnas?page_size=150

HTTP 200
[Asserts]
header "X-Per-Page" == "{{max_page_size}}"
jsonpath "$" count <= {{max_page_size}}

# Queries the BNA with an invalid page number.
GET {{host}}/bnas?page=-5

HTTP 400
[Asserts]
jsonpath "$.errors" count == 1

# Queries the BNA with a valid page number but which does not exist.
GET {{host}}/bnas?page=5000

HTTP 200
[Asserts]
jsonpath "$" count == 0
