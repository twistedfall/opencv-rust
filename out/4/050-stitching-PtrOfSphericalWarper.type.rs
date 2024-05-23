ptr_extern! { crate::stitching::SphericalWarper,
	cv_PtrLcv_SphericalWarperG_new_null_const, cv_PtrLcv_SphericalWarperG_delete, cv_PtrLcv_SphericalWarperG_getInnerPtr_const, cv_PtrLcv_SphericalWarperG_getInnerPtrMut
}

ptr_extern_ctor! { crate::stitching::SphericalWarper, cv_PtrLcv_SphericalWarperG_new_const_SphericalWarper }
impl core::Ptr<crate::stitching::SphericalWarper> {
	#[inline] pub fn as_raw_PtrOfSphericalWarper(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSphericalWarper(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::stitching::SphericalWarperTraitConst for core::Ptr<crate::stitching::SphericalWarper> {
	#[inline] fn as_raw_SphericalWarper(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::SphericalWarperTrait for core::Ptr<crate::stitching::SphericalWarper> {
	#[inline] fn as_raw_mut_SphericalWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::WarperCreatorTraitConst for core::Ptr<crate::stitching::SphericalWarper> {
	#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::WarperCreatorTrait for core::Ptr<crate::stitching::SphericalWarper> {
	#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::stitching::SphericalWarper>, core::Ptr<crate::stitching::WarperCreator>, cv_PtrLcv_SphericalWarperG_to_PtrOfWarperCreator }

impl std::fmt::Debug for core::Ptr<crate::stitching::SphericalWarper> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfSphericalWarper")
			.finish()
	}
}

