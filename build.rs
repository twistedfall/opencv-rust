use std::collections::HashSet;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::Instant;
use std::{env, io, iter};

use once_cell::sync::{Lazy, OnceCell};
use semver::{Version, VersionReq};

use library::Library;

use crate::docs::{handle_running_in_docsrs, GenerateFullBindings};

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
static HOST_TRIPLE: Lazy<Option<String>> = Lazy::new(|| env::var("HOST_TRIPLE").ok());

static OPENCV_BRANCH_32: Lazy<VersionReq> =
	Lazy::new(|| VersionReq::parse("~3.2").expect("Can't parse OpenCV 3.2 version requirement"));
static OPENCV_BRANCH_34: Lazy<VersionReq> =
	Lazy::new(|| VersionReq::parse("~3.4").expect("Can't parse OpenCV 3.4 version requirement"));
static OPENCV_BRANCH_4: Lazy<VersionReq> =
	Lazy::new(|| VersionReq::parse("~4").expect("Can't parse OpenCV 4 version requirement"));

static ENV_VARS: [&str; 16] = [
	"OPENCV_PACKAGE_NAME",
	"OPENCV_PKGCONFIG_NAME",
	"OPENCV_CMAKE_NAME",
	"OPENCV_CMAKE_BIN",
	"OPENCV_VCPKG_NAME",
	"OPENCV_LINK_LIBS",
	"OPENCV_LINK_PATHS",
	"OPENCV_INCLUDE_PATHS",
	"OPENCV_DISABLE_PROBES",
	"CMAKE_PREFIX_PATH",
	"OpenCV_DIR",
	"PKG_CONFIG_PATH",
	"VCPKG_ROOT",
	"VCPKGRS_DYNAMIC",
	"OCVRS_DOCS_GENERATE_DIR",
	"DOCS_RS",
];

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

			// strip "lib" prefix from the filename
			if env::var("CARGO_CFG_TARGET_ENV").unwrap() != "msvc" {
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
						major = Some(version.to_string());
					}
					"MINOR" => {
						minor = Some(version.to_string());
					}
					"REVISION" => {
						revision = Some(version.to_string());
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
		Some(Version::new(major.parse().ok()?, minor.parse().ok()?, revision.parse().ok()?))
	} else {
		None
	}
}

fn make_modules(opencv_dir: &Path) -> Result<()> {
	let enable_modules = IntoIterator::into_iter(["core".to_string()])
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
		if cfg!(target_vendor = "apple") {
			out.flag_if_supported(&format!("-F{}", p.to_str().expect("Can't convert path to str")));
		}
	});
	if cfg!(target_env = "msvc") {
		out.flag_if_supported("-std=c++14"); // clang says error: 'auto' return without trailing return type; deduced return types are a C++14 extension
	}
	if out.get_compiler().is_like_msvc() {
		out.flag("-std:c++latest")
			.flag("-EHsc")
			.flag("-bigobj")
			.flag("-wd4996")
			.flag("-wd5054") // deprecated between enumerations of different types
			.flag("-wd4190") // has C-linkage specified, but returns UDT 'Result<cv::Rect_<int>>' which is incompatible with C
			.flag("-wd4702") // core.cpp(386) : unreachable code
			.pic(false);
	} else {
		out.flag("-std=c++11").flag_if_supported("-Wa,-mbig-obj");
	}
	out
}

fn build_job_server() -> Option<jobserver::Client> {
	unsafe { jobserver::Client::from_env() }
		.and_then(|c| {
			let available_jobs = c.available().unwrap_or(0);
			if available_jobs > 0 {
				eprintln!("=== Using environment job server with the the amount of available jobs: {available_jobs}");
				Some(c)
			} else {
				eprintln!(
					"=== Available jobs from the environment created jobserver is: {available_jobs} or there is an error reading that value"
				);
				None
			}
		})
		.or_else(|| {
			let num_jobs = env::var("NUM_JOBS")
				.ok()
				.and_then(|jobs| jobs.parse().ok())
				.unwrap_or(2)
				.max(1);
			eprintln!("=== Creating a new job server with num_jobs: {num_jobs}");
			jobserver::Client::new(num_jobs).ok()
		})
}

// todo: replace by https://github.com/rust-lang/cargo/issues/9096 when stable
fn build_clang_generator() -> io::Result<Child> {
	let cargo_bin = PathBuf::from(env::var_os("CARGO").unwrap_or_else(|| "cargo".into()));
	let mut cargo = Command::new(cargo_bin);
	// generator script is quite slow in debug mode, so we force it to be built in release mode
	cargo
		.args([
			"build",
			"--release",
			"--package",
			"opencv-binding-generator",
			"--bin",
			"binding-generator",
		])
		.env("CARGO_TARGET_DIR", &*OUT_DIR);
	if let Some(host_triple) = HOST_TRIPLE.as_ref() {
		cargo.args(["--target", host_triple]);
	}
	println!("=== Running: {:?}", &cargo);
	cargo.stdout(Stdio::piped());
	cargo.stderr(Stdio::piped());
	cargo.spawn()
}

fn setup_rerun() -> Result<()> {
	for &v in ENV_VARS.iter() {
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

	let job_server = build_job_server().ok_or("Can't create job server")?;
	let generator_build = build_clang_generator()?;

	eprintln!("=== Crate version: {:?}", env::var_os("CARGO_PKG_VERSION"));
	eprintln!("=== Environment configuration:");
	for v in ENV_VARS.iter().copied().chain(iter::once("PATH")) {
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

	generator::gen_wrapper(opencv_header_dir, &opencv, job_server, generator_build)?;
	build_wrapper(&opencv);
	// -l linker args should be emitted after -l static
	opencv.emit_cargo_metadata();
	Ok(())
}
