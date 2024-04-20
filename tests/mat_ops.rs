use opencv::core::{add_mat_mat, ElemMul, Scalar, StsBadArg};
use opencv::prelude::*;
use opencv::{Error, Result};

#[test]
fn mat_ops() -> Result<()> {
	#![allow(non_upper_case_globals)]
	// success 2 operands
	{
		let lhs = Mat::new_rows_cols_with_default(3, 3, u8::opencv_type(), 3.into())?;
		let rhs = Mat::new_rows_cols_with_default(3, 3, u8::opencv_type(), 8.into())?;
		let res = (&lhs + &rhs).into_result()?.to_mat()?;
		let expected = Mat::new_rows_cols_with_default(3, 3, u8::opencv_type(), 11.into())?;
		assert_eq!(expected.data_typed::<u8>()?, res.data_typed()?);

		let res = add_mat_mat(&lhs, &rhs)?.to_mat()?;
		assert_eq!(expected.data_typed::<u8>()?, res.data_typed()?);
	}

	// success 3 operands
	{
		let lhs = Mat::new_rows_cols_with_default(3, 3, u8::opencv_type(), 3.into())?;
		let rhs = Mat::new_rows_cols_with_default(3, 3, u8::opencv_type(), 8.into())?;
		let rhs2 = Mat::new_rows_cols_with_default(3, 3, u8::opencv_type(), 20.into())?;
		let res = (&lhs + &rhs - &rhs2).into_result()?.to_mat()?;
		let expected = Mat::new_rows_cols_with_default(3, 3, u8::opencv_type(), (-9).into())?;
		assert_eq!(expected.data_typed::<u8>()?, res.data_typed()?);
	}

	// 3rd operand error
	{
		let lhs = Mat::new_rows_cols_with_default(3, 3, u8::opencv_type(), 3.into())?;
		let rhs = Mat::new_rows_cols_with_default(3, 3, u8::opencv_type(), 8.into())?;
		let rhs2 = Mat::default();
		let res = (&lhs + &rhs - &rhs2).into_result();
		assert!(matches!(res, Err(Error { code: StsBadArg, .. })));
	}

	// 1st operand error
	{
		let lhs = Mat::default();
		let rhs = Mat::new_rows_cols_with_default(3, 3, u8::opencv_type(), 8.into())?;
		let rhs2 = Mat::new_rows_cols_with_default(3, 3, u8::opencv_type(), 20.into())?;
		let res = (&lhs + &rhs - &rhs2).into_result();
		assert!(matches!(res, Err(Error { code: StsBadArg, .. })));
	}

	// result + operand error
	{
		let m = Mat::default();
		let res = (&m + &m - (&m + &m)).into_result();
		assert!(matches!(res, Err(Error { code: StsBadArg, .. })));
		assert!(res.err().unwrap().message.starts_with("Both sides of operator have error"));
	}

	// empty operands are an error
	{
		let lhs = Mat::default();
		let rhs = Mat::default();
		let res = (lhs + rhs).into_result();
		assert!(matches!(res, Err(Error { code: StsBadArg, .. })));
	}

	// ElemMul
	{
		let lhs = Mat::new_rows_cols_with_default(3, 3, u8::opencv_type(), 11.into())?;
		let rhs = Mat::new_rows_cols_with_default(3, 3, u8::opencv_type(), 2.into())?;
		let res = lhs.elem_mul(&rhs).into_result()?.to_mat()?;

		let expected = Mat::new_rows_cols_with_default(3, 3, u8::opencv_type(), 22.into())?;
		assert_eq!(expected.data_typed::<u8>()?, res.data_typed()?);
	}

	// Scalar
	{
		let lhs = Mat::new_rows_cols_with_default(3, 3, u8::opencv_type(), 11.into())?;
		let rhs = Scalar::from(5);
		let res = (&lhs + rhs).into_result()?.to_mat()?;

		let expected = Mat::new_rows_cols_with_default(3, 3, u8::opencv_type(), 16.into())?;
		assert_eq!(expected.data_typed::<u8>()?, res.data_typed()?);

		let res = (rhs + &lhs).into_result()?.to_mat()?;
		assert_eq!(expected.data_typed::<u8>()?, res.data_typed()?);
	}

	// f64
	{
		let lhs = Mat::new_rows_cols_with_default(3, 3, u8::opencv_type(), 11.into())?;
		let rhs = 5f64;
		let res = (&lhs * rhs).into_result()?.to_mat()?;

		let expected = Mat::new_rows_cols_with_default(3, 3, u8::opencv_type(), 55.into())?;
		assert_eq!(expected.data_typed::<u8>()?, res.data_typed()?);

		let res = (rhs * &lhs).into_result()?.to_mat()?;
		assert_eq!(expected.data_typed::<u8>()?, res.data_typed()?);
	}

	Ok(())
}
