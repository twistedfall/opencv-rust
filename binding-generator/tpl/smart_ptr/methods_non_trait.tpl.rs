

/// Get reference to the inner object
pub fn get(&self) -> crate::smart_ptr::PtrRef<{{inner_rust_full}}> {
	let inner = {{inner_rust_full}} { ptr: self.get_inner_ptr() };
	crate::smart_ptr::PtrRef {
		inner: std::mem::ManuallyDrop::new(inner),
		owner: std::marker::PhantomData,
	}
}

/// Get mutable reference to the inner object
pub fn get_mut(&mut self) -> crate::smart_ptr::PtrRefMut<{{inner_rust_full}}> {
	let inner = {{inner_rust_full}} { ptr: self.get_inner_ptr() };
	crate::smart_ptr::PtrRefMut {
		inner: std::mem::ManuallyDrop::new(inner),
		owner: std::marker::PhantomData,
	}
}

