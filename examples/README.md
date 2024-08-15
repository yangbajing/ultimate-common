# Examples

```sh
git clone https://github.com/yangbajing/ultimate-common
cd ultimate-common/examples
```

## 服务依赖

使用 docker compose 启动并初始化 PG 数据库

```sh
# 当前目录下有容器，先删除容器及数据卷（一般在 sql 有变更时需要）
#docker compose down --volumes --remove-orphans

docker compose up -d --build && docker compose logs -f
```

## api-example

### 启动服务

重新打开一个终端窗口并进入 `examples/` 目录，执行以下命令运行程序

```sh
cargo run --release --bin api-example
```

### 测试服务

#### 使用密码登录

```sh
curl -v --location 'http://localhost:8888/auth/login/pwd' \
--header 'Content-Type: application/json' \
--data-raw '{
    "email": "admin@ultimate.com",
    "pwd": "2024.Ultimate"
}' | python -m json.tool
```

登录成功返回 token

```sh
{
    "token": "eyJ0eXAiOiJKV1QiLCJlbmMiOiJBMTI4Q0JDLUhTMjU2IiwiYWxnIjoiZGlyIn0..EZwETCBq1CNs8yO5Zec09Q.g3JoMryHoq01ZO3TQ2Ja_ppJZb9SYdon-LfB6OGyH7s.sBCGn14NuoxujmAgRpkYPg",
    "token_type": "Bearer"
}
```

#### 用户-分页查询

_你需要使用上面 [使用密码登录](#使用密码登录) 来获取 token，并替换到下面 `Bearer <token>` 位置。_

```sh
curl -v --location 'http://localhost:8888/v1/user/page' \
--header 'Content-Type: application/json' \
--header 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJlbmMiOiJBMTI4Q0JDLUhTMjU2IiwiYWxnIjoiZGlyIn0..EZwETCBq1CNs8yO5Zec09Q.g3JoMryHoq01ZO3TQ2Ja_ppJZb9SYdon-LfB6OGyH7s.sBCGn14NuoxujmAgRpkYPg' \
--data '{
    "filter": {
        "ctime": {
            "$gte": "2024-08-15T00:00:00+0800",
            "$lt": "2024-08-16T00:00:00+0800"
        }
    }
}' | python -m json.tool --no-ensure-ascii
```

#### 用户-根据ID查询

```sh
curl -v --location 'http://localhost:8888/v1/user/10000' \
--header 'Content-Type: application/json' \
--header 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJlbmMiOiJBMTI4Q0JDLUhTMjU2IiwiYWxnIjoiZGlyIn0..EZwETCBq1CNs8yO5Zec09Q.g3JoMryHoq01ZO3TQ2Ja_ppJZb9SYdon-LfB6OGyH7s.sBCGn14NuoxujmAgRpkYPg' | python -m json.tool --no-ensure-ascii
```
