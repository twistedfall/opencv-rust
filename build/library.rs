use std::borrow::Cow;
use std::collections::HashSet;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::{env, fmt, iter};

use dunce::canonicalize;
use semver::Version;

use super::cmake_probe::{CmakeProbe, LinkLib, LinkSearch};
use super::header::IncludePath;
use super::{header, Result, MANIFEST_DIR, OUT_DIR, TARGET_VENDOR_APPLE};

struct PackageName;

impl PackageName {
	pub fn env() -> Option<String> {
		env::var("OPENCV_PACKAGE_NAME").ok()
	}

	pub fn env_pkg_config() -> Option<String> {
		env::var("OPENCV_PKGCONFIG_NAME").ok()
	}

	pub fn env_cmake() -> Option<String> {
		env::var("OPENCV_CMAKE_NAME").ok()
	}

	pub fn env_vcpkg() -> Option<String> {
		env::var("OPENCV_VCPKG_NAME").ok()
	}

	pub fn pkg_config() -> Vec<Cow<'static, str>> {
		Self::env()
			.or_else(Self::env_pkg_config)
			.map_or_else(|| vec!["opencv4".into(), "opencv".into()], |env_name| vec![env_name.into()])
	}

	pub fn cmake() -> Cow<'static, str> {
		Self::env()
			.or_else(Self::env_cmake)
			.map_or_else(|| "OpenCV".into(), |env_name| env_name.into())
	}

	pub fn vcpkg() -> Vec<Cow<'static, str>> {
		Self::env()
			.or_else(Self::env_vcpkg)
			.map_or_else(|| vec!["opencv4".into(), "opencv3".into()], |env_name| vec![env_name.into()])
	}
}

#[derive(Clone, Copy, Debug)]
pub struct EnvList<'s> {
	src: &'s str,
}

impl<'s> EnvList<'s> {
	pub fn is_extend(&self) -> bool {
		self.src.starts_with('+')
	}

	pub fn iter(&self) -> impl Iterator<Item = &'s str> {
		if self.is_extend() {
			&self.src[1..]
		} else {
			self.src
		}
		.split(',')
	}
}

impl<'s> From<&'s str> for EnvList<'s> {
	fn from(src: &'s str) -> Self {
		Self { src }
	}
}

impl fmt::Display for EnvList<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fmt::Display::fmt(self.src, f)
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Linkage {
	Default,
	Dynamic,
	Static,
	Framework,
}

impl Linkage {
	pub fn as_cargo_rustc_link_spec(self) -> &'static str {
		match self {
			Self::Default => "",
			Self::Dynamic => "dylib=",
			Self::Static => "static=",
			Self::Framework => "framework=",
		}
	}

	pub fn as_cargo_rustc_link_spec_no_static(self) -> &'static str {
		// fixme: specifying static linkage breaks things in CI
		match self {
			Self::Default | Self::Dynamic | Self::Static => "",
			Self::Framework => "framework=",
		}
	}

	pub fn as_cargo_rustc_link_search_spec(self) -> &'static str {
		match self {
			Self::Default => "",
			Self::Dynamic | Self::Static => "native=",
			Self::Framework => "framework=",
		}
	}

	pub fn from_path(path: &Path) -> Self {
		let ext = path.extension();
		if Self::is_static_archive(ext) {
			Self::Static
		} else {
			Self::Default
		}
	}

	pub fn from_prefixed_str(s: &str) -> (Self, &str) {
		// for backwards compatibility to allow specifying as "OpenCL.framework" in addition to "framework=OpenCL"
		if let Some(name) = s.strip_suffix(".framework") {
			return (Self::Framework, name);
		}
		[Self::Dynamic, Self::Static, Self::Framework]
			.iter()
			.find_map(|l| s.strip_prefix(l.as_cargo_rustc_link_spec()).map(|s| (*l, s)))
			.unwrap_or((Self::Default, s))
	}

	fn is_static_archive(ext: Option<&OsStr>) -> bool {
		ext.is_some_and(|ext| ext.eq_ignore_ascii_case("a"))
	}
}

