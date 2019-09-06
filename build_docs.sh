#!/bin/bash

BRANCH=$1
FOLDER=$2

[ -z "$BRANCH" ] && echo "first argument must be holochain-rust branch/tag name e.g. v0.0.25-alpha2" && exit 1
[ -z "$FOLDER" ] && echo "Second argument must be folder name, usually 'latest' or version number, e.g. 0.0.25-alpha2" && exit 1

if [ ! -d "build/api/$FOLDER" ] || [ ! -d "build/guide/$FOLDER" ]; then
  git clone --depth 1 --branch $BRANCH https://github.com/holochain/holochain-rust.git
fi
# api reference
if [ ! -d "build/api/$FOLDER" ]; then
  cargo doc --no-deps --manifest-path holochain-rust/Cargo.toml --target-dir build/api/$FOLDER
  rm -rf build/api/$FOLDER/debug
  mv -v build/api/$FOLDER/doc/* build/api/$FOLDER/
  rm -rf build/api/$FOLDER/doc
fi


# guidebook
if [ ! -d "build/guide/$FOLDER" ]; then
  cargo install --version 0.2.2 mdbook
  mdbook build holochain-rust/doc/holochain_101 --dest-dir ../../../build/guide/$FOLDER
fi
