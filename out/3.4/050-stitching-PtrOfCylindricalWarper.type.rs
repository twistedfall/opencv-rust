ptr_extern! { crate::stitching::CylindricalWarper,
	cv_PtrLcv_CylindricalWarperG_new_null_const, cv_PtrLcv_CylindricalWarperG_delete, cv_PtrLcv_CylindricalWarperG_getInnerPtr_const, cv_PtrLcv_CylindricalWarperG_getInnerPtrMut
}

ptr_extern_ctor! { crate::stitching::CylindricalWarper, cv_PtrLcv_CylindricalWarperG_new_const_CylindricalWarper }
impl core::Ptr<crate::stitching::CylindricalWarper> {
	#[inline] pub fn as_raw_PtrOfCylindricalWarper(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfCylindricalWarper(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::stitching::CylindricalWarperTraitConst for core::Ptr<crate::stitching::CylindricalWarper> {
	#[inline] fn as_raw_CylindricalWarper(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::CylindricalWarperTrait for core::Ptr<crate::stitching::CylindricalWarper> {
	#[inline] fn as_raw_mut_CylindricalWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::WarperCreatorTraitConst for core::Ptr<crate::stitching::CylindricalWarper> {
	#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::WarperCreatorTrait for core::Ptr<crate::stitching::CylindricalWarper> {
	#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::stitching::CylindricalWarper>, core::Ptr<crate::stitching::WarperCreator>, cv_PtrLcv_CylindricalWarperG_to_PtrOfWarperCreator }

impl std::fmt::Debug for core::Ptr<crate::stitching::CylindricalWarper> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfCylindricalWarper")
			.finish()
	}
}

