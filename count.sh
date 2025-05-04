#!//bin/sh
for i in $(grep "members" Cargo.toml | awk -F '[][]' '{print $2}' | sed 's/,//g' | sed 's/"//g'); do
    echo "$i crate代码行数: "
    find $(dirname "$0")/$i  -name "*.rs" -exec cat {} \; | wc -l
done
