#!/bin/bash
echo "SGS => Stopping load Test"
#test_ips=("172.20.73.171" "172.20.79.146" "172.20.74.79" "172.20.74.80")   # DEBIAN 11
test_ips=("10.62.31.57" "172.20.93.65" "172.20.90.154" "172.20.63.20")
spwd=$1
echo spwd

pkill -9 loadtest.bin
ps aux | grep loadtest.bin

#remote=172.20.79.146

for remote in "${test_ips[@]}";
do
        echo "STOP IN $remote"
        sshpass -P "password" -p "$spwd" ssh sas@$remote "sh ~/george/loadtest/websocket_loadtest_rust/stop_load_slave.sh"
done
#ssh ~/george/loadtest/websocket_loadtest_rust/stop_load_slave.sh"
