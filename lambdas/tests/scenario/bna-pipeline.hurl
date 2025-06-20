# ############
# BNA Pipeline
# ############

# Create a new BNA pipeline
POST {{host}}/pipelines/bna
content-type: application/json
{
  "s3_bucket": "united states/{{region}}/{{name}}/25.06",
  "sqs_message": {
    "city": "{{name}}",
    "country": "{{country}}",
    "fips_code": "3570670",
    "region": "{{region}}"
  },
  "state_machine_id": "3fa85f64-5717-4562-b3fc-2c963f66afa6"
}
HTTP 201
[Captures]
state_machine_id: jsonpath "$.state_machine_id"

# Update pipeline with additional information
PATCH {{host}}/pipelines/bna/{{state_machine_id}}
content-type: application/json
{
  "fargate_task_arn": "arn:aws:ecs:us-west-2:123456789012:task/bna/29f979fc9fca402d94b014aa23d2f6e0",
  "status": "Processing",
  "step": "Analysis"
}
HTTP 200

# Retrieve the city.
GET {{host}}/cities/{{country}}/{{region}}/{{name}}
HTTP 200
[Captures]
city_id: jsonpath "$.id"

# Post the city rating.
POST {{host}}/ratings
content-type: application/json
{
  "city_id": "{{city_id}}",
  "core_services": {
    "dentists": 0,
    "doctors": 0,
    "grocery": 1.69,
    "hospitals": 5.18,
    "pharmacies": 0,
    "score": 3.24
  },
  "infrastructure": {
    "high_stress_miles": 64.5,
    "low_stress_miles": 9.3
  },
  "opportunity": {
    "employment": 8.26,
    "higher_education": 0,
    "k12_education": 8.31,
    "score": 8.29,
    "technical_vocational_college": 0
  },
  "people": {
    "people": 19.17
  },
  "recreation": {
    "community_centers": 0,
    "parks": 7.13,
    "score": 7.13,
    "trails": 0
  },
  "retail": {
    "retail": 0
  },
  "transit": {
    "transit": 0
  },
  "measure": {},
  "pop_size": 2,
  "population": 123456,
  "version": "24.12",
  "score": 8.93
}
HTTP 201

# Update the pipeline and mark it as complete.
PATCH {{host}}/pipelines/bna/{{state_machine_id}}
content-type: application/json
{
  "cost": "0.63749",
  "end_time": "2025-06-20T14:11:32.395437Z",
  "start_time": "2025-06-20T14:16:11.133641Z",
  "status": "Completed",
  "step": "Cleanup"
}
HTTP 200
