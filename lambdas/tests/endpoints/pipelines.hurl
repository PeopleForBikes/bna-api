# BNA pipelines.

# Lists the pipelines with the pagination headers.
GET {{host}}/pipelines/bna?page_size=1
HTTP 200
[Captures]
pipelines_total: header "x-total"
[Asserts]
jsonpath "$" count == 1
header "x-per-page" == "1"
header "x-total" toInt >= 3
header "x-total-pages" == "{{pipelines_total}}"
header "link" exists

# Rejects an invalid query string parameter with a JSON error.
GET {{host}}/pipelines/bna?page=abc
HTTP 400
[Asserts]
header "content-type" == "application/json"
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].status" == "400"
jsonpath "$.errors[0].source.parameter" == "page"

# Rejects an invalid path parameter with a JSON error.
GET {{host}}/pipelines/bna/notauuid
HTTP 400
[Asserts]
header "content-type" == "application/json"
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].status" == "400"
jsonpath "$.errors[0].source.pointer" == "/pipelines/bna/notauuid"

# Rejects a body missing a required field.
POST {{host}}/pipelines/bna
Authorization: Bearer {{token}}
content-type: application/json
```
{}
```
HTTP 400
[Asserts]
header "content-type" == "application/json"
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].status" == "400"
jsonpath "$.errors[0].source.pointer" == "/state_machine_id"

# Reading a pipeline that does not exist is not found.
GET {{host}}/pipelines/bna/00000000-0000-4000-8000-000000000000
HTTP 404
[Asserts]
header "content-type" == "application/json"
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].status" == "404"
jsonpath "$.errors[0].source.pointer" == "/pipelines/bna/00000000-0000-4000-8000-000000000000"

# Creates a pipeline for this run. A new pipeline starts pending, at the setup
# step.
POST {{host}}/pipelines/bna
Authorization: Bearer {{token}}
content-type: application/json
{
  "cost": "1.25",
  "fargate_price_id": 1,
  "fargate_task_arn": "arn:aws:ecs:us-west-2:123456789012:task/bna/hurl",
  "s3_bucket": "united states/tennessee/hurl/25.06",
  "sqs_message": {
    "city": "hurl",
    "country": "United States",
    "fips_code": "4714000",
    "region": "Tennessee"
  },
  "start_time": "2025-06-20T14:11:32Z",
  "state_machine_id": "{{newUuid}}"
}
HTTP 201
[Captures]
pipeline_id: jsonpath "$.state_machine_id"
[Asserts]
jsonpath "$.status" == "Pending"
jsonpath "$.step" == "Setup"

# Creating a pipeline that already exists is a conflict.
POST {{host}}/pipelines/bna
Authorization: Bearer {{token}}
content-type: application/json
{
  "state_machine_id": "{{pipeline_id}}"
}
HTTP 409
[Asserts]
header "content-type" == "application/json"
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].status" == "409"
jsonpath "$.errors[0].source.pointer" == "/pipelines/bna"

# Reads the new pipeline back.
GET {{host}}/pipelines/bna/{{pipeline_id}}
HTTP 200
[Asserts]
jsonpath "$.state_machine_id" == "{{pipeline_id}}"
jsonpath "$.cost" == "1.25"
jsonpath "$.fargate_price_id" == 1
jsonpath "$.fargate_task_arn" == "arn:aws:ecs:us-west-2:123456789012:task/bna/hurl"
jsonpath "$.s3_bucket" == "united states/tennessee/hurl/25.06"
jsonpath "$.sqs_message.city" == "hurl"
jsonpath "$.sqs_message.fips_code" == "4714000"
jsonpath "$.start_time" == "2025-06-20T14:11:32Z"
jsonpath "$.end_time" == null
jsonpath "$.status" == "Pending"
jsonpath "$.step" == "Setup"

