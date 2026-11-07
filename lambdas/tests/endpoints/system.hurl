# Checks the system liveness.
GET {{host}}/health
HTTP 200
[Asserts]
header "content-type" startsWith "text/plain"
body == "OK"
