#!/bin/bash

source .env # for API_BASE_URL and PROJECT_ID

GET_TASKS_ENDPOINT="$API_BASE_URL/projects/$PROJECT_ID/tasks?user=kavish"

TODAYS_DAY=$(expr $1 + 0)
TASKS_RESPONSE=$(curl -s -X GET "$GET_TASKS_ENDPOINT")
TASK_ID=$(echo "$TASKS_RESPONSE" | jq -r --arg DAY "Day $TODAYS_DAY" '.[] | select(.name | contains($DAY)) | .id')
COMPLETE_STATUS='{"status": "complete"}'

UPDATE_RESPONSE=$(curl -s -X PUT "$API_BASE_URL/tasks/$TASK_ID/status?user=kavish" -d "$COMPLETE_STATUS")
