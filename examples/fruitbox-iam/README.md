# fruitbox-iam

## grpc

```sh
grpcurl -import-path ./examples/fruitbox-iam/proto/fruitbox-iam/fruitbox -proto auth.proto \
  -plaintext -d '{"email":"admin@ultimate.com", "password":"2024.Ultimate"}' \
  localhost:8889 fruitbox.auth.AuthService/Signin
```
