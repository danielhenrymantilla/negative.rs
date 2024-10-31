#!/bin/sh

set -euxo pipefail

cargo update -vw
[[ -z "$(git status --porcelain)" ]]


(cd src/proc_macros
    cargo publish
)

for i in $(seq 10)
do
    cargo publish && exit 0
    sleep 5
done
cargo publish
