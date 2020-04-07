// todo add support for cmake: cmake --find-package -DNAME=OpenCV -DCOMPILER_ID=GNU -DLANGUAGE=CXX -DMODE=COMPILE/LINK

use std::{
	borrow::Cow,
	collections::HashSet,
	env,
	ffi::OsStr,
	fs::{self, File},
	io::{BufRead, BufReader},
	iter::{self, FromIterator},
	path::{Path, PathBuf},
};

use glob_crate::glob;
use once_cell::sync::{Lazy, OnceCell};
use semver::{Version, VersionReq};

#[cfg(feature = "buildtime-bindgen")]
mod generator {
	use std::{
		ffi::OsStr,
		fs::{self, DirEntry, File, OpenOptions},
		io::{self, BufRead, BufReader, Write},
		path::{Path, PathBuf},
		time::Instant,
	};

	use glob_crate::glob;
	use rayon::prelude::*;

	use super::{
		file_copy_to_dir,
		get_versioned_hub_dirs,
		is_core_module,
		MODULES,
		OUT_DIR,
		Result,
		SRC_CPP_DIR,
		SRC_DIR,
	};

	fn read_dir(path: &Path) -> Result<impl Iterator<Item=DirEntry>> {
		Ok(path.read_dir()?.filter_map(|e| e.ok()))
	}

	fn copy_indent(mut read: impl BufRead, mut write: impl Write, indent: &str) -> Result<()> {
		let mut line = Vec::with_capacity(100);
		while read.read_until(b'\n', &mut line)? != 0 {
			write.write(indent.as_bytes())?;
			write.write(&line)?;
			line.clear();
		}
		Ok(())
	}

	fn file_move_to_dir(src_file: &Path, target_dir: &Path) -> Result<PathBuf> {
		if !target_dir.exists() {
			fs::create_dir_all(&target_dir)?;
		}
		let src_filename = src_file.file_name()
			.ok_or_else(|| "Can't calculate filename")?;
		let target_file = target_dir.join(src_filename);
		if fs::rename(&src_file, &target_file).is_err() {
			fs::copy(&src_file, &target_file)?;
			fs::remove_file(src_file)?;
		}
		Ok(target_file)
	}

