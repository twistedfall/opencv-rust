use std::env::ArgsOs;
use std::ffi::OsString;
use std::iter::Peekable;
use std::path::{Path, PathBuf};

use opencv_binding_generator::writer::RustNativeBindingWriter;
use opencv_binding_generator::Generator;

use crate::{get_version_from_headers, GenerateFullBindings, Result};

/// Because clang can't be used from multiple threads we run the binding generator helper for each
/// module as a separate process. Building an additional helper binary from the build script is problematic
/// so we employ the trick and we actually run the build script itself again with some additional arguments.
/// When those arguments are detected the build script will generate the bindings for a single
/// OpenCV module instead of running its main logic.
pub fn handle_running_binding_generator(mut args: Peekable<ArgsOs>) -> Result<GenerateFullBindings> {
	if args.peek().is_some() {
		run(args)?;
		Ok(GenerateFullBindings::Stop)
	} else {
		Ok(GenerateFullBindings::Proceed)
	}
}

pub fn run(mut args: impl Iterator<Item = OsString>) -> Result<()> {
	let opencv_header_dir = PathBuf::from(args.next().ok_or("1st argument must be OpenCV header dir")?);
	let src_cpp_dir = PathBuf::from(args.next().ok_or("2nd argument must be dir with custom cpp")?);
	let out_dir = PathBuf::from(args.next().ok_or("3rd argument must be output dir")?);
	let module = args.next().ok_or("4th argument must be module name")?;
	let module = module.to_str().ok_or("Not a valid module name")?;
	let version = get_version_from_headers(&opencv_header_dir)
		.ok_or("Can't find the version in the headers")?
		.to_string();
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
	let bindings_writer = RustNativeBindingWriter::new(&src_cpp_dir, &out_dir, module, &version, false);
	Generator::new(&opencv_header_dir, &additional_include_dirs, &src_cpp_dir).process_opencv_module(module, bindings_writer);
	Ok(())
}
