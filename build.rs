use std::collections::{HashMap, HashSet};
use std::env;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::time::Instant;

use binding_generator::handle_running_binding_generator;
use docs::handle_running_in_docsrs;
use generator::BindingGenerator;
use header::IncludePath;
use library::Library;
use once_cell::sync::Lazy;
use semver::{Version, VersionReq};

#[path = "build/binding-generator.rs"]
mod binding_generator;
#[path = "build/cmake_probe.rs"]
pub mod cmake_probe;
#[path = "build/docs.rs"]
mod docs;
#[path = "build/generator.rs"]
mod generator;
#[path = "build/header.rs"]
mod header;
#[path = "build/library.rs"]
pub mod library;

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

// replace `Lazy` with `LazyLock` when MSRV is 1.80.0
static OUT_DIR: Lazy<PathBuf> = Lazy::new(|| PathBuf::from(env::var_os("OUT_DIR").expect("Can't read OUT_DIR env var")));
static MANIFEST_DIR: Lazy<PathBuf> =
	Lazy::new(|| PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("Can't read CARGO_MANIFEST_DIR env var")));
static SRC_DIR: Lazy<PathBuf> = Lazy::new(|| MANIFEST_DIR.join("src"));
static SRC_CPP_DIR: Lazy<PathBuf> = Lazy::new(|| MANIFEST_DIR.join("src_cpp"));
static TARGET_ENV_MSVC: Lazy<bool> = Lazy::new(|| env::var("CARGO_CFG_TARGET_ENV").is_ok_and(|target_env| target_env == "msvc"));
static TARGET_VENDOR_APPLE: Lazy<bool> =
	Lazy::new(|| env::var("CARGO_CFG_TARGET_VENDOR").is_ok_and(|target_vendor| target_vendor == "apple"));

static OPENCV_BRANCH_34: Lazy<VersionReq> =
	Lazy::new(|| VersionReq::parse("~3.4").expect("Can't parse OpenCV 3.4 version requirement"));
static OPENCV_BRANCH_4: Lazy<VersionReq> =
	Lazy::new(|| VersionReq::parse("~4").expect("Can't parse OpenCV 4 version requirement"));
static OPENCV_BRANCH_5: Lazy<VersionReq> =
	Lazy::new(|| VersionReq::parse("~5").expect("Can't parse OpenCV 5 version requirement"));

/// Environment vars that affect the build, the source will be rebuilt if those change, the contents of those vars will also
/// be present in the debug log
static AFFECTING_ENV_VARS: [&str; 18] = [
	"OPENCV_PACKAGE_NAME",
	"OPENCV_PKGCONFIG_NAME",
	"OPENCV_CMAKE_NAME",
	"OPENCV_CMAKE_BIN",
	"OPENCV_VCPKG_NAME",
	"OPENCV_LINK_LIBS",
	"OPENCV_LINK_PATHS",
	"OPENCV_INCLUDE_PATHS",
	"OPENCV_DISABLE_PROBES",
	"OPENCV_MSVC_CRT",
	"CMAKE_PREFIX_PATH",
	"OpenCV_DIR",
	"PKG_CONFIG_PATH",
	"VCPKG_ROOT",
	"VCPKGRS_DYNAMIC",
	"VCPKGRS_TRIPLET",
	"OCVRS_DOCS_GENERATE_DIR",
	"DOCS_RS",
];

static SUPPORTED_MODULES: [&str; 73] = [
	"3d",
	"alphamat",
	"aruco",
	"aruco_detector",
	"barcode",
	"bgsegm",
	"bioinspired",
	"calib",
	"calib3d",
	// "cannops",
	"ccalib",
	"core",
	"cudaarithm",
	"cudabgsegm",
	"cudacodec",
	"cudafeatures2d",
	"cudafilters",
	"cudaimgproc",
	"cudalegacy",
	"cudaobjdetect",
	"cudaoptflow",
	"cudastereo",
	"cudawarping",
	// "cudev",
	"cvv",
	"dnn",
	"dnn_superres",
	"dpm",
	"face",
	"features",
	"features2d",
	"flann",
	"freetype",
	"fuzzy",
	"gapi",
	"hdf",
	"hfs",
	"highgui",
	"img_hash",
	"imgcodecs",
	"imgproc",
	"intensity_transform",
	"line_descriptor",
	"mcc",
	"ml",
	"objdetect",
	"optflow",
	"ovis",
	"phase_unwrapping",
	"photo",
	"plot",
	"quality",
	"rapid",
	"rgbd",
	"saliency",
	"sfm",
	"shape",
	"signal",
	"stereo",
	"stitching",
	"structured_light",
	"superres",
	"surface_matching",
	"text",
	"tracking",
	"video",
	"videoio",
	"videostab",
	"viz",
	"wechat_qrcode",
	"xfeatures2d",
	"ximgproc",
	"xobjdetect",
	"xphoto",
	"xstereo",
];

