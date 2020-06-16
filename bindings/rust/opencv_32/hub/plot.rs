#![allow(unused_parens)]
//! # Plot function for Mat data
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::Plot2d };
}

/// Creates Plot2d object
/// 
/// ## Parameters
/// * data: ![inline formula](https://latex.codecogs.com/png.latex?1xN) or ![inline formula](https://latex.codecogs.com/png.latex?Nx1) matrix containing ![inline formula](https://latex.codecogs.com/png.latex?Y) values of points to plot. ![inline formula](https://latex.codecogs.com/png.latex?X) values
/// will be equal to indexes of correspondind elements in data matrix.
pub fn create_plot2d(data: &dyn core::ToInputArray) -> Result<core::Ptr::<dyn crate::plot::Plot2d>> {
	input_array_arg!(data);
	unsafe { sys::cv_plot_createPlot2d_const__InputArrayR(data.as_raw__InputArray()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::plot::Plot2d>::opencv_from_extern(r) } )
}

/// Creates Plot2d object
/// 
/// ## Parameters
/// * dataX: ![inline formula](https://latex.codecogs.com/png.latex?1xN) or ![inline formula](https://latex.codecogs.com/png.latex?Nx1) matrix ![inline formula](https://latex.codecogs.com/png.latex?X) values of points to plot.
/// * dataY: ![inline formula](https://latex.codecogs.com/png.latex?1xN) or ![inline formula](https://latex.codecogs.com/png.latex?Nx1) matrix containing ![inline formula](https://latex.codecogs.com/png.latex?Y) values of points to plot.
pub fn create_plot2d_1(data_x: &dyn core::ToInputArray, data_y: &dyn core::ToInputArray) -> Result<core::Ptr::<dyn crate::plot::Plot2d>> {
	input_array_arg!(data_x);
	input_array_arg!(data_y);
	unsafe { sys::cv_plot_createPlot2d_const__InputArrayR_const__InputArrayR(data_x.as_raw__InputArray(), data_y.as_raw__InputArray()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::plot::Plot2d>::opencv_from_extern(r) } )
}

pub trait Plot2d: core::AlgorithmTrait {
	fn as_raw_Plot2d(&self) -> *const c_void;
	fn as_raw_mut_Plot2d(&mut self) -> *mut c_void;

	fn set_min_x(&mut self, _plot_min_x: f64) -> Result<()> {
		unsafe { sys::cv_plot_Plot2d_setMinX_double(self.as_raw_mut_Plot2d(), _plot_min_x) }.into_result()
	}
	
	fn set_min_y(&mut self, _plot_min_y: f64) -> Result<()> {
		unsafe { sys::cv_plot_Plot2d_setMinY_double(self.as_raw_mut_Plot2d(), _plot_min_y) }.into_result()
	}
	
	fn set_max_x(&mut self, _plot_max_x: f64) -> Result<()> {
		unsafe { sys::cv_plot_Plot2d_setMaxX_double(self.as_raw_mut_Plot2d(), _plot_max_x) }.into_result()
	}
	
	fn set_max_y(&mut self, _plot_max_y: f64) -> Result<()> {
		unsafe { sys::cv_plot_Plot2d_setMaxY_double(self.as_raw_mut_Plot2d(), _plot_max_y) }.into_result()
	}
	
	fn set_plot_line_width(&mut self, _plot_line_width: i32) -> Result<()> {
		unsafe { sys::cv_plot_Plot2d_setPlotLineWidth_int(self.as_raw_mut_Plot2d(), _plot_line_width) }.into_result()
	}
	
	/// Switches data visualization mode
	/// 
	/// ## Parameters
	/// * _needPlotLine: if true then neighbour plot points will be connected by lines.
	/// In other case data will be plotted as a set of standalone points.
	fn set_need_plot_line(&mut self, _need_plot_line: bool) -> Result<()> {
		unsafe { sys::cv_plot_Plot2d_setNeedPlotLine_bool(self.as_raw_mut_Plot2d(), _need_plot_line) }.into_result()
	}
	
	fn set_plot_line_color(&mut self, _plot_line_color: core::Scalar) -> Result<()> {
		unsafe { sys::cv_plot_Plot2d_setPlotLineColor_Scalar(self.as_raw_mut_Plot2d(), _plot_line_color.opencv_to_extern()) }.into_result()
	}
	
	fn set_plot_background_color(&mut self, _plot_background_color: core::Scalar) -> Result<()> {
		unsafe { sys::cv_plot_Plot2d_setPlotBackgroundColor_Scalar(self.as_raw_mut_Plot2d(), _plot_background_color.opencv_to_extern()) }.into_result()
	}
	
	fn set_plot_axis_color(&mut self, _plot_axis_color: core::Scalar) -> Result<()> {
		unsafe { sys::cv_plot_Plot2d_setPlotAxisColor_Scalar(self.as_raw_mut_Plot2d(), _plot_axis_color.opencv_to_extern()) }.into_result()
	}
	
	fn set_plot_grid_color(&mut self, _plot_grid_color: core::Scalar) -> Result<()> {
		unsafe { sys::cv_plot_Plot2d_setPlotGridColor_Scalar(self.as_raw_mut_Plot2d(), _plot_grid_color.opencv_to_extern()) }.into_result()
	}
	
	fn set_plot_text_color(&mut self, _plot_text_color: core::Scalar) -> Result<()> {
		unsafe { sys::cv_plot_Plot2d_setPlotTextColor_Scalar(self.as_raw_mut_Plot2d(), _plot_text_color.opencv_to_extern()) }.into_result()
	}
	
	fn set_plot_size(&mut self, _plot_size_width: i32, _plot_size_height: i32) -> Result<()> {
		unsafe { sys::cv_plot_Plot2d_setPlotSize_int_int(self.as_raw_mut_Plot2d(), _plot_size_width, _plot_size_height) }.into_result()
	}
	
	fn render(&mut self, _plot_result: &mut dyn core::ToOutputArray) -> Result<()> {
		output_array_arg!(_plot_result);
		unsafe { sys::cv_plot_Plot2d_render_const__OutputArrayR(self.as_raw_mut_Plot2d(), _plot_result.as_raw__OutputArray()) }.into_result()
	}
	
}
