use std::{
	collections::HashSet,
	env,
	ffi::OsStr,
	fs::File,
	io::{BufRead, BufReader},
	path::{Path, PathBuf},
	process::Command,
};

use glob::glob;
use once_cell::sync::{Lazy, OnceCell};
use semver::{Version, VersionReq};

use library::Library;

#[path = "build/cmake_probe.rs"]
mod cmake_probe;
#[path = "build/generator.rs"]
mod generator;
#[path = "build/library.rs"]
mod library;

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

static MODULES: OnceCell<Vec<String>> = OnceCell::new();

static OUT_DIR: Lazy<PathBuf> = Lazy::new(|| PathBuf::from(env::var_os("OUT_DIR").expect("Can't read OUT_DIR env var")));
static MANIFEST_DIR: Lazy<PathBuf> = Lazy::new(|| PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("Can't read CARGO_MANIFEST_DIR env var")));
static SRC_DIR: Lazy<PathBuf> = Lazy::new(|| MANIFEST_DIR.join("src"));
static SRC_CPP_DIR: Lazy<PathBuf> = Lazy::new(|| MANIFEST_DIR.join("src_cpp"));
static HOST_TRIPLE: Lazy<Option<String>> = Lazy::new(|| env::var("HOST_TRIPLE").ok());

static OPENCV_BRANCH_32: Lazy<VersionReq> = Lazy::new(|| VersionReq::parse("~3.2").expect("Can't parse OpenCV 3.2 version requirement"));
static OPENCV_BRANCH_34: Lazy<VersionReq> = Lazy::new(|| VersionReq::parse("~3.4").expect("Can't parse OpenCV 3.4 version requirement"));
static OPENCV_BRANCH_4: Lazy<VersionReq> = Lazy::new(|| VersionReq::parse("~4").expect("Can't parse OpenCV 4 version requirement"));

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
	"OPENCV_MODULE_WHITELIST",
	"OPENCV_MODULE_BLACKLIST",
	"CMAKE_PREFIX_PATH",
	"OpenCV_DIR",
	"PKG_CONFIG_PATH",
	"VCPKG_ROOT",
	"VCPKGRS_DYNAMIC",
];

fn cleanup_lib_filename(filename: &OsStr) -> Option<&OsStr> {
	let mut strip_performed = false;
	let mut filename_path = Path::new(filename);
	// used to check for the file extension (with dots stripped) and for the part of the filename
	const LIB_EXTS: [&str; 7] = [".so.", ".a.", ".dll.", ".lib.", ".dylib.", ".framework.", ".tbd."];
	if let (Some(stem), Some(extension)) = (filename_path.file_stem(), filename_path.extension().and_then(OsStr::to_str)) {
		if LIB_EXTS.iter().any(|e| e.trim_matches('.').eq_ignore_ascii_case(extension)) {
			filename_path = Path::new(stem);
			strip_performed = true;
		}
	}

	if let Some(mut file) = filename_path.file_name().and_then(OsStr::to_str) {
		let orig_len = file.len();
		file = file.strip_prefix("lib").unwrap_or(file);
		LIB_EXTS.iter()
			.for_each(|&inner_ext| if let Some(inner_ext_idx) = file.find(inner_ext) {
				file = &file[..inner_ext_idx];
			});
		if orig_len != file.len() {
			strip_performed = true;
			filename_path = Path::new(file);
		}
	}
	if strip_performed {
		Some(filename_path.as_os_str())
	} else {
		None
	}
}

fn get_version_header(header_dir: &Path) -> Option<PathBuf> {
	let out = header_dir.join("opencv2/core/version.hpp");
	if out.is_file() {
		Some(out)
	} else {
		let out = header_dir.join("Headers/core/version.hpp");
		if out.is_file() {
			Some(out)
		} else {
			None
		}
	}
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
		Some(Version::new(0, 0, 0))
	}
}

fn make_modules(opencv_dir: &Path) -> Result<()> {
	let ignore_modules: HashSet<&'static str> = [
		"core_detect",
		"cudalegacy",
		"cudev",
		"gapi",
		"opencv",
		"opencv_modules",
	].iter().copied().collect();
	let env_whitelist = env::var("OPENCV_MODULE_WHITELIST").ok();
	let env_whitelist = env_whitelist.as_ref()
		.map(|wl| wl.split(',')
			.map(|e| e.trim())
			.collect::<HashSet<_>>()
		);

	let env_blacklist = env::var("OPENCV_MODULE_BLACKLIST").ok();
	let env_blacklist = env_blacklist.as_ref()
		.map(|wl| wl.split(',')
			.map(|e| e.trim())
			.collect::<HashSet<_>>()
		);

	let modules: Vec<String> = glob(&format!("{}/*.hpp", opencv_dir.to_str().ok_or("Can't OpenCV header directory to UTF-8 string")?))?
		.filter_map(|entry| {
			let entry = entry.expect("Can't get path for module file");
			let module = entry.file_stem()
				.and_then(OsStr::to_str).expect("Can't calculate file stem");
			Some(module)
				.filter(|m| !ignore_modules.contains(m))
				.filter(|m| env_blacklist.as_ref().map_or(true, |bl| !bl.contains(m)))
				.filter(|m| env_whitelist.as_ref().map_or(true, |wl| wl.contains(m)))
				.map(str::to_string)
		})
		.collect();

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
		.flag_if_supported("-Wno-ignored-qualifiers") //  type qualifiers ignored on function return type in const size_t cv_MatStep_operator___const_int(const cv::MatStep* instance, int i)
	;
	opencv.include_paths.iter().for_each(|p| { out.include(p); });
	if cfg!(target_env = "msvc") {
		out.flag_if_supported("-std:c++latest")
			.flag_if_supported("-EHsc")
			.flag_if_supported("-bigobj")
			.flag_if_supported("-wd4996")
			.flag_if_supported("-wd5054") // deprecated between enumerations of different types
			.flag_if_supported("-wd4190") // has C-linkage specified, but returns UDT 'Result<cv::Rect_<int>>' which is incompatible with C
			.pic(false)
		;
	} else {
		out.flag("-std=c++11")
			.flag_if_supported("-Wa,-mbig-obj")
		;
	}
	out
}

