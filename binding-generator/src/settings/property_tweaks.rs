use std::collections::HashMap;

pub type PropertyTweaks = HashMap<&'static str, PropertyTweak<'static>>;

#[derive(Debug)]
pub struct PropertyTweak<'l> {
	pub rename: Option<&'l str>,
	pub read_write: Option<PropertyReadWrite>,
}

/// Rename property, format: (cpp_refname -> rust_custom_leafname)
pub fn property_tweaks_factory(module: &str) -> PropertyTweaks {
	match module {
		"core" => HashMap::from([
			(
				"cv::Mat::size",
				PropertyTweak {
					rename: Some("mat_size"),
					..PropertyTweak::none()
				},
			),
			(
				"cv::Mat::step",
				PropertyTweak {
					rename: Some("mat_step"),
					read_write: Some(PropertyReadWrite::ReadOnly), // MatStep type prevents assignment
				},
			),
			(
				"cv::UMat::size",
				PropertyTweak {
					rename: Some("mat_size"),
					..PropertyTweak::none()
				},
			),
			(
				"cv::UMat::step",
				PropertyTweak {
					rename: Some("mat_step"),
					read_write: Some(PropertyReadWrite::ReadOnly), // MatStep type prevents assignment
				},
			),
		]),
		_ => HashMap::new(),
	}
}

impl PropertyTweak<'_> {
	pub fn none() -> PropertyTweak<'static> {
		PropertyTweak {
			rename: None,
			read_write: None,
		}
	}
}

#[derive(Clone, Copy, Debug)]
pub enum PropertyReadWrite {
	ReadOnly,
	ReadWrite,
}

impl PropertyReadWrite {
	pub fn is_read(self) -> bool {
		match self {
			PropertyReadWrite::ReadOnly | PropertyReadWrite::ReadWrite => true,
		}
	}

	pub fn is_write(self) -> bool {
		match self {
			PropertyReadWrite::ReadWrite => true,
			PropertyReadWrite::ReadOnly => false,
		}
	}
}
