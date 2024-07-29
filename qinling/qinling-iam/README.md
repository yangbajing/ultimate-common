# qinling-iam

## Generate Entities

从数据库生成 rust 代码。不要直接覆盖代码，先生成到临时目录，再将差异按需拷贝到代码中。使用上更多用于方便新数据表的代码生成，历史数据表建议手动调整。
更多 `sea-orm-cli` 使用说明见 [Using sea-orm-cli](https://www.sea-ql.org/SeaORM/docs/generate-entity/sea-orm-cli/) 。

> 因为对于生成的代码，很多时候会考虑进行调整以满足更多其它功能。比如添加 serde 和 graphql 注解。

iam

```
sea-orm-cli generate entity \
    --with-serde=both \
    --date-time-crate=chrono \
    --seaography \
    --serde-skip-hidden-column \
    --database-schema=iam \
    -o .tmp/iam/entity
```
