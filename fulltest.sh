#!/bin/zsh
# vim: ft=sh

export RUST_BACKTRACE=1
TESTS_DIR="lp_tests"
[ -d "./cargo" ] && CARGO="./cargo/bin/cargo" || CARGO="cargo"

single="false"
mode="debug"
should_diff="false"
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
        --easy) inputs=(./$TESTS_DIR/input/vanderbei* ./$TESTS_DIR/input/v2* ./$TESTS_DIR/input/445k21* ./$TESTS_DIR/input/cycle.txt)
            ;;
        --vanderbei) inputs=(./$TESTS_DIR/input/vanderbei*)
            ;;
        --vol2) inputs=(./$TESTS_DIR/input/v2*)
            ;;
        --netlib) inputs=(./$TESTS_DIR/input/netlib*)
            ;;
        --diff) should_diff="true"
            ;;
        *) inputs+=( "$1" )
            ;;
    esac
    shift
done

(if [ "$mode" = "release" ]; then $CARGO build --release; else $CARGO build; fi) || exit

execpath=./target/$mode/lp

for input in $inputs; do
    output=$(sed -E 's/input/output/' <<< $input)

    if [ "$should_diff" = "false" ]; then
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
