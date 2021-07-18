#!/bin/zsh
# vim: ft=sh

export RUST_BACKTRACE=1
TESTS_DIR="lp_tests"
[ -d "./cargo" ] && CARGO="./cargo/bin/cargo" || CARGO="cargo"

single="false"
opt_level="release"
mode="cmp"
err_out="null"
inputs=()
flags=("--no-perturb")

while test $# -gt 0
do
    case "$1" in
        --debug) opt_level="debug"
            ;;
        --verbose) flags+=("--debug"); err_out="stderr"
            ;;
        --easy)
            inputs=(./$TESTS_DIR/input/vanderbei* ./$TESTS_DIR/input/v2* ./$TESTS_DIR/input/445k21* ./$TESTS_DIR/input/cycle.txt)
            mode="diff"
            ;;
        --vanderbei)
            inputs=(./$TESTS_DIR/input/vanderbei*)
            mode="diff"
            ;;
        --vol2)
            inputs=(./$TESTS_DIR/input/v2*)
            mode="diff"
            ;;
        --netlib)
            inputs=(./$TESTS_DIR/input/netlib*)
            mode="diff"
            ;;
        --pure)
            mode="pure"
            ;;
        *) inputs+=( "$1" )
            ;;
    esac
    shift
done

(if [ "$opt_level" = "release" ]; then $CARGO build --release; else $CARGO build; fi) || exit

execpath=./target/$opt_level/lp

for input in $inputs; do
    output=$(sed -E 's/input/output/' <<< $input)

    case "$mode" in
        pure)
            $execpath < $input
            ;;
        cmp)
            time $execpath $input $flags
            cat $output
            echo
            ;;
        diff)
            echo -n "\e[1m${input:t:r} \e[0m"
            [[ "$err_out" = "stderr" ]] && echo
            my_result=$($execpath $input $flags 2>/dev/$err_out)
            diff=$(git --no-pager diff --no-index =(echo $my_result) $output)
            [[ "$?" = "0" ]] && echo "\e[1m\e[32mOK\e[0m" || (echo && echo "$diff" | diff-so-fancy)
            [[ "$err_out" = "stderr" ]] && echo
            ;;
        esac
done
