use std::{
	fmt::Debug,
	fs::{self, File, OpenOptions},
	io::{ErrorKind, Write},
	path::{Path, PathBuf},
};

use dunce::canonicalize;
use maplit::hashmap;
use once_cell::sync::Lazy;

use element::RustNativeGeneratedElement;

use crate::{
	Class,
	comment,
	CompiledInterpolation,
	Const,
	Constness,
	DependentType,
	Element,
	Enum,
	Func,
	GeneratorVisitor,
	is_ephemeral_header,
	IteratorExt,
	opencv_module_from_path,
	settings,
	StrExt,
	type_ref::{FishStyle, NameStyle},
	Typedef,
};

mod abstract_ref_wrapper;
mod class;
mod constant;
mod element;
mod enumeration;
mod func;
mod return_type_wrapper;
mod smart_ptr;
mod typedef;
mod vector;

type Entries = Vec<(String, String)>;

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
	found_traits: Vec<String>,
	consts: Entries,
	enums: Entries,
	rust_funcs: Entries,
	rust_typedefs: Entries,
	rust_classes: Entries,
	export_funcs: Entries,
	export_classes: Entries,
	cpp_funcs: Entries,
	cpp_classes: Entries,
}

impl<'s> RustNativeBindingWriter<'s> {
	pub fn new(src_cpp_dir: &Path, out_dir: impl Into<PathBuf>, module: &'s str, opencv_version: &'s str, debug: bool) -> Self {
		let out_dir = out_dir.into();
		let debug_path = out_dir.join(format!("{}.log", module));
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
			rust_path: out_dir.join(format!("{}.rs", module)),
			exports_path: out_dir.join(format!("{}.externs.rs", module)),
			cpp_path: out_dir.join(format!("{}.cpp", module)),
			types_dir: out_dir,
			comment: String::new(),
			found_traits: vec![],
			consts: vec![],
			enums: vec![],
			rust_funcs: vec![],
			rust_typedefs: vec![],
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
				let mut f = OpenOptions::new().append(true).open(&self.debug_path).expect("Can't open debug file");
				writeln!(f, "{:#?}", obj).expect("Can't write debug info");
			}
		}
	}
}

impl GeneratorVisitor for RustNativeBindingWriter<'_> {
	fn wants_file(&mut self, path: &Path) -> bool {
		is_ephemeral_header(path) || opencv_module_from_path(path).map_or(false, |m| m == self.module)
	}

	fn visit_module_comment(&mut self, comment: String) {
		self.comment = comment;
	}

	fn visit_const(&mut self, cnst: Const) {
		self.emit_debug_log(&cnst);
		self.consts.push((cnst.rust_localname(FishStyle::No).into_owned(), cnst.gen_rust(self.opencv_version)));
	}

	fn visit_enum(&mut self, enm: Enum) {
		self.emit_debug_log(&enm);
		self.enums.push((enm.rust_localname(FishStyle::No).into_owned(), enm.gen_rust(self.opencv_version)));
	}

	fn visit_func(&mut self, func: Func) {
		self.emit_debug_log(&func);
		let name: String = func.identifier().into_owned();
		self.rust_funcs.push((name.clone(), func.gen_rust(self.opencv_version)));
		self.export_funcs.push((name.clone(), func.gen_rust_exports()));
		self.cpp_funcs.push((name, func.gen_cpp()));
	}

	fn visit_typedef(&mut self, typedef: Typedef) {
		self.emit_debug_log(&typedef);
		self.rust_typedefs.push((typedef.cpp_fullname().into_owned(), typedef.gen_rust(self.opencv_version)))
	}

	fn visit_class(&mut self, class: Class) {
		self.emit_debug_log(&class);
		if class.is_trait() {
			self.found_traits.push(format!("super::{}", class.rust_trait_name(NameStyle::Declaration, Constness::Const).into_owned()));
			self.found_traits.push(format!("super::{}", class.rust_trait_name(NameStyle::Declaration, Constness::Mut).into_owned()));
		}
		let name: String = class.cpp_fullname().into_owned();
		self.rust_classes.push((name.clone(), class.gen_rust(self.opencv_version)));
		self.export_classes.push((name.clone(), class.gen_rust_exports()));
		self.cpp_classes.push((name, class.gen_cpp()));
	}

	fn visit_dependent_type(&mut self, typ: DependentType) {
		let prio = typ.element_order();
		let safe_id = typ.element_safe_id();

		let path = self.types_dir.join(format!("{:03}-{}.type.rs", prio, safe_id));
		let file = OpenOptions::new().create_new(true).write(true).open(&path);
		match file {
			Ok(mut file) => {
				let gen = typ.gen_rust(self.opencv_version);
				if !gen.is_empty() {
					file.write_all(gen.as_bytes()).expect("Can't write to rust file");
				} else {
					drop(file);
					fs::remove_file(&path).expect("Can't remove empty file");
				}
			},
			Err(e) if e.kind() == ErrorKind::AlreadyExists => { /* expected, we need to exclusively create file */ },
			Err(e) if e.kind() == ErrorKind::PermissionDenied => { /* happens sporadically on Windows */ },
			Err(e) => {
				panic!("Error while creating file for rust dependent type: {}", e)
			},
		}

		let path = self.types_dir.join(format!("{:03}-{}.type.cpp", prio, safe_id));
		let file = OpenOptions::new().create_new(true).write(true).open(&path);
		match file {
			Ok(mut file) => {
				let gen = typ.gen_cpp();
				if !gen.is_empty() {
					file.write_all(gen.as_bytes()).expect("Can't write to cpp file");
				} else {
					drop(file);
					fs::remove_file(&path).expect("Can't remove empty file");
				}
			},
			Err(e) if e.kind() == ErrorKind::AlreadyExists => { /* expected, we need to exclusively create file */ },
			Err(e) if e.kind() == ErrorKind::PermissionDenied => { /* happens sporadically on Windows */ },
			Err(e) => {
				panic!("Error while creating file for cpp dependent type: {}", e)
			},
		}
	}

	fn visit_ephemeral_header(&mut self, contents: &str) {
		if self.debug {
			let mut file = File::create(self.types_dir.join(format!("ocvrs_ephemeral_{}.hpp", self.module))).expect("Can't create debug ephemeral file");
			file.write_all(contents.as_bytes()).expect("Can't write debug ephemeral file");
		}
	}
}

