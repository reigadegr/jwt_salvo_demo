## 安装
```sh
 cargo install sea-orm-cli@2.0.0-rc.38
```

## 准备.env
```sh
echo "DATABASE_URL=sqlite://target/jsd.db" | tee .env
```

## 使用
```sh
sea-orm-cli generate entity --with-serde=both -o crates/entities/src
```
