//! # Plot function for Mat data
use crate::mod_prelude::*;
use crate::{core, sys, types};
pub mod prelude {
	pub use super::{Plot2dTrait, Plot2dTraitConst};
}

/// Constant methods for [crate::plot::Plot2d]
// Plot2d /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/plot.hpp:65
pub trait Plot2dTraitConst: core::AlgorithmTraitConst {
	fn as_raw_Plot2d(&self) -> *const c_void;

}

/// Mutable methods for [crate::plot::Plot2d]
pub trait Plot2dTrait: core::AlgorithmTrait + crate::plot::Plot2dTraitConst {
	fn as_raw_mut_Plot2d(&mut self) -> *mut c_void;

	// setMinX(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/plot.hpp:69
	// ("cv::plot::Plot2d::setMinX", vec![(pred!(mut, ["_plotMinX"], ["double"]), _)]),
	#[inline]
	fn set_min_x(&mut self, _plot_min_x: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_plot_Plot2d_setMinX_double(self.as_raw_mut_Plot2d(), _plot_min_x, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMinY(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/plot.hpp:70
	// ("cv::plot::Plot2d::setMinY", vec![(pred!(mut, ["_plotMinY"], ["double"]), _)]),
	#[inline]
	fn set_min_y(&mut self, _plot_min_y: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_plot_Plot2d_setMinY_double(self.as_raw_mut_Plot2d(), _plot_min_y, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMaxX(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/plot.hpp:71
	// ("cv::plot::Plot2d::setMaxX", vec![(pred!(mut, ["_plotMaxX"], ["double"]), _)]),
	#[inline]
	fn set_max_x(&mut self, _plot_max_x: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_plot_Plot2d_setMaxX_double(self.as_raw_mut_Plot2d(), _plot_max_x, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setMaxY(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/plot.hpp:72
	// ("cv::plot::Plot2d::setMaxY", vec![(pred!(mut, ["_plotMaxY"], ["double"]), _)]),
	#[inline]
	fn set_max_y(&mut self, _plot_max_y: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_plot_Plot2d_setMaxY_double(self.as_raw_mut_Plot2d(), _plot_max_y, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setPlotLineWidth(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/plot.hpp:73
	// ("cv::plot::Plot2d::setPlotLineWidth", vec![(pred!(mut, ["_plotLineWidth"], ["int"]), _)]),
	#[inline]
	fn set_plot_line_width(&mut self, _plot_line_width: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_plot_Plot2d_setPlotLineWidth_int(self.as_raw_mut_Plot2d(), _plot_line_width, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Switches data visualization mode
	///
	/// ## Parameters
	/// * _needPlotLine: if true then neighbour plot points will be connected by lines.
	/// In other case data will be plotted as a set of standalone points.
	// setNeedPlotLine(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/plot.hpp:80
	// ("cv::plot::Plot2d::setNeedPlotLine", vec![(pred!(mut, ["_needPlotLine"], ["bool"]), _)]),
	#[inline]
	fn set_need_plot_line(&mut self, _need_plot_line: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_plot_Plot2d_setNeedPlotLine_bool(self.as_raw_mut_Plot2d(), _need_plot_line, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setPlotLineColor(Scalar)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/plot.hpp:81
	// ("cv::plot::Plot2d::setPlotLineColor", vec![(pred!(mut, ["_plotLineColor"], ["cv::Scalar"]), _)]),
	#[inline]
	fn set_plot_line_color(&mut self, _plot_line_color: core::Scalar) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_plot_Plot2d_setPlotLineColor_Scalar(self.as_raw_mut_Plot2d(), &_plot_line_color, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setPlotBackgroundColor(Scalar)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/plot.hpp:82
	// ("cv::plot::Plot2d::setPlotBackgroundColor", vec![(pred!(mut, ["_plotBackgroundColor"], ["cv::Scalar"]), _)]),
	#[inline]
	fn set_plot_background_color(&mut self, _plot_background_color: core::Scalar) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_plot_Plot2d_setPlotBackgroundColor_Scalar(self.as_raw_mut_Plot2d(), &_plot_background_color, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setPlotAxisColor(Scalar)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/plot.hpp:83
	// ("cv::plot::Plot2d::setPlotAxisColor", vec![(pred!(mut, ["_plotAxisColor"], ["cv::Scalar"]), _)]),
	#[inline]
	fn set_plot_axis_color(&mut self, _plot_axis_color: core::Scalar) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_plot_Plot2d_setPlotAxisColor_Scalar(self.as_raw_mut_Plot2d(), &_plot_axis_color, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setPlotGridColor(Scalar)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/plot.hpp:84
	// ("cv::plot::Plot2d::setPlotGridColor", vec![(pred!(mut, ["_plotGridColor"], ["cv::Scalar"]), _)]),
	#[inline]
	fn set_plot_grid_color(&mut self, _plot_grid_color: core::Scalar) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_plot_Plot2d_setPlotGridColor_Scalar(self.as_raw_mut_Plot2d(), &_plot_grid_color, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setPlotTextColor(Scalar)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/plot.hpp:85
	// ("cv::plot::Plot2d::setPlotTextColor", vec![(pred!(mut, ["_plotTextColor"], ["cv::Scalar"]), _)]),
	#[inline]
	fn set_plot_text_color(&mut self, _plot_text_color: core::Scalar) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_plot_Plot2d_setPlotTextColor_Scalar(self.as_raw_mut_Plot2d(), &_plot_text_color, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setPlotSize(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/plot.hpp:86
	// ("cv::plot::Plot2d::setPlotSize", vec![(pred!(mut, ["_plotSizeWidth", "_plotSizeHeight"], ["int", "int"]), _)]),
	#[inline]
	fn set_plot_size(&mut self, _plot_size_width: i32, _plot_size_height: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_plot_Plot2d_setPlotSize_int_int(self.as_raw_mut_Plot2d(), _plot_size_width, _plot_size_height, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setShowGrid(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/plot.hpp:87
	// ("cv::plot::Plot2d::setShowGrid", vec![(pred!(mut, ["needShowGrid"], ["bool"]), _)]),
	#[inline]
	fn set_show_grid(&mut self, need_show_grid: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_plot_Plot2d_setShowGrid_bool(self.as_raw_mut_Plot2d(), need_show_grid, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setShowText(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/plot.hpp:88
	// ("cv::plot::Plot2d::setShowText", vec![(pred!(mut, ["needShowText"], ["bool"]), _)]),
	#[inline]
	fn set_show_text(&mut self, need_show_text: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_plot_Plot2d_setShowText_bool(self.as_raw_mut_Plot2d(), need_show_text, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setGridLinesNumber(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/plot.hpp:89
	// ("cv::plot::Plot2d::setGridLinesNumber", vec![(pred!(mut, ["gridLinesNumber"], ["int"]), _)]),
	#[inline]
	fn set_grid_lines_number(&mut self, grid_lines_number: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_plot_Plot2d_setGridLinesNumber_int(self.as_raw_mut_Plot2d(), grid_lines_number, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setInvertOrientation(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/plot.hpp:90
	// ("cv::plot::Plot2d::setInvertOrientation", vec![(pred!(mut, ["_invertOrientation"], ["bool"]), _)]),
	#[inline]
	fn set_invert_orientation(&mut self, _invert_orientation: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_plot_Plot2d_setInvertOrientation_bool(self.as_raw_mut_Plot2d(), _invert_orientation, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Sets the index of a point which coordinates will be printed on the top left corner of the plot (if ShowText flag is true).
	///
	/// ## Parameters
	/// * pointIdx: index of the required point in data array.
	// setPointIdxToPrint(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/plot.hpp:96
	// ("cv::plot::Plot2d::setPointIdxToPrint", vec![(pred!(mut, ["pointIdx"], ["int"]), _)]),
	#[inline]
	fn set_point_idx_to_print(&mut self, point_idx: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_plot_Plot2d_setPointIdxToPrint_int(self.as_raw_mut_Plot2d(), point_idx, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// render(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/plot.hpp:97
	// ("cv::plot::Plot2d::render", vec![(pred!(mut, ["_plotResult"], ["const cv::_OutputArray*"]), _)]),
	#[inline]
	fn render(&mut self, _plot_result: &mut impl ToOutputArray) -> Result<()> {
		output_array_arg!(_plot_result);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_plot_Plot2d_render_const__OutputArrayR(self.as_raw_mut_Plot2d(), _plot_result.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// Plot2d /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/plot.hpp:65
pub struct Plot2d {
	ptr: *mut c_void,
}

opencv_type_boxed! { Plot2d }

impl Drop for Plot2d {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_plot_Plot2d_delete(self.as_raw_mut_Plot2d()) };
	}
}

unsafe impl Send for Plot2d {}

impl core::AlgorithmTraitConst for Plot2d {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for Plot2d {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Plot2d, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::plot::Plot2dTraitConst for Plot2d {
	#[inline] fn as_raw_Plot2d(&self) -> *const c_void { self.as_raw() }
}

impl crate::plot::Plot2dTrait for Plot2d {
	#[inline] fn as_raw_mut_Plot2d(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Plot2d, crate::plot::Plot2dTraitConst, as_raw_Plot2d, crate::plot::Plot2dTrait, as_raw_mut_Plot2d }

impl Plot2d {
	/// Creates Plot2d object
	///
	/// ## Parameters
	/// * data: ![inline formula](https://latex.codecogs.com/png.latex?1xN) or ![inline formula](https://latex.codecogs.com/png.latex?Nx1) matrix containing ![inline formula](https://latex.codecogs.com/png.latex?Y) values of points to plot. ![inline formula](https://latex.codecogs.com/png.latex?X) values
	/// will be equal to indexes of correspondind elements in data matrix.
	// create(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/plot.hpp:105
	// ("cv::plot::Plot2d::create", vec![(pred!(mut, ["data"], ["const cv::_InputArray*"]), _)]),
	#[inline]
	pub fn create(data: &impl ToInputArray) -> Result<core::Ptr<crate::plot::Plot2d>> {
		input_array_arg!(data);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_plot_Plot2d_create_const__InputArrayR(data.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::plot::Plot2d>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Creates Plot2d object
	///
	/// ## Parameters
	/// * dataX: ![inline formula](https://latex.codecogs.com/png.latex?1xN) or ![inline formula](https://latex.codecogs.com/png.latex?Nx1) matrix ![inline formula](https://latex.codecogs.com/png.latex?X) values of points to plot.
	/// * dataY: ![inline formula](https://latex.codecogs.com/png.latex?1xN) or ![inline formula](https://latex.codecogs.com/png.latex?Nx1) matrix containing ![inline formula](https://latex.codecogs.com/png.latex?Y) values of points to plot.
	// create(InputArray, InputArray)(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/plot.hpp:113
	// ("cv::plot::Plot2d::create", vec![(pred!(mut, ["dataX", "dataY"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	#[inline]
	pub fn create_1(data_x: &impl ToInputArray, data_y: &impl ToInputArray) -> Result<core::Ptr<crate::plot::Plot2d>> {
		input_array_arg!(data_x);
		input_array_arg!(data_y);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_plot_Plot2d_create_const__InputArrayR_const__InputArrayR(data_x.as_raw__InputArray(), data_y.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::plot::Plot2d>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { Plot2d, core::Algorithm, cv_plot_Plot2d_to_Algorithm }

impl std::fmt::Debug for Plot2d {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Plot2d")
			.finish()
	}
}
