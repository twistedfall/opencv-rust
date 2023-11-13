use std::ffi::OsStr;
use std::path::Path;
use std::{env, fs};

use crate::{files_with_extension, GenerateFullBindings, MANIFEST_DIR, OUT_DIR};

pub fn handle_running_in_docsrs() -> GenerateFullBindings {
	if env::var_os("DOCS_RS").is_some() {
		let docs_dir = MANIFEST_DIR.join("docs");
		// fake setup for docs.rs
		println!(r#"cargo:rustc-cfg=ocvrs_opencv_branch_4"#);
		transfer_bindings_from_docs(&docs_dir, &OUT_DIR);
		for path in files_with_extension(&docs_dir, "rs").expect("Can't read hub dir") {
			if let Some(module) = path.file_stem().and_then(OsStr::to_str) {
				println!("cargo:rustc-cfg=ocvrs_has_module_{module}");
			}
		}
		GenerateFullBindings::Stop
	} else {
		GenerateFullBindings::Proceed
	}
}

/// Copies files from docs/ to OUT_DIR, for building in docs.rs
pub fn transfer_bindings_from_docs(src_dir: &Path, out_dir: &Path) {
	let target_dir = out_dir.join("opencv");
	fs::create_dir_all(&target_dir).expect("Can't create target directory");
	for path in files_with_extension(src_dir, "rs").expect("Can't read hub dir") {
		let file_name = path.file_name().expect("Can't get file name");
		fs::copy(&path, target_dir.join(file_name))
			.unwrap_or_else(|e| panic!("Can't copy module file: {}, error: {e}", path.display()));
	}
}

/// Copies files from OUT_DIR to docs/ for generating files for docs.rs
pub fn transfer_bindings_to_docs(out_dir: &Path, target_dir: &Path) {
	let src_dir = out_dir.join("opencv");
	fs::create_dir_all(target_dir).expect("Can't create target directory");
	for path in files_with_extension(target_dir, "rs").expect("Can't read target directory") {
		let _ = fs::remove_file(path);
	}
	for path in files_with_extension(&src_dir, "rs").expect("Can't read hub dir") {
		let file_name = path.file_name().expect("Can't get file name");
		fs::copy(&path, target_dir.join(file_name))
			.unwrap_or_else(|e| panic!("Can't copy module file: {}, error: {e}", path.display()));
	}
}
