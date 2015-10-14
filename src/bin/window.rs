extern crate opencv;
use opencv::highgui;
use opencv::imgcodecs;

#[allow(dead_code)]
fn main() {
    let image = imgcodecs::imread("lena.jpg", 0).unwrap();
    highgui::named_window("hello opencv!", 0).unwrap();
    highgui::imshow("hello opencv!", &image).unwrap();
    highgui::wait_key(10000).unwrap();
}
