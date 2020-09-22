use opencv::prelude::*;

fn main() {
	let m = Mat::default().unwrap();
	println!("{}", opencv::core::type_to_str(m.typ().unwrap()).unwrap());
}
