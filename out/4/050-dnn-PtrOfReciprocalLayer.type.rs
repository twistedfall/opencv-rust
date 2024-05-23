ptr_extern! { crate::dnn::ReciprocalLayer,
	cv_PtrLcv_dnn_ReciprocalLayerG_new_null_const, cv_PtrLcv_dnn_ReciprocalLayerG_delete, cv_PtrLcv_dnn_ReciprocalLayerG_getInnerPtr_const, cv_PtrLcv_dnn_ReciprocalLayerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::ReciprocalLayer, cv_PtrLcv_dnn_ReciprocalLayerG_new_const_ReciprocalLayer }
impl core::Ptr<crate::dnn::ReciprocalLayer> {
	#[inline] pub fn as_raw_PtrOfReciprocalLayer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfReciprocalLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::ReciprocalLayerTraitConst for core::Ptr<crate::dnn::ReciprocalLayer> {
	#[inline] fn as_raw_ReciprocalLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ReciprocalLayerTrait for core::Ptr<crate::dnn::ReciprocalLayer> {
	#[inline] fn as_raw_mut_ReciprocalLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ReciprocalLayer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::ReciprocalLayer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::ReciprocalLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ReciprocalLayerG_to_PtrOfAlgorithm }

impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::ReciprocalLayer> {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::ReciprocalLayer> {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::ReciprocalLayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_ReciprocalLayerG_to_PtrOfActivationLayer }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ReciprocalLayer> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ReciprocalLayer> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::dnn::ReciprocalLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ReciprocalLayerG_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::ReciprocalLayer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfReciprocalLayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

