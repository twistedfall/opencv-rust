use std::iter::FusedIterator;

use crate::core::{Vector, VectorExtern};
use crate::platform_types::size_t;
use crate::traits::OpenCVFromExtern;

impl<T: OpenCVFromExtern> IntoIterator for Vector<T>
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

impl<'v, T: OpenCVFromExtern> IntoIterator for &'v Vector<T>
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

pub struct VectorIterator<T>
where
	Vector<T>: VectorExtern<T>,
{
	vec: Vector<T>,
	start: size_t,
	end: size_t,
}

impl<T> VectorIterator<T>
where
	Vector<T>: VectorExtern<T>,
{
	#[inline]
	pub fn new(vec: Vector<T>) -> Self {
		Self {
			end: vec.len(),
			vec,
			start: 0,
		}
	}
}

impl<T: OpenCVFromExtern> Iterator for VectorIterator<T>
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
		let len = self.len();
		(len, Some(len))
	}

	#[inline]
	fn nth(&mut self, n: usize) -> Option<Self::Item> {
		self.start = self.start.saturating_add(n);
		if self.start < self.end {
			let out = Some(unsafe { self.vec.get_unchecked(self.start) });
			self.start += 1;
			out
		} else {
			None
		}
	}
}

impl<T: OpenCVFromExtern> DoubleEndedIterator for VectorIterator<T>
where
	Vector<T>: VectorExtern<T>,
{
	#[inline]
	fn next_back(&mut self) -> Option<Self::Item> {
		self.nth_back(0)
	}

	#[inline]
	fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
		self.end = self.end.saturating_sub(n);
		if self.start < self.end {
			let out = Some(unsafe { self.vec.get_unchecked(self.end - 1) });
			self.end -= 1;
			out
		} else {
			None
		}
	}
}

impl<T: OpenCVFromExtern> ExactSizeIterator for VectorIterator<T>
where
	Vector<T>: VectorExtern<T>,
{
	#[inline]
	fn len(&self) -> usize {
		self.end - self.start
	}
}

impl<T: OpenCVFromExtern> FusedIterator for VectorIterator<T> where Vector<T>: VectorExtern<T> {}

pub struct VectorRefIterator<'v, T>
where
	Vector<T>: VectorExtern<T>,
{
	vec: &'v Vector<T>,
	start: size_t,
	end: size_t,
}

impl<'v, T> VectorRefIterator<'v, T>
where
	Vector<T>: VectorExtern<T>,
{
	#[inline]
	pub fn new(vec: &'v Vector<T>) -> Self {
		Self {
			end: vec.len(),
			vec,
			start: 0,
		}
	}
}

impl<T: OpenCVFromExtern> Iterator for VectorRefIterator<'_, T>
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
		let len = self.len();
		(len, Some(len))
	}

	#[inline]
	fn nth(&mut self, n: usize) -> Option<Self::Item> {
		self.start += n;
		if self.start < self.end {
			let out = Some(unsafe { self.vec.get_unchecked(self.start) });
			self.start += 1;
			out
		} else {
			None
		}
	}
}

impl<T: OpenCVFromExtern> DoubleEndedIterator for VectorRefIterator<'_, T>
where
	Vector<T>: VectorExtern<T>,
{
	#[inline]
	fn next_back(&mut self) -> Option<Self::Item> {
		self.nth_back(0)
	}

	#[inline]
	fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
		self.end -= n;
		if self.start < self.end {
			let out = Some(unsafe { self.vec.get_unchecked(self.end - 1) });
			self.end -= 1;
			out
		} else {
			None
		}
	}
}

impl<T: OpenCVFromExtern> ExactSizeIterator for VectorRefIterator<'_, T>
where
	Vector<T>: VectorExtern<T>,
{
	#[inline]
	fn len(&self) -> usize {
		self.end - self.start
	}
}

impl<T: OpenCVFromExtern> FusedIterator for VectorRefIterator<'_, T> where Vector<T>: VectorExtern<T> {}

impl<T> Clone for VectorRefIterator<'_, T>
where
	Vector<T>: VectorExtern<T>,
{
	#[inline]
	fn clone(&self) -> Self {
		Self { ..*self }
	}
}