	pub fn gen_wrapper(opencv_header_dir: &Path) -> Result<()> {
		let out_dir_as_str = OUT_DIR.to_str().unwrap();
		let (rust_hub_dir, cpp_hub_dir) = get_versioned_hub_dirs();
		let module_dir = rust_hub_dir.join("hub");
		let manual_dir = SRC_DIR.join("manual");
		let opencv_dir = opencv_header_dir.join("opencv2");

		eprintln!("=== Using OpenCV headers from: {}", opencv_dir.display());
		eprintln!("=== Generating code in: {}", out_dir_as_str);
		eprintln!("=== Placing generated bindings into: {}", rust_hub_dir.display());

		let modules = MODULES.get().expect("MODULES not initialized");

		for entry in read_dir(&OUT_DIR)? {
			let path = entry.path();
			if path.is_file() && path.extension().and_then(OsStr::to_str).map_or(true, |ext| !ext.eq_ignore_ascii_case("dll")) {
				let _ = fs::remove_file(path);
			}
		}

		let version = if cfg!(feature = "opencv-32") {
			"3.2.0"
		} else if cfg!(feature = "opencv-34") {
			"3.4.10"
		} else if cfg!(feature = "opencv-4") {
			"4.3.0"
		} else {
			unreachable!();
		};
		let clang = clang::Clang::new().expect("Cannot initialize clang");
		let start = Instant::now();
		modules.par_iter().for_each(|module| {
			let mut module_file = SRC_CPP_DIR.join(format!("{}.hpp", module));
			if !module_file.exists() {
				module_file = opencv_dir.join(format!("{}.hpp", module));
			}
			let bindings_writer = binding_generator::writer::RustBindingWriter::new(
				&*SRC_CPP_DIR,
				&*OUT_DIR,
				&module,
				version,
				false,
			);
			binding_generator::Generator::new(&opencv_header_dir, &*SRC_CPP_DIR, module, &clang)
				.process_file(&module_file, bindings_writer);
			println!("Generated: {}", module);
		});
		println!("Total binding generation time: {:?}", start.elapsed());

		if !module_dir.exists() {
			fs::create_dir_all(&module_dir)?;
		}

		for entry in read_dir(&module_dir)? {
			let path = entry.path();
			if path.extension().map_or(false, |e| e == "rs") {
				let _ = fs::remove_file(path);
			}
		}

		if !cpp_hub_dir.exists() {
			fs::create_dir_all(&cpp_hub_dir)?;
		}
		for entry in read_dir(&cpp_hub_dir)? {
			let path = entry.path();
			if path.is_file() {
				let _ = fs::remove_file(path);
			}
		}

		for module in modules {
			let module_cpp = OUT_DIR.join(format!("{}.cpp", module));
			if module_cpp.is_file() {
				file_copy_to_dir(&module_cpp, &cpp_hub_dir)?;
				let module_types_cpp = OUT_DIR.join(format!("{}_types.hpp", module));
				let mut module_types_file = OpenOptions::new().create(true).truncate(true).write(true).open(&module_types_cpp)?;
				let mut type_files: Vec<PathBuf> = glob(&format!("{}/???-{}-*.type.cpp", out_dir_as_str, module))?
					.collect::<Result<_, glob_crate::GlobError>>()?;
				type_files.sort_unstable();
				for entry in type_files.into_iter() {
					io::copy(&mut File::open(entry)?, &mut module_types_file)?;
				}
				file_copy_to_dir(&module_types_cpp, &cpp_hub_dir)?;
			}
		}

		let add_manual = |file: &mut File, mod_name: &str| -> Result<bool> {
			if manual_dir.join(format!("{}.rs", mod_name)).exists() {
				writeln!(file, "pub use crate::manual::{}::*;", mod_name)?;
				Ok(true)
			} else {
				Ok(false)
			}
		};

		{
			let mut hub_rs = File::create(rust_hub_dir.join("hub.rs"))?;

			let mut types_rs = File::create(module_dir.join("types.rs"))?;
			writeln!(&mut types_rs)?;

			let mut sys_rs = File::create(module_dir.join("sys.rs"))?;
			writeln!(&mut sys_rs, "use crate::{{mod_prelude_types::*, core}};")?;
			writeln!(&mut sys_rs)?;
			for module in modules {
				let is_core_module = is_core_module(module.as_str());
				let write_if_contrib = |write: &mut File| -> Result<()> {
					if !is_core_module {
						writeln!(write, r#"#[cfg(feature = "contrib")]"#)?;
					}
					Ok(())
				};
				// hub
				write_if_contrib(&mut hub_rs)?;
				writeln!(&mut hub_rs, "pub mod {};", module)?;
				let module_filename = format!("{}.rs", module);
				let target_file = file_move_to_dir(&OUT_DIR.join(&module_filename), &module_dir)?;
				let mut f = OpenOptions::new().append(true).open(&target_file)?;
				add_manual(&mut f, module)?;

				// types
				let mut write_header = true;
				for entry in glob(&format!("{}/???-{}-*.type.rs", out_dir_as_str, module))? {
					let entry = entry?;
					if entry.metadata().map(|meta| meta.len()).unwrap_or(0) > 0 {
						if write_header {
							write_if_contrib(&mut types_rs)?;
							writeln!(&mut types_rs, "mod {}_types {{", module)?;
							writeln!(&mut types_rs, "\tuse crate::{{mod_prelude::*, core, types, sys}};")?;
							writeln!(&mut types_rs)?;
							write_header = false;
						}
						copy_indent(BufReader::new(File::open(&entry)?), &mut types_rs, "\t")?;
					}
				}
				if !write_header {
					writeln!(&mut types_rs, "}}")?;
					write_if_contrib(&mut types_rs)?;
					writeln!(&mut types_rs, "pub use {}_types::*;", module)?;
					writeln!(&mut types_rs)?;
				}

				// sys
				let path = OUT_DIR.join(format!("{}.externs.rs", module));
				write_if_contrib(&mut sys_rs)?;
				writeln!(&mut sys_rs, "mod {}_sys {{", module)?;
				writeln!(&mut sys_rs, "\tuse super::*;")?;
				writeln!(&mut sys_rs)?;
				for entry in glob(&format!("{}/{}-*.rv.rs", out_dir_as_str, module))? {
					let entry: PathBuf = entry?;
					copy_indent(BufReader::new(File::open(entry)?), &mut sys_rs, "\t")?;
				}
				copy_indent(BufReader::new(File::open(&path)?), &mut sys_rs, "\t")?;
				writeln!(&mut sys_rs, "}}")?;
				write_if_contrib(&mut sys_rs)?;
				writeln!(&mut sys_rs, "pub use {}_sys::*;", module)?;
				writeln!(&mut sys_rs)?;
			}
			writeln!(&mut hub_rs, "pub mod types;")?;
			writeln!(&mut hub_rs, "#[doc(hidden)]")?;
			writeln!(&mut hub_rs, "pub mod sys;")?;

			add_manual(&mut types_rs, "types")?;

			add_manual(&mut sys_rs, "sys")?;

			writeln!(&mut hub_rs, "pub mod hub_prelude {{")?;
			for module in modules {
				if !is_core_module(module.as_str()) {
					writeln!(&mut hub_rs, r#"	#[cfg(feature = "contrib")]"#)?;
				}
				writeln!(&mut hub_rs, r#"	pub use super::{}::prelude::*;"#, module)?;
			}
			writeln!(&mut hub_rs, "}}")?;
		}

		Ok(())
	}
}

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

static CORE_MODULES: Lazy<HashSet<&'static str>> = Lazy::new(|| HashSet::from_iter([
	"calib3d",
	"core",
	#[cfg(not(feature = "opencv-32"))]
	"dnn",
	#[cfg(feature = "opencv-4")]
	"dnn_superres",
	"features2d",
	"flann",
	#[cfg(feature = "opencv-4")]
	"gapi",
	"highgui",
	"imgcodecs",
	"imgproc",
	"ml",
	"objdetect",
	"photo",
	#[cfg(any(feature = "opencv-32", feature = "opencv-34"))]
	"shape",
	"stitching",
	#[cfg(any(feature = "opencv-32", feature = "opencv-34"))]
	"superres",
	"video",
	"videoio",
	#[cfg(any(feature = "opencv-32", feature = "opencv-34"))]
	"videostab",
	"viz",
].iter().copied()));

