if [ ! -d target ]; then
    mkdir target
    uid=$(dumpsys package com.termux | grep appId | awk 'NR==1{print $1}' | cut -d '=' -f2)
    chown -R $uid:$uid ./tatget
    chmod -R 0755 ./target
fi
name=$(basename $(dirname "$0"))
{
    for i in Cargo.toml; do
        echo -e "---以下为项目的$i---\n"
        cat $i
        echo "------------------------"
    done
    
    for i in $(grep "members" Cargo.toml | awk -F '[][]' '{print $2}' | sed 's/,//g' | sed 's/"//g'); do
        for i in $(find $i/src -name "*.rs") $i/Cargo.toml; do
            echo "这是$i: "
            cat $i
            echo "\n--------------\n"
        done
    done
} > target/"$name"_all_code.txt
