ptr_extern! { crate::features::Feature2D,
	cv_PtrLcv_Feature2DG_new_null_const, cv_PtrLcv_Feature2DG_delete, cv_PtrLcv_Feature2DG_getInnerPtr_const, cv_PtrLcv_Feature2DG_getInnerPtrMut
}

ptr_extern_ctor! { crate::features::Feature2D, cv_PtrLcv_Feature2DG_new_const_Feature2D }
impl core::Ptr<crate::features::Feature2D> {
	#[inline] pub fn as_raw_PtrOfFeature2D(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfFeature2D(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::features::Feature2DTraitConst for core::Ptr<crate::features::Feature2D> {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features::Feature2DTrait for core::Ptr<crate::features::Feature2D> {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::features::Feature2D> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::features::Feature2D> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::features::Feature2D>, core::Ptr<core::Algorithm>, cv_PtrLcv_Feature2DG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::features::Feature2D> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfFeature2D")
			.finish()
	}
}

