ptr_extern! { crate::stitching::CylindricalWarperGpu,
	cv_PtrLcv_CylindricalWarperGpuG_new_null_const, cv_PtrLcv_CylindricalWarperGpuG_delete, cv_PtrLcv_CylindricalWarperGpuG_getInnerPtr_const, cv_PtrLcv_CylindricalWarperGpuG_getInnerPtrMut
}

ptr_extern_ctor! { crate::stitching::CylindricalWarperGpu, cv_PtrLcv_CylindricalWarperGpuG_new_const_CylindricalWarperGpu }
impl core::Ptr<crate::stitching::CylindricalWarperGpu> {
	#[inline] pub fn as_raw_PtrOfCylindricalWarperGpu(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfCylindricalWarperGpu(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::stitching::CylindricalWarperGpuTraitConst for core::Ptr<crate::stitching::CylindricalWarperGpu> {
	#[inline] fn as_raw_CylindricalWarperGpu(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::CylindricalWarperGpuTrait for core::Ptr<crate::stitching::CylindricalWarperGpu> {
	#[inline] fn as_raw_mut_CylindricalWarperGpu(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::WarperCreatorTraitConst for core::Ptr<crate::stitching::CylindricalWarperGpu> {
	#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::WarperCreatorTrait for core::Ptr<crate::stitching::CylindricalWarperGpu> {
	#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::stitching::CylindricalWarperGpu>, core::Ptr<crate::stitching::WarperCreator>, cv_PtrLcv_CylindricalWarperGpuG_to_PtrOfWarperCreator }

impl std::fmt::Debug for core::Ptr<crate::stitching::CylindricalWarperGpu> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfCylindricalWarperGpu")
			.finish()
	}
}

