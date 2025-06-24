use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;
use std::{env, fs, io, thread};

use collector::Collector;
use opencv_binding_generator::{Generator, IteratorExt, SupportedModule};

use super::docs::transfer_bindings_to_docs;
use super::{
	files_with_predicate, Library, Result, OUT_DIR, SRC_CPP_DIR, SRC_DIR, SUPPORTED_INHERENT_FEATURES, SUPPORTED_MODULES,
	SUPPORTED_OPENCV_BRANCHES,
};

#[path = "generator/collector.rs"]
mod collector;

pub struct BindingGenerator<'r> {
	build_script_path: &'r Path,
	modules: &'r [SupportedModule],
	module_aliases: &'r HashMap<SupportedModule, SupportedModule>,
}

impl<'r> BindingGenerator<'r> {
	pub fn new(
		build_script_path: &'r Path,
		modules: &'r [SupportedModule],
		module_aliases: &'r HashMap<SupportedModule, SupportedModule>,
	) -> Self {
		Self {
			build_script_path,
			modules,
			module_aliases,
		}
	}

	pub fn generate_wrapper(&self, opencv_header_dir: &Path, opencv: &Library, ffi_export_suffix: &str) -> Result<()> {
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
			let extension_is_dll = p.extension().is_some_and(|ext| ext.eq_ignore_ascii_case("dll"));
			!extension_is_dll
		})?;
		for path in non_dll_files {
			let _ = fs::remove_file(path);
		}

		self.run(opencv_header_dir, opencv)?;

		Collector::new(
			self.modules,
			self.module_aliases,
			ffi_export_suffix,
			&target_module_dir,
			&manual_dir,
			&OUT_DIR,
		)
		.collect_bindings()?;
		self.generate_opencv_branch_cond_macros()?;
		self.generate_opencv_module_cond_macros()?;
		self.generate_opencv_inherent_feature_cond_macros()?;

		if let Some(target_docs_dir) = target_docs_dir {
			if !target_docs_dir.exists() {
				fs::create_dir(&target_docs_dir)?;
			}
			transfer_bindings_to_docs(&OUT_DIR, &target_docs_dir);
		}

		Ok(())
	}

	fn generate_opencv_branch_cond_macros(&self) -> Result<()> {
		static COND_MACRO_TPL: &str = include_str!("cond_macros/opencv_branch.rs");

		let mut cond_macros_file = BufWriter::new(File::create(OUT_DIR.join("opencv/cond_macros.rs"))?);
		for (_, branch) in SUPPORTED_OPENCV_BRANCHES {
			write_replace(COND_MACRO_TPL, "OPENCV_BRANCH", branch, &mut cond_macros_file)?;
		}
		Ok(())
	}

	fn generate_opencv_module_cond_macros(&self) -> Result<()> {
		static COND_MACRO_TPL: &str = include_str!("cond_macros/opencv_module.rs");

		let mut cond_macros_file = BufWriter::new(File::options().append(true).open(OUT_DIR.join("opencv/cond_macros.rs"))?);
		for module in SUPPORTED_MODULES {
			write_replace(COND_MACRO_TPL, "OPENCV_MODULE", module.opencv_name(), &mut cond_macros_file)?;
		}
		Ok(())
	}

	fn generate_opencv_inherent_feature_cond_macros(&self) -> Result<()> {
		static COND_MACRO_TPL: &str = include_str!("cond_macros/opencv_inherent_feature.rs");

		let mut cond_macros_file = BufWriter::new(File::options().append(true).open(OUT_DIR.join("opencv/cond_macros.rs"))?);
		for inherent_feature in SUPPORTED_INHERENT_FEATURES {
			write_replace(
				COND_MACRO_TPL,
				"OPENCV_INHERENT_FEATURE",
				inherent_feature,
				&mut cond_macros_file,
			)?;
		}
		Ok(())
	}

	fn run(&self, opencv_header_dir: &Path, opencv: &Library) -> Result<()> {
		let additional_include_dirs = opencv
			.include_paths
			.iter()
			.filter(|&include_path| include_path != opencv_header_dir)
			.map(|path| path.as_path())
			.collect::<Vec<_>>();

		let gen = Generator::new(opencv_header_dir, &additional_include_dirs, &SRC_CPP_DIR);
		if !gen.is_clang_loaded() {
			eprintln!("=== ERROR: Unable to load libclang library, check item #8 in https://github.com/twistedfall/opencv-rust/blob/master/TROUBLESHOOTING.md");
			eprintln!(
				"=== Try enabling `clang-runtime` feature of the `opencv` crate, or alternatively disabling it if it's enabled"
			);
			return Err("a `libclang` shared library is not loaded on this thread".into());
		}
		eprintln!("=== Clang: {}", gen.clang_version());
		eprintln!("=== Clang command line args: {:#?}", gen.build_clang_command_line_args());

		let additional_include_dirs = additional_include_dirs
			.into_iter()
			.map(|p| p.to_str().expect("Can't convert additional include dir to UTF-8 string"))
			.join(",");
		let job_server = Jobserver::build()?;
		let start = Instant::now();
		eprintln!("=== Generating {} modules", self.modules.len());
		thread::scope(|scope| {
			let join_handles = self
				.modules
				.iter()
				.map(|module| {
					let token = job_server.acquire().expect("Can't acquire token from job server");
					let module_opencv_name = module.opencv_name();
					thread::Builder::new()
						.name(format!("gen-{module_opencv_name}"))
						.spawn_scoped(scope, {
							let additional_include_dirs = additional_include_dirs.as_str();
							move || {
								let module_start = Instant::now();
								let mut bin_generator = Command::new(self.build_script_path);
								bin_generator
									.arg(opencv_header_dir)
									.arg(&*SRC_CPP_DIR)
									.arg(&*OUT_DIR)
									.arg(module_opencv_name)
									.arg(additional_include_dirs);
								eprintln!("=== Running: {bin_generator:?}");
								let res = bin_generator.status().unwrap_or_else(|e| {
									panic!("Can't run bindings generator for module: {module_opencv_name}, error: {e}")
								});
								if !res.success() {
									panic!("Failed to run the bindings generator for module: {module_opencv_name}");
								}
								eprintln!(
									"=== Generated module bindings: {module_opencv_name} in: {:?}",
									module_start.elapsed()
								);
								drop(token); // needed to move the token to the thread
							}
						})
						.expect("Error spawning thread")
				})
				.collect::<Vec<_>>();
			for join_handle in join_handles {
				join_handle.join().expect("Generator process panicked");
			}
		});
		eprintln!("=== Total binding generation time: {:?}", start.elapsed());
		Ok(())
	}
}

pub struct Jobserver {
	client: jobserver::Client,
	reacquire_token_on_drop: bool,
}

impl Jobserver {
	pub fn build() -> Result<Self> {
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
					if own_token_released {
						client.acquire_raw().expect("Can't reacquire build script thread token");
					}
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
					.or_else(|| thread::available_parallelism().map(|p| p.get()).ok())
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

fn write_replace(mut content: &str, search: &str, replace: &str, mut to: impl Write) -> io::Result<()> {
	while let Some(search_idx) = content.find(search) {
		let (unchanged_content, new_content) = content.split_at(search_idx);
		to.write_all(unchanged_content.as_bytes())?;
		to.write_all(replace.as_bytes())?;
		content = &new_content[search.len()..];
	}
	to.write_all(content.as_bytes())?;
	Ok(())
}
