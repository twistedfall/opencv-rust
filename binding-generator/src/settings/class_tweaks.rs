use std::collections::HashMap;

use crate::writer::rust_native::type_ref::Lifetime;
use crate::SupportedModule;

pub type ClassTweaks = HashMap<&'static str, ClassTweak>;

#[derive(Clone, Copy, Debug)]
pub enum ClassTweak {
	Lifetime(Lifetime),
}

pub fn class_tweaks_factory(module: SupportedModule) -> ClassTweaks {
	match module {
		SupportedModule::Core => HashMap::from([("cv::MatSize", ClassTweak::Lifetime(Lifetime::Custom("mat")))]),
		_ => HashMap::default(),
	}
}
