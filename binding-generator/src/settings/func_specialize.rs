use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::type_ref::{TypeRef, TypeRefDesc};
use crate::FuncId;

pub type TypeRefFactory = fn() -> TypeRef<'static, 'static>;

// todo: get rid of FUNC_SPECIALIZE and property reader/writer generation in favor of just injecting manual functions
pub static FUNC_SPECIALIZE: Lazy<HashMap<FuncId, Vec<HashMap<&str, TypeRefFactory>>>> = Lazy::new(|| {
	HashMap::from([
		(
			FuncId::new("cv::dnn::Dict::set", ["key", "value"]),
			vec![
				HashMap::from([("const T", TypeRefDesc::cv_string as _)]),
				HashMap::from([("const T", TypeRefDesc::cv_dnn_dict_value as _)]),
				HashMap::from([("const T", TypeRefDesc::double as _)]),
				HashMap::from([("const T", TypeRefDesc::int64_t as _)]),
			],
		),
		(
			FuncId::new("cv::dnn::DictValue::get", ["idx"]),
			vec![
				HashMap::from([("T", TypeRefDesc::cv_string as _)]),
				HashMap::from([("T", TypeRefDesc::double as _)]),
				HashMap::from([("T", TypeRefDesc::int as _)]),
				HashMap::from([("T", TypeRefDesc::int64_t as _)]),
			],
		),
		(
			FuncId::new("cv::CommandLineParser::get", ["name", "space_delete"]),
			vec![
				HashMap::from([("T", TypeRefDesc::bool as _)]),
				HashMap::from([("T", TypeRefDesc::int as _)]),
				HashMap::from([("T", TypeRefDesc::double as _)]),
				HashMap::from([("T", TypeRefDesc::cv_string as _)]),
				HashMap::from([("T", TypeRefDesc::uint64_t as _)]),
			],
		),
		(
			FuncId::new("cv::CommandLineParser::get", ["index", "space_delete"]),
			vec![
				HashMap::from([("T", TypeRefDesc::bool as _)]),
				HashMap::from([("T", TypeRefDesc::int as _)]),
				HashMap::from([("T", TypeRefDesc::double as _)]),
				HashMap::from([("T", TypeRefDesc::cv_string as _)]),
				HashMap::from([("T", TypeRefDesc::uint64_t as _)]),
			],
		),
	])
});
