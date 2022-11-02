use std::borrow::Cow;
use std::collections::HashSet;

use crate::StringExt;

pub struct NamePool {
	names: HashSet<String>,
}

impl NamePool {
	pub fn with_capacity(capacity: usize) -> Self {
		Self {
			names: HashSet::with_capacity(capacity),
		}
	}

	pub fn make_unique_name<'a>(&mut self, name: &mut Cow<'a, str>) {
		while self.names.contains(name.as_ref()) {
			name.to_mut().bump_counter();
		}
		self.names.insert(name.clone().into_owned());
	}

	pub fn into_disambiguator<T, I, CB>(mut self, args: I, mut name_cb: CB) -> impl Iterator<Item = (String, T)>
	where
		I: IntoIterator<Item = T>,
		CB: for<'a> FnMut(&'a T) -> Cow<'a, str>,
	{
		args.into_iter().map(move |f| {
			let mut name = name_cb(&f);
			self.make_unique_name(&mut name);
			(name.into_owned(), f)
		})
	}
}
