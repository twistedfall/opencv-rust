use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::{File, OpenOptions};
use std::io::{ErrorKind, Write};
use std::path::{Path, PathBuf};
use std::{fs, iter};

use dunce::canonicalize;
use once_cell::sync::Lazy;

use class::ClassExt;
use element::{RustElement, RustNativeGeneratedElement};
pub use string_ext::RustStringExt;

use crate::field::Field;
use crate::name_pool::NamePool;
use crate::type_ref::{Constness, CppNameStyle, FishStyle, NameStyle};
use crate::writer::rust_native::func::FuncExt;
use crate::{
	opencv_module_from_path, settings, Class, CompiledInterpolation, Const, Element, Enum, Func, GeneratedType, GeneratorVisitor,
	IteratorExt, StrExt, Typedef,
};

mod abstract_ref_wrapper;
mod class;
mod comment;
mod constant;
pub mod element;
mod enumeration;
mod field;
mod func;
mod function;
pub mod renderer;
mod smart_ptr;
mod string_ext;
mod tuple;
pub mod type_ref;
mod typedef;
mod vector;

type Entries = Vec<(String, String)>;
type UniqueEntries = HashMap<String, String>;

/// Writer of the source files used when building OpenCV to run on a native platform (as opposed to for example WASM)
#[derive(Clone, Debug)]
pub struct RustNativeBindingWriter<'s> {
	debug: bool,
	src_cpp_dir: PathBuf,
	module: &'s str,
	opencv_version: &'s str,
	debug_path: PathBuf,
	rust_path: PathBuf,
	types_dir: PathBuf,
	exports_path: PathBuf,
	cpp_path: PathBuf,
	comment: String,
	prelude_traits: Vec<String>,
	consts: Entries,
	enums: Entries,
	rust_funcs: Entries,
	rust_typedefs: Option<UniqueEntries>,
	rust_classes: Entries,
	export_funcs: Entries,
	export_classes: Entries,
	cpp_funcs: Entries,
	cpp_classes: Entries,
}

impl<'s> RustNativeBindingWriter<'s> {
	pub fn new(src_cpp_dir: &Path, out_dir: impl Into<PathBuf>, module: &'s str, opencv_version: &'s str, debug: bool) -> Self {
		let out_dir = out_dir.into();
		let debug_path = out_dir.join(format!("{module}.log"));
		#[allow(clippy::collapsible_if)]
		if false {
			if debug {
				File::create(&debug_path).expect("Can't create debug log");
			}
		}
		Self {
			debug,
			src_cpp_dir: canonicalize(src_cpp_dir).expect("Can't canonicalize src_cpp_dir"),
			module,
			opencv_version,
			debug_path,
			rust_path: out_dir.join(format!("{module}.rs")),
			exports_path: out_dir.join(format!("{module}.externs.rs")),
			cpp_path: out_dir.join(format!("{module}.cpp")),
			types_dir: out_dir,
			comment: String::new(),
			prelude_traits: vec![],
			consts: vec![],
			enums: vec![],
			rust_funcs: vec![],
			rust_typedefs: Some(UniqueEntries::new()),
			rust_classes: vec![],
			export_funcs: vec![],
			export_classes: vec![],
			cpp_funcs: vec![],
			cpp_classes: vec![],
		}
	}

	fn emit_debug_log(&mut self, obj: &impl Debug) {
		#[allow(clippy::collapsible_if)]
		if false {
			if self.debug {
				let mut f = OpenOptions::new()
					.append(true)
					.open(&self.debug_path)
					.expect("Can't open debug file");
				writeln!(f, "{obj:#?}").expect("Can't write debug info");
			}
		}
	}
}