fn setup_rerun() -> Result<()> {
	for &v in ENV_VARS.iter() {
		println!("cargo:rerun-if-env-changed={}", v);
	}

	let include_exts = &[OsStr::new("cpp"), OsStr::new("hpp")];
	for entry in SRC_CPP_DIR.read_dir()?.map(|e| e.unwrap()) {
		let path = entry.path();
		if path.is_file() && path.extension().map_or(false, |e| include_exts.contains(&e)) {
			if let Some(path) = path.to_str() {
				println!("cargo:rerun-if-changed={}", path);
			}
		}
	}
	Ok(())
}

fn build_wrapper(opencv: &Library) {
	let mut cc = build_compiler(opencv);
	let modules = MODULES.get().expect("MODULES not initialized");
	for module in &["sys", "types"] { // special internal modules
		println!("cargo:rustc-cfg=ocvrs_has_module_{}", module);
	}
	for module in modules.iter() {
		println!("cargo:rustc-cfg=ocvrs_has_module_{}", module);
		cc.file(OUT_DIR.join(format!("{}.cpp", module)));
		let manual_cpp = SRC_CPP_DIR.join(format!("manual-{}.cpp", module));
		if manual_cpp.exists() {
			cc.file(manual_cpp);
		}
	}
	cc.compile("ocvrs");
}

fn main() -> Result<()> {
	if cfg!(feature = "docs-only") { // fake setup for docs.rs
		println!(r#"cargo:rustc-cfg=ocvrs_opencv_branch_4"#);
		for entry in SRC_DIR.join("opencv/hub").read_dir().expect("Can't read hub dir") {
			let entry = entry.expect("Can't read directory entry");
			let path = entry.path();
			if entry.file_type().map(|f| f.is_file()).unwrap_or(false)
				&& path.extension().map_or(false, |e| e == "rs") {
				if let Some(module) = path.file_stem().and_then(OsStr::to_str) {
					println!("cargo:rustc-cfg=ocvrs_has_module_{}", module);
				}
			}
		}
		return Ok(());
	}

	let generator_build = if cfg!(feature = "clang-runtime") { // start building binding generator as early as possible
		let cargo_bin = PathBuf::from(env::var_os("CARGO").unwrap_or_else(|| "cargo".into()));
		let mut cargo = Command::new(cargo_bin);
		// generator script is quite slow in debug mode, so we force it to be built in release mode
		cargo.args(&["build", "--release", "--package", "opencv-binding-generator", "--bin", "binding-generator"])
			.env("CARGO_TARGET_DIR", &*OUT_DIR);
		if let Some(host_triple) = HOST_TRIPLE.as_ref() {
			cargo.args(&["--target", host_triple]);
		}
		println!("running: {:?}", &cargo);
		Some(cargo.spawn()?)
	} else {
		None
	};

	eprintln!("=== Crate version: {:?}", env::var_os("CARGO_PKG_VERSION"));
	eprintln!("=== Environment configuration:");
	for &v in ENV_VARS.iter() {
		eprintln!("===   {} = {:?}", v, env::var_os(v));
	}
	eprintln!("=== Enabled features:");
	let features = env::vars()
		.filter_map(|(mut name, val)| {
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
		eprintln!("===   {}", feature);
	}

	let opencv = Library::probe()?;
	eprintln!("=== OpenCV library configuration: {:#?}", opencv);
	if OPENCV_BRANCH_4.matches(&opencv.version) {
		println!("cargo:rustc-cfg=ocvrs_opencv_branch_4");
	} else if OPENCV_BRANCH_34.matches(&opencv.version) {
		println!("cargo:rustc-cfg=ocvrs_opencv_branch_34");
	} else if OPENCV_BRANCH_32.matches(&opencv.version) {
		println!("cargo:rustc-cfg=ocvrs_opencv_branch_32");
	} else {
		panic!("Unsupported OpenCV version: {}, must be from 3.2, 3.4 or 4.x branch", opencv.version);
	}
	let opencv_header_dir = opencv.include_paths.iter()
		.find(|p| get_version_header(p).is_some())
		.expect("Discovered OpenCV include paths is empty or contains non-existent paths");

	make_modules(&opencv_header_dir.join("opencv2"))?;

	if let Some(header_version) = get_version_from_headers(opencv_header_dir) {
		if header_version != opencv.version {
			panic!(
				"Version from the headers: {} (at {}) doesn't match version of the OpenCV library: {} (include paths: {:?})",
				header_version,
				opencv_header_dir.display(),
				opencv.version,
				opencv.include_paths,
			);
		}
		eprintln!("=== Found OpenCV version: {} in headers located at: {}", header_version, opencv_header_dir.display());
	} else {
		panic!("Unable to find OpenCV version in headers located at: {}", opencv_header_dir.display())
	}

	setup_rerun()?;

	generator::gen_wrapper(opencv_header_dir, &opencv, generator_build)?;
	build_wrapper(&opencv);
	// -l linker args should be emitted after -l static
	opencv.emit_cargo_metadata();
	Ok(())
}
