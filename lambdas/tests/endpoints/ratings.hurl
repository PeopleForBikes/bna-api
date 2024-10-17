# Queries the first page of the bnas.
GET {{host}}/ratings

HTTP 200
[Asserts]
jsonpath "$" count > 0
[Captures]
rating_id: jsonpath "$.[0].id"


# Queries a specific bna run.
GET {{host}}/ratings/{{bna_id}}

HTTP 200

# Queries a specific bna with an invalid id.
GET {{host}}/ratings/1

HTTP 400

# Queries a non-existing bna run
GET {{host}}/ratings/{{fake_bna_id}}

HTTP 404
[Asserts]
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].source.pointer" == "/ratings/{{fake_bna_id}}"

# Queries a specific bna run and its associated city.
GET {{host}}/ratings/{{bna_id}}/city

HTTP 200
[Asserts]
jsonpath "$" count > 0

# Queries a non-existing bna run and its associated city.
GET {{host}}/ratings/{{fake_bna_id}}/city

HTTP 404
[Asserts]
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].source.pointer" == "/ratings/{{fake_bna_id}}/city"
