#!/bin/bash

export RUSTUP_HOME=$PWD/rustup
export CARGO_HOME=$PWD/cargo
export RUSTUP_TOOLCHAIN=stable
export RUSTUP_INIT_SKIP_PATH_CHECK=1
curl https://sh.rustup.rs -sSf | sh -s -- -y --profile minimal --no-modify-path
