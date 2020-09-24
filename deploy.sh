#!/usr/bin/env bash

[[ x"$1" == x"" ]] && echo "need target dir" && exit 0

cargo build --release --target x86_64-pc-windows-gnu

[[ x"$?" != x"0" ]] && exit $?

cp -fv target/x86_64-pc-windows-gnu/release/backtestd.exe "$1"
