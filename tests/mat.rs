use std::mem::transmute;

use common::*;
use opencv::{
    core::{self, DataType, Mat, Rect, Scalar, Size, Vec2b, Vec3d, Vec3f},
    Result,
    types::{VectorOfint, VectorOfMat},
};

mod common;

const PIXEL: &[u8] = include_bytes!("pixel.png");

#[test]
fn mat_for_rows_and_cols() -> Result<()> {
    let typ = Vec3d::typ();
    let mat = unsafe { Mat::new_rows_cols(400, 300, typ) }?;
    assert_eq!(mat.typ()?, typ);
    assert!(mat.is_continuous()?);
    assert!(!mat.is_submatrix()?);
    assert_eq!(mat.size()?, Size::new(300, 400));
    assert_eq!(mat.rows()?, 400);
    assert_eq!(mat.cols()?, 300);
    assert_eq!(Vec3d::depth(), mat.depth()?);
    assert_eq!(2, mat.dims()?);
    assert_eq!(3, mat.channels()?);
    assert_eq!(24, mat.elem_size()?);
    assert_eq!(8, mat.elem_size1()?);
    assert_eq!(120000, mat.total()?);
    Ok(())
}

#[test]
fn mat_at_1d_refers_to_rows() -> Result<()> {
    let mut mat =
        Mat::new_rows_cols_with_default(100, 100, f32::typ(), Scalar::all(1.23))?;
    *mat.at_mut::<f32>(5)? = 1.;
    assert_eq!(*mat.at_2d::<f32>(5, 0)?, 1.);
    Ok(())
}

#[test]
fn mat_2d_i0_is_rows_i1_is_cols() -> Result<()> {
    // Just a sanity check about which Mat dimension corresponds to which in Size
    let mat =
        Mat::new_rows_cols_with_default(5, 6, f32::typ(), Scalar::all(1.23))?;
    let size = mat.size()?;
    assert_eq!(size.width, 6);
    assert_eq!(size.height, 5);
    Ok(())
}

#[test]
fn mat_at_2d() -> Result<()> {
    let mut mat = Mat::new_rows_cols_with_default(100, 100, f32::typ(), Scalar::all(1.23))?;
    assert_eq!(*mat.at_2d::<f32>(0, 0)?, 1.23);
    *mat.at_2d_mut::<f32>(0, 0)? = 1.;
    assert_eq!(*mat.at_2d::<f32>(0, 0)?, 1.);
    assert_is_err(mat.at::<i32>(0));
    assert_is_err(mat.at::<f32>(100));
    Ok(())
}

#[test]
fn mat_at_2d_multichannel() -> Result<()> {
    let mut mat = Mat::new_rows_cols_with_default(100, 100, Vec3f::typ(), Scalar::all(1.23))?;
    let pix = *mat.at_2d::<Vec3f>(0, 0)?;
    assert_eq!(pix[0], 1.23);
    assert_eq!(pix[1], 1.23);
    assert_eq!(pix[2], 1.23);

    *mat.at_2d_mut::<Vec3f>(0, 0)? = Vec3f::from([1.1, 2.2, 3.3]);

    let pix = *mat.at_2d::<Vec3f>(0, 0)?;
    assert_eq!(pix[0], 1.1);
    assert_eq!(pix[1], 2.2);
    assert_eq!(pix[2], 3.3);

    assert_is_err(mat.at_2d::<i32>(0, 0));
    assert_is_err(mat.at_2d::<Vec3f>(100, 1));
    assert_is_err(mat.at_2d::<Vec3f>(1, 100));
    Ok(())
}

#[test]
fn mat_at_row() -> Result<()> {
    let mut mat = Mat::new_rows_cols_with_default(100, 100, f32::typ(), Scalar::all(1.23))?;

    let row = mat.at_row::<f32>(0)?;
    assert_eq!(row.len(), 100);
    assert_eq!(row[0], 1.23);

    let row = mat.at_row_mut::<f32>(1)?;
    row[0..4].copy_from_slice(&[10., 20., 30., 40.]);

    let data = mat.data_typed::<f32>()?;
    assert_eq!(data[0], 1.23);
    assert_eq!(data[100], 10.);
    assert_eq!(data[101], 20.);
    assert_eq!(data[102], 30.);
    assert_eq!(data[103], 40.);

    assert_is_err(mat.at_row::<i32>(0));
    assert_is_err(mat.at_row::<i32>(100));
    assert_is_err(mat.at_row_mut::<i32>(0));
    assert_is_err(mat.at_row_mut::<i32>(100));
    Ok(())
}

#[test]
fn mat_vec() -> Result<()> {
    {
        let s = vec![
            vec![1.0f32, 2., 3.],
            vec![4., 5., 6.],
            vec![7., 8., 9.],
        ];

        let mat = Mat::from_slice_2d(&s)?;
        assert_eq!(mat.size()?, core::Size { width: 3, height: 3 });
        assert_eq!(*mat.at_2d::<f32>(1, 1)?, 5.);

        let v = mat.to_vec_2d::<f32>()?;
        assert_eq!(s, v);
    }

    {
        let mut dims = VectorOfint::new();
        dims.push(3);
        dims.push(3);
        dims.push(3);
        let mut mat = Mat::new_nd_with_default(&dims, f64::typ(), Scalar::all(2.))?;
        *mat.at_3d_mut::<f64>(1, 1, 1)? = 10.;
        assert_eq!(3, mat.dims()?);
        if let Ok(..) = mat.to_vec_2d::<f64>() {
            assert!(false, "dims too high");
        }
    }

    Ok(())
}

