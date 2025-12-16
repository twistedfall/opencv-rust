use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use semver::Version;
use shlex::Shlex;

use super::library::{LinkLib, LinkSearch, Linkage};
use super::Result;

pub struct ProbeResult {
	pub version: Option<Version>,
	pub include_paths: Vec<PathBuf>,
	pub link_paths: Vec<LinkSearch>,
	pub link_libs: Vec<LinkLib>,
}

pub struct CmakeProbe<'r> {
	cmake_bin: &'r Path,
	build_dir: PathBuf,
	src_dir: &'r Path,
	package_name: &'r str,
	toolchain: Option<&'r Path>,
	is_release: bool,
	args: Option<&'r str>,
}

impl<'r> CmakeProbe<'r> {
	pub fn new(
		cmake_bin: Option<&'r Path>,
		build_dir: &Path,
		src_dir: &'r Path,
		package_name: &'r str,
		toolchain: Option<&'r Path>,
		is_release: bool,
		args: Option<&'r str>,
	) -> Self {
		Self {
			cmake_bin: cmake_bin.unwrap_or(Path::new("cmake")),
			build_dir: build_dir.join("cmake_probe_build"),
			src_dir,
			package_name,
			toolchain,
			is_release,
			args,
		}
	}

	fn prepare(&self) -> Result<()> {
		self.cleanup()?;
		fs::create_dir(&self.build_dir)?;
		Ok(())
	}

	fn cleanup(&self) -> Result<()> {
		if self.build_dir.exists() {
			fs::remove_dir_all(&self.build_dir)?;
		}
		Ok(())
	}

	fn make_cmd(&self) -> Command {
		let mut out = Command::new(self.cmake_bin);
		out.current_dir(&self.build_dir)
			.args(["-S"])
			.arg(self.src_dir)
			.arg(format!("-DOCVRS_PACKAGE_NAME={}", &self.package_name));

		if let Some(toolchain) = self.toolchain {
			out.arg(format!(
				"-DCMAKE_TOOLCHAIN_FILE={}",
				toolchain.to_str().expect("Non-UTF-8 toolchain location")
			));
		}
		if self.is_release {
			out.arg("-DCMAKE_BUILD_TYPE=Release");
		} else {
			out.arg("-DCMAKE_BUILD_TYPE=Debug");
		}
		if let Some(args) = &self.args {
			for arg in Shlex::new(args) {
				out.arg(arg);
			}
		}
		out
	}

	fn extract_from_output(output: &Output, version: &mut Option<Version>, opencv_include_paths: &mut Vec<PathBuf>) -> Result<()> {
		if output.status.success() {
			let mut line = String::with_capacity(256);
			let mut reader = BufReader::new(output.stderr.as_slice());
			while let Ok(bytes_read) = reader.read_line(&mut line) {
				if bytes_read == 0 {
					break;
				}
				if let Some((key, value)) = line.strip_prefix("OCVRS_").and_then(|kv| kv.split_once(':')) {
					match key {
						"INCLUDE_DIRS" => {
							opencv_include_paths.extend(value.split(';').filter(|s| !s.is_empty()).map(|s| PathBuf::from(s.trim())));
						}
						"VERSION" => {
							*version = Some(Version::parse(value.trim())?);
						}
						_ => {}
					}
				}
				line.clear();
			}
			Ok(())
		} else {
			Err(
				format!(
					"cmake returned an error\n    stdout: {}\n    stderr: {}",
					String::from_utf8_lossy(&output.stdout),
					String::from_utf8_lossy(&output.stderr)
				)
				.into(),
			)
		}
	}

