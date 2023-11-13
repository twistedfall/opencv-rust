#!/bin/bash

set -eu

script_dir="$(dirname "$(readlink -f "$BASH_SOURCE")")"

. "$script_dir/config.sh"

cd "$script_dir/.."

rsync -av --progress --delete \
	--exclude "/.idea" --exclude "/.git" --exclude "/out" --exclude "/target" --exclude "/ci/test-proj/target" \
	./ "$WIN_ADDR:/c/Users/win/opencv-rust/"
