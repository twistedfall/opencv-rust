use std::iter::FusedIterator;

use libc::size_t;

use crate::core::{Vector, VectorElement, VectorExtern};

pub struct VectorIterator<T: VectorElement> where Vector<T>: VectorExtern<T> {
	vec: Vector<T>,
	i: size_t,
}

impl<T: VectorElement> VectorIterator<T> where Vector<T>: VectorExtern<T> {
	pub fn new(vec: Vector<T>) -> Self {
		Self { vec, i: 0 }
	}
}

impl<T: VectorElement> Iterator for VectorIterator<T> where Vector<T>: VectorExtern<T> {
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		let out = self.vec.get(self.i);
		self.i += 1;
		out.ok()
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		let len = self.vec.len();
		(len, Some(len))
	}

	fn nth(&mut self, n: usize) -> Option<Self::Item> {
		self.vec.get(n).ok()
	}
}

impl<T: VectorElement> ExactSizeIterator for VectorIterator<T> where Vector<T>: VectorExtern<T> {}

impl<T: VectorElement> FusedIterator for VectorIterator<T> where Vector<T>: VectorExtern<T> {}

pub struct VectorRefIterator<'v, T: VectorElement> where Vector<T>: VectorExtern<T> {
	vec: &'v Vector<T>,
	i: size_t,
}

impl<'v, T: VectorElement> VectorRefIterator<'v, T> where Vector<T>: VectorExtern<T> {
	pub fn new(vec: &'v Vector<T>) -> Self {
		Self { vec, i: 0 }
	}
}

impl<T: VectorElement> Iterator for VectorRefIterator<'_, T> where Vector<T>: VectorExtern<T> {
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		let out = self.vec.get(self.i);
		self.i += 1;
		out.ok()
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		let len = self.vec.len();
		(len, Some(len))
	}

	fn nth(&mut self, n: usize) -> Option<Self::Item> {
		self.vec.get(n).ok()
	}
}

impl<T: VectorElement> ExactSizeIterator for VectorRefIterator<'_, T> where Vector<T>: VectorExtern<T> {}

impl<T: VectorElement> FusedIterator for VectorRefIterator<'_, T> where Vector<T>: VectorExtern<T> {}
