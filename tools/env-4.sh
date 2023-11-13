script_dir="$(dirname "$(readlink -f "$BASH_SOURCE")")"

. "$script_dir/config.sh"

export OpenCV_DIR="$OPENCV_4_CMAKE_DIR"
export LD_LIBRARY_PATH="$OPENCV_4_LD_LIBRARY_PATH"
