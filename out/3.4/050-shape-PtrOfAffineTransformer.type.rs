ptr_extern! { crate::shape::AffineTransformer,
	cv_PtrLcv_AffineTransformerG_new_null_const, cv_PtrLcv_AffineTransformerG_delete, cv_PtrLcv_AffineTransformerG_getInnerPtr_const, cv_PtrLcv_AffineTransformerG_getInnerPtrMut
}

impl core::Ptr<crate::shape::AffineTransformer> {
	#[inline] pub fn as_raw_PtrOfAffineTransformer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfAffineTransformer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::shape::AffineTransformerTraitConst for core::Ptr<crate::shape::AffineTransformer> {
	#[inline] fn as_raw_AffineTransformer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::shape::AffineTransformerTrait for core::Ptr<crate::shape::AffineTransformer> {
	#[inline] fn as_raw_mut_AffineTransformer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::shape::AffineTransformer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::shape::AffineTransformer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::shape::AffineTransformer>, core::Ptr<core::Algorithm>, cv_PtrLcv_AffineTransformerG_to_PtrOfAlgorithm }

impl crate::shape::ShapeTransformerTraitConst for core::Ptr<crate::shape::AffineTransformer> {
	#[inline] fn as_raw_ShapeTransformer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::shape::ShapeTransformerTrait for core::Ptr<crate::shape::AffineTransformer> {
	#[inline] fn as_raw_mut_ShapeTransformer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::shape::AffineTransformer>, core::Ptr<crate::shape::ShapeTransformer>, cv_PtrLcv_AffineTransformerG_to_PtrOfShapeTransformer }

impl std::fmt::Debug for core::Ptr<crate::shape::AffineTransformer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfAffineTransformer")
			.finish()
	}
}

