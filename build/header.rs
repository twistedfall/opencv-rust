use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::Command;

use semver::Version;

pub trait IncludePath {
	fn get_module_header_dir(&self) -> Option<PathBuf>;
	fn get_version_header(&self) -> Option<PathBuf>;
	fn get_config_header(&self) -> Option<PathBuf>;
	fn find_version(&self) -> Option<Version>;
	fn find_inherent_features(&self) -> Option<Vec<String>>;
}

impl IncludePath for Path {
	fn get_module_header_dir(&self) -> Option<PathBuf> {
		let mut out = self.join("opencv2.framework/Headers");
		if out.is_dir() {
			return Some(out);
		}
		out = self.join("opencv2");
		if out.is_dir() {
			return Some(out);
		}
		None
	}

	fn get_version_header(&self) -> Option<PathBuf> {
		self
			.get_module_header_dir()
			.map(|dir| dir.join("core/version.hpp"))
			.filter(|hdr| hdr.is_file())
	}

	fn get_config_header(&self) -> Option<PathBuf> {
		self
			.get_module_header_dir()
			.map(|dir| dir.join("cvconfig.h"))
			.filter(|hdr| hdr.is_file())
	}

	fn find_version(&self) -> Option<Version> {
		let version_hpp = self.get_version_header()?;
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
				// todo: MSRV 1.88 use let chains
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
		// todo: MSRV 1.88 use let chains
		if let (Some(major), Some(minor), Some(revision)) = (major, minor, revision) {
			Some(Version::new(major, minor, revision))
		} else {
			None
		}
	}

	fn find_inherent_features(&self) -> Option<Vec<String>> {
		let config_h = self.get_config_header()?;
		let mut out = Vec::with_capacity(64);
		let mut line = String::with_capacity(256);
		let mut reader = BufReader::new(File::open(config_h).ok()?);
		while let Ok(bytes_read) = reader.read_line(&mut line) {
			if bytes_read == 0 {
				break;
			}
			if let Some(feature) = line.strip_prefix("#define HAVE_") {
				out.push(feature.trim().to_lowercase());
			}
			line.clear();
		}
		Some(out)
	}
}

const POSSIBLE_MULTIARCHES: &[&str] = &[
	"x86_64-linux-gnu",
	"aarch64-linux-gnu",
	"arm-linux-gnueabihf",
	"i386-linux-gnu",
	"arm-linux-gnueabi",
];

/// When cross-compiling, try to derive the target's multiarch from Cargo's target triple
///
/// https://doc.rust-lang.org/reference/conditional-compilation.html#target_arch
/// https://wiki.debian.org/Multiarch/Tuples#Architectures_in_Debian
fn target_linux_multiarch_from_cargo() -> Option<&'static str> {
	let target_arch = env::var("CARGO_CFG_TARGET_ARCH").ok()?;
	let target_abi = env::var("CARGO_CFG_TARGET_ABI").ok()?;
	match (target_arch.as_str(), target_abi.as_str()) {
		("x86", _) => Some("i386-linux-gnu"),
		("x86_64", _) => Some("x86_64-linux-gnu"),
		("aarch64", _) => Some("aarch64-linux-gnu"),
		("arm", "eabihf") => Some("arm-linux-gnueabihf"),
		("arm", _) => Some("arm-linux-gnueabi"),
		_ => None,
	}
}

fn target_linux_multiarch_from_path(cmake_dir: &Path) -> Option<&str> {
	cmake_dir.components().rev().find_map(|comp| {
		comp
			.as_os_str()
			.to_str()
			.filter(|comp_str| POSSIBLE_MULTIARCHES.contains(comp_str))
	})
}

pub fn get_multiarch_header_dirs(non_multiarch_include_dirs: &[PathBuf]) -> Vec<PathBuf> {
	let mut try_multiarches = vec![];

	try_multiarches.extend(
		target_linux_multiarch_from_cargo()
			.map(|s| s.to_string())
			.inspect(|target_arch| eprintln!("=== Detected target multiarch from Cargo target: {target_arch}")),
	);
	try_multiarches.extend(
		env::var_os("OpenCV_DIR")
			.and_then(|d| target_linux_multiarch_from_path(&PathBuf::from(d)).map(|s| s.to_string()))
			.inspect(|target_arch| eprintln!("=== Detected target multiarch from cmake OpenCV_DIR: {target_arch}")),
	);
	if try_multiarches.is_empty() {
		// Fallback to detecting the host's multiarch
		try_multiarches.extend(
			Command::new("dpkg-architecture")
				.args(["--query", "DEB_TARGET_MULTIARCH"])
				.output()
				.inspect_err(|e| eprintln!("=== Failed to query DEB_TARGET_MULTIARCH: {e}"))
				.ok()
				.or_else(|| {
					Command::new("cc")
						.arg("-print-multiarch")
						.output()
						.inspect_err(|e| eprintln!("=== Failed to get -print-multiarch: {e}"))
						.ok()
				})
				.and_then(|output| String::from_utf8(output.stdout).ok())
				.map_or_else(
					|| {
						eprintln!("=== Failed to detect multiarch path, trying common paths");
						POSSIBLE_MULTIARCHES.iter().map(|s| s.to_string()).collect::<Vec<_>>()
					},
					|multiarch| vec![multiarch.trim().to_string()],
				),
		);
	};

	eprintln!("=== Trying multiarches: {try_multiarches:?}");

	let mut out = Vec::with_capacity(non_multiarch_include_dirs.len());
	for multiarch in try_multiarches {
		for non_multiarch_include_dir in non_multiarch_include_dirs {
			if let Some(multiarch_include_dir) = add_multiarch_dir_before_opencv4(non_multiarch_include_dir, &multiarch) {
				if multiarch_include_dir.is_dir() {
					out.push(multiarch_include_dir);
				}
			} else if let Some(multiarch_include_dir) = add_multiarch_dir_after_include(non_multiarch_include_dir, &multiarch) {
				if multiarch_include_dir.is_dir() {
					out.push(multiarch_include_dir);
				}
			}
		}
	}
	eprintln!("=== Found multiarch header dirs: {out:?}");
	out
}

/// Finds the "opencv4" component in the path and inserts the multiarch directory before it
///
/// This works starting with OpenCV 4.0.0
fn add_multiarch_dir_before_opencv4(path: &Path, multiarch: &str) -> Option<PathBuf> {
	let opencv4_idx = path
		.components()
		.position(|comp| comp.as_os_str().to_str().is_some_and(|s| s == "opencv4"))?;
	let mut comps = path.components();
	let mut out = PathBuf::with_capacity(path.as_os_str().len() + multiarch.len() + 1);
	out.extend((&mut comps).take(opencv4_idx));
	out.push(multiarch);
	out.extend(comps);
	Some(out)
}

/// Finds the "include" component in the path and inserts the multiarch directory after it
///
/// This works starting for OpenCV 3.4 as it doesn't have the "opencv4" component
fn add_multiarch_dir_after_include(path: &Path, multiarch: &str) -> Option<PathBuf> {
	path.ends_with("include").then(|| path.join(multiarch))
}
