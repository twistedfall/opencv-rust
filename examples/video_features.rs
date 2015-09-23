extern crate opencv as cv;

use cv::FeatureDetector;

fn run() -> Result<(),String> {
    let window = "video capture";
    try!(cv::named_window(window,1));
    let mut cam = try!(cv::VideoCapture::device(1));
    let mut orb = try!(cv::ORB::new(500, 1.2f32, 8, 31, 0, 2,
        cv::ORB_HARRIS_SCORE, 31));
    loop {
        let mut frame = try!(cv::Mat::new());
        try!(cam.read(&mut frame));
        if try!(frame.size()).width > 0 {
            let mut gray = try!(cv::Mat::new());
            try!(cv::cvt_color(&frame, &mut gray, cv::CV_BGR2GRAY, 0));
            let mut kps = cv::VectorOfKeyPoint::new();
            let mut desc = try!(cv::Mat::new());
            let mask = try!(cv::Mat::new());
//            try!(orb.detect_and_compute(&gray, &mask, &mut kps, &mut desc, false));
            let mut display = try!(cv::Mat::new());
            try!(cv::draw_keypoints(&gray, &kps, &mut display,
                cv::Scalar { data:[-1f64;4] }, cv::DrawMatchesFlags_DEFAULT));
            try!(cv::imshow(window, &display));
        }
        if try!(cv::wait_key(10)) > 0 {
            break;
        }
    }
    Ok(())
}

fn main() {
    run().unwrap()
}
