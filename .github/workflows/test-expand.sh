#!/bin/bash

TEST_MODULES=(convolution dsu fenwicktree lazysegtree math maxflow mincostflow modint scc segtree string twosat)
TMP_PATH=$(mktemp -d)
SCRIPT_DIR=$(cd $(dirname $0); pwd)
TEST_FILE="test.rs"
FILE_HEAD="fn main() {}"

for MODULE in ${TEST_MODULES[@]};do
     python3 $SCRIPT_DIR/../../expand.py $MODULE > $TMP_PATH/$TEST_FILE
     echo $FILE_HEAD >> $TMP_PATH/$TEST_FILE
     rustc $TMP_PATH/$TEST_FILE 2>/dev/null
     if [ $? -ne 0 ];then
        echo "Error compiling for $MODULE"
        exit 1
    else
        echo "Test passed($MODULE)"
     fi
done