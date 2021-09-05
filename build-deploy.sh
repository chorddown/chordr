#!/usr/bin/env bash

cd $(dirname $0)

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
cargo run --bin chordr -- build-catalog webchordr/static/songs webchordr/static/catalog.json -p

echo "[TASK] Create deploy-build"
pushd webchordr
if [[ $* == *--dev* ]]; then
  command="build:dev"
else
  command="build"
fi
if hash yarn 2>/dev/null; then
  yarn ${command}
else
  npm run ${command}
fi
popd

echo "[TASK] Upload to $1"
rsync -i --exclude '*.scss' -rzu webchordr/dist/ $1