#[derive(Debug)]
pub struct Library {
	pub include_paths: Vec<PathBuf>,
	pub version: Version,
	pub inherent_features: Vec<String>,
	pub cargo_metadata: Vec<String>,
}

impl Library {
	fn add_multiarch_dir_if_needed(include_paths: &mut Vec<PathBuf>) {
		if include_paths.iter().all(|p| p.get_config_header().is_none()) {
			if let Some(multiarch_include_path) = header::get_multiarch_header_dir() {
				include_paths.push(multiarch_include_path);
			}
		}
	}

	fn version_from_include_paths(include_paths: impl IntoIterator<Item = impl AsRef<Path>>) -> Option<Version> {
		include_paths.into_iter().find_map(|x| x.as_ref().find_version())
	}

	fn inherent_features_from_include_paths(include_paths: impl IntoIterator<Item = impl AsRef<Path>>) -> Option<Vec<String>> {
		include_paths.into_iter().find_map(|x| x.as_ref().find_inherent_features())
	}

	fn process_env_var_list<'a, T: From<&'a str>>(env_list: Option<EnvList<'a>>, sys_list: Vec<T>) -> Vec<T> {
		if let Some(env_list) = env_list {
			let mut paths = if env_list.is_extend() {
				sys_list
			} else {
				vec![]
			};
			paths.extend(env_list.iter().filter(|v| !v.is_empty()).map(T::from));
			paths
		} else {
			sys_list
		}
	}

	fn process_link_paths<'a>(link_paths: Option<EnvList>, sys_link_paths: Vec<LinkSearch>) -> impl Iterator<Item = String> + 'a {
		Self::process_env_var_list(link_paths, sys_link_paths)
			.into_iter()
			.flat_map(move |path| {
				iter::once(path.emit_cargo_rustc_link_search()).chain(
					(*TARGET_VENDOR_APPLE && path.0 != Linkage::Framework)
						.then(|| LinkSearch(Linkage::Framework, path.1).emit_cargo_rustc_link_search()),
				)
			})
	}

	fn process_link_libs<'a>(link_libs: Option<EnvList>, sys_link_libs: Vec<LinkLib>) -> impl Iterator<Item = String> + 'a {
		Self::process_env_var_list(link_libs, sys_link_libs)
			.into_iter()
			.map(|l| l.emit_cargo_rustc_link())
	}

	fn find_vcpkg_tool(vcpkg_root: &Path, tool_name: &str) -> Option<PathBuf> {
		let tool_dirs = vcpkg_root
			.join("downloads/tools")
			.read_dir()
			.into_iter()
			.flatten()
			.flatten()
			.map(|e| e.path())
			.filter(|p| {
				p.is_dir()
					&& p
						.file_name()
						.and_then(OsStr::to_str)
						.is_some_and(|n| n.starts_with(tool_name))
			});

		tool_dirs
			.flat_map(|d| d.read_dir().into_iter().flatten().flatten()) // all subdirs inside tool dirs
			.map(|e| e.path())
			.filter(|p| p.is_dir())
			.flat_map(|p| p.read_dir().into_iter().flatten().flatten()) // all subdirs inside those dirs
			.map(|e| e.path())
			.flat_map(|p| [p.join(format!("bin/{tool_name}")), p.join(format!("bin/{tool_name}.exe"))])
			.filter_map(|p| canonicalize(p).ok())
			.find(|p| p.is_file())
	}

