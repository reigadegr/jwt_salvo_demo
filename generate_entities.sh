#!/bin/sh

set -eu

dir="crates/entities/src"

# 默认使用 SQLite（从 .env 文件读取或使用默认值）
DATABASE_URL="${1:-}"

if [ -z "$DATABASE_URL" ]; then
    # 尝试从 .env 文件读取
    if [ -f ".env" ]; then
        . ./.env
        DATABASE_URL="${DATABASE_URL:-sqlite://target/jsd.db}"
    else
        DATABASE_URL="sqlite://target/jsd.db"
    fi
fi

echo "Generating entities from: $DATABASE_URL"
echo "Output directory: $dir"

# 确保目标目录存在
mkdir -p "$dir"

# 生成实体（使用 lib.rs 而不是 mod.rs）
sea-orm-cli generate entity \
    --database-url "$DATABASE_URL" \
    --with-serde=both \
    --lib \
    -o "$dir"

# sea-orm-cli 使用 --lib 会生成 lib.rs，删除不需要的 mod.rs
rm -f "$dir/mod.rs"

# 后处理：统一不同数据库的类型差异
# MySQL: TINYINT(1) -> i8, SQLite: INTEGER -> i32, PostgreSQL: BOOLEAN -> bool
# MySQL: BIGINT -> i64, SQLite: INTEGER -> i32, PostgreSQL: BIGINT -> i64
# MySQL/PostgreSQL: BINARY(16) -> Vec<u8>, SQLite: TEXT -> String (for UUID)

for i in "$dir"/*; do
    [ -f "$i" ] || continue
    filename=$(basename "$i")

    # 跳过 lib.rs 和 prelude.rs
    case "$filename" in
        lib.rs|prelude.rs|mod.rs)
            echo "$i 跳过"
            continue
            ;;
    esac

    echo "Processing: $i"

    # 统一类型：
    # 1. i8 -> bool (MySQL TINYINT(1) 用于布尔值)
    # 2. i64 -> i32 (统一整数类型)
    # 3. Vec<u8> -> Uuid (UUID 类型)
    sed -i 's/i8/bool/g' "$i"
    sed -i 's/i64/i32/g' "$i"
    sed -i 's/Vec<u8>/Uuid/g' "$i"

    # 移除 PostgreSQL 特有的 schema_name
    sed -i 's/schema_name = "public", //g' "$i"

    # 移除 MySQL/PostgreSQL 特有的 column_type 注解
    sed -i 's/, column_type = "Binary(16)"//g' "$i"

    # 修复 SQLite TEXT 主键问题：sea-orm-cli 错误地将 PRIMARY KEY TEXT 识别为 Option<String>
    # 将主键从 Option<String> 改为 String
    sed -i 's/pub id: Option<String>/pub id: String/g' "$i"

    # 移除主键的 nullable 属性（处理 #[sea_orm(primary_key, ..., nullable, ...)]）
    sed -i 's/, nullable//g' "$i"

    # 添加 uuid crate 导入（如果使用了 Uuid 类型且还没有导入）
    if grep -q "Uuid" "$i" && ! grep -q "use uuid::Uuid;" "$i"; then
        sed -i '/use serde::{Deserialize, Serialize};/a use uuid::Uuid;' "$i"
    fi
done

echo "Entity generation complete!"
