use std::collections::HashMap;

use crate::func::FuncMatcher;
use crate::SupportedModule;

pub const CFG_ATTR_NOT_ON_WINDOWS: (&str, &str) = (r#"not(target_os = "windows")"#, r#"(!OCVRS_TARGET_OS_WINDOWS)"#);
pub const CFG_ATTR_ONLY_OPENCV_5: (&str, &str) = ("ocvrs_opencv_branch_5", "(CV_VERSION_MAJOR == 5)");

pub type FuncCfgAttr = FuncMatcher<'static, (&'static str, &'static str)>;

/// identifier => (rust_attr, cpp_attr)
pub fn func_cfg_attr_factory(module: SupportedModule) -> FuncCfgAttr {
	match module {
		SupportedModule::ImgProc => imgproc_factory(),
		SupportedModule::Tracking => tracking_factory(),
		_ => FuncCfgAttr::empty(),
	}
}

fn imgproc_factory() -> FuncCfgAttr {
	FuncCfgAttr::create(HashMap::from([(
		"cv::getRotationMatrix2D_",
		vec![(pred!(mut, ["center", "angle", "scale"]), CFG_ATTR_NOT_ON_WINDOWS)],
	)]))
}

fn tracking_factory() -> FuncCfgAttr {
	FuncCfgAttr::create(HashMap::from([
		(
			"cv::TrackerStateEstimatorAdaBoosting::TrackerAdaBoostingTargetState::TrackerAdaBoostingTargetState",
			vec![(
				pred!(mut, ["position", "width", "height", "foreground", "responses"]),
				CFG_ATTR_NOT_ON_WINDOWS, // 3.4
			)],
		),
		(
			"cv::TrackerStateEstimatorAdaBoosting::TrackerAdaBoostingTargetState::getTargetResponses",
			vec![(
				pred!(const, []),
				CFG_ATTR_NOT_ON_WINDOWS, // 3.4
			)],
		),
		(
			"cv::TrackerStateEstimatorAdaBoosting::TrackerAdaBoostingTargetState::isTargetFg",
			vec![(
				pred!(const, []),
				CFG_ATTR_NOT_ON_WINDOWS, // 3.4
			)],
		),
		(
			"cv::TrackerStateEstimatorAdaBoosting::TrackerAdaBoostingTargetState::setTargetFg",
			vec![(
				pred!(mut, ["foreground"]),
				CFG_ATTR_NOT_ON_WINDOWS, // 3.4
			)],
		),
		(
			"cv::TrackerStateEstimatorAdaBoosting::TrackerAdaBoostingTargetState::setTargetResponses",
			vec![(
				pred!(mut, ["responses"]),
				CFG_ATTR_NOT_ON_WINDOWS, // 3.4
			)],
		),
		(
			"cv::TrackerStateEstimatorMILBoosting::TrackerMILTargetState::TrackerMILTargetState",
			vec![(
				pred!(mut, ["position", "width", "height", "foreground", "features"]),
				CFG_ATTR_NOT_ON_WINDOWS, // 3.4
			)],
		),
		(
			"cv::TrackerStateEstimatorMILBoosting::TrackerMILTargetState::getFeatures",
			vec![(
				pred!(const, []),
				CFG_ATTR_NOT_ON_WINDOWS, // 3.4
			)],
		),
		(
			"cv::TrackerStateEstimatorMILBoosting::TrackerMILTargetState::isTargetFg",
			vec![(
				pred!(const, []),
				CFG_ATTR_NOT_ON_WINDOWS, // 3.4
			)],
		),
		(
			"cv::TrackerStateEstimatorMILBoosting::TrackerMILTargetState::setFeatures",
			vec![(
				pred!(mut, ["features"]),
				CFG_ATTR_NOT_ON_WINDOWS, // 3.4
			)],
		),
		(
			"cv::TrackerStateEstimatorMILBoosting::TrackerMILTargetState::setTargetFg",
			vec![(
				pred!(mut, ["foreground"]),
				CFG_ATTR_NOT_ON_WINDOWS, // 3.4
			)],
		),
	]))
}
