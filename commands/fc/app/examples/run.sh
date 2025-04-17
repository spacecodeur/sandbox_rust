#!/bin/bash

show_help() {
    echo "Usage: ./commands.sh fc/app/examples/run <example>"
    echo "  example           : the example's directory name"
}

if [[ $# -lt 1 ]]; then
    echo "Error: One required parameter is missing."
    show_help
    exit 1
fi

EXAMPLE=$1

cargo run --example $EXAMPLE