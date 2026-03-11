#!/bin/bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_PATH="$(dirname "$SCRIPT_DIR")/ironveil"
BIN=${1:-both}

if [ "$EUID" -ne 0 ]; then
    echo "not running as root, relaunching w/ sudo"
    sudo bash "$0" "$BIN"
    exit
fi

echo "running as root"
cd "$PROJECT_PATH"

if [ "$BIN" = "both" ]; then
    echo "starting server and client in separate terminals..."
    mintty -e bash -c "cd '$PROJECT_PATH' && cargo run --bin server; read" &
    sleep 2  
    mintty -e bash -c "cd '$PROJECT_PATH' && cargo run --bin client; read" &
else
    cargo run --bin $BIN
fi