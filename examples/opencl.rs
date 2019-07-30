use std::time;

use opencv::{core, imgcodecs, imgproc, Result};
#[cfg(not(feature = "opencv-41"))]
use opencv::core::ACCESS_READ;
#[cfg(feature = "opencv-41")]
use opencv::core::AccessFlag::ACCESS_READ;

fn main() -> Result<()> {
//    core::set_use_opencl(false)?;
    let img = imgcodecs::imread("image.jpg", imgcodecs::IMREAD_COLOR)?.get_umat(ACCESS_READ, core::UMatUsageFlags::USAGE_DEFAULT)?;
    let start = time::Instant::now();
    for _ in 0..200 {
        let mut gray = core::UMat::new(core::UMatUsageFlags::USAGE_DEFAULT)?;
        imgproc::cvt_color(&img, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;
        let mut gray1 = core::UMat::new(core::UMatUsageFlags::USAGE_DEFAULT)?;
        imgproc::gaussian_blur(&gray, &mut gray1, core::Size::new(7, 7), 1.5, 0., core::BORDER_DEFAULT)?;
        let mut gray2 = core::UMat::new(core::UMatUsageFlags::USAGE_DEFAULT)?;
        imgproc::canny(&gray1, &mut gray2, 0., 50., 3, false)?;
    }
    eprintln!("Total time: {:#?}", start.elapsed());
    Ok(())
}
