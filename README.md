
# DDD理念在使用 Rust 开发 Web 服务中的应用

## diesel 管理数据库

diesel 是 Rust 生态中的用于数据库管理的工具，

### 安装

```shell
# diesel_cli 管理工具安装
cargo install diesel_cli --no-default-features --features postgres
```

### 使用

- 初始化

```shell
diesel setup
```

在项目的根目录执行，开启数据迁移记录, 产生 `migrations` 文件夹, 内部记录数据库模型变更记录。并生成初始化记录，这次记录是可以删除的

- 进行一次变更

```shell
diesel migration generate create_user 
```

每一次变更都生成一个文件夹, 包含两个文件

- `down.sql` : 用于记录从 up.sql 中需要撤销的操作
- `up.sql` : 用于记录从 本次数据迁移需要执行的 sql

```tree
.
├── 00000000000000_diesel_initial_setup
│   ├── down.sql
│   └── up.sql
└── 2024-07-23-025412_create_user
    ├── down.sql
    └── up.sql
```

当需要执行的迁移操作已经写入的 `up.sql` / `down.sql` 文件中，再执行如下的操作

```shell
diesel migration run
```
---
```text
  Running migration 2024-07-23-025412_create_user
````
`
`执行成功后会再项目的 `src` 目录中生成 `schema.rs` 文件

```shell

// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        phone -> Varchar,
        address -> Varchar,
    }
}
```

至此，数据库表就创建好了

### 框架说明

- presentation
- application
- domain
- infrastructure
