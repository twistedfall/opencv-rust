extern crate opencv;

use opencv::highgui;

#[allow(dead_code)]
fn main() {
    let image = highgui::imread("jarres.jpg", 0).unwrap();
    highgui::namedWindow("hello opencv!", 0).unwrap();
    highgui::imshow("hello opencv!", &image).unwrap();
    highgui::waitKey(10000).unwrap();
}
