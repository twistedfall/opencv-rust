use std::ffi::{OsStr, OsString};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Arc;
use std::time::Instant;
use std::{env, fs, io, thread};

use opencv_binding_generator::{Generator, IteratorExt};

use crate::docs::transfer_bindings_to_docs;

use super::{files_with_extension, files_with_predicate, Library, Result, MODULES, OUT_DIR, SRC_CPP_DIR, SRC_DIR};

pub struct BindingGenerator {
	build_script_path: OsString,
}

impl BindingGenerator {
	pub fn new(build_script_path: OsString) -> Self {
		Self { build_script_path }
	}

	pub fn generate_wrapper(&self, opencv_header_dir: &Path, opencv: &Library) -> Result<()> {
		let target_docs_dir = env::var_os("OCVRS_DOCS_GENERATE_DIR").map(PathBuf::from);
		let target_module_dir = OUT_DIR.join("opencv");
		let manual_dir = SRC_DIR.join("manual");

		eprintln!("=== Generating code in: {}", OUT_DIR.display());
		eprintln!("=== Placing generated bindings into: {}", target_module_dir.display());
		if let Some(target_docs_dir) = target_docs_dir.as_ref() {
			eprintln!(
				"=== Placing static generated docs bindings into: {}",
				target_docs_dir.display()
			);
		}
		eprintln!("=== Using OpenCV headers from: {}", opencv_header_dir.display());

		let non_dll_files = files_with_predicate(&OUT_DIR, |p| {
			p.extension().map_or(true, |ext| !ext.eq_ignore_ascii_case("dll"))
		})?;
		for path in non_dll_files {
			let _ = fs::remove_file(path);
		}

		let modules = MODULES.get().expect("MODULES not initialized");

		self.run(modules, opencv_header_dir, opencv)?;

		collect_generated_bindings(modules, &target_module_dir, &manual_dir)?;

		if let Some(target_docs_dir) = target_docs_dir {
			if !target_docs_dir.exists() {
				fs::create_dir(&target_docs_dir)?;
			}
			transfer_bindings_to_docs(&OUT_DIR, &target_docs_dir);
		}

		Ok(())
	}

	fn run(&self, modules: &'static [String], opencv_header_dir: &Path, opencv: &Library) -> Result<()> {
		let additional_include_dirs = opencv
			.include_paths
			.iter()
			.map(|path| path.as_path())
			.filter(|&include_path| include_path != opencv_header_dir)
			.collect::<Vec<_>>();

		let gen = Generator::new(opencv_header_dir, &additional_include_dirs, &SRC_CPP_DIR);
		eprintln!("=== Clang: {}", gen.clang_version());
		eprintln!("=== Clang command line args: {:#?}", gen.build_clang_command_line_args());

		let additional_include_dirs = Arc::new(
			additional_include_dirs
				.into_iter()
				.map(|p| p.to_str().expect("Can't convert additional include dir to UTF-8 string"))
				.join(","),
		);
		let opencv_header_dir = Arc::new(opencv_header_dir.to_owned());
		let job_server = build_job_server()?;
		let mut join_handles = Vec::with_capacity(modules.len());
		let start = Instant::now();
		// todo use thread::scope when MSRV is 1.63
		eprintln!("=== Generating {} modules", modules.len());
		modules.iter().for_each(|module| {
			let token = job_server.acquire().expect("Can't acquire token from job server");
			let join_handle = thread::spawn({
				let additional_include_dirs = Arc::clone(&additional_include_dirs);
				let opencv_header_dir = Arc::clone(&opencv_header_dir);
				let build_script_path = self.build_script_path.clone();
				move || {
					let module_start = Instant::now();
					let mut bin_generator = Command::new(build_script_path);
					bin_generator
						.arg(&*opencv_header_dir)
						.arg(&*SRC_CPP_DIR)
						.arg(&*OUT_DIR)
						.arg(module)
						.arg(&*additional_include_dirs);
					eprintln!("=== Running: {bin_generator:?}");
					let res = bin_generator
						.status()
						.unwrap_or_else(|e| panic!("Can't run bindings generator for module: {module}, error: {e}"));
					if !res.success() {
						panic!("Failed to run the bindings generator for module: {module}");
					}
					eprintln!("=== Generated: {module} in {:?}", module_start.elapsed());
					drop(token); // needed to move the token to the thread
				}
			});
			join_handles.push(join_handle);
		});
		for join_handle in join_handles {
			join_handle.join().expect("Generator process panicked");
		}
		eprintln!("=== Total binding generation time: {:?}", start.elapsed());
		Ok(())
	}
}

fn is_type_file(path: &Path, module: &str) -> bool {
	path.file_stem().and_then(OsStr::to_str).map_or(false, |stem| {
		let mut stem_chars = stem.chars();
		(&mut stem_chars).take(3).all(|c| c.is_ascii_digit()) && // first 3 chars are digits
			matches!(stem_chars.next(), Some('-')) && // dash
			module.chars().zip(&mut stem_chars).all(|(m, s)| m == s) && // module name
			matches!(stem_chars.next(), Some('-')) && // dash
			stem.ends_with(".type") // ends with ".type"
	})
}