	pub fn probe_from_paths(
		include_paths: Option<EnvList>,
		link_paths: Option<EnvList>,
		link_libs: Option<EnvList>,
	) -> Result<Self> {
		if let (Some(include_paths), Some(link_paths), Some(link_libs)) = (include_paths, link_paths, link_libs) {
			eprintln!("=== Configuring OpenCV library from the environment:");
			eprintln!("===   include_paths: {include_paths}");
			eprintln!("===   link_paths: {link_paths}");
			eprintln!("===   link_libs: {link_libs}");
			let mut cargo_metadata = Vec::with_capacity(64);
			let include_paths = include_paths.iter().map(PathBuf::from).collect::<Vec<_>>();

			let version =
				Self::version_from_include_paths(&include_paths).ok_or("Could not get OpenCV version from include_paths")?;
			let inherent_features =
				Self::inherent_features_from_include_paths(&include_paths).ok_or("Could not get OpenCV config from include_paths")?;

			cargo_metadata.extend(Self::process_link_paths(Some(link_paths), vec![]));
			cargo_metadata.extend(Self::process_link_libs(Some(link_libs), vec![]));

			Ok(Self {
				include_paths,
				version,
				inherent_features,
				cargo_metadata,
			})
		} else {
			Err("Some environment variables are missing".into())
		}
	}

	pub fn probe_pkg_config(
		include_paths: Option<EnvList>,
		link_paths: Option<EnvList>,
		link_libs: Option<EnvList>,
	) -> Result<Self> {
		eprintln!("=== Probing OpenCV library using pkg_config");
		let mut config = pkg_config::Config::new();
		config.cargo_metadata(false);
		let mut errors = vec![];
		let mut opencv = None;
		let possible_opencvs = PackageName::pkg_config().into_iter().map(|pkg_name| config.probe(&pkg_name));
		for possible_opencv in possible_opencvs {
			match possible_opencv {
				Ok(possible_opencv) => {
					opencv = Some(possible_opencv);
					break;
				}
				Err(e) => {
					errors.push(e.to_string());
				}
			}
		}
		let opencv = opencv.ok_or_else(|| errors.join(", "))?;
		let mut cargo_metadata = Vec::with_capacity(64);

		cargo_metadata.extend(Self::process_link_paths(
			link_paths,
			opencv
				.link_paths
				.into_iter()
				.map(|p| LinkSearch(Linkage::Default, p))
				.collect(),
		));
		if link_paths.map_or(true, |link_paths| link_paths.is_extend()) {
			cargo_metadata.extend(Self::process_link_paths(
				None,
				opencv
					.framework_paths
					.into_iter()
					.map(|p| LinkSearch(Linkage::Framework, p))
					.collect(),
			));
		}

		cargo_metadata.extend(Self::process_link_libs(
			link_libs,
			opencv.libs.into_iter().map(|l| LinkLib(Linkage::Default, l)).collect(),
		));
		if link_libs.is_some_and(|link_libs| link_libs.is_extend()) {
			cargo_metadata.extend(Self::process_link_libs(
				None,
				opencv
					.frameworks
					.into_iter()
					.map(|f| LinkLib(Linkage::Framework, f))
					.collect(),
			));
		}

		let mut include_paths = Self::process_env_var_list(include_paths, opencv.include_paths);
		Self::add_multiarch_dir_if_needed(&mut include_paths);
		let inherent_features =
			Self::inherent_features_from_include_paths(&include_paths).ok_or("Could not get OpenCV config from include_paths")?;

		Ok(Self {
			include_paths,
			version: Version::parse(&opencv.version)?,
			inherent_features,
			cargo_metadata,
		})
	}