	pub(crate) fn extract_from_cmdline(
		cmdline: &str,
		skip_cmd: bool,
		include_paths: &mut Vec<PathBuf>,
		link_paths: &mut Vec<LinkSearch>,
		link_libs: &mut Vec<LinkLib>,
	) {
		eprintln!("=== Extracting build arguments from: {cmdline}");
		let mut args = Shlex::new(cmdline.trim());
		if skip_cmd {
			args.next();
		}
		while let Some(arg) = args.next() {
			let arg = arg.trim();
			if Self::skip_ignorable_arg(&mut args, arg) {
				continue;
			}
			if let Some(path) = arg.strip_prefix("-I") {
				let path = PathBuf::from(path.trim_start());
				if !include_paths.contains(&path) {
					include_paths.push(path);
				}
			} else if let Some(path) = arg.strip_prefix("-L").or_else(|| arg.strip_prefix("-Wl,-rpath,")) {
				let path = LinkSearch(Linkage::Default, PathBuf::from(path.trim_start()));
				if !link_paths.contains(&path) {
					link_paths.push(path);
				}
			} else if let Some(lib) = arg.strip_prefix("-l") {
				// unresolved cmake dependency specification like Qt5::Core
				if !lib.contains("::") && lib != "gflags_shared" {
					link_libs.push(LinkLib(Linkage::Default, lib.trim_start().to_string()));
				}
			} else if let Some(framework) = arg.strip_prefix("-framework") {
				let framework = framework.trim_start();
				let framework = if framework.is_empty() {
					args.next().expect("No framework name after -framework")
				} else {
					framework.to_string()
				};
				link_libs.push(LinkLib(Linkage::Framework, framework));
			} else if !arg.starts_with('-') {
				let path = Path::new(arg);
				if let Some(link_lib) = LinkLib::try_from_library_path(path) {
					if let Some(parent) = path.parent().map(|p| p.to_owned()) {
						// only care about non-empty parent paths
						if parent.components().next().is_some() {
							let search_path = LinkSearch(link_lib.0, parent);
							if !link_paths.contains(&search_path) {
								link_paths.push(search_path);
							}
						}
					}
					link_libs.push(link_lib);
				}
			} else {
				eprintln!("=== Unexpected cmake compiler argument found: {arg}");
			}
		}
	}

	fn skip_ignorable_arg(args: &mut Shlex, arg: &str) -> bool {
		if let Some(output_file) = arg.strip_prefix("-o") {
			if output_file.trim().is_empty() {
				args.next().expect("No output file after -o");
				return true;
			}
		} else if let Some(sysroot) = arg.strip_prefix("-isysroot") {
			if sysroot.trim().is_empty() {
				args.next().expect("No sysroot path after -isysroot");
				return true;
			}
		} else if let Some(arch) = arg.strip_prefix("-arch") {
			if arch.trim().is_empty() {
				args.next().expect("No arch name after -arch");
				return true;
			}
		}
		false
	}

	fn extract_from_makefile(&self, link_paths: &mut Vec<LinkSearch>, link_libs: &mut Vec<LinkLib>) -> Result<()> {
		let link_cmdline = fs::read_to_string(self.build_dir.join("CMakeFiles/ocvrs_probe.dir/link.txt"))?;
		Self::extract_from_cmdline(&link_cmdline, true, &mut vec![], link_paths, link_libs);
		Ok(())
	}

	fn extract_from_ninja(
		&self,
		include_paths: &mut Vec<PathBuf>,
		link_paths: &mut Vec<LinkSearch>,
		link_libs: &mut Vec<LinkLib>,
	) -> Result<()> {
		let mut link_cmdline = BufReader::new(File::open(self.build_dir.join("build.ninja"))?);
		let mut line = String::with_capacity(2048);
		#[derive(Copy, Clone)]
		enum State {
			Searching,
			Reading,
		}
		let mut state = State::Searching;
		while let Ok(bytes_read) = link_cmdline.read_line(&mut line) {
			if bytes_read == 0 {
				break;
			}
			match state {
				State::Searching => {
					if line.starts_with("build ocvrs_probe") {
						state = State::Reading;
					}
				}
				State::Reading => {
					let trimmed_line = line.trim_start();
					if let Some(paths) = trimmed_line.strip_prefix("LINK_PATH = ") {
						Self::extract_from_cmdline(paths, false, include_paths, link_paths, link_libs);
					} else if let Some(libs) = trimmed_line.strip_prefix("LINK_LIBRARIES = ") {
						Self::extract_from_cmdline(libs, false, include_paths, link_paths, link_libs);
					}
				}
			}
			line.clear();
		}
		Ok(())
	}

