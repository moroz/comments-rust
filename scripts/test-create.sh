#!/bin/bash -e

BODY='{"body":"Test","url":"working","signature":"test"}'

ENDPOINT="${ENDPOINT:-http://localhost:9000}"

curl -X POST $ENDPOINT -d "$BODY" -H "Content-Type: application/json"
