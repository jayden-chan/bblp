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

NAME=bblp

mkdir $NAME
cp -r src ./lp_tests .cargo vendor bundle.sh fulltest.sh install.sh Cargo.lock Cargo.toml Makefile README.md $NAME
tar -jcvf $NAME.tar.bz2 $NAME

rm -rf $NAME
