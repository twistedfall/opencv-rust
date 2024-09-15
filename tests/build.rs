use std::path::PathBuf;

use build::cmake_probe::{CmakeProbe, LinkLib, LinkSearch};
use build::library::Linkage;

#[allow(dead_code)]
#[path = "../build.rs"]
mod build;

#[test]
fn test_extract_from_cmdline() {
	{
		let cmdline = "/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/c++ -g -arch arm64 -isysroot /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX13.3.sdk -Wl,-search_paths_first -Wl,-headerpad_max_install_names CMakeFiles/ocvrs_probe.dir/ocvrs_probe.cpp.o -o ocvrs_probe  /vcpkg/installed/arm64-osx/debug/lib/libopencv_calib3d4d.a /vcpkg/installed/arm64-osx/debug/lib/libopencv_core4d.a /vcpkg/installed/arm64-osx/debug/lib/libjpeg.a /vcpkg/installed/arm64-osx/debug/lib/libtiffd.a /vcpkg/installed/arm64-osx/debug/lib/liblzma.a /vcpkg/installed/arm64-osx/debug/lib/libjpeg.a /vcpkg/installed/arm64-osx/debug/lib/libtiffd.a -lm -framework AppKit -framework Accelerate -framework AVFoundation -framework CoreGraphics /vcpkg/installed/arm64-osx/debug/lib/libopencv_calib3d4d.a /vcpkg/installed/arm64-osx/debug/lib/libopencv_features2d4d.a /vcpkg/installed/arm64-osx/debug/lib/libz.a -framework OpenCL /vcpkg/installed/arm64-osx/debug/lib/manual-link/opencv4_thirdparty/libtegra_hald.a";
		let expect_include_paths: Vec<PathBuf> = vec![];
		let expect_link_paths = vec![
			LinkSearch(Linkage::Static, PathBuf::from("/vcpkg/installed/arm64-osx/debug/lib")),
			LinkSearch(
				Linkage::Static,
				PathBuf::from("/vcpkg/installed/arm64-osx/debug/lib/manual-link/opencv4_thirdparty"),
			),
		];
		let expect_link_libs = vec![
			LinkLib(Linkage::Static, "opencv_calib3d4d".to_string()),
			LinkLib(Linkage::Static, "opencv_core4d".to_string()),
			LinkLib(Linkage::Static, "jpeg".to_string()),
			LinkLib(Linkage::Static, "tiffd".to_string()),
			LinkLib(Linkage::Static, "lzma".to_string()),
			LinkLib(Linkage::Static, "jpeg".to_string()),
			LinkLib(Linkage::Static, "tiffd".to_string()),
			LinkLib(Linkage::Default, "m".to_string()),
			LinkLib(Linkage::Framework, "AppKit".to_string()),
			LinkLib(Linkage::Framework, "Accelerate".to_string()),
			LinkLib(Linkage::Framework, "AVFoundation".to_string()),
			LinkLib(Linkage::Framework, "CoreGraphics".to_string()),
			LinkLib(Linkage::Static, "opencv_calib3d4d".to_string()),
			LinkLib(Linkage::Static, "opencv_features2d4d".to_string()),
			LinkLib(Linkage::Static, "z".to_string()),
			LinkLib(Linkage::Framework, "OpenCL".to_string()),
			LinkLib(Linkage::Static, "tegra_hald".to_string()),
		];
		assert_extract_from_cmdline(cmdline, true, expect_include_paths, expect_link_paths, expect_link_libs);
	}

	{
		let cmdline = "/usr/bin/c++ -g CMakeFiles/ocvrs_probe.dir/ocvrs_probe.cpp.o -o ocvrs_probe  /usr/lib/x86_64-linux-gnu/libopencv_stitching.so.3.4.20 /usr/lib/x86_64-linux-gnu/libopencv_superres.so.3.4.20 /usr/lib/x86_64-linux-gnu/libopencv_videostab.so.3.4.20";
		let expect_include_paths: Vec<PathBuf> = vec![];
		let expect_link_paths = vec![LinkSearch(Linkage::Default, PathBuf::from("/usr/lib/x86_64-linux-gnu"))];
		let expect_link_libs = vec![
			LinkLib(Linkage::Default, "opencv_stitching".to_string()),
			LinkLib(Linkage::Default, "opencv_superres".to_string()),
			LinkLib(Linkage::Default, "opencv_videostab".to_string()),
		];
		assert_extract_from_cmdline(cmdline, true, expect_include_paths, expect_link_paths, expect_link_libs);
	}

	{
		let cmdline = "/home/pro/projects/opencv-lib/opencv-4/install-static/lib64/libopencv_calib3d.a  /home/pro/projects/opencv-lib/opencv-4/install-static/lib64/libopencv_core.a  /home/pro/projects/opencv-lib/opencv-4/install-static/lib64/libopencv_dnn.a  /home/pro/projects/opencv-lib/opencv-4/install-static/lib64/libopencv_features2d.a  /home/pro/projects/opencv-lib/opencv-4/install-static/lib64/libopencv_flann.a  /home/pro/projects/opencv-lib/opencv-4/install-static/lib64/opencv4/3rdparty/libade.a  /usr/lib64/libfreetype.so  /usr/lib64/libharfbuzz.so  -lm  -lOgreBites  -lOgreMeshLodGenerator  /home/pro/projects/opencv-lib/opencv-4/install-static/lib64/opencv4/3rdparty/libopencv.sfm.correspondence.a  /home/pro/projects/opencv-lib/opencv-4/install-static/lib64/opencv4/3rdparty/libopencv.sfm.multiview.a  -lglog::glog  -lgflags_shared  /home/pro/projects/opencv-lib/opencv-4/install-static/lib64/libopencv_phase_unwrapping.a  -lVTK::FiltersExtraction  -lVTK::FiltersSources  /home/pro/projects/opencv-lib/opencv-4/install-static/lib64/libopencv_videoio.a  -lIconv::Iconv  /home/pro/projects/opencv-lib/opencv-4/install-static/lib64/opencv4/3rdparty/liblibpng.a  /home/pro/projects/opencv-lib/opencv-4/install-static/lib64/opencv4/3rdparty/liblibtiff.a  /home/pro/projects/opencv-lib/opencv-4/install-static/lib64/opencv4/3rdparty/liblibopenjp2.a  -lva  -lva-drm  /usr/lib64/libOpenGL.so  /home/pro/projects/opencv-lib/opencv-4/install-static/lib64/opencv4/3rdparty/libittnotify.a  -ldl  -lm  -lpthread  -lrt  /home/pro/projects/opencv-lib/opencv-4/install-static/lib64/opencv4/3rdparty/libippiw.a  /home/pro/projects/opencv-lib/opencv-4/install-static/lib64/opencv4/3rdparty/libippicv.a  -lEigen3::Eigen ";
		let expect_include_paths: Vec<PathBuf> = vec![];
		let expect_link_paths = vec![
			LinkSearch(
				Linkage::Static,
				PathBuf::from("/home/pro/projects/opencv-lib/opencv-4/install-static/lib64"),
			),
			LinkSearch(
				Linkage::Static,
				PathBuf::from("/home/pro/projects/opencv-lib/opencv-4/install-static/lib64/opencv4/3rdparty"),
			),
			LinkSearch(Linkage::Default, PathBuf::from("/usr/lib64")),
		];
		let expect_link_libs = vec![
			LinkLib(Linkage::Static, "opencv_calib3d".to_string()),
			LinkLib(Linkage::Static, "opencv_core".to_string()),
			LinkLib(Linkage::Static, "opencv_dnn".to_string()),
			LinkLib(Linkage::Static, "opencv_features2d".to_string()),
			LinkLib(Linkage::Static, "opencv_flann".to_string()),
			LinkLib(Linkage::Static, "ade".to_string()),
			LinkLib(Linkage::Default, "freetype".to_string()),
			LinkLib(Linkage::Default, "harfbuzz".to_string()),
			LinkLib(Linkage::Default, "m".to_string()),
			LinkLib(Linkage::Default, "OgreBites".to_string()),
			LinkLib(Linkage::Default, "OgreMeshLodGenerator".to_string()),
			LinkLib(Linkage::Static, "opencv.sfm.correspondence".to_string()),
			LinkLib(Linkage::Static, "opencv.sfm.multiview".to_string()),
			LinkLib(Linkage::Static, "opencv_phase_unwrapping".to_string()),
			LinkLib(Linkage::Static, "opencv_videoio".to_string()),
			LinkLib(Linkage::Static, "libpng".to_string()),
			LinkLib(Linkage::Static, "libtiff".to_string()),
			LinkLib(Linkage::Static, "libopenjp2".to_string()),
			LinkLib(Linkage::Default, "va".to_string()),
			LinkLib(Linkage::Default, "va-drm".to_string()),
			LinkLib(Linkage::Default, "OpenGL".to_string()),
			LinkLib(Linkage::Static, "ittnotify".to_string()),
			LinkLib(Linkage::Default, "dl".to_string()),
			LinkLib(Linkage::Default, "m".to_string()),
			LinkLib(Linkage::Default, "pthread".to_string()),
			LinkLib(Linkage::Default, "rt".to_string()),
			LinkLib(Linkage::Static, "ippiw".to_string()),
			LinkLib(Linkage::Static, "ippicv".to_string()),
		];
		assert_extract_from_cmdline(cmdline, false, expect_include_paths, expect_link_paths, expect_link_libs);
	}
}

#[track_caller]
fn assert_extract_from_cmdline(
	cmdline: &str,
	skip_cmd: bool,
	expect_include_paths: Vec<PathBuf>,
	expect_link_paths: Vec<LinkSearch>,
	expect_link_libs: Vec<LinkLib>,
) {
	let mut include_paths = vec![];
	let mut link_paths = vec![];
	let mut link_libs = vec![];
	CmakeProbe::extract_from_cmdline(cmdline, skip_cmd, &mut include_paths, &mut link_paths, &mut link_libs);
	assert_eq!(expect_include_paths, include_paths);
	assert_eq!(expect_link_paths, link_paths);
	assert_eq!(expect_link_libs, link_libs);
}
