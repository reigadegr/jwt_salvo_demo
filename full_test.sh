#!/bin/bash
set -e

BASE_URL="http://127.0.0.1:3000"
echo -e "\033[32m=== JWT认证一键测试脚本 ===\033[0m"
echo "服务地址: $BASE_URL"

# 步骤1：登录拿token
echo -e "\n\033[34m【步骤1】登录获取JWT令牌\033[0m"
RESP=$(curl -s -w "\n%{http_code}" -X POST "$BASE_URL/login" \
  -H "Content-Type: application/json" \
  -d '{"username":"user1","password":"password1"}')
HTTP_CODE=$(echo "$RESP" | tail -n1)
RESP_BODY=$(echo "$RESP" | head -n-1)

if [ "$HTTP_CODE" != "200" ]; then
  echo -e "\033[31m登录失败！响应: $RESP_BODY\033[0m"
  exit 1
fi
TOKEN=$(echo "$RESP_BODY" | grep -o '"data":"[^"]*' | cut -d'"' -f4)
echo -e "\033[32m登录成功！Token: $TOKEN\033[0m"

# 步骤2：正常访问profile（带有效token）
echo -e "\n\033[34m【步骤2】带有效Token访问/profile\033[0m"
curl -i -X GET "$BASE_URL/profile" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json"

# 步骤3：无token访问profile（预期401）
echo -e "\n\033[34m【步骤3】无Token访问/profile（预期401）\033[0m"
curl -i -X GET "$BASE_URL/profile" \
  -H "Content-Type: application/json"

# 步骤4：无效token访问profile（预期403）
echo -e "\n\033[34m【步骤4】无效Token访问/profile（预期403）\033[0m"
curl -i -X GET "$BASE_URL/profile" \
  -H "Authorization: Bearer invalid_token_123" \
  -H "Content-Type: application/json"

echo -e "\n\033[32m=== 全场景测试完成 ===\033[0m"

# 步骤1：登录拿token
echo -e "\n\033[34m【步骤1】登录获取JWT令牌\033[0m"
RESP=$(curl -s -w "\n%{http_code}" -X POST "$BASE_URL/login" \
  -H "Content-Type: application/json" \
  -d '{"username":"user2","password":"password2"}')
HTTP_CODE=$(echo "$RESP" | tail -n1)
RESP_BODY=$(echo "$RESP" | head -n-1)

if [ "$HTTP_CODE" != "200" ]; then
  echo -e "\033[31m登录失败！响应: $RESP_BODY\033[0m"
  exit 1
fi
TOKEN=$(echo "$RESP_BODY" | grep -o '"data":"[^"]*' | cut -d'"' -f4)
echo -e "\033[32m登录成功！Token: $TOKEN\033[0m"

# 步骤2：正常访问profile（带有效token）
echo -e "\n\033[34m【步骤2】带有效Token访问/profile\033[0m"
curl -i -X GET "$BASE_URL/profile" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json"