static SUPPORTED_INHERENT_FEATURES: [&str; 2] = ["hfloat", "opencl"];

/// The contents of these vars will be present in the debug log, but will not cause the source rebuild
static DEBUG_ENV_VARS: [&str; 1] = ["PATH"];

pub enum GenerateFullBindings {
	Stop,
	Proceed,
}

fn files_with_predicate<'p>(
	dir: &Path,
	mut predicate: impl FnMut(&Path) -> bool + 'p,
) -> Result<impl Iterator<Item = PathBuf> + 'p> {
	Ok(dir
		.read_dir()?
		.flatten()
		.filter_map(|e| e.file_type().is_ok_and(|typ| typ.is_file()).then(|| e.path()))
		.filter(move |p| predicate(p)))
}

fn files_with_extension<'e>(dir: &Path, extension: impl AsRef<OsStr> + 'e) -> Result<impl Iterator<Item = PathBuf> + 'e> {
	files_with_predicate(dir, move |p| {
		p.extension().is_some_and(|e| e.eq_ignore_ascii_case(extension.as_ref()))
	})
}

fn make_modules_and_alises(
	opencv_dir: &Path,
	opencv_version: &Version,
) -> Result<(Vec<String>, HashMap<&'static str, &'static str>)> {
	let enable_modules = ["core".to_string()]
		.into_iter()
		.chain(env::vars_os().filter_map(|(k, _)| {
			k.to_str()
				.and_then(|s| s.strip_prefix("CARGO_FEATURE_"))
				.map(str::to_lowercase)
		}))
		.collect::<HashSet<_>>();

	let mut modules = files_with_extension(opencv_dir, "hpp")?
		.filter_map(|entry| {
			entry
				.file_stem()
				.and_then(OsStr::to_str)
				.filter(|&m| enable_modules.contains(m))
				.map(str::to_string)
		})
		.collect::<Vec<_>>();

	let aliases = if OPENCV_BRANCH_5.matches(opencv_version)
		&& modules.iter().any(|x| x == "features2d")
		&& modules.iter().any(|x| x == "features")
	{
		// In OpenCV 5 `features2d` is a compatibility header that just includes `features.hpp`, and they don't work together
		HashMap::from([("features2d", "features")])
	} else {
		HashMap::new()
	};

	modules.sort_unstable();
	Ok((modules, aliases))
}

fn emit_inherent_features(opencv: &Library) {
	if VersionReq::parse(">=4.10")
		.expect("Static version requirement")
		.matches(&opencv.version)
	{
		println!("cargo::rustc-cfg=ocvrs_has_inherent_feature_hfloat");
	}
	for feature in &opencv.enabled_features {
		if SUPPORTED_INHERENT_FEATURES.contains(&feature.as_str()) {
			println!("cargo::rustc-cfg=ocvrs_has_inherent_feature_{feature}");
		}
	}
}

