use opencv::prelude::*;
use opencv::core;
use opencv::features2d;
use opencv::features2d::Feature2DTrait;
use opencv::highgui;
use opencv::imgproc;
use opencv::videoio;

fn run() -> opencv::Result<()> {
    let window = "video capture";
    highgui::named_window(window, 1)?;
    #[cfg(feature = "opencv-32")]
    let mut cam = videoio::VideoCapture::new(0)?;  // 0 is the default camera
    #[cfg(not(feature = "opencv-32"))]
    let mut cam = videoio::VideoCapture::new_with_backend(0, videoio::CAP_ANY)?;  // 0 is the default camera
    let opened = videoio::VideoCapture::is_opened(&cam)?;
    if !opened {
        panic!("Unable to open default camera!");
    }
    let mut orb = features2d::ORB::default()?;
    loop {
        let mut frame = core::Mat::default()?;
        cam.read(&mut frame)?;
        if frame.size()?.width > 0 {
            let mut gray = core::Mat::default()?;
            imgproc::cvt_color(
                &frame,
                &mut gray,
                imgproc::COLOR_BGR2GRAY,
                0
            )?;
            let mut kps = opencv::types::VectorOfKeyPoint::new();
            //            let mut desc = try!(core::Mat::new());
            let mask = core::Mat::default()?;
            orb.detect(&gray, &mut kps, &mask)?;
            let mut display = core::Mat::default()?;
            #[cfg(not(feature = "opencv-41"))]
            let default_draw_matches_flags = features2d::DrawMatchesFlags_DEFAULT;
            #[cfg(feature = "opencv-41")]
            let default_draw_matches_flags = features2d::DrawMatchesFlags::DEFAULT;
            features2d::draw_keypoints(
                &gray,
                &kps,
                &mut display,
                core::Scalar::all(-1f64),
                default_draw_matches_flags
            )?;
            highgui::imshow(window, &display)?;
        }
        if highgui::wait_key(10)? > 0 {
            break;
        }
    }
    Ok(())
}

fn main() {
    run().unwrap()
}
