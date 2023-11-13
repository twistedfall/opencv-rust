use std::collections::HashMap;

use once_cell::sync::Lazy;

/// identifier => (rust_attr, cpp_attr)
pub static FUNC_CFG_ATTR: Lazy<HashMap<&str, (&str, &str)>> = Lazy::new(|| {
	HashMap::from([
		// ### imgproc ###
		("cv_getRotationMatrix2D__Point2f_double_double", ("not(target_os = \"windows\")", "!defined(OCVRS_TARGET_OS_WINDOWS)")),

		// ### tracking ###
		("cv_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState_TrackerAdaBoostingTargetState_const_Point2fR_int_int_bool_const_MatR", ("not(target_os = \"windows\")", "!defined(OCVRS_TARGET_OS_WINDOWS)")),
		("cv_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState_getTargetResponses_const", ("not(target_os = \"windows\")", "!defined(OCVRS_TARGET_OS_WINDOWS)")),
		("cv_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState_isTargetFg_const", ("not(target_os = \"windows\")", "!defined(OCVRS_TARGET_OS_WINDOWS)")),
		("cv_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState_setTargetFg_bool", ("not(target_os = \"windows\")", "!defined(OCVRS_TARGET_OS_WINDOWS)")),
		("cv_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState_setTargetResponses_const_MatR", ("not(target_os = \"windows\")", "!defined(OCVRS_TARGET_OS_WINDOWS)")),
		("cv_TrackerStateEstimatorMILBoosting_TrackerMILTargetState_TrackerMILTargetState_const_Point2fR_int_int_bool_const_MatR", ("not(target_os = \"windows\")", "!defined(OCVRS_TARGET_OS_WINDOWS)")),
		("cv_TrackerStateEstimatorMILBoosting_TrackerMILTargetState_getFeatures_const", ("not(target_os = \"windows\")", "!defined(OCVRS_TARGET_OS_WINDOWS)")),
		("cv_TrackerStateEstimatorMILBoosting_TrackerMILTargetState_isTargetFg_const", ("not(target_os = \"windows\")", "!defined(OCVRS_TARGET_OS_WINDOWS)")),
		("cv_TrackerStateEstimatorMILBoosting_TrackerMILTargetState_setFeatures_const_MatR", ("not(target_os = \"windows\")", "!defined(OCVRS_TARGET_OS_WINDOWS)")),
		("cv_TrackerStateEstimatorMILBoosting_TrackerMILTargetState_setTargetFg_bool", ("not(target_os = \"windows\")", "!defined(OCVRS_TARGET_OS_WINDOWS)")),
	])
});
