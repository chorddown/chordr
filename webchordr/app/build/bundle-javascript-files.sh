#!/usr/bin/env bash

if [[ -z ${TRUNK_SOURCE_DIR+x} ]]; then
    echo "[ERROR] \$TRUNK_SOURCE_DIR is not set"
    exit 1
fi

INPUT=static/javascripts/main.js
OUTPUT=static/javascripts/build.js
if command -v bun &>/dev/null; then
    cmd="bun build $INPUT --outfile $OUTPUT --format iife"
elif
    command -v rollup &>/dev/null
then
    cmd="rollup $INPUT --file $OUTPUT --format iife"
else
    echo "[WARN] Rollup and bun could not be found"
    echo "For more information visit https://rollupjs.org or https://bun.com/"
    exit 1
fi

cd "$TRUNK_SOURCE_DIR" || exit 1

if [[ "$1" == "-v" ]]; then
    $cmd
else
    $cmd &>/dev/null
fi
