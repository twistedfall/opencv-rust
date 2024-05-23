ptr_extern! { crate::stitching::SphericalWarperGpu,
	cv_PtrLcv_SphericalWarperGpuG_new_null_const, cv_PtrLcv_SphericalWarperGpuG_delete, cv_PtrLcv_SphericalWarperGpuG_getInnerPtr_const, cv_PtrLcv_SphericalWarperGpuG_getInnerPtrMut
}

ptr_extern_ctor! { crate::stitching::SphericalWarperGpu, cv_PtrLcv_SphericalWarperGpuG_new_const_SphericalWarperGpu }
impl core::Ptr<crate::stitching::SphericalWarperGpu> {
	#[inline] pub fn as_raw_PtrOfSphericalWarperGpu(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSphericalWarperGpu(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::stitching::SphericalWarperGpuTraitConst for core::Ptr<crate::stitching::SphericalWarperGpu> {
	#[inline] fn as_raw_SphericalWarperGpu(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::SphericalWarperGpuTrait for core::Ptr<crate::stitching::SphericalWarperGpu> {
	#[inline] fn as_raw_mut_SphericalWarperGpu(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::WarperCreatorTraitConst for core::Ptr<crate::stitching::SphericalWarperGpu> {
	#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::WarperCreatorTrait for core::Ptr<crate::stitching::SphericalWarperGpu> {
	#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::stitching::SphericalWarperGpu>, core::Ptr<crate::stitching::WarperCreator>, cv_PtrLcv_SphericalWarperGpuG_to_PtrOfWarperCreator }

impl std::fmt::Debug for core::Ptr<crate::stitching::SphericalWarperGpu> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfSphericalWarperGpu")
			.finish()
	}
}

