use std::mem::transmute;

use opencv::core::{self, DataType, Mat, Rect, Scalar, Size, Vec2b, Vec3d};
use opencv::types::{VectorOfint, VectorOfMat};

mod common;
use common::*;

const PIXEL: &[u8] = include_bytes!("../pixel.png");

#[test]
fn mat_for_rows_and_cols() {
    let typ = Vec3d::typ();
    let mat = unsafe { Mat::new_rows_cols(400, 300, typ) }.unwrap();
    assert_eq!(mat.typ().unwrap(), typ);
    assert!(mat.is_continuous().unwrap());
    assert!(!mat.is_submatrix().unwrap());
    assert_eq!(mat.size().unwrap(), Size::new(300, 400));
    assert_eq!(mat.rows().unwrap(), 400);
    assert_eq!(mat.cols().unwrap(), 300);
    assert_eq!(Vec3d::depth(), mat.depth().unwrap());
    assert_eq!(2, mat.dims().unwrap());
    assert_eq!(3, mat.channels().unwrap());
    assert_eq!(24, mat.elem_size().unwrap());
    assert_eq!(8, mat.elem_size1().unwrap());
    assert_eq!(120000, mat.total().unwrap());
}


#[test]
fn mat_at_2d_CV_32FC1() {
    let mut mat =
        Mat::new_rows_cols_with_default(100, 100, core::CV_32FC1, Scalar::all(1.23)).unwrap();
    assert_almost_eq(*mat.at_2d::<f32>(0, 0).unwrap(), 1.23);
    *mat.at_2d_mut::<f32>(0, 0).unwrap() = 1.;
    assert_almost_eq(*mat.at_2d::<f32>(0, 0).unwrap(), 1.);
    assert_is_err(mat.at::<i32>(0));
}

#[test]
fn mat_at_2d_CV_32FC3() {
    let mut mat =
        Mat::new_rows_cols_with_default(100, 100, core::CV_32FC3, Scalar::all(1.23)).unwrap();
    let pix = *mat.at_2d::<core::Vec3f>(0, 0).unwrap();
    assert_almost_eq(pix[0], 1.23);
    assert_almost_eq(pix[1], 1.23);
    assert_almost_eq(pix[2], 1.23);

    *mat.at_2d_mut::<core::Vec3f>(0, 0).unwrap() = core::Vec3f::from([1.1, 2.2, 3.3]);
    
    let pix = *mat.at_2d::<core::Vec3f>(0, 0).unwrap();
    assert_almost_eq(pix[0], 1.1);
    assert_almost_eq(pix[1], 2.2);
    assert_almost_eq(pix[2], 3.3);
}

#[test]
fn mat_at_row_CV_32FC1() {
    let mut mat = Mat::new_rows_cols_with_default(100, 100, core::CV_32FC1, Scalar::all(1.23)).unwrap();

    let row = mat.at_row::<f32>(0).unwrap();
    assert_eq!(row.len(), 100);
    assert_eq!(row[0], 1.23);

    let row = mat.at_row_mut::<f32>(1).unwrap();
    row[0..4].copy_from_slice(&[10., 20., 30., 40.]);

    let data = mat.data_typed::<f32>().unwrap();
    assert_almost_eq(data[0], 1.23);
    assert_almost_eq(data[100], 10.);
    assert_almost_eq(data[101], 20.);
    assert_almost_eq(data[102], 30.);
    assert_almost_eq(data[103], 40.);
}

#[test]
fn mat_vec() {
    {
        let s = vec![
            vec![1.0f32, 2., 3.],
            vec![4., 5., 6.],
            vec![7., 8., 9.],
        ];

        let mat = Mat::from_slice_2d(&s).unwrap();
        assert_eq!(mat.size().unwrap(), core::Size { width: 3, height: 3 });
        assert_eq!(*mat.at_2d::<f32>(1, 1).unwrap(), 5.);

        let v = mat.to_vec_2d::<f32>().unwrap();
        assert_eq!(s, v);
    }

    {
        let mut dims = VectorOfint::new();
        dims.push(3);
        dims.push(3);
        dims.push(3);
        let mut mat = Mat::new_nd_with_default(&dims, f64::typ(), Scalar::all(2.)).unwrap();
        *mat.at_3d_mut::<f64>(1, 1, 1).unwrap() = 10.;
        assert_eq!(3, mat.dims().unwrap());
        if let Ok(..) = mat.to_vec_2d::<f64>() {
            assert!(false, "dims too high");
        }
    }
}

