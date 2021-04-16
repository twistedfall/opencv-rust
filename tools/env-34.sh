script_dir="$(dirname "$(readlink -f "$BASH_SOURCE")")"

. "$script_dir/config.sh"

export OpenCV_DIR="$OPENCV_34_CMAKE_DIR"
export LD_LIBRARY_PATH="$OPENCV_34_LD_LIBRARY_PATH"
