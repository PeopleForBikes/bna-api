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

# Queries a specific bna run.
GET {{host}}/bnas/{{bna_id}}

HTTP 200

# Queries a specific bna with an invalid id.
GET {{host}}/bnas/1

HTTP 400
[Asserts]
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].source.Parameter" == "bna_id"

# Queries a non-existing bna run
GET {{host}}/bnas/eac1dbfb-2137-44c5-be59-71fc613f2962

HTTP 404
[Asserts]
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].source.Pointer" == "/bnas/eac1dbfb-2137-44c5-be59-71fc613f2962"

# Queries a specific bna run and its associated city.
GET {{host}}/bnas/{{bna_id}}/city

HTTP 200
[Asserts]
jsonpath "$" count > 0

# Queries a non-existing bna run and its associated city.
GET {{host}}/bnas/374386aa-c34a-4cd6-b296-ae420017847e/city

HTTP 404
[Asserts]
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].source.Pointer" == "/bnas/374386aa-c34a-4cd6-b296-ae420017847e/city"
