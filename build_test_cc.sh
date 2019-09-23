#!/bin/bash

CONCEPT=$1

[ -z "$CONCEPT" ] && echo "first argument must be core concept name eg. hello_holo" && exit 1

if [ ! -d "coreconcepts_tuts" ]; then
  git clone --depth 1 --branch test https://github.com/freesig/coreconcepts_tuts.git
fi

cd coreconcepts_tuts
../utility/single_source code ../coreconcepts/$CONCEPT.md zomes/hello/code/src/lib.rs rust
hc package
if [ "${?}" -gt 0 ]; then
  echo ${CONCEPT}
  exit 1
fi
#./update_hash.sh
cd ..
utility/single_source md coreconcepts/$CONCEPT.md docs/coreconcepts/$CONCEPT.md
