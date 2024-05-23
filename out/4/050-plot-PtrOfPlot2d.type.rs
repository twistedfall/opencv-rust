ptr_extern! { crate::plot::Plot2d,
	cv_PtrLcv_plot_Plot2dG_new_null_const, cv_PtrLcv_plot_Plot2dG_delete, cv_PtrLcv_plot_Plot2dG_getInnerPtr_const, cv_PtrLcv_plot_Plot2dG_getInnerPtrMut
}

impl core::Ptr<crate::plot::Plot2d> {
	#[inline] pub fn as_raw_PtrOfPlot2d(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfPlot2d(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::plot::Plot2dTraitConst for core::Ptr<crate::plot::Plot2d> {
	#[inline] fn as_raw_Plot2d(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::plot::Plot2dTrait for core::Ptr<crate::plot::Plot2d> {
	#[inline] fn as_raw_mut_Plot2d(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::plot::Plot2d> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::plot::Plot2d> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::plot::Plot2d>, core::Ptr<core::Algorithm>, cv_PtrLcv_plot_Plot2dG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::plot::Plot2d> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfPlot2d")
			.finish()
	}
}

