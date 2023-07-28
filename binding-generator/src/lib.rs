// todo change dyn InputArray to impl InputArray and friends
// todo support converting pointer + size to slice of Mat and other similar objects
// todo add support for arrays in dnn::DictValue
// todo allow ergonomically combining of enum variants with |
// todo cv_utils_logging_internal_getGlobalLogTag() returns LogTag**, but Rust interprets it as LogTag*, check why it doesn't crash and fix if needed
// todo almost everything from the manual module must be connected to the binding generator, not the main crate
// todo check that FN_FaceDetector works at all (receiving InputArray, passing as callback)
// fixme vector<Mat*> get's interpreted as Vector<Mat> which should be wrong (e.g. Layer::forward and Layer::apply_halide_scheduler)
// fixme MatConstIterator::m return Mat**, is it handled correctly?
// fixme VectorOfMat::get allows mutation

// copy-pasted form python generator (may be obsolete):
// fixme returning MatAllocator (trait) by reference is bad, check knearestneighbour

#![allow(clippy::nonminimal_bool)] // pattern `!type_ref.as_vector().is_some()` used for more clarity

use std::borrow::Cow;
use std::fs::File;
use std::io::{BufRead, Read, Seek, SeekFrom};

use clang::Entity;

pub use abstract_ref_wrapper::AbstractRefWrapper;
pub use class::Class;
pub use constant::Const;
pub use element::{is_opencv_path, opencv_module_from_path, DefaultElement, Element, EntityElement};
#[allow(unused)]
use entity::dbg_clang_entity;
pub use entity::{EntityExt, WalkAction, WalkResult};
pub use enumeration::Enum;
use field::{Field, FieldTypeHint};
pub use func::{Func, FuncId, FuncTypeHint};
use function::Function;
pub use generator::{GeneratedType, Generator, GeneratorVisitor};
pub use generator_env::{ClassSimplicity, ExportConfig, GeneratorEnv};
pub use iterator_ext::IteratorExt;
use memoize::{MemoizeMap, MemoizeMapExt};
use name_pool::NamePool;
use smart_ptr::SmartPtr;
pub use string_ext::{CompiledInterpolation, StrExt, StringExt};
use tuple::Tuple;
#[allow(unused)]
use type_ref::dbg_clang_type;
use type_ref::TypeRef;
pub use type_ref::{CppNameStyle, NameStyle};
pub use typedef::Typedef;
use vector::Vector;
pub use walker::{EntityWalkerExt, EntityWalkerVisitor};

use crate::debug::NameDebug;

mod abstract_ref_wrapper;
mod class;
pub mod comment;
mod constant;
mod debug;
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
mod renderer;
pub mod settings;
mod smart_ptr;
mod string_ext;
#[cfg(test)]
mod test;
mod tuple;
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

fn reserved_rename(val: Cow<str>) -> Cow<str> {
	if let Some(&v) = settings::RESERVED_RENAME.get(val.as_ref()) {
		v.into()
	} else {
		val
	}
}

pub enum LineReaderAction {
	Continue,
	Break,
}

#[inline(always)]
pub fn line_reader(mut b: impl BufRead, mut cb: impl FnMut(&str) -> LineReaderAction) {
	let mut line = String::with_capacity(256);
	while let Ok(bytes_read) = b.read_line(&mut line) {
		if bytes_read == 0 {
			break;
		}
		if matches!(cb(&line), LineReaderAction::Break) {
			break;
		}
		line.clear();
	}
}
