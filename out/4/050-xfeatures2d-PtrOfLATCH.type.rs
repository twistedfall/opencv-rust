ptr_extern! { crate::xfeatures2d::LATCH,
	cv_PtrLcv_xfeatures2d_LATCHG_new_null_const, cv_PtrLcv_xfeatures2d_LATCHG_delete, cv_PtrLcv_xfeatures2d_LATCHG_getInnerPtr_const, cv_PtrLcv_xfeatures2d_LATCHG_getInnerPtrMut
}

impl core::Ptr<crate::xfeatures2d::LATCH> {
	#[inline] pub fn as_raw_PtrOfLATCH(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfLATCH(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::xfeatures2d::LATCHTraitConst for core::Ptr<crate::xfeatures2d::LATCH> {
	#[inline] fn as_raw_LATCH(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::xfeatures2d::LATCHTrait for core::Ptr<crate::xfeatures2d::LATCH> {
	#[inline] fn as_raw_mut_LATCH(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::xfeatures2d::LATCH> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::xfeatures2d::LATCH> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::xfeatures2d::LATCH>, core::Ptr<core::Algorithm>, cv_PtrLcv_xfeatures2d_LATCHG_to_PtrOfAlgorithm }

impl crate::features2d::Feature2DTraitConst for core::Ptr<crate::xfeatures2d::LATCH> {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::Feature2DTrait for core::Ptr<crate::xfeatures2d::LATCH> {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::xfeatures2d::LATCH>, core::Ptr<crate::features2d::Feature2D>, cv_PtrLcv_xfeatures2d_LATCHG_to_PtrOfFeature2D }

impl std::fmt::Debug for core::Ptr<crate::xfeatures2d::LATCH> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfLATCH")
			.finish()
	}
}

