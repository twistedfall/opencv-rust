ptr_extern! { crate::features2d::MSER,
	cv_PtrLcv_MSERG_new_null_const, cv_PtrLcv_MSERG_delete, cv_PtrLcv_MSERG_getInnerPtr_const, cv_PtrLcv_MSERG_getInnerPtrMut
}

impl core::Ptr<crate::features2d::MSER> {
	#[inline] pub fn as_raw_PtrOfMSER(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfMSER(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::features2d::MSERTraitConst for core::Ptr<crate::features2d::MSER> {
	#[inline] fn as_raw_MSER(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::MSERTrait for core::Ptr<crate::features2d::MSER> {
	#[inline] fn as_raw_mut_MSER(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::features2d::MSER> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::features2d::MSER> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::features2d::MSER>, core::Ptr<core::Algorithm>, cv_PtrLcv_MSERG_to_PtrOfAlgorithm }

impl crate::features2d::Feature2DTraitConst for core::Ptr<crate::features2d::MSER> {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::Feature2DTrait for core::Ptr<crate::features2d::MSER> {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::features2d::MSER>, core::Ptr<crate::features2d::Feature2D>, cv_PtrLcv_MSERG_to_PtrOfFeature2D }

impl std::fmt::Debug for core::Ptr<crate::features2d::MSER> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfMSER")
			.finish()
	}
}

