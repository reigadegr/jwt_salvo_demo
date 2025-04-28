# 生成私钥
openssl genpkey -algorithm ED25519 -out private_key.pem

# 从私钥导出公钥
openssl pkey -in private_key.pem -pubout -out public_key.pem
