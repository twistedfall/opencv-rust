#!/bin/bash

set -e

script_dir="$(dirname "$(readlink -f "$BASH_SOURCE")")"

. "$script_dir/config.sh"

cd "$script_dir/.."

rsync -av --progress --delete \
	--exclude "/.idea" --exclude "/.git" --exclude "/out" --exclude "/target" --exclude "/ci/test-proj/target" --exclude "/src/opencv/hub" --exclude "/src/opencv/hub.rs" \
	./ "$MACOS_ADDR:opencv-rust/"
