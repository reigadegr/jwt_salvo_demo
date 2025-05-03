set -x
rm $(find ./ -name "*.bak")
rm $(dirname "$0")/*.bak $(dirname "$0")/.*.bak module/*.bak

rm $(find ./*/src -name "jwt_salvo_demo")
rm ./jwt_salvo_demo

for i in $(find ./*/src -name "*.rs"); do
    nohup dos2unix $i >/dev/null 2>&1 &
done

nohup rm -rf $(find ./target -name "*jwt_salvo_demo*") >/dev/null 2>&1 &
uid=$(dumpsys package com.termux | grep appId | awk 'NR==1{print $1}' | cut -d '=' -f2)
chown -R $uid:$uid  ./*/src ./*/*.toml ./*/keys  ./casbin
chmod -R 0644  ./*/src ./*/*.toml ./*/keys  ./casbin
