# jwt与salvo的demo
## 介绍 
- 使用RS256非对称加密算法，更安全

## 运行
```shell
cargo run
sh test.sh
```
- 脚本响应:
```txt
{"code":0,"msg":"成功获取用户信息","data":{"user":"user1"}}
```

- dev模式打印token:
```txt
Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJzdWIiOiJ1c2VyMSIsImV4cCI6MTc0NTg0MzkxMiwiaWF0IjoxNzQ1ODQzOTYzfQ.YOKCzIiadZ3JqgJezUOn6kSN9hwZFdZq2oIKQrvHsg9p5_asPf9Rf9XTl_7QRgepUAdk2-m5bwL750VT6ZZKAWtlkc_bwhqFmhIEk2fqQ3voGqaM2eynoIS5k55I_oolgQup_aohPUa1eCuSoSVTWo6gGqkb7EAKQzQNM-wqDvdSgovyhtZ5s6QGotQVlUvc9jF0DfJdiHDcWarFYbRt7syfVAGKaQXsuvUQfEQkjPztWr7WEGlyib3sI7UVM33OwW6P1a58H7ci3svBCP8XtGQ2ikSwP205aRpdH9WRYwcsMbFllMJyIrCRBB44slJytRVOPS687uyTfhkz1RCP7Q
```
