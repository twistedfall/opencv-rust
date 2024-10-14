use std::collections::HashMap;

pub type PropertyRename = HashMap<&'static str, &'static str>;

/// Rename property, format: (cpp_refname -> rust_custom_leafname)
pub fn property_rename_factory(module: &str) -> PropertyRename {
	match module {
		"core" => HashMap::from([
			("cv::Mat::size", "mat_size"),
			("cv::Mat::step", "mat_step"),
			("cv::UMat::size", "mat_size"),
			("cv::UMat::step", "mat_step"),
		]),
		_ => HashMap::new(),
	}
}
