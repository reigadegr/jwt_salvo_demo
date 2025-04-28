#!//bin/sh
echo "rs代码行数: "
find $(dirname "$0")/src -name "*.rs" -exec cat {} \; | wc -l
