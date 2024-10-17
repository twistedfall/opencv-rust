use std::collections::HashMap;

use crate::func::FuncMatcher;

pub type FuncCompanionTweak = FuncMatcher<'static, CompanionTweak>;

#[derive(Debug, Clone, Copy)]
pub enum CompanionTweak {
	SkipDefault,
}

pub fn func_companion_tweak_factory(module: &str) -> FuncCompanionTweak {
	match module {
		"text" => FuncMatcher::create(HashMap::from([(
			"cv::text::OCRBeamSearchDecoder::create",
			vec![(
				pred!(
					mut,
					[
						"classifier",
						"vocabulary",
						"transition_probabilities_table",
						"emission_probabilities_table",
						"mode",
						"beam_size"
					],
				),
				CompanionTweak::SkipDefault, // with OpenCV 4.2 this leads to https://github.com/twistedfall/opencv-rust/issues/505
			)],
		)])),
		_ => FuncCompanionTweak::empty(),
	}
}

impl CompanionTweak {
	pub fn skip_default(&self) -> bool {
		match self {
			CompanionTweak::SkipDefault => true,
		}
	}
}
