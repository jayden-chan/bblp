#!/bin/zsh
# vim: ft=sh

RUST_BACKTRACE=1

set -e
[[ "$1" = "--release" ]] && cargo build --release || cargo build
[[ "$1" = "--release" ]] && folder="release" || folder="debug"
set +e

flags=()
if [ "$2" = "--single" ]; then
    inputs=($3)
elif [ "$2" = "--vanderbei" ]; then
    inputs=($(ls ./test_LPs/input/vanderbei*))
elif [ "$2" = "--vol2" ]; then
    inputs=($(ls ./test_LPs/input/v2*))
elif [ "$2" = "--netlib" ]; then
    inputs=($(ls ./test_LPs/input/netlib*))
else
    inputs=($(ls ./test_LPs/input/*))
fi

execpath=./target/$folder/lp;

for input in $inputs; do
    local output=$(echo $input | sed -E 's/input/output/')
    # echo $input
    # echo $output
    GIT_PAGER="diff-so-fancy | less --tabs=4 -RFX" git diff --no-index =($execpath $input $flags 2>/dev/null) $output
    # cat $input
    # cat $output
done
