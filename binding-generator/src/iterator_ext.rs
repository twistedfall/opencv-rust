use crate::StringExt;

pub trait IteratorExt {
	fn join(self, sep: &str) -> String;
}

impl<T: Iterator<Item = impl AsRef<str>>> IteratorExt for T {
	fn join(self, sep: &str) -> String {
		let mut out = String::new();
		out.extend_join(self, sep);
		out
	}
}
