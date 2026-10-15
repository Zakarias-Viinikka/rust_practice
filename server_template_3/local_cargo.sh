#!/usr/bin/env bash
set -e

ROOT="$(cd "$(dirname "$0")" && pwd)"
CLIENT="$ROOT/client_side"

if [ "$1" = "run" ]; then
    shift
    case "$PWD" in
        "$CLIENT"*)
            cd "$ROOT/client_side/test_web_client"
            exec trunk serve "$@"
            ;;
        *)
            exec cargo run -p server "$@"
            ;;
    esac
fi

if [ "$1" = "web" ]; then
    shift
    cd "$ROOT/client_side/test_web_client"
    exec trunk serve "$@"
fi

exec cargo "$@"