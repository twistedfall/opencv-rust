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

	pub fn make_unique_name(&mut self, name: &mut Cow<str>) -> MakeUniqueNameResult {
		let mut out = MakeUniqueNameResult::Unchanged;
		while self.names.contains(name.as_ref()) {
			name.to_mut().bump_counter();
			out = MakeUniqueNameResult::Changed;
		}
		self.names.insert(name.to_string());
		out
	}

	pub fn into_disambiguator<T, I, CB>(mut self, args: I, name_cb: CB) -> impl Iterator<Item = (String, T)>
	where
		I: IntoIterator<Item = T>,
		CB: for<'a> Fn(&'a T) -> Cow<'a, str>,
	{
		args.into_iter().map(move |item| {
			let mut name = name_cb(&item);
			self.make_unique_name(&mut name);
			(name.into_owned(), item)
		})
	}
}

#[derive(Copy, Clone)]
pub enum MakeUniqueNameResult {
	Unchanged,
	Changed,
}

impl MakeUniqueNameResult {
	pub fn is_changed(self) -> bool {
		match self {
			MakeUniqueNameResult::Unchanged => false,
			MakeUniqueNameResult::Changed => true,
		}
	}
}
