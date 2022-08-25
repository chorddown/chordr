#!/usr/bin/env bash
set -e
cd "$(dirname "$0")" || exit

if ! command -v trunk &>/dev/null; then
  echo "[ERROR] Trunk must be installed (https://trunkrs.dev/#install)"
  exit 1
fi

if [[ "$1" == "" ]]; then
  echo "[ERROR] Missing argument 1 'rsync target'"
  exit 1
fi

if pgrep trunk >/dev/null; then
  echo "*"
  echo "*"
  echo "[WARN] A running trunk program was detected. This may lead to unexpected side effects"
  echo "*"
  echo "*"
  echo
fi
echo "[TASK] Build the catalog"
cargo run --bin chordr -- build-catalog webchordr/app/static/songs webchordr/app/static/catalog.json

echo "[TASK] Create deploy-build"
pushd webchordr/app || exit 1

if [[ $* == *--dev* ]]; then
  if [[ $* == *--verbose* ]]; then
    trunk -v build
  else
    trunk build
  fi
else
  if [[ $* == *--verbose* ]]; then
    trunk -v build --release
  else
    trunk build --release
  fi
fi

if [[ $* == *--verbose* ]] && type twiggy &>/dev/null; then
  twiggy top -n 10 ./dist/*.wasm
fi
popd >/dev/null
echo "[TASK] Upload to $1"
rsync -i --exclude '*.scss' -rzu webchordr/app/dist/ $1
