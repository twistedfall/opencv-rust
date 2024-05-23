ptr_extern! { crate::stitching::MercatorWarper,
	cv_PtrLcv_MercatorWarperG_new_null_const, cv_PtrLcv_MercatorWarperG_delete, cv_PtrLcv_MercatorWarperG_getInnerPtr_const, cv_PtrLcv_MercatorWarperG_getInnerPtrMut
}

ptr_extern_ctor! { crate::stitching::MercatorWarper, cv_PtrLcv_MercatorWarperG_new_const_MercatorWarper }
impl core::Ptr<crate::stitching::MercatorWarper> {
	#[inline] pub fn as_raw_PtrOfMercatorWarper(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfMercatorWarper(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::stitching::MercatorWarperTraitConst for core::Ptr<crate::stitching::MercatorWarper> {
	#[inline] fn as_raw_MercatorWarper(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::MercatorWarperTrait for core::Ptr<crate::stitching::MercatorWarper> {
	#[inline] fn as_raw_mut_MercatorWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::WarperCreatorTraitConst for core::Ptr<crate::stitching::MercatorWarper> {
	#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::WarperCreatorTrait for core::Ptr<crate::stitching::MercatorWarper> {
	#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::stitching::MercatorWarper>, core::Ptr<crate::stitching::WarperCreator>, cv_PtrLcv_MercatorWarperG_to_PtrOfWarperCreator }

impl std::fmt::Debug for core::Ptr<crate::stitching::MercatorWarper> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfMercatorWarper")
			.finish()
	}
}