#[test]
fn mat_continous() -> Result<()> {
    let s = vec![
        vec![1.0f32, 2., 3.],
        vec![4., 5., 6.],
        vec![7., 8., 9.],
    ];

    let mat = Mat::from_slice_2d(&s)?;

    {
        let sub_mat_non_cont = Mat::roi(&mat, Rect::new(1, 1, 2, 2))?;
        assert_eq!(mat.typ()?, sub_mat_non_cont.typ()?);
        assert_eq!(2, sub_mat_non_cont.rows()?);
        assert_eq!(2, sub_mat_non_cont.cols()?);
        assert!(sub_mat_non_cont.is_submatrix()?);
        assert!(!sub_mat_non_cont.is_continuous()?);
        assert_eq!(5., *sub_mat_non_cont.at_2d::<f32>(0, 0)?);
        assert_eq!(6., *sub_mat_non_cont.at_2d::<f32>(0, 1)?);
        assert_eq!(8., *sub_mat_non_cont.at_2d::<f32>(1, 0)?);
        assert_eq!(9., *sub_mat_non_cont.at_2d::<f32>(1, 1)?);

        let vec = sub_mat_non_cont.to_vec_2d::<f32>()?;
        assert_eq!(5., vec[0][0]);
        assert_eq!(6., vec[0][1]);
        assert_eq!(8., vec[1][0]);
        assert_eq!(9., vec[1][1]);

        let mat_clone = sub_mat_non_cont.clone()?;
        assert_eq!(mat.typ()?, mat_clone.typ()?);
        assert_eq!(2, mat_clone.rows()?);
        assert_eq!(2, mat_clone.cols()?);
        assert!(!mat_clone.is_submatrix()?);
        assert!(mat_clone.is_continuous()?);
        assert_eq!(5., *mat_clone.at_2d::<f32>(0, 0)?);
        assert_eq!(6., *mat_clone.at_2d::<f32>(0, 1)?);
        assert_eq!(8., *mat_clone.at_2d::<f32>(1, 0)?);
        assert_eq!(9., *mat_clone.at_2d::<f32>(1, 1)?);
    }

    {
        let sub_mat_cont = Mat::roi(&mat, Rect::new(0, 1, 3, 2))?;
        assert_eq!(mat.typ()?, sub_mat_cont.typ()?);
        assert_eq!(2, sub_mat_cont.rows()?);
        assert_eq!(3, sub_mat_cont.cols()?);
        assert!(sub_mat_cont.is_submatrix()?);
        assert!(sub_mat_cont.is_continuous()?);
        assert_eq!(4., *sub_mat_cont.at_2d::<f32>(0, 0)?);
        assert_eq!(6., *sub_mat_cont.at_2d::<f32>(0, 2)?);
        assert_eq!(7., *sub_mat_cont.at_2d::<f32>(1, 0)?);
        assert_eq!(9., *sub_mat_cont.at_2d::<f32>(1, 2)?);

        let vec = sub_mat_cont.to_vec_2d::<f32>()?;
        assert_eq!(4., vec[0][0]);
        assert_eq!(6., vec[0][2]);
        assert_eq!(7., vec[1][0]);
        assert_eq!(9., vec[1][2]);
    }

    Ok(())
}

#[test]
fn mat_operations() -> Result<()> {
    {
        let mut src = VectorOfMat::new();
        src.push(Mat::new_rows_cols_with_default(1, 3, u8::typ(), Scalar::all(1.))?);
        src.push(Mat::new_rows_cols_with_default(1, 3, u8::typ(), Scalar::all(2.))?);
        let mut merged = Mat::new()?;
        core::merge(&src, &mut merged)?;
        assert_eq!(merged.typ()?, Vec2b::typ());
        assert_eq!(merged.at_2d::<Vec2b>(0, 1)?[0], 1);
        assert_eq!(merged.at_2d::<Vec2b>(0, 2)?[1], 2);

        let mut split = VectorOfMat::new();
        core::split(&merged, &mut split)?;
        assert_eq!(2, split.len());
        let mat = split.get(0);
        assert_eq!(u8::typ(), mat.typ()?);
        assert_eq!(1, mat.channels()?);
        assert_eq!(1, *mat.at_2d::<u8>(0, 2)?);
        let mat = split.get(1);
        assert_eq!(u8::typ(), mat.typ()?);
        assert_eq!(1, mat.channels()?);
        assert_eq!(2, *mat.at_2d::<u8>(0, 0)?);
    }

    Ok(())
}


#[test]
fn mat_from_data() -> Result<()> {
    let mut bytes = PIXEL.to_vec();
    let src = Mat::new_rows_cols_with_data(1, PIXEL.len() as _, u8::typ(), unsafe { transmute(bytes.as_mut_ptr()) }, core::Mat_AUTO_STEP)?;
    assert_eq!(src.size()?, Size::new(PIXEL.len() as _, 1));
    assert_eq!(src.total()?, PIXEL.len());
    let row = src.at_row::<u8>(0)?;
    assert_eq!(row[0], 0x89);
    assert_eq!(row[11], 0x0D);
    assert_eq!(row[89], 0x82);
    Ok(())
}
