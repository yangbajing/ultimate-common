# fruitbox-iam

## 运行

```sh
cargo run -p fruitbox-iam --bin fruitbox-iam
```

## grpc

```sh
grpcurl -plaintext -import-path ./examples/fruitbox-iam/proto \
  -import-path ./examples/fruitbox-iam/proto/fruitbox_iam/v1 -proto auth.proto \
  -d '{"email":"admin@ultimate.com", "password":"2024.Ultimate"}' \
  localhost:8889 fruitbox_iam.v1.AuthService/Signin

grpcurl -plaintext -import-path ./examples/fruitbox-iam/proto \
  -import-path ./examples/fruitbox-iam/proto/fruitbox_iam/v1 \
  -proto user.proto \
  -H 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJlbmMiOiJBMTI4Q0JDLUhTMjU2IiwiYWxnIjoiZGlyIn0..Y2qiFBZyc7T01VV4ZG_jAw.mGTz66crmc054z7elr9QgS2-CYZHVptu1hOTIt2nQdY.f4TwgO3dFXPKAY4qqNu6mA' \
  -d '{"id":1}' \
  localhost:8889 fruitbox_iam.v1.UserService/Find
```
