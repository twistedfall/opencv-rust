use opencv::core;
use opencv::features2d;
use opencv::features2d::Feature2D;
use opencv::highgui;
use opencv::imgproc;
use opencv::videoio;

fn run() -> opencv::Result<()> {
    let window = "video capture";
    highgui::named_window(window, 1)?;
    let mut cam = videoio::VideoCapture::new(0)?;  // 0 is the default camera
    let opened = videoio::VideoCapture::is_opened(&cam)?;
    if !opened {
        panic!("Unable to open default camera!");
    }
    let mut orb = features2d::ORB::create(
        500,
        1.2f32,
        8,
        31,
        0,
        2,
        features2d::ORB_HARRIS_SCORE,
        31,
        20
    )?;
    loop {
        let mut frame = core::Mat::new()?;
        cam.read(&mut frame)?;
        if frame.size()?.width > 0 {
            let mut gray = core::Mat::new()?;
            imgproc::cvt_color(
                &frame,
                &mut gray,
                imgproc::COLOR_BGR2GRAY,
                0
            )?;
            let mut kps = opencv::types::VectorOfKeyPoint::new();
            //            let mut desc = try!(core::Mat::new());
            let mask = core::Mat::new()?;
            orb.detect(&gray, &mut kps, &mask)?;
            let mut display = core::Mat::new()?;
            features2d::draw_keypoints(
                &gray,
                &kps,
                &mut display,
                core::Scalar::all(-1f64),
                features2d::DrawMatchesFlags_DEFAULT
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
