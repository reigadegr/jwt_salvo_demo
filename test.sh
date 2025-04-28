jwt="$(curl -X POST http://localhost:3000/login -H "Content-Type: application/json" -d '{"username": "user1", "password": "password1"}' | jq  ".data.token" | sed 's/"//g')"

curl -X GET 'http://localhost:3000/profile' -H "Authorization: Bearer $jwt"

