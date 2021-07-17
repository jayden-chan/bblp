#!/bin/zsh
# vim: ft=sh

export RUST_BACKTRACE=1
export GIT_PAGER="diff-so-fancy" 

set -e
[[ "$1" = "--release" ]] && cargo build --release || cargo build
[[ "$1" = "--release" ]] && folder="release" || folder="debug"
set +e

flags=()
if [ "$2" = "--single" ]; then
    inputs=($3)
elif [ "$2" = "--easy" ]; then
    inputs=($(ls ./test_LPs/input/vanderbei* ./test_LPs/input/v2* ./test_LPs/input/445k21*))
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

    if [ "$2" = "--single" ]; then
        my_result=$($execpath $input $flags)
        echo $my_result
        cat $output
    else
        echo -n ${input:t:r}
        my_result=$($execpath $input $flags 2>/dev/null)
        git diff --no-index =(echo $my_result) $output
        [[ "$?" = "0" ]] && echo " \e[1m\e[32mOK\e[0m" || echo
    fi
done