fn is_type_externs_file(path: &Path, module: &str) -> bool {
	path.file_stem().and_then(OsStr::to_str).map_or(false, |stem| {
		let mut stem_chars = stem.chars();
		(&mut stem_chars).take(3).all(|c| c.is_ascii_digit()) && // first 3 chars are digits
			matches!(stem_chars.next(), Some('-')) && // dash
			module.chars().zip(&mut stem_chars).all(|(m, s)| m == s) && // module name
			matches!(stem_chars.next(), Some('-')) && // dash
			stem.ends_with(".type.externs") // ends with ".type"
	})
}

fn copy_indent(mut read: impl BufRead, mut write: impl Write, indent: &str) -> Result<()> {
	let mut line = Vec::with_capacity(100);
	while read.read_until(b'\n', &mut line)? != 0 {
		write.write_all(indent.as_bytes())?;
		write.write_all(&line)?;
		line.clear();
	}
	Ok(())
}

fn collect_generated_bindings(modules: &[String], target_module_dir: &Path, manual_dir: &Path) -> Result<()> {
	if !target_module_dir.exists() {
		fs::create_dir(target_module_dir)?;
	}
	for path in files_with_extension(target_module_dir, "rs")? {
		let _ = fs::remove_file(path);
	}

	fn write_has_module(mut write: impl Write, module: &str) -> Result<()> {
		Ok(writeln!(write, "#[cfg(ocvrs_has_module_{module})]")?)
	}

	fn write_module_include(write: &mut BufWriter<File>, module: &str) -> Result<()> {
		// Use include instead of #[path] attribute because rust-analyzer doesn't handle #[path] inside other include! too well:
		// https://github.com/twistedfall/opencv-rust/issues/418
		// https://github.com/rust-lang/rust-analyzer/issues/11682
		Ok(writeln!(
			write,
			r#"include!(concat!(env!("OUT_DIR"), "/opencv/{module}.rs"));"#
		)?)
	}

	let add_manual = |file: &mut BufWriter<File>, module: &str| -> Result<bool> {
		if manual_dir.join(format!("{module}.rs")).exists() {
			writeln!(file, "pub use crate::manual::{module}::*;")?;
			Ok(true)
		} else {
			Ok(false)
		}
	};

	let start = Instant::now();
	let mut hub_rs = BufWriter::new(File::create(target_module_dir.join("hub.rs"))?);

	let mut types_rs = BufWriter::new(File::create(target_module_dir.join("types.rs"))?);
	writeln!(types_rs)?;

	let mut sys_rs = BufWriter::new(File::create(target_module_dir.join("sys.rs"))?);
	writeln!(sys_rs, "use crate::{{mod_prelude_sys::*, core}};")?;
	writeln!(sys_rs)?;

	for module in modules {
		// merge multiple *-type.cpp files into a single module_types.hpp
		let module_cpp = OUT_DIR.join(format!("{module}.cpp"));
		if module_cpp.is_file() {
			let module_types_cpp = OUT_DIR.join(format!("{module}_types.hpp"));
			let mut module_types_file = BufWriter::new(
				OpenOptions::new()
					.create(true)
					.truncate(true)
					.write(true)
					.open(module_types_cpp)?,
			);
			let mut type_files = files_with_extension(&OUT_DIR, "cpp")?
				.filter(|f| is_type_file(f, module))
				.collect::<Vec<_>>();
			type_files.sort_unstable();
			for entry in type_files {
				io::copy(&mut BufReader::new(File::open(&entry)?), &mut module_types_file)?;
				let _ = fs::remove_file(entry);
			}
		}

		// add module entry to hub.rs and move the module file into opencv/
		write_has_module(&mut hub_rs, module)?;
		write_module_include(&mut hub_rs, module)?;
		let module_filename = format!("{module}.rs");
		let module_src_file = OUT_DIR.join(&module_filename);
		let mut module_rs = BufWriter::new(File::create(&target_module_dir.join(&module_filename))?);
		// Need to wrap modules inside `mod { }` because they have top-level comments (//!) and those don't play well when
		// module file is include!d (as opposed to connecting the module with `mod` from the parent module).
		// The same doesn't apply to `sys` and `types` below because they don't contain top-level comments.
		writeln!(module_rs, "pub mod {module} {{")?;
		copy_indent(BufReader::new(File::open(&module_src_file)?), &mut module_rs, "\t")?;
		add_manual(&mut module_rs, module)?;
		writeln!(module_rs, "}}")?;
		let _ = fs::remove_file(module_src_file);

		// merge multiple *-.type.rs files into a single types.rs
		let mut header_written = false;
		let mut type_files = files_with_extension(&OUT_DIR, "rs")?
			.filter(|f| is_type_file(f, module))
			.collect::<Vec<_>>();
		type_files.sort_unstable();
		for entry in type_files {
			if entry.metadata().map(|meta| meta.len()).unwrap_or(0) > 0 {
				if !header_written {
					write_has_module(&mut types_rs, module)?;
					writeln!(types_rs, "mod {module}_types {{")?;
					writeln!(types_rs, "\tuse crate::{{mod_prelude::*, core, types, sys}};")?;
					writeln!(types_rs)?;
					header_written = true;
				}
				copy_indent(BufReader::new(File::open(&entry)?), &mut types_rs, "\t")?;
			}
			let _ = fs::remove_file(entry);
		}
		if header_written {
			writeln!(types_rs, "}}")?;
			write_has_module(&mut types_rs, module)?;
			writeln!(types_rs, "pub use {module}_types::*;")?;
			writeln!(types_rs)?;
		}

		// merge module-specific *.externs.rs and generated type-specific *.type.externs.rs into a single sys.rs
		let externs_rs = OUT_DIR.join(format!("{module}.externs.rs"));
		write_has_module(&mut sys_rs, module)?;
		writeln!(sys_rs, "mod {module}_sys {{")?;
		writeln!(sys_rs, "\tuse super::*;")?;
		writeln!(sys_rs)?;
		writeln!(sys_rs, "\textern \"C\" {{")?;
		copy_indent(BufReader::new(File::open(&externs_rs)?), &mut sys_rs, "\t\t")?;
		let _ = fs::remove_file(externs_rs);
		let mut type_extern_files = files_with_extension(&OUT_DIR, "rs")?
			.filter(|f| is_type_externs_file(f, module))
			.collect::<Vec<_>>();
		type_extern_files.sort_unstable();
		for entry in type_extern_files {
			if entry.metadata().map(|meta| meta.len()).unwrap_or(0) > 0 {
				copy_indent(BufReader::new(File::open(&entry)?), &mut sys_rs, "\t\t")?;
			}
			let _ = fs::remove_file(entry);
		}
		writeln!(sys_rs, "\t}}")?;
		writeln!(sys_rs, "}}")?;
		write_has_module(&mut sys_rs, module)?;
		writeln!(sys_rs, "pub use {module}_sys::*;")?;
		writeln!(sys_rs)?;
	}
	writeln!(hub_rs, "pub mod types {{")?;
	write_module_include(&mut hub_rs, "types")?;
	writeln!(hub_rs, "}}")?;
	writeln!(hub_rs, "#[doc(hidden)]")?;
	writeln!(hub_rs, "pub mod sys {{")?;
	write_module_include(&mut hub_rs, "sys")?;
	writeln!(hub_rs, "}}")?;

	add_manual(&mut types_rs, "types")?;

	add_manual(&mut sys_rs, "sys")?;

	// write hub_prelude that imports all module-specific preludes
	writeln!(hub_rs, "pub mod hub_prelude {{")?;
	for module in modules {
		write!(hub_rs, "\t")?;
		write_has_module(&mut hub_rs, module)?;
		writeln!(hub_rs, "\tpub use super::{module}::prelude::*;")?;
	}
	writeln!(hub_rs, "}}")?;
	eprintln!("=== Total binding collection time: {:?}", start.elapsed());
	Ok(())
}

