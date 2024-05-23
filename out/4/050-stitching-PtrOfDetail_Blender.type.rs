ptr_extern! { crate::stitching::Detail_Blender,
	cv_PtrLcv_detail_BlenderG_new_null_const, cv_PtrLcv_detail_BlenderG_delete, cv_PtrLcv_detail_BlenderG_getInnerPtr_const, cv_PtrLcv_detail_BlenderG_getInnerPtrMut
}

ptr_extern_ctor! { crate::stitching::Detail_Blender, cv_PtrLcv_detail_BlenderG_new_const_Blender }
impl core::Ptr<crate::stitching::Detail_Blender> {
	#[inline] pub fn as_raw_PtrOfDetail_Blender(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetail_Blender(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::stitching::Detail_BlenderTraitConst for core::Ptr<crate::stitching::Detail_Blender> {
	#[inline] fn as_raw_Detail_Blender(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_BlenderTrait for core::Ptr<crate::stitching::Detail_Blender> {
	#[inline] fn as_raw_mut_Detail_Blender(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::stitching::Detail_Blender> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfDetail_Blender")
			.finish()
	}
}

