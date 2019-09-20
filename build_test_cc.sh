#!/bin/bash

CONCEPT=$1

[ -z "$CONCEPT" ] && echo "first argument must be core concept name eg. hello_holo" && exit 1

if [ ! -d "coreconcepts_tuts" ]; then
  git clone --depth 1 --branch test https://github.com/freesig/coreconcepts_tuts.git
fi

cd coreconcepts_tuts
single_source code ../coreconcepts/$CONCEPT.md zomes/hello/code/src/lib.rs rust
hc package
./update_hash.sh
cd ..
single_source md coreconcepts/$CONCEPT.md docs/coreconcepts/$CONCEPT.md rust
