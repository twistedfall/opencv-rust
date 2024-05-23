ptr_extern! { crate::rgbd::RgbdNormals,
	cv_PtrLcv_rgbd_RgbdNormalsG_new_null_const, cv_PtrLcv_rgbd_RgbdNormalsG_delete, cv_PtrLcv_rgbd_RgbdNormalsG_getInnerPtr_const, cv_PtrLcv_rgbd_RgbdNormalsG_getInnerPtrMut
}

ptr_extern_ctor! { crate::rgbd::RgbdNormals, cv_PtrLcv_rgbd_RgbdNormalsG_new_const_RgbdNormals }
impl core::Ptr<crate::rgbd::RgbdNormals> {
	#[inline] pub fn as_raw_PtrOfRgbdNormals(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfRgbdNormals(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::rgbd::RgbdNormalsTraitConst for core::Ptr<crate::rgbd::RgbdNormals> {
	#[inline] fn as_raw_RgbdNormals(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::RgbdNormalsTrait for core::Ptr<crate::rgbd::RgbdNormals> {
	#[inline] fn as_raw_mut_RgbdNormals(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::rgbd::RgbdNormals> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::rgbd::RgbdNormals> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::rgbd::RgbdNormals>, core::Ptr<core::Algorithm>, cv_PtrLcv_rgbd_RgbdNormalsG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::rgbd::RgbdNormals> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfRgbdNormals")
			.finish()
	}
}

