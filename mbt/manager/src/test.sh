#!/bin/bash

# 第1引数を名前として受け取る
name="$1"

# 入力を待機
read -r input

# 出力
echo "$name：$input"