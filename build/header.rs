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

/// Something like `/usr/include/x86_64-linux-gnu/opencv4/` on newer Debian-derived distros
///
/// https://wiki.debian.org/Multiarch/Implementation
pub fn get_multiarch_header_dir() -> Option<PathBuf> {
	let try_multiarch = Command::new("dpkg-architecture")
		.args(["--query", "DEB_TARGET_MULTIARCH"])
		.output()
		.inspect_err(|e| eprintln!("=== Failed to get DEB_TARGET_MULTIARCH: {e}"))
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
				eprintln!("=== Failed to get DEB_TARGET_MULTIARCH, trying common multiarch paths");
				vec![
					"x86_64-linux-gnu".to_string(),
					"aarch64-linux-gnu".to_string(),
					"arm-linux-gnueabihf".to_string(),
				]
			},
			|multiarch| vec![multiarch.trim().to_string()],
		);

	eprintln!("=== Trying multiarch paths: {try_multiarch:?}");

	// Get PKG_CONFIG_SYSROOT_DIR if set (used in cross-compilation scenarios)
	let sysroot = env::var_os("PKG_CONFIG_SYSROOT_DIR").map(PathBuf::from);
	if let Some(ref sysroot_path) = sysroot {
		eprintln!("=== PKG_CONFIG_SYSROOT_DIR is set: {}", sysroot_path.display());
	}

	for multiarch in try_multiarch {
		// Try with sysroot first (for cross-compilation)
		if let Some(ref sysroot_path) = sysroot {
			let header_dir = sysroot_path.join(format!("usr/include/{multiarch}/opencv4"));
			eprintln!("=== Checking sysroot multiarch path: {}", header_dir.display());
			if header_dir.is_dir() {
				return Some(header_dir);
			}
		}
		// Fallback to host system paths
		let header_dir = PathBuf::from(format!("/usr/include/{multiarch}/opencv4"));
		if header_dir.is_dir() {
			return Some(header_dir);
		}
	}
	None
}
