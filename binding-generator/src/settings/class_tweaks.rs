use std::collections::HashMap;

use semver::Version;

use crate::SupportedModule;
use crate::version::OpenCVVersionExt;
use crate::writer::rust_native::type_ref::Lifetime;

pub type ClassTweaks = HashMap<&'static str, ClassTweak>;

#[derive(Clone, Copy, Debug)]
pub enum ClassTweak {
	Lifetime(Lifetime),
}

pub fn class_tweaks_factory(_: SupportedModule, opencv_version: &Version) -> ClassTweaks {
	let mut out = HashMap::from([("cv::MatSize", ClassTweak::Lifetime(Lifetime::Custom("mat")))]);
	if !opencv_version.is_opencv_5() {
		out.insert("cv::MatStep", ClassTweak::Lifetime(Lifetime::Custom("mat")));
	}
	out
}
