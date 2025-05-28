// MSRV: switch to #[expect] when MSRV is 1.81
#[allow(
	clippy::doc_lazy_continuation,
	clippy::double_must_use,
	clippy::excessive_precision,
	clippy::let_and_return,
	clippy::let_unit_value,
	clippy::missing_safety_doc,
	clippy::should_implement_trait,
	clippy::tabs_in_doc_comments,
	clippy::too_many_arguments,
	clippy::unnecessary_operation,
	clippy::unused_unit,
	dead_code,
	deprecated,
	non_camel_case_types,
	non_snake_case,
	non_upper_case_globals,
	overflowing_literals,
	unused_imports,
	unused_parens
)]
pub mod hub {
	include!(concat!(env!("OUT_DIR"), "/opencv/hub.rs"));
}
mod cond_macros {
	include!(concat!(env!("OUT_DIR"), "/opencv/cond_macros.rs"));
}
