ptr_extern! { crate::videostab::FromFileMotionReader,
	cv_PtrLcv_videostab_FromFileMotionReaderG_new_null_const, cv_PtrLcv_videostab_FromFileMotionReaderG_delete, cv_PtrLcv_videostab_FromFileMotionReaderG_getInnerPtr_const, cv_PtrLcv_videostab_FromFileMotionReaderG_getInnerPtrMut
}

ptr_extern_ctor! { crate::videostab::FromFileMotionReader, cv_PtrLcv_videostab_FromFileMotionReaderG_new_const_FromFileMotionReader }
impl core::Ptr<crate::videostab::FromFileMotionReader> {
	#[inline] pub fn as_raw_PtrOfFromFileMotionReader(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfFromFileMotionReader(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::videostab::FromFileMotionReaderTraitConst for core::Ptr<crate::videostab::FromFileMotionReader> {
	#[inline] fn as_raw_FromFileMotionReader(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::FromFileMotionReaderTrait for core::Ptr<crate::videostab::FromFileMotionReader> {
	#[inline] fn as_raw_mut_FromFileMotionReader(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::ImageMotionEstimatorBaseTraitConst for core::Ptr<crate::videostab::FromFileMotionReader> {
	#[inline] fn as_raw_ImageMotionEstimatorBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::ImageMotionEstimatorBaseTrait for core::Ptr<crate::videostab::FromFileMotionReader> {
	#[inline] fn as_raw_mut_ImageMotionEstimatorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::videostab::FromFileMotionReader>, core::Ptr<crate::videostab::ImageMotionEstimatorBase>, cv_PtrLcv_videostab_FromFileMotionReaderG_to_PtrOfImageMotionEstimatorBase }

impl std::fmt::Debug for core::Ptr<crate::videostab::FromFileMotionReader> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfFromFileMotionReader")
			.finish()
	}
}

