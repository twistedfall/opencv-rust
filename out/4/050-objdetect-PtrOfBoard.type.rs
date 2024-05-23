ptr_extern! { crate::objdetect::Board,
	cv_PtrLcv_aruco_BoardG_new_null_const, cv_PtrLcv_aruco_BoardG_delete, cv_PtrLcv_aruco_BoardG_getInnerPtr_const, cv_PtrLcv_aruco_BoardG_getInnerPtrMut
}

ptr_extern_ctor! { crate::objdetect::Board, cv_PtrLcv_aruco_BoardG_new_const_Board }
impl core::Ptr<crate::objdetect::Board> {
	#[inline] pub fn as_raw_PtrOfBoard(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfBoard(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::objdetect::BoardTraitConst for core::Ptr<crate::objdetect::Board> {
	#[inline] fn as_raw_Board(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::objdetect::BoardTrait for core::Ptr<crate::objdetect::Board> {
	#[inline] fn as_raw_mut_Board(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::objdetect::Board> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfBoard")
			.finish()
	}
}

