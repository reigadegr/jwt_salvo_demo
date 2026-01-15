## 安装
```sh
 cargo install sea-orm-cli@2.0.0-rc.28
```

## 准备.env
```sh
echo "DATABASE_URL=postgres://user:pass@127.0.0.1:5432/dbname" | tee .env
```

## 使用
```sh
sea-orm-cli generate entity --with-serde=both -o app/src/entities
```
