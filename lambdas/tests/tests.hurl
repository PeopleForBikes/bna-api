# Queries the first page of the bnas.
GET {{host}}/bnas

HTTP 200
[Asserts]
jsonpath "$" count > 0

# Queries a the first page of the BNA with a page_size too big.
# Ensures at most MAX_PAGE_SIZE is returned.
GET {{host}}/bnas?page_size=150

HTTP 200
[Asserts]
header "X-Per-Page" == "{{max_page_size}}"
jsonpath "$" count <= {{max_page_size}}

# Queries a the first page of the BNA with an invalid page number.
GET {{host}}/bnas?page=-5

HTTP 500

# Queries a the first page of the BNA with a valid page number which does not exist.
GET {{host}}/bnas?page=5000

HTTP 200
[Asserts]
jsonpath "$" count == 0

# Queries a specific bna run.
GET {{host}}/bnas/{{bna_id}}

HTTP 200

# Queries a specific bna run and its associated city.
GET {{host}}/bnas/{{bna_id}}/city

HTTP 200
[Asserts]
jsonpath "$" count > 0

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
