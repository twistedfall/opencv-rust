extern crate opencv;

#[allow(dead_code)]
fn main() {
    let image = opencv::imread("lena.jpg", 0).unwrap();
    opencv::named_window("hello opencv!", 0).unwrap();
    opencv::imshow("hello opencv!", &image).unwrap();
    opencv::wait_key(10000).unwrap();
}
