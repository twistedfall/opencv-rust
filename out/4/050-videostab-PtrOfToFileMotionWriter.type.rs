ptr_extern! { crate::videostab::ToFileMotionWriter,
	cv_PtrLcv_videostab_ToFileMotionWriterG_new_null_const, cv_PtrLcv_videostab_ToFileMotionWriterG_delete, cv_PtrLcv_videostab_ToFileMotionWriterG_getInnerPtr_const, cv_PtrLcv_videostab_ToFileMotionWriterG_getInnerPtrMut
}

ptr_extern_ctor! { crate::videostab::ToFileMotionWriter, cv_PtrLcv_videostab_ToFileMotionWriterG_new_const_ToFileMotionWriter }
impl core::Ptr<crate::videostab::ToFileMotionWriter> {
	#[inline] pub fn as_raw_PtrOfToFileMotionWriter(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfToFileMotionWriter(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::videostab::ToFileMotionWriterTraitConst for core::Ptr<crate::videostab::ToFileMotionWriter> {
	#[inline] fn as_raw_ToFileMotionWriter(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::ToFileMotionWriterTrait for core::Ptr<crate::videostab::ToFileMotionWriter> {
	#[inline] fn as_raw_mut_ToFileMotionWriter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::ImageMotionEstimatorBaseTraitConst for core::Ptr<crate::videostab::ToFileMotionWriter> {
	#[inline] fn as_raw_ImageMotionEstimatorBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::ImageMotionEstimatorBaseTrait for core::Ptr<crate::videostab::ToFileMotionWriter> {
	#[inline] fn as_raw_mut_ImageMotionEstimatorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::videostab::ToFileMotionWriter>, core::Ptr<crate::videostab::ImageMotionEstimatorBase>, cv_PtrLcv_videostab_ToFileMotionWriterG_to_PtrOfImageMotionEstimatorBase }

impl std::fmt::Debug for core::Ptr<crate::videostab::ToFileMotionWriter> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfToFileMotionWriter")
			.finish()
	}
}