impl GeneratorVisitor for RustNativeBindingWriter<'_> {
	fn wants_file(&mut self, path: &Path) -> bool {
		opencv_module_from_path(path).map_or(false, |m| m == self.module)
	}

	fn visit_module_comment(&mut self, comment: String) {
		self.comment = comment;
	}

	fn visit_const(&mut self, cnst: Const) {
		self.emit_debug_log(&cnst);
		self.consts.push((
			cnst.rust_name(NameStyle::decl()).into_owned(),
			cnst.gen_rust(self.opencv_version),
		));
	}

	fn visit_enum(&mut self, enm: Enum) {
		self.emit_debug_log(&enm);
		self.enums.push((
			enm.rust_name(NameStyle::decl()).into_owned(),
			enm.gen_rust(self.opencv_version),
		));
	}

	fn visit_func(&mut self, func: Func) {
		self.emit_debug_log(&func);
		let companion_funcs = func.companion_functions();
		for func in iter::once(func).chain(companion_funcs) {
			let name = func.identifier();
			self.rust_funcs.push((name.clone(), func.gen_rust(self.opencv_version)));
			self.export_funcs.push((name.clone(), func.gen_rust_exports()));
			self.cpp_funcs.push((name, func.gen_cpp()));
		}
	}

	fn visit_typedef(&mut self, typedef: Typedef) {
		self.emit_debug_log(&typedef);
		let opencv_version = self.opencv_version;
		if let Some(typedefs) = self.rust_typedefs.as_mut() {
			let cpp_refname = typedef.cpp_name(CppNameStyle::Reference);
			if !typedefs.contains_key(cpp_refname.as_ref()) {
				typedefs.insert(cpp_refname.into_owned(), typedef.gen_rust(opencv_version));
			}
		}
	}

	fn visit_class(&mut self, class: Class) {
		self.emit_debug_log(&class);
		if class.is_trait() {
			self.prelude_traits.push(format!(
				"super::{}",
				class.rust_trait_name(NameStyle::decl(), Constness::Const).into_owned()
			));
			self.prelude_traits.push(format!(
				"super::{}",
				class.rust_trait_name(NameStyle::decl(), Constness::Mut).into_owned()
			));
		}
		let name = class.cpp_name(CppNameStyle::Reference).into_owned();
		self.rust_classes.push((name.clone(), class.gen_rust(self.opencv_version)));
		self.export_classes.push((name.clone(), class.gen_rust_exports()));
		self.cpp_classes.push((name, class.gen_cpp()));
	}

	fn visit_generated_type(&mut self, typ: GeneratedType) {
		let typ = typ.as_ref();
		let prio = typ.element_order();
		let safe_id = typ.element_safe_id();

		fn write_generated_type(types_dir: &Path, typ: &str, prio: u8, safe_id: &str, generator: impl FnOnce() -> String) {
			let suffix = format!(".type.{typ}");
			let mut file_name = format!("{prio:03}-{safe_id}");
			ensure_filename_length(&mut file_name, suffix.len());
			file_name.push_str(&suffix);
			let path = types_dir.join(file_name);
			let file = OpenOptions::new().create_new(true).write(true).open(&path);
			match file {
				Ok(mut file) => {
					let gen = generator();
					if !gen.is_empty() {
						file
							.write_all(gen.as_bytes())
							.unwrap_or_else(|e| panic!("Can't write to {typ} file: {e}"));
					} else {
						drop(file);
						fs::remove_file(&path).expect("Can't remove empty file");
					}
				}
				Err(e) if e.kind() == ErrorKind::AlreadyExists => { /* expected, we need to exclusively create file */ }
				Err(e) if e.kind() == ErrorKind::PermissionDenied => { /* happens sporadically on Windows */ }
				Err(e) => {
					panic!("Error while creating file for {typ} generated type: {e}")
				}
			}
		}

		write_generated_type(&self.types_dir, "rs", prio, &safe_id, || typ.gen_rust(self.opencv_version));
		write_generated_type(&self.types_dir, "externs.rs", prio, &safe_id, || typ.gen_rust_exports());
		write_generated_type(&self.types_dir, "cpp", prio, &safe_id, || typ.gen_cpp());
	}
}

fn join<T: AsRef<str>>(v: &mut Vec<(String, T)>) -> String {
	v.sort_unstable_by(|(name_left, _), (name_right, _)| name_left.cmp(name_right));
	v.drain(..).map(|(_, gen)| gen).join("")
}

