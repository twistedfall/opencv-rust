ptr_extern! { crate::objdetect::QRCodeEncoder,
	cv_PtrLcv_QRCodeEncoderG_new_null_const, cv_PtrLcv_QRCodeEncoderG_delete, cv_PtrLcv_QRCodeEncoderG_getInnerPtr_const, cv_PtrLcv_QRCodeEncoderG_getInnerPtrMut
}

impl core::Ptr<crate::objdetect::QRCodeEncoder> {
	#[inline] pub fn as_raw_PtrOfQRCodeEncoder(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfQRCodeEncoder(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::objdetect::QRCodeEncoderTraitConst for core::Ptr<crate::objdetect::QRCodeEncoder> {
	#[inline] fn as_raw_QRCodeEncoder(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::objdetect::QRCodeEncoderTrait for core::Ptr<crate::objdetect::QRCodeEncoder> {
	#[inline] fn as_raw_mut_QRCodeEncoder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::objdetect::QRCodeEncoder> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfQRCodeEncoder")
			.finish()
	}
}