fn build_job_server() -> Result<Jobserver> {
	unsafe { jobserver::Client::from_env() }
		.and_then(|client| {
			let own_token_released = client.release_raw().is_ok();
			let available_jobs = client.available().unwrap_or(0);
			if available_jobs > 0 {
				eprintln!("=== Using environment job server with the the amount of available jobs: {available_jobs}");
				Some(Jobserver {
					client,
					reacquire_token_on_drop: own_token_released,
				})
			} else {
				client.acquire_raw().expect("Can't reacquire build script thread token");
				eprintln!(
					"=== Available jobs from the environment created jobserver is: {available_jobs} or there is an error reading that value"
				);
				None
			}
		})
		.or_else(|| {
			let num_jobs = env::var("NUM_JOBS")
				.ok()
				.and_then(|jobs| jobs.parse().ok())
				.unwrap_or(2)
				.max(1);
			eprintln!("=== Creating a new job server with num_jobs: {num_jobs}");
			jobserver::Client::new(num_jobs).ok().map(|client| Jobserver {
				client,
				reacquire_token_on_drop: false,
			})
		})
		.ok_or_else(|| "Can't create job server".into())
}

pub struct Jobserver {
	client: jobserver::Client,
	reacquire_token_on_drop: bool,
}

impl Drop for Jobserver {
	fn drop(&mut self) {
		if self.reacquire_token_on_drop {
			self.client.acquire_raw().expect("Can't reacquire build script thread token");
		}
	}
}

impl Deref for Jobserver {
	type Target = jobserver::Client;

	fn deref(&self) -> &Self::Target {
		&self.client
	}
}
