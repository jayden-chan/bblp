#!/bin/zsh
# vim: ft=sh

NAME=lp

mkdir $NAME
cp -r src ./lp_tests .cargo vendor bundle.sh fulltest.sh install.sh Cargo.lock Cargo.toml Makefile README.md $NAME
tar -jcvf $NAME.tar.bz2 $NAME

rm -rf $NAME
