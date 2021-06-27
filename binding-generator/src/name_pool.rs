use std::{
	borrow::Cow,
	collections::HashSet,
};

use crate::StringExt;

pub struct NamePool {
	names: HashSet<String>,
}

impl NamePool {
	pub fn with_capacity(capacity: usize) -> Self {
		Self { names: HashSet::with_capacity(capacity) }
	}

	pub fn make_unique_name<'a>(&mut self, name: &mut Cow<'a, str>) {
		while self.names.contains(name.as_ref()) {
			name.to_mut().bump_counter();
		}
		self.names.insert(name.clone().into_owned());
	}

	pub fn into_disambiguator<T>(mut self, args: impl IntoIterator<Item=T>, mut name_cb: impl for<'a> FnMut(&'a T) -> Cow<'a, str>) -> impl Iterator<Item=(String, T)> {
		args.into_iter()
			.map(move |f| {
				let mut name = name_cb(&f);
				self.make_unique_name(&mut name);
				(name.into_owned(), f)
			})
	}
}
