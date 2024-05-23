ptr_extern! { crate::stitching::FisheyeWarper,
	cv_PtrLcv_FisheyeWarperG_new_null_const, cv_PtrLcv_FisheyeWarperG_delete, cv_PtrLcv_FisheyeWarperG_getInnerPtr_const, cv_PtrLcv_FisheyeWarperG_getInnerPtrMut
}

ptr_extern_ctor! { crate::stitching::FisheyeWarper, cv_PtrLcv_FisheyeWarperG_new_const_FisheyeWarper }
impl core::Ptr<crate::stitching::FisheyeWarper> {
	#[inline] pub fn as_raw_PtrOfFisheyeWarper(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfFisheyeWarper(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::stitching::FisheyeWarperTraitConst for core::Ptr<crate::stitching::FisheyeWarper> {
	#[inline] fn as_raw_FisheyeWarper(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::FisheyeWarperTrait for core::Ptr<crate::stitching::FisheyeWarper> {
	#[inline] fn as_raw_mut_FisheyeWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::WarperCreatorTraitConst for core::Ptr<crate::stitching::FisheyeWarper> {
	#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::WarperCreatorTrait for core::Ptr<crate::stitching::FisheyeWarper> {
	#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::stitching::FisheyeWarper>, core::Ptr<crate::stitching::WarperCreator>, cv_PtrLcv_FisheyeWarperG_to_PtrOfWarperCreator }

impl std::fmt::Debug for core::Ptr<crate::stitching::FisheyeWarper> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfFisheyeWarper")
			.finish()
	}
}

