ptr_extern! { crate::stitching::PlaneWarperGpu,
	cv_PtrLcv_PlaneWarperGpuG_new_null_const, cv_PtrLcv_PlaneWarperGpuG_delete, cv_PtrLcv_PlaneWarperGpuG_getInnerPtr_const, cv_PtrLcv_PlaneWarperGpuG_getInnerPtrMut
}

ptr_extern_ctor! { crate::stitching::PlaneWarperGpu, cv_PtrLcv_PlaneWarperGpuG_new_const_PlaneWarperGpu }
impl core::Ptr<crate::stitching::PlaneWarperGpu> {
	#[inline] pub fn as_raw_PtrOfPlaneWarperGpu(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfPlaneWarperGpu(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::stitching::PlaneWarperGpuTraitConst for core::Ptr<crate::stitching::PlaneWarperGpu> {
	#[inline] fn as_raw_PlaneWarperGpu(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::PlaneWarperGpuTrait for core::Ptr<crate::stitching::PlaneWarperGpu> {
	#[inline] fn as_raw_mut_PlaneWarperGpu(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::WarperCreatorTraitConst for core::Ptr<crate::stitching::PlaneWarperGpu> {
	#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::WarperCreatorTrait for core::Ptr<crate::stitching::PlaneWarperGpu> {
	#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::stitching::PlaneWarperGpu>, core::Ptr<crate::stitching::WarperCreator>, cv_PtrLcv_PlaneWarperGpuG_to_PtrOfWarperCreator }

impl std::fmt::Debug for core::Ptr<crate::stitching::PlaneWarperGpu> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfPlaneWarperGpu")
			.finish()
	}
}

