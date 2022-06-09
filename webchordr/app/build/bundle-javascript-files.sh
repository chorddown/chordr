#!/usr/bin/env bash

if [[ -z ${TRUNK_SOURCE_DIR+x} ]]; then
  echo "[ERROR] \$TRUNK_SOURCE_DIR is not set"
  exit 1
fi

if ! command -v rollup &>/dev/null; then
  echo "[WARN] Rollup could not be found"
  echo "For more information visit https://rollupjs.org"
  exit 1
fi

cd "$TRUNK_SOURCE_DIR" || exit 1
cmd="rollup static/javascripts/main.js --file static/javascripts/bundle.js --format iife"

if [[ "$1" == "-v" ]]; then
  $cmd
else
  $cmd &>/dev/null
fi
