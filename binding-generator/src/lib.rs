// todo apply_halide_scheduler accepts vector<Mat*> which gets generated as VectorOfMat, it's probably a bug
// todo change dyn InputArray to impl InputArray and friends
// todo drop unused entries from settings
// todo support converting pointer + size to slice of Mat and other similar objects
// todo add support for more operators
// todo add support for arrays in dnn::DictValue
// todo allow ergonomically combining of enum variants with |
// todo implement std::pair wrapper
// todo prevent generation of vector<std::string> in favor of vector<cv::String>>
// todo cv_utils_logging_internal_getGlobalLogTag() returns LogTag**, but Rust interprets it as LogTag*, check why it doesn't crash and fix if needed
// todo almost everything from the manual module must be connected to the binding generator, not the main crate
// fixme most constructors of Mat have invalid comment "download data from GpuMat"
// fixme MatConstIterator::m return Mat**, is it handled correctly?
// fixme docs.rs fails to build crates that have opencv as dependency

// copy-pasted form python generator (may be obsolete):
// fixme returning MatAllocator (trait) by reference is bad, check knearestneighbour
// fixme VectorOfMat::get allows to mutate?

use std::{
	borrow::Cow,
	fmt,
	fs::File,
	io::{BufRead, Read, Seek, SeekFrom},
};

use clang::Entity;
use dunce::canonicalize;

pub use class::Class;
pub use constant::Const;
use element::{DefaultElement, EntityElement};
pub use element::{Element, GeneratedElement};
use element::{is_opencv_path, main_module_from_path, main_opencv_module_from_path, module_from_path};
use entity::EntityExt;
pub use enumeration::Enum;
use field::{Field, FieldTypeHint};
pub use func::{Func, FunctionTypeHint};
use function::Function;
#[allow(unused)]
use generator::{dbg_clang_entity, dbg_clang_type};
pub use generator::{Generator, GeneratorVisitor};
use generator_env::{ExportConfig, GeneratorEnv};
pub use iterator_ext::IteratorExt;
#[allow(unused)]
use memoize::{memo, memo_map, Memoize, MemoizeMap};
use name_pool::NamePool;
use return_type_wrapper::{DefinitionLocation, ReturnTypeWrapper};
use smart_ptr::SmartPtr;
pub use string_ext::{CompiledInterpolation, StrExt, StringExt};
use type_ref::{DependentTypeMode, TypeRef, TypeRefTypeHint};
pub use typedef::Typedef;
use vector::Vector;
use walker::{EntityWalker, EntityWalkerVisitor};

mod class;
pub mod comment;
mod constant;
mod element;
mod entity;
mod enumeration;
mod field;
mod func;
mod function;
mod generator;
mod generator_env;
mod iterator_ext;
mod memoize;
mod name_pool;
mod return_type_wrapper;
pub mod settings;
mod smart_ptr;
mod string_ext;
#[cfg(test)]
mod test;
mod type_ref;
mod typedef;
mod vector;
mod walker;
pub mod writer;

fn get_definition_text(entity: Entity) -> String {
	if let Some(range) = entity.get_range() {
		let loc = range.get_start().get_spelling_location();
		let mut source = File::open(loc.file.expect("Can't get file").get_path()).expect("Can't open source file");
		let start = loc.offset;
		let end = range.get_end().get_spelling_location().offset;
		let mut def_bytes = vec![0; (end - start) as usize];
		source.seek(SeekFrom::Start(u64::from(start))).expect("Cannot seek");
		source.read_exact(&mut def_bytes).expect("Can't read definition");
		String::from_utf8(def_bytes).expect("Can't parse definition")
	} else {
		unreachable!("Can't get entity range: {:#?}", entity)
	}
}

fn get_debug<'tu>(e: &(impl EntityElement<'tu> + fmt::Display)) -> String {
	if false {
		let loc = e.entity()
			.get_location().expect("Can't get entity location")
			.get_file_location();

		format!(
			"// {} {}:{}",
			e,
			canonicalize(loc.file.expect("Can't get file for debug").get_path()).expect("Can't canonicalize path").display(),
			loc.line
		)
	} else {
		"".to_string()
	}
}

fn reserved_rename(val: Cow<str>) -> Cow<str> {
	if let Some(&v) = settings::RESERVED_RENAME.get(val.as_ref()) {
		v.into()
	} else {
		val
	}
}

#[inline(always)]
fn line_reader(mut b: impl BufRead, mut cb: impl FnMut(&str) -> bool) {
	let mut line = String::with_capacity(256);
	while let Ok(bytes_read) = b.read_line(&mut line) {
		if bytes_read == 0 {
			break
		}
		if !cb(&line) {
			break;
		}
		line.clear();
	}
}
