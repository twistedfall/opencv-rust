ptr_extern! { crate::ml::KNearest,
	cv_PtrLcv_ml_KNearestG_new_null_const, cv_PtrLcv_ml_KNearestG_delete, cv_PtrLcv_ml_KNearestG_getInnerPtr_const, cv_PtrLcv_ml_KNearestG_getInnerPtrMut
}

impl core::Ptr<crate::ml::KNearest> {
	#[inline] pub fn as_raw_PtrOfKNearest(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfKNearest(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::ml::KNearestTraitConst for core::Ptr<crate::ml::KNearest> {
	#[inline] fn as_raw_KNearest(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ml::KNearestTrait for core::Ptr<crate::ml::KNearest> {
	#[inline] fn as_raw_mut_KNearest(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::ml::KNearest> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::ml::KNearest> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::ml::KNearest>, core::Ptr<core::Algorithm>, cv_PtrLcv_ml_KNearestG_to_PtrOfAlgorithm }

impl crate::ml::StatModelTraitConst for core::Ptr<crate::ml::KNearest> {
	#[inline] fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ml::StatModelTrait for core::Ptr<crate::ml::KNearest> {
	#[inline] fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::ml::KNearest>, core::Ptr<crate::ml::StatModel>, cv_PtrLcv_ml_KNearestG_to_PtrOfStatModel }

impl std::fmt::Debug for core::Ptr<crate::ml::KNearest> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfKNearest")
			.finish()
	}
}

