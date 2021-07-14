#!/bin/zsh
# vim: ft=sh

NAME=lp

git clean -X -d -f
mkdir $NAME
cp -r src bundle.sh Cargo.lock Cargo.toml install.sh Makefile README.md $NAME
tar -jcvf $NAME.tar.bz2 $NAME

rm -r $NAME
