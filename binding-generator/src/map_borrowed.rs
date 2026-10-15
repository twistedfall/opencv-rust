use std::borrow::{Borrow, Cow};

pub trait CowMapBorrowedExt<'c, 'b, IN, OUT>
where
	IN: ToOwned + ?Sized + 'b,
	OUT: ToOwned + ?Sized + 'b,
{
	/// Maps the borrowed view of this [`Cow`] to another [`Cow`] using `f`.
	///
	/// For [`Cow::Borrowed`], returns the result of `f` directly, allowing it to borrow
	/// from the original value. For [`Cow::Owned`], converts the result of `f` into
	/// an owned value before the original value is dropped.
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
