use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use opencv_binding_generator::writer::RustNativeBindingWriter;
use opencv_binding_generator::{line_reader, Generator, LineReaderAction};

fn get_version_header(header_dir: &Path) -> Option<PathBuf> {
	let out = header_dir.join("opencv2/core/version.hpp");
	if out.is_file() {
		Some(out)
	} else {
		let out = header_dir.join("opencv2.framework/Headers/core/version.hpp");
		if out.is_file() {
			Some(out)
		} else {
			None
		}
	}
}

fn get_version_from_headers(header_dir: &Path) -> Option<String> {
	let version_hpp = get_version_header(header_dir)?;
	let mut major = None;
	let mut minor = None;
	let mut revision = None;
	let reader = BufReader::new(File::open(version_hpp).ok()?);
	line_reader(reader, |line| {
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
				return LineReaderAction::Break;
			}
		}
		LineReaderAction::Continue
	});
	if let (Some(major), Some(minor), Some(revision)) = (major, minor, revision) {
		Some(format!("{major}.{minor}.{revision}"))
	} else {
		None
	}
}

fn main() {
	let mut args = env::args_os().skip(1);
	let mut opencv_header_dir = args.next();
	let mut debug = false;
	if opencv_header_dir.as_ref().map_or(false, |debug| debug == "--debug") {
		debug = true;
		opencv_header_dir = args.next();
	}
	let opencv_header_dir = PathBuf::from(opencv_header_dir.expect("1st argument must be OpenCV header dir"));
	let src_cpp_dir = PathBuf::from(args.next().expect("2nd argument must be dir with custom cpp"));
	let out_dir = PathBuf::from(args.next().expect("3rd argument must be output dir"));
	let module = args.next().expect("4th argument must be module name");
	let module = module.to_str().expect("Not a valid module name");
	let version = get_version_from_headers(&opencv_header_dir).expect("Can't find the version in the headers");
	let arg_additional_include_dirs = args.next();
	let additional_include_dirs = arg_additional_include_dirs
		.as_ref()
		.and_then(|dirs| dirs.to_str())
		.map(|s| s.split(','))
		.into_iter()
		.flatten()
		.filter(|s| !s.is_empty())
		.map(Path::new)
		.collect::<Vec<_>>();
	let bindings_writer = RustNativeBindingWriter::new(&src_cpp_dir, &out_dir, module, &version, debug);
	Generator::new(&opencv_header_dir, &additional_include_dirs, &src_cpp_dir).process_opencv_module(module, bindings_writer);
}
