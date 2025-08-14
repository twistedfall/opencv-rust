use std::fmt;
use std::fmt::Write;

/// Represents a Rust lifetime. Call `next()` to get the next lifetime (supported for `Automatic`, `Elided` and static lifetimes).
///
/// Also implements `Display` to render the lifetime as a string. The implementation supports the following formatting options:
/// * `align` (`<`/`>`) - if specified, then `fill` will be processed, the direction is not taken into account
/// * `fill` - can meaningfully be only "," or " "; the former renders ", " after the lifetime, and the latter renders " "
/// * `alternate` (`#`) - if specified, then `'_` is rendered for an [Lifetime::Elided] lifetime, otherwise empty string
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lifetime {
	/// Rust determines lifetime automatically
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
			Self::Elided => false,
			Self::Custom(_) | Self::Automatic(_) => true,
		}
	}

	pub fn to_explicit(self) -> Self {
		match self {
			Self::Elided => Self::automatic(),
			Self::Custom(_) | Self::Automatic(_) => self,
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
	type Item = Self;
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
			Self::Elided if f.alternate() => {
				f.write_str("'_")?;
				write_align(f)
			}
			Self::Elided => Ok(()),
			Self::Custom(name) => {
				f.write_char('\'')?;
				f.write_str(name)?;
				write_align(f)
			}
			Self::Automatic(n) if n > 25 => panic!("Too many lifetimes"),
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
