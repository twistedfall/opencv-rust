ptr_extern! { crate::stitching::Detail_FeatherBlender,
	cv_PtrLcv_detail_FeatherBlenderG_new_null_const, cv_PtrLcv_detail_FeatherBlenderG_delete, cv_PtrLcv_detail_FeatherBlenderG_getInnerPtr_const, cv_PtrLcv_detail_FeatherBlenderG_getInnerPtrMut
}

ptr_extern_ctor! { crate::stitching::Detail_FeatherBlender, cv_PtrLcv_detail_FeatherBlenderG_new_const_FeatherBlender }
impl core::Ptr<crate::stitching::Detail_FeatherBlender> {
	#[inline] pub fn as_raw_PtrOfDetail_FeatherBlender(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetail_FeatherBlender(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::stitching::Detail_FeatherBlenderTraitConst for core::Ptr<crate::stitching::Detail_FeatherBlender> {
	#[inline] fn as_raw_Detail_FeatherBlender(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_FeatherBlenderTrait for core::Ptr<crate::stitching::Detail_FeatherBlender> {
	#[inline] fn as_raw_mut_Detail_FeatherBlender(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::Detail_BlenderTraitConst for core::Ptr<crate::stitching::Detail_FeatherBlender> {
	#[inline] fn as_raw_Detail_Blender(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_BlenderTrait for core::Ptr<crate::stitching::Detail_FeatherBlender> {
	#[inline] fn as_raw_mut_Detail_Blender(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::stitching::Detail_FeatherBlender>, core::Ptr<crate::stitching::Detail_Blender>, cv_PtrLcv_detail_FeatherBlenderG_to_PtrOfDetail_Blender }

impl std::fmt::Debug for core::Ptr<crate::stitching::Detail_FeatherBlender> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfDetail_FeatherBlender")
			.finish()
	}
}

