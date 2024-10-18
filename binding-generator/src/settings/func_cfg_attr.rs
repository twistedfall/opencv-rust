use std::collections::HashMap;

use crate::func::FuncMatcher;

// pub type FuncCfgAttr = HashMap<&'static str, (&'static str, &'static str)>;
pub type FuncCfgAttr = FuncMatcher<'static, (&'static str, &'static str)>;

/// identifier => (rust_attr, cpp_attr)
pub fn func_cfg_attr_factory(module: &str) -> FuncCfgAttr {
	match module {
		"imgproc" => imgproc_factory(),
		"tracking" => tracking_factory(),
		_ => FuncCfgAttr::empty(),
	}
}

fn imgproc_factory() -> FuncCfgAttr {
	FuncCfgAttr::create(HashMap::from([(
		"cv::getRotationMatrix2D_",
		vec![(pred!(mut, ["center", "angle", "scale"]), NOT_ON_WINDOWS)],
	)]))
}

fn tracking_factory() -> FuncCfgAttr {
	FuncCfgAttr::create(HashMap::from([
		(
			"cv::TrackerStateEstimatorAdaBoosting::TrackerAdaBoostingTargetState::TrackerAdaBoostingTargetState",
			vec![(
				pred!(mut, ["position", "width", "height", "foreground", "responses"]),
				NOT_ON_WINDOWS, // 3.4
			)],
		),
		(
			"cv::TrackerStateEstimatorAdaBoosting::TrackerAdaBoostingTargetState::getTargetResponses",
			vec![(
				pred!(const, []),
				NOT_ON_WINDOWS, // 3.4
			)],
		),
		(
			"cv::TrackerStateEstimatorAdaBoosting::TrackerAdaBoostingTargetState::isTargetFg",
			vec![(
				pred!(const, []),
				NOT_ON_WINDOWS, // 3.4
			)],
		),
		(
			"cv::TrackerStateEstimatorAdaBoosting::TrackerAdaBoostingTargetState::setTargetFg",
			vec![(
				pred!(mut, ["foreground"]),
				NOT_ON_WINDOWS, // 3.4
			)],
		),
		(
			"cv::TrackerStateEstimatorAdaBoosting::TrackerAdaBoostingTargetState::setTargetResponses",
			vec![(
				pred!(mut, ["responses"]),
				NOT_ON_WINDOWS, // 3.4
			)],
		),
		(
			"cv::TrackerStateEstimatorMILBoosting::TrackerMILTargetState::TrackerMILTargetState",
			vec![(
				pred!(mut, ["position", "width", "height", "foreground", "features"]),
				NOT_ON_WINDOWS, // 3.4
			)],
		),
		(
			"cv::TrackerStateEstimatorMILBoosting::TrackerMILTargetState::getFeatures",
			vec![(
				pred!(const, []),
				NOT_ON_WINDOWS, // 3.4
			)],
		),
		(
			"cv::TrackerStateEstimatorMILBoosting::TrackerMILTargetState::isTargetFg",
			vec![(
				pred!(const, []),
				NOT_ON_WINDOWS, // 3.4
			)],
		),
		(
			"cv::TrackerStateEstimatorMILBoosting::TrackerMILTargetState::setFeatures",
			vec![(
				pred!(mut, ["features"]),
				NOT_ON_WINDOWS, // 3.4
			)],
		),
		(
			"cv::TrackerStateEstimatorMILBoosting::TrackerMILTargetState::setTargetFg",
			vec![(
				pred!(mut, ["foreground"]),
				NOT_ON_WINDOWS, // 3.4
			)],
		),
	]))
}

const NOT_ON_WINDOWS: (&str, &str) = ("not(target_os = \"windows\")", "!defined(OCVRS_TARGET_OS_WINDOWS)");
