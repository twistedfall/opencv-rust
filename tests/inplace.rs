use opencv::core::Vector;
use opencv::prelude::*;
use opencv::{imgproc, Result};

#[test]
fn inplace() -> Result<()> {
	// owned Mat
	{
		let data: [f32; 9] = [1., 2., 3., 4., 5., 6., 7., 8., 9.];
		let mut m = Mat::new_rows_cols_with_data(3, 3, &data)?.clone_pointee();
		unsafe {
			m.modify_inplace(|i, o| imgproc::threshold(i, o, 4., 100., imgproc::THRESH_BINARY))?;
		}
		assert_eq!([0., 0., 0., 0., 100., 100., 100., 100., 100.], m.data_typed::<f32>()?);
	}

	// BoxedRefMut<Mat>
	{
		let mut data: [f32; 9] = [1., 2., 3., 4., 5., 6., 7., 8., 9.];
		let mut m = Mat::new_rows_cols_with_data_mut(3, 3, &mut data)?;
		unsafe {
			m.modify_inplace(|i, o| imgproc::threshold(i, o, 4., 100., imgproc::THRESH_BINARY))?;
		}
		assert_eq!([0., 0., 0., 0., 100., 100., 100., 100., 100.], m.data_typed::<f32>()?);
		assert_eq!([0., 0., 0., 0., 100., 100., 100., 100., 100.], data);
	}

	// Vector
	{
		let mut data = Vector::<f64>::from_slice(&[1., 2., 3., 4., 5., 6., 7., 8., 9.]);
		unsafe {
			data.modify_inplace(|i, o| imgproc::threshold(i, o, 4., 100., imgproc::THRESH_BINARY))?;
		}
		assert_eq!([0., 0., 0., 0., 100., 100., 100., 100., 100.], data.as_slice());
	}
	Ok(())
}
