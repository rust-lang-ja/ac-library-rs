#!/bin/bash

TEST_MODULES=(convolution dsu fenwicktree lazysegtree math maxflow mincostflow modint scc segtree string twosat --all)
TMP_PATH=$(mktemp -d)
# shellcheck disable=SC2164
SCRIPT_DIR="$(cd "$(dirname "$0")"; pwd)"
TEST_FILE="test.rs"
FILE_HEAD="fn main() {}"

for MODULE in "${TEST_MODULES[@]}" ;do
     echo Test module "$MODULE"
     python3 "$SCRIPT_DIR/../../expand.py" "$MODULE" > "$TMP_PATH/$TEST_FILE"
     echo Output "$(wc -c < "$TMP_PATH/$TEST_FILE")" Bytes
     echo "$FILE_HEAD" >> "$TMP_PATH/$TEST_FILE"
     if ! rustc -A warnings "$TMP_PATH/$TEST_FILE";
     then
        echo Error compiling for "$MODULE"
        exit 1
    else
        echo Test passed
     fi
done
