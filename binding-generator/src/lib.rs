// todo change dyn InputArray to impl InputArray and friends
// todo support converting pointer + size to slice of Mat and other similar objects
// todo add support for more operators
// todo add support for arrays in dnn::DictValue
// todo allow ergonomically combining of enum variants with |
// todo implement std::pair wrapper
// todo cv_utils_logging_internal_getGlobalLogTag() returns LogTag**, but Rust interprets it as LogTag*, check why it doesn't crash and fix if needed
// todo almost everything from the manual module must be connected to the binding generator, not the main crate
// todo check that FN_FaceDetector works at all (receiving InputArray, passing as callback)
// fixme vector<Mat*> get's interpreted as Vector<Mat> which should be wrong (e.g. Layer::forward and Layer::apply_halide_scheduler)
// fixme MatConstIterator::m return Mat**, is it handled correctly?
// fixme VectorOfMat::get allows mutation
// fixme TextDetectionModel_EAST (and all others inheriting from Model) seems to have no fields, but it fails when generated as simple, probably makes sense to have a special case for that

// copy-pasted form python generator (may be obsolete):
// fixme returning MatAllocator (trait) by reference is bad, check knearestneighbour

#![allow(clippy::nonminimal_bool)] // pattern `!type_ref.as_vector().is_some()` used for more clarity

use std::{
	borrow::Cow,
	env,
	fmt,
	fs::File,
	io::{BufRead, Read, Seek, SeekFrom},
};

use clang::Entity;
use dunce::canonicalize;
use once_cell::sync::Lazy;

pub use abstract_ref_wrapper::AbstractRefWrapper;
pub use class::Class;
pub use constant::Const;
pub use element::{DefaultElement, Element, EntityElement, is_opencv_path, opencv_module_from_path};
pub use entity::EntityExt;
pub use enumeration::Enum;
use field::{Field, FieldTypeHint};
pub use func::{Func, FuncId, FunctionTypeHint};
use function::Function;
#[allow(unused)]
use generator::{dbg_clang_entity, dbg_clang_type};
pub use generator::{DependentType, Generator, GeneratorVisitor, is_ephemeral_header};
pub use generator_env::{ExportConfig, GeneratorEnv};
pub use iterator_ext::IteratorExt;
#[allow(unused)]
use memoize::{memo, memo_map, Memoize, MemoizeMap};
use name_pool::NamePool;
use return_type_wrapper::{DefinitionLocation, ReturnTypeWrapper};
use smart_ptr::SmartPtr;
pub use string_ext::{CompiledInterpolation, StrExt, StringExt};
use type_ref::{Constness, ConstnessOverride, DependentTypeMode, TypeRef};
pub use typedef::Typedef;
use vector::Vector;
pub use walker::{EntityWalker, EntityWalkerVisitor};

mod abstract_ref_wrapper;
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

static EMIT_DEBUG: Lazy<bool> = Lazy::new(|| env::var("OPENCV_BINDING_GENERATOR_EMIT_DEBUG").map(|v| v == "1").unwrap_or(false));

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
	if *EMIT_DEBUG {
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
