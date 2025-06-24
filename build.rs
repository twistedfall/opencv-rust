use std::collections::{HashMap, HashSet};
use std::env;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::time::Instant;

use binding_generator::handle_running_binding_generator;
use docs::handle_running_in_docsrs;
use enums::{InherentFeature, SUPPORTED_INHERENT_FEATURES, SUPPORTED_MODULES};
use generator::BindingGenerator;
use header::IncludePath;
use library::Library;
use once_cell::sync::Lazy;
use opencv_binding_generator::SupportedModule;
use semver::{Version, VersionReq};

#[path = "build/binding-generator.rs"]
mod binding_generator;
#[path = "build/cmake_probe.rs"]
pub mod cmake_probe;
#[path = "build/docs.rs"]
mod docs;
#[path = "build/enums.rs"]
mod enums;
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

static SUPPORTED_OPENCV_BRANCHES: [(&str, &str); 3] = [("~3.4", "34"), ("~4", "4"), ("~5", "5")];

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
) -> Result<(Vec<SupportedModule>, HashMap<SupportedModule, SupportedModule>)> {
	let enable_modules = [SupportedModule::Core]
		.into_iter()
		.chain(env::vars_os().filter_map(|(k, _)| {
			k.to_str()
				.and_then(|s| s.strip_prefix("CARGO_FEATURE_"))
				.and_then(|s| SupportedModule::try_from_opencv_name(&s.to_lowercase()))
		}))
		.collect::<HashSet<_>>();

	let mut modules = files_with_extension(opencv_dir, "hpp")?
		.filter_map(|entry| {
			entry
				.file_stem()
				.and_then(OsStr::to_str)
				.and_then(SupportedModule::try_from_opencv_name)
				.filter(|m| enable_modules.contains(m))
		})
		.collect::<Vec<_>>();

	let aliases = if opencv_version.major == 5
		&& modules.iter().any(|x| matches!(x, SupportedModule::Features2d))
		&& modules.iter().any(|x| matches!(x, SupportedModule::Features))
	{
		// In OpenCV 5 `features2d` is a compatibility header that just includes `features.hpp`, and they don't work together
		HashMap::from([(SupportedModule::Features2d, SupportedModule::Features)])
	} else {
		HashMap::new()
	};

	modules.sort_unstable();
	Ok((modules, aliases))
}

fn emit_opencv_branch(opencv: &Library) {
	let mut version_matched = false;
	for (version_req, branch) in SUPPORTED_OPENCV_BRANCHES {
		let version_matcher = VersionReq::parse(version_req).expect("Invalid version");
		if version_matcher.matches(&opencv.version) {
			println!("cargo::rustc-cfg=ocvrs_opencv_branch_{branch}");
			version_matched = true;
			break;
		}
	}
	if !version_matched {
		panic!(
			"Unsupported OpenCV version: {}, must match one of the following: {}",
			opencv.version,
			SUPPORTED_OPENCV_BRANCHES
				.iter()
				.map(|(v, _)| *v)
				.collect::<Vec<_>>()
				.join(", ")
		);
	}
}

fn emit_inherent_features(opencv: &Library) {
	let mut fake_features = Vec::with_capacity(2);
	if VersionReq::parse(">=4.10")
		.expect("Static version requirement")
		.matches(&opencv.version)
	{
		fake_features.push(InherentFeature::Hfloat);
		if VersionReq::parse(">=4.11")
			.expect("Static version requirement")
			.matches(&opencv.version)
		{
			fake_features.push(InherentFeature::AlgorithmHint);
		}
	}
	let detected_supported_inherent_features = opencv.inherent_features.iter().flat_map(|f| InherentFeature::try_from_str(f));
	for inherent_feature in detected_supported_inherent_features.chain(fake_features) {
		println!("cargo::rustc-cfg=ocvrs_has_inherent_feature_{}", inherent_feature.as_str());
	}
}

fn emit_modules(modules: &[SupportedModule]) {
	for module in modules {
		println!("cargo::rustc-cfg=ocvrs_has_module_{}", module.opencv_name());
	}
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
	out.define("OCVRS_TARGET_OS_WINDOWS", target_os_windows());
	out
}

fn target_os_windows() -> &'static str {
	if env::var("CARGO_CFG_TARGET_OS").expect("Can't read CARGO_CFG_TARGET_OS env var") == "windows" {
		"1"
	} else {
		"0"
	}
}

fn build_wrapper(mut cc: cc::Build, modules: &[SupportedModule], module_aliases: &HashMap<SupportedModule, SupportedModule>) {
	eprintln!("=== Compiler information: {:#?}", cc.get_compiler());
	for module in modules.iter().filter(|m| !module_aliases.contains_key(m)) {
		let module_opencv_name = module.opencv_name();
		cc.file(OUT_DIR.join(format!("{module_opencv_name}.cpp")));
		let manual_cpp = SRC_CPP_DIR.join(format!("manual-{module_opencv_name}.cpp"));
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

	// these need to be emitted before handle_running_in_docsrs() call
	for (_, branch) in SUPPORTED_OPENCV_BRANCHES {
		println!("cargo::rustc-check-cfg=cfg(ocvrs_opencv_branch_{branch})");
	}
	for module in SUPPORTED_MODULES {
		println!("cargo::rustc-check-cfg=cfg(ocvrs_has_module_{})", module.opencv_name());
	}
	for inherent_feature in SUPPORTED_INHERENT_FEATURES {
		println!(
			"cargo::rustc-check-cfg=cfg(ocvrs_has_inherent_feature_{})",
			inherent_feature.as_str()
		);
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
	emit_opencv_branch(&opencv);

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

	emit_inherent_features(&opencv);

	let opencv_module_header_dir = opencv_header_dir
		.get_module_header_dir()
		.expect("Can't find OpenCV module header dir");
	eprintln!(
		"=== Detected OpenCV module header dir at: {}",
		opencv_module_header_dir.display()
	);
	let (modules, module_aliases) = make_modules_and_alises(&opencv_module_header_dir, &opencv.version)?;
	emit_modules(&modules);

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
