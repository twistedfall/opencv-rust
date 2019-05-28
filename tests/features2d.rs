use std::path::PathBuf;

use opencv::{
    core::Mat,
    features2d::{Feature2D, ORB},
    imgcodecs,
    types::{PtrOfORB, VectorOfKeyPoint},
};
use opencv::core::Size;

#[test]
fn orb() {
    let blox_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/blox.jpg");
    let img = imgcodecs::imread(blox_path.to_str().unwrap(), imgcodecs::IMREAD_COLOR).unwrap();
    let mut orb: PtrOfORB = ORB::default().unwrap();
    let mut kp = VectorOfKeyPoint::new();
    let mut des = Mat::default();
    orb.detect_and_compute(&img, &Mat::default(), &mut kp, &mut des, false).unwrap();
    assert_eq!(290, kp.len());
    assert_eq!(Size::new(32, 290), des.size().unwrap());
}
