#!/usr/bin/env bash

get_script_dir() {
    local SOURCE_PATH="${BASH_SOURCE[0]}"
    local SYMLINK_DIR
    local SCRIPT_DIR
    # Resolve symlinks recursively
    while [ -L "$SOURCE_PATH" ]; do
        # Get symlink directory
        SYMLINK_DIR="$(cd -P "$(dirname "$SOURCE_PATH")" >/dev/null 2>&1 && pwd)"
        # Resolve symlink target (relative or absolute)
        SOURCE_PATH="$(readlink "$SOURCE_PATH")"
        # Check if candidate path is relative or absolute
        if [[ $SOURCE_PATH != /* ]]; then
            # Candidate path is relative, resolve to full path
            SOURCE_PATH=$SYMLINK_DIR/$SOURCE_PATH
        fi
    done
    # Get final script directory path from fully resolved source path
    SCRIPT_DIR="$(cd -P "$(dirname "$SOURCE_PATH")" >/dev/null 2>&1 && pwd)"
    echo "$SCRIPT_DIR"
}

if [[ -z ${TRUNK_STAGING_DIR+x} ]]; then
    BUILD_DIR=$(get_script_dir)"/../dist"
else
    BUILD_DIR=${TRUNK_STAGING_DIR}
fi

cd "$BUILD_DIR" || exit 1

export LC_CTYPE=en_US.UTF-8
export LC_ALL=en_US.UTF-8

SERVICE_WORKER_FILE=$BUILD_DIR/service-worker.js

RANDOM_ID=$(openssl rand -hex 12)
perl -i -pe"s+{RANDOM_ID}+$RANDOM_ID+g" "$SERVICE_WORKER_FILE"

JS_PATH=("$BUILD_DIR"/webchordr-*.js)
JS_FILE_NAME=$(basename "${JS_PATH[@]}")
if [[ "$JS_FILE_NAME" == "index-*.wasm" ]]; then
    echo "[ERROR] Could not find file matching 'index-*_bg.wasm'"
    exit 1
fi
perl -i -pe"s+//{JS}+'/$JS_FILE_NAME',+g" "$SERVICE_WORKER_FILE"

WASM_PATH=("$BUILD_DIR"/webchordr-*_bg.wasm)
WASM_FILE_NAME=$(basename "${WASM_PATH[@]}")
if [[ "$WASM_FILE_NAME" == "index-*_bg.wasm" ]]; then
    echo "[ERROR] Could not find file matching 'index-*_bg.wasm'"
    exit 1
fi
perl -i -pe"s+//{WASM}+'/$WASM_FILE_NAME',+g" "$SERVICE_WORKER_FILE"

SORTABLE_PATH=(snippets/webchordr-song-list-*/dist/sortable.js)
SORTABLE_FILE_NAME="${SORTABLE_PATH[*]}"
perl -i -pe"s+//{SORTABLE}+'/$SORTABLE_FILE_NAME',+g" "$SERVICE_WORKER_FILE"

INDEX_FILE=$BUILD_DIR/index.html
RANDOM_ID=$(openssl rand -hex 12)
perl -i -pe"s+{RANDOM_ID}+$RANDOM_ID+g" "$INDEX_FILE"
