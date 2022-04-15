opencv_lib_base_dir="<home of the separate opencv install>"
BINDINGS_OUT_DIR="$opencv_lib_base_dir/bindings"

# 3.2
OPENCV_32_PKG_CONFIG_PATH="$opencv_lib_base_dir/opencv-3.2/install/lib/pkgconfig/"
OPENCV_32_HEADER_DIR="$opencv_lib_base_dir/opencv-3.2/install/include/"
OPENCV_32_OPENCV_PACKAGE_NAME="opencv"
OPENCV_32_LD_LIBRARY_PATH="$opencv_lib_base_dir/opencv-3.2/install/lib/"
OPENCV_32_OPENCV_INCLUDE_PATHS=+/usr/include/eigen3/

# 3.4
OPENCV_34_CMAKE_DIR="$opencv_lib_base_dir/opencv-3.4/install/share/OpenCV"
OPENCV_34_HEADER_DIR="$opencv_lib_base_dir/opencv-3.4/install/include/"
OPENCV_34_LD_LIBRARY_PATH="$opencv_lib_base_dir/opencv-3.4/install/lib64/"

# 4.x
OPENCV_4_CMAKE_DIR="$opencv_lib_base_dir/opencv-4/install/lib64/cmake/opencv4"
OPENCV_4_HEADER_DIR="$opencv_lib_base_dir/opencv-4/install/include/opencv4"
OPENCV_4_LD_LIBRARY_PATH="$opencv_lib_base_dir/opencv-4/install/lib64/"
OPENCV_4_OPENCV_INCLUDE_PATHS=+/opt/cuda/targets/x86_64-linux/include/

# other OS machines
MACOS_ADDR="<ssh address for macos machine>"
WIN_ADDR="<ssh address for win machine>"
