set -x
curl -X POST http://localhost:3000/forward -H "Content-Type: application/json" -d '{"username": "user1", "password": "password1"}' | jq ".data" | sed 's/"//g'
