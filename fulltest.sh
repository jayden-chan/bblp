#!/bin/zsh
# vim: ft=sh

# Copyright © 2021 Jayden Chan. All rights reserved.

# bblp is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License version 3
# as published by the Free Software Foundation.

# bblp is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
# GNU General Public License for more details.

# You should have received a copy of the GNU General Public License
# along with bblp. If not, see <https://www.gnu.org/licenses/>.

export RUST_BACKTRACE=1

TESTS_DIR="lp_tests"

single="false"
cargo_exe="cargo"
opt_level="release"
mode="cmp"
err_out="null"
inputs=()
flags=()

[ -d "./cargo" ] && cargo_exe="./cargo/bin/cargo"

while test $# -gt 0
do
    case "$1" in
        --debug) opt_level="debug"
            ;;
        --verbose) flags+=("--debug"); err_out="stderr"
            ;;
        --easy)
            inputs+=(./$TESTS_DIR/input/vanderbei* ./$TESTS_DIR/input/v2* ./$TESTS_DIR/input/445k21* ./$TESTS_DIR/input/cycle.txt)
            mode="diff"
            ;;
        --vanderbei)
            inputs+=(./$TESTS_DIR/input/vanderbei*)
            mode="diff"
            ;;
        --vol2)
            inputs+=(./$TESTS_DIR/input/v2*)
            mode="diff"
            ;;
        --netlib)
            inputs+=(./$TESTS_DIR/input/netlib*)
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

(if [ "$opt_level" = "release" ]; then $cargo_exe build --release; else $cargo_exe build; fi) || exit
execpath=./target/$opt_level/bblp

for input in $inputs
do
    output=$(sed -E 's/input/output/' <<< "$input")

    case "$mode" in
        pure)
            $execpath < "$input"
            ;;
        cmp)
            time $execpath "$input" $flags
            cat "$output"
            echo
            ;;
        diff)
            echo -n "\e[1m${input:t:r} \e[0m"
            [[ "$err_out" = "stderr" ]] && echo
            my_result=$($execpath "$input" $flags 2>/dev/$err_out)
            diff=$(git --no-pager diff --no-index =(echo $my_result) "$output")
            [[ "$?" = "0" ]] && echo "\e[1m\e[32mOK\e[0m" || (echo && echo "$diff" | diff-so-fancy)
            [[ "$err_out" = "stderr" ]] && echo
            ;;
        esac
done
