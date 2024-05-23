ptr_extern! { crate::tracking::TrackerFeatureHOG,
	cv_PtrLcv_TrackerFeatureHOGG_new_null_const, cv_PtrLcv_TrackerFeatureHOGG_delete, cv_PtrLcv_TrackerFeatureHOGG_getInnerPtr_const, cv_PtrLcv_TrackerFeatureHOGG_getInnerPtrMut
}

ptr_extern_ctor! { crate::tracking::TrackerFeatureHOG, cv_PtrLcv_TrackerFeatureHOGG_new_const_TrackerFeatureHOG }
impl core::Ptr<crate::tracking::TrackerFeatureHOG> {
	#[inline] pub fn as_raw_PtrOfTrackerFeatureHOG(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTrackerFeatureHOG(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::tracking::TrackerFeatureHOGTraitConst for core::Ptr<crate::tracking::TrackerFeatureHOG> {
	#[inline] fn as_raw_TrackerFeatureHOG(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::tracking::TrackerFeatureHOGTrait for core::Ptr<crate::tracking::TrackerFeatureHOG> {
	#[inline] fn as_raw_mut_TrackerFeatureHOG(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::tracking::TrackerFeatureTraitConst for core::Ptr<crate::tracking::TrackerFeatureHOG> {
	#[inline] fn as_raw_TrackerFeature(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::tracking::TrackerFeatureTrait for core::Ptr<crate::tracking::TrackerFeatureHOG> {
	#[inline] fn as_raw_mut_TrackerFeature(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::tracking::TrackerFeatureHOG>, core::Ptr<crate::tracking::TrackerFeature>, cv_PtrLcv_TrackerFeatureHOGG_to_PtrOfTrackerFeature }

impl std::fmt::Debug for core::Ptr<crate::tracking::TrackerFeatureHOG> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfTrackerFeatureHOG")
			.finish()
	}
}

