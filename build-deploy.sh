#!/usr/bin/env bash

cd $(dirname $0);

if [[ "$1" == "" ]];then
  echo "[ERROR] Missing argument 1 'rsync target'";
  exit 1;
fi

echo "[TASK] Build the catalog"
cargo run --bin chordr -- build-catalog webchordr/static/songs webchordr/static/catalog.json -p

echo "[TASK] Create deploy-build"
pushd webchordr
if hash yarn 2>/dev/null; then
  yarn build
else
  npm run build
fi
popd

echo "[TASK] Upload to $1"
rsync --progress -rzu webchordr/dist/ $1