static MODULES: OnceCell<Vec<String>> = OnceCell::new();

static OUT_DIR: Lazy<PathBuf> = Lazy::new(|| PathBuf::from(env::var_os("OUT_DIR").expect("Can't read OUT_DIR env var")));
static MANIFEST_DIR: Lazy<PathBuf> = Lazy::new(|| PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("Can't read CARGO_MANIFEST_DIR env var")));
static SRC_DIR: Lazy<PathBuf> = Lazy::new(|| MANIFEST_DIR.join("src"));
static SRC_CPP_DIR: Lazy<PathBuf> = Lazy::new(|| MANIFEST_DIR.join("src_cpp"));

static ENV_VARS: [&str; 8] = [
	"OPENCV_HEADER_DIR",
	"OPENCV_PACKAGE_NAME",
	"OPENCV_PKGCONFIG_NAME",
	"OPENCV_LINK_LIBS",
	"OPENCV_LINK_PATHS",
	"OPENCV_INCLUDE_PATHS",
	"PKG_CONFIG_PATH",
	"VCPKG_ROOT",
];

#[derive(Debug)]
struct Library {
	pub pkg_name: String,
	pub include_paths: Vec<PathBuf>,
	pub version: String,
	pub cargo_metadata: Vec<String>,
}

impl Library {
	fn process_library_list(libs: impl IntoIterator<Item=impl Into<PathBuf>>) -> impl Iterator<Item=String> {
		libs.into_iter()
			.map(|x| {
				let mut path: PathBuf = x.into();
				let extension = path.extension().and_then(OsStr::to_str).unwrap_or_default();
				let is_framework = extension.eq_ignore_ascii_case("framework");
				const LIB_EXTS: [&str; 7] = ["so", "a", "dll", "lib", "dylib", "framework", "tbd"];
				if is_framework || LIB_EXTS.iter().any(|e| e.eq_ignore_ascii_case(extension)) {
					path.set_extension("");
				}
				path.file_name()
					.and_then(|f| f.to_str()
						.map(|f| if is_framework {
							format!("framework={}", f)
						} else {
							f.to_owned()
						})
					).expect("Invalid library name")
			})
	}

