//! # Plot function for Mat data
use std::os::raw::{c_char, c_void};
use libc::size_t;
use crate::{Error, Result, core, sys, types};


// Generating impl for trait cv::plot::Plot2d (trait)
pub trait Plot2d: core::Algorithm {
    #[inline(always)] fn as_raw_Plot2d(&self) -> *mut c_void;
    fn set_min_x(&mut self, _plot_min_x: f64) -> Result<()> {
        unsafe { sys::cv_plot_Plot2d_setMinX_double(self.as_raw_Plot2d(), _plot_min_x) }.into_result()
    }
    
    fn set_min_y(&mut self, _plot_min_y: f64) -> Result<()> {
        unsafe { sys::cv_plot_Plot2d_setMinY_double(self.as_raw_Plot2d(), _plot_min_y) }.into_result()
    }
    
    fn set_max_x(&mut self, _plot_max_x: f64) -> Result<()> {
        unsafe { sys::cv_plot_Plot2d_setMaxX_double(self.as_raw_Plot2d(), _plot_max_x) }.into_result()
    }
    
    fn set_max_y(&mut self, _plot_max_y: f64) -> Result<()> {
        unsafe { sys::cv_plot_Plot2d_setMaxY_double(self.as_raw_Plot2d(), _plot_max_y) }.into_result()
    }
    
    fn set_plot_line_width(&mut self, _plot_line_width: i32) -> Result<()> {
        unsafe { sys::cv_plot_Plot2d_setPlotLineWidth_int(self.as_raw_Plot2d(), _plot_line_width) }.into_result()
    }
    
    /// Switches data visualization mode
    /// 
    /// ## Parameters
    /// * _needPlotLine: if true then neighbour plot points will be connected by lines.
    /// In other case data will be plotted as a set of standalone points.
    fn set_need_plot_line(&mut self, _need_plot_line: bool) -> Result<()> {
        unsafe { sys::cv_plot_Plot2d_setNeedPlotLine_bool(self.as_raw_Plot2d(), _need_plot_line) }.into_result()
    }
    
    fn set_plot_line_color(&mut self, _plot_line_color: core::Scalar) -> Result<()> {
        unsafe { sys::cv_plot_Plot2d_setPlotLineColor_Scalar(self.as_raw_Plot2d(), _plot_line_color) }.into_result()
    }
    
    fn set_plot_background_color(&mut self, _plot_background_color: core::Scalar) -> Result<()> {
        unsafe { sys::cv_plot_Plot2d_setPlotBackgroundColor_Scalar(self.as_raw_Plot2d(), _plot_background_color) }.into_result()
    }
    
    fn set_plot_axis_color(&mut self, _plot_axis_color: core::Scalar) -> Result<()> {
        unsafe { sys::cv_plot_Plot2d_setPlotAxisColor_Scalar(self.as_raw_Plot2d(), _plot_axis_color) }.into_result()
    }
    
    fn set_plot_grid_color(&mut self, _plot_grid_color: core::Scalar) -> Result<()> {
        unsafe { sys::cv_plot_Plot2d_setPlotGridColor_Scalar(self.as_raw_Plot2d(), _plot_grid_color) }.into_result()
    }
    
    fn set_plot_text_color(&mut self, _plot_text_color: core::Scalar) -> Result<()> {
        unsafe { sys::cv_plot_Plot2d_setPlotTextColor_Scalar(self.as_raw_Plot2d(), _plot_text_color) }.into_result()
    }
    
    fn set_plot_size(&mut self, _plot_size_width: i32, _plot_size_height: i32) -> Result<()> {
        unsafe { sys::cv_plot_Plot2d_setPlotSize_int_int(self.as_raw_Plot2d(), _plot_size_width, _plot_size_height) }.into_result()
    }
    
    fn set_show_grid(&mut self, need_show_grid: bool) -> Result<()> {
        unsafe { sys::cv_plot_Plot2d_setShowGrid_bool(self.as_raw_Plot2d(), need_show_grid) }.into_result()
    }
    
    fn set_show_text(&mut self, need_show_text: bool) -> Result<()> {
        unsafe { sys::cv_plot_Plot2d_setShowText_bool(self.as_raw_Plot2d(), need_show_text) }.into_result()
    }
    
    fn set_grid_lines_number(&mut self, grid_lines_number: i32) -> Result<()> {
        unsafe { sys::cv_plot_Plot2d_setGridLinesNumber_int(self.as_raw_Plot2d(), grid_lines_number) }.into_result()
    }
    
    fn set_invert_orientation(&mut self, _invert_orientation: bool) -> Result<()> {
        unsafe { sys::cv_plot_Plot2d_setInvertOrientation_bool(self.as_raw_Plot2d(), _invert_orientation) }.into_result()
    }
    
    /// Sets the index of a point which coordinates will be printed on the top left corner of the plot (if ShowText flag is true).
    /// 
    /// ## Parameters
    /// * pointIdx: index of the required point in data array.
    fn set_point_idx_to_print(&mut self, point_idx: i32) -> Result<()> {
        unsafe { sys::cv_plot_Plot2d_setPointIdxToPrint_int(self.as_raw_Plot2d(), point_idx) }.into_result()
    }
    
    fn render(&mut self, _plot_result: &mut core::Mat) -> Result<()> {
        unsafe { sys::cv_plot_Plot2d_render_Mat(self.as_raw_Plot2d(), _plot_result.as_raw_Mat()) }.into_result()
    }
    
}

impl<'a> Plot2d + 'a {

    /// Creates Plot2d object
    /// 
    /// ## Parameters
    /// * data: <span lang='latex'>1xN</span> or <span lang='latex'>Nx1</span> matrix containing <span lang='latex'>Y</span> values of points to plot. <span lang='latex'>X</span> values
    /// will be equal to indexes of correspondind elements in data matrix.
    pub fn create(data: &core::Mat) -> Result<types::PtrOfPlot2d> {
        unsafe { sys::cv_plot_Plot2d_create_Mat(data.as_raw_Mat()) }.into_result().map(|ptr| types::PtrOfPlot2d { ptr })
    }
    
    /// Creates Plot2d object
    /// 
    /// ## Parameters
    /// * dataX: <span lang='latex'>1xN</span> or <span lang='latex'>Nx1</span> matrix <span lang='latex'>X</span> values of points to plot.
    /// * dataY: <span lang='latex'>1xN</span> or <span lang='latex'>Nx1</span> matrix containing <span lang='latex'>Y</span> values of points to plot.
    pub fn create_1(data_x: &core::Mat, data_y: &core::Mat) -> Result<types::PtrOfPlot2d> {
        unsafe { sys::cv_plot_Plot2d_create_Mat_Mat(data_x.as_raw_Mat(), data_y.as_raw_Mat()) }.into_result().map(|ptr| types::PtrOfPlot2d { ptr })
    }
    
}