impl Drop for RustNativeBindingWriter<'_> {
	fn drop(&mut self) {
		static RUST: Lazy<CompiledInterpolation> = Lazy::new(|| include_str!("tpl/module/rust.tpl.rs").compile_interpolation());

		static RUST_PRELUDE: Lazy<CompiledInterpolation> =
			Lazy::new(|| include_str!("tpl/module/prelude.tpl.rs").compile_interpolation());

		static RUST_EXTERNS_TPL: Lazy<CompiledInterpolation> =
			Lazy::new(|| include_str!("tpl/module/externs.tpl.rs").compile_interpolation());

		static CPP: Lazy<CompiledInterpolation> = Lazy::new(|| include_str!("tpl/module/cpp.tpl.cpp").compile_interpolation());

		let mut rust = join(&mut self.consts);
		rust += &join(&mut self.enums);
		let mut typedefs = self
			.rust_typedefs
			.take()
			.map_or(vec![], |typedefs| typedefs.into_iter().collect());
		rust += &join(&mut typedefs);
		rust += &join(&mut self.rust_funcs);
		rust += &join(&mut self.rust_classes);
		let prelude = RUST_PRELUDE.interpolate(&HashMap::from([("traits", self.prelude_traits.join(", "))]));
		let comment = comment::render_doc_comment(&self.comment, "//!", self.opencv_version);
		File::create(&self.rust_path)
			.expect("Can't create rust file")
			.write_all(
				RUST
					.interpolate(&HashMap::from([
						("static_modules", settings::STATIC_MODULES.iter().join(", ")),
						("comment", comment),
						("prelude", prelude),
						("code", rust),
					]))
					.as_bytes(),
			)
			.expect("Can't write rust file");

		let mut cpp = join(&mut self.cpp_funcs);
		cpp += &join(&mut self.cpp_classes);
		let includes = if self.src_cpp_dir.join(format!("{}.hpp", self.module)).exists() {
			format!("#include \"{module}.hpp\"", module = self.module)
		} else {
			format!(
				"#include \"ocvrs_common.hpp\"\n#include <opencv2/{module}.hpp>",
				module = self.module
			)
		};
		File::create(&self.cpp_path)
			.expect("Can't create cpp file")
			.write_all(
				CPP.interpolate(&HashMap::from([
					("module", self.module),
					("includes", includes.as_str()),
					("code", cpp.as_str()),
				]))
				.as_bytes(),
			)
			.expect("Can't write cpp file");

		let mut exports = join(&mut self.export_funcs);
		exports += &join(&mut self.export_classes);
		File::create(&self.exports_path)
			.expect("Can't create rust exports file")
			.write_all(RUST_EXTERNS_TPL.interpolate(&HashMap::from([("code", exports)])).as_bytes())
			.expect("Can't write rust exports file");
	}
}

fn ensure_filename_length(file_name: &mut String, reserve: usize) {
	const MAX_FILENAME_LEN: usize = 255;

	let max_length = MAX_FILENAME_LEN - reserve;

	if file_name.len() > max_length {
		*file_name = file_name[..max_length].to_string();
	}
}

fn rust_disambiguate_names<'tu, 'ge>(
	args: impl IntoIterator<Item = Field<'tu, 'ge>>,
) -> impl Iterator<Item = (String, Field<'tu, 'ge>)>
where
	'tu: 'ge,
{
	let args = args.into_iter();
	let size_hint = args.size_hint();
	NamePool::with_capacity(size_hint.1.unwrap_or(size_hint.0)).into_disambiguator(args, |f| f.rust_leafname(FishStyle::No))
}

pub fn disambiguate_single_name(name: &str) -> impl Iterator<Item = String> + '_ {
	let mut i = 0;
	iter::from_fn(move || {
		let out = format!("{}{}", name, disambiguate_num(i));
		i += 1;
		Some(out)
	})
}

fn disambiguate_num(counter: usize) -> String {
	match counter {
		0 => "".to_string(),
		n => format!("_{n}"),
	}
}
