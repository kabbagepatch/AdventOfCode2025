#!/bin/bash

source .env # for API_BASE_URL and PROJECT_ID

GET_TASKS_ENDPOINT="$API_BASE_URL/projects/$PROJECT_ID/tasks?user=kavish"

TODAYS_DAY=$(expr $1 + 0)
TOMMORROWS_DAY=$(expr $TODAYS_DAY + 1)
TASKS_RESPONSE=$(curl -s -X GET "$GET_TASKS_ENDPOINT")
ACTIVE_TASK_ID=$(echo "$TASKS_RESPONSE" | jq -r --arg DAY "Day $TODAYS_DAY" '.[] | select(.name | contains($DAY)) | .id')
QUEUED_TASK_ID=$(echo "$TASKS_RESPONSE" | jq -r --arg DAY "Day $TOMMORROWS_DAY" '.[] | select(.name | contains($DAY)) | .id')
ACTIVE_STATUS='{"status": "active"}'
QUEUED_STATUS='{"status": "queued"}'

UPDATE_RESPONSE=$(curl -s -X PUT "$API_BASE_URL/tasks/$ACTIVE_TASK_ID/status?user=kavish" -d "$ACTIVE_STATUS")
UPDATE_RESPONSE=$(curl -s -X PUT "$API_BASE_URL/tasks/$QUEUED_TASK_ID/status?user=kavish" -d "$QUEUED_STATUS")
