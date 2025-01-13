use std::fmt;
use std::fmt::Write;

/// Represents a Rust lifetime. Call `next()` to get the next lifetime (supported for `Automatic`, `Elided` and static lifetimes).
///
/// Also implements `Display` to render the lifetime as a string. The implementation supports `align` and `fill` formatting options.
/// The `align` option can have any direction, and `fill` can meaningfully be only "," or " ". The former additionally renders ", "
/// after the lifetime, and the latter renders " ".
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lifetime {
	/// Lifetime is determined by Rust automatically
	Elided,
	/// Custom lifetime name (without the apostrophe)
	Custom(&'static str),
	/// Automatic lifetime, starting with `'a` and incrementing for each `next()` lifetime
	Automatic(u8),
}

impl Lifetime {
	pub fn automatic() -> Self {
		Self::Automatic(0)
	}

	pub fn statik() -> Self {
		Self::Custom("static")
	}

	pub fn is_explicit(&self) -> bool {
		match self {
			Lifetime::Elided => false,
			Lifetime::Custom(_) | Lifetime::Automatic(_) => true,
		}
	}

	pub fn next(self) -> Option<Self> {
		match self {
			Self::Elided => Some(Self::Elided),
			Self::Custom("static") => Some(self),
			Self::Custom(_) => None,
			Self::Automatic(n) if n >= 25 => None,
			Self::Automatic(n) => Some(Self::Automatic(n + 1)),
		}
	}
}

impl IntoIterator for Lifetime {
	type Item = Lifetime;
	type IntoIter = LifetimeIterator;

	fn into_iter(self) -> LifetimeIterator {
		LifetimeIterator {
			cur_lifetime: Some(self),
		}
	}
}

impl fmt::Display for Lifetime {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		#[inline]
		fn write_align(f: &mut fmt::Formatter) -> fmt::Result {
			if f.align().is_some() {
				match f.fill() {
					',' => f.write_str(", ")?,
					' ' => f.write_char(' ')?,
					_ => {}
				}
			}
			Ok(())
		}
		match *self {
			Self::Elided => Ok(()),
			Self::Custom(name) => {
				f.write_char('\'')?;
				f.write_str(name)?;
				write_align(f)
			}
			Self::Automatic(n) if n > 25 => {
				panic!("Too many lifetimes")
			}
			Self::Automatic(n) => {
				f.write_char('\'')?;
				f.write_char(char::from(b'a' + n))?;
				write_align(f)
			}
		}
	}
}

pub struct LifetimeIterator {
	cur_lifetime: Option<Lifetime>,
}

impl Iterator for LifetimeIterator {
	type Item = Lifetime;

	fn next(&mut self) -> Option<Self::Item> {
		let out = self.cur_lifetime;
		self.cur_lifetime = self.cur_lifetime.and_then(|l| l.next());
		out
	}
}
