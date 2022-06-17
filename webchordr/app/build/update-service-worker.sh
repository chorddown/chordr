#!/usr/bin/env bash

if [[ -z ${TRUNK_STAGING_DIR+x} ]]; then
  echo "[ERROR] \$TRUNK_STAGING_DIR is not set"
  exit 1
fi

cd "$TRUNK_STAGING_DIR" || exit 1

export LC_CTYPE=en_US.UTF-8
export LC_ALL=en_US.UTF-8

SERVICE_WORKER_FILE=$TRUNK_STAGING_DIR/service-worker.js

RANDOM_ID=$(openssl rand -hex 12)
perl -i -pe"s+{RANDOM_ID}+$RANDOM_ID+g" "$SERVICE_WORKER_FILE"

JS_PATH=("$TRUNK_STAGING_DIR"/index-*.js)
JS_FILE_NAME=$(basename "${JS_PATH[@]}")
perl -i -pe"s+//{JS}+'/$JS_FILE_NAME',+g" "$SERVICE_WORKER_FILE"

WASM_PATH=("$TRUNK_STAGING_DIR"/index-*_bg.wasm)
WASM_FILE_NAME=$(basename "${WASM_PATH[@]}")
perl -i -pe"s+//{WASM}+'/$WASM_FILE_NAME',+g" "$SERVICE_WORKER_FILE"

SORTABLE_PATH=(snippets/webchordr-song-list-*/dist/sortable.js)
SORTABLE_FILE_NAME="${SORTABLE_PATH[*]}"
perl -i -pe"s+//{SORTABLE}+'/$SORTABLE_FILE_NAME',+g" "$SERVICE_WORKER_FILE"

INDEX_FILE=$TRUNK_STAGING_DIR/index.html
RANDOM_ID=$(openssl rand -hex 12)
perl -i -pe"s+{RANDOM_ID}+$RANDOM_ID+g" "$INDEX_FILE"