fn make_compiler(opencv: &Library, ffi_export_suffix: &str) -> cc::Build {
	let mut out = cc::Build::new();
	out.cpp(true)
		.std("c++17") // clang says error: 'auto' return without trailing return type; deduced return types are a C++14 extension
		.include(&*SRC_CPP_DIR)
		.include(&*OUT_DIR)
		.include(".")
		// OpenCV warnings
		.flag_if_supported("-Wno-deprecated-declarations") // declarations marked as CV_DEPRECATED
		.flag_if_supported("-Wno-deprecated-copy") // implicitly-declared ‘constexpr cv::MatStep::MatStep(const cv::MatStep&)’ is deprecated
		.flag_if_supported("-Wno-unused-parameter") // unused parameter ‘src’ in virtual void cv::dnn::dnn4_v20211004::ActivationLayer::forwardSlice(const float*, float*, int, size_t, int, int) const
		.flag_if_supported("-Wno-sign-compare") // comparison of integer expressions of different signedness: ‘size_t’ {aka ‘long unsigned int’} and ‘int’ in bool cv::dnn::dnn4_v20211004::isAllOnes(const MatShape&, int, int)
		.flag_if_supported("-Wno-comment") // multi-line comment in include/opencv4/opencv2/mcc/ccm.hpp:73:25
		.flag_if_supported("-Wno-uninitialized") // /usr/include/opencv4/opencv2/gapi/render/render_types.hpp:164:30: warning: 'ret.cv::gapi::wip::draw::Circle::radius' is used uninitialized
		// crate warnings
		.flag_if_supported("-Wno-unused-variable") // ‘cv::CV_VERSION_OCVRS_OVERRIDE’ defined but not used
		.flag_if_supported("-Wno-ignored-qualifiers") // type qualifiers ignored on function return type in const size_t cv_MatStep_operator___const_int(const cv::MatStep* instance, int i)
		.flag_if_supported("-Wno-return-type-c-linkage") // warning: 'cv_aruco_CharucoBoard_getChessboardSize_const' has C-linkage specified, but returns user-defined type 'Result<cv::Size>' (aka 'Result<Size_<int> >') which is incompatible with C
		.flag_if_supported("-Wno-overloaded-virtual")
		.flag_if_supported("-Wno-delete-non-virtual-dtor"); // warning: deleting object of abstract class type 'cv::cudacodec::NVSurfaceToColorConverter' which has non-virtual destructor will cause undefined behavior

	opencv.include_paths.iter().for_each(|p| {
		out.include(p);
		if *TARGET_VENDOR_APPLE {
			// Weirdly causes issues on macOS: https://github.com/twistedfall/opencv-rust/issues/620
			// MSRV: replace with `reason` when MSRV is 1.81.0
			#[allow(clippy::needless_borrows_for_generic_args)]
			out.flag_if_supported(&format!("-F{}", p.to_str().expect("Can't convert path to str")));
		}
	});

	if out.get_compiler().is_like_msvc() {
		if let Ok(crt) = env::var("OPENCV_MSVC_CRT") {
			if crt.trim().to_lowercase() == "dynamic" {
				out.static_crt(false);
			} else if crt.trim().to_lowercase() == "static" {
				out.static_crt(true);
			} else {
				panic!("Invalid value of OPENCV_MSVC_CRT var, expected \"static\" or \"dynamic\"");
			}
		}
		out.flag("-EHsc")
			.flag("-bigobj")
			.flag("-utf-8")
			.flag("-wd4996")
			.flag("-wd5054") // deprecated between enumerations of different types
			.flag("-wd4190") // has C-linkage specified, but returns UDT 'Result<cv::Rect_<int>>' which is incompatible with C
			.flag("-wd4702") // core.cpp(386) : unreachable code
			.flag("-wd4100") // unreferenced formal parameter
			.flag("-wd4127") // conditional expression is constant
			.pic(false);
	} else {
		out.flag_if_supported("-Wa,-mbig-obj");
	}
	out.define("OCVRS_FFI_EXPORT_SUFFIX", ffi_export_suffix);
	out
}

fn setup_rerun() -> Result<()> {
	for &v in AFFECTING_ENV_VARS.iter() {
		println!("cargo::rerun-if-env-changed={v}");
	}

	let include_exts = &[OsStr::new("cpp"), OsStr::new("hpp")];
	let files_with_include_exts =
		files_with_predicate(&SRC_CPP_DIR, |p| p.extension().is_some_and(|e| include_exts.contains(&e)))?;
	for path in files_with_include_exts {
		if let Some(path) = path.to_str() {
			println!("cargo::rerun-if-changed={path}");
		}
	}
	println!("cargo::rerun-if-changed=Cargo.toml");
	Ok(())
}

fn build_wrapper(mut cc: cc::Build, modules: &[String], module_aliases: &HashMap<&str, &str>) {
	eprintln!("=== Compiler information: {:#?}", cc.get_compiler());
	for module in modules.iter().filter(|m| !module_aliases.contains_key(m.as_str())) {
		cc.file(OUT_DIR.join(format!("{module}.cpp")));
		let manual_cpp = SRC_CPP_DIR.join(format!("manual-{module}.cpp"));
		if manual_cpp.exists() {
			cc.file(manual_cpp);
		}
	}
	let start = Instant::now();
	cc.compile("ocvrs");
	eprintln!("=== Total cpp build time: {:?}", start.elapsed());
}

