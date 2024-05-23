ptr_extern! { crate::stitching::PaniniPortraitWarper,
	cv_PtrLcv_PaniniPortraitWarperG_new_null_const, cv_PtrLcv_PaniniPortraitWarperG_delete, cv_PtrLcv_PaniniPortraitWarperG_getInnerPtr_const, cv_PtrLcv_PaniniPortraitWarperG_getInnerPtrMut
}

ptr_extern_ctor! { crate::stitching::PaniniPortraitWarper, cv_PtrLcv_PaniniPortraitWarperG_new_const_PaniniPortraitWarper }
impl core::Ptr<crate::stitching::PaniniPortraitWarper> {
	#[inline] pub fn as_raw_PtrOfPaniniPortraitWarper(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfPaniniPortraitWarper(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::stitching::PaniniPortraitWarperTraitConst for core::Ptr<crate::stitching::PaniniPortraitWarper> {
	#[inline] fn as_raw_PaniniPortraitWarper(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::PaniniPortraitWarperTrait for core::Ptr<crate::stitching::PaniniPortraitWarper> {
	#[inline] fn as_raw_mut_PaniniPortraitWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::WarperCreatorTraitConst for core::Ptr<crate::stitching::PaniniPortraitWarper> {
	#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::WarperCreatorTrait for core::Ptr<crate::stitching::PaniniPortraitWarper> {
	#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::stitching::PaniniPortraitWarper>, core::Ptr<crate::stitching::WarperCreator>, cv_PtrLcv_PaniniPortraitWarperG_to_PtrOfWarperCreator }

impl std::fmt::Debug for core::Ptr<crate::stitching::PaniniPortraitWarper> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfPaniniPortraitWarper")
			.finish()
	}
}

