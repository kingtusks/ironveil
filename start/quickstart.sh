#!/bin/bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

if [ "$EUID" -ne 0 ]; then
    echo "not running as root, relaunching w/ sudo"
    sudo bash "$0" "$@"
    exit
fi

echo "running as root"
cargo run --bin ironveil