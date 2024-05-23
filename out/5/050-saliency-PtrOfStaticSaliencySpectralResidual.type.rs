ptr_extern! { crate::saliency::StaticSaliencySpectralResidual,
	cv_PtrLcv_saliency_StaticSaliencySpectralResidualG_new_null_const, cv_PtrLcv_saliency_StaticSaliencySpectralResidualG_delete, cv_PtrLcv_saliency_StaticSaliencySpectralResidualG_getInnerPtr_const, cv_PtrLcv_saliency_StaticSaliencySpectralResidualG_getInnerPtrMut
}

ptr_extern_ctor! { crate::saliency::StaticSaliencySpectralResidual, cv_PtrLcv_saliency_StaticSaliencySpectralResidualG_new_const_StaticSaliencySpectralResidual }
impl core::Ptr<crate::saliency::StaticSaliencySpectralResidual> {
	#[inline] pub fn as_raw_PtrOfStaticSaliencySpectralResidual(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfStaticSaliencySpectralResidual(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::saliency::StaticSaliencySpectralResidualTraitConst for core::Ptr<crate::saliency::StaticSaliencySpectralResidual> {
	#[inline] fn as_raw_StaticSaliencySpectralResidual(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::saliency::StaticSaliencySpectralResidualTrait for core::Ptr<crate::saliency::StaticSaliencySpectralResidual> {
	#[inline] fn as_raw_mut_StaticSaliencySpectralResidual(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::saliency::StaticSaliencySpectralResidual> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::saliency::StaticSaliencySpectralResidual> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::saliency::StaticSaliencySpectralResidual>, core::Ptr<core::Algorithm>, cv_PtrLcv_saliency_StaticSaliencySpectralResidualG_to_PtrOfAlgorithm }

impl crate::saliency::SaliencyTraitConst for core::Ptr<crate::saliency::StaticSaliencySpectralResidual> {
	#[inline] fn as_raw_Saliency(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::saliency::SaliencyTrait for core::Ptr<crate::saliency::StaticSaliencySpectralResidual> {
	#[inline] fn as_raw_mut_Saliency(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::saliency::StaticSaliencySpectralResidual>, core::Ptr<crate::saliency::Saliency>, cv_PtrLcv_saliency_StaticSaliencySpectralResidualG_to_PtrOfSaliency }

impl crate::saliency::StaticSaliencyTraitConst for core::Ptr<crate::saliency::StaticSaliencySpectralResidual> {
	#[inline] fn as_raw_StaticSaliency(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::saliency::StaticSaliencyTrait for core::Ptr<crate::saliency::StaticSaliencySpectralResidual> {
	#[inline] fn as_raw_mut_StaticSaliency(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::saliency::StaticSaliencySpectralResidual>, core::Ptr<crate::saliency::StaticSaliency>, cv_PtrLcv_saliency_StaticSaliencySpectralResidualG_to_PtrOfStaticSaliency }

impl std::fmt::Debug for core::Ptr<crate::saliency::StaticSaliencySpectralResidual> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfStaticSaliencySpectralResidual")
			.finish()
	}
}