	pub fn probe_cmake(
		include_paths: Option<EnvList>,
		link_paths: Option<EnvList>,
		link_libs: Option<EnvList>,
		toolchain: Option<&Path>,
		cmake_bin: Option<&Path>,
		ninja_bin: Option<&Path>,
	) -> Result<Self> {
		let toolchain_var = env::var_os("OPENCV_CMAKE_TOOLCHAIN_FILE").map(PathBuf::from);
		let toolchain = toolchain.or(toolchain_var.as_deref());
		eprintln!(
			"=== Probing OpenCV library using cmake{}",
			toolchain.map_or_else(|| "".to_string(), |tc| format!(" with toolchain: {}", tc.display()))
		);

		let src_dir = MANIFEST_DIR.join("cmake");
		let package_name = PackageName::cmake();
		let args = env::var_os("OPENCV_CMAKE_ARGS");
		let cmake_bin_var = env::var_os("OPENCV_CMAKE_BIN").map(PathBuf::from);
		let cmake = CmakeProbe::new(
			cmake_bin.or(cmake_bin_var.as_deref()),
			&OUT_DIR,
			&src_dir,
			package_name.as_ref(),
			toolchain,
			env::var_os("PROFILE").is_some_and(|p| p == "release"),
			args.as_ref().and_then(|p| p.to_str()),
		);
		let mut probe_result = cmake
			.probe_ninja(ninja_bin)
			.or_else(|e| {
				eprintln!("=== Probing with cmake ninja generator failed, will try Makefile generator, error: {e}");
				cmake.probe_makefile()
			})
			.or_else(|e| {
				eprintln!("=== Probing with cmake Makefile generator failed, will try deprecated find_package, error: {e}");
				cmake.probe_find_package()
			})?;

		if probe_result.version.is_none() {
			probe_result.version = Self::version_from_include_paths(&probe_result.include_paths);
		}

		let mut cargo_metadata = Vec::with_capacity(probe_result.link_paths.len() + probe_result.link_libs.len());
		cargo_metadata.extend(Self::process_link_paths(link_paths, probe_result.link_paths));
		cargo_metadata.extend(Self::process_link_libs(link_libs, probe_result.link_libs));

		let mut include_paths = Self::process_env_var_list(include_paths, probe_result.include_paths);
		Self::add_multiarch_dir_if_needed(&mut include_paths);
		let inherent_features =
			Self::inherent_features_from_include_paths(&include_paths).ok_or("Could not get OpenCV config from include_paths")?;

		Ok(Self {
			include_paths,
			version: probe_result.version.unwrap_or_else(|| Version::new(0, 0, 0)),
			inherent_features,
			cargo_metadata,
		})
	}

	pub fn probe_vcpkg(include_paths: Option<EnvList>, link_paths: Option<EnvList>, link_libs: Option<EnvList>) -> Result<Self> {
		eprintln!("=== Probing OpenCV library using vcpkg");
		let mut config = vcpkg::Config::new();
		config.cargo_metadata(false);
		let mut errors = vec![];
		let mut opencv = None;
		let possible_opencvs = PackageName::vcpkg()
			.into_iter()
			.map(|pkg_name| config.find_package(pkg_name.as_ref()));
		for possible_opencv in possible_opencvs {
			match possible_opencv {
				Ok(possible_opencv) => {
					opencv = Some(possible_opencv);
					break;
				}
				Err(e) => {
					errors.push(e.to_string());
				}
			}
		}
		let opencv = opencv.ok_or_else(|| errors.join(", "))?;

		let mut opencv_include_paths = opencv.include_paths;
		// workaround for the wrong detected include path for vcpkg 2024.11.16+: https://github.com/twistedfall/opencv-rust/issues/640
		for include_path in &mut opencv_include_paths {
			if include_path.ends_with("include") {
				let fixed_include_path = include_path.join("opencv4");
				if fixed_include_path.exists() {
					*include_path = fixed_include_path;
				}
			}
		}

		let version = Self::version_from_include_paths(&opencv_include_paths);

		let mut cargo_metadata = opencv.cargo_metadata;

		if link_paths.as_ref().is_some_and(|lp| !lp.is_extend()) {
			cargo_metadata.retain(|p| !p.starts_with("cargo:rustc-link-search=") && !p.starts_with("cargo::rustc-link-search="));
		}
		cargo_metadata.extend(Self::process_link_paths(link_paths, vec![]));

		if link_libs.as_ref().is_some_and(|ll| !ll.is_extend()) {
			cargo_metadata.retain(|p| !p.starts_with("cargo:rustc-link-lib=") && !p.starts_with("cargo::rustc-link-lib="));
		}
		cargo_metadata.extend(Self::process_link_libs(link_libs, vec![]));

		let mut include_paths = Self::process_env_var_list(include_paths, opencv_include_paths);
		Self::add_multiarch_dir_if_needed(&mut include_paths);
		let inherent_features =
			Self::inherent_features_from_include_paths(&include_paths).ok_or("Could not get OpenCV config from include_paths")?;

		Ok(Self {
			include_paths,
			version: version.unwrap_or_else(|| Version::new(0, 0, 0)),
			inherent_features,
			cargo_metadata,
		})
	}

