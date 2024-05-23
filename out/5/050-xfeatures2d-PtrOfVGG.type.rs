ptr_extern! { crate::xfeatures2d::VGG,
	cv_PtrLcv_xfeatures2d_VGGG_new_null_const, cv_PtrLcv_xfeatures2d_VGGG_delete, cv_PtrLcv_xfeatures2d_VGGG_getInnerPtr_const, cv_PtrLcv_xfeatures2d_VGGG_getInnerPtrMut
}

impl core::Ptr<crate::xfeatures2d::VGG> {
	#[inline] pub fn as_raw_PtrOfVGG(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfVGG(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::xfeatures2d::VGGTraitConst for core::Ptr<crate::xfeatures2d::VGG> {
	#[inline] fn as_raw_VGG(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::xfeatures2d::VGGTrait for core::Ptr<crate::xfeatures2d::VGG> {
	#[inline] fn as_raw_mut_VGG(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::xfeatures2d::VGG> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::xfeatures2d::VGG> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::xfeatures2d::VGG>, core::Ptr<core::Algorithm>, cv_PtrLcv_xfeatures2d_VGGG_to_PtrOfAlgorithm }

impl crate::features::Feature2DTraitConst for core::Ptr<crate::xfeatures2d::VGG> {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features::Feature2DTrait for core::Ptr<crate::xfeatures2d::VGG> {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::xfeatures2d::VGG>, core::Ptr<crate::features::Feature2D>, cv_PtrLcv_xfeatures2d_VGGG_to_PtrOfFeature2D }

impl std::fmt::Debug for core::Ptr<crate::xfeatures2d::VGG> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfVGG")
			.finish()
	}
}

