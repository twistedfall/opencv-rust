#!/bin/bash

set -eu

script_dir="$(dirname "$(readlink -f "$BASH_SOURCE")")"

cd "$script_dir"

./regen.sh 34
./regen.sh 4

rm -rf target/release/build/opencv-*
