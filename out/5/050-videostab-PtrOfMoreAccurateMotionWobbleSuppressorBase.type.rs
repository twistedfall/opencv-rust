ptr_extern! { crate::videostab::MoreAccurateMotionWobbleSuppressorBase,
	cv_PtrLcv_videostab_MoreAccurateMotionWobbleSuppressorBaseG_new_null_const, cv_PtrLcv_videostab_MoreAccurateMotionWobbleSuppressorBaseG_delete, cv_PtrLcv_videostab_MoreAccurateMotionWobbleSuppressorBaseG_getInnerPtr_const, cv_PtrLcv_videostab_MoreAccurateMotionWobbleSuppressorBaseG_getInnerPtrMut
}

impl core::Ptr<crate::videostab::MoreAccurateMotionWobbleSuppressorBase> {
	#[inline] pub fn as_raw_PtrOfMoreAccurateMotionWobbleSuppressorBase(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfMoreAccurateMotionWobbleSuppressorBase(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::videostab::MoreAccurateMotionWobbleSuppressorBaseTraitConst for core::Ptr<crate::videostab::MoreAccurateMotionWobbleSuppressorBase> {
	#[inline] fn as_raw_MoreAccurateMotionWobbleSuppressorBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::MoreAccurateMotionWobbleSuppressorBaseTrait for core::Ptr<crate::videostab::MoreAccurateMotionWobbleSuppressorBase> {
	#[inline] fn as_raw_mut_MoreAccurateMotionWobbleSuppressorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::WobbleSuppressorBaseTraitConst for core::Ptr<crate::videostab::MoreAccurateMotionWobbleSuppressorBase> {
	#[inline] fn as_raw_WobbleSuppressorBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::WobbleSuppressorBaseTrait for core::Ptr<crate::videostab::MoreAccurateMotionWobbleSuppressorBase> {
	#[inline] fn as_raw_mut_WobbleSuppressorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::videostab::MoreAccurateMotionWobbleSuppressorBase>, core::Ptr<crate::videostab::WobbleSuppressorBase>, cv_PtrLcv_videostab_MoreAccurateMotionWobbleSuppressorBaseG_to_PtrOfWobbleSuppressorBase }

impl std::fmt::Debug for core::Ptr<crate::videostab::MoreAccurateMotionWobbleSuppressorBase> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfMoreAccurateMotionWobbleSuppressorBase")
			.finish()
	}
}

