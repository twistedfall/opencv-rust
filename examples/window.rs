use opencv::{highgui, imgcodecs, Result};

fn main() -> Result<()> {
	let image = imgcodecs::imread("lena.jpg", 0)?;
	highgui::named_window("hello opencv!", 0)?;
	highgui::imshow("hello opencv!", &image)?;
	highgui::wait_key(10000)?;
	Ok(())
}
