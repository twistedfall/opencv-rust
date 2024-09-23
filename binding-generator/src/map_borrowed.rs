use std::borrow::{Borrow, Cow};

pub trait CowMapBorrowedExt<'c, 'b, IN, OUT>
where
	IN: ToOwned + ?Sized + 'b,
	OUT: ToOwned + ?Sized + 'b,
{
	fn map_borrowed<F>(self, f: F) -> Cow<'c, OUT>
	where
		F: for<'f> FnOnce(&'f IN) -> Cow<'f, OUT>;
}

impl<'c, 'b, IN, OUT> CowMapBorrowedExt<'c, 'b, IN, OUT> for Cow<'c, IN>
where
	IN: ToOwned + ?Sized + 'b,
	OUT: ToOwned + ?Sized + 'b,
{
	#[inline(always)]
	fn map_borrowed<F>(self, f: F) -> Cow<'c, OUT>
	where
		F: for<'f> FnOnce(&'f IN) -> Cow<'f, OUT>,
	{
		match self {
			Cow::Borrowed(v) => f(v),
			Cow::Owned(v) => Cow::Owned(f(v.borrow()).into_owned()),
		}
	}
}
