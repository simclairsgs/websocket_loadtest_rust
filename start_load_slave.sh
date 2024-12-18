#!/bin/bash
echo "SGS => Starting load Test"
nohup ~/george/loadtest/websocket_loadtest_rust/loadtest.bin > loadtest.log &
