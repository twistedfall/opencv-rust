use std::iter::FusedIterator;

use crate::{
	core::{Vector, VectorElement, VectorExtern},
	platform_types::size_t,
};

impl<T: VectorElement> IntoIterator for Vector<T>
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

impl<'v, T: VectorElement> IntoIterator for &'v Vector<T>
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

pub struct VectorIterator<T: VectorElement>
where
	Vector<T>: VectorExtern<T>,
{
	vec: Vector<T>,
	i: size_t,
	len: size_t,
}

impl<T: VectorElement> VectorIterator<T>
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

impl<T: VectorElement> Iterator for VectorIterator<T>
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

impl<T: VectorElement> ExactSizeIterator for VectorIterator<T> where Vector<T>: VectorExtern<T> {}

impl<T: VectorElement> FusedIterator for VectorIterator<T> where Vector<T>: VectorExtern<T> {}

pub struct VectorRefIterator<'v, T: VectorElement>
where
	Vector<T>: VectorExtern<T>,
{
	vec: &'v Vector<T>,
	i: size_t,
	len: size_t,
}

impl<'v, T: VectorElement> VectorRefIterator<'v, T>
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

impl<T: VectorElement> Iterator for VectorRefIterator<'_, T>
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

impl<T: VectorElement> ExactSizeIterator for VectorRefIterator<'_, T> where Vector<T>: VectorExtern<T> {}

impl<T: VectorElement> FusedIterator for VectorRefIterator<'_, T> where Vector<T>: VectorExtern<T> {}

impl<T: VectorElement> Clone for VectorRefIterator<'_, T>
where
	Vector<T>: VectorExtern<T>,
{
	fn clone(&self) -> Self {
		Self { ..*self }
	}
}
