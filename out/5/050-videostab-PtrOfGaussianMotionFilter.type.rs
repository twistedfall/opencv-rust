ptr_extern! { crate::videostab::GaussianMotionFilter,
	cv_PtrLcv_videostab_GaussianMotionFilterG_new_null_const, cv_PtrLcv_videostab_GaussianMotionFilterG_delete, cv_PtrLcv_videostab_GaussianMotionFilterG_getInnerPtr_const, cv_PtrLcv_videostab_GaussianMotionFilterG_getInnerPtrMut
}

ptr_extern_ctor! { crate::videostab::GaussianMotionFilter, cv_PtrLcv_videostab_GaussianMotionFilterG_new_const_GaussianMotionFilter }
impl core::Ptr<crate::videostab::GaussianMotionFilter> {
	#[inline] pub fn as_raw_PtrOfGaussianMotionFilter(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfGaussianMotionFilter(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::videostab::GaussianMotionFilterTraitConst for core::Ptr<crate::videostab::GaussianMotionFilter> {
	#[inline] fn as_raw_GaussianMotionFilter(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::GaussianMotionFilterTrait for core::Ptr<crate::videostab::GaussianMotionFilter> {
	#[inline] fn as_raw_mut_GaussianMotionFilter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::IMotionStabilizerTraitConst for core::Ptr<crate::videostab::GaussianMotionFilter> {
	#[inline] fn as_raw_IMotionStabilizer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::IMotionStabilizerTrait for core::Ptr<crate::videostab::GaussianMotionFilter> {
	#[inline] fn as_raw_mut_IMotionStabilizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::videostab::GaussianMotionFilter>, core::Ptr<crate::videostab::IMotionStabilizer>, cv_PtrLcv_videostab_GaussianMotionFilterG_to_PtrOfIMotionStabilizer }

impl crate::videostab::MotionFilterBaseTraitConst for core::Ptr<crate::videostab::GaussianMotionFilter> {
	#[inline] fn as_raw_MotionFilterBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::MotionFilterBaseTrait for core::Ptr<crate::videostab::GaussianMotionFilter> {
	#[inline] fn as_raw_mut_MotionFilterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::videostab::GaussianMotionFilter>, core::Ptr<crate::videostab::MotionFilterBase>, cv_PtrLcv_videostab_GaussianMotionFilterG_to_PtrOfMotionFilterBase }

impl std::fmt::Debug for core::Ptr<crate::videostab::GaussianMotionFilter> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfGaussianMotionFilter")
			.finish()
	}
}