fn main() -> Result<()> {
	let args = env::args_os().skip(1).peekable();
	if matches!(handle_running_binding_generator(args)?, GenerateFullBindings::Stop) {
		return Ok(());
	}

	for branch in ["34", "4", "5"] {
		println!("cargo::rustc-check-cfg=cfg(ocvrs_opencv_branch_{branch})");
	}
	for module in SUPPORTED_MODULES {
		println!("cargo::rustc-check-cfg=cfg(ocvrs_has_module_{module})");
	}
	for inherent_feature in SUPPORTED_INHERENT_FEATURES {
		println!("cargo::rustc-check-cfg=cfg(ocvrs_has_inherent_feature_{inherent_feature})");
	}

	if matches!(handle_running_in_docsrs(), GenerateFullBindings::Stop) {
		return Ok(());
	}

	let pkg_version = env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "unknown_crate_version".to_string());
	eprintln!("=== Crate version: {pkg_version}");
	eprintln!("=== Environment configuration:");
	for v in AFFECTING_ENV_VARS.into_iter().chain(DEBUG_ENV_VARS) {
		eprintln!("===   {v} = {:?}", env::var_os(v));
	}
	eprintln!("=== Enabled features:");
	for (mut name, val) in env::vars() {
		if val == "1" {
			const PREFIX: &str = "CARGO_FEATURE_";
			if name.starts_with(PREFIX) {
				name.drain(..PREFIX.len());
				eprintln!("===   {name}");
			}
		}
	}

	let opencv = Library::probe()?;
	eprintln!("=== OpenCV library configuration: {opencv:#?}");
	if OPENCV_BRANCH_5.matches(&opencv.version) {
		println!("cargo::rustc-cfg=ocvrs_opencv_branch_5");
	} else if OPENCV_BRANCH_4.matches(&opencv.version) {
		println!("cargo::rustc-cfg=ocvrs_opencv_branch_4");
	} else if OPENCV_BRANCH_34.matches(&opencv.version) {
		println!("cargo::rustc-cfg=ocvrs_opencv_branch_34");
	} else {
		panic!(
			"Unsupported OpenCV version: {}, must be from 3.4, 4.x or 5.x branch",
			opencv.version
		);
	}

	let opencv_header_dir = opencv
		.include_paths
		.iter()
		.find(|p| p.get_version_header().is_some())
		.expect("Discovered OpenCV include paths do not contain valid OpenCV headers");

	if let Some(header_version) = opencv_header_dir.find_version() {
		if header_version != opencv.version {
			panic!(
				"OpenCV version from the headers: {header_version} (at {}) must match version of the OpenCV library: {} (include paths: {:?})",
				opencv_header_dir.display(),
				opencv.version,
				opencv.include_paths,
			);
		}
		eprintln!(
			"=== Found OpenCV version: {header_version} in headers located at: {}",
			opencv_header_dir.display()
		);
	} else {
		panic!(
			"Unable to find OpenCV version in headers located at: {}",
			opencv_header_dir.display()
		)
	}

	let opencv_module_header_dir = opencv_header_dir
		.get_module_header_dir()
		.expect("Can't find OpenCV module header dir");
	eprintln!(
		"=== Detected OpenCV module header dir at: {}",
		opencv_module_header_dir.display()
	);
	let (modules, module_aliases) = make_modules_and_alises(&opencv_module_header_dir, &opencv.version)?;
	for module in &modules {
		println!("cargo::rustc-cfg=ocvrs_has_module_{module}");
	}

	emit_inherent_features(&opencv);

	setup_rerun()?;

	let ffi_export_suffix = format!("_{}", pkg_version.replace(".", "_"));
	let build_script_path = env::current_exe()?;
	let binding_generator = BindingGenerator::new(&build_script_path, &modules, &module_aliases);
	binding_generator.generate_wrapper(opencv_header_dir, &opencv, &ffi_export_suffix)?;
	let cc = make_compiler(&opencv, &ffi_export_suffix);
	build_wrapper(cc, &modules, &module_aliases);
	// -l linker args should be emitted after -l static
	opencv.emit_cargo_metadata();
	Ok(())
}
