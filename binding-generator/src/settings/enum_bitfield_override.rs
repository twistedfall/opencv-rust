use std::collections::HashMap;

use EnumBitfield::NotBitfield;

use crate::enumeration::EnumBitfield;
use crate::SupportedModule;

pub type EnumBitfieldOverride = HashMap<&'static str, EnumBitfield>;

/// cpp_name(Reference)
pub fn enum_bitfield_override_factory(module: SupportedModule) -> EnumBitfieldOverride {
	match module {
		SupportedModule::Core => HashMap::from([("cv::RotateFlags", NotBitfield)]),
		SupportedModule::ImgCodecs => HashMap::from([("cv::ImwriteEXRTypeFlags", NotBitfield)]),
		_ => HashMap::new(),
	}
}
