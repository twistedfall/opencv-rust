use std::path::PathBuf;

use opencv::{
    core::Size,
    features2d::{Feature2DTrait, ORB},
    imgcodecs,
    prelude::*,
    Result,
    types::{PtrOfORB, VectorOfKeyPoint}
};

#[test]
fn orb() -> Result<()> {
    let blox_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/blox.jpg");
    let img = imgcodecs::imread(blox_path.to_str().unwrap(), imgcodecs::IMREAD_COLOR)?;
    let mut orb: PtrOfORB = ORB::default()?;
    let mut kp = VectorOfKeyPoint::new();
    let mut des = Mat::default()?;
    orb.detect_and_compute(&img, &Mat::default()?, &mut kp, &mut des, false)?;
    let size = if cfg!(feature = "opencv-32") { 296 } else { 290 };
    assert_eq!(size, kp.len());
    assert_eq!(Size::new(32, size as i32), des.size()?);
    Ok(())
}
