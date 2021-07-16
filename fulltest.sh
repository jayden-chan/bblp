#!/bin/zsh
# vim: ft=sh

set -e
[[ "$1" = "--release" ]] && cargo build --release || cargo build
[[ "$1" = "--release" ]] && folder="release" || folder="debug"
set +e

flags=()
if [ "$2" = "--single" ]; then
    inputs=($3)
    # flags+=("--debug")
else
    inputs=($(ls ./test_LPs/input/*))
fi

execpath=./target/$folder/lp;

for input in $inputs; do
    local output=$(echo $input | sed -E 's/input/output/')
    # echo $input
    # echo $output
    $execpath $input $flags
    # cat $input
    cat $output
    echo
    echo "####################################################"
    echo
done
