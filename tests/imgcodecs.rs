use std::mem::transmute;

use opencv::{
    core::{self, Mat, Size, Vec3b, DataType},
    imgcodecs,
};

const PIXEL: &[u8] = include_bytes!("pixel.png");

#[test]
fn decode() {
    {
        let src = Mat::from_slice::<u8>(PIXEL).unwrap();
        let dest = imgcodecs::imdecode(&src, imgcodecs::IMREAD_COLOR).unwrap();
        assert_eq!(dest.size().unwrap(), Size::new(1, 1));
        assert_eq!(dest.channels().unwrap(), 3);
        assert_eq!(*dest.at_2d::<Vec3b>(0, 0).unwrap(), Vec3b::from([56u8, 56, 191]));
    }

    {
        let mut bytes = PIXEL.to_vec();
        let src = Mat::new_rows_cols_with_data(1, PIXEL.len() as _, u8::typ(), unsafe { transmute(bytes.as_mut_ptr()) }, core::Mat_AUTO_STEP).unwrap();
        let mut dest = Mat::default();
        imgcodecs::imdecode_to(&src, imgcodecs::IMREAD_COLOR, &mut dest).unwrap();
        assert_eq!(dest.size().unwrap(), Size::new(1, 1));
        assert_eq!(dest.channels().unwrap(), 3);
        assert_eq!(*dest.at_2d::<Vec3b>(0, 0).unwrap(), Vec3b::from([56u8, 56, 191]));
    }
}
