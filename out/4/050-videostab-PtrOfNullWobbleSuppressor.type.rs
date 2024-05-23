ptr_extern! { crate::videostab::NullWobbleSuppressor,
	cv_PtrLcv_videostab_NullWobbleSuppressorG_new_null_const, cv_PtrLcv_videostab_NullWobbleSuppressorG_delete, cv_PtrLcv_videostab_NullWobbleSuppressorG_getInnerPtr_const, cv_PtrLcv_videostab_NullWobbleSuppressorG_getInnerPtrMut
}

ptr_extern_ctor! { crate::videostab::NullWobbleSuppressor, cv_PtrLcv_videostab_NullWobbleSuppressorG_new_const_NullWobbleSuppressor }
impl core::Ptr<crate::videostab::NullWobbleSuppressor> {
	#[inline] pub fn as_raw_PtrOfNullWobbleSuppressor(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfNullWobbleSuppressor(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::videostab::NullWobbleSuppressorTraitConst for core::Ptr<crate::videostab::NullWobbleSuppressor> {
	#[inline] fn as_raw_NullWobbleSuppressor(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::NullWobbleSuppressorTrait for core::Ptr<crate::videostab::NullWobbleSuppressor> {
	#[inline] fn as_raw_mut_NullWobbleSuppressor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::WobbleSuppressorBaseTraitConst for core::Ptr<crate::videostab::NullWobbleSuppressor> {
	#[inline] fn as_raw_WobbleSuppressorBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::WobbleSuppressorBaseTrait for core::Ptr<crate::videostab::NullWobbleSuppressor> {
	#[inline] fn as_raw_mut_WobbleSuppressorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::videostab::NullWobbleSuppressor>, core::Ptr<crate::videostab::WobbleSuppressorBase>, cv_PtrLcv_videostab_NullWobbleSuppressorG_to_PtrOfWobbleSuppressorBase }

impl std::fmt::Debug for core::Ptr<crate::videostab::NullWobbleSuppressor> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfNullWobbleSuppressor")
			.finish()
	}
}