	pub fn probe_from_paths(pkg_name: &str, link_libs: &str, link_paths: &str, include_paths: &str) -> Result<Self> {
		eprintln!("=== Setting up OpenCV library from environment:");
		eprintln!("===   pkg_name: {}", pkg_name);
		eprintln!("===   link_libs: {}", link_libs);
		eprintln!("===   link_paths: {}", link_paths);
		eprintln!("===   include_paths: {}", include_paths);
		let mut cargo_metadata = Vec::with_capacity(64);
		cargo_metadata.extend(
			link_paths.split(',')
				.map(str::trim)
				.filter(|x| !x.is_empty())
				.map(|path| {
					let out = iter::once(format!("cargo:rustc-link-search={}", path));
					#[cfg(target_os = "macos")] {
						out.chain(
							iter::once(format!("cargo:rustc-link-search=framework={}", path))
						)
					}
					#[cfg(not(target_os = "macos"))] {
						out
					}
				})
				.flatten()
				.chain(Self::process_library_list(
					link_libs.split(',')
						.map(str::trim)
						.filter(|x| !x.is_empty()))
					.map(|l| format!("cargo:rustc-link-lib={}", l))
				)
		);

		let include_paths: Vec<_> = include_paths.split(',')
			.map(str::trim)
			.filter(|x| !x.is_empty())
			.map(PathBuf::from)
			.collect();

		let version = include_paths.iter()
			.filter_map(get_version_from_headers)
			.next();

		Ok(Self {
			pkg_name: pkg_name.to_owned(),
			include_paths,
			version: version.unwrap_or_else(|| "0.0.0".to_owned()),
			cargo_metadata,
		})
	}

	#[cfg(not(target_env = "msvc"))]
	pub fn probe_system(pkg_name: &str) -> Result<Self> {
		eprintln!("=== Setting up OpenCV library from pkg_config");
		let mut config = pkg_config::Config::new();
		config.cargo_metadata(false);
		let opencv = config.probe(pkg_name)?;
		let mut cargo_metadata = Vec::with_capacity(64);
		cargo_metadata.extend(
			opencv.link_paths.iter()
				.map(|link_path| format!("cargo:rustc-link-search=native={}", link_path.to_str().expect("Invalid link path")))
				.chain(opencv.libs.iter()
					.map(|lib| format!("cargo:rustc-link-lib={}", lib))
				)
				.chain(opencv.framework_paths.iter()
					.map(|fw_path| format!("cargo:rustc-link-search=framework={}", fw_path.to_str().expect("Invalid framework path")))
				)
				.chain(opencv.frameworks.iter()
					.map(|fw| format!("rustc-link-lib=framework={}", fw))
				)
		);
		Ok(Self {
			pkg_name: pkg_name.to_owned(),
			include_paths: opencv.include_paths,
			version: opencv.version,
			cargo_metadata,
		})
	}

	#[cfg(target_env = "msvc")]
	pub fn probe_system(pkg_name: &str) -> Result<Self> {
		eprintln!("=== Setting up OpenCV library from vcpkg");
		let mut config = vcpkg::Config::new();
		config.cargo_metadata(false);
		let opencv = config.find_package(pkg_name)?;

		let version = opencv.include_paths.iter()
			.filter_map(get_version_from_headers)
			.next();

		Ok(Self {
			pkg_name: pkg_name.to_owned(),
			include_paths: opencv.include_paths,
			version: version.unwrap_or("0.0.0".to_owned()),
			cargo_metadata: opencv.cargo_metadata,
		})
	}

	pub fn probe() -> Result<Self> {
		let env_vars = (env::var("OPENCV_LINK_LIBS"), env::var("OPENCV_LINK_PATHS"), env::var("OPENCV_INCLUDE_PATHS"));
		let env_vars = if let (Ok(link_libs), Ok(link_paths), Ok(include_paths)) = env_vars {
			Some((link_libs, link_paths, include_paths))
		} else {
			None
		};
		let pkg_name = if cfg!(feature = "opencv-32") || cfg!(feature = "opencv-34") {
			env::var("OPENCV_PACKAGE_NAME")
				.or_else(|_| env::var("OPENCV_PKGCONFIG_NAME"))
				.map(Cow::Owned)
				.unwrap_or_else(|_| if cfg!(target_env = "msvc") && env_vars.is_none() {
					// the package name in vcpkg for OpenCV3.x is different: https://github.com/microsoft/vcpkg/tree/master/ports/opencv3
					"opencv3".into()
				} else {
					"opencv".into()
				})
		} else if cfg!(feature = "opencv-4") {
			env::var("OPENCV_PACKAGE_NAME")
				.or_else(|_| env::var("OPENCV_PKGCONFIG_NAME"))
				.map(Cow::Owned)
				.unwrap_or_else(|_| "opencv4".into())
		} else {
			unreachable!("Feature flags should have been checked in main()");
		};
		if let Some((link_libs, link_paths, include_paths)) = env_vars {
			Self::probe_from_paths(&pkg_name, &link_libs, &link_paths, &include_paths)
		} else {
			Self::probe_system(&pkg_name)
		}.map_err(|e| format!("Package {} is not found, caused by: {}", pkg_name, e).into())
	}

	pub fn emit_cargo_metadata(&self) {
		self.cargo_metadata.iter().for_each(|meta| {
			println!("{}", meta);
		});
	}
}

