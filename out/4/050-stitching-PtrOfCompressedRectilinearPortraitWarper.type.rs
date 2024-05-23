ptr_extern! { crate::stitching::CompressedRectilinearPortraitWarper,
	cv_PtrLcv_CompressedRectilinearPortraitWarperG_new_null_const, cv_PtrLcv_CompressedRectilinearPortraitWarperG_delete, cv_PtrLcv_CompressedRectilinearPortraitWarperG_getInnerPtr_const, cv_PtrLcv_CompressedRectilinearPortraitWarperG_getInnerPtrMut
}

ptr_extern_ctor! { crate::stitching::CompressedRectilinearPortraitWarper, cv_PtrLcv_CompressedRectilinearPortraitWarperG_new_const_CompressedRectilinearPortraitWarper }
impl core::Ptr<crate::stitching::CompressedRectilinearPortraitWarper> {
	#[inline] pub fn as_raw_PtrOfCompressedRectilinearPortraitWarper(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfCompressedRectilinearPortraitWarper(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::stitching::CompressedRectilinearPortraitWarperTraitConst for core::Ptr<crate::stitching::CompressedRectilinearPortraitWarper> {
	#[inline] fn as_raw_CompressedRectilinearPortraitWarper(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::CompressedRectilinearPortraitWarperTrait for core::Ptr<crate::stitching::CompressedRectilinearPortraitWarper> {
	#[inline] fn as_raw_mut_CompressedRectilinearPortraitWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::WarperCreatorTraitConst for core::Ptr<crate::stitching::CompressedRectilinearPortraitWarper> {
	#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::WarperCreatorTrait for core::Ptr<crate::stitching::CompressedRectilinearPortraitWarper> {
	#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::stitching::CompressedRectilinearPortraitWarper>, core::Ptr<crate::stitching::WarperCreator>, cv_PtrLcv_CompressedRectilinearPortraitWarperG_to_PtrOfWarperCreator }

impl std::fmt::Debug for core::Ptr<crate::stitching::CompressedRectilinearPortraitWarper> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfCompressedRectilinearPortraitWarper")
			.finish()
	}
}

