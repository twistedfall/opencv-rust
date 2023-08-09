use std::collections::HashSet;
use std::env;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::time::Instant;

use once_cell::sync::{Lazy, OnceCell};
use semver::{Version, VersionReq};

use library::Library;

use crate::binding_generator::handle_running_binding_generator;
use crate::docs::handle_running_in_docsrs;
use crate::generator::BindingGenerator;

#[path = "build/binding-generator.rs"]
mod binding_generator;
#[path = "build/cmake_probe.rs"]
mod cmake_probe;
#[path = "build/docs.rs"]
mod docs;
#[path = "build/generator.rs"]
mod generator;
#[path = "build/library.rs"]
mod library;

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

static MODULES: OnceCell<Vec<String>> = OnceCell::new();

static OUT_DIR: Lazy<PathBuf> = Lazy::new(|| PathBuf::from(env::var_os("OUT_DIR").expect("Can't read OUT_DIR env var")));
static MANIFEST_DIR: Lazy<PathBuf> =
	Lazy::new(|| PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("Can't read CARGO_MANIFEST_DIR env var")));
static SRC_DIR: Lazy<PathBuf> = Lazy::new(|| MANIFEST_DIR.join("src"));
static SRC_CPP_DIR: Lazy<PathBuf> = Lazy::new(|| MANIFEST_DIR.join("src_cpp"));
static TARGET_ENV_MSVC: Lazy<bool> =
	Lazy::new(|| env::var("CARGO_CFG_TARGET_ENV").map_or(false, |target_env| target_env == "msvc"));
static TARGET_VENDOR_APPLE: Lazy<bool> =
	Lazy::new(|| env::var("CARGO_CFG_TARGET_VENDOR").map_or(false, |target_vendor| target_vendor == "apple"));
static TARGET_OS_WINDOWS: Lazy<bool> =
	Lazy::new(|| env::var("CARGO_CFG_TARGET_OS").map_or(false, |target_os| target_os == "windows"));

static OPENCV_BRANCH_32: Lazy<VersionReq> =
	Lazy::new(|| VersionReq::parse("~3.2").expect("Can't parse OpenCV 3.2 version requirement"));
static OPENCV_BRANCH_34: Lazy<VersionReq> =
	Lazy::new(|| VersionReq::parse("~3.4").expect("Can't parse OpenCV 3.4 version requirement"));
static OPENCV_BRANCH_4: Lazy<VersionReq> =
	Lazy::new(|| VersionReq::parse("~4").expect("Can't parse OpenCV 4 version requirement"));

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
		.filter_map(|e| e.file_type().map_or(false, |typ| typ.is_file()).then(|| e.path()))
		.filter(move |p| predicate(p)))
}

fn files_with_extension<'e>(dir: &Path, extension: impl AsRef<OsStr> + 'e) -> Result<impl Iterator<Item = PathBuf> + 'e> {
	files_with_predicate(dir, move |p| {
		p.extension().map_or(false, |e| e.eq_ignore_ascii_case(extension.as_ref()))
	})
}

/// Returns Some(new_file_name) if some parts of the filename were removed, None otherwise
fn cleanup_lib_filename(filename: &OsStr) -> Option<&OsStr> {
	if let Some(mut new_filename) = Path::new(filename).file_name() {
		// used to check for the file extension (with dots stripped) and for the part of the filename
		const LIB_EXTS: [&str; 7] = [".so.", ".a.", ".dll.", ".lib.", ".dylib.", ".framework.", ".tbd."];
		let filename_path = Path::new(new_filename);
		// strip lib extension from the filename
		if let (Some(stem), Some(extension)) = (filename_path.file_stem(), filename_path.extension().and_then(OsStr::to_str)) {
			if LIB_EXTS.iter().any(|e| e.trim_matches('.').eq_ignore_ascii_case(extension)) {
				new_filename = stem;
			}
		}
		if let Some(mut file) = new_filename.to_str() {
			let orig_len = file.len();

			// strip "lib" prefix from the filename unless targeting MSVC
			if !*TARGET_ENV_MSVC {
				file = file.strip_prefix("lib").unwrap_or(file);
			}

			// strip lib extension + suffix (e.g. .so.4.6.0) from the filename
			LIB_EXTS.iter().for_each(|&inner_ext| {
				if let Some(inner_ext_idx) = file.find(inner_ext) {
					file = &file[..inner_ext_idx];
				}
			});
			if orig_len != file.len() {
				new_filename = OsStr::new(file);
			}
		}
		if new_filename.len() != filename.len() {
			Some(new_filename)
		} else {
			None
		}
	} else {
		None
	}
}

