use std::path::PathBuf;

use opencv::{
    prelude::*,
    features2d::{Feature2D, ORB},
    imgcodecs,
    Result,
    types::{PtrOfORB, VectorOfKeyPoint},
};
use opencv::core::Size;

#[test]
fn orb() -> Result<()> {
    let blox_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/blox.jpg");
    let img = imgcodecs::imread(blox_path.to_str().unwrap(), imgcodecs::IMREAD_COLOR)?;
    let mut orb: PtrOfORB = ORB::default()?;
    let mut kp = VectorOfKeyPoint::new();
    let mut des = Mat::new()?;
    orb.detect_and_compute(&img, &Mat::new()?, &mut kp, &mut des, false)?;
    assert_eq!(290, kp.len());
    assert_eq!(Size::new(32, 290), des.size()?);
    Ok(())
}
