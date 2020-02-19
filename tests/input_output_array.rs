use matches::assert_matches;

use opencv::{
    core::{self, Scalar, UMatUsageFlags},
    prelude::*,
    Result,
    types::{VectorOff64, VectorOfu8},
};
#[cfg(not(feature = "opencv-4"))]
use opencv::core::ACCESS_READ;
#[cfg(feature = "opencv-4")]
use opencv::core::AccessFlag::ACCESS_READ;

#[test]
fn input_output_array() -> Result<()> {
    {
        let mat_expr = Mat::ones(1, 3, u8::typ())?;
        let umat = Mat::new_rows_cols_with_default(1, 3, u8::typ(), Scalar::all(3.))?.get_umat(ACCESS_READ, UMatUsageFlags::USAGE_DEFAULT)?;
        {
            let mut trg = VectorOfu8::new();
            core::add(&mat_expr, &umat, &mut trg, &Mat::default()?, -1)?;
            assert_eq!(3, trg.len());
            assert_eq!(4, trg.get(0)?);
            assert_eq!(4, trg.get(1)?);
            assert_eq!(4, trg.get(2)?);
        }

        {
            let mut trg = VectorOfu8::new();
            core::add(&&mat_expr, &&umat, &mut &mut trg, &Mat::default()?, -1)?;
            assert_eq!(3, trg.len());
            assert_eq!(4, trg.get(0)?);
            assert_eq!(4, trg.get(1)?);
            assert_eq!(4, trg.get(2)?);
        }
    }

    {
        let mut t = VectorOff64::new();
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

#[test]
fn no_array() -> Result<()> {
    use self::core::no_array;
    assert_eq!(Scalar::all(0.), core::sum_elems(&no_array()?)?);
    assert_matches!(core::complete_symm(&mut no_array()?, false), Ok(()));
    let m = Mat::new_rows_cols_with_default(1, 1, u16::typ(), Scalar::all(0.))?;
    assert_matches!(core::mean_std_dev(&m, &mut no_array()?, &mut no_array()?, &no_array()?), Ok(()));
    Ok(())
}
