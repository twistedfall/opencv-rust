ptr_extern! { crate::videostab::MoreAccurateMotionWobbleSuppressor,
	cv_PtrLcv_videostab_MoreAccurateMotionWobbleSuppressorG_new_null_const, cv_PtrLcv_videostab_MoreAccurateMotionWobbleSuppressorG_delete, cv_PtrLcv_videostab_MoreAccurateMotionWobbleSuppressorG_getInnerPtr_const, cv_PtrLcv_videostab_MoreAccurateMotionWobbleSuppressorG_getInnerPtrMut
}

ptr_extern_ctor! { crate::videostab::MoreAccurateMotionWobbleSuppressor, cv_PtrLcv_videostab_MoreAccurateMotionWobbleSuppressorG_new_const_MoreAccurateMotionWobbleSuppressor }
impl core::Ptr<crate::videostab::MoreAccurateMotionWobbleSuppressor> {
	#[inline] pub fn as_raw_PtrOfMoreAccurateMotionWobbleSuppressor(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfMoreAccurateMotionWobbleSuppressor(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::videostab::MoreAccurateMotionWobbleSuppressorTraitConst for core::Ptr<crate::videostab::MoreAccurateMotionWobbleSuppressor> {
	#[inline] fn as_raw_MoreAccurateMotionWobbleSuppressor(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::MoreAccurateMotionWobbleSuppressorTrait for core::Ptr<crate::videostab::MoreAccurateMotionWobbleSuppressor> {
	#[inline] fn as_raw_mut_MoreAccurateMotionWobbleSuppressor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::MoreAccurateMotionWobbleSuppressorBaseTraitConst for core::Ptr<crate::videostab::MoreAccurateMotionWobbleSuppressor> {
	#[inline] fn as_raw_MoreAccurateMotionWobbleSuppressorBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::MoreAccurateMotionWobbleSuppressorBaseTrait for core::Ptr<crate::videostab::MoreAccurateMotionWobbleSuppressor> {
	#[inline] fn as_raw_mut_MoreAccurateMotionWobbleSuppressorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::videostab::MoreAccurateMotionWobbleSuppressor>, core::Ptr<crate::videostab::MoreAccurateMotionWobbleSuppressorBase>, cv_PtrLcv_videostab_MoreAccurateMotionWobbleSuppressorG_to_PtrOfMoreAccurateMotionWobbleSuppressorBase }

impl crate::videostab::WobbleSuppressorBaseTraitConst for core::Ptr<crate::videostab::MoreAccurateMotionWobbleSuppressor> {
	#[inline] fn as_raw_WobbleSuppressorBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::WobbleSuppressorBaseTrait for core::Ptr<crate::videostab::MoreAccurateMotionWobbleSuppressor> {
	#[inline] fn as_raw_mut_WobbleSuppressorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::videostab::MoreAccurateMotionWobbleSuppressor>, core::Ptr<crate::videostab::WobbleSuppressorBase>, cv_PtrLcv_videostab_MoreAccurateMotionWobbleSuppressorG_to_PtrOfWobbleSuppressorBase }

impl std::fmt::Debug for core::Ptr<crate::videostab::MoreAccurateMotionWobbleSuppressor> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfMoreAccurateMotionWobbleSuppressor")
			.finish()
	}
}

