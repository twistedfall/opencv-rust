ptr_extern! { crate::img_hash::PHash,
	cv_PtrLcv_img_hash_PHashG_new_null_const, cv_PtrLcv_img_hash_PHashG_delete, cv_PtrLcv_img_hash_PHashG_getInnerPtr_const, cv_PtrLcv_img_hash_PHashG_getInnerPtrMut
}

ptr_extern_ctor! { crate::img_hash::PHash, cv_PtrLcv_img_hash_PHashG_new_const_PHash }
impl core::Ptr<crate::img_hash::PHash> {
	#[inline] pub fn as_raw_PtrOfPHash(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfPHash(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::img_hash::PHashTraitConst for core::Ptr<crate::img_hash::PHash> {
	#[inline] fn as_raw_PHash(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::img_hash::PHashTrait for core::Ptr<crate::img_hash::PHash> {
	#[inline] fn as_raw_mut_PHash(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::img_hash::PHash> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::img_hash::PHash> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::img_hash::PHash>, core::Ptr<core::Algorithm>, cv_PtrLcv_img_hash_PHashG_to_PtrOfAlgorithm }

impl crate::img_hash::ImgHashBaseTraitConst for core::Ptr<crate::img_hash::PHash> {
	#[inline] fn as_raw_ImgHashBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::img_hash::ImgHashBaseTrait for core::Ptr<crate::img_hash::PHash> {
	#[inline] fn as_raw_mut_ImgHashBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::img_hash::PHash>, core::Ptr<crate::img_hash::ImgHashBase>, cv_PtrLcv_img_hash_PHashG_to_PtrOfImgHashBase }

impl std::fmt::Debug for core::Ptr<crate::img_hash::PHash> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfPHash")
			.finish()
	}
}

