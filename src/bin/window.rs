extern crate opencv;

use opencv::highgui;

#[allow(dead_code)]
fn main() {
    let image = highgui::imread("jarres.jpg", 0);
    highgui::namedWindow("hello opencv!", 0);
    highgui::imshow("hello opencv!", &image);
    highgui::waitKey(10000);
}
