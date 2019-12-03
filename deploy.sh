#!/usr/bin/env bash

[[ x"$1" == x"" ]] && (echo "need target dir"; exit 1)

cargo build --release --target x86_64-pc-windows-gnu

[[ "$?" != "0" ]] && exit $?

rm -rf $1
mkdir $1

# if command -v progress &>/dev/null; then
  # CP=progress
# else
  # CP="cp -rv"
# fi

cp -r target/x86_64-pc-windows-gnu/release/backtest-run.exe config $1 &
pid=$!

if command -v progress &>/dev/null; then
  progress --pid $pid
fi

wait
