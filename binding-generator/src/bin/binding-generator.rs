use std::{
	env,
	path::PathBuf,
};

use clang::Clang;

use opencv_binding_generator::{
	Generator,
	writer::RustBindingWriter,
};

fn main() {
	let mut args = env::args_os().skip(1);
	let opencv_header_dir = PathBuf::from(args.next().expect("1st argument must be OpenCV header dir"));
	let src_cpp_dir = PathBuf::from(args.next().expect("2nd argument must be dir with custom cpp"));
	let out_dir = PathBuf::from(args.next().expect("3rd argument must be output dir"));
	let module = args.next().expect("4th argument must be module name");
	let module = module.to_str().expect("Not a valid module name");
	let version = args.next().expect("5th argument must be OpenCV version");
	let version = version.to_str().expect("Not a valid version");
	let debug = args.next().map_or(false, |x| x == "1");
	let clang = Clang::new().expect("Cannot initialize clang");
	let bindings_writer = RustBindingWriter::new(&src_cpp_dir, &out_dir, module, version, debug);
	Generator::new(None, &opencv_header_dir, &src_cpp_dir, clang)
		.process_opencv_module(&module, bindings_writer);
}
