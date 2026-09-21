#!/usr/bin/env bash
set -e

if [ "$1" = "run" ] && [ "$2" = "web" ]; then
    ROOT="$(cd "$(dirname "$0")" && pwd)"
    cd "$ROOT/test_web_client"
    exec trunk serve
fi

exec cargo "$@"
