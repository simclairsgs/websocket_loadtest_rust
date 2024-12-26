#!/bin/bash
#test_ips=("172.20.73.171" "172.20.79.146" "172.20.74.79" "172.20.74.80")   # DEBIAN 11
test_ips=("10.62.31.57" "172.20.93.65" "172.20.90.154" "172.20.63.20") # DEBIAN 12
echo "SGS => Starting load Test"
spwd=$1
echo $spwd
cargo build --release
cp target/release/mgserver_loadtest_impl ./loadtest.bin
# Prepare Sync
#remote=172.20.79.146
#sshpass -P "password" -p "$spwd" ssh sas@$remote "mkdir -p ~/george/loadtest/websocket_loadtest_rust" && sshpass -P "password" -p "$spwd" scp loadtest.bin start_load_slave.sh stop_load_slave.sh sas@$remote:~/george/loadtest/websocket_loadtest_rust/
# start local
#nohup ./loadtest.bin > loadtest.log &

#start remote
#sshpass -P "password" -p "$spwd" ssh sas@$remote "sh ~/george/loadtest/websocket_loadtest_rust/start_load_slave.sh"

for remote in "${test_ips[@]}";
do
	echo "SYNC IN $remote"
	sshpass -P "password" -p "$spwd" ssh sas@$remote "mkdir -p ~/george/loadtest/websocket_loadtest_rust" && sshpass -P "password" -p "$spwd" scp loadtest.bin start_load_slave.sh stop_load_slave.sh sas@$remote:~/george/loadtest/websocket_loadtest_rust/
	# sshpass -P "password" -p "$spwd" ssh sas@$remote "sh ~/george/loadtest/websocket_loadtest_rust/start_load_slave.sh"
done

#sh ~/george/loadtest/websocket_loadtest_rust/start_load_slave.sh
