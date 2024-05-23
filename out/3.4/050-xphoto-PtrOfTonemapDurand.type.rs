ptr_extern! { crate::xphoto::TonemapDurand,
	cv_PtrLcv_xphoto_TonemapDurandG_new_null_const, cv_PtrLcv_xphoto_TonemapDurandG_delete, cv_PtrLcv_xphoto_TonemapDurandG_getInnerPtr_const, cv_PtrLcv_xphoto_TonemapDurandG_getInnerPtrMut
}

impl core::Ptr<crate::xphoto::TonemapDurand> {
	#[inline] pub fn as_raw_PtrOfTonemapDurand(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTonemapDurand(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::xphoto::TonemapDurandTraitConst for core::Ptr<crate::xphoto::TonemapDurand> {
	#[inline] fn as_raw_TonemapDurand(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::xphoto::TonemapDurandTrait for core::Ptr<crate::xphoto::TonemapDurand> {
	#[inline] fn as_raw_mut_TonemapDurand(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::xphoto::TonemapDurand> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::xphoto::TonemapDurand> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::xphoto::TonemapDurand>, core::Ptr<core::Algorithm>, cv_PtrLcv_xphoto_TonemapDurandG_to_PtrOfAlgorithm }

impl crate::photo::TonemapTraitConst for core::Ptr<crate::xphoto::TonemapDurand> {
	#[inline] fn as_raw_Tonemap(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::photo::TonemapTrait for core::Ptr<crate::xphoto::TonemapDurand> {
	#[inline] fn as_raw_mut_Tonemap(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::xphoto::TonemapDurand>, core::Ptr<crate::photo::Tonemap>, cv_PtrLcv_xphoto_TonemapDurandG_to_PtrOfTonemap }

impl std::fmt::Debug for core::Ptr<crate::xphoto::TonemapDurand> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfTonemapDurand")
			.finish()
	}
}

