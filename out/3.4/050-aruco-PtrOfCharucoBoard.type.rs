ptr_extern! { crate::aruco::CharucoBoard,
	cv_PtrLcv_aruco_CharucoBoardG_new_null_const, cv_PtrLcv_aruco_CharucoBoardG_delete, cv_PtrLcv_aruco_CharucoBoardG_getInnerPtr_const, cv_PtrLcv_aruco_CharucoBoardG_getInnerPtrMut
}

ptr_extern_ctor! { crate::aruco::CharucoBoard, cv_PtrLcv_aruco_CharucoBoardG_new_const_CharucoBoard }
impl core::Ptr<crate::aruco::CharucoBoard> {
	#[inline] pub fn as_raw_PtrOfCharucoBoard(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfCharucoBoard(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::aruco::CharucoBoardTraitConst for core::Ptr<crate::aruco::CharucoBoard> {
	#[inline] fn as_raw_CharucoBoard(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::aruco::CharucoBoardTrait for core::Ptr<crate::aruco::CharucoBoard> {
	#[inline] fn as_raw_mut_CharucoBoard(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::aruco::BoardTraitConst for core::Ptr<crate::aruco::CharucoBoard> {
	#[inline] fn as_raw_Board(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::aruco::BoardTrait for core::Ptr<crate::aruco::CharucoBoard> {
	#[inline] fn as_raw_mut_Board(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::aruco::CharucoBoard>, core::Ptr<crate::aruco::Board>, cv_PtrLcv_aruco_CharucoBoardG_to_PtrOfBoard }

impl std::fmt::Debug for core::Ptr<crate::aruco::CharucoBoard> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfCharucoBoard")
			.field("chessboard_corners", &crate::aruco::CharucoBoardTraitConst::chessboard_corners(self))
			.field("nearest_marker_idx", &crate::aruco::CharucoBoardTraitConst::nearest_marker_idx(self))
			.field("nearest_marker_corners", &crate::aruco::CharucoBoardTraitConst::nearest_marker_corners(self))
			.field("obj_points", &crate::aruco::BoardTraitConst::obj_points(self))
			.field("ids", &crate::aruco::BoardTraitConst::ids(self))
			.field("right_bottom_border", &crate::aruco::BoardTraitConst::right_bottom_border(self))
			.finish()
	}
}

