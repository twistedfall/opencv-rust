#!/bin/bash

set -eu

script_dir="$(dirname "$(readlink -f "$BASH_SOURCE")")"

cd "$script_dir"

./regen.sh 4
./regen.sh 5

rm -rf target/release/build/opencv-*