	pub fn probe_vcpkg_cmake(
		include_paths: Option<EnvList>,
		link_paths: Option<EnvList>,
		link_libs: Option<EnvList>,
	) -> Result<Self> {
		eprintln!("=== Probing OpenCV library using vcpkg_cmake");
		let mut config = vcpkg::Config::new();
		config.cargo_metadata(false);
		// don't care about the error here, only need to have dlls copied to outdir
		PackageName::vcpkg()
			.into_iter()
			.map(|pkg_name| config.find_package(&pkg_name))
			.take_while(|r| r.is_err())
			.count();

		let vcpkg_root = canonicalize(vcpkg::find_vcpkg_root(&config)?)?;
		eprintln!("=== Discovered vcpkg root: {}", vcpkg_root.display());
		let vcpkg_cmake = Self::find_vcpkg_tool(&vcpkg_root, "cmake");
		let vcpkg_ninja = Self::find_vcpkg_tool(&vcpkg_root, "ninja");
		let toolchain = vcpkg_root.join("scripts/buildsystems/vcpkg.cmake");
		Self::probe_cmake(
			include_paths,
			link_paths,
			link_libs,
			Some(&toolchain),
			vcpkg_cmake.as_deref(),
			vcpkg_ninja.as_deref(),
		)
	}

	pub fn probe_system(include_paths: Option<EnvList>, link_paths: Option<EnvList>, link_libs: Option<EnvList>) -> Result<Self> {
		let explicit_pkg_config = env::var_os("PKG_CONFIG_PATH").is_some() || env::var_os("OPENCV_PKGCONFIG_NAME").is_some();
		let explicit_cmake = env::var_os("OpenCV_DIR").is_some()
			|| env::var_os("OPENCV_CMAKE_NAME").is_some()
			|| env::var_os("CMAKE_PREFIX_PATH").is_some()
			|| env::var_os("OPENCV_CMAKE_BIN").is_some()
			|| env::var_os("OPENCV_CMAKE_TOOLCHAIN_FILE").is_some()
			|| env::var_os("OPENCV_CMAKE_ARGS").is_some();
		let explicit_vcpkg = env::var_os("VCPKG_ROOT").is_some();
		eprintln!(
			"=== Detected probe priority boost based on environment vars: pkg_config: {explicit_pkg_config}, cmake: {explicit_cmake}, vcpkg: {explicit_vcpkg}"
		);

		let mut probes = [
			Probe::Environment,
			Probe::PkgConfig,
			Probe::CMake,
			Probe::VcpkgCMake,
			Probe::Vcpkg,
		];

		let disabled_probes = env::var("OPENCV_DISABLE_PROBES");
		let disabled_probes = disabled_probes.as_ref().ok().map_or_else(HashSet::new, |s| {
			EnvList::from(s.as_str())
				.iter()
				.map(|s| {
					Probe::try_from_str(s).unwrap_or_else(|| {
						panic!(
							"Invalid value in OPENCV_DISABLE_PROBES: {s}, valid values are: {}",
							probe_list(&probes)
						)
					})
				})
				.collect()
		});

		let mut prioritize = |higher_prio: Probe, lower_prio: Probe| {
			let (probe_idx, over_idx) = probes
				.iter()
				.position(|probe| probe == &higher_prio)
				.and_then(|probe_idx| {
					probes
						.iter()
						.position(|probe| probe == &lower_prio)
						.map(|over_idx| (probe_idx, over_idx))
				})
				.expect("Can't find probe to swap");
			if probe_idx > over_idx {
				for i in (over_idx..probe_idx).rev() {
					probes.swap(i, i + 1);
				}
			}
		};

		if explicit_pkg_config {
			if explicit_vcpkg && !explicit_cmake {
				prioritize(Probe::VcpkgCMake, Probe::CMake);
				prioritize(Probe::Vcpkg, Probe::CMake);
			}
		} else if explicit_cmake {
			prioritize(Probe::CMake, Probe::PkgConfig);
			if explicit_vcpkg {
				prioritize(Probe::VcpkgCMake, Probe::PkgConfig);
				prioritize(Probe::Vcpkg, Probe::PkgConfig);
			}
		} else if explicit_vcpkg {
			prioritize(Probe::VcpkgCMake, Probe::PkgConfig);
			prioritize(Probe::Vcpkg, Probe::PkgConfig);
		}

		eprintln!(
			"=== Probing the OpenCV library in the following order: {}",
			probe_list(&probes)
		);

		let mut out = None;
		for probe in &probes {
			if !disabled_probes.contains(probe) {
				match probe.run_probe(include_paths, link_paths, link_libs) {
					Ok(lib) => {
						out = Some(lib);
						eprintln!("=== Successfully probed using: {}", probe.as_str());
						break;
					}
					Err(e) => {
						eprintln!(
							"=== Can't probe using: {}, continuing with other methods because: {e}",
							probe.as_str()
						);
					}
				}
			} else {
				eprintln!(
					"=== Skipping probe: {} because it's disabled using OPENCV_DISABLE_PROBES",
					probe.as_str()
				);
			}
		}
		out.ok_or_else(|| {
			let probes = probes
				.into_iter()
				.filter(|name| !disabled_probes.contains(name))
				.collect::<Vec<_>>();
			format!("Failed to find installed OpenCV package using probes: {}, refer to https://github.com/twistedfall/opencv-rust#getting-opencv for help", probe_list(&probes)).into()

		})
	}

