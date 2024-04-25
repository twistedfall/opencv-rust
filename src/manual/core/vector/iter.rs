use std::iter::FusedIterator;

use crate::core::{Vector, VectorExtern};
use crate::platform_types::size_t;
use crate::traits::OpenCVType;

impl<T: for<'o> OpenCVType<'o>> IntoIterator for Vector<T>
where
	Vector<T>: VectorExtern<T>,
{
	type Item = T;
	type IntoIter = VectorIterator<T>;

	#[inline]
	fn into_iter(self) -> VectorIterator<T> {
		Self::IntoIter::new(self)
	}
}

impl<'v, T: for<'o> OpenCVType<'o>> IntoIterator for &'v Vector<T>
where
	Vector<T>: VectorExtern<T>,
{
	type Item = T;
	type IntoIter = VectorRefIterator<'v, T>;

	#[inline]
	fn into_iter(self) -> VectorRefIterator<'v, T> {
		self.iter()
	}
}

pub struct VectorIterator<T: for<'o> OpenCVType<'o>>
where
	Vector<T>: VectorExtern<T>,
{
	vec: Vector<T>,
	i: size_t,
	len: size_t,
}

impl<T: for<'o> OpenCVType<'o>> VectorIterator<T>
where
	Vector<T>: VectorExtern<T>,
{
	#[inline]
	pub fn new(vec: Vector<T>) -> Self {
		Self {
			len: vec.len(),
			vec,
			i: 0,
		}
	}
}

impl<T: for<'o> OpenCVType<'o>> Iterator for VectorIterator<T>
where
	Vector<T>: VectorExtern<T>,
{
	type Item = T;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		#![allow(clippy::iter_nth_zero)]
		self.nth(0)
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		let len = self.len - self.i;
		(len, Some(len))
	}

	#[inline]
	fn nth(&mut self, n: usize) -> Option<Self::Item> {
		self.i += n;
		if self.i < self.len {
			let out = Some(unsafe { self.vec.get_unchecked(self.i) });
			self.i += 1;
			out
		} else {
			None
		}
	}
}

impl<T: for<'o> OpenCVType<'o>> ExactSizeIterator for VectorIterator<T> where Vector<T>: VectorExtern<T> {}

impl<T: for<'o> OpenCVType<'o>> FusedIterator for VectorIterator<T> where Vector<T>: VectorExtern<T> {}

pub struct VectorRefIterator<'v, T: for<'o> OpenCVType<'o>>
where
	Vector<T>: VectorExtern<T>,
{
	vec: &'v Vector<T>,
	i: size_t,
	len: size_t,
}

impl<'v, T: for<'o> OpenCVType<'o>> VectorRefIterator<'v, T>
where
	Vector<T>: VectorExtern<T>,
{
	#[inline]
	pub fn new(vec: &'v Vector<T>) -> Self {
		Self {
			len: vec.len(),
			vec,
			i: 0,
		}
	}
}

impl<T: for<'o> OpenCVType<'o>> Iterator for VectorRefIterator<'_, T>
where
	Vector<T>: VectorExtern<T>,
{
	type Item = T;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		#![allow(clippy::iter_nth_zero)]
		self.nth(0)
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		let len = self.len - self.i;
		(len, Some(len))
	}

	#[inline]
	fn nth(&mut self, n: usize) -> Option<Self::Item> {
		self.i += n;
		if self.i < self.len {
			let out = Some(unsafe { self.vec.get_unchecked(self.i) });
			self.i += 1;
			out
		} else {
			None
		}
	}
}

impl<T: for<'o> OpenCVType<'o>> ExactSizeIterator for VectorRefIterator<'_, T> where Vector<T>: VectorExtern<T> {}

impl<T: for<'o> OpenCVType<'o>> FusedIterator for VectorRefIterator<'_, T> where Vector<T>: VectorExtern<T> {}

impl<T: for<'o> OpenCVType<'o>> Clone for VectorRefIterator<'_, T>
where
	Vector<T>: VectorExtern<T>,
{
	fn clone(&self) -> Self {
		Self { ..*self }
	}
}
