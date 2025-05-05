#!//bin/sh
for i in $(grep "members" Cargo.toml | awk -F '[][]' '{print $2}' | sed 's/,//g' | sed 's/"//g'); do
    echo "$i 代码行数: "
    find $(dirname "$0")/$i  -name "*.rs" -exec cat {} \; | wc -l
    echo "$i clone行数: "
    find $(dirname "$0")/$i  -name "*.rs" -exec cat {} \; | grep clone | wc -l
done
