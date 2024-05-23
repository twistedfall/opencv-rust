ptr_extern! { crate::stitching::Detail_MultiBandBlender,
	cv_PtrLcv_detail_MultiBandBlenderG_new_null_const, cv_PtrLcv_detail_MultiBandBlenderG_delete, cv_PtrLcv_detail_MultiBandBlenderG_getInnerPtr_const, cv_PtrLcv_detail_MultiBandBlenderG_getInnerPtrMut
}

ptr_extern_ctor! { crate::stitching::Detail_MultiBandBlender, cv_PtrLcv_detail_MultiBandBlenderG_new_const_MultiBandBlender }
impl core::Ptr<crate::stitching::Detail_MultiBandBlender> {
	#[inline] pub fn as_raw_PtrOfDetail_MultiBandBlender(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetail_MultiBandBlender(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::stitching::Detail_MultiBandBlenderTraitConst for core::Ptr<crate::stitching::Detail_MultiBandBlender> {
	#[inline] fn as_raw_Detail_MultiBandBlender(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_MultiBandBlenderTrait for core::Ptr<crate::stitching::Detail_MultiBandBlender> {
	#[inline] fn as_raw_mut_Detail_MultiBandBlender(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::Detail_BlenderTraitConst for core::Ptr<crate::stitching::Detail_MultiBandBlender> {
	#[inline] fn as_raw_Detail_Blender(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_BlenderTrait for core::Ptr<crate::stitching::Detail_MultiBandBlender> {
	#[inline] fn as_raw_mut_Detail_Blender(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::stitching::Detail_MultiBandBlender>, core::Ptr<crate::stitching::Detail_Blender>, cv_PtrLcv_detail_MultiBandBlenderG_to_PtrOfDetail_Blender }

impl std::fmt::Debug for core::Ptr<crate::stitching::Detail_MultiBandBlender> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfDetail_MultiBandBlender")
			.finish()
	}
}