	pub fn probe_makefile(&self) -> Result<ProbeResult> {
		self.prepare()?;

		let mut cmd = self.make_cmd();
		cmd.args(["-G", "Unix Makefiles"]);

		let mut version = None;
		let mut include_paths = Vec::with_capacity(2);
		let mut link_paths = Vec::with_capacity(2);
		let mut link_libs = Vec::with_capacity(64);

		eprintln!("=== cmake makefiles probe command: {cmd:?}");
		cmd.output()
			.map_err(Box::<dyn std::error::Error>::from)
			.and_then(|output| Self::extract_from_output(&output, &mut version, &mut include_paths))?;

		self.extract_from_makefile(&mut link_paths, &mut link_libs)?;

		self.cleanup()?;
		Ok(ProbeResult {
			version,
			include_paths,
			link_paths,
			link_libs,
		})
	}

	pub fn probe_ninja(&self, ninja_bin: Option<&Path>) -> Result<ProbeResult> {
		self.prepare()?;

		let mut cmd = self.make_cmd();
		cmd.args(["-G", "Ninja"]);
		if let Some(ninja_bin) = ninja_bin {
			cmd.arg(format!(
				"-DCMAKE_MAKE_PROGRAM={}",
				ninja_bin.to_str().expect("Non-UTF-8 ninja location")
			));
		}

		let mut version = None;
		let mut include_paths = Vec::with_capacity(2);
		let mut link_paths = Vec::with_capacity(2);
		let mut link_libs = Vec::with_capacity(64);

		eprintln!("=== cmake ninja probe command: {cmd:?}");
		cmd.output()
			.map_err(Box::<dyn std::error::Error>::from)
			.and_then(|output| Self::extract_from_output(&output, &mut version, &mut include_paths))?;

		self.extract_from_ninja(&mut include_paths, &mut link_paths, &mut link_libs)?;

		self.cleanup()?;
		Ok(ProbeResult {
			version,
			include_paths,
			link_paths,
			link_libs,
		})
	}

	pub fn probe_find_package(&self) -> Result<ProbeResult> {
		self.prepare()?;

		let mut include_paths = Vec::with_capacity(2);
		let mut link_paths = Vec::with_capacity(2);
		let mut link_libs = Vec::with_capacity(64);
		let mut cmd = self.make_cmd();
		cmd.args(["--find-package", "-DCOMPILER_ID=GNU", "-DLANGUAGE=CXX", "-DMODE=COMPILE"])
			.arg(format!("-DNAME={}", self.package_name));
		eprintln!("=== cmake find-package compile probe command: {cmd:?}");
		cmd.output().map_err(Box::<dyn std::error::Error>::from).and_then(|output| {
			if output.status.success() {
				let stdout = String::from_utf8(output.stdout)?;
				eprintln!("=== cmake include arguments: {stdout:#?}");
				Self::extract_from_cmdline(&stdout, false, &mut include_paths, &mut link_paths, &mut link_libs);
				Ok(())
			} else {
				Err(
					format!(
						"cmake returned an error\n    stdout: {}\n    stderr: {}",
						String::from_utf8_lossy(&output.stdout),
						String::from_utf8_lossy(&output.stderr)
					)
					.into(),
				)
			}
		})?;

		cmd = self.make_cmd();
		cmd.args(["--find-package", "-DCOMPILER_ID=GNU", "-DLANGUAGE=CXX", "-DMODE=LINK"])
			.arg(format!("-DNAME={}", self.package_name));
		eprintln!("=== cmake find-package link probe command: {cmd:?}");
		cmd.output().map_err(Box::<dyn std::error::Error>::from).and_then(|output| {
			if output.status.success() {
				let stdout = String::from_utf8(output.stdout)?;
				eprintln!("=== cmake link arguments: {stdout:#?}");
				Self::extract_from_cmdline(&stdout, false, &mut include_paths, &mut link_paths, &mut link_libs);
				Ok(())
			} else {
				Err(
					format!(
						"cmake returned an error\n    stdout: {}\n    stderr: {}",
						String::from_utf8_lossy(&output.stdout),
						String::from_utf8_lossy(&output.stderr)
					)
					.into(),
				)
			}
		})?;

		Ok(ProbeResult {
			version: None,
			include_paths,
			link_paths,
			link_libs,
		})
	}
}
