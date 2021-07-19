project(OpenCVProbe)

cmake_minimum_required(VERSION 3.0)

find_package(${OCVRS_PACKAGE_NAME} REQUIRED)

message("OCVRS_INCLUDE_DIRS:${OpenCV_INCLUDE_DIRS}")
message("OCVRS_VERSION:${OpenCV_VERSION}")

include_directories(${OpenCV_INCLUDE_DIRS})
add_executable(ocvrs_probe ocvrs_probe.cpp)
target_link_libraries(ocvrs_probe ${OpenCV_LIBS})
