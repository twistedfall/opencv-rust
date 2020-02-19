pub struct {{rust_local}} {
	pub(crate) ptr: {{rust_extern}}
}

impl {{rust_local}} {
	pub fn as_raw_{{rust_local}}(&self) -> {{rust_extern}} { self.ptr }

	#[inline]
	pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
		crate::templ::VectorRefIterator::new(self)
	}
	{{inherent_methods}}
}

impl Drop for {{rust_local}} {
	#[inline]
	fn drop(&mut self) {
		extern "C" { fn cv_{{rust_local}}_delete(instance: {{rust_extern}}); }
		unsafe { cv_{{rust_local}}_delete(self.as_raw_{{rust_local}}()) };
	}
}

impl IntoIterator for {{rust_local}} {
	type Item = {{inner_rust_full}};
	type IntoIter = crate::templ::VectorIterator<Self>;

	#[inline]
	fn into_iter(self) -> Self::IntoIter {
		Self::IntoIter::new(self)
	}
}

impl<'i> IntoIterator for &'i {{rust_local}} {
	type Item = {{inner_rust_full}};
	type IntoIter = crate::templ::VectorRefIterator<'i, {{rust_local}}>;

	#[inline]
	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

impl<'i> crate::templ::Vector<'i> for {{rust_local}} {
	type Storage = {{inner_rust_full}};

	#[inline]
	fn new() -> Self {
		extern "C" { fn cv_{{rust_local}}_new() -> {{rust_extern}}; }
		Self { ptr: unsafe { cv_{{rust_local}}_new() } }
	}

	#[inline]
	fn len(&self) -> size_t {
		extern "C" { fn cv_{{rust_local}}_len(instance: {{rust_extern}}) -> size_t; }
		unsafe { cv_{{rust_local}}_len(self.as_raw_{{rust_local}}()) }
	}

	#[inline]
	fn is_empty(&self) -> bool {
		extern "C" { fn cv_{{rust_local}}_is_empty(instance: {{rust_extern}}) -> bool; }
		unsafe { cv_{{rust_local}}_is_empty(self.as_raw_{{rust_local}}()) }
	}

	#[inline]
	fn capacity(&self) -> size_t {
		extern "C" { fn cv_{{rust_local}}_capacity(instance: {{rust_extern}}) -> size_t; }
		unsafe { cv_{{rust_local}}_capacity(self.as_raw_{{rust_local}}()) }
	}

	#[inline]
	fn shrink_to_fit(&mut self) {
		extern "C" { fn cv_{{rust_local}}_shrink_to_fit(instance: {{rust_extern}}); }
		unsafe { cv_{{rust_local}}_shrink_to_fit(self.as_raw_{{rust_local}}()) }
	}

	#[inline]
	fn reserve(&mut self, additional: size_t) {
		extern "C" { fn cv_{{rust_local}}_reserve(instance: {{rust_extern}}, additional: size_t); }
		unsafe { cv_{{rust_local}}_reserve(self.as_raw_{{rust_local}}(), additional) }
	}

	#[inline]
	fn remove(&mut self, index: size_t) -> Result<()> {
		crate::templ::vector_index_check(index, self.len())?;
		extern "C" { fn cv_{{rust_local}}_remove(instance: {{rust_extern}}, index: size_t); }
		unsafe { cv_{{rust_local}}_remove(self.as_raw_{{rust_local}}(), index) };
		Ok(())
	}

	#[inline]
	fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
		let len = self.len();
		crate::templ::vector_index_check(index1, len)?;
		crate::templ::vector_index_check(index2, len)?;
		if index1 != index2 {
			extern "C" { fn cv_{{rust_local}}_swap(instance: {{rust_extern}}, index1: size_t, index2: size_t); }
			unsafe { cv_{{rust_local}}_swap(self.as_raw_{{rust_local}}(), index1, index2) };
		}
		Ok(())
	}

	#[inline]
	fn clear(&mut self) {
		extern "C" { fn cv_{{rust_local}}_clear(instance: {{rust_extern}}); }
		unsafe { cv_{{rust_local}}_clear(self.as_raw_{{rust_local}}()) }
	}

	{{vector_methods}}
}

unsafe impl Send for {{rust_local}} {}
{{impls}}


