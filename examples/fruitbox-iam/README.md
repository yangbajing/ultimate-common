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
  -H 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJlbmMiOiJBMTI4Q0JDLUhTMjU2IiwiYWxnIjoiZGlyIn0..pmcUdN9wb8J63fkU6JDOJw.kDRISHrRKvo58GSC1TVCNGmjfnojcWFgcuhfNypsQjI.kPWYQa3ApiP7QFkVLNWwrw' \
  -d '{"id":1}' \
  localhost:8889 fruitbox_iam.v1.UserService/Find


grpcurl -plaintext -import-path ./examples/fruitbox-iam/proto \
  -import-path ./examples/fruitbox-iam/proto/fruitbox_iam/v1 \
  -proto role.proto \
  -H 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJlbmMiOiJBMTI4Q0JDLUhTMjU2IiwiYWxnIjoiZGlyIn0..pmcUdN9wb8J63fkU6JDOJw.kDRISHrRKvo58GSC1TVCNGmjfnojcWFgcuhfNypsQjI.kPWYQa3ApiP7QFkVLNWwrw' \
  -d '{
       "field_mask":{ "paths": ["role", "permissions"]},
       "create_role": {
         "name":"test2",
         "description":"测试角色2",
         "status":"ROLE_STATUS_DISABLED"
       },
       "permission_ids":[1,2]
     }' \
  localhost:8889 fruitbox_iam.v1.RoleService/Create
```
