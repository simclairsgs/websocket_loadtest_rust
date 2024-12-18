#!/bin/bash
echo "SGS => Starting load Test"
nohup ~/george/loadtest/websocket_loadtest_rust/loadtest.bin > ~/george/loadtest/websocket_loadtest_rust/loadtest.log 2>&1 </dev/null &
