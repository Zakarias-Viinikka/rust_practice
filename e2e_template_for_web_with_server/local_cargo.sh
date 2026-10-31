#!/usr/bin/env bash
set -e

if [ "$1" = "run" ] || [ "$1" = "r" ]; then
    shift
    if [ "$1" = "r" ] || [ "$1" = "release" ] || [ "$1" = "--release" ]; then
        case "$1" in
            r|release) shift ;;
            --release) shift ;;
        esac
        exec cargo run --release -p orchestrator "$@"
    fi
    exec cargo run -p orchestrator "$@"
fi

exec cargo "$@"
