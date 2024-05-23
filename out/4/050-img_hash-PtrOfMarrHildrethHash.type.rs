ptr_extern! { crate::img_hash::MarrHildrethHash,
	cv_PtrLcv_img_hash_MarrHildrethHashG_new_null_const, cv_PtrLcv_img_hash_MarrHildrethHashG_delete, cv_PtrLcv_img_hash_MarrHildrethHashG_getInnerPtr_const, cv_PtrLcv_img_hash_MarrHildrethHashG_getInnerPtrMut
}

ptr_extern_ctor! { crate::img_hash::MarrHildrethHash, cv_PtrLcv_img_hash_MarrHildrethHashG_new_const_MarrHildrethHash }
impl core::Ptr<crate::img_hash::MarrHildrethHash> {
	#[inline] pub fn as_raw_PtrOfMarrHildrethHash(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfMarrHildrethHash(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::img_hash::MarrHildrethHashTraitConst for core::Ptr<crate::img_hash::MarrHildrethHash> {
	#[inline] fn as_raw_MarrHildrethHash(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::img_hash::MarrHildrethHashTrait for core::Ptr<crate::img_hash::MarrHildrethHash> {
	#[inline] fn as_raw_mut_MarrHildrethHash(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::img_hash::MarrHildrethHash> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::img_hash::MarrHildrethHash> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::img_hash::MarrHildrethHash>, core::Ptr<core::Algorithm>, cv_PtrLcv_img_hash_MarrHildrethHashG_to_PtrOfAlgorithm }

impl crate::img_hash::ImgHashBaseTraitConst for core::Ptr<crate::img_hash::MarrHildrethHash> {
	#[inline] fn as_raw_ImgHashBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::img_hash::ImgHashBaseTrait for core::Ptr<crate::img_hash::MarrHildrethHash> {
	#[inline] fn as_raw_mut_ImgHashBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::img_hash::MarrHildrethHash>, core::Ptr<crate::img_hash::ImgHashBase>, cv_PtrLcv_img_hash_MarrHildrethHashG_to_PtrOfImgHashBase }

impl std::fmt::Debug for core::Ptr<crate::img_hash::MarrHildrethHash> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfMarrHildrethHash")
			.finish()
	}
}

