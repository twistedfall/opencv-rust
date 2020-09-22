use std::{
	env,
	ffi::OsStr,
	fs::{self, DirEntry, File, OpenOptions},
	io::{self, BufRead, BufReader, Write},
	path::{Path, PathBuf},
	process::{Child, Command},
	sync::Arc,
	thread,
	time::Instant,
};

use glob::glob;

use super::{
	file_copy_to_dir,
	get_versioned_hub_dirs,
	is_core_module,
	MODULES,
	OUT_DIR,
	Result,
	SRC_CPP_DIR,
	SRC_DIR,
};

fn read_dir(path: &Path) -> Result<impl Iterator<Item=DirEntry>> {
	Ok(path.read_dir()?.filter_map(|e| e.ok()))
}

fn copy_indent(mut read: impl BufRead, mut write: impl Write, indent: &str) -> Result<()> {
	let mut line = Vec::with_capacity(100);
	while read.read_until(b'\n', &mut line)? != 0 {
		write.write(indent.as_bytes())?;
		write.write(&line)?;
		line.clear();
	}
	Ok(())
}

fn file_move_to_dir(src_file: &Path, target_dir: &Path) -> Result<PathBuf> {
	if !target_dir.exists() {
		fs::create_dir_all(&target_dir)?;
	}
	let src_filename = src_file.file_name()
		.ok_or_else(|| "Can't calculate filename")?;
	let target_file = target_dir.join(src_filename);
	if fs::rename(&src_file, &target_file).is_err() {
		fs::copy(&src_file, &target_file)?;
		fs::remove_file(src_file)?;
	}
	Ok(target_file)
}

