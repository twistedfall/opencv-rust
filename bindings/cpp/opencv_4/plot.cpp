#include "common.hpp"
#include <opencv2/plot.hpp>
#include "plot_types.hpp"

extern "C" {
	Result_void cv_plot_Plot2d_setMinX_double(void* instance, double _plotMinX) {
		try {
			reinterpret_cast<cv::plot::Plot2d*>(instance)->setMinX(_plotMinX);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_plot_Plot2d_setMinY_double(void* instance, double _plotMinY) {
		try {
			reinterpret_cast<cv::plot::Plot2d*>(instance)->setMinY(_plotMinY);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_plot_Plot2d_setMaxX_double(void* instance, double _plotMaxX) {
		try {
			reinterpret_cast<cv::plot::Plot2d*>(instance)->setMaxX(_plotMaxX);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_plot_Plot2d_setMaxY_double(void* instance, double _plotMaxY) {
		try {
			reinterpret_cast<cv::plot::Plot2d*>(instance)->setMaxY(_plotMaxY);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_plot_Plot2d_setPlotLineWidth_int(void* instance, int _plotLineWidth) {
		try {
			reinterpret_cast<cv::plot::Plot2d*>(instance)->setPlotLineWidth(_plotLineWidth);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_plot_Plot2d_setNeedPlotLine_bool(void* instance, bool _needPlotLine) {
		try {
			reinterpret_cast<cv::plot::Plot2d*>(instance)->setNeedPlotLine(_needPlotLine);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_plot_Plot2d_setPlotLineColor_Scalar(void* instance, cv::Scalar _plotLineColor) {
		try {
			reinterpret_cast<cv::plot::Plot2d*>(instance)->setPlotLineColor(_plotLineColor);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_plot_Plot2d_setPlotBackgroundColor_Scalar(void* instance, cv::Scalar _plotBackgroundColor) {
		try {
			reinterpret_cast<cv::plot::Plot2d*>(instance)->setPlotBackgroundColor(_plotBackgroundColor);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_plot_Plot2d_setPlotAxisColor_Scalar(void* instance, cv::Scalar _plotAxisColor) {
		try {
			reinterpret_cast<cv::plot::Plot2d*>(instance)->setPlotAxisColor(_plotAxisColor);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_plot_Plot2d_setPlotGridColor_Scalar(void* instance, cv::Scalar _plotGridColor) {
		try {
			reinterpret_cast<cv::plot::Plot2d*>(instance)->setPlotGridColor(_plotGridColor);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_plot_Plot2d_setPlotTextColor_Scalar(void* instance, cv::Scalar _plotTextColor) {
		try {
			reinterpret_cast<cv::plot::Plot2d*>(instance)->setPlotTextColor(_plotTextColor);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_plot_Plot2d_setPlotSize_int_int(void* instance, int _plotSizeWidth, int _plotSizeHeight) {
		try {
			reinterpret_cast<cv::plot::Plot2d*>(instance)->setPlotSize(_plotSizeWidth, _plotSizeHeight);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_plot_Plot2d_setShowGrid_bool(void* instance, bool needShowGrid) {
		try {
			reinterpret_cast<cv::plot::Plot2d*>(instance)->setShowGrid(needShowGrid);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_plot_Plot2d_setShowText_bool(void* instance, bool needShowText) {
		try {
			reinterpret_cast<cv::plot::Plot2d*>(instance)->setShowText(needShowText);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_plot_Plot2d_setGridLinesNumber_int(void* instance, int gridLinesNumber) {
		try {
			reinterpret_cast<cv::plot::Plot2d*>(instance)->setGridLinesNumber(gridLinesNumber);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_plot_Plot2d_setInvertOrientation_bool(void* instance, bool _invertOrientation) {
		try {
			reinterpret_cast<cv::plot::Plot2d*>(instance)->setInvertOrientation(_invertOrientation);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_plot_Plot2d_setPointIdxToPrint_int(void* instance, int pointIdx) {
		try {
			reinterpret_cast<cv::plot::Plot2d*>(instance)->setPointIdxToPrint(pointIdx);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_plot_Plot2d_render_const__OutputArrayX(void* instance, void* _plotResult) {
		try {
			reinterpret_cast<cv::plot::Plot2d*>(instance)->render(*reinterpret_cast<const cv::_OutputArray*>(_plotResult));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_plot_Plot2d_create_const__InputArrayX(void* data) {
		try {
			cv::Ptr<cv::plot::Plot2d> ret = cv::plot::Plot2d::create(*reinterpret_cast<const cv::_InputArray*>(data));
			return Ok<void*>(new cv::Ptr<cv::plot::Plot2d>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_plot_Plot2d_create_const__InputArrayX_const__InputArrayX(void* dataX, void* dataY) {
		try {
			cv::Ptr<cv::plot::Plot2d> ret = cv::plot::Plot2d::create(*reinterpret_cast<const cv::_InputArray*>(dataX), *reinterpret_cast<const cv::_InputArray*>(dataY));
			return Ok<void*>(new cv::Ptr<cv::plot::Plot2d>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
}