fn file_copy_to_dir(src_file: &Path, target_dir: &Path) -> Result<PathBuf> {
	if !target_dir.exists() {
		fs::create_dir_all(&target_dir)?;
	}
	let src_filename = src_file.file_name()
		.ok_or_else(|| "Can't calculate filename")?;
	let target_file = target_dir.join(src_filename);
	fs::copy(&src_file, &target_file)?;
	Ok(target_file)
}

fn get_version_from_headers(header_dir: &PathBuf) -> Option<String> {
	let version_hpp = {
		let out = header_dir.join("opencv2/core/version.hpp");
		if out.is_file() {
			out
		} else {
			let out = header_dir.join("Headers/core/version.hpp");
			if out.is_file() {
				out
			} else {
				return None;
			}
		}
	};
	let mut major = None;
	let mut minor = None;
	let mut revision = None;
	let mut line = String::with_capacity(256);
	let mut reader = BufReader::new(File::open(version_hpp).ok()?);
	while let Ok(bytes_read) = reader.read_line(&mut line) {
		if bytes_read == 0 {
			break;
		}
		const PREFIX: &str = "#define CV_VERSION_";
		if line.starts_with(PREFIX) {
			let mut parts = line[PREFIX.len()..].split_whitespace();
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
		Some(format!("{}.{}.{}", major, minor, revision))
	} else {
		Some("0.0.0".to_string())
	}
}

fn check_matching_version(version: &str) -> Result<()> {
	if cfg!(feature = "opencv-32") && !VersionReq::parse("~3.2")?.matches(&Version::parse(version)?) {
		Err(format!("OpenCV version: {} must be from 3.2 branch because of the feature: opencv-32", version).into())
	} else if cfg!(feature = "opencv-34") && !VersionReq::parse("~3.4")?.matches(&Version::parse(version)?) {
		Err(format!("OpenCV version: {} must be from 3.4 branch because of the feature: opencv-34", version).into())
	} else if cfg!(feature = "opencv-4") && !VersionReq::parse("~4")?.matches(&Version::parse(version)?) {
		Err(format!("OpenCV version: {} must be from 4.x branch because of the feature: opencv-4", version).into())
	} else {
		Ok(())
	}
}

fn get_versioned_hub_dirs() -> (PathBuf, PathBuf) {
	let bindings_dir = MANIFEST_DIR.join("bindings");
	let mut rust_hub_dir = bindings_dir.join("rust");
	let mut cpp_hub_dir = bindings_dir.join("cpp");
	if cfg!(feature = "opencv-32") {
		rust_hub_dir.push("opencv_32");
		cpp_hub_dir.push("opencv_32");
	} else if cfg!(feature = "opencv-34") {
		rust_hub_dir.push("opencv_34");
		cpp_hub_dir.push("opencv_34");
	} else if cfg!(feature = "opencv-4") {
		rust_hub_dir.push("opencv_4");
		cpp_hub_dir.push("opencv_4");
	}
	(rust_hub_dir, cpp_hub_dir)
}

fn make_modules(opencv_dir_as_string: &str) -> Result<()> {
	if MODULES.get().is_some() {
		return Ok(());
	}
	let ignore_modules: HashSet<&'static str> = HashSet::from_iter([
		"core_detect",
		"cudalegacy",
		"cudev",
		"gapi",
		"hal",
		"hfs",
		"ippicv",
		"opencv",
		"opencv_modules",
		"optflow",
		"quality",
		"rgbd",
		"saliency",
		"stereo",
		"tracking",
		"ximgproc",
	].iter().copied());

	let modules: Vec<String> = glob(&format!("{}/*.hpp", opencv_dir_as_string))?
		.filter_map(|entry| {
			let entry = entry.expect("Can't get path for module file");
			let module: String = entry.file_stem().expect("Can't calculate file stem")
				.to_string_lossy()
				.into_owned();
			if ignore_modules.contains(module.as_str()) {
				None
			} else {
				Some(module)
			}
		})
		.collect();

	MODULES.set(modules).expect("Can't set MODULES cache");
	Ok(())
}

