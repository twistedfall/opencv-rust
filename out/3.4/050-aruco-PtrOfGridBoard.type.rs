ptr_extern! { crate::aruco::GridBoard,
	cv_PtrLcv_aruco_GridBoardG_new_null_const, cv_PtrLcv_aruco_GridBoardG_delete, cv_PtrLcv_aruco_GridBoardG_getInnerPtr_const, cv_PtrLcv_aruco_GridBoardG_getInnerPtrMut
}

ptr_extern_ctor! { crate::aruco::GridBoard, cv_PtrLcv_aruco_GridBoardG_new_const_GridBoard }
impl core::Ptr<crate::aruco::GridBoard> {
	#[inline] pub fn as_raw_PtrOfGridBoard(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfGridBoard(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::aruco::GridBoardTraitConst for core::Ptr<crate::aruco::GridBoard> {
	#[inline] fn as_raw_GridBoard(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::aruco::GridBoardTrait for core::Ptr<crate::aruco::GridBoard> {
	#[inline] fn as_raw_mut_GridBoard(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::aruco::BoardTraitConst for core::Ptr<crate::aruco::GridBoard> {
	#[inline] fn as_raw_Board(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::aruco::BoardTrait for core::Ptr<crate::aruco::GridBoard> {
	#[inline] fn as_raw_mut_Board(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::aruco::GridBoard>, core::Ptr<crate::aruco::Board>, cv_PtrLcv_aruco_GridBoardG_to_PtrOfBoard }

impl std::fmt::Debug for core::Ptr<crate::aruco::GridBoard> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfGridBoard")
			.field("obj_points", &crate::aruco::BoardTraitConst::obj_points(self))
			.field("ids", &crate::aruco::BoardTraitConst::ids(self))
			.field("right_bottom_border", &crate::aruco::BoardTraitConst::right_bottom_border(self))
			.finish()
	}
}

