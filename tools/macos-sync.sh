#!/bin/bash

set -e

script_dir="$(dirname "$(readlink -f "$BASH_SOURCE")")"

. "$script_dir/config.sh"

cd "$script_dir/.."

rsync -av --progress --exclude "/target" --exclude "/.idea" ./ "$MACOS_ADDR:opencv-rust/"
