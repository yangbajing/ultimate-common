# Ultimate Common

Rust 实用工具库，微服务融合……

- [crates](crates/): 工具库、组件库，可以包含对第 3 方服务的依赖。
- [ultimates](ultimates/): ultimate 融合库。比如：数据库、消息系统等，可以放置类似 `spring-boot` 一样的 **starter**。

## 开发

自动编译

```sh
cargo watch -d 2 -s 'cargo clippy-all && cargo build'
```

静态检查

```sh
cargo clippy-all && cargo check-all
```
