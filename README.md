# blog-project

создание структуры проекта:
cargo new blog-client --lib
cargo new blog-wasm --lib
cargo new blog-proto --lib
cargo new blog-server
cargo new blog-cli 


---
curl http://127.0.0.1:8080/health
---
curl -X POST http://127.0.0.1:8080/api/auth/register \
-H "Content-Type: application/json" \
-d '{
    "username":"vasiliy",
    "email":"vasiliy@example.com",
    "password":"123456"
}'

{"user":{"id":1,"username":"vasiliy","email":"vasiliy@example.com","password_hash":"$argon2id$v=19$m=19456,t=2,p=1$AbxDZ7XgLur8dNlFVi9bOg$nOyBoQE2ebbcvCfeYFfDlYG9TsOm22FZXh6qdRMS05A","created_at":"2026-07-12T13:33:33.759441Z"},"token":"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VyX2lkIjoxLCJ1c2VybmFtZSI6InZhc2lsaXkiLCJleHAiOjE3ODM5NDk2MTN9.RrZAC3AnKD6duTrCvQZht4cZE8TBBc-lWeyHMqENrRE"}

export TOKEN='eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VyX2lkIjoxLCJ1c2VybmFtZSI6InZhc2lsaXkiLCJleHAiOjE3ODM5NDk2MTN9.RrZAC3AnKD6duTrCvQZht4cZE8TBBc-lWeyHMqENrRE'
----

curl -i -X POST http://127.0.0.1:8080/api/posts \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Первый пост",
    "content": "Мой первый пост на Rust и Actix Web"
  }'


  ---

  curl -i http://127.0.0.1:8080/api/posts/1

  ---

  curl -i "http://127.0.0.1:8080/api/posts?limit=10&offset=0"

  ---

  curl -i -X DELETE http://127.0.0.1:8080/api/posts/1 \
  -H "Authorization: Bearer $TOKEN"

  ---

  curl -i http://127.0.0.1:8080/api/posts/1

  ---
  curl -i "http://127.0.0.1:8080/api/posts?limit=10&offset=0"

  ---

ss -ltn | grep 50051
---
grpcurl -plaintext localhost:50051 list

---

grpcurl \
  -plaintext \
  -import-path blog-proto/proto \
  -proto blog.proto \
  localhost:50051 list

  ---
grpcurl \
  -plaintext \
  -emit-defaults \
  -import-path blog-proto/proto \
  -proto blog.proto \
  -d '{"limit":10,"offset":0}' \
  localhost:50051 \
  blog.BlogService/ListPosts


---
grpcurl \
  -plaintext \
  -import-path blog-proto/proto \
  -proto blog.proto \
  -d '{"id":1}' \
  localhost:50051 \
  blog.BlogService/GetPost

  ---

  grpcurl \
  -plaintext \
  -import-path blog-proto/proto \
  -proto blog.proto \
  -H "authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VyX2lkIjoyLCJ1c2VybmFtZSI6Iml2YW4iLCJleHAiOjE3ODM5NjkzMjl9.puXJqHIdMsJjMPOs6DvdvtWmxuzBzKi7fQ2f4APQHqY" \
  -d '{
    "title":"Первый gRPC пост",
    "content":"Пост создан через grpcurl"
  }' \
  localhost:50051 \
  blog.BlogService/CreatePost

  ---
cargo run -p blog-server
---

  cargo run -p blog-cli -- register \
  --username vasiliy2 \
  --email vasiliy@example2.com \
  --password secret123

  ---


  cargo run -p blog-cli -- login \
  --username vasiliy2 \
  --password secret123

  ---

  cargo run -p blog-cli -- create \
  --title "Первый пост" \
  --content "Проверка HTTP-клиента"

  ---

  cargo run -p blog-cli -- list

  cargo run -p blog-cli -- get --id 2