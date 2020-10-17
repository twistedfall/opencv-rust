use opencv::prelude::*;

fn main() {
	let m = Mat::default().unwrap();
	println!("{}", m.typ().unwrap());
}
