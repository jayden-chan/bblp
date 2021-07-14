#!/bin/zsh
# vim: ft=sh

files=($(fd . ./test_LPs/input))

for f in $files; do
    ./target/debug/lp $f
done
