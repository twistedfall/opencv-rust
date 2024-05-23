ptr_extern! { crate::stitching::CompressedRectilinearWarper,
	cv_PtrLcv_CompressedRectilinearWarperG_new_null_const, cv_PtrLcv_CompressedRectilinearWarperG_delete, cv_PtrLcv_CompressedRectilinearWarperG_getInnerPtr_const, cv_PtrLcv_CompressedRectilinearWarperG_getInnerPtrMut
}

ptr_extern_ctor! { crate::stitching::CompressedRectilinearWarper, cv_PtrLcv_CompressedRectilinearWarperG_new_const_CompressedRectilinearWarper }
impl core::Ptr<crate::stitching::CompressedRectilinearWarper> {
	#[inline] pub fn as_raw_PtrOfCompressedRectilinearWarper(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfCompressedRectilinearWarper(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::stitching::CompressedRectilinearWarperTraitConst for core::Ptr<crate::stitching::CompressedRectilinearWarper> {
	#[inline] fn as_raw_CompressedRectilinearWarper(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::CompressedRectilinearWarperTrait for core::Ptr<crate::stitching::CompressedRectilinearWarper> {
	#[inline] fn as_raw_mut_CompressedRectilinearWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::WarperCreatorTraitConst for core::Ptr<crate::stitching::CompressedRectilinearWarper> {
	#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::WarperCreatorTrait for core::Ptr<crate::stitching::CompressedRectilinearWarper> {
	#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::stitching::CompressedRectilinearWarper>, core::Ptr<crate::stitching::WarperCreator>, cv_PtrLcv_CompressedRectilinearWarperG_to_PtrOfWarperCreator }

impl std::fmt::Debug for core::Ptr<crate::stitching::CompressedRectilinearWarper> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfCompressedRectilinearWarper")
			.finish()
	}
}

