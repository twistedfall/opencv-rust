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

	pub fn get_name<'a>(&mut self, mut name: Cow<'a, str>) -> Cow<'a, str> {
		while self.names.contains(name.as_ref()) {
			name.to_mut().bump_counter();
		}
		self.names.insert(name.clone().into_owned());
		name
	}

	pub fn into_disambiguator<T>(mut self, args: impl IntoIterator<Item=T>, mut name_cb: impl for<'a> FnMut(&'a T) -> Cow<'a, str>) -> impl Iterator<Item=(String, T)> {
		args.into_iter()
			.map(move |f| {
				let name = self.get_name(name_cb(&f)).into_owned();
				(name, f)
			})
	}
}
