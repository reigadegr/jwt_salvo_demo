if [ ! -d target ]; then
    mkdir target
    uid=$(dumpsys package com.termux | grep appId | awk 'NR==1{print $1}' | cut -d '=' -f2)
    chown -R $uid:$uid ./tatget
    chmod -R 0755 ./target
fi
name=$(cat Cargo.toml | grep "name = " | cut -d ' ' -f3 | sed 's/"//g')
{
    for i in Cargo.toml; do
        echo -e "---以下为项目的$i---\n"
        cat $i
        echo "------------------------"
    done
    
    for i in $(find ./src -name "*.rs"); do
        echo "这是$i: "
        cat $i
        echo "\n--------------\n"
    done
} > target/"$name"_all_code.txt
