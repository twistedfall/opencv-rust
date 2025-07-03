#[expect(
	clippy::doc_lazy_continuation,
	clippy::doc_overindented_list_items,
	clippy::double_must_use,
	clippy::excessive_precision,
	clippy::let_and_return,
	clippy::let_unit_value,
	clippy::missing_safety_doc,
	clippy::should_implement_trait,
	clippy::tabs_in_doc_comments,
	clippy::too_many_arguments,
	clippy::unused_unit,
	clippy::approx_constant,
	dead_code,
	non_camel_case_types,
	non_snake_case,
	non_upper_case_globals,
	unused_imports,
	unused_parens
)]
#[cfg_attr(ocvrs_opencv_branch_4, expect(deprecated))]
pub mod hub {
	include!(concat!(env!("OUT_DIR"), "/opencv/hub.rs"));
}
mod cond_macros {
	include!(concat!(env!("OUT_DIR"), "/opencv/cond_macros.rs"));
}
