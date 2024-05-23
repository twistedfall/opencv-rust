ptr_extern! { crate::img_hash::ColorMomentHash,
	cv_PtrLcv_img_hash_ColorMomentHashG_new_null_const, cv_PtrLcv_img_hash_ColorMomentHashG_delete, cv_PtrLcv_img_hash_ColorMomentHashG_getInnerPtr_const, cv_PtrLcv_img_hash_ColorMomentHashG_getInnerPtrMut
}

ptr_extern_ctor! { crate::img_hash::ColorMomentHash, cv_PtrLcv_img_hash_ColorMomentHashG_new_const_ColorMomentHash }
impl core::Ptr<crate::img_hash::ColorMomentHash> {
	#[inline] pub fn as_raw_PtrOfColorMomentHash(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfColorMomentHash(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::img_hash::ColorMomentHashTraitConst for core::Ptr<crate::img_hash::ColorMomentHash> {
	#[inline] fn as_raw_ColorMomentHash(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::img_hash::ColorMomentHashTrait for core::Ptr<crate::img_hash::ColorMomentHash> {
	#[inline] fn as_raw_mut_ColorMomentHash(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::img_hash::ColorMomentHash> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::img_hash::ColorMomentHash> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::img_hash::ColorMomentHash>, core::Ptr<core::Algorithm>, cv_PtrLcv_img_hash_ColorMomentHashG_to_PtrOfAlgorithm }

impl crate::img_hash::ImgHashBaseTraitConst for core::Ptr<crate::img_hash::ColorMomentHash> {
	#[inline] fn as_raw_ImgHashBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::img_hash::ImgHashBaseTrait for core::Ptr<crate::img_hash::ColorMomentHash> {
	#[inline] fn as_raw_mut_ImgHashBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::img_hash::ColorMomentHash>, core::Ptr<crate::img_hash::ImgHashBase>, cv_PtrLcv_img_hash_ColorMomentHashG_to_PtrOfImgHashBase }

impl std::fmt::Debug for core::Ptr<crate::img_hash::ColorMomentHash> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfColorMomentHash")
			.finish()
	}
}

