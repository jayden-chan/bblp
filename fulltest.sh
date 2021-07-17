#!/bin/zsh
# vim: ft=sh

export RUST_BACKTRACE=1
single="false"
mode="debug"
err_out="null"
inputs=()
flags=()

while test $# -gt 0
do
    case "$1" in
        --release) mode="release"
            ;;
        --debug) mode="debug"
            ;;
        --verbose) flags+=("--debug"); err_out="stderr"
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
        $execpath $input $flags
        cat $output
        echo
    else
        echo -n "\e[1m${input:t:r} \e[0m"
        [[ "$err_out" = "stderr" ]] && echo
        my_result=$($execpath $input $flags 2>/dev/$err_out)
        diff=$(git --no-pager diff --no-index =(echo $my_result) $output)
        [[ "$?" = "0" ]] && echo "\e[1m\e[32mOK\e[0m" || (echo && echo "$diff" | diff-so-fancy)
        [[ "$err_out" = "stderr" ]] && echo
    fi
done
