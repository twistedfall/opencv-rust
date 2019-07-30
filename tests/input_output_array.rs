use opencv::{
    core::{self, Mat, Scalar, UMatUsageFlags},
    prelude::*,
    Result,
    types::{VectorOfdouble, VectorOfuchar},
};

#[cfg(not(feature = "opencv-41"))]
use opencv::core::ACCESS_READ;
#[cfg(feature = "opencv-41")]
use opencv::core::AccessFlag::ACCESS_READ;

#[test]
fn input_output_array() -> Result<()> {
    {
        let mat_expr = Mat::ones(1, 3, u8::typ())?;
        let umat = Mat::new_rows_cols_with_default(1, 3, u8::typ(), Scalar::all(3.))?.get_umat(ACCESS_READ, UMatUsageFlags::USAGE_DEFAULT)?;
        let mut trg = VectorOfuchar::new();
        core::add(&mat_expr, &umat, &mut trg, &Mat::default()?, -1)?;
        assert_eq!(3, trg.len());
        assert_eq!(4, trg.get(0)?);
        assert_eq!(4, trg.get(1)?);
        assert_eq!(4, trg.get(2)?);
    }

    {
        let mut t = VectorOfdouble::new();
        core::add(&2.5, &4., &mut t, &Mat::default()?, -1)?;
        assert_eq!(6.5, t.get(0)?);
    }

    {
        let mut mat = Mat::from_slice_2d(&[
            &[ 1,  2,  3,  4],
            &[ 5,  6,  7,  8],
            &[ 9, 10, 11, 12],
            &[13, 14, 15, 16u8],
        ])?;
        core::complete_symm(&mut mat, false)?;
        let expected = Mat::from_slice_2d(&[
            &[ 1,  2,  3,  4],
            &[ 2,  6,  7,  8],
            &[ 3,  7, 11, 12],
            &[ 4,  8, 12, 16u8],
        ])?;
        assert_eq!(mat.to_vec_2d::<u8>()?, expected.to_vec_2d()?);
    }

    Ok(())
}
