script_dir="$(dirname "$(readlink -f "$BASH_SOURCE")")"

. "$script_dir/config.sh"

export OpenCV_DIR="$OPENCV_5_CMAKE_DIR"
export LD_LIBRARY_PATH="$OPENCV_5_LD_LIBRARY_PATH"
export OPENCV_INCLUDE_PATHS="+,$OPENCV_5_ADDITIONAL_INCLUDE_DIRS"
