extern crate opencv;
use opencv::highgui;
use opencv::core;
use opencv::imgproc;
use opencv::features2d;
use opencv::videoio;

use opencv::features2d::Feature2D;

fn run() -> Result<(), String> {
    let window = "video capture";
    try!(highgui::named_window(window, 1));
    let mut cam = try!(videoio::VideoCapture::device(1));
    let opened = try!(videoio::VideoCapture::is_opened(&cam));
    if !opened {
        println!("Using different camera");
        cam = try!(videoio::VideoCapture::device(0));
    }
    let mut orb = try!(features2d::ORB::create(
        500,
        1.2f32,
        8,
        31,
        0,
        2,
        features2d::ORB_HARRIS_SCORE,
        31
    ));
    loop {
        let mut frame = try!(core::Mat::new());
        try!(cam.read(&mut frame));
        if try!(frame.size()).width > 0 {
            let mut gray = try!(core::Mat::new());
            try!(imgproc::cvt_color(
                &frame,
                &mut gray,
                imgproc::COLOR_BGR2GRAY,
                0
            ));
            let mut kps = opencv::types::VectorOfKeyPoint::new();
            //            let mut desc = try!(core::Mat::new());
            let mask = try!(core::Mat::new());
            try!(orb.detect(&gray, &mut kps, &mask));
            let mut display = try!(core::Mat::new());
            try!(features2d::draw_keypoints(
                &gray,
                &kps,
                &mut display,
                core::Scalar { data: [-1f64; 4] },
                features2d::DrawMatchesFlags_DEFAULT
            ));
            try!(highgui::imshow(window, &display));
        }
        if try!(highgui::wait_key(10)) > 0 {
            break;
        }
    }
    Ok(())
}

fn main() {
    run().unwrap()
}
