# Blog Project

Проект объединяет:

- HTTP API на `actix-web`;
- gRPC API на `tonic`;
- PostgreSQL и `sqlx`;
- JWT-аутентификацию;
- хеширование паролей через Argon2;
- клиентскую библиотеку с HTTP- и gRPC-транспортами;
- CLI-клиент;
- браузерный интерфейс на Rust/WebAssembly.

## Возможности

Пользователь может:

- зарегистрироваться;
- войти в систему;
- получить JWT-токен;
- просматривать список постов;
- просматривать отдельный пост;
- создавать посты;
- редактировать свои посты;
- удалять свои посты.

Просмотр постов доступен без авторизации.

Создание, редактирование и удаление требуют JWT-токен.

---
Сборка:

cargo build --workspace

rustup target add wasm32-unknown-unknown

cd blog-wasm

wasm-pack build --target web

cd ..

cargo run -p blog-server

cd blog-wasm
python3 -m http.server 8000

http://127.0.0.1:8000

---

Curl:

curl http://127.0.0.1:8080/health

curl -X POST http://127.0.0.1:8080/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "test_user",
    "email": "test_user@example.com",
    "password": "secret123"
  }'

  export TOKEN='<JWT_TOKEN>'

  curl -X POST http://127.0.0.1:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "test_user",
    "password": "secret123"
  }'

curl -X POST http://127.0.0.1:8080/api/posts \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Тестовый пост",
    "content": "Пост создан при помощи curl"
  }'

curl "http://127.0.0.1:8080/api/posts?limit=10&offset=0"

wasm-pack build --target web

curl -X PUT http://127.0.0.1:8080/api/posts/1 \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Обновлённый пост",
    "content": "Новое содержание поста"
  }'

  curl -i -X DELETE http://127.0.0.1:8080/api/posts/1 \
  -H "Authorization: Bearer $TOKEN"