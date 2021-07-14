#!/bin/zsh
# vim: ft=sh

[[ "$1" = "--release" ]] && cargo build --release || cargo build
[[ "$1" = "--release" ]] && folder="release" || folder="debug"

files=($(fd . ./test_LPs/input))
path=./target/$folder/lp;

for f in $files; do
    $path $f
done
