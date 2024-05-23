ptr_extern! { crate::img_hash::RadialVarianceHash,
	cv_PtrLcv_img_hash_RadialVarianceHashG_new_null_const, cv_PtrLcv_img_hash_RadialVarianceHashG_delete, cv_PtrLcv_img_hash_RadialVarianceHashG_getInnerPtr_const, cv_PtrLcv_img_hash_RadialVarianceHashG_getInnerPtrMut
}

ptr_extern_ctor! { crate::img_hash::RadialVarianceHash, cv_PtrLcv_img_hash_RadialVarianceHashG_new_const_RadialVarianceHash }
impl core::Ptr<crate::img_hash::RadialVarianceHash> {
	#[inline] pub fn as_raw_PtrOfRadialVarianceHash(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfRadialVarianceHash(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::img_hash::RadialVarianceHashTraitConst for core::Ptr<crate::img_hash::RadialVarianceHash> {
	#[inline] fn as_raw_RadialVarianceHash(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::img_hash::RadialVarianceHashTrait for core::Ptr<crate::img_hash::RadialVarianceHash> {
	#[inline] fn as_raw_mut_RadialVarianceHash(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::img_hash::RadialVarianceHash> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::img_hash::RadialVarianceHash> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::img_hash::RadialVarianceHash>, core::Ptr<core::Algorithm>, cv_PtrLcv_img_hash_RadialVarianceHashG_to_PtrOfAlgorithm }

impl crate::img_hash::ImgHashBaseTraitConst for core::Ptr<crate::img_hash::RadialVarianceHash> {
	#[inline] fn as_raw_ImgHashBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::img_hash::ImgHashBaseTrait for core::Ptr<crate::img_hash::RadialVarianceHash> {
	#[inline] fn as_raw_mut_ImgHashBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::img_hash::RadialVarianceHash>, core::Ptr<crate::img_hash::ImgHashBase>, cv_PtrLcv_img_hash_RadialVarianceHashG_to_PtrOfImgHashBase }

impl std::fmt::Debug for core::Ptr<crate::img_hash::RadialVarianceHash> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfRadialVarianceHash")
			.finish()
	}
}