# Updates the pipeline.
PATCH {{host}}/pipelines/bna/{{pipeline_id}}
Authorization: Bearer {{token}}
content-type: application/json
{
  "cost": "0.63749",
  "end_time": "2025-06-20T14:16:11Z",
  "fargate_task_arn": "arn:aws:ecs:us-west-2:123456789012:task/bna/hurl-2",
  "s3_bucket": "united states/tennessee/hurl/25.07",
  "status": "Completed",
  "step": "Cleanup"
}
HTTP 200
[Asserts]
jsonpath "$.state_machine_id" == "{{pipeline_id}}"
jsonpath "$.status" == "Completed"

# Reads the updated pipeline back: the update changed only the sent fields.
GET {{host}}/pipelines/bna/{{pipeline_id}}
HTTP 200
[Asserts]
jsonpath "$.cost" == "0.63749"
jsonpath "$.end_time" == "2025-06-20T14:16:11Z"
jsonpath "$.fargate_price_id" == 1
jsonpath "$.fargate_task_arn" == "arn:aws:ecs:us-west-2:123456789012:task/bna/hurl-2"
jsonpath "$.s3_bucket" == "united states/tennessee/hurl/25.07"
jsonpath "$.sqs_message.city" == "hurl"
jsonpath "$.start_time" == "2025-06-20T14:11:32Z"
jsonpath "$.status" == "Completed"
jsonpath "$.step" == "Cleanup"

# Rejects an update body with a field of the wrong type.
PATCH {{host}}/pipelines/bna/{{pipeline_id}}
Authorization: Bearer {{token}}
content-type: application/json
```
{"fargate_price_id": "one"}
```
HTTP 400
[Asserts]
header "content-type" == "application/json"
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].status" == "400"
jsonpath "$.errors[0].source.pointer" == "/fargate_price_id"

# Rejects an SQS message that is not a JSON object.
POST {{host}}/pipelines/bna
Authorization: Bearer {{token}}
content-type: application/json
{
  "sqs_message": "{\"city\": \"hurl\"}",
  "state_machine_id": "{{newUuid}}"
}
HTTP 400
[Asserts]
header "content-type" == "application/json"
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].status" == "400"
jsonpath "$.errors[0].source.pointer" == "/sqs_message"

# Updating a pipeline that does not exist is not found.
PATCH {{host}}/pipelines/bna/00000000-0000-4000-8000-000000000000
Authorization: Bearer {{token}}
content-type: application/json
{
  "status": "Processing"
}
HTTP 404
[Asserts]
header "content-type" == "application/json"
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].status" == "404"
jsonpath "$.errors[0].source.pointer" == "/pipelines/bna/00000000-0000-4000-8000-000000000000"

# Updating a pipeline that does not exist is not found, even without changes.
PATCH {{host}}/pipelines/bna/00000000-0000-4000-8000-000000000000
Authorization: Bearer {{token}}
content-type: application/json
{}
HTTP 404
[Asserts]
jsonpath "$.errors[0].status" == "404"

# Rejects an update with an unknown status.
PATCH {{host}}/pipelines/bna/{{pipeline_id}}
Authorization: Bearer {{token}}
content-type: application/json
{
  "status": "Bogus"
}
HTTP 400
[Asserts]
header "content-type" == "application/json"
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].status" == "400"
jsonpath "$.errors[0].source.pointer" == "/status"

# Rejects an update with an unknown step.
PATCH {{host}}/pipelines/bna/{{pipeline_id}}
Authorization: Bearer {{token}}
content-type: application/json
{
  "step": "Bogus"
}
HTTP 400
[Asserts]
header "content-type" == "application/json"
jsonpath "$.errors" count == 1
jsonpath "$.errors[0].status" == "400"
jsonpath "$.errors[0].source.pointer" == "/step"

# Updates the start time of the pipeline, and reads it back.
PATCH {{host}}/pipelines/bna/{{pipeline_id}}
Authorization: Bearer {{token}}
content-type: application/json
{
  "start_time": "2025-06-20T14:00:00Z"
}
HTTP 200
[Asserts]
jsonpath "$.start_time" == "2025-06-20T14:00:00Z"

GET {{host}}/pipelines/bna/{{pipeline_id}}
HTTP 200
[Asserts]
jsonpath "$.start_time" == "2025-06-20T14:00:00Z"
