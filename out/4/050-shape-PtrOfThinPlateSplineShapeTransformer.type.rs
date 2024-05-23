ptr_extern! { crate::shape::ThinPlateSplineShapeTransformer,
	cv_PtrLcv_ThinPlateSplineShapeTransformerG_new_null_const, cv_PtrLcv_ThinPlateSplineShapeTransformerG_delete, cv_PtrLcv_ThinPlateSplineShapeTransformerG_getInnerPtr_const, cv_PtrLcv_ThinPlateSplineShapeTransformerG_getInnerPtrMut
}

impl core::Ptr<crate::shape::ThinPlateSplineShapeTransformer> {
	#[inline] pub fn as_raw_PtrOfThinPlateSplineShapeTransformer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfThinPlateSplineShapeTransformer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::shape::ThinPlateSplineShapeTransformerTraitConst for core::Ptr<crate::shape::ThinPlateSplineShapeTransformer> {
	#[inline] fn as_raw_ThinPlateSplineShapeTransformer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::shape::ThinPlateSplineShapeTransformerTrait for core::Ptr<crate::shape::ThinPlateSplineShapeTransformer> {
	#[inline] fn as_raw_mut_ThinPlateSplineShapeTransformer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::shape::ThinPlateSplineShapeTransformer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::shape::ThinPlateSplineShapeTransformer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::shape::ThinPlateSplineShapeTransformer>, core::Ptr<core::Algorithm>, cv_PtrLcv_ThinPlateSplineShapeTransformerG_to_PtrOfAlgorithm }

impl crate::shape::ShapeTransformerTraitConst for core::Ptr<crate::shape::ThinPlateSplineShapeTransformer> {
	#[inline] fn as_raw_ShapeTransformer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::shape::ShapeTransformerTrait for core::Ptr<crate::shape::ThinPlateSplineShapeTransformer> {
	#[inline] fn as_raw_mut_ShapeTransformer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::shape::ThinPlateSplineShapeTransformer>, core::Ptr<crate::shape::ShapeTransformer>, cv_PtrLcv_ThinPlateSplineShapeTransformerG_to_PtrOfShapeTransformer }

impl std::fmt::Debug for core::Ptr<crate::shape::ThinPlateSplineShapeTransformer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfThinPlateSplineShapeTransformer")
			.finish()
	}
}