pub fn gen_wrapper(opencv_header_dir: &Path, generator_build: Option<Child>) -> Result<()> {
	let out_dir_as_str = OUT_DIR.to_str().unwrap();
	let (rust_hub_dir, cpp_hub_dir) = get_versioned_hub_dirs();
	let module_dir = rust_hub_dir.join("hub");
	let manual_dir = SRC_DIR.join("manual");
	let opencv_dir = opencv_header_dir.join("opencv2");

	eprintln!("=== Using OpenCV headers from: {}", opencv_dir.display());
	eprintln!("=== Generating code in: {}", out_dir_as_str);
	eprintln!("=== Placing generated bindings into: {}", rust_hub_dir.display());

	let modules = MODULES.get().expect("MODULES not initialized");

	for entry in read_dir(&OUT_DIR)? {
		let path = entry.path();
		if path.is_file() && path.extension().and_then(OsStr::to_str).map_or(true, |ext| !ext.eq_ignore_ascii_case("dll")) {
			let _ = fs::remove_file(path);
		}
	}

	let version = if cfg!(feature = "opencv-32") {
		"3.2.0"
	} else if cfg!(feature = "opencv-34") {
		"3.4.10"
	} else if cfg!(feature = "opencv-4") {
		"4.3.0"
	} else {
		unreachable!();
	};

	let clang_stdlib_include_dir = Arc::new(env::var_os("OPENCV_CLANG_STDLIB_PATH")
		.map(|p| PathBuf::from(p))
	);
	let num_jobs = env::var("NUM_JOBS").ok()
		.and_then(|jobs| jobs.parse().ok())
		.unwrap_or(2);
	let job_server = jobserver::Client::new(num_jobs).expect("Can't create job server");
	let mut join_handles = Vec::with_capacity(modules.len());
	let start;
	let clang = clang::Clang::new().expect("Cannot initialize clang");
	println!("=== Clang: {}", clang::get_version());
	let gen = binding_generator::Generator::new(clang_stdlib_include_dir.as_deref(), &opencv_header_dir, &*SRC_CPP_DIR, clang);
	eprintln!("=== Clang command line args: {:#?}", gen.build_clang_command_line_args());
	if cfg!(feature = "clang-runtime") {
		let status = generator_build.expect("Impossible").wait()?;
		if !status.success() {
			return Err("Failed to build the bindings generator".into());
		}
		let opencv_header_dir = Arc::new(opencv_header_dir.to_owned());
		start = Instant::now();
		modules.iter().for_each(|module| {
			let token = job_server.acquire().expect("Can't acquire token from job server");
			let join_handle = thread::spawn({
				let clang_stdlib_include_dir = Arc::clone(&clang_stdlib_include_dir);
				let opencv_header_dir = Arc::clone(&opencv_header_dir);
				move || {
					let clang_stdlib_include_dir = (*clang_stdlib_include_dir).as_ref()
						.and_then(|p| p.to_str())
						.unwrap_or("None");
					let mut bin_generator = Command::new(OUT_DIR.join("release/binding-generator"));
					bin_generator.arg(&*opencv_header_dir)
						.arg(&*SRC_CPP_DIR)
						.arg(&*OUT_DIR)
						.arg(&module)
						.arg(version)
						.arg(clang_stdlib_include_dir);
					eprintln!("=== Running binding generator binary: {:#?}", bin_generator);
					let res = bin_generator.status().expect("Can't run bindings generator");
					if !res.success() {
						panic!("Failed to run the bindings generator");
					}
					eprintln!("=== Generated: {}", module);
					drop(token); // needed to move the token to the thread
				}
			});
			join_handles.push(join_handle);
		});
	} else {
		let gen = Arc::new(gen);
		start = Instant::now();
		modules.iter().for_each(|module| {
			let token = job_server.acquire().expect("Can't acquire token from job server");
			let join_handle = thread::spawn({
				let gen = Arc::clone(&gen);
				move || {
					let bindings_writer = binding_generator::writer::RustNativeBindingWriter::new(
						&*SRC_CPP_DIR,
						&*OUT_DIR,
						&module,
						version,
						false,
					);
					gen.process_opencv_module(&module, bindings_writer);
					eprintln!("=== Generated: {}", module);
					drop(token); // needed to move the token to the thread
				}
			});
			join_handles.push(join_handle);
		});
	}
	for join_handle in join_handles {
		join_handle.join().expect("Can't join thread");
	}
	eprintln!("=== Total binding generation time: {:?}", start.elapsed());

	if !module_dir.exists() {
		fs::create_dir_all(&module_dir)?;
	}

	for entry in read_dir(&module_dir)? {
		let path = entry.path();
		if path.extension().map_or(false, |e| e == "rs") {
			let _ = fs::remove_file(path);
		}
	}

	if !cpp_hub_dir.exists() {
		fs::create_dir_all(&cpp_hub_dir)?;
	}
	for entry in read_dir(&cpp_hub_dir)? {
		let path = entry.path();
		if path.is_file() {
			let _ = fs::remove_file(path);
		}
	}

	for module in modules {
		let module_cpp = OUT_DIR.join(format!("{}.cpp", module));
		if module_cpp.is_file() {
			file_copy_to_dir(&module_cpp, &cpp_hub_dir)?;
			let module_types_cpp = OUT_DIR.join(format!("{}_types.hpp", module));
			let mut module_types_file = OpenOptions::new().create(true).truncate(true).write(true).open(&module_types_cpp)?;
			let mut type_files: Vec<PathBuf> = glob(&format!("{}/???-{}-*.type.cpp", out_dir_as_str, module))?
				.collect::<Result<_, glob::GlobError>>()?;
			type_files.sort_unstable();
			for entry in type_files.into_iter() {
				io::copy(&mut File::open(entry)?, &mut module_types_file)?;
			}
			file_copy_to_dir(&module_types_cpp, &cpp_hub_dir)?;
		}
	}

	let add_manual = |file: &mut File, mod_name: &str| -> Result<bool> {
		if manual_dir.join(format!("{}.rs", mod_name)).exists() {
			writeln!(file, "pub use crate::manual::{}::*;", mod_name)?;
			Ok(true)
		} else {
			Ok(false)
		}
	};

	{
		let mut hub_rs = File::create(rust_hub_dir.join("hub.rs"))?;

		let mut types_rs = File::create(module_dir.join("types.rs"))?;
		writeln!(&mut types_rs)?;

		let mut sys_rs = File::create(module_dir.join("sys.rs"))?;
		writeln!(&mut sys_rs, "use crate::{{mod_prelude_types::*, core}};")?;
		writeln!(&mut sys_rs)?;
		for module in modules {
			let is_core_module = is_core_module(module.as_str());
			let write_if_contrib = |write: &mut File| -> Result<()> {
				if !is_core_module {
					writeln!(write, r#"#[cfg(feature = "contrib")]"#)?;
				}
				Ok(())
			};
			// hub
			write_if_contrib(&mut hub_rs)?;
			writeln!(&mut hub_rs, "pub mod {};", module)?;
			let module_filename = format!("{}.rs", module);
			let target_file = file_move_to_dir(&OUT_DIR.join(&module_filename), &module_dir)?;
			let mut f = OpenOptions::new().append(true).open(&target_file)?;
			add_manual(&mut f, module)?;

			// types
			let mut write_header = true;
			for entry in glob(&format!("{}/???-{}-*.type.rs", out_dir_as_str, module))? {
				let entry = entry?;
				if entry.metadata().map(|meta| meta.len()).unwrap_or(0) > 0 {
					if write_header {
						write_if_contrib(&mut types_rs)?;
						writeln!(&mut types_rs, "mod {}_types {{", module)?;
						writeln!(&mut types_rs, "\tuse crate::{{mod_prelude::*, core, types, sys}};")?;
						writeln!(&mut types_rs)?;
						write_header = false;
					}
					copy_indent(BufReader::new(File::open(&entry)?), &mut types_rs, "\t")?;
				}
			}
			if !write_header {
				writeln!(&mut types_rs, "}}")?;
				write_if_contrib(&mut types_rs)?;
				writeln!(&mut types_rs, "pub use {}_types::*;", module)?;
				writeln!(&mut types_rs)?;
			}

			// sys
			let path = OUT_DIR.join(format!("{}.externs.rs", module));
			write_if_contrib(&mut sys_rs)?;
			writeln!(&mut sys_rs, "mod {}_sys {{", module)?;
			writeln!(&mut sys_rs, "\tuse super::*;")?;
			writeln!(&mut sys_rs)?;
			for entry in glob(&format!("{}/{}-*.rv.rs", out_dir_as_str, module))? {
				let entry: PathBuf = entry?;
				copy_indent(BufReader::new(File::open(entry)?), &mut sys_rs, "\t")?;
			}
			copy_indent(BufReader::new(File::open(&path)?), &mut sys_rs, "\t")?;
			writeln!(&mut sys_rs, "}}")?;
			write_if_contrib(&mut sys_rs)?;
			writeln!(&mut sys_rs, "pub use {}_sys::*;", module)?;
			writeln!(&mut sys_rs)?;
		}
		writeln!(&mut hub_rs, "pub mod types;")?;
		writeln!(&mut hub_rs, "#[doc(hidden)]")?;
		writeln!(&mut hub_rs, "pub mod sys;")?;

		add_manual(&mut types_rs, "types")?;

		add_manual(&mut sys_rs, "sys")?;

		writeln!(&mut hub_rs, "pub mod hub_prelude {{")?;
		for module in modules {
			if !is_core_module(module.as_str()) {
				writeln!(&mut hub_rs, r#"	#[cfg(feature = "contrib")]"#)?;
			}
			writeln!(&mut hub_rs, r#"	pub use super::{}::prelude::*;"#, module)?;
		}
		writeln!(&mut hub_rs, "}}")?;
	}

	Ok(())
}
