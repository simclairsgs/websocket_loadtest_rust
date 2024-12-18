#!/bin/bash
echo "SGS => Starting load Test"
cargo build --release
cp target/release/mgserver_loadtest_impl ./loadtest.bin
nohup ./loadtest.bin > loadtest.log &