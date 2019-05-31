use opencv::{
    core::{DataType, Mat, Point2d, Scalar},
    Result,
    types::{VectorOfbool, VectorOfint, VectorOfMat, VectorOfPoint2d},
};

#[test]
fn types() -> Result<()> {
    {
        let mut vec = VectorOfint::with_capacity(10);
        vec.push(1);
        vec.push(2);
        vec.push(3);
        assert_eq!(1, *vec.get(0));
        assert_eq!(2, *vec.get(1));
        assert_eq!(3, *vec.get(2));
    }

    {
        let mut vec = VectorOfbool::new();
        vec.push(true);
        vec.push(true);
        vec.push(false);
        assert_eq!(true, *vec.get(0));
        assert_eq!(true, *vec.get(1));
        assert_eq!(false, *vec.get(2));
    }

    {
        let mut vec = VectorOfMat::new();
        vec.push(Mat::new_rows_cols_with_default(1, 3, u8::typ(), Scalar::all(1.))?);
        vec.push(Mat::new_rows_cols_with_default(1, 3, u16::typ(), Scalar::all(2.))?);
        vec.push(Mat::new_rows_cols_with_default(1, 3, i32::typ(), Scalar::all(3.))?);
        assert_eq!(u8::typ(), vec.get(0).typ()?);
        assert_eq!(1, *vec.get(0).at_2d::<u8>(0, 1)?);
        assert_eq!(u16::typ(), vec.get(1).typ()?);
        assert_eq!(2, *vec.get(1).at_2d::<u16>(0, 1)?);
        assert_eq!(i32::typ(), vec.get(2).typ()?);
        assert_eq!(3, *vec.get(2).at_2d::<i32>(0, 1)?);
    }

    {
        let mut vec = VectorOfPoint2d::new();
        vec.push(Point2d::new(10., 10.));
        vec.push(Point2d::new(20., 20.));
        vec.push(Point2d::new(30., 30.));
        assert_eq!(Point2d::new(10., 10.), *vec.get(0));
        assert_eq!(Point2d::new(20., 20.), *vec.get(1));
        assert_eq!(Point2d::new(30., 30.), *vec.get(2));
    }

    Ok(())
}

#[test]
fn capacity() {
    {
        let mut vec = VectorOfint::with_capacity(10);
        assert_eq!(0, vec.len());
        assert_eq!(10, vec.capacity());
        vec.push(1);
        assert_eq!(1, vec.len());
        assert_eq!(10, vec.capacity());
        vec.reserve(10);
        assert_eq!(1, vec.len());
        assert_eq!(11, vec.capacity());
        vec.reserve(10);
        assert_eq!(11, vec.capacity());
    }

    {
        let mut vec = VectorOfbool::new();
        assert_eq!(0, vec.len());
        assert_eq!(0, vec.capacity());
        vec.push(true);
        assert_eq!(1, vec.len());
        assert_eq!(64, vec.capacity());
    }
}
