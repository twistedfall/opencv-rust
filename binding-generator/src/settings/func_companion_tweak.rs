use std::collections::HashMap;

use crate::func::FuncMatcher;
use crate::SupportedModule;

pub type FuncCompanionTweak = FuncMatcher<'static, CompanionTweak>;

#[derive(Debug, Clone, Copy)]
pub enum CompanionTweak {
	/// Don't generate the `*_def` function companion
	SkipDefault,
}

pub fn func_companion_tweak_factory(module: SupportedModule) -> FuncCompanionTweak {
	match module {
		SupportedModule::Dnn => dnn_factory(),
		SupportedModule::Text => text_factory(),
		_ => FuncCompanionTweak::empty(),
	}
}

fn dnn_factory() -> FuncCompanionTweak {
	FuncMatcher::create(HashMap::from([(
		"cv::dnn::Graph::append", // 2 functions with the same name and same non-default arguments lead to the name clash and ambiguous call in C++
		vec![
			(pred!(mut, ["layer", "outnames"]), CompanionTweak::SkipDefault),
			(pred!(mut, ["layer", "outname"]), CompanionTweak::SkipDefault),
		],
	)]))
}

fn text_factory() -> FuncCompanionTweak {
	FuncMatcher::create(HashMap::from([(
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
	)]))
}

impl CompanionTweak {
	pub fn skip_default(&self) -> bool {
		match self {
			CompanionTweak::SkipDefault => true,
		}
	}
}
