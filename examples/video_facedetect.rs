extern crate opencv as cv;

fn run() -> Result<(),String> {
    let window = "video capture";
    let xml = "/usr/local/share/OpenCV/haarcascades/haarcascade_frontalface_alt.xml";
    try!(cv::named_window(window,1));
    let mut cam = try!(cv::VideoCapture::device(1));
    let mut face = try!(cv::CascadeClassifier::new(xml));
    loop {
        let mut frame = try!(cv::Mat::new());
        try!(cam.read(&mut frame));
        if try!(frame.size()).width == 0 {
            ::std::thread::sleep_ms(50);
            continue;
        }
        let mut gray = try!(cv::Mat::new());
        try!(cv::cvt_color(&frame, &mut gray, cv::CV_BGR2GRAY, 0));
        let mut reduced = try!(cv::Mat::new());
        try!(cv::resize(&gray, &mut reduced, cv::Size{width:0,height:0},
            0.25f64, 0.25f64, cv::INTER_LINEAR));
        let mut faces = cv::VectorOfRect::new();
        try!(face.detect_multi_scale(&reduced, &mut faces, 1.1, 2,
            cv::CV_HAAR_SCALE_IMAGE,
            cv::Size{ width:30, height:30 },
            cv::Size{ width:0, height:0 }));
        println!("faces: {}", faces.len());
        for face in faces.iter() {
            println!("face {:?}", face);
            let scaled_face = cv::Rect{
                x: face.x*4, y:face.y*4,
                width:face.width*4, height:face.height*4
            };
            try!(cv::rectangle(&frame, scaled_face,
                cv::Scalar{ data:[0f64,-1f64,-1f64,-1f64] },
                1, 8, 0));
        }
        try!(cv::imshow(window, &frame));
        if try!(cv::wait_key(10)) > 0 {
            break;
       }
    }
    Ok(())
}

fn main() {
    run().unwrap()
}
