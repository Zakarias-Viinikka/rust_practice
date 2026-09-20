#!/usr/bin/env bash
set -e

if [ "$1" = "run" ] && [ "$2" = "web" ]; then
    ROOT="$(cd "$(dirname "$0")" && pwd)"
    xfce4-terminal --title="trunk serve" -x bash -c "export PATH=\"\$HOME/.cargo/bin:\$PATH\"; cd '$ROOT/web_client' && trunk serve; exec bash"
    exit 0
fi

exec cargo "$@"
