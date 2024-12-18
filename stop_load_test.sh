#!/bin/bash
echo "SGS => Stopping load Test"
spwd=$1
echo spwd

pkill -9 loadtest.bin
ps aux | grep loadtest.bin

remote=172.20.79.146
sshpass -p spwd ssh sas@$remote "sh ~/george/loadtest/websocket_loadtest_rust/stop_load_slave.sh"