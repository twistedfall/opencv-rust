use matches::assert_matches;

use opencv::boxed_ref::{BoxedRef, BoxedRefMut};
#[cfg(ocvrs_opencv_branch_4)]
use opencv::core::AccessFlag::ACCESS_READ;
#[cfg(not(ocvrs_opencv_branch_4))]
use opencv::core::ACCESS_READ;
use opencv::core::{
	Matx12d, Scalar, ToInputArray, ToInputOutputArray, ToOutputArray, UMat, Vec2b, VecN, Vector, _InputArray_MAT,
	_InputArray_MATX, _InputArray_STD_BOOL_VECTOR, _InputArray_STD_VECTOR, _InputArray_STD_VECTOR_MAT,
	_InputArray_STD_VECTOR_UMAT, _InputArray_STD_VECTOR_VECTOR, _InputArray_UMAT,
};
use opencv::core::{_InputArray, _InputOutputArray, _OutputArray};
use opencv::prelude::*;
use opencv::types::{VectorOff64, VectorOfu8};
use opencv::{core, Result};

#[test]
fn input_array() -> Result<()> {
	let mat = Mat::new_rows_cols_with_default(10, 10, u8::opencv_type(), Scalar::all(1.))?;
	let input_array = mat.input_array()?;
	assert_eq!(Scalar::from(100), core::sum_elems(&input_array)?);
	Ok(())
}

#[test]
fn input_output_array_types() -> Result<()> {
	#[track_caller]
	fn check_input(
		src: impl ToInputArray,
		expect_kind: i32,
		checker_input: impl FnOnce(&BoxedRef<_InputArray>) -> Result<bool>,
	) -> Result<()> {
		let arr = src.input_array()?;
		assert!(checker_input(&arr)?);
		assert_eq!(expect_kind, arr.kind()? as i32);
		Ok(())
	}

	#[track_caller]
	fn check(
		mut src: impl ToInputArray + ToOutputArray + ToInputOutputArray + Clone,
		expect_kind: i32,
		checker_input: impl FnOnce(&BoxedRef<_InputArray>) -> Result<bool>,
		checker_output: impl FnOnce(&BoxedRefMut<_OutputArray>) -> Result<bool>,
		checker_input_output: impl FnOnce(&BoxedRefMut<_InputOutputArray>) -> Result<bool>,
	) -> Result<()> {
		check_input(src.clone(), expect_kind, checker_input)?;

		let arr = src.output_array()?;
		assert!(checker_output(&arr)?);
		assert_eq!(expect_kind, arr.kind()? as i32);

		let arr = src.input_output_array()?;
		assert!(checker_input_output(&arr)?);
		assert_eq!(expect_kind, arr.kind()? as i32);
		Ok(())
	}

	check(
		Mat::default(),
		_InputArray_MAT,
		|m| m.is_mat(),
		|m| m.is_mat(),
		|m| m.is_mat(),
	)?;
	check(
		UMat::new_rows_cols_with_default_def(1, 1, u8::opencv_type(), 8.into())?,
		_InputArray_UMAT,
		|m| m.is_umat(),
		|m| m.is_umat(),
		|m| m.is_umat(),
	)?;
	check(
		Vector::<u8>::new(),
		_InputArray_STD_VECTOR,
		|m| m.is_vector(),
		|m| m.is_vector(),
		|m| m.is_vector(),
	)?;
	check(
		Vector::<Vector<u8>>::new(),
		_InputArray_STD_VECTOR_VECTOR,
		|_| Ok(true),
		|_| Ok(true),
		|_| Ok(true),
	)?;
	check(
		Vector::<Mat>::new(),
		_InputArray_STD_VECTOR_MAT,
		|m| m.is_mat_vector(),
		|m| m.is_mat_vector(),
		|m| m.is_mat_vector(),
	)?;
	check(
		Vector::<UMat>::new(),
		_InputArray_STD_VECTOR_UMAT,
		|m| m.is_umat_vector(),
		|m| m.is_umat_vector(),
		|m| m.is_umat_vector(),
	)?;
	check_input(Vector::<bool>::new(), _InputArray_STD_BOOL_VECTOR, |_| Ok(true))?;
	// _InputArray_KindFlag::EXPR is no longer used (it's still used in 4.2.0)
	// check_input(Mat::zeros(0, 0, u8::opencv_type())?, _InputArray_KindFlag::MAT, |_| Ok(true))?;
	check(
		Matx12d::default(),
		_InputArray_MATX,
		|m| m.is_matx(),
		|m| m.is_matx(),
		|m| m.is_matx(),
	)?;
	check(
		Scalar::default(),
		_InputArray_MATX,
		|m| m.is_matx(),
		|m| m.is_matx(),
		|m| m.is_matx(),
	)?;
	check(
		Vec2b::default(),
		_InputArray_MATX,
		|m| m.is_matx(),
		|m| m.is_matx(),
		|m| m.is_matx(),
	)?;
	check(
		VecN::<f64, 18>::default(),
		_InputArray_MATX,
		|m| m.is_matx(),
		|m| m.is_matx(),
		|m| m.is_matx(),
	)?;
	check_input(0.0f64, _InputArray_MATX, |m| m.is_matx())?;

	let slice: &[u8] = &[];
	check_input(slice, _InputArray_MATX, |m| m.is_matx())?;

	#[cfg(ocvrs_has_module_cudaimgproc)]
	{
		use opencv::core::{
			GpuMat, HostMem, _InputArray_CUDA_GPU_MAT, _InputArray_CUDA_HOST_MEM, _InputArray_STD_VECTOR_CUDA_GPU_MAT,
		};

		check(
			HostMem::new_def()?,
			_InputArray_CUDA_HOST_MEM,
			|_| Ok(true),
			|_| Ok(true),
			|_| Ok(true),
		)?;
		check(
			GpuMat::new_rows_cols_with_default_def(1, 1, u8::opencv_type(), 0.into())?,
			_InputArray_CUDA_GPU_MAT,
			|m| m.is_gpu_mat(),
			|m| m.is_gpu_mat(),
			|m| m.is_gpu_mat(),
		)?;
		check_input(Vector::<GpuMat>::new(), _InputArray_STD_VECTOR_CUDA_GPU_MAT, |m| {
			m.is_gpu_mat_vector()
		})?;
	}

	Ok(())
}

#[test]
fn input_output_array() -> Result<()> {
	{
		let mat_expr = Mat::ones(1, 3, u8::opencv_type())?;
		let mat = Mat::new_rows_cols_with_default(1, 3, u8::opencv_type(), Scalar::all(3.))?;
		let umat = mat.get_umat_def(ACCESS_READ)?;
		{
			let mut trg = VectorOfu8::new();
			core::add(&mat_expr, &umat, &mut trg, &core::no_array(), -1)?;
			assert_eq!(3, trg.len());
			assert_eq!(4, trg.get(0)?);
			assert_eq!(4, trg.get(1)?);
			assert_eq!(4, trg.get(2)?);
		}

		{
			let mut trg = VectorOfu8::new();
			core::add(&&mat_expr, &&umat, &mut &mut trg, &core::no_array(), -1)?;
			assert_eq!(3, trg.len());
			assert_eq!(4, trg.get(0)?);
			assert_eq!(4, trg.get(1)?);
			assert_eq!(4, trg.get(2)?);
		}
	}

	{
		let mut t = VectorOff64::new();
		core::add(&2.5, &4., &mut t, &core::no_array(), -1)?;
		assert_eq!(6.5, t.get(0)?);
	}

	{
		let mut mat = Mat::from_slice_2d(&[&[1, 2, 3, 4], &[5, 6, 7, 8], &[9, 10, 11, 12], &[13, 14, 15, 16u8]])?;
		core::complete_symm(&mut mat, false)?;
		let expected = Mat::from_slice_2d(&[&[1, 2, 3, 4], &[2, 6, 7, 8], &[3, 7, 11, 12], &[4, 8, 12, 16u8]])?;
		assert_eq!(mat.to_vec_2d::<u8>()?, expected.to_vec_2d()?);
	}

	{
		let bytes = &[1, 2, 3, 4, 5, 6, 7, 8u8];
		let bytes_slice = bytes.as_slice();
		let input_array = bytes_slice.input_array()?;
		assert_eq!(8, input_array.cols_def()?);
		assert_eq!(1, input_array.rows_def()?);

		let input_array = bytes.input_array()?;
		assert_eq!(8, input_array.cols_def()?);
		assert_eq!(1, input_array.rows_def()?);
	}

	Ok(())
}

#[test]
fn no_array() -> Result<()> {
	use self::core::no_array;

	assert!(no_array().empty()?);

	{
		let m = Mat::new_rows_cols_with_default(1, 1, u16::opencv_type(), 0.into())?;
		assert_matches!(core::mean_std_dev(&m, &mut no_array(), &mut no_array(), &no_array()), Ok(()));
	}
	Ok(())
}
