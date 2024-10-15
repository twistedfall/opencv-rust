use std::ffi::OsStr;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;
use std::time::Instant;
use std::{fs, io};

use super::super::{files_with_extension, Result};

pub struct Collector<'r> {
	modules: &'r [String],
	ffi_export_suffix: &'r str,
	target_module_dir: &'r Path,
	manual_dir: &'r Path,
	out_dir: &'r Path,
}

impl<'r> Collector<'r> {
	pub fn new(
		modules: &'r [String],
		ffi_export_suffix: &'r str,
		target_module_dir: &'r Path,
		manual_dir: &'r Path,
		out_dir: &'r Path,
	) -> Self {
		Self {
			modules,
			ffi_export_suffix,
			target_module_dir,
			manual_dir,
			out_dir,
		}
	}

	pub fn collect_bindings(&self) -> Result<()> {
		if !self.target_module_dir.exists() {
			fs::create_dir(self.target_module_dir)?;
		}
		for path in files_with_extension(self.target_module_dir, "rs")? {
			let _ = fs::remove_file(path);
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

		let start = Instant::now();
		let mut hub_rs = BufWriter::new(File::create(self.target_module_dir.join("hub.rs"))?);

		let mut types_rs = BufWriter::new(File::create(self.target_module_dir.join("types.rs"))?);
		writeln!(types_rs)?;

		let mut sys_rs = BufWriter::new(File::create(self.target_module_dir.join("sys.rs"))?);
		writeln!(sys_rs, "use crate::{{mod_prelude_sys::*, core}};")?;
		writeln!(sys_rs)?;

		for module in self.modules {
			// add module entry to hub.rs
			write_has_module(&mut hub_rs, module)?;
			write_module_include(&mut hub_rs, module)?;
			self.collect_module(module, &mut sys_rs, &mut types_rs)?
		}
		writeln!(hub_rs, "pub mod types {{")?;
		write!(hub_rs, "\t")?;
		write_module_include(&mut hub_rs, "types")?;
		writeln!(hub_rs, "}}")?;
		writeln!(hub_rs, "#[doc(hidden)]")?;
		writeln!(hub_rs, "pub mod sys {{")?;
		write!(hub_rs, "\t")?;
		write_module_include(&mut hub_rs, "sys")?;
		writeln!(hub_rs, "}}")?;

		self.write_use_manual(&mut types_rs, "types")?;

		self.write_use_manual(&mut sys_rs, "sys")?;

		// write hub_prelude that imports all module-specific preludes
		writeln!(hub_rs, "pub mod hub_prelude {{")?;
		for module in self.modules {
			write!(hub_rs, "\t")?;
			write_has_module(&mut hub_rs, module)?;
			writeln!(hub_rs, "\tpub use super::{module}::prelude::*;")?;
		}
		writeln!(hub_rs, "}}")?;
		self.inject_ffi_exports(&mut hub_rs)?;
		eprintln!("=== Total binding collection time: {:?}", start.elapsed());
		Ok(())
	}

	fn collect_module(&self, module: &str, sys_rs: &mut impl Write, types_rs: &mut impl Write) -> Result<()> {
		// merge multiple *-type.cpp files into a single module_types.hpp
		let module_cpp = self.out_dir.join(format!("{module}.cpp"));
		if module_cpp.is_file() {
			let module_types_cpp = self.out_dir.join(format!("{module}_types.hpp"));
			let mut module_types_file = BufWriter::new(
				OpenOptions::new()
					.create(true)
					.truncate(true)
					.write(true)
					.open(module_types_cpp)?,
			);
			let mut type_files = files_with_extension(self.out_dir, "cpp")?
				.filter(|f| is_type_file(f, module))
				.collect::<Vec<_>>();
			type_files.sort_unstable();
			for entry in type_files {
				io::copy(&mut BufReader::new(File::open(&entry)?), &mut module_types_file)?;
				let _ = fs::remove_file(entry);
			}
		}

		// move the module file into opencv/
		let module_filename = format!("{module}.rs");
		let module_src_file = self.out_dir.join(&module_filename);
		let mut module_rs = BufWriter::new(File::create(self.target_module_dir.join(&module_filename))?);
		// Need to wrap modules inside `mod { }` because they have top-level comments (//!) and those don't play well when
		// module file is include!d (as opposed to connecting the module with `mod` from the parent module).
		// The same doesn't apply to `sys` and `types` below because they don't contain top-level comments.
		writeln!(module_rs, "pub mod {module} {{")?;
		copy_indent(BufReader::new(File::open(&module_src_file)?), &mut module_rs, "\t")?;
		self.write_use_manual(&mut module_rs, module)?;
		writeln!(module_rs, "}}")?;
		let _ = fs::remove_file(module_src_file);

		// merge multiple *-.type.rs files into a single types.rs
		let mut header_written = false;
		let mut type_files = files_with_extension(self.out_dir, "rs")?
			.filter(|f| is_type_file(f, module))
			.collect::<Vec<_>>();
		type_files.sort_unstable();
		for entry in type_files {
			if entry.metadata().map(|meta| meta.len()).unwrap_or(0) > 0 {
				if !header_written {
					write_has_module(types_rs, module)?;
					writeln!(types_rs, "mod {module}_types {{")?;
					writeln!(types_rs, "\tuse crate::{{mod_prelude::*, core, types, sys}};")?;
					writeln!(types_rs)?;
					header_written = true;
				}
				copy_indent(BufReader::new(File::open(&entry)?), types_rs, "\t")?;
			}
			let _ = fs::remove_file(entry);
		}
		if header_written {
			writeln!(types_rs, "}}")?;
			write_has_module(types_rs, module)?;
			writeln!(types_rs, "pub use {module}_types::*;")?;
			writeln!(types_rs)?;
		}

		// merge module-specific *.externs.rs and generated type-specific *.type.externs.rs into a single sys.rs
		let externs_rs = self.out_dir.join(format!("{module}.externs.rs"));
		write_has_module(sys_rs, module)?;
		writeln!(sys_rs, "mod {module}_sys {{")?;
		writeln!(sys_rs, "\tuse super::*;")?;
		writeln!(sys_rs)?;
		writeln!(sys_rs, "\textern \"C\" {{")?;
		copy_indent(BufReader::new(File::open(&externs_rs)?), sys_rs, "\t\t")?;
		let _ = fs::remove_file(externs_rs);
		let mut type_extern_files = files_with_extension(self.out_dir, "rs")?
			.filter(|f| is_type_externs_file(f, module))
			.collect::<Vec<_>>();
		type_extern_files.sort_unstable();
		for entry in type_extern_files {
			if entry.metadata().map(|meta| meta.len()).unwrap_or(0) > 0 {
				copy_indent(BufReader::new(File::open(&entry)?), sys_rs, "\t\t")?;
			}
			let _ = fs::remove_file(entry);
		}
		writeln!(sys_rs, "\t}}")?;
		writeln!(sys_rs, "}}")?;
		write_has_module(sys_rs, module)?;
		writeln!(sys_rs, "pub use {module}_sys::*;")?;
		writeln!(sys_rs)?;
		Ok(())
	}

	/// The #no_mangle function in the bindings cause duplicate export names when 2 different version of the crate are used
	/// (https://github.com/twistedfall/opencv-rust/issues/597). This function injects the version of the exported functions with
	/// a crate version suffix to avoid this conflict. On the C++ side it works with the help of the `OCVRS_FFI_EXPORT_SUFFIX`
	/// macro which is passed in `build_compiler()`.
	fn inject_ffi_exports(&self, hub_rs: &mut impl Write) -> Result<()> {
		writeln!(hub_rs, "\nmod ffi_exports {{")?;
		writeln!(hub_rs, "\tuse crate::mod_prelude_sys::*;")?;
		write!(hub_rs, "\t")?;
		writeln!(
			hub_rs,
			r#"#[no_mangle] unsafe extern "C" fn ocvrs_create_string{}(s: *const c_char) -> *mut String {{ crate::templ::ocvrs_create_string(s) }}"#,
			self.ffi_export_suffix
		)?;
		write!(hub_rs, "\t")?;
		writeln!(
			hub_rs,
			r#"#[no_mangle] unsafe extern "C" fn ocvrs_create_byte_string{}(v: *const u8, len: size_t) -> *mut Vec<u8> {{ crate::templ::ocvrs_create_byte_string(v, len) }}"#,
			self.ffi_export_suffix
		)?;
		writeln!(hub_rs, "}}")?;

		Ok(())
	}

	fn write_use_manual(&self, file: &mut BufWriter<File>, module: &str) -> Result<bool> {
		if self.manual_dir.join(format!("{module}.rs")).exists() {
			writeln!(file, "pub use crate::manual::{module}::*;")?;
			Ok(true)
		} else {
			Ok(false)
		}
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

fn copy_indent(mut read: impl BufRead, write: &mut impl Write, indent: &str) -> Result<()> {
	let mut line = Vec::with_capacity(100);
	while read.read_until(b'\n', &mut line)? != 0 {
		// there is more than just a newline in the buffer
		if line.len() > 1 {
			write.write_all(indent.as_bytes())?;
		}
		write.write_all(&line)?;
		line.clear();
	}
	Ok(())
}

fn write_has_module(write: &mut impl Write, module: &str) -> Result<()> {
	Ok(writeln!(write, "#[cfg(ocvrs_has_module_{module})]")?)
}
