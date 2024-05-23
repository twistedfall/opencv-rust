ptr_extern! { crate::videostab::WobbleSuppressorBase,
	cv_PtrLcv_videostab_WobbleSuppressorBaseG_new_null_const, cv_PtrLcv_videostab_WobbleSuppressorBaseG_delete, cv_PtrLcv_videostab_WobbleSuppressorBaseG_getInnerPtr_const, cv_PtrLcv_videostab_WobbleSuppressorBaseG_getInnerPtrMut
}

impl core::Ptr<crate::videostab::WobbleSuppressorBase> {
	#[inline] pub fn as_raw_PtrOfWobbleSuppressorBase(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfWobbleSuppressorBase(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::videostab::WobbleSuppressorBaseTraitConst for core::Ptr<crate::videostab::WobbleSuppressorBase> {
	#[inline] fn as_raw_WobbleSuppressorBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::WobbleSuppressorBaseTrait for core::Ptr<crate::videostab::WobbleSuppressorBase> {
	#[inline] fn as_raw_mut_WobbleSuppressorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::videostab::WobbleSuppressorBase> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfWobbleSuppressorBase")
			.finish()
	}
}

