ptr_extern! { core::Formatted,
	cv_PtrLcv_FormattedG_new_null_const, cv_PtrLcv_FormattedG_delete, cv_PtrLcv_FormattedG_getInnerPtr_const, cv_PtrLcv_FormattedG_getInnerPtrMut
}

impl core::Ptr<core::Formatted> {
	#[inline] pub fn as_raw_PtrOfFormatted(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfFormatted(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl core::FormattedTraitConst for core::Ptr<core::Formatted> {
	#[inline] fn as_raw_Formatted(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::FormattedTrait for core::Ptr<core::Formatted> {
	#[inline] fn as_raw_mut_Formatted(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<core::Formatted> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfFormatted")
			.finish()
	}
}

