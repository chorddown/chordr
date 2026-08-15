#!/usr/bin/env bash
set -e
cd "$(dirname "$0")" || exit

if ! command -v trunk &>/dev/null; then
  echo "[ERROR] Trunk must be installed (https://trunkrs.dev/#install)"
  exit 1
fi

usage() {
  echo "Script to build and deploy Chorddown Web

Usage: $0 [DESTINATION] [OPTIONS]

Arguments:
  [DESTINATION]  rsync destination
                 e.g. user@server:remote_path
                 e.g. ./local/path

Options:
  -h, --held                       Show this message
      --verbose                    Run 'trunk' with verbose output
      --dev                        Create a debug build
"
}

if [[ $* == *-h* ]] || [[ $* == *--help** ]]; then
  usage
  exit 1
fi

if [[ "$1" == "" ]]; then
  echo "[ERROR] Missing argument 1 'DESTINATION'"
  echo
  usage
  exit 1
elif [[ "$1" == -* ]]; then
  echo "[ERROR] Invalid argument 1 'DESTINATION'

Hint: the destination must be the first argument "
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
cargo run --bin chordr --release -- \
  build-catalog \
  webchordr/app/static/songs \
  webchordr/app/static/catalog.json

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
rsync -i --exclude '*.scss' -rzu webchordr/app/dist/ "$1"
