ptr_extern! { crate::stitching::StereographicWarper,
	cv_PtrLcv_StereographicWarperG_new_null_const, cv_PtrLcv_StereographicWarperG_delete, cv_PtrLcv_StereographicWarperG_getInnerPtr_const, cv_PtrLcv_StereographicWarperG_getInnerPtrMut
}

ptr_extern_ctor! { crate::stitching::StereographicWarper, cv_PtrLcv_StereographicWarperG_new_const_StereographicWarper }
impl core::Ptr<crate::stitching::StereographicWarper> {
	#[inline] pub fn as_raw_PtrOfStereographicWarper(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfStereographicWarper(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::stitching::StereographicWarperTraitConst for core::Ptr<crate::stitching::StereographicWarper> {
	#[inline] fn as_raw_StereographicWarper(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::StereographicWarperTrait for core::Ptr<crate::stitching::StereographicWarper> {
	#[inline] fn as_raw_mut_StereographicWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::WarperCreatorTraitConst for core::Ptr<crate::stitching::StereographicWarper> {
	#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::WarperCreatorTrait for core::Ptr<crate::stitching::StereographicWarper> {
	#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::stitching::StereographicWarper>, core::Ptr<crate::stitching::WarperCreator>, cv_PtrLcv_StereographicWarperG_to_PtrOfWarperCreator }

impl std::fmt::Debug for core::Ptr<crate::stitching::StereographicWarper> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfStereographicWarper")
			.finish()
	}
}

