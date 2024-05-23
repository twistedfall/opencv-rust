ptr_extern! { crate::videoio::IStreamReader,
	cv_PtrLcv_IStreamReaderG_new_null_const, cv_PtrLcv_IStreamReaderG_delete, cv_PtrLcv_IStreamReaderG_getInnerPtr_const, cv_PtrLcv_IStreamReaderG_getInnerPtrMut
}

impl core::Ptr<crate::videoio::IStreamReader> {
	#[inline] pub fn as_raw_PtrOfIStreamReader(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfIStreamReader(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::videoio::IStreamReaderTraitConst for core::Ptr<crate::videoio::IStreamReader> {
	#[inline] fn as_raw_IStreamReader(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videoio::IStreamReaderTrait for core::Ptr<crate::videoio::IStreamReader> {
	#[inline] fn as_raw_mut_IStreamReader(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::videoio::IStreamReader> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfIStreamReader")
			.finish()
	}
}

