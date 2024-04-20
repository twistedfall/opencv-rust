use std::path::PathBuf;

use build::cmake_probe::CmakeProbe;

#[allow(dead_code)]
#[path = "../build.rs"]
mod build;

#[test]
fn test_extract_from_cmdline() {
	let cmdline = "/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/c++ -g -arch arm64 -isysroot /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX13.3.sdk -Wl,-search_paths_first -Wl,-headerpad_max_install_names CMakeFiles/ocvrs_probe.dir/ocvrs_probe.cpp.o -o ocvrs_probe  /vcpkg/installed/arm64-osx/debug/lib/libopencv_calib3d4d.a /vcpkg/installed/arm64-osx/debug/lib/libopencv_core4d.a /vcpkg/installed/arm64-osx/debug/lib/libopencv_features2d4d.a /vcpkg/installed/arm64-osx/debug/lib/libopencv_flann4d.a /vcpkg/installed/arm64-osx/debug/lib/libopencv_highgui4d.a /vcpkg/installed/arm64-osx/debug/lib/libopencv_imgcodecs4d.a /vcpkg/installed/arm64-osx/debug/lib/libopencv_imgproc4d.a /vcpkg/installed/arm64-osx/debug/lib/libopencv_ml4d.a /vcpkg/installed/arm64-osx/debug/lib/libopencv_objdetect4d.a /vcpkg/installed/arm64-osx/debug/lib/libopencv_photo4d.a /vcpkg/installed/arm64-osx/debug/lib/libopencv_stitching4d.a /vcpkg/installed/arm64-osx/debug/lib/libopencv_video4d.a /vcpkg/installed/arm64-osx/debug/lib/libopencv_videoio4d.a /vcpkg/installed/arm64-osx/debug/lib/libopencv_imgcodecs4d.a /vcpkg/installed/arm64-osx/debug/lib/libjpeg.a /vcpkg/installed/arm64-osx/debug/lib/libtiffd.a /vcpkg/installed/arm64-osx/debug/lib/liblzma.a /vcpkg/installed/arm64-osx/debug/lib/libjpeg.a /vcpkg/installed/arm64-osx/debug/lib/libtiffd.a /vcpkg/installed/arm64-osx/debug/lib/liblzma.a -lm -framework AppKit -framework Accelerate -framework AVFoundation -framework CoreGraphics -framework CoreMedia -framework CoreVideo -framework QuartzCore -framework Cocoa /vcpkg/installed/arm64-osx/debug/lib/libopencv_calib3d4d.a /vcpkg/installed/arm64-osx/debug/lib/libopencv_features2d4d.a /vcpkg/installed/arm64-osx/debug/lib/libopencv_flann4d.a /vcpkg/installed/arm64-osx/debug/lib/libopencv_imgproc4d.a /vcpkg/installed/arm64-osx/debug/lib/libopencv_core4d.a /vcpkg/installed/arm64-osx/debug/lib/libz.a -framework OpenCL /vcpkg/installed/arm64-osx/debug/lib/manual-link/opencv4_thirdparty/libtegra_hald.a";
	let mut include_paths = Vec::new();
	let mut link_paths = Vec::new();
	let mut link_libs = Vec::new();
	CmakeProbe::extract_from_cmdline(cmdline, &mut include_paths, &mut link_paths, &mut link_libs);
	let expect_include_paths: Vec<PathBuf> = vec![];
	assert_eq!(expect_include_paths, include_paths);
	let expect_link_paths = vec![
		PathBuf::from("/vcpkg/installed/arm64-osx/debug/lib"),
		PathBuf::from("/vcpkg/installed/arm64-osx/debug/lib/manual-link/opencv4_thirdparty"),
	];
	assert_eq!(expect_link_paths, link_paths);
	let expect_link_libs = vec![
		"opencv_calib3d4d",
		"opencv_core4d",
		"opencv_features2d4d",
		"opencv_flann4d",
		"opencv_highgui4d",
		"opencv_imgcodecs4d",
		"opencv_imgproc4d",
		"opencv_ml4d",
		"opencv_objdetect4d",
		"opencv_photo4d",
		"opencv_stitching4d",
		"opencv_video4d",
		"opencv_videoio4d",
		"opencv_imgcodecs4d",
		"jpeg",
		"tiffd",
		"lzma",
		"jpeg",
		"tiffd",
		"lzma",
		"m",
		"AppKit.framework",
		"Accelerate.framework",
		"AVFoundation.framework",
		"CoreGraphics.framework",
		"CoreMedia.framework",
		"CoreVideo.framework",
		"QuartzCore.framework",
		"Cocoa.framework",
		"opencv_calib3d4d",
		"opencv_features2d4d",
		"opencv_flann4d",
		"opencv_imgproc4d",
		"opencv_core4d",
		"z",
		"OpenCL.framework",
		"tegra_hald",
	];
	assert_eq!(expect_link_libs, link_libs);
}
