ptr_extern! { crate::objdetect::CharucoBoard,
	cv_PtrLcv_aruco_CharucoBoardG_new_null_const, cv_PtrLcv_aruco_CharucoBoardG_delete, cv_PtrLcv_aruco_CharucoBoardG_getInnerPtr_const, cv_PtrLcv_aruco_CharucoBoardG_getInnerPtrMut
}

ptr_extern_ctor! { crate::objdetect::CharucoBoard, cv_PtrLcv_aruco_CharucoBoardG_new_const_CharucoBoard }
impl core::Ptr<crate::objdetect::CharucoBoard> {
	#[inline] pub fn as_raw_PtrOfCharucoBoard(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfCharucoBoard(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::objdetect::CharucoBoardTraitConst for core::Ptr<crate::objdetect::CharucoBoard> {
	#[inline] fn as_raw_CharucoBoard(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::objdetect::CharucoBoardTrait for core::Ptr<crate::objdetect::CharucoBoard> {
	#[inline] fn as_raw_mut_CharucoBoard(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::objdetect::BoardTraitConst for core::Ptr<crate::objdetect::CharucoBoard> {
	#[inline] fn as_raw_Board(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::objdetect::BoardTrait for core::Ptr<crate::objdetect::CharucoBoard> {
	#[inline] fn as_raw_mut_Board(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::objdetect::CharucoBoard>, core::Ptr<crate::objdetect::Board>, cv_PtrLcv_aruco_CharucoBoardG_to_PtrOfBoard }

impl std::fmt::Debug for core::Ptr<crate::objdetect::CharucoBoard> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfCharucoBoard")
			.finish()
	}
}

