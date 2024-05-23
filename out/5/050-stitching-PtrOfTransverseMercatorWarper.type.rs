ptr_extern! { crate::stitching::TransverseMercatorWarper,
	cv_PtrLcv_TransverseMercatorWarperG_new_null_const, cv_PtrLcv_TransverseMercatorWarperG_delete, cv_PtrLcv_TransverseMercatorWarperG_getInnerPtr_const, cv_PtrLcv_TransverseMercatorWarperG_getInnerPtrMut
}

ptr_extern_ctor! { crate::stitching::TransverseMercatorWarper, cv_PtrLcv_TransverseMercatorWarperG_new_const_TransverseMercatorWarper }
impl core::Ptr<crate::stitching::TransverseMercatorWarper> {
	#[inline] pub fn as_raw_PtrOfTransverseMercatorWarper(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTransverseMercatorWarper(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::stitching::TransverseMercatorWarperTraitConst for core::Ptr<crate::stitching::TransverseMercatorWarper> {
	#[inline] fn as_raw_TransverseMercatorWarper(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::TransverseMercatorWarperTrait for core::Ptr<crate::stitching::TransverseMercatorWarper> {
	#[inline] fn as_raw_mut_TransverseMercatorWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::WarperCreatorTraitConst for core::Ptr<crate::stitching::TransverseMercatorWarper> {
	#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::WarperCreatorTrait for core::Ptr<crate::stitching::TransverseMercatorWarper> {
	#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::stitching::TransverseMercatorWarper>, core::Ptr<crate::stitching::WarperCreator>, cv_PtrLcv_TransverseMercatorWarperG_to_PtrOfWarperCreator }

impl std::fmt::Debug for core::Ptr<crate::stitching::TransverseMercatorWarper> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfTransverseMercatorWarper")
			.finish()
	}
}

