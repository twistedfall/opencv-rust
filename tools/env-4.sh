script_dir="$(dirname "$(readlink -f "$BASH_SOURCE")")"

. "$script_dir/config.sh"

export OpenCV_DIR="$OPENCV_4_CMAKE_DIR"
export LD_LIBRARY_PATH="$OPENCV_4_LD_LIBRARY_PATH"
export OPENCV_INCLUDE_PATHS="+,$OPENCV_4_ADDITIONAL_INCLUDE_DIRS"
export OPENCV_DISABLE_PROBES=pkg_config
