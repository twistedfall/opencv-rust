ptr_extern! { crate::objdetect::GridBoard,
	cv_PtrLcv_aruco_GridBoardG_new_null_const, cv_PtrLcv_aruco_GridBoardG_delete, cv_PtrLcv_aruco_GridBoardG_getInnerPtr_const, cv_PtrLcv_aruco_GridBoardG_getInnerPtrMut
}

ptr_extern_ctor! { crate::objdetect::GridBoard, cv_PtrLcv_aruco_GridBoardG_new_const_GridBoard }
impl core::Ptr<crate::objdetect::GridBoard> {
	#[inline] pub fn as_raw_PtrOfGridBoard(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfGridBoard(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::objdetect::GridBoardTraitConst for core::Ptr<crate::objdetect::GridBoard> {
	#[inline] fn as_raw_GridBoard(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::objdetect::GridBoardTrait for core::Ptr<crate::objdetect::GridBoard> {
	#[inline] fn as_raw_mut_GridBoard(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::objdetect::BoardTraitConst for core::Ptr<crate::objdetect::GridBoard> {
	#[inline] fn as_raw_Board(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::objdetect::BoardTrait for core::Ptr<crate::objdetect::GridBoard> {
	#[inline] fn as_raw_mut_Board(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::objdetect::GridBoard>, core::Ptr<crate::objdetect::Board>, cv_PtrLcv_aruco_GridBoardG_to_PtrOfBoard }

impl std::fmt::Debug for core::Ptr<crate::objdetect::GridBoard> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfGridBoard")
			.finish()
	}
}

