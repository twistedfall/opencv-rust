use std::env;
use std::path::{Path, PathBuf};

use opencv_binding_generator::version::OpenCVHeaderVersionExt;
use opencv_binding_generator::writer::RustNativeBindingWriter;
use opencv_binding_generator::{Generator, SupportedModule};

fn main() {
	let mut args = env::args_os().skip(1);
	let mut opencv_header_dir = args.next();
	let mut debug = false;
	if opencv_header_dir.as_ref().is_some_and(|debug| debug == "--debug") {
		debug = true;
		opencv_header_dir = args.next();
	}
	let opencv_header_dir = PathBuf::from(opencv_header_dir.expect("1st argument must be OpenCV header dir"));
	let src_cpp_dir = PathBuf::from(args.next().expect("2nd argument must be dir with custom cpp"));
	let out_dir = PathBuf::from(args.next().expect("3rd argument must be output dir"));
	let module = args.next().expect("4th argument must be module name");
	let module = module
		.to_str()
		.and_then(SupportedModule::try_from_opencv_name)
		.unwrap_or_else(|| panic!("Not a valid module name: {module:?}"));
	let version = opencv_header_dir
		.opencv_find_version()
		.expect("Can't find the version in the headers");
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
	let bindings_writer = RustNativeBindingWriter::new(&src_cpp_dir, &out_dir, module, version.clone(), debug);
	Generator::new(&opencv_header_dir, &additional_include_dirs, &src_cpp_dir).generate(module, &version, !debug, bindings_writer);
}
