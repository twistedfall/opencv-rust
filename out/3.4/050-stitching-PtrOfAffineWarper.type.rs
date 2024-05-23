ptr_extern! { crate::stitching::AffineWarper,
	cv_PtrLcv_AffineWarperG_new_null_const, cv_PtrLcv_AffineWarperG_delete, cv_PtrLcv_AffineWarperG_getInnerPtr_const, cv_PtrLcv_AffineWarperG_getInnerPtrMut
}

ptr_extern_ctor! { crate::stitching::AffineWarper, cv_PtrLcv_AffineWarperG_new_const_AffineWarper }
impl core::Ptr<crate::stitching::AffineWarper> {
	#[inline] pub fn as_raw_PtrOfAffineWarper(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfAffineWarper(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::stitching::AffineWarperTraitConst for core::Ptr<crate::stitching::AffineWarper> {
	#[inline] fn as_raw_AffineWarper(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::AffineWarperTrait for core::Ptr<crate::stitching::AffineWarper> {
	#[inline] fn as_raw_mut_AffineWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::WarperCreatorTraitConst for core::Ptr<crate::stitching::AffineWarper> {
	#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::WarperCreatorTrait for core::Ptr<crate::stitching::AffineWarper> {
	#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::stitching::AffineWarper>, core::Ptr<crate::stitching::WarperCreator>, cv_PtrLcv_AffineWarperG_to_PtrOfWarperCreator }

impl std::fmt::Debug for core::Ptr<crate::stitching::AffineWarper> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfAffineWarper")
			.finish()
	}
}

