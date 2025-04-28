# jwt与salvo的demo
## 运行
```shell
cargo run
sh test.sh
```
- 脚本响应:
```txt
{"code":0,"data":{"user":{"exp":1745824101,"iat":1745824152,"sub":"user1"}},"msg":"成功 获取用户信息"}
```

- dev模式打印token:
```txt
Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ1c2VyMSIsImV4cCI6MTc0NTgyNDEwMSwiaWF0IjoxNzQ1ODI0MTUyfQ.ER8OXhmz2im1ndv9_NljZvz_TDA66meeWhI3w65pOe8
```