fn get_module_header_dir(header_dir: &Path) -> Option<PathBuf> {
	let mut out = header_dir.join("opencv2.framework/Headers");
	if out.exists() {
		return Some(out);
	}
	out = header_dir.join("opencv2");
	if out.exists() {
		return Some(out);
	}
	None
}

fn get_version_header(header_dir: &Path) -> Option<PathBuf> {
	get_module_header_dir(header_dir)
		.map(|dir| dir.join("core/version.hpp"))
		.filter(|dir| dir.is_file())
}

fn get_version_from_headers(header_dir: &Path) -> Option<Version> {
	let version_hpp = get_version_header(header_dir)?;
	let mut major = None;
	let mut minor = None;
	let mut revision = None;
	let mut line = String::with_capacity(256);
	let mut reader = BufReader::new(File::open(version_hpp).ok()?);
	while let Ok(bytes_read) = reader.read_line(&mut line) {
		if bytes_read == 0 {
			break;
		}
		if let Some(line) = line.strip_prefix("#define CV_VERSION_") {
			let mut parts = line.split_whitespace();
			if let (Some(ver_spec), Some(version)) = (parts.next(), parts.next()) {
				match ver_spec {
					"MAJOR" => {
						major = Some(version.parse().ok()?);
					}
					"MINOR" => {
						minor = Some(version.parse().ok()?);
					}
					"REVISION" => {
						revision = Some(version.parse().ok()?);
					}
					_ => {}
				}
			}
			if major.is_some() && minor.is_some() && revision.is_some() {
				break;
			}
		}
		line.clear();
	}
	if let (Some(major), Some(minor), Some(revision)) = (major, minor, revision) {
		Some(Version::new(major, minor, revision))
	} else {
		None
	}
}

fn make_modules(opencv_dir: &Path) -> Result<()> {
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
	modules.sort_unstable();

	MODULES.set(modules).expect("Can't set MODULES cache");
	Ok(())
}

fn build_compiler(opencv: &Library) -> cc::Build {
	let mut out = cc::Build::new();
	out.cpp(true)
		.include(&*SRC_CPP_DIR)
		.include(&*OUT_DIR)
		.include(".")
		// OpenCV warnings
		.flag_if_supported("-Wno-deprecated-declarations") // declarations marked as CV_DEPRECATED
		.flag_if_supported("-Wno-deprecated-copy") // implicitly-declared ‘constexpr cv::MatStep::MatStep(const cv::MatStep&)’ is deprecated
		.flag_if_supported("-Wno-unused-parameter") // unused parameter ‘src’ in virtual void cv::dnn::dnn4_v20211004::ActivationLayer::forwardSlice(const float*, float*, int, size_t, int, int) const
		.flag_if_supported("-Wno-sign-compare") // comparison of integer expressions of different signedness: ‘size_t’ {aka ‘long unsigned int’} and ‘int’ in bool cv::dnn::dnn4_v20211004::isAllOnes(const MatShape&, int, int)
		.flag_if_supported("-Wno-comment") // multi-line comment in include/opencv4/opencv2/mcc/ccm.hpp:73:25
		// crate warnings
		.flag_if_supported("-Wno-unused-variable") // ‘cv::CV_VERSION_OCVRS_OVERRIDE’ defined but not used
		.flag_if_supported("-Wno-ignored-qualifiers") // type qualifiers ignored on function return type in const size_t cv_MatStep_operator___const_int(const cv::MatStep* instance, int i)
		.flag_if_supported("-Wno-return-type-c-linkage") // warning: 'cv_aruco_CharucoBoard_getChessboardSize_const' has C-linkage specified, but returns user-defined type 'Result<cv::Size>' (aka 'Result<Size_<int> >') which is incompatible with C
	;

	opencv.include_paths.iter().for_each(|p| {
		out.include(p);
		if *TARGET_VENDOR_APPLE {
			out.flag_if_supported(&format!("-F{}", p.to_str().expect("Can't convert path to str")));
		}
	});

	if *TARGET_ENV_MSVC {
		out.flag_if_supported("-std=c++14"); // clang says error: 'auto' return without trailing return type; deduced return types are a C++14 extension
		if let Ok(crt) = env::var("OPENCV_MSVC_CRT") {
			if crt.trim().to_lowercase() == "dynamic" {
				out.static_crt(false);
			} else if crt.trim().to_lowercase() == "static" {
				out.static_crt(true);
			} else {
				panic!("Invalid value of OPENCV_MSVC_CRT var, expected \"static\" or \"dynamic\"");
			}
		}
	}
	if out.get_compiler().is_like_msvc() {
		out.flag("-std:c++latest")
			.flag("-EHsc")
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
		out.flag("-std=c++11").flag_if_supported("-Wa,-mbig-obj");
	}
	out
}

