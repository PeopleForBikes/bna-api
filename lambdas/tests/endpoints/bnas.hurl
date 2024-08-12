# Queries the first page of the bnas.
GET {{host}}/bnas

HTTP 200
[Asserts]
jsonpath "$" count > 0
[Captures]
bna_id: jsonpath "$.[0].bna_id"


# Queries a specific bna run.
GET {{host}}/bnas/{{bna_id}}

HTTP 200

# Queries a specific bna with an invalid id.
GET {{host}}/bnas/1

HTTP 400
[Asserts]
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].source.Pointer" == "bna_id"

# Queries a non-existing bna run
GET {{host}}/bnas/{{fake_bna_id}}

HTTP 404
[Asserts]
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].source.Parameter" == "/bnas/{{fake_bna_id}}?"

# Queries a specific bna run and its associated city.
GET {{host}}/bnas/{{bna_id}}/city

HTTP 200
[Asserts]
jsonpath "$" count > 0

# Queries a non-existing bna run and its associated city.
GET {{host}}/bnas/{{fake_bna_id}}/city

HTTP 404
[Asserts]
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].source.Parameter" == "/bnas/{{fake_bna_id}}/city?"
