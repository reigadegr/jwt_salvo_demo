# jwt与salvo的demo
## 介绍 
- 使用EdDSA非对称加密算法，更安全，性能更好

## 运行
```shell
cargo run
sh test.sh
```
- 脚本响应:
```txt
{"code":0,"msg":"成功获取用户信息","data":{"role":"admin","username":"user1","exp":1746085855}}
```

- dev模式打印token:
```txt
Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSJ9.eyJyb2xlIjoiYWRtaW4iLCJ1c2VybmFtZSI6InVzZXIxIiwiZXhwIjoxNzQ2MDg1ODU1fQ.YJWsi_2L62QxHE8PBvbX1p_oEUAeUDwd-9I6eqkFzfod_aZRX60V-HF4Xk7PPelelKZ5EnuLU8anloA1vHUQBA
```

- 本项目的deepwiki
https://github.com/reigadegr/jwt_salvo_demo
