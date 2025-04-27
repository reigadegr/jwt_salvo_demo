jwt="$(curl -X POST http://localhost:3000/login -H "Content-Type: application/json" -d '{"username": "user1", "password": "password1"}' | jq  ".token" | cut -d '"' -f2)"

curl -X GET 'http://localhost:3000/profile' -H "Authorization: Bearer $jwt"

