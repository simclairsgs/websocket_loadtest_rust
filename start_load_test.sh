#!/bin/bash
echo "SGS => Starting load Test"
spwd=$1
echo spwd
cargo build --release
cp target/release/mgserver_loadtest_impl ./loadtest.bin
# Prepare Sync
remote=172.20.79.146
sshpass -p spwd ssh sas@$remote "mkdir -p ~/george/loadtest/websocket_loadtest_rust" && sshpass -p spwd scp loadtest.bin start_load_slave.sh stop_load_slave.sh sas@$remote:~/george/loadtest/websocket_loadtest_rust/
# start local
nohup ./loadtest.bin > loadtest.log &

#start remote
sshpass -p spwd ssh sas@$remote "sh ~/george/loadtest/websocket_loadtest_rust/start_load_slave.sh"