#!/bin/bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_PATH="$(dirname "$SCRIPT_DIR")/ironveil"
BIN=${1:-server}

if [ "$EUID" -ne 0 ]; then
    echo "not running as root, relaunching w/ sudo"
    sudo bash "$0" "$BIN"
    exit
fi

echo "running as root"
cd "$PROJECT_PATH"
cargo run --bin $BIN
