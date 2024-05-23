#include "ocvrs_common.hpp"
#include <opencv2/plot.hpp>
#include "plot_types.hpp"

extern "C" {
	// setMinX(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/plot.hpp:69
	// ("cv::plot::Plot2d::setMinX", vec![(pred!(mut, ["_plotMinX"], ["double"]), _)]),
	void cv_plot_Plot2d_setMinX_double(cv::plot::Plot2d* instance, double _plotMinX, ResultVoid* ocvrs_return) {
		try {
			instance->setMinX(_plotMinX);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMinY(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/plot.hpp:70
	// ("cv::plot::Plot2d::setMinY", vec![(pred!(mut, ["_plotMinY"], ["double"]), _)]),
	void cv_plot_Plot2d_setMinY_double(cv::plot::Plot2d* instance, double _plotMinY, ResultVoid* ocvrs_return) {
		try {
			instance->setMinY(_plotMinY);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxX(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/plot.hpp:71
	// ("cv::plot::Plot2d::setMaxX", vec![(pred!(mut, ["_plotMaxX"], ["double"]), _)]),
	void cv_plot_Plot2d_setMaxX_double(cv::plot::Plot2d* instance, double _plotMaxX, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxX(_plotMaxX);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxY(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/plot.hpp:72
	// ("cv::plot::Plot2d::setMaxY", vec![(pred!(mut, ["_plotMaxY"], ["double"]), _)]),
	void cv_plot_Plot2d_setMaxY_double(cv::plot::Plot2d* instance, double _plotMaxY, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxY(_plotMaxY);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setPlotLineWidth(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/plot.hpp:73
	// ("cv::plot::Plot2d::setPlotLineWidth", vec![(pred!(mut, ["_plotLineWidth"], ["int"]), _)]),
	void cv_plot_Plot2d_setPlotLineWidth_int(cv::plot::Plot2d* instance, int _plotLineWidth, ResultVoid* ocvrs_return) {
		try {
			instance->setPlotLineWidth(_plotLineWidth);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNeedPlotLine(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/plot.hpp:80
	// ("cv::plot::Plot2d::setNeedPlotLine", vec![(pred!(mut, ["_needPlotLine"], ["bool"]), _)]),
	void cv_plot_Plot2d_setNeedPlotLine_bool(cv::plot::Plot2d* instance, bool _needPlotLine, ResultVoid* ocvrs_return) {
		try {
			instance->setNeedPlotLine(_needPlotLine);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setPlotLineColor(Scalar)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/plot.hpp:81
	// ("cv::plot::Plot2d::setPlotLineColor", vec![(pred!(mut, ["_plotLineColor"], ["cv::Scalar"]), _)]),
	void cv_plot_Plot2d_setPlotLineColor_Scalar(cv::plot::Plot2d* instance, cv::Scalar* _plotLineColor, ResultVoid* ocvrs_return) {
		try {
			instance->setPlotLineColor(*_plotLineColor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setPlotBackgroundColor(Scalar)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/plot.hpp:82
	// ("cv::plot::Plot2d::setPlotBackgroundColor", vec![(pred!(mut, ["_plotBackgroundColor"], ["cv::Scalar"]), _)]),
	void cv_plot_Plot2d_setPlotBackgroundColor_Scalar(cv::plot::Plot2d* instance, cv::Scalar* _plotBackgroundColor, ResultVoid* ocvrs_return) {
		try {
			instance->setPlotBackgroundColor(*_plotBackgroundColor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setPlotAxisColor(Scalar)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/plot.hpp:83
	// ("cv::plot::Plot2d::setPlotAxisColor", vec![(pred!(mut, ["_plotAxisColor"], ["cv::Scalar"]), _)]),
	void cv_plot_Plot2d_setPlotAxisColor_Scalar(cv::plot::Plot2d* instance, cv::Scalar* _plotAxisColor, ResultVoid* ocvrs_return) {
		try {
			instance->setPlotAxisColor(*_plotAxisColor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setPlotGridColor(Scalar)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/plot.hpp:84
	// ("cv::plot::Plot2d::setPlotGridColor", vec![(pred!(mut, ["_plotGridColor"], ["cv::Scalar"]), _)]),
	void cv_plot_Plot2d_setPlotGridColor_Scalar(cv::plot::Plot2d* instance, cv::Scalar* _plotGridColor, ResultVoid* ocvrs_return) {
		try {
			instance->setPlotGridColor(*_plotGridColor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setPlotTextColor(Scalar)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/plot.hpp:85
	// ("cv::plot::Plot2d::setPlotTextColor", vec![(pred!(mut, ["_plotTextColor"], ["cv::Scalar"]), _)]),
	void cv_plot_Plot2d_setPlotTextColor_Scalar(cv::plot::Plot2d* instance, cv::Scalar* _plotTextColor, ResultVoid* ocvrs_return) {
		try {
			instance->setPlotTextColor(*_plotTextColor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setPlotSize(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/plot.hpp:86
	// ("cv::plot::Plot2d::setPlotSize", vec![(pred!(mut, ["_plotSizeWidth", "_plotSizeHeight"], ["int", "int"]), _)]),
	void cv_plot_Plot2d_setPlotSize_int_int(cv::plot::Plot2d* instance, int _plotSizeWidth, int _plotSizeHeight, ResultVoid* ocvrs_return) {
		try {
			instance->setPlotSize(_plotSizeWidth, _plotSizeHeight);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setShowGrid(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/plot.hpp:87
	// ("cv::plot::Plot2d::setShowGrid", vec![(pred!(mut, ["needShowGrid"], ["bool"]), _)]),
	void cv_plot_Plot2d_setShowGrid_bool(cv::plot::Plot2d* instance, bool needShowGrid, ResultVoid* ocvrs_return) {
		try {
			instance->setShowGrid(needShowGrid);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setShowText(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/plot.hpp:88
	// ("cv::plot::Plot2d::setShowText", vec![(pred!(mut, ["needShowText"], ["bool"]), _)]),
	void cv_plot_Plot2d_setShowText_bool(cv::plot::Plot2d* instance, bool needShowText, ResultVoid* ocvrs_return) {
		try {
			instance->setShowText(needShowText);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setGridLinesNumber(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/plot.hpp:89
	// ("cv::plot::Plot2d::setGridLinesNumber", vec![(pred!(mut, ["gridLinesNumber"], ["int"]), _)]),
	void cv_plot_Plot2d_setGridLinesNumber_int(cv::plot::Plot2d* instance, int gridLinesNumber, ResultVoid* ocvrs_return) {
		try {
			instance->setGridLinesNumber(gridLinesNumber);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setInvertOrientation(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/plot.hpp:90
	// ("cv::plot::Plot2d::setInvertOrientation", vec![(pred!(mut, ["_invertOrientation"], ["bool"]), _)]),
	void cv_plot_Plot2d_setInvertOrientation_bool(cv::plot::Plot2d* instance, bool _invertOrientation, ResultVoid* ocvrs_return) {
		try {
			instance->setInvertOrientation(_invertOrientation);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setPointIdxToPrint(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/plot.hpp:96
	// ("cv::plot::Plot2d::setPointIdxToPrint", vec![(pred!(mut, ["pointIdx"], ["int"]), _)]),
	void cv_plot_Plot2d_setPointIdxToPrint_int(cv::plot::Plot2d* instance, int pointIdx, ResultVoid* ocvrs_return) {
		try {
			instance->setPointIdxToPrint(pointIdx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// render(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/plot.hpp:97
	// ("cv::plot::Plot2d::render", vec![(pred!(mut, ["_plotResult"], ["const cv::_OutputArray*"]), _)]),
	void cv_plot_Plot2d_render_const__OutputArrayR(cv::plot::Plot2d* instance, const cv::_OutputArray* _plotResult, ResultVoid* ocvrs_return) {
		try {
			instance->render(*_plotResult);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/plot.hpp:105
	// ("cv::plot::Plot2d::create", vec![(pred!(mut, ["data"], ["const cv::_InputArray*"]), _)]),
	void cv_plot_Plot2d_create_const__InputArrayR(const cv::_InputArray* data, Result<cv::Ptr<cv::plot::Plot2d>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::plot::Plot2d> ret = cv::plot::Plot2d::create(*data);
			Ok(new cv::Ptr<cv::plot::Plot2d>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(InputArray, InputArray)(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/plot.hpp:113
	// ("cv::plot::Plot2d::create", vec![(pred!(mut, ["dataX", "dataY"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_plot_Plot2d_create_const__InputArrayR_const__InputArrayR(const cv::_InputArray* dataX, const cv::_InputArray* dataY, Result<cv::Ptr<cv::plot::Plot2d>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::plot::Plot2d> ret = cv::plot::Plot2d::create(*dataX, *dataY);
			Ok(new cv::Ptr<cv::plot::Plot2d>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::plot::Plot2d::to_Algorithm() generated
	// ("cv::plot::Plot2d::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_plot_Plot2d_to_Algorithm(cv::plot::Plot2d* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::plot::Plot2d::delete() generated
	// ("cv::plot::Plot2d::delete", vec![(pred!(mut, [], []), _)]),
	void cv_plot_Plot2d_delete(cv::plot::Plot2d* instance) {
			delete instance;
	}

}
