extern crate opencv;

use opencv::highgui;

#[allow(dead_code)]
fn main() {
    let image = highgui::imread("lena.jpg", 0).unwrap();
    highgui::named_window("hello opencv!", 0).unwrap();
    highgui::imshow("hello opencv!", &image).unwrap();
    highgui::wait_key(10000).unwrap();
}
