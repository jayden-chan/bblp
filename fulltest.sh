#!/bin/zsh
# vim: ft=sh

export RUST_BACKTRACE=1
single="false"
mode="debug"
inputs=()

while test $# -gt 0
do
    case "$1" in
        --release) mode="release"
            ;;
        --debug) mode="debug"
            ;;
        --easy) inputs=(./test_LPs/input/vanderbei* ./test_LPs/input/v2* ./test_LPs/input/445k21*)
            ;;
        --vanderbei) inputs=(./test_LPs/input/vanderbei*)
            ;;
        --vol2) inputs=(./test_LPs/input/v2*)
            ;;
        --netlib) inputs=(./test_LPs/input/netlib*)
            ;;
        *) inputs+=( "$1" ); single="true"
            ;;
    esac
    shift
done

(if [ "$mode" = "release" ]; then cargo build --release; else cargo build; fi) || exit

execpath=./target/$mode/lp

for input in $inputs; do
    local output=$(echo $input | sed -E 's/input/output/')

    if [ "$single" = "true" ]; then
        $execpath $input
        cat $output
        echo
    else
        echo -n ${input:t:r}
        local my_result=$($execpath $input 2>/dev/null)
        local diff=$(git --no-pager diff --no-index =(echo $my_result) $output)
        [[ "$?" = "0" ]] && echo " \e[1m\e[32mOK\e[0m" || (echo && echo "$diff" | diff-so-fancy)
    fi
done
