ptr_extern! { crate::videostab::MoreAccurateMotionWobbleSuppressorGpu,
	cv_PtrLcv_videostab_MoreAccurateMotionWobbleSuppressorGpuG_new_null_const, cv_PtrLcv_videostab_MoreAccurateMotionWobbleSuppressorGpuG_delete, cv_PtrLcv_videostab_MoreAccurateMotionWobbleSuppressorGpuG_getInnerPtr_const, cv_PtrLcv_videostab_MoreAccurateMotionWobbleSuppressorGpuG_getInnerPtrMut
}

ptr_extern_ctor! { crate::videostab::MoreAccurateMotionWobbleSuppressorGpu, cv_PtrLcv_videostab_MoreAccurateMotionWobbleSuppressorGpuG_new_const_MoreAccurateMotionWobbleSuppressorGpu }
impl core::Ptr<crate::videostab::MoreAccurateMotionWobbleSuppressorGpu> {
	#[inline] pub fn as_raw_PtrOfMoreAccurateMotionWobbleSuppressorGpu(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfMoreAccurateMotionWobbleSuppressorGpu(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::videostab::MoreAccurateMotionWobbleSuppressorGpuTraitConst for core::Ptr<crate::videostab::MoreAccurateMotionWobbleSuppressorGpu> {
	#[inline] fn as_raw_MoreAccurateMotionWobbleSuppressorGpu(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::MoreAccurateMotionWobbleSuppressorGpuTrait for core::Ptr<crate::videostab::MoreAccurateMotionWobbleSuppressorGpu> {
	#[inline] fn as_raw_mut_MoreAccurateMotionWobbleSuppressorGpu(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::MoreAccurateMotionWobbleSuppressorBaseTraitConst for core::Ptr<crate::videostab::MoreAccurateMotionWobbleSuppressorGpu> {
	#[inline] fn as_raw_MoreAccurateMotionWobbleSuppressorBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::MoreAccurateMotionWobbleSuppressorBaseTrait for core::Ptr<crate::videostab::MoreAccurateMotionWobbleSuppressorGpu> {
	#[inline] fn as_raw_mut_MoreAccurateMotionWobbleSuppressorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::videostab::MoreAccurateMotionWobbleSuppressorGpu>, core::Ptr<crate::videostab::MoreAccurateMotionWobbleSuppressorBase>, cv_PtrLcv_videostab_MoreAccurateMotionWobbleSuppressorGpuG_to_PtrOfMoreAccurateMotionWobbleSuppressorBase }

impl crate::videostab::WobbleSuppressorBaseTraitConst for core::Ptr<crate::videostab::MoreAccurateMotionWobbleSuppressorGpu> {
	#[inline] fn as_raw_WobbleSuppressorBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::WobbleSuppressorBaseTrait for core::Ptr<crate::videostab::MoreAccurateMotionWobbleSuppressorGpu> {
	#[inline] fn as_raw_mut_WobbleSuppressorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::videostab::MoreAccurateMotionWobbleSuppressorGpu>, core::Ptr<crate::videostab::WobbleSuppressorBase>, cv_PtrLcv_videostab_MoreAccurateMotionWobbleSuppressorGpuG_to_PtrOfWobbleSuppressorBase }

impl std::fmt::Debug for core::Ptr<crate::videostab::MoreAccurateMotionWobbleSuppressorGpu> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfMoreAccurateMotionWobbleSuppressorGpu")
			.finish()
	}
}