fn is_core_module(module: &str) -> bool {
	CORE_MODULES.contains(module)
}

fn build_compiler(opencv_header_dir: &Path) -> cc::Build {
	let mut out = cc::Build::new();
	out.cpp(true)
		.include(&*SRC_CPP_DIR)
		.include(opencv_header_dir)
		.include(&*OUT_DIR)
		.include(".")
		.flag_if_supported("-Wno-class-memaccess")
		.flag_if_supported("-Wno-deprecated-declarations")
		.flag_if_supported("-Wno-deprecated-copy")
		.flag_if_supported("-Wno-unused-variable")
		.flag_if_supported("-Wno-return-type-c-linkage")
	;
	if cfg!(target_env = "msvc") {
		out.flag_if_supported("-std:c++latest")
			.flag_if_supported("-wd4996")
			.flag_if_supported("-wd5054") // deprecated between enumerations of different types
			.flag_if_supported("-wd4190") // has C-linkage specified, but returns UDT 'Result<cv::Rect_<int>>' which is incompatible with C
			.flag_if_supported("-EHsc")
		;
	} else {
		out.flag("-std=c++11");
	}
	out
}

fn build_wrapper(opencv_header_dir: &Path) -> Result<()> {
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

	let mut cc = build_compiler(opencv_header_dir);
	let modules = MODULES.get().expect("MODULES not initialized");
	for module in modules.iter().filter(|m| cfg!(feature = "contrib") || is_core_module(m)) {
		cc.file(OUT_DIR.join(format!("{}.cpp", module)));
		let manual_cpp = SRC_CPP_DIR.join(format!("manual-{}.cpp", module));
		if manual_cpp.exists() {
			cc.file(manual_cpp);
		}
	}
	cc.compile("ocvrs");
	Ok(())
}

