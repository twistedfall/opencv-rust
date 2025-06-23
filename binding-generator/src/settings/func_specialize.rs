use std::collections::HashMap;

use super::TypeRefFactory;
use crate::func::FuncMatcher;
use crate::type_ref::TypeRefDesc;
use crate::SupportedModule;

pub type FuncSpecialize = FuncMatcher<'static, Vec<FuncSpec>>;

pub type FuncSpec = (&'static str, HashMap<&'static str, TypeRefFactory>);

// todo: get rid of func_specialize in favor of just injecting manual functions
pub fn func_specialize_factory(module: SupportedModule) -> FuncSpecialize {
	match module {
		SupportedModule::Core => FuncMatcher::create(HashMap::from([(
			"cv::CommandLineParser::get",
			vec![
				(
					pred!(const, ["name", "space_delete"]),
					vec![
						("+_bool", HashMap::from([("T", TypeRefDesc::bool as _)])),
						("+_i32", HashMap::from([("T", TypeRefDesc::int as _)])),
						("+_f64", HashMap::from([("T", TypeRefDesc::double as _)])),
						("+_str", HashMap::from([("T", TypeRefDesc::cv_string as _)])),
						("+_u64", HashMap::from([("T", TypeRefDesc::uint64_t as _)])),
					],
				),
				(
					pred!(const, ["index", "space_delete"]),
					vec![
						("+_bool_idx", HashMap::from([("T", TypeRefDesc::bool as _)])),
						("+_i32_idx", HashMap::from([("T", TypeRefDesc::int as _)])),
						("+_f64_idx", HashMap::from([("T", TypeRefDesc::double as _)])),
						("+_str_idx", HashMap::from([("T", TypeRefDesc::cv_string as _)])),
						("+_u64_idx", HashMap::from([("T", TypeRefDesc::uint64_t as _)])),
					],
				),
			],
		)])),
		SupportedModule::Dnn => FuncMatcher::create(HashMap::from([
			(
				"cv::dnn::Dict::set",
				vec![(
					pred!(mut, ["key", "value"]),
					vec![
						("+_str", HashMap::from([("const T", TypeRefDesc::cv_string as _)])),
						("+", HashMap::from([("const T", TypeRefDesc::cv_dnn_dict_value as _)])),
						("+_f64", HashMap::from([("const T", TypeRefDesc::double as _)])),
						("+_i64", HashMap::from([("const T", TypeRefDesc::int64_t as _)])),
					],
				)],
			),
			(
				"cv::dnn::DictValue::get",
				vec![(
					pred!(const, ["idx"]),
					vec![
						("+_str", HashMap::from([("T", TypeRefDesc::cv_string as _)])),
						("+_f64", HashMap::from([("T", TypeRefDesc::double as _)])),
						("+_i32", HashMap::from([("T", TypeRefDesc::int as _)])),
						("+_i64", HashMap::from([("T", TypeRefDesc::int64_t as _)])),
					],
				)],
			),
		])),
		_ => FuncMatcher::empty(),
	}
}
