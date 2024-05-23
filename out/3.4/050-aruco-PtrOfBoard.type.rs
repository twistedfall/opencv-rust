ptr_extern! { crate::aruco::Board,
	cv_PtrLcv_aruco_BoardG_new_null_const, cv_PtrLcv_aruco_BoardG_delete, cv_PtrLcv_aruco_BoardG_getInnerPtr_const, cv_PtrLcv_aruco_BoardG_getInnerPtrMut
}

ptr_extern_ctor! { crate::aruco::Board, cv_PtrLcv_aruco_BoardG_new_const_Board }
impl core::Ptr<crate::aruco::Board> {
	#[inline] pub fn as_raw_PtrOfBoard(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfBoard(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::aruco::BoardTraitConst for core::Ptr<crate::aruco::Board> {
	#[inline] fn as_raw_Board(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::aruco::BoardTrait for core::Ptr<crate::aruco::Board> {
	#[inline] fn as_raw_mut_Board(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::aruco::Board> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfBoard")
			.field("obj_points", &crate::aruco::BoardTraitConst::obj_points(self))
			.field("ids", &crate::aruco::BoardTraitConst::ids(self))
			.field("right_bottom_border", &crate::aruco::BoardTraitConst::right_bottom_border(self))
			.finish()
	}
}

