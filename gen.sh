#!/usr/bin/env bash

if [[ $# == 2 ]]; then
    out_dir=$1
    indi="$2"
elif [[ -f $1 ]]; then
    out_dir=`basename $(dirname $1)`
    indi=`basename $1`
elif [[ -f confirm/generate/$1 ]]; then
    out_dir=`basename $(dirname $1)`
    indi=`basename $1`
else
    echo "something went wrong with the params"
    exit 1
fi
echo "$indi"

# if [[ ! -f config/generate/$1/$2 ]]; then
    # echo config/generate/$1/$2 does not exist
    # exit 1
# fi

RUST_BACKTRACE=1
RUST_LOG=debug

workdir="/home/stefan/.wine/drive_c/Program Files/MetaTrader 5"

# generate the signal
cargo run -- gen -i config/indicator/$out_dir config/generate/$out_dir/$indi

if [[ $? != 0 ]]; then
    echo -e "\ngenerate failed"
    exit 1
fi

pushd "$workdir"

# watch the compiler output
tail -Fn0 MQL5/Experts/BacktestExpert/nnfx-ea/nnfx-ea.log &
    # grep --color=always i error &
tail_pid=$!

# compile the EA
wine64 ~/.wine/drive_c/Program\ Files/MetaTrader\ 5/metaeditor64.exe \
    /compile:"MQL5/Experts/BacktestExpert/nnfx-ea/nnfx-ea.mq5" \
    /include:"MQL5" \
    /log \
    &>/dev/null

ret=$?

kill %%
wait

if [[ $ret == 0 ]]; then
    echo -e "\ncompilation failed"
    exit 1
fi

popd
command cp -f config/run_test.yaml /tmp/run.yaml
echo "  confirm: config/indicator/$out_dir/$indi" >> /tmp/run.yaml

# run a single backtest with the new signal as confirmation
cargo run -- -c config/config_single.yaml run /tmp/run.yaml

