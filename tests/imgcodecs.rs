#![cfg(ocvrs_has_module_imgcodecs)]

use std::ffi::c_void;

use opencv::{
	core::{self, Size, Vec3b},
	imgcodecs,
	prelude::*,
	Result,
};

const PIXEL: &[u8] = include_bytes!("pixel.png");

#[test]
fn decode() -> Result<()> {
	{
		let src = Mat::from_slice::<u8>(PIXEL)?;
		let dest = imgcodecs::imdecode(&src, imgcodecs::IMREAD_COLOR)?;
		assert_eq!(dest.size()?, Size::new(1, 1));
		assert_eq!(dest.channels(), 3);
		assert_eq!(*dest.at_2d::<Vec3b>(0, 0)?, Vec3b::from([56u8, 56, 191]));
	}

	{
		let mut bytes = PIXEL.to_vec();
		let src = unsafe {
			Mat::new_rows_cols_with_data(
				1,
				PIXEL.len() as _,
				u8::opencv_type(),
				bytes.as_mut_ptr() as *mut c_void,
				core::Mat_AUTO_STEP,
			)
		}?;
		let mut dest = Mat::default();
		imgcodecs::imdecode_to(&src, imgcodecs::IMREAD_COLOR, &mut dest)?;
		assert_eq!(dest.size()?, Size::new(1, 1));
		assert_eq!(dest.channels(), 3);
		assert_eq!(*dest.at_2d::<Vec3b>(0, 0)?, Vec3b::from([56u8, 56, 191]));
	}

	Ok(())
}
