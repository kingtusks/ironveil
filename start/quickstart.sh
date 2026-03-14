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

open_terminal() {
    local cmd="$1"
    if command -v gnome-terminal &>/dev/null; then
        gnome-terminal -- bash -c "$cmd; read"
    elif command -v konsole &>/dev/null; then
        konsole -e bash -c "$cmd; read"
    elif command -v xterm &>/dev/null; then
        xterm -e bash -c "$cmd; read" &
    else
        eval "$cmd" &
    fi
}

if [ "$BIN" = "both" ]; then
    echo "starting server and client in separate terminals"
    open_terminal "cd '$PROJECT_PATH' && cargo run --bin server"
    sleep 2
    open_terminal "cd '$PROJECT_PATH' && cargo run --bin client"
elif [ "$BIN" = "cleanup" ]; then
    echo "cleaning up routes."
    ip route del 0.0.0.0/1 2>/dev/null
    ip route del 128.0.0.0/1 2>/dev/null
    echo "done"
else
    cargo run --bin $BIN
fi