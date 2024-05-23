ptr_extern! { crate::ml::TrainData,
	cv_PtrLcv_ml_TrainDataG_new_null_const, cv_PtrLcv_ml_TrainDataG_delete, cv_PtrLcv_ml_TrainDataG_getInnerPtr_const, cv_PtrLcv_ml_TrainDataG_getInnerPtrMut
}

impl core::Ptr<crate::ml::TrainData> {
	#[inline] pub fn as_raw_PtrOfTrainData(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTrainData(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::ml::TrainDataTraitConst for core::Ptr<crate::ml::TrainData> {
	#[inline] fn as_raw_TrainData(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ml::TrainDataTrait for core::Ptr<crate::ml::TrainData> {
	#[inline] fn as_raw_mut_TrainData(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::ml::TrainData> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfTrainData")
			.finish()
	}
}

