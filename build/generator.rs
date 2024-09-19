use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;
use std::{env, fs, thread};

use collector::Collector;
use opencv_binding_generator::{Generator, IteratorExt};

use super::docs::transfer_bindings_to_docs;
use super::{files_with_predicate, Library, Result, MODULES, OUT_DIR, SRC_CPP_DIR, SRC_DIR};

#[path = "generator/collector.rs"]
mod collector;

pub struct BindingGenerator {
	build_script_path: PathBuf,
}

impl BindingGenerator {
	pub fn new(build_script_path: PathBuf) -> Self {
		Self { build_script_path }
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
			p.extension().map_or(true, |ext| !ext.eq_ignore_ascii_case("dll"))
		})?;
		for path in non_dll_files {
			let _ = fs::remove_file(path);
		}

		let modules = MODULES.get().expect("MODULES not initialized");

		self.run(modules, opencv_header_dir, opencv)?;

		Collector::new(modules, &ffi_export_suffix, &target_module_dir, &manual_dir, &OUT_DIR).collect_bindings()?;

		if let Some(target_docs_dir) = target_docs_dir {
			if !target_docs_dir.exists() {
				fs::create_dir(&target_docs_dir)?;
			}
			transfer_bindings_to_docs(&OUT_DIR, &target_docs_dir);
		}

		Ok(())
	}

	fn run(&self, modules: &[String], opencv_header_dir: &Path, opencv: &Library) -> Result<()> {
		let additional_include_dirs = opencv
			.include_paths
			.iter()
			.filter(|&include_path| include_path != opencv_header_dir)
			.map(|path| path.as_path())
			.collect::<Vec<_>>();

		let gen = Generator::new(opencv_header_dir, &additional_include_dirs, &SRC_CPP_DIR);
		if !gen.is_clang_loaded() {
			eprintln!("=== ERROR: Unable to load libclang library, check item #8 in https://github.com/twistedfall/opencv-rust/blob/master/README.md#troubleshooting");
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
		eprintln!("=== Generating {} modules", modules.len());
		thread::scope(|scope| {
			let join_handles = modules
				.iter()
				.map(|module| {
					let token = job_server.acquire().expect("Can't acquire token from job server");
					scope.spawn({
						let additional_include_dirs = additional_include_dirs.as_str();
						move || {
							let module_start = Instant::now();
							let mut bin_generator = Command::new(&self.build_script_path);
							bin_generator
								.arg(opencv_header_dir)
								.arg(&*SRC_CPP_DIR)
								.arg(&*OUT_DIR)
								.arg(module)
								.arg(additional_include_dirs);
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
					})
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
