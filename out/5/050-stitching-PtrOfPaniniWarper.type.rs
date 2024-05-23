ptr_extern! { crate::stitching::PaniniWarper,
	cv_PtrLcv_PaniniWarperG_new_null_const, cv_PtrLcv_PaniniWarperG_delete, cv_PtrLcv_PaniniWarperG_getInnerPtr_const, cv_PtrLcv_PaniniWarperG_getInnerPtrMut
}

ptr_extern_ctor! { crate::stitching::PaniniWarper, cv_PtrLcv_PaniniWarperG_new_const_PaniniWarper }
impl core::Ptr<crate::stitching::PaniniWarper> {
	#[inline] pub fn as_raw_PtrOfPaniniWarper(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfPaniniWarper(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::stitching::PaniniWarperTraitConst for core::Ptr<crate::stitching::PaniniWarper> {
	#[inline] fn as_raw_PaniniWarper(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::PaniniWarperTrait for core::Ptr<crate::stitching::PaniniWarper> {
	#[inline] fn as_raw_mut_PaniniWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::WarperCreatorTraitConst for core::Ptr<crate::stitching::PaniniWarper> {
	#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::WarperCreatorTrait for core::Ptr<crate::stitching::PaniniWarper> {
	#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::stitching::PaniniWarper>, core::Ptr<crate::stitching::WarperCreator>, cv_PtrLcv_PaniniWarperG_to_PtrOfWarperCreator }

impl std::fmt::Debug for core::Ptr<crate::stitching::PaniniWarper> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfPaniniWarper")
			.finish()
	}
}

