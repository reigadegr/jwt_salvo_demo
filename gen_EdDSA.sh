# 生成私钥
openssl genpkey -algorithm ED25519 -out dev_kit/keys/private_key.pem

# 从私钥导出公钥
openssl pkey -in dev_kit/keys/private_key.pem -pubout -out dev_kit/keys/public_key.pem
