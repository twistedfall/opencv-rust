ptr_extern! { crate::shape::ShapeTransformer,
	cv_PtrLcv_ShapeTransformerG_new_null_const, cv_PtrLcv_ShapeTransformerG_delete, cv_PtrLcv_ShapeTransformerG_getInnerPtr_const, cv_PtrLcv_ShapeTransformerG_getInnerPtrMut
}

impl core::Ptr<crate::shape::ShapeTransformer> {
	#[inline] pub fn as_raw_PtrOfShapeTransformer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfShapeTransformer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::shape::ShapeTransformerTraitConst for core::Ptr<crate::shape::ShapeTransformer> {
	#[inline] fn as_raw_ShapeTransformer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::shape::ShapeTransformerTrait for core::Ptr<crate::shape::ShapeTransformer> {
	#[inline] fn as_raw_mut_ShapeTransformer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::shape::ShapeTransformer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::shape::ShapeTransformer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::shape::ShapeTransformer>, core::Ptr<core::Algorithm>, cv_PtrLcv_ShapeTransformerG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::shape::ShapeTransformer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfShapeTransformer")
			.finish()
	}
}

