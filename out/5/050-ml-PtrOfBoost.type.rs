ptr_extern! { crate::ml::Boost,
	cv_PtrLcv_ml_BoostG_new_null_const, cv_PtrLcv_ml_BoostG_delete, cv_PtrLcv_ml_BoostG_getInnerPtr_const, cv_PtrLcv_ml_BoostG_getInnerPtrMut
}

impl core::Ptr<crate::ml::Boost> {
	#[inline] pub fn as_raw_PtrOfBoost(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfBoost(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::ml::BoostTraitConst for core::Ptr<crate::ml::Boost> {
	#[inline] fn as_raw_Boost(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ml::BoostTrait for core::Ptr<crate::ml::Boost> {
	#[inline] fn as_raw_mut_Boost(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::ml::Boost> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::ml::Boost> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::ml::Boost>, core::Ptr<core::Algorithm>, cv_PtrLcv_ml_BoostG_to_PtrOfAlgorithm }

impl crate::ml::DTreesTraitConst for core::Ptr<crate::ml::Boost> {
	#[inline] fn as_raw_DTrees(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ml::DTreesTrait for core::Ptr<crate::ml::Boost> {
	#[inline] fn as_raw_mut_DTrees(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::ml::Boost>, core::Ptr<crate::ml::DTrees>, cv_PtrLcv_ml_BoostG_to_PtrOfDTrees }

impl crate::ml::StatModelTraitConst for core::Ptr<crate::ml::Boost> {
	#[inline] fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ml::StatModelTrait for core::Ptr<crate::ml::Boost> {
	#[inline] fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::ml::Boost>, core::Ptr<crate::ml::StatModel>, cv_PtrLcv_ml_BoostG_to_PtrOfStatModel }

impl std::fmt::Debug for core::Ptr<crate::ml::Boost> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfBoost")
			.finish()
	}
}

