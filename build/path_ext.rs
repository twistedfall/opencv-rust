use std::ffi::OsStr;
use std::iter;
use std::path::Path;

use super::TARGET_ENV_MSVC;

/// Static library extensions
const STATIC_LIB_EXTENSIONS: [&str; 2] = ["a", "lib"];
/// Framework extensions
const FRAMEWORK_EXTENSIONS: [&str; 1] = ["framework"];
/// Other library extensions where we don't need to distinguish between different types
const OTHER_LIB_EXTENSIONS: [&str; 4] = ["so", "dll", "dylib", "tbd"];
/// To handle the special case on Linux of the libraries named like libopencv_gapi.so.4.12.0
const SO_LIB_SUBSTRINGS: [&str; 1] = [".so."];

#[derive(Debug, Clone, Copy)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum LibraryKind {
	Static,
	Framework,
	Other,
}

pub trait PathExt {
	/// Returns filename with some common library filename parts that are not meant to be passed as part of the link name removed.
	///
	/// E.g. "libopencv_core.so.4.6.0" becomes "opencv_core".
	fn cleanedup_lib_filename(&self) -> Option<&OsStr>;

	/// Checks if the file is a library file (dynamic or static) based on its extension and returns the library kind, `None` otherwise.
	fn library_kind(&self) -> Option<LibraryKind>;
}

impl PathExt for Path {
	fn cleanedup_lib_filename(&self) -> Option<&OsStr> {
		self.file_name().map(|filename| {
			let mut out = filename;
			// strip lib extension from the filename
			if let Some(stem) = self.file_stem() {
				if self.library_kind().is_some() {
					out = stem;
				}
			}
			if let Some(mut file) = out.to_str() {
				let orig_len = file.len();

				// strip "lib" prefix from the filename unless targeting MSVC
				if !*TARGET_ENV_MSVC {
					file = file.strip_prefix("lib").unwrap_or(file);
				}

				// strip lib extension + suffix (e.g. .so.4.6.0) from the filename
				SO_LIB_SUBSTRINGS.into_iter().for_each(|inner_ext| {
					if let Some(inner_ext_idx) = file.rfind(inner_ext) {
						file = &file[..inner_ext_idx];
					}
				});
				if orig_len != file.len() {
					out = OsStr::new(file);
				}
			}
			out
		})
	}

	fn library_kind(&self) -> Option<LibraryKind> {
		self.extension().and_then(|ext| {
			STATIC_LIB_EXTENSIONS
				.into_iter()
				.zip(iter::repeat(LibraryKind::Static))
				.chain(FRAMEWORK_EXTENSIONS.into_iter().zip(iter::repeat(LibraryKind::Framework)))
				.chain(OTHER_LIB_EXTENSIONS.into_iter().zip(iter::repeat(LibraryKind::Other)))
				.find_map(|(check_ext, kind)| ext.eq_ignore_ascii_case(check_ext).then_some(kind))
				.or_else(|| {
					SO_LIB_SUBSTRINGS.into_iter().find_map(|inner_ext| {
						self.file_name().and_then(|filename| {
							filename
								.to_str()
								.and_then(|filename_str| filename_str.contains(inner_ext).then_some(LibraryKind::Other))
						})
					})
				})
		})
	}
}

#[cfg(test)]
mod tests {
	use std::path::Path;

	use super::PathExt;

	#[test]
	fn test_library_kind() {
		let test_cases = vec![
			("libopencv_core.so", Some(super::LibraryKind::Other)),
			("libopencv_core.so.4.6.0", Some(super::LibraryKind::Other)),
			("libopencv_core.a", Some(super::LibraryKind::Static)),
			("libopencv_core.lib", Some(super::LibraryKind::Static)),
			("opencv_core.framework", Some(super::LibraryKind::Framework)),
			("opencv_core.dll", Some(super::LibraryKind::Other)),
			("opencv_core.dll.10", None),
			("some_random_file.txt", None),
		];

		for (filename, expected_kind) in test_cases {
			let path = Path::new(filename);
			assert_eq!(path.library_kind(), expected_kind, "Failed for filename: {}", filename);
		}
	}
}