fn join<T: AsRef<str>>(v: &mut Vec<(String, T)>) -> String {
	v.sort_unstable_by(|(name_left, _), (name_right, _)| name_left.cmp(name_right));
	v.drain(..)
		.map(|(_, gen)| gen)
		.join("")
}

impl Drop for RustNativeBindingWriter<'_> {
	fn drop(&mut self) {
		static RUST: Lazy<CompiledInterpolation> = Lazy::new(||
			include_str!("tpl/module/rust.tpl.rs").compile_interpolation()
		);

		static RUST_PRELUDE: Lazy<CompiledInterpolation> = Lazy::new(||
			include_str!("tpl/module/prelude.tpl.rs").compile_interpolation()
		);

		static RUST_EXTERNS_TPL: Lazy<CompiledInterpolation> = Lazy::new(||
			include_str!("tpl/module/externs.tpl.rs").compile_interpolation()
		);

		static CPP: Lazy<CompiledInterpolation> = Lazy::new( ||
			include_str!("tpl/module/cpp.tpl.cpp").compile_interpolation()
		);

		let mut rust = join(&mut self.consts);
		rust += &join(&mut self.enums);
		rust += &join(&mut self.rust_typedefs);
		rust += &join(&mut self.rust_funcs);
		rust += &join(&mut self.rust_classes);
		let prelude = RUST_PRELUDE.interpolate(&hashmap! {
			"traits" => self.found_traits.join(", "),
		});
		File::create(&self.rust_path).expect("Can't create rust file")
			.write_all(RUST.interpolate(&hashmap! {
				"static_modules" => settings::STATIC_MODULES.iter().join(", "),
				"comment" => comment::render_doc_comment(&self.comment, "//!", self.opencv_version),
				"prelude" => prelude,
				"code" => rust,
			}).as_bytes()).expect("Can't write rust file");

		let mut cpp = join(&mut self.cpp_funcs);
		cpp += &join(&mut self.cpp_classes);
		let includes = if self.src_cpp_dir.join(format!("{}.hpp", self.module)).exists() {
			format!("#include \"{module}.hpp\"", module=self.module)
		} else {
			format!("#include \"ocvrs_common.hpp\"\n#include <opencv2/{module}.hpp>", module=self.module)
		};
		File::create(&self.cpp_path).expect("Can't create cpp file")
			.write_all(CPP.interpolate(&hashmap! {
				"module" => self.module,
				"includes" => includes.as_str(),
				"code" => cpp.as_str(),
			}).as_bytes()).expect("Can't write cpp file");

		let mut exports = join(&mut self.export_funcs);
		exports += &join(&mut self.export_classes);
		File::create(&self.exports_path).expect("Can't create rust exports file")
			.write_all(RUST_EXTERNS_TPL.interpolate(&hashmap! {
				"code" => exports,
			}).as_bytes()).expect("Can't write rust exports file");
	}
}
