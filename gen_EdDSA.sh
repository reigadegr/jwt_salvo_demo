# 生成私钥
openssl genpkey -algorithm ED25519 -out keys/private_key.pem

# 从私钥导出公钥
openssl pkey -in keys/private_key.pem -pubout -out keys/public_key.pem
