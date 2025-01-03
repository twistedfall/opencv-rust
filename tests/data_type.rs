use opencv::core;
use opencv::core::{Vec3i, CV_MAKETYPE, CV_MAKE_TYPE};
use opencv::prelude::*;

#[test]
fn make_type() {
	if cfg!(ocvrs_opencv_branch_5) {
		assert_eq!(32, CV_MAKETYPE(core::CV_8U, 2));
		assert_eq!(68, CV_MAKE_TYPE(core::CV_32S, 3));
		assert_eq!(6, CV_MAKETYPE(core::CV_64F, 1));
	} else {
		assert_eq!(8, CV_MAKETYPE(core::CV_8U, 2));
		assert_eq!(20, CV_MAKE_TYPE(core::CV_32S, 3));
		assert_eq!(6, CV_MAKETYPE(core::CV_64F, 1));
	}
	assert_eq!(core::CV_32SC3, CV_MAKETYPE(core::CV_32S, 3));
	assert_eq!(core::CV_32FC(6), CV_MAKETYPE(core::CV_32F, 6));
	assert_eq!(core::CV_64FC(4), core::CV_64FC4);
}

#[test]
fn data_type() {
	assert_eq!(core::CV_8U, u8::opencv_type());
	assert_eq!(core::CV_8S, i8::opencv_type());
	assert_eq!(core::CV_16U, u16::opencv_type());
	assert_eq!(core::CV_16S, i16::opencv_type());
	assert_eq!(core::CV_32S, i32::opencv_type());

	assert_eq!(core::CV_32F, f32::opencv_type());
	assert_eq!(core::CV_64F, f64::opencv_type());
	#[cfg(not(ocvrs_opencv_branch_34))]
	{
		#[cfg(ocvrs_has_inherent_feature_hfloat)]
		assert_eq!(core::CV_16F, core::hfloat::opencv_type());
		#[cfg(feature = "f16")]
		assert_eq!(core::CV_16F, half::f16::opencv_type());
	}

	assert_eq!(core::CV_32SC3, Vec3i::opencv_type());
}

#[test]
fn data_type_opencv_5() {
	#![cfg(ocvrs_opencv_branch_5)]

	assert_eq!(core::CV_32U, u32::opencv_type());
	assert_eq!(core::CV_64U, u64::opencv_type());
	assert_eq!(core::CV_64S, i64::opencv_type());

	assert_eq!(core::CV_Bool, bool::opencv_type());

	assert_eq!(core::CV_16BF, core::bfloat::opencv_type());
	#[cfg(feature = "f16")]
	assert_eq!(core::CV_16BF, half::bf16::opencv_type());
}

#[test]
fn data_type_is_int() {
	#![cfg(ocvrs_opencv_branch_5)]

	assert!(core::CV_IS_INT_TYPE(core::CV_8U));
	assert!(core::CV_IS_INT_TYPE(core::CV_8S));
	assert!(core::CV_IS_INT_TYPE(core::CV_16U));
	assert!(core::CV_IS_INT_TYPE(core::CV_16S));
	assert!(core::CV_IS_INT_TYPE(core::CV_32U));
	assert!(core::CV_IS_INT_TYPE(core::CV_32S));
	assert!(core::CV_IS_INT_TYPE(core::CV_64U));
	assert!(core::CV_IS_INT_TYPE(core::CV_64S));
	assert!(core::CV_IS_INT_TYPE(core::CV_64SC3));

	assert!(!core::CV_IS_INT_TYPE(core::CV_16F));
	assert!(!core::CV_IS_INT_TYPE(core::CV_32F));
	assert!(!core::CV_IS_INT_TYPE(core::CV_64F));

	assert!(core::CV_IS_INT_TYPE(core::CV_Bool));
}

#[test]
fn data_type_is_float() {
	#![cfg(ocvrs_opencv_branch_5)]

	assert!(!core::CV_IS_FLOAT_TYPE(core::CV_8U));
	assert!(!core::CV_IS_FLOAT_TYPE(core::CV_8S));
	assert!(!core::CV_IS_FLOAT_TYPE(core::CV_16U));
	assert!(!core::CV_IS_FLOAT_TYPE(core::CV_16S));
	assert!(!core::CV_IS_FLOAT_TYPE(core::CV_32U));
	assert!(!core::CV_IS_FLOAT_TYPE(core::CV_32S));
	assert!(!core::CV_IS_FLOAT_TYPE(core::CV_64U));
	assert!(!core::CV_IS_FLOAT_TYPE(core::CV_64S));

	assert!(core::CV_IS_FLOAT_TYPE(core::CV_16F));
	assert!(core::CV_IS_FLOAT_TYPE(core::CV_32F));
	assert!(core::CV_IS_FLOAT_TYPE(core::CV_64F));
	assert!(core::CV_IS_FLOAT_TYPE(core::CV_64FC4));

	assert!(!core::CV_IS_FLOAT_TYPE(core::CV_Bool));
}

#[test]
fn hfloat() -> opencv::Result<()> {
	#![cfg(ocvrs_has_inherent_feature_hfloat)]
	let f = core::hfloat::new(65.1234)?;
	assert_eq!(65.125, f.to_f32()?);

	assert_eq!(0., core::hfloat::default()?.to_f32()?);
	Ok(())
}