	pub fn probe() -> Result<Self> {
		let include_paths = env::var("OPENCV_INCLUDE_PATHS").ok();
		let include_paths = include_paths.as_deref().map(EnvList::from);
		let link_paths = env::var("OPENCV_LINK_PATHS").ok();
		let link_paths = link_paths.as_deref().map(EnvList::from);
		let link_libs = env::var("OPENCV_LINK_LIBS").ok();
		let link_libs = link_libs.as_deref().map(EnvList::from);
		Self::probe_system(include_paths, link_paths, link_libs)
	}

	pub fn emit_cargo_metadata(&self) {
		self.cargo_metadata.iter().for_each(|meta| {
			println!("{meta}");
		});
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Probe {
	Environment,
	PkgConfig,
	CMake,
	VcpkgCMake,
	Vcpkg,
}

impl Probe {
	fn run_probe(
		&self,
		include_paths: Option<EnvList>,
		link_paths: Option<EnvList>,
		link_libs: Option<EnvList>,
	) -> Result<Library> {
		match self {
			Probe::Environment => Library::probe_from_paths(include_paths, link_paths, link_libs),
			Probe::PkgConfig => Library::probe_pkg_config(include_paths, link_paths, link_libs),
			Probe::CMake => Library::probe_cmake(include_paths, link_paths, link_libs, None, None, None),
			Probe::VcpkgCMake => Library::probe_vcpkg_cmake(include_paths, link_paths, link_libs),
			Probe::Vcpkg => Library::probe_vcpkg(include_paths, link_paths, link_libs),
		}
	}

	fn try_from_str(s: &str) -> Option<Self> {
		match s {
			"environment" => Some(Probe::Environment),
			"pkg_config" => Some(Probe::PkgConfig),
			"cmake" => Some(Probe::CMake),
			"vcpkg_cmake" => Some(Probe::VcpkgCMake),
			"vcpkg" => Some(Probe::Vcpkg),
			_ => None,
		}
	}

	fn as_str(&self) -> &'static str {
		match self {
			Probe::Environment => "environment",
			Probe::PkgConfig => "pkg_config",
			Probe::CMake => "cmake",
			Probe::VcpkgCMake => "vcpkg_cmake",
			Probe::Vcpkg => "vcpkg",
		}
	}
}

fn probe_list(probes: &[Probe]) -> String {
	probes.iter().map(|probe| probe.as_str()).collect::<Vec<_>>().join(", ")
}
