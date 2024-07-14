use crate::boxed_ref::BoxedRefMut;
use crate::traits::Boxed;

pub trait ModifyInplace {
	/// Helper function to call OpenCV functions that allow in-place modification of a `Mat` or another similar object. By passing
	/// a mutable reference to the `Mat` to this function your closure will get called with the read reference and a write references
	/// to the same `Mat`. This is unsafe in a general case as it leads to having non-exclusive mutable access to the internal data,
	/// but it can be useful for some performance sensitive operations. One example of an OpenCV function that allows such in-place
	/// modification is `imgproc::threshold`.
	///
	/// # Safety
	/// Caller must make sure that any functions called inside the closure can act on a `Mat` in-place.
	unsafe fn modify_inplace<Res>(&mut self, f: impl FnOnce(&Self, &mut Self) -> Res) -> Res;
}

impl<Mat: Boxed> ModifyInplace for Mat {
	#[inline(always)]
	unsafe fn modify_inplace<Res>(&mut self, f: impl FnOnce(&Self, &mut Self) -> Res) -> Res {
		let mut m_alias = Mat::from_raw(self.as_raw_mut());
		let out = f(self, &mut m_alias);
		// prevent running destructor on m_alias
		let _ = m_alias.into_raw();
		out
	}
}

impl<Mat: Boxed> ModifyInplace for BoxedRefMut<'_, Mat> {
	#[inline(always)]
	unsafe fn modify_inplace<Res>(&mut self, f: impl FnOnce(&Self, &mut Self) -> Res) -> Res {
		let mut m_alias = BoxedRefMut::from(Mat::from_raw(self.reference.as_raw_mut()));
		let out = f(self, &mut m_alias);
		// prevent running destructor on m_alias
		let _ = m_alias.reference.into_raw();
		out
	}
}