fn install_wrapper() -> Result<()> {
	let (rust_hub_dir, cpp_hub_dir) = get_versioned_hub_dirs();
	let target_hub_dir = SRC_DIR.join("opencv");
	let target_module_dir = target_hub_dir.join("hub");

	for entry in glob(&format!("{}/*.cpp", cpp_hub_dir.to_str().unwrap()))? {
		let entry = entry?;
		file_copy_to_dir(&entry, &OUT_DIR)?;
	}
	for entry in glob(&format!("{}/*.hpp", cpp_hub_dir.to_str().unwrap()))? {
		let entry = entry?;
		file_copy_to_dir(&entry, &OUT_DIR)?;
	}

	if !cfg!(feature = "docs-only") {
		for entry in glob(&format!("{}/*.rs", target_module_dir.to_str().unwrap()))? {
			let _ = fs::remove_file(entry?);
		}
		for entry in glob(&format!("{}/**/*.rs", rust_hub_dir.to_str().unwrap())).unwrap() {
			let entry = entry?;
			let target_file = target_hub_dir.join(entry.strip_prefix(&rust_hub_dir)?);
			if let Some(target_dir) = target_file.parent() {
				if !target_dir.exists() {
					fs::create_dir_all(target_dir)?;
				}
			}
			fs::copy(&entry, target_file)?;
		}
	}
	Ok(())
}

fn cleanup() -> Result<()> {
	// fixme, shouldn't be needed
	for entry in glob(&format!("{}/*.rs", OUT_DIR.to_string_lossy()))? {
		let _ = fs::remove_file(entry?);
	}
	Ok(())
}

fn main() -> Result<()> {
	let features = [cfg!(feature = "opencv-32"), cfg!(feature = "opencv-34"), cfg!(feature = "opencv-4")].iter().map(|&x| i32::from(x)).sum::<i32>();
	if features != 1 {
		panic!("Please select exactly one of the features: opencv-32, opencv-34, opencv-4");
	}
	eprintln!("=== Environment configuration:");
	for &v in ENV_VARS.iter() {
		eprintln!("===   {} = {:?}", v, env::var_os(v));
	}
	let opencv = if cfg!(feature = "docs-only") {
		Library::probe_from_paths("opencv", "", "", "")?
	} else {
		Library::probe()?
	};
	eprintln!("=== OpenCV library configuration: {:#?}", opencv);
	if !cfg!(feature = "docs-only") {
		check_matching_version(&opencv.version)
			.map_err(|e| format!("{}, (version coming from the detected package/environment)", e))?;
	}
	let opencv_header_dir = env::var_os("OPENCV_HEADER_DIR").map(PathBuf::from).unwrap_or_else(|| {
		if cfg!(feature = "buildtime-bindgen") {
			opencv.include_paths.iter()
				.find(|p| p.join("opencv2/core/version.hpp").is_file() || p.join("Headers/core/version.hpp").is_file())
				.expect("Using buildtime-bindgen, but discovered OpenCV include paths is empty or contains non-existent paths").clone()
		} else if cfg!(feature = "opencv-32") {
			MANIFEST_DIR.join("headers/3.2")
		} else if cfg!(feature = "opencv-34") {
			MANIFEST_DIR.join("headers/3.4")
		} else if cfg!(feature = "opencv-4") {
			MANIFEST_DIR.join("headers/4")
		} else {
			panic!("Please select one OpenCV major version using one of the opencv-* features or specify OpenCV header path manually via OPENCV_HEADER_DIR environment var");
		}
	});

	make_modules(&opencv_header_dir.join("opencv2").to_string_lossy())?;

	if let Some(version) = get_version_from_headers(&opencv_header_dir) {
		check_matching_version(&version).map_err(|e| format!("{}, (version coming from headers at: {})", e, opencv_header_dir.display()))?;
		eprintln!("=== Found OpenCV library version: {} in headers located at: {}", version, opencv_header_dir.display());
	} else {
		panic!("Unable to find header version in: {}", opencv_header_dir.display())
	}

	#[cfg(feature = "buildtime-bindgen")]
	generator::gen_wrapper(&opencv_header_dir)?;
	install_wrapper()?;
	build_wrapper(&opencv_header_dir)?;
	// -l linker args should be emitted after -l static
	opencv.emit_cargo_metadata();
	cleanup()?;
	Ok(())
}