#[test]
fn mat_continous() {
    let s = vec![
        vec![1.0f32, 2., 3.],
        vec![4., 5., 6.],
        vec![7., 8., 9.],
    ];

    let mat = Mat::from_slice_2d(&s).unwrap();

    {
        let sub_mat_non_cont = Mat::roi(&mat, Rect::new(1, 1, 2, 2)).unwrap();
        assert_eq!(mat.typ().unwrap(), sub_mat_non_cont.typ().unwrap());
        assert_eq!(2, sub_mat_non_cont.rows().unwrap());
        assert_eq!(2, sub_mat_non_cont.cols().unwrap());
        assert!(sub_mat_non_cont.is_submatrix().unwrap());
        assert!(!sub_mat_non_cont.is_continuous().unwrap());
        assert_eq!(5., *sub_mat_non_cont.at_2d::<f32>(0, 0).unwrap());
        assert_eq!(6., *sub_mat_non_cont.at_2d::<f32>(0, 1).unwrap());
        assert_eq!(8., *sub_mat_non_cont.at_2d::<f32>(1, 0).unwrap());
        assert_eq!(9., *sub_mat_non_cont.at_2d::<f32>(1, 1).unwrap());

        let vec = sub_mat_non_cont.to_vec_2d::<f32>().unwrap();
        assert_eq!(5., vec[0][0]);
        assert_eq!(6., vec[0][1]);
        assert_eq!(8., vec[1][0]);
        assert_eq!(9., vec[1][1]);

        let mat_clone = sub_mat_non_cont.clone().unwrap();
        assert_eq!(mat.typ().unwrap(), mat_clone.typ().unwrap());
        assert_eq!(2, mat_clone.rows().unwrap());
        assert_eq!(2, mat_clone.cols().unwrap());
        assert!(!mat_clone.is_submatrix().unwrap());
        assert!(mat_clone.is_continuous().unwrap());
        assert_eq!(5., *mat_clone.at_2d::<f32>(0, 0).unwrap());
        assert_eq!(6., *mat_clone.at_2d::<f32>(0, 1).unwrap());
        assert_eq!(8., *mat_clone.at_2d::<f32>(1, 0).unwrap());
        assert_eq!(9., *mat_clone.at_2d::<f32>(1, 1).unwrap());
    }

    {
        let sub_mat_cont = Mat::roi(&mat, Rect::new(0, 1, 3, 2)).unwrap();
        assert_eq!(mat.typ().unwrap(), sub_mat_cont.typ().unwrap());
        assert_eq!(2, sub_mat_cont.rows().unwrap());
        assert_eq!(3, sub_mat_cont.cols().unwrap());
        assert!(sub_mat_cont.is_submatrix().unwrap());
        assert!(sub_mat_cont.is_continuous().unwrap());
        assert_eq!(4., *sub_mat_cont.at_2d::<f32>(0, 0).unwrap());
        assert_eq!(6., *sub_mat_cont.at_2d::<f32>(0, 2).unwrap());
        assert_eq!(7., *sub_mat_cont.at_2d::<f32>(1, 0).unwrap());
        assert_eq!(9., *sub_mat_cont.at_2d::<f32>(1, 2).unwrap());

        let vec = sub_mat_cont.to_vec_2d::<f32>().unwrap();
        assert_eq!(4., vec[0][0]);
        assert_eq!(6., vec[0][2]);
        assert_eq!(7., vec[1][0]);
        assert_eq!(9., vec[1][2]);
    }
}

#[test]
fn mat_operations() {
    {
        let mut src = VectorOfMat::new();
        src.push(Mat::new_rows_cols_with_default(1, 3, u8::typ(), Scalar::all(1.)).unwrap());
        src.push(Mat::new_rows_cols_with_default(1, 3, u8::typ(), Scalar::all(2.)).unwrap());
        let mut merged = Mat::default();
        core::merge(&src, &mut merged).unwrap();
        assert_eq!(merged.typ().unwrap(), Vec2b::typ());
        assert_eq!(merged.at_2d::<Vec2b>(0, 1).unwrap()[0], 1);
        assert_eq!(merged.at_2d::<Vec2b>(0, 2).unwrap()[1], 2);

        let mut split = VectorOfMat::new();
        core::split(&merged, &mut split).unwrap();
        assert_eq!(2, split.len());
        let mat = split.get(0);
        assert_eq!(u8::typ(), mat.typ().unwrap());
        assert_eq!(1, mat.channels().unwrap());
        assert_eq!(1, *mat.at_2d::<u8>(0, 2).unwrap());
        let mat = split.get(1);
        assert_eq!(u8::typ(), mat.typ().unwrap());
        assert_eq!(1, mat.channels().unwrap());
        assert_eq!(2, *mat.at_2d::<u8>(0, 0).unwrap());
    }

}


#[test]
fn mat_from_data() {
    let mut bytes = PIXEL.to_vec();
    let src = Mat::new_rows_cols_with_data(1, PIXEL.len() as _, u8::typ(), unsafe { transmute(bytes.as_mut_ptr()) }, core::Mat_AUTO_STEP).unwrap();
    assert_eq!(src.size().unwrap(), Size::new(PIXEL.len() as _, 1));
    assert_eq!(src.total().unwrap(), PIXEL.len());
    let row = src.at_row::<u8>(0).unwrap();
    assert_eq!(row[0], 0x89);
    assert_eq!(row[11], 0x0D);
    assert_eq!(row[89], 0x82);
}
