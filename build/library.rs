use std::{
	borrow::Cow,
	collections::HashSet,
	env,
	ffi::OsStr,
	fmt,
	iter,
	path::{Path, PathBuf},
};

use dunce::canonicalize;
use glob::glob;
use semver::Version;

use super::{
	cleanup_lib_filename,
	cmake_probe::CmakeProbe,
	get_version_from_headers,
	MANIFEST_DIR,
	OUT_DIR,
	Result,
};

struct PackageName;

impl PackageName {
	pub fn env() -> Option<Cow<'static, str>> {
		env::var("OPENCV_PACKAGE_NAME")
			.ok()
			.map(|x| x.into())
	}

	pub fn env_pkg_config() -> Option<Cow<'static, str>> {
		env::var("OPENCV_PKGCONFIG_NAME")
			.ok()
			.map(|x| x.into())
	}

	pub fn env_cmake() -> Option<Cow<'static, str>> {
		env::var("OPENCV_CMAKE_NAME")
			.ok()
			.map(|x| x.into())
	}

	pub fn env_vcpkg() -> Option<Cow<'static, str>> {
		env::var("OPENCV_VCPKG_NAME")
			.ok()
			.map(|x| x.into())
	}

	pub fn pkg_config() -> Vec<Cow<'static, str>> {
		if let Some(env_name) = Self::env().or_else(Self::env_pkg_config) {
			vec![env_name]
		} else {
			vec!["opencv4".into(), "opencv".into()]
		}
	}

	pub fn cmake() -> Cow<'static, str> {
		Self::env()
			.or_else(Self::env_cmake)
			.unwrap_or_else(|| "OpenCV".into())
	}

	pub fn vcpkg() -> Vec<Cow<'static, str>> {
		if let Some(env_name) = Self::env().or_else(Self::env_vcpkg) {
			vec![env_name]
		} else {
			vec!["opencv4".into(), "opencv3".into()]
		}
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

	pub fn iter(&self) -> impl Iterator<Item=&'s str> {
		if self.is_extend() {
			&self.src[1..]
		} else {
			self.src
		}.split(',')
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

#[derive(Debug)]
pub struct Library {
	pub include_paths: Vec<PathBuf>,
	pub version: Version,
	pub cargo_metadata: Vec<String>,
}

impl Library {
	fn process_library_list(libs: impl IntoIterator<Item=impl Into<PathBuf>>) -> impl Iterator<Item=String> {
		libs.into_iter()
			.filter_map(|x| {
				let mut path: PathBuf = x.into();
				let is_framework = path.extension()
					.and_then(OsStr::to_str)
					.map_or(false, |e| e.eq_ignore_ascii_case("framework"));
				if let Some(filename) = path.file_name().and_then(cleanup_lib_filename) {
					let filename = filename.to_owned();
					path.set_file_name(filename);
				}
				path.file_name()
					.and_then(|f| f.to_str()
						.map(|f| if is_framework {
							format!("framework={}", f)
						} else {
							f.to_owned()
						})
					)
			})
	}

	fn version_from_include_paths(include_paths: impl IntoIterator<Item=impl AsRef<Path>>) -> Option<Version> {
		include_paths.into_iter().find_map(|x| get_version_from_headers(x.as_ref()))
	}

	#[inline]
	fn emit_link_search(path: &Path, typ: Option<&str>) -> String {
		format!(
			"cargo:rustc-link-search={}{}",
			typ.map_or_else(|| "".to_string(), |t| format!("{}=", t)),
			path.to_str().expect("Can't convert link search path to UTF-8 string")
		)
	}

	#[inline]
	fn emit_link_lib(lib: &str, typ: Option<&str>) -> String {
		format!("cargo:rustc-link-lib={}{}", typ.map_or_else(|| "".to_string(), |t| format!("{}=", t)), lib)
	}

	fn process_env_var_list<'a, T: From<&'a str>>(env_list: Option<EnvList<'a>>, sys_list: Vec<T>) -> Vec<T> {
		if let Some(include_paths) = env_list {
			let mut includes = if include_paths.is_extend() {
				sys_list
			} else {
				vec![]
			};
			includes.extend(include_paths.iter().filter(|v| !v.is_empty()).map(T::from));
			includes
		} else {
			sys_list
		}
	}

