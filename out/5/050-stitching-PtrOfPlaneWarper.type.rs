ptr_extern! { crate::stitching::PlaneWarper,
	cv_PtrLcv_PlaneWarperG_new_null_const, cv_PtrLcv_PlaneWarperG_delete, cv_PtrLcv_PlaneWarperG_getInnerPtr_const, cv_PtrLcv_PlaneWarperG_getInnerPtrMut
}

ptr_extern_ctor! { crate::stitching::PlaneWarper, cv_PtrLcv_PlaneWarperG_new_const_PlaneWarper }
impl core::Ptr<crate::stitching::PlaneWarper> {
	#[inline] pub fn as_raw_PtrOfPlaneWarper(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfPlaneWarper(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::stitching::PlaneWarperTraitConst for core::Ptr<crate::stitching::PlaneWarper> {
	#[inline] fn as_raw_PlaneWarper(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::PlaneWarperTrait for core::Ptr<crate::stitching::PlaneWarper> {
	#[inline] fn as_raw_mut_PlaneWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::WarperCreatorTraitConst for core::Ptr<crate::stitching::PlaneWarper> {
	#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::WarperCreatorTrait for core::Ptr<crate::stitching::PlaneWarper> {
	#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::stitching::PlaneWarper>, core::Ptr<crate::stitching::WarperCreator>, cv_PtrLcv_PlaneWarperG_to_PtrOfWarperCreator }

impl std::fmt::Debug for core::Ptr<crate::stitching::PlaneWarper> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfPlaneWarper")
			.finish()
	}
}