fn setup_rerun() -> Result<()> {
	for &v in AFFECTING_ENV_VARS.iter() {
		println!("cargo:rerun-if-env-changed={v}");
	}

	let include_exts = &[OsStr::new("cpp"), OsStr::new("hpp")];
	let files_with_include_exts =
		files_with_predicate(&SRC_CPP_DIR, |p| p.extension().map_or(false, |e| include_exts.contains(&e)))?;
	for path in files_with_include_exts {
		if let Some(path) = path.to_str() {
			println!("cargo:rerun-if-changed={path}");
		}
	}
	println!("cargo:rerun-if-changed=Cargo.toml");
	Ok(())
}

fn build_wrapper(opencv: &Library) {
	let mut cc = build_compiler(opencv);
	eprintln!("=== Compiler information: {:#?}", cc.get_compiler());
	let modules = MODULES.get().expect("MODULES not initialized");
	for module in modules.iter() {
		println!("cargo:rustc-cfg=ocvrs_has_module_{module}");
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
	if matches!(handle_running_in_docsrs(), GenerateFullBindings::Stop) {
		return Ok(());
	}

	let mut args = env::args_os().peekable();
	let build_script_path = args.next().ok_or("Can't read build script path")?;
	if matches!(handle_running_binding_generator(args)?, GenerateFullBindings::Stop) {
		return Ok(());
	}

	eprintln!("=== Crate version: {:?}", env::var_os("CARGO_PKG_VERSION"));
	eprintln!("=== Environment configuration:");
	for v in AFFECTING_ENV_VARS.into_iter().chain(DEBUG_ENV_VARS) {
		eprintln!("===   {v} = {:?}", env::var_os(v));
	}
	eprintln!("=== Enabled features:");
	let features = env::vars().filter_map(|(mut name, val)| {
		if val != "1" {
			return None;
		}
		const PREFIX: &str = "CARGO_FEATURE_";
		if name.starts_with(PREFIX) {
			name.drain(..PREFIX.len());
			Some(name)
		} else {
			None
		}
	});
	for feature in features {
		eprintln!("===   {feature}");
	}

	let opencv = Library::probe()?;
	eprintln!("=== OpenCV library configuration: {opencv:#?}");
	if OPENCV_BRANCH_4.matches(&opencv.version) {
		println!("cargo:rustc-cfg=ocvrs_opencv_branch_4");
	} else if OPENCV_BRANCH_34.matches(&opencv.version) {
		println!("cargo:rustc-cfg=ocvrs_opencv_branch_34");
	} else if OPENCV_BRANCH_32.matches(&opencv.version) {
		println!("cargo:rustc-cfg=ocvrs_opencv_branch_32");
	} else {
		panic!(
			"Unsupported OpenCV version: {}, must be from 3.2, 3.4 or 4.x branch",
			opencv.version
		);
	}
	let opencv_header_dir = opencv
		.include_paths
		.iter()
		.find(|p| get_version_header(p).is_some())
		.expect("Discovered OpenCV include paths is empty or contains non-existent paths");

	let opencv_module_header_dir = get_module_header_dir(opencv_header_dir).expect("Can't find OpenCV module header dir");
	eprintln!(
		"=== Detected OpenCV module header dir at: {}",
		opencv_module_header_dir.display()
	);
	make_modules(&opencv_module_header_dir)?;

	if let Some(header_version) = get_version_from_headers(opencv_header_dir) {
		if header_version != opencv.version {
			panic!(
				"Version from the headers: {header_version} (at {}) doesn't match version of the OpenCV library: {} (include paths: {:?})",
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

	setup_rerun()?;

	let binding_generator = BindingGenerator::new(build_script_path);
	binding_generator.generate_wrapper(opencv_header_dir, &opencv)?;
	build_wrapper(&opencv);
	// -l linker args should be emitted after -l static
	opencv.emit_cargo_metadata();
	Ok(())
}