	fn process_link_paths<'a>(link_paths: Option<EnvList>, sys_link_paths: Vec<PathBuf>, typ: Option<&'a str>) -> impl Iterator<Item=String> + 'a {
		Self::process_env_var_list(link_paths, sys_link_paths).into_iter()
			.flat_map(move |path| {
				let out = iter::once(Self::emit_link_search(&path, typ));
				#[cfg(target_vendor = "apple")] {
					out.chain(
						iter::once(Self::emit_link_search(&path, Some("framework")))
					)
				}
				#[cfg(not(target_vendor = "apple"))] {
					out
				}
			})
	}

	fn process_link_libs<'a>(link_libs: Option<EnvList>, sys_link_libs: Vec<String>, typ: Option<&'a str>) -> impl Iterator<Item=String> + 'a {
		Self::process_library_list(Self::process_env_var_list(link_libs, sys_link_libs).into_iter())
			.map(move |l| Self::emit_link_lib(&l, typ))
	}

	pub fn probe_from_paths(include_paths: Option<EnvList>, link_paths: Option<EnvList>, link_libs: Option<EnvList>) -> Result<Self> {
		if let (Some(include_paths), Some(link_paths), Some(link_libs)) = (include_paths, link_paths, link_libs) {
			if include_paths.is_extend() || link_paths.is_extend() || link_libs.is_extend() {
				return Err("Some environment variables extend the system default paths (i.e. start with '+')".into());
			}
			eprintln!("=== Configuring OpenCV library from the environment:");
			eprintln!("===   include_paths: {}", include_paths);
			eprintln!("===   link_paths: {}", link_paths);
			eprintln!("===   link_libs: {}", link_libs);
			let mut cargo_metadata = Vec::with_capacity(64);
			let include_paths: Vec<_> = include_paths.iter()
				.map(PathBuf::from)
				.collect();

			let version = Self::version_from_include_paths(&include_paths);

			cargo_metadata.extend(Self::process_link_paths(Some(link_paths), vec![], None));
			cargo_metadata.extend(Self::process_link_libs(Some(link_libs), vec![], None));

			Ok(Self {
				include_paths,
				version: version.unwrap_or_else(|| Version::new(0, 0, 0)),
				cargo_metadata,
			})
		} else {
			Err("Some environment variables are missing".into())
		}
	}

	pub fn probe_pkg_config(include_paths: Option<EnvList>, link_paths: Option<EnvList>, link_libs: Option<EnvList>) -> Result<Self> {
		eprintln!("=== Probing OpenCV library using pkg_config");
		let mut config = pkg_config::Config::new();
		config.cargo_metadata(false);
		let mut errors = vec![];
		let mut opencv = None;
		let possible_opencvs = PackageName::pkg_config().into_iter()
			.map(|pkg_name| config.probe(&pkg_name));
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

		cargo_metadata.extend(Self::process_link_paths(link_paths, opencv.link_paths, None));
		if link_paths.map_or(true, |link_paths| link_paths.is_extend()) {
			cargo_metadata.extend(Self::process_link_paths(None, opencv.framework_paths, Some("framework")));
		}

		cargo_metadata.extend(Self::process_link_libs(link_libs, opencv.libs, None));
		if link_libs.map_or(false, |link_libs| link_libs.is_extend()) {
			cargo_metadata.extend(Self::process_link_libs(None, opencv.frameworks, Some("framework")));
		}

		let include_paths = Self::process_env_var_list(include_paths, opencv.include_paths);

		Ok(Self {
			include_paths,
			version: Version::parse(&opencv.version)?,
			cargo_metadata,
		})
	}

	pub fn probe_cmake(include_paths: Option<EnvList>, link_paths: Option<EnvList>, link_libs: Option<EnvList>, toolchain: Option<&Path>, cmake_bin: Option<&Path>, ninja_bin: Option<&Path>) -> Result<Self> {
		eprintln!(
			"=== Probing OpenCV library using cmake{}",
			toolchain.map(|tc| format!(" with toolchain: {}", tc.display())).unwrap_or_else(|| "".to_string())
		);

		let src_dir = MANIFEST_DIR.join("cmake");
		let package_name = PackageName::cmake();
		let cmake = CmakeProbe::new(
			env::var_os("OPENCV_CMAKE_BIN")
				.map(PathBuf::from)
				.or_else(|| cmake_bin.map(PathBuf::from)),
			&OUT_DIR,
			&src_dir,
			package_name.as_ref(),
			toolchain,
			env::var_os("PROFILE").map_or(false, |p| p == "release"),
		);
		let mut probe_result = cmake.probe_ninja(ninja_bin)
			.or_else(|e| {
				eprintln!("=== Probing with cmake ninja generator failed, will try Makefile generator, error: {}", e);
				cmake.probe_makefile()
			})
			.or_else(|e| {
				eprintln!("=== Probing with cmake Makefile generator failed, will try deprecated find_package, error: {}", e);
				cmake.probe_find_package()
			})?;

		if probe_result.version.is_none() {
			probe_result.version = Self::version_from_include_paths(&probe_result.include_paths);
		}

		let mut cargo_metadata = Vec::with_capacity(probe_result.link_paths.len() + probe_result.link_libs.len());
		cargo_metadata.extend(Self::process_link_paths(link_paths, probe_result.link_paths, None));
		cargo_metadata.extend(Self::process_link_libs(link_libs, probe_result.link_libs, None));

		Ok(Self {
			include_paths: Self::process_env_var_list(include_paths, probe_result.include_paths),
			version: probe_result.version.unwrap_or_else(|| Version::new(0, 0, 0)),
			cargo_metadata,
		})
	}

	pub fn probe_vcpkg(include_paths: Option<EnvList>, link_paths: Option<EnvList>, link_libs: Option<EnvList>) -> Result<Self> {
		eprintln!("=== Probing OpenCV library using vcpkg");
		let mut config = vcpkg::Config::new();
		config.cargo_metadata(false);
		let mut errors = vec![];
		let mut opencv = None;
		let possible_opencvs = PackageName::vcpkg().into_iter()
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

		let version = Self::version_from_include_paths(&opencv.include_paths);

		let include_paths = Self::process_env_var_list(include_paths, opencv.include_paths);

		let mut cargo_metadata = opencv.cargo_metadata;

		if link_paths.as_ref().map_or(false, |lp| !lp.is_extend()) {
			cargo_metadata = cargo_metadata.into_iter()
				.filter(|p| !p.starts_with("cargo:rustc-link-search="))
				.collect();
		}
		cargo_metadata.extend(Self::process_link_paths(link_paths, vec![], None));

		if link_libs.as_ref().map_or(false, |ll| !ll.is_extend()) {
			cargo_metadata = cargo_metadata.into_iter()
				.filter(|p| !p.starts_with("cargo:rustc-link-lib="))
				.collect();
		}
		cargo_metadata.extend(Self::process_link_libs(link_libs, vec![], None));

		Ok(Self {
			include_paths,
			version: version.unwrap_or_else(|| Version::new(0, 0, 0)),
			cargo_metadata,
		})
	}

	pub fn probe_vcpkg_cmake(include_paths: Option<EnvList>, link_paths: Option<EnvList>, link_libs: Option<EnvList>) -> Result<Self> {
		eprintln!("=== Probing OpenCV library using vcpkg_cmake");
		let mut config = vcpkg::Config::new();
		config.cargo_metadata(false);
		// don't care about the error here, only need to have dlls copied to outdir
		PackageName::vcpkg().into_iter()
			.map(|pkg_name| config.find_package(&pkg_name))
			.take_while(|r| r.is_err())
			.count();

		let vcpkg_root = canonicalize(vcpkg::find_vcpkg_root(&config)?)?;
		eprintln!("=== Discovered vcpkg root: {}", vcpkg_root.display());
		let vcpkg_cmake = vcpkg_root.to_str()
			.and_then(|vcpkg_root| {
				glob(&format!("{}/downloads/tools/cmake*/*/bin/cmake", vcpkg_root)).ok()
					.and_then(|cmake_iter| glob(&format!("{}/downloads/tools/cmake*/*/bin/cmake.exe", vcpkg_root)).ok()
						.map(|cmake_exe_iter| cmake_iter.chain(cmake_exe_iter))
					)
			})
			.and_then(|paths| paths.filter_map(|r| r.ok())
				.filter_map(|p| canonicalize(p).ok())
				.find(|p| p.is_file())
			);
		let vcpkg_ninja = vcpkg_root.to_str()
			.and_then(|vcpkg_root| {
				glob(&format!("{}/downloads/tools/ninja*/ninja", vcpkg_root)).ok()
					.and_then(|ninja_iter| glob(&format!("{}/downloads/tools/ninja*/*/ninja.exe", vcpkg_root)).ok()
						.map(|ninja_exe_iter| ninja_iter.chain(ninja_exe_iter))
					)
			})
			.and_then(|paths| paths.filter_map(|r| r.ok())
				.filter_map(|p| canonicalize(p).ok())
				.find(|p| p.is_file())
			);
		let toolchain = vcpkg_root.join("scripts/buildsystems/vcpkg.cmake");
		Self::probe_cmake(include_paths, link_paths, link_libs, Some(&toolchain), vcpkg_cmake.as_deref(), vcpkg_ninja.as_deref())
	}

	pub fn probe_system(include_paths: Option<EnvList>, link_paths: Option<EnvList>, link_libs: Option<EnvList>) -> Result<Self> {
		let probe_paths = || Self::probe_from_paths(include_paths, link_paths, link_libs);
		let probe_pkg_config = || Self::probe_pkg_config(include_paths, link_paths, link_libs);
		let probe_cmake = || Self::probe_cmake(include_paths, link_paths, link_libs, None, None, None);
		let probe_vcpkg_cmake = || Self::probe_vcpkg_cmake(include_paths, link_paths, link_libs);
		let probe_vcpkg = || Self::probe_vcpkg(include_paths, link_paths, link_libs);

		let explicit_pkg_config = env::var_os("PKG_CONFIG_PATH").is_some() || env::var_os("OPENCV_PKGCONFIG_NAME").is_some();
		let explicit_cmake = env::var_os("OpenCV_DIR").is_some()
			|| env::var_os("OPENCV_CMAKE_NAME").is_some()
			|| env::var_os("CMAKE_PREFIX_PATH").is_some()
			|| env::var_os("OPENCV_CMAKE_BIN").is_some();
		let explicit_vcpkg = env::var_os("VCPKG_ROOT").is_some() || cfg!(target_os = "windows");
		eprintln!("=== Detected probe priority based on environment vars: pkg_config: {}, cmake: {}, vcpkg: {}",
		          explicit_pkg_config, explicit_cmake, explicit_vcpkg
		);

		let disabled_probes = env::var("OPENCV_DISABLE_PROBES");
		let disabled_probes = disabled_probes.as_ref()
			.map(|s| EnvList::from(s.as_str()).iter().collect())
			.unwrap_or_else(|_| HashSet::new());

		let mut probes = [
			("environment", &probe_paths as &dyn Fn() -> Result<Self>),
			("pkg_config", &probe_pkg_config),
			("cmake", &probe_cmake),
			("vcpkg_cmake", &probe_vcpkg_cmake),
			("vcpkg", &probe_vcpkg),
		];

		let mut prioritize = |probe: &str, over: &str| {
			let (probe_idx, over_idx) = probes.iter().position(|(name, _)| name == &probe)
				.and_then(|probe_idx| probes.iter().position(|(name, _)| name == &over)
					.map(|over_idx| (probe_idx, over_idx))
				)
				.expect("Can't find probe to swap");
			if probe_idx > over_idx {
				for i in (over_idx..probe_idx).rev() {
					probes.swap(i, i + 1);
				}
			}
		};

		if explicit_pkg_config {
			if explicit_vcpkg && !explicit_cmake {
				prioritize("vcpkg_cmake", "cmake");
				prioritize("vcpkg", "cmake");
			}
		} else if explicit_cmake {
			prioritize("cmake", "pkg_config");
			if explicit_vcpkg {
				prioritize("vcpkg_cmake", "pkg_config");
				prioritize("vcpkg", "pkg_config");
			}
		} else if explicit_vcpkg {
			prioritize("vcpkg_cmake", "pkg_config");
			prioritize("vcpkg", "pkg_config");
		}

		let probe_list = probes.iter()
			.map(|(name, _)| *name)
			.collect::<Vec<_>>()
			.join(", ");
		eprintln!("=== Probing the OpenCV library in the following order: {}", probe_list);

		let mut out = None;
		for &(name, probe) in &probes {
			if !disabled_probes.contains(name) {
				match probe() {
					Ok(lib) => {
						out = Some(lib);
						break;
					}
					Err(e) => {
						eprintln!("=== Can't probe using: {}, continuing with other methods because: {}", name, e);
					}
				}
			} else {
				eprintln!("=== Skipping probe: {} because of the environment configuration", name);
			}
		}
		out.ok_or_else(|| {
			let methods = probes.iter()
				.map(|&(name, _)| name)
				.filter(|&name| !disabled_probes.contains(name))
				.collect::<Vec<_>>()
				.join(", ");
			format!("Failed to find OpenCV package using probes: {}", methods).into()
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
			println!("{}", meta);
		});
	}
}
