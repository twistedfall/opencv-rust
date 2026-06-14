use std::fs::File;
use std::io::BufReader;
use std::ops::ControlFlow;
use std::path::{Path, PathBuf};

use semver::Version;

use crate::line_reader;

pub trait OpenCVHeaderVersionExt {
	fn opencv_find_module_header_dir(&self) -> Option<PathBuf>;
	fn opencv_find_version_header(&self) -> Option<PathBuf>;
	fn opencv_find_version(&self) -> Option<Version>;
}

impl OpenCVHeaderVersionExt for Path {
	fn opencv_find_module_header_dir(&self) -> Option<PathBuf> {
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

	fn opencv_find_version_header(&self) -> Option<PathBuf> {
		self
			.opencv_find_module_header_dir()
			.map(|dir| dir.join("core/version.hpp"))
			.filter(|hdr| hdr.is_file())
	}

	fn opencv_find_version(&self) -> Option<Version> {
		let version_hpp = self.opencv_find_version_header()?;
		let mut major = None;
		let mut minor = None;
		let mut revision = None;
		let reader = BufReader::new(File::open(version_hpp).ok()?);
		line_reader(reader, |line| {
			if let Some(line) = line.strip_prefix("#define CV_VERSION_") {
				let mut parts = line.split_whitespace();
				if let Some(ver_spec) = parts.next()
					&& let Some(version) = parts.next()
				{
					match ver_spec {
						"MAJOR" => major = version.parse().ok(),
						"MINOR" => minor = version.parse().ok(),
						"REVISION" => revision = version.parse().ok(),
						_ => {}
					}
				}
				if major.is_some() && minor.is_some() && revision.is_some() {
					return ControlFlow::Break(());
				}
			}
			ControlFlow::Continue(())
		});
		if let Some(major) = major
			&& let Some(minor) = minor
			&& let Some(revision) = revision
		{
			Some(Version::new(major, minor, revision))
		} else {
			None
		}
	}
}

pub trait OpenCVVersionExt {
	fn is_opencv_5(&self) -> bool;
}

impl OpenCVVersionExt for Version {
	fn is_opencv_5(&self) -> bool {
		self.major == 5
	}
}
