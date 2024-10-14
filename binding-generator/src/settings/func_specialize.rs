use std::collections::HashMap;

use super::TypeRefFactory;
use crate::type_ref::TypeRefDesc;
use crate::FuncId;

pub type FuncSpecialize = HashMap<FuncId<'static>, Vec<FuncSpec>>;

pub type FuncSpec = (&'static str, HashMap<&'static str, TypeRefFactory>);

// todo: get rid of func_specialize in favor of just injecting manual functions
pub fn func_specialize_factory(module: &str) -> FuncSpecialize {
	match module {
		"core" => HashMap::from([
			(
				FuncId::new_const("cv::CommandLineParser::get", ["name", "space_delete"]),
				vec![
					("+_bool", HashMap::from([("T", TypeRefDesc::bool as _)])),
					("+_i32", HashMap::from([("T", TypeRefDesc::int as _)])),
					("+_f64", HashMap::from([("T", TypeRefDesc::double as _)])),
					("+_str", HashMap::from([("T", TypeRefDesc::cv_string as _)])),
					("+_u64", HashMap::from([("T", TypeRefDesc::uint64_t as _)])),
				],
			),
			(
				FuncId::new_const("cv::CommandLineParser::get", ["index", "space_delete"]),
				vec![
					("+_bool_idx", HashMap::from([("T", TypeRefDesc::bool as _)])),
					("+_i32_idx", HashMap::from([("T", TypeRefDesc::int as _)])),
					("+_f64_idx", HashMap::from([("T", TypeRefDesc::double as _)])),
					("+_str_idx", HashMap::from([("T", TypeRefDesc::cv_string as _)])),
					("+_u64_idx", HashMap::from([("T", TypeRefDesc::uint64_t as _)])),
				],
			),
		]),
		"dnn" => HashMap::from([
			(
				FuncId::new_mut("cv::dnn::Dict::set", ["key", "value"]),
				vec![
					("+_str", HashMap::from([("const T", TypeRefDesc::cv_string as _)])),
					("+", HashMap::from([("const T", TypeRefDesc::cv_dnn_dict_value as _)])),
					("+_f64", HashMap::from([("const T", TypeRefDesc::double as _)])),
					("+_i64", HashMap::from([("const T", TypeRefDesc::int64_t as _)])),
				],
			),
			(
				FuncId::new_const("cv::dnn::DictValue::get", ["idx"]),
				vec![
					("+_str", HashMap::from([("T", TypeRefDesc::cv_string as _)])),
					("+_f64", HashMap::from([("T", TypeRefDesc::double as _)])),
					("+_i32", HashMap::from([("T", TypeRefDesc::int as _)])),
					("+_i64", HashMap::from([("T", TypeRefDesc::int64_t as _)])),
				],
			),
		]),
		_ => HashMap::new(),
	}
}
