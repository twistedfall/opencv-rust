#include "imgproc.hpp"
#include "imgproc_types.hpp"

extern "C" {
	Result_void cv_Canny_const__InputArrayX_const__InputArrayX_const__OutputArrayX_double_double_bool(void* dx, void* dy, void* edges, double threshold1, double threshold2, bool L2gradient) {
		try {
			cv::Canny(*reinterpret_cast<const cv::_InputArray*>(dx), *reinterpret_cast<const cv::_InputArray*>(dy), *reinterpret_cast<const cv::_OutputArray*>(edges), threshold1, threshold2, L2gradient);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_Canny_const__InputArrayX_const__OutputArrayX_double_double_int_bool(void* image, void* edges, double threshold1, double threshold2, int apertureSize, bool L2gradient) {
		try {
			cv::Canny(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<const cv::_OutputArray*>(edges), threshold1, threshold2, apertureSize, L2gradient);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_EMD_const__InputArrayX_const__InputArrayX_int_const__InputArrayX_floatX_const__OutputArrayX(void* signature1, void* signature2, int distType, void* cost, float* lowerBound, void* flow) {
		try {
			float ret = cv::EMD(*reinterpret_cast<const cv::_InputArray*>(signature1), *reinterpret_cast<const cv::_InputArray*>(signature2), distType, *reinterpret_cast<const cv::_InputArray*>(cost), lowerBound, *reinterpret_cast<const cv::_OutputArray*>(flow));
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_GaussianBlur_const__InputArrayX_const__OutputArrayX_Size_double_double_int(void* src, void* dst, cv::Size ksize, double sigmaX, double sigmaY, int borderType) {
		try {
			cv::GaussianBlur(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), ksize, sigmaX, sigmaY, borderType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_HoughCircles_const__InputArrayX_const__OutputArrayX_int_double_double_double_double_int_int(void* image, void* circles, int method, double dp, double minDist, double param1, double param2, int minRadius, int maxRadius) {
		try {
			cv::HoughCircles(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<const cv::_OutputArray*>(circles), method, dp, minDist, param1, param2, minRadius, maxRadius);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_HoughLinesP_const__InputArrayX_const__OutputArrayX_double_double_int_double_double(void* image, void* lines, double rho, double theta, int threshold, double minLineLength, double maxLineGap) {
		try {
			cv::HoughLinesP(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<const cv::_OutputArray*>(lines), rho, theta, threshold, minLineLength, maxLineGap);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_HoughLinesPointSet_const__InputArrayX_const__OutputArrayX_int_int_double_double_double_double_double_double(void* _point, void* _lines, int lines_max, int threshold, double min_rho, double max_rho, double rho_step, double min_theta, double max_theta, double theta_step) {
		try {
			cv::HoughLinesPointSet(*reinterpret_cast<const cv::_InputArray*>(_point), *reinterpret_cast<const cv::_OutputArray*>(_lines), lines_max, threshold, min_rho, max_rho, rho_step, min_theta, max_theta, theta_step);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_HoughLines_const__InputArrayX_const__OutputArrayX_double_double_int_double_double_double_double(void* image, void* lines, double rho, double theta, int threshold, double srn, double stn, double min_theta, double max_theta) {
		try {
			cv::HoughLines(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<const cv::_OutputArray*>(lines), rho, theta, threshold, srn, stn, min_theta, max_theta);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_HuMoments_const_MomentsX_const__OutputArrayX(const cv::Moments* m, void* hu) {
		try {
			cv::HuMoments(*m, *reinterpret_cast<const cv::_OutputArray*>(hu));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_HuMoments_const_MomentsX_double_X__7_(const cv::Moments* moments, double(*hu)[7]) {
		try {
			cv::HuMoments(*moments, *hu);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_Laplacian_const__InputArrayX_const__OutputArrayX_int_int_double_double_int(void* src, void* dst, int ddepth, int ksize, double scale, double delta, int borderType) {
		try {
			cv::Laplacian(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), ddepth, ksize, scale, delta, borderType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_Scharr_const__InputArrayX_const__OutputArrayX_int_int_int_double_double_int(void* src, void* dst, int ddepth, int dx, int dy, double scale, double delta, int borderType) {
		try {
			cv::Scharr(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), ddepth, dx, dy, scale, delta, borderType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_Sobel_const__InputArrayX_const__OutputArrayX_int_int_int_int_double_double_int(void* src, void* dst, int ddepth, int dx, int dy, int ksize, double scale, double delta, int borderType) {
		try {
			cv::Sobel(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), ddepth, dx, dy, ksize, scale, delta, borderType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_accumulateProduct_const__InputArrayX_const__InputArrayX_const__InputOutputArrayX_const__InputArrayX(void* src1, void* src2, void* dst, void* mask) {
		try {
			cv::accumulateProduct(*reinterpret_cast<const cv::_InputArray*>(src1), *reinterpret_cast<const cv::_InputArray*>(src2), *reinterpret_cast<const cv::_InputOutputArray*>(dst), *reinterpret_cast<const cv::_InputArray*>(mask));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_accumulateSquare_const__InputArrayX_const__InputOutputArrayX_const__InputArrayX(void* src, void* dst, void* mask) {
		try {
			cv::accumulateSquare(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_InputOutputArray*>(dst), *reinterpret_cast<const cv::_InputArray*>(mask));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_accumulateWeighted_const__InputArrayX_const__InputOutputArrayX_double_const__InputArrayX(void* src, void* dst, double alpha, void* mask) {
		try {
			cv::accumulateWeighted(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_InputOutputArray*>(dst), alpha, *reinterpret_cast<const cv::_InputArray*>(mask));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_accumulate_const__InputArrayX_const__InputOutputArrayX_const__InputArrayX(void* src, void* dst, void* mask) {
		try {
			cv::accumulate(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_InputOutputArray*>(dst), *reinterpret_cast<const cv::_InputArray*>(mask));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_adaptiveThreshold_const__InputArrayX_const__OutputArrayX_double_int_int_int_double(void* src, void* dst, double maxValue, int adaptiveMethod, int thresholdType, int blockSize, double C) {
		try {
			cv::adaptiveThreshold(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), maxValue, adaptiveMethod, thresholdType, blockSize, C);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_applyColorMap_const__InputArrayX_const__OutputArrayX_const__InputArrayX(void* src, void* dst, void* userColor) {
		try {
			cv::applyColorMap(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), *reinterpret_cast<const cv::_InputArray*>(userColor));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_applyColorMap_const__InputArrayX_const__OutputArrayX_int(void* src, void* dst, int colormap) {
		try {
			cv::applyColorMap(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), colormap);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_approxPolyDP_const__InputArrayX_const__OutputArrayX_double_bool(void* curve, void* approxCurve, double epsilon, bool closed) {
		try {
			cv::approxPolyDP(*reinterpret_cast<const cv::_InputArray*>(curve), *reinterpret_cast<const cv::_OutputArray*>(approxCurve), epsilon, closed);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_arcLength_const__InputArrayX_bool(void* curve, bool closed) {
		try {
			double ret = cv::arcLength(*reinterpret_cast<const cv::_InputArray*>(curve), closed);
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_arrowedLine_const__InputOutputArrayX_Point_Point_const_ScalarX_int_int_int_double(void* img, cv::Point pt1, cv::Point pt2, const cv::Scalar* color, int thickness, int line_type, int shift, double tipLength) {
		try {
			cv::arrowedLine(*reinterpret_cast<const cv::_InputOutputArray*>(img), pt1, pt2, *color, thickness, line_type, shift, tipLength);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bilateralFilter_const__InputArrayX_const__OutputArrayX_int_double_double_int(void* src, void* dst, int d, double sigmaColor, double sigmaSpace, int borderType) {
		try {
			cv::bilateralFilter(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), d, sigmaColor, sigmaSpace, borderType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_blendLinear_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__OutputArrayX(void* src1, void* src2, void* weights1, void* weights2, void* dst) {
		try {
			cv::blendLinear(*reinterpret_cast<const cv::_InputArray*>(src1), *reinterpret_cast<const cv::_InputArray*>(src2), *reinterpret_cast<const cv::_InputArray*>(weights1), *reinterpret_cast<const cv::_InputArray*>(weights2), *reinterpret_cast<const cv::_OutputArray*>(dst));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_blur_const__InputArrayX_const__OutputArrayX_Size_Point_int(void* src, void* dst, cv::Size ksize, cv::Point anchor, int borderType) {
		try {
			cv::blur(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), ksize, anchor, borderType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Rect> cv_boundingRect_const__InputArrayX(void* array) {
		try {
			cv::Rect ret = cv::boundingRect(*reinterpret_cast<const cv::_InputArray*>(array));
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(Result<cv::Rect>)
	}
	
	Result_void cv_boxFilter_const__InputArrayX_const__OutputArrayX_int_Size_Point_bool_int(void* src, void* dst, int ddepth, cv::Size ksize, cv::Point anchor, bool normalize, int borderType) {
		try {
			cv::boxFilter(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), ddepth, ksize, anchor, normalize, borderType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_boxPoints_RotatedRect_const__OutputArrayX(void* box, void* points) {
		try {
			cv::boxPoints(*reinterpret_cast<cv::RotatedRect*>(box), *reinterpret_cast<const cv::_OutputArray*>(points));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_buildPyramid_const__InputArrayX_const__OutputArrayX_int_int(void* src, void* dst, int maxlevel, int borderType) {
		try {
			cv::buildPyramid(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), maxlevel, borderType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_calcBackProject_const__InputArrayX_const_vector_int_X_const__InputArrayX_const__OutputArrayX_const_vector_float_X_double(void* images, void* channels, void* hist, void* dst, void* ranges, double scale) {
		try {
			cv::calcBackProject(*reinterpret_cast<const cv::_InputArray*>(images), *reinterpret_cast<const std::vector<int>*>(channels), *reinterpret_cast<const cv::_InputArray*>(hist), *reinterpret_cast<const cv::_OutputArray*>(dst), *reinterpret_cast<const std::vector<float>*>(ranges), scale);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_calcHist_const__InputArrayX_const_vector_int_X_const__InputArrayX_const__OutputArrayX_const_vector_int_X_const_vector_float_X_bool(void* images, void* channels, void* mask, void* hist, void* histSize, void* ranges, bool accumulate) {
		try {
			cv::calcHist(*reinterpret_cast<const cv::_InputArray*>(images), *reinterpret_cast<const std::vector<int>*>(channels), *reinterpret_cast<const cv::_InputArray*>(mask), *reinterpret_cast<const cv::_OutputArray*>(hist), *reinterpret_cast<const std::vector<int>*>(histSize), *reinterpret_cast<const std::vector<float>*>(ranges), accumulate);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_circle_const__InputOutputArrayX_Point_int_const_ScalarX_int_int_int(void* img, cv::Point center, int radius, const cv::Scalar* color, int thickness, int lineType, int shift) {
		try {
			cv::circle(*reinterpret_cast<const cv::_InputOutputArray*>(img), center, radius, *color, thickness, lineType, shift);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_clipLine_Rect_PointX_PointX(cv::Rect imgRect, cv::Point* pt1, cv::Point* pt2) {
		try {
			bool ret = cv::clipLine(imgRect, *pt1, *pt2);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_clipLine_Size2l_Point2lX_Point2lX(cv::Size2l imgSize, cv::Point2l* pt1, cv::Point2l* pt2) {
		try {
			bool ret = cv::clipLine(imgSize, *pt1, *pt2);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_clipLine_Size_PointX_PointX(cv::Size imgSize, cv::Point* pt1, cv::Point* pt2) {
		try {
			bool ret = cv::clipLine(imgSize, *pt1, *pt2);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<double> cv_compareHist_const_SparseMatX_const_SparseMatX_int(void* H1, void* H2, int method) {
		try {
			double ret = cv::compareHist(*reinterpret_cast<const cv::SparseMat*>(H1), *reinterpret_cast<const cv::SparseMat*>(H2), method);
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result<double> cv_compareHist_const__InputArrayX_const__InputArrayX_int(void* H1, void* H2, int method) {
		try {
			double ret = cv::compareHist(*reinterpret_cast<const cv::_InputArray*>(H1), *reinterpret_cast<const cv::_InputArray*>(H2), method);
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result<int> cv_connectedComponentsWithStats_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_int_int(void* image, void* labels, void* stats, void* centroids, int connectivity, int ltype) {
		try {
			int ret = cv::connectedComponentsWithStats(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<const cv::_OutputArray*>(labels), *reinterpret_cast<const cv::_OutputArray*>(stats), *reinterpret_cast<const cv::_OutputArray*>(centroids), connectivity, ltype);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_connectedComponentsWithStats_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_int_int_int(void* image, void* labels, void* stats, void* centroids, int connectivity, int ltype, int ccltype) {
		try {
			int ret = cv::connectedComponentsWithStats(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<const cv::_OutputArray*>(labels), *reinterpret_cast<const cv::_OutputArray*>(stats), *reinterpret_cast<const cv::_OutputArray*>(centroids), connectivity, ltype, ccltype);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_connectedComponents_const__InputArrayX_const__OutputArrayX_int_int(void* image, void* labels, int connectivity, int ltype) {
		try {
			int ret = cv::connectedComponents(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<const cv::_OutputArray*>(labels), connectivity, ltype);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_connectedComponents_const__InputArrayX_const__OutputArrayX_int_int_int(void* image, void* labels, int connectivity, int ltype, int ccltype) {
		try {
			int ret = cv::connectedComponents(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<const cv::_OutputArray*>(labels), connectivity, ltype, ccltype);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<double> cv_contourArea_const__InputArrayX_bool(void* contour, bool oriented) {
		try {
			double ret = cv::contourArea(*reinterpret_cast<const cv::_InputArray*>(contour), oriented);
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_convertMaps_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_int_bool(void* map1, void* map2, void* dstmap1, void* dstmap2, int dstmap1type, bool nninterpolation) {
		try {
			cv::convertMaps(*reinterpret_cast<const cv::_InputArray*>(map1), *reinterpret_cast<const cv::_InputArray*>(map2), *reinterpret_cast<const cv::_OutputArray*>(dstmap1), *reinterpret_cast<const cv::_OutputArray*>(dstmap2), dstmap1type, nninterpolation);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_convexHull_const__InputArrayX_const__OutputArrayX_bool_bool(void* points, void* hull, bool clockwise, bool returnPoints) {
		try {
			cv::convexHull(*reinterpret_cast<const cv::_InputArray*>(points), *reinterpret_cast<const cv::_OutputArray*>(hull), clockwise, returnPoints);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_convexityDefects_const__InputArrayX_const__InputArrayX_const__OutputArrayX(void* contour, void* convexhull, void* convexityDefects) {
		try {
			cv::convexityDefects(*reinterpret_cast<const cv::_InputArray*>(contour), *reinterpret_cast<const cv::_InputArray*>(convexhull), *reinterpret_cast<const cv::_OutputArray*>(convexityDefects));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_cornerEigenValsAndVecs_const__InputArrayX_const__OutputArrayX_int_int_int(void* src, void* dst, int blockSize, int ksize, int borderType) {
		try {
			cv::cornerEigenValsAndVecs(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), blockSize, ksize, borderType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_cornerHarris_const__InputArrayX_const__OutputArrayX_int_int_double_int(void* src, void* dst, int blockSize, int ksize, double k, int borderType) {
		try {
			cv::cornerHarris(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), blockSize, ksize, k, borderType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_cornerMinEigenVal_const__InputArrayX_const__OutputArrayX_int_int_int(void* src, void* dst, int blockSize, int ksize, int borderType) {
		try {
			cv::cornerMinEigenVal(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), blockSize, ksize, borderType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_cornerSubPix_const__InputArrayX_const__InputOutputArrayX_Size_Size_TermCriteria(void* image, void* corners, cv::Size winSize, cv::Size zeroZone, void* criteria) {
		try {
			cv::cornerSubPix(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<const cv::_InputOutputArray*>(corners), winSize, zeroZone, *reinterpret_cast<cv::TermCriteria*>(criteria));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_createCLAHE_double_Size(double clipLimit, cv::Size tileGridSize) {
		try {
			cv::Ptr<cv::CLAHE> ret = cv::createCLAHE(clipLimit, tileGridSize);
			return Ok<void*>(new cv::Ptr<cv::CLAHE>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_createGeneralizedHoughBallard() {
		try {
			cv::Ptr<cv::GeneralizedHoughBallard> ret = cv::createGeneralizedHoughBallard();
			return Ok<void*>(new cv::Ptr<cv::GeneralizedHoughBallard>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_createGeneralizedHoughGuil() {
		try {
			cv::Ptr<cv::GeneralizedHoughGuil> ret = cv::createGeneralizedHoughGuil();
			return Ok<void*>(new cv::Ptr<cv::GeneralizedHoughGuil>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_createHanningWindow_const__OutputArrayX_Size_int(void* dst, cv::Size winSize, int type) {
		try {
			cv::createHanningWindow(*reinterpret_cast<const cv::_OutputArray*>(dst), winSize, type);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_createLineSegmentDetector_int_double_double_double_double_double_double_int(int _refine, double _scale, double _sigma_scale, double _quant, double _ang_th, double _log_eps, double _density_th, int _n_bins) {
		try {
			cv::Ptr<cv::LineSegmentDetector> ret = cv::createLineSegmentDetector(_refine, _scale, _sigma_scale, _quant, _ang_th, _log_eps, _density_th, _n_bins);
			return Ok<void*>(new cv::Ptr<cv::LineSegmentDetector>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_cvtColorTwoPlane_const__InputArrayX_const__InputArrayX_const__OutputArrayX_int(void* src1, void* src2, void* dst, int code) {
		try {
			cv::cvtColorTwoPlane(*reinterpret_cast<const cv::_InputArray*>(src1), *reinterpret_cast<const cv::_InputArray*>(src2), *reinterpret_cast<const cv::_OutputArray*>(dst), code);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_cvtColor_const__InputArrayX_const__OutputArrayX_int_int(void* src, void* dst, int code, int dstCn) {
		try {
			cv::cvtColor(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), code, dstCn);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_demosaicing_const__InputArrayX_const__OutputArrayX_int_int(void* src, void* dst, int code, int dstCn) {
		try {
			cv::demosaicing(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), code, dstCn);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dilate_const__InputArrayX_const__OutputArrayX_const__InputArrayX_Point_int_int_const_ScalarX(void* src, void* dst, void* kernel, cv::Point anchor, int iterations, int borderType, const cv::Scalar* borderValue) {
		try {
			cv::dilate(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), *reinterpret_cast<const cv::_InputArray*>(kernel), anchor, iterations, borderType, *borderValue);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_distanceTransform_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_int_int_int(void* src, void* dst, void* labels, int distanceType, int maskSize, int labelType) {
		try {
			cv::distanceTransform(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), *reinterpret_cast<const cv::_OutputArray*>(labels), distanceType, maskSize, labelType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_distanceTransform_const__InputArrayX_const__OutputArrayX_int_int_int(void* src, void* dst, int distanceType, int maskSize, int dstType) {
		try {
			cv::distanceTransform(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), distanceType, maskSize, dstType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_drawContours_const__InputOutputArrayX_const__InputArrayX_int_const_ScalarX_int_int_const__InputArrayX_int_Point(void* image, void* contours, int contourIdx, const cv::Scalar* color, int thickness, int lineType, void* hierarchy, int maxLevel, cv::Point offset) {
		try {
			cv::drawContours(*reinterpret_cast<const cv::_InputOutputArray*>(image), *reinterpret_cast<const cv::_InputArray*>(contours), contourIdx, *color, thickness, lineType, *reinterpret_cast<const cv::_InputArray*>(hierarchy), maxLevel, offset);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_drawMarker_const__InputOutputArrayX_Point_const_ScalarX_int_int_int_int(void* img, cv::Point position, const cv::Scalar* color, int markerType, int markerSize, int thickness, int line_type) {
		try {
			cv::drawMarker(*reinterpret_cast<const cv::_InputOutputArray*>(img), position, *color, markerType, markerSize, thickness, line_type);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ellipse2Poly_Point2d_Size2d_int_int_int_int_vector_Point2d_X(cv::Point2d center, cv::Size2d axes, int angle, int arcStart, int arcEnd, int delta, void* pts) {
		try {
			cv::ellipse2Poly(center, axes, angle, arcStart, arcEnd, delta, *reinterpret_cast<std::vector<cv::Point2d>*>(pts));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ellipse2Poly_Point_Size_int_int_int_int_vector_Point_X(cv::Point center, cv::Size axes, int angle, int arcStart, int arcEnd, int delta, void* pts) {
		try {
			cv::ellipse2Poly(center, axes, angle, arcStart, arcEnd, delta, *reinterpret_cast<std::vector<cv::Point>*>(pts));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ellipse_const__InputOutputArrayX_Point_Size_double_double_double_const_ScalarX_int_int_int(void* img, cv::Point center, cv::Size axes, double angle, double startAngle, double endAngle, const cv::Scalar* color, int thickness, int lineType, int shift) {
		try {
			cv::ellipse(*reinterpret_cast<const cv::_InputOutputArray*>(img), center, axes, angle, startAngle, endAngle, *color, thickness, lineType, shift);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ellipse_const__InputOutputArrayX_const_RotatedRectX_const_ScalarX_int_int(void* img, void* box, const cv::Scalar* color, int thickness, int lineType) {
		try {
			cv::ellipse(*reinterpret_cast<const cv::_InputOutputArray*>(img), *reinterpret_cast<const cv::RotatedRect*>(box), *color, thickness, lineType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_equalizeHist_const__InputArrayX_const__OutputArrayX(void* src, void* dst) {
		try {
			cv::equalizeHist(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_erode_const__InputArrayX_const__OutputArrayX_const__InputArrayX_Point_int_int_const_ScalarX(void* src, void* dst, void* kernel, cv::Point anchor, int iterations, int borderType, const cv::Scalar* borderValue) {
		try {
			cv::erode(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), *reinterpret_cast<const cv::_InputArray*>(kernel), anchor, iterations, borderType, *borderValue);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_fillConvexPoly_const__InputOutputArrayX_const__InputArrayX_const_ScalarX_int_int(void* img, void* points, const cv::Scalar* color, int lineType, int shift) {
		try {
			cv::fillConvexPoly(*reinterpret_cast<const cv::_InputOutputArray*>(img), *reinterpret_cast<const cv::_InputArray*>(points), *color, lineType, shift);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_fillPoly_const__InputOutputArrayX_const__InputArrayX_const_ScalarX_int_int_Point(void* img, void* pts, const cv::Scalar* color, int lineType, int shift, cv::Point offset) {
		try {
			cv::fillPoly(*reinterpret_cast<const cv::_InputOutputArray*>(img), *reinterpret_cast<const cv::_InputArray*>(pts), *color, lineType, shift, offset);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_filter2D_const__InputArrayX_const__OutputArrayX_int_const__InputArrayX_Point_double_int(void* src, void* dst, int ddepth, void* kernel, cv::Point anchor, double delta, int borderType) {
		try {
			cv::filter2D(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), ddepth, *reinterpret_cast<const cv::_InputArray*>(kernel), anchor, delta, borderType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_findContours_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_int_int_Point(void* image, void* contours, void* hierarchy, int mode, int method, cv::Point offset) {
		try {
			cv::findContours(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<const cv::_OutputArray*>(contours), *reinterpret_cast<const cv::_OutputArray*>(hierarchy), mode, method, offset);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_findContours_const__InputArrayX_const__OutputArrayX_int_int_Point(void* image, void* contours, int mode, int method, cv::Point offset) {
		try {
			cv::findContours(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<const cv::_OutputArray*>(contours), mode, method, offset);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_fitEllipseAMS_const__InputArrayX(void* points) {
		try {
			cv::RotatedRect ret = cv::fitEllipseAMS(*reinterpret_cast<const cv::_InputArray*>(points));
			return Ok<void*>(new cv::RotatedRect(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_fitEllipseDirect_const__InputArrayX(void* points) {
		try {
			cv::RotatedRect ret = cv::fitEllipseDirect(*reinterpret_cast<const cv::_InputArray*>(points));
			return Ok<void*>(new cv::RotatedRect(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_fitEllipse_const__InputArrayX(void* points) {
		try {
			cv::RotatedRect ret = cv::fitEllipse(*reinterpret_cast<const cv::_InputArray*>(points));
			return Ok<void*>(new cv::RotatedRect(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_fitLine_const__InputArrayX_const__OutputArrayX_int_double_double_double(void* points, void* line, int distType, double param, double reps, double aeps) {
		try {
			cv::fitLine(*reinterpret_cast<const cv::_InputArray*>(points), *reinterpret_cast<const cv::_OutputArray*>(line), distType, param, reps, aeps);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_floodFill_const__InputOutputArrayX_Point_Scalar_RectX_Scalar_Scalar_int(void* image, cv::Point seedPoint, cv::Scalar newVal, cv::Rect* rect, cv::Scalar loDiff, cv::Scalar upDiff, int flags) {
		try {
			int ret = cv::floodFill(*reinterpret_cast<const cv::_InputOutputArray*>(image), seedPoint, newVal, rect, loDiff, upDiff, flags);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_floodFill_const__InputOutputArrayX_const__InputOutputArrayX_Point_Scalar_RectX_Scalar_Scalar_int(void* image, void* mask, cv::Point seedPoint, cv::Scalar newVal, cv::Rect* rect, cv::Scalar loDiff, cv::Scalar upDiff, int flags) {
		try {
			int ret = cv::floodFill(*reinterpret_cast<const cv::_InputOutputArray*>(image), *reinterpret_cast<const cv::_InputOutputArray*>(mask), seedPoint, newVal, rect, loDiff, upDiff, flags);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<void*> cv_getAffineTransform_const_Point2fX_const_Point2fX(const cv::Point2f* src, const cv::Point2f* dst) {
		try {
			cv::Mat ret = cv::getAffineTransform(src, dst);
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_getAffineTransform_const__InputArrayX_const__InputArrayX(void* src, void* dst) {
		try {
			cv::Mat ret = cv::getAffineTransform(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_InputArray*>(dst));
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_getDerivKernels_const__OutputArrayX_const__OutputArrayX_int_int_int_bool_int(void* kx, void* ky, int dx, int dy, int ksize, bool normalize, int ktype) {
		try {
			cv::getDerivKernels(*reinterpret_cast<const cv::_OutputArray*>(kx), *reinterpret_cast<const cv::_OutputArray*>(ky), dx, dy, ksize, normalize, ktype);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_getFontScaleFromHeight_int_int_int(int fontFace, int pixelHeight, int thickness) {
		try {
			double ret = cv::getFontScaleFromHeight(fontFace, pixelHeight, thickness);
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result<void*> cv_getGaborKernel_Size_double_double_double_double_double_int(cv::Size ksize, double sigma, double theta, double lambd, double gamma, double psi, int ktype) {
		try {
			cv::Mat ret = cv::getGaborKernel(ksize, sigma, theta, lambd, gamma, psi, ktype);
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_getGaussianKernel_int_double_int(int ksize, double sigma, int ktype) {
		try {
			cv::Mat ret = cv::getGaussianKernel(ksize, sigma, ktype);
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_getPerspectiveTransform_const_Point2fX_const_Point2fX_int(const cv::Point2f* src, const cv::Point2f* dst, int solveMethod) {
		try {
			cv::Mat ret = cv::getPerspectiveTransform(src, dst, solveMethod);
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_getPerspectiveTransform_const__InputArrayX_const__InputArrayX_int(void* src, void* dst, int solveMethod) {
		try {
			cv::Mat ret = cv::getPerspectiveTransform(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_InputArray*>(dst), solveMethod);
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_getRectSubPix_const__InputArrayX_Size_Point2f_const__OutputArrayX_int(void* image, cv::Size patchSize, cv::Point2f center, void* patch, int patchType) {
		try {
			cv::getRectSubPix(*reinterpret_cast<const cv::_InputArray*>(image), patchSize, center, *reinterpret_cast<const cv::_OutputArray*>(patch), patchType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_getRotationMatrix2D_Point2f_double_double(cv::Point2f center, double angle, double scale) {
		try {
			cv::Mat ret = cv::getRotationMatrix2D(center, angle, scale);
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<cv::Matx23d> cv_getRotationMatrix2D__Point2f_double_double(cv::Point2f center, double angle, double scale) {
		try {
			cv::Matx23d ret = cv::getRotationMatrix2D_(center, angle, scale);
			return Ok<cv::Matx23d>(ret);
		} OCVRS_CATCH(Result<cv::Matx23d>)
	}
	
	Result<void*> cv_getStructuringElement_int_Size_Point(int shape, cv::Size ksize, cv::Point anchor) {
		try {
			cv::Mat ret = cv::getStructuringElement(shape, ksize, anchor);
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<cv::Size> cv_getTextSize_const_StringX_int_double_int_intX(const char* text, int fontFace, double fontScale, int thickness, int* baseLine) {
		try {
			cv::Size ret = cv::getTextSize(std::string(text), fontFace, fontScale, thickness, baseLine);
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_goodFeaturesToTrack_const__InputArrayX_const__OutputArrayX_int_double_double_const__InputArrayX_int_bool_double(void* image, void* corners, int maxCorners, double qualityLevel, double minDistance, void* mask, int blockSize, bool useHarrisDetector, double k) {
		try {
			cv::goodFeaturesToTrack(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<const cv::_OutputArray*>(corners), maxCorners, qualityLevel, minDistance, *reinterpret_cast<const cv::_InputArray*>(mask), blockSize, useHarrisDetector, k);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_goodFeaturesToTrack_const__InputArrayX_const__OutputArrayX_int_double_double_const__InputArrayX_int_int_bool_double(void* image, void* corners, int maxCorners, double qualityLevel, double minDistance, void* mask, int blockSize, int gradientSize, bool useHarrisDetector, double k) {
		try {
			cv::goodFeaturesToTrack(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<const cv::_OutputArray*>(corners), maxCorners, qualityLevel, minDistance, *reinterpret_cast<const cv::_InputArray*>(mask), blockSize, gradientSize, useHarrisDetector, k);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_grabCut_const__InputArrayX_const__InputOutputArrayX_Rect_const__InputOutputArrayX_const__InputOutputArrayX_int_int(void* img, void* mask, cv::Rect rect, void* bgdModel, void* fgdModel, int iterCount, int mode) {
		try {
			cv::grabCut(*reinterpret_cast<const cv::_InputArray*>(img), *reinterpret_cast<const cv::_InputOutputArray*>(mask), rect, *reinterpret_cast<const cv::_InputOutputArray*>(bgdModel), *reinterpret_cast<const cv::_InputOutputArray*>(fgdModel), iterCount, mode);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_integral_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_int_int(void* src, void* sum, void* sqsum, void* tilted, int sdepth, int sqdepth) {
		try {
			cv::integral(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(sum), *reinterpret_cast<const cv::_OutputArray*>(sqsum), *reinterpret_cast<const cv::_OutputArray*>(tilted), sdepth, sqdepth);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_integral_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_int_int(void* src, void* sum, void* sqsum, int sdepth, int sqdepth) {
		try {
			cv::integral(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(sum), *reinterpret_cast<const cv::_OutputArray*>(sqsum), sdepth, sqdepth);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_integral_const__InputArrayX_const__OutputArrayX_int(void* src, void* sum, int sdepth) {
		try {
			cv::integral(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(sum), sdepth);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_intersectConvexConvex_const__InputArrayX_const__InputArrayX_const__OutputArrayX_bool(void* _p1, void* _p2, void* _p12, bool handleNested) {
		try {
			float ret = cv::intersectConvexConvex(*reinterpret_cast<const cv::_InputArray*>(_p1), *reinterpret_cast<const cv::_InputArray*>(_p2), *reinterpret_cast<const cv::_OutputArray*>(_p12), handleNested);
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_invertAffineTransform_const__InputArrayX_const__OutputArrayX(void* M, void* iM) {
		try {
			cv::invertAffineTransform(*reinterpret_cast<const cv::_InputArray*>(M), *reinterpret_cast<const cv::_OutputArray*>(iM));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_isContourConvex_const__InputArrayX(void* contour) {
		try {
			bool ret = cv::isContourConvex(*reinterpret_cast<const cv::_InputArray*>(contour));
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_line_const__InputOutputArrayX_Point_Point_const_ScalarX_int_int_int(void* img, cv::Point pt1, cv::Point pt2, const cv::Scalar* color, int thickness, int lineType, int shift) {
		try {
			cv::line(*reinterpret_cast<const cv::_InputOutputArray*>(img), pt1, pt2, *color, thickness, lineType, shift);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_linearPolar_const__InputArrayX_const__OutputArrayX_Point2f_double_int(void* src, void* dst, cv::Point2f center, double maxRadius, int flags) {
		try {
			cv::linearPolar(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), center, maxRadius, flags);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_logPolar_const__InputArrayX_const__OutputArrayX_Point2f_double_int(void* src, void* dst, cv::Point2f center, double M, int flags) {
		try {
			cv::logPolar(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), center, M, flags);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_matchShapes_const__InputArrayX_const__InputArrayX_int_double(void* contour1, void* contour2, int method, double parameter) {
		try {
			double ret = cv::matchShapes(*reinterpret_cast<const cv::_InputArray*>(contour1), *reinterpret_cast<const cv::_InputArray*>(contour2), method, parameter);
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_matchTemplate_const__InputArrayX_const__InputArrayX_const__OutputArrayX_int_const__InputArrayX(void* image, void* templ, void* result, int method, void* mask) {
		try {
			cv::matchTemplate(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<const cv::_InputArray*>(templ), *reinterpret_cast<const cv::_OutputArray*>(result), method, *reinterpret_cast<const cv::_InputArray*>(mask));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_medianBlur_const__InputArrayX_const__OutputArrayX_int(void* src, void* dst, int ksize) {
		try {
			cv::medianBlur(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), ksize);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_minAreaRect_const__InputArrayX(void* points) {
		try {
			cv::RotatedRect ret = cv::minAreaRect(*reinterpret_cast<const cv::_InputArray*>(points));
			return Ok<void*>(new cv::RotatedRect(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_minEnclosingCircle_const__InputArrayX_Point2fX_floatX(void* points, cv::Point2f* center, float* radius) {
		try {
			cv::minEnclosingCircle(*reinterpret_cast<const cv::_InputArray*>(points), *center, *radius);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_minEnclosingTriangle_const__InputArrayX_const__OutputArrayX(void* points, void* triangle) {
		try {
			double ret = cv::minEnclosingTriangle(*reinterpret_cast<const cv::_InputArray*>(points), *reinterpret_cast<const cv::_OutputArray*>(triangle));
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result<cv::Moments> cv_moments_const__InputArrayX_bool(void* array, bool binaryImage) {
		try {
			cv::Moments ret = cv::moments(*reinterpret_cast<const cv::_InputArray*>(array), binaryImage);
			return Ok<cv::Moments>(ret);
		} OCVRS_CATCH(Result<cv::Moments>)
	}
	
	Result<cv::Scalar> cv_morphologyDefaultBorderValue() {
		try {
			cv::Scalar ret = cv::morphologyDefaultBorderValue();
			return Ok<cv::Scalar>(ret);
		} OCVRS_CATCH(Result<cv::Scalar>)
	}
	
	Result_void cv_morphologyEx_const__InputArrayX_const__OutputArrayX_int_const__InputArrayX_Point_int_int_const_ScalarX(void* src, void* dst, int op, void* kernel, cv::Point anchor, int iterations, int borderType, const cv::Scalar* borderValue) {
		try {
			cv::morphologyEx(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), op, *reinterpret_cast<const cv::_InputArray*>(kernel), anchor, iterations, borderType, *borderValue);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Point2d> cv_phaseCorrelate_const__InputArrayX_const__InputArrayX_const__InputArrayX_doubleX(void* src1, void* src2, void* window, double* response) {
		try {
			cv::Point2d ret = cv::phaseCorrelate(*reinterpret_cast<const cv::_InputArray*>(src1), *reinterpret_cast<const cv::_InputArray*>(src2), *reinterpret_cast<const cv::_InputArray*>(window), response);
			return Ok<cv::Point2d>(ret);
		} OCVRS_CATCH(Result<cv::Point2d>)
	}
	
	Result<double> cv_pointPolygonTest_const__InputArrayX_Point2f_bool(void* contour, cv::Point2f pt, bool measureDist) {
		try {
			double ret = cv::pointPolygonTest(*reinterpret_cast<const cv::_InputArray*>(contour), pt, measureDist);
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_polylines_const__InputOutputArrayX_const__InputArrayX_bool_const_ScalarX_int_int_int(void* img, void* pts, bool isClosed, const cv::Scalar* color, int thickness, int lineType, int shift) {
		try {
			cv::polylines(*reinterpret_cast<const cv::_InputOutputArray*>(img), *reinterpret_cast<const cv::_InputArray*>(pts), isClosed, *color, thickness, lineType, shift);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_preCornerDetect_const__InputArrayX_const__OutputArrayX_int_int(void* src, void* dst, int ksize, int borderType) {
		try {
			cv::preCornerDetect(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), ksize, borderType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_putText_const__InputOutputArrayX_const_StringX_Point_int_double_Scalar_int_int_bool(void* img, const char* text, cv::Point org, int fontFace, double fontScale, cv::Scalar color, int thickness, int lineType, bool bottomLeftOrigin) {
		try {
			cv::putText(*reinterpret_cast<const cv::_InputOutputArray*>(img), std::string(text), org, fontFace, fontScale, color, thickness, lineType, bottomLeftOrigin);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_pyrDown_const__InputArrayX_const__OutputArrayX_const_SizeX_int(void* src, void* dst, const cv::Size* dstsize, int borderType) {
		try {
			cv::pyrDown(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), *dstsize, borderType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_pyrMeanShiftFiltering_const__InputArrayX_const__OutputArrayX_double_double_int_TermCriteria(void* src, void* dst, double sp, double sr, int maxLevel, void* termcrit) {
		try {
			cv::pyrMeanShiftFiltering(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), sp, sr, maxLevel, *reinterpret_cast<cv::TermCriteria*>(termcrit));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_pyrUp_const__InputArrayX_const__OutputArrayX_const_SizeX_int(void* src, void* dst, const cv::Size* dstsize, int borderType) {
		try {
			cv::pyrUp(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), *dstsize, borderType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_rectangle_const__InputOutputArrayX_Point_Point_const_ScalarX_int_int_int(void* img, cv::Point pt1, cv::Point pt2, const cv::Scalar* color, int thickness, int lineType, int shift) {
		try {
			cv::rectangle(*reinterpret_cast<const cv::_InputOutputArray*>(img), pt1, pt2, *color, thickness, lineType, shift);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_rectangle_const__InputOutputArrayX_Rect_const_ScalarX_int_int_int(void* img, cv::Rect rec, const cv::Scalar* color, int thickness, int lineType, int shift) {
		try {
			cv::rectangle(*reinterpret_cast<const cv::_InputOutputArray*>(img), rec, *color, thickness, lineType, shift);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_remap_const__InputArrayX_const__OutputArrayX_const__InputArrayX_const__InputArrayX_int_int_const_ScalarX(void* src, void* dst, void* map1, void* map2, int interpolation, int borderMode, const cv::Scalar* borderValue) {
		try {
			cv::remap(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), *reinterpret_cast<const cv::_InputArray*>(map1), *reinterpret_cast<const cv::_InputArray*>(map2), interpolation, borderMode, *borderValue);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_resize_const__InputArrayX_const__OutputArrayX_Size_double_double_int(void* src, void* dst, cv::Size dsize, double fx, double fy, int interpolation) {
		try {
			cv::resize(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), dsize, fx, fy, interpolation);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_rotatedRectangleIntersection_const_RotatedRectX_const_RotatedRectX_const__OutputArrayX(void* rect1, void* rect2, void* intersectingRegion) {
		try {
			int ret = cv::rotatedRectangleIntersection(*reinterpret_cast<const cv::RotatedRect*>(rect1), *reinterpret_cast<const cv::RotatedRect*>(rect2), *reinterpret_cast<const cv::_OutputArray*>(intersectingRegion));
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_sepFilter2D_const__InputArrayX_const__OutputArrayX_int_const__InputArrayX_const__InputArrayX_Point_double_int(void* src, void* dst, int ddepth, void* kernelX, void* kernelY, cv::Point anchor, double delta, int borderType) {
		try {
			cv::sepFilter2D(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), ddepth, *reinterpret_cast<const cv::_InputArray*>(kernelX), *reinterpret_cast<const cv::_InputArray*>(kernelY), anchor, delta, borderType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_spatialGradient_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_int_int(void* src, void* dx, void* dy, int ksize, int borderType) {
		try {
			cv::spatialGradient(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dx), *reinterpret_cast<const cv::_OutputArray*>(dy), ksize, borderType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_sqrBoxFilter_const__InputArrayX_const__OutputArrayX_int_Size_Point_bool_int(void* src, void* dst, int ddepth, cv::Size ksize, cv::Point anchor, bool normalize, int borderType) {
		try {
			cv::sqrBoxFilter(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), ddepth, ksize, anchor, normalize, borderType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_threshold_const__InputArrayX_const__OutputArrayX_double_double_int(void* src, void* dst, double thresh, double maxval, int type) {
		try {
			double ret = cv::threshold(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), thresh, maxval, type);
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_warpAffine_const__InputArrayX_const__OutputArrayX_const__InputArrayX_Size_int_int_const_ScalarX(void* src, void* dst, void* M, cv::Size dsize, int flags, int borderMode, const cv::Scalar* borderValue) {
		try {
			cv::warpAffine(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), *reinterpret_cast<const cv::_InputArray*>(M), dsize, flags, borderMode, *borderValue);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_warpPerspective_const__InputArrayX_const__OutputArrayX_const__InputArrayX_Size_int_int_const_ScalarX(void* src, void* dst, void* M, cv::Size dsize, int flags, int borderMode, const cv::Scalar* borderValue) {
		try {
			cv::warpPerspective(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), *reinterpret_cast<const cv::_InputArray*>(M), dsize, flags, borderMode, *borderValue);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_warpPolar_const__InputArrayX_const__OutputArrayX_Size_Point2f_double_int(void* src, void* dst, cv::Size dsize, cv::Point2f center, double maxRadius, int flags) {
		try {
			cv::warpPolar(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst), dsize, center, maxRadius, flags);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_watershed_const__InputArrayX_const__InputOutputArrayX(void* image, void* markers) {
		try {
			cv::watershed(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<const cv::_InputOutputArray*>(markers));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_wrapperEMD_const__InputArrayX_const__InputArrayX_int_const__InputArrayX_Ptr_float__const__OutputArrayX(void* signature1, void* signature2, int distType, void* cost, void* lowerBound, void* flow) {
		try {
			float ret = cv::wrapperEMD(*reinterpret_cast<const cv::_InputArray*>(signature1), *reinterpret_cast<const cv::_InputArray*>(signature2), distType, *reinterpret_cast<const cv::_InputArray*>(cost), *reinterpret_cast<cv::Ptr<float>*>(lowerBound), *reinterpret_cast<const cv::_OutputArray*>(flow));
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_CLAHE_apply_const__InputArrayX_const__OutputArrayX(void* instance, void* src, void* dst) {
		try {
			reinterpret_cast<cv::CLAHE*>(instance)->apply(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_OutputArray*>(dst));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_CLAHE_setClipLimit_double(void* instance, double clipLimit) {
		try {
			reinterpret_cast<cv::CLAHE*>(instance)->setClipLimit(clipLimit);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_CLAHE_getClipLimit_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::CLAHE*>(instance)->getClipLimit();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_CLAHE_setTilesGridSize_Size(void* instance, cv::Size tileGridSize) {
		try {
			reinterpret_cast<cv::CLAHE*>(instance)->setTilesGridSize(tileGridSize);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Size> cv_CLAHE_getTilesGridSize_const(void* instance) {
		try {
			cv::Size ret = reinterpret_cast<cv::CLAHE*>(instance)->getTilesGridSize();
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_CLAHE_collectGarbage(void* instance) {
		try {
			reinterpret_cast<cv::CLAHE*>(instance)->collectGarbage();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_GeneralizedHough_setTemplate_const__InputArrayX_Point(void* instance, void* templ, cv::Point templCenter) {
		try {
			reinterpret_cast<cv::GeneralizedHough*>(instance)->setTemplate(*reinterpret_cast<const cv::_InputArray*>(templ), templCenter);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_GeneralizedHough_setTemplate_const__InputArrayX_const__InputArrayX_const__InputArrayX_Point(void* instance, void* edges, void* dx, void* dy, cv::Point templCenter) {
		try {
			reinterpret_cast<cv::GeneralizedHough*>(instance)->setTemplate(*reinterpret_cast<const cv::_InputArray*>(edges), *reinterpret_cast<const cv::_InputArray*>(dx), *reinterpret_cast<const cv::_InputArray*>(dy), templCenter);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_GeneralizedHough_detect_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(void* instance, void* image, void* positions, void* votes) {
		try {
			reinterpret_cast<cv::GeneralizedHough*>(instance)->detect(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<const cv::_OutputArray*>(positions), *reinterpret_cast<const cv::_OutputArray*>(votes));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_GeneralizedHough_detect_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(void* instance, void* edges, void* dx, void* dy, void* positions, void* votes) {
		try {
			reinterpret_cast<cv::GeneralizedHough*>(instance)->detect(*reinterpret_cast<const cv::_InputArray*>(edges), *reinterpret_cast<const cv::_InputArray*>(dx), *reinterpret_cast<const cv::_InputArray*>(dy), *reinterpret_cast<const cv::_OutputArray*>(positions), *reinterpret_cast<const cv::_OutputArray*>(votes));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_GeneralizedHough_setCannyLowThresh_int(void* instance, int cannyLowThresh) {
		try {
			reinterpret_cast<cv::GeneralizedHough*>(instance)->setCannyLowThresh(cannyLowThresh);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_GeneralizedHough_getCannyLowThresh_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::GeneralizedHough*>(instance)->getCannyLowThresh();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_GeneralizedHough_setCannyHighThresh_int(void* instance, int cannyHighThresh) {
		try {
			reinterpret_cast<cv::GeneralizedHough*>(instance)->setCannyHighThresh(cannyHighThresh);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_GeneralizedHough_getCannyHighThresh_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::GeneralizedHough*>(instance)->getCannyHighThresh();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_GeneralizedHough_setMinDist_double(void* instance, double minDist) {
		try {
			reinterpret_cast<cv::GeneralizedHough*>(instance)->setMinDist(minDist);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_GeneralizedHough_getMinDist_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::GeneralizedHough*>(instance)->getMinDist();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_GeneralizedHough_setDp_double(void* instance, double dp) {
		try {
			reinterpret_cast<cv::GeneralizedHough*>(instance)->setDp(dp);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_GeneralizedHough_getDp_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::GeneralizedHough*>(instance)->getDp();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_GeneralizedHough_setMaxBufferSize_int(void* instance, int maxBufferSize) {
		try {
			reinterpret_cast<cv::GeneralizedHough*>(instance)->setMaxBufferSize(maxBufferSize);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_GeneralizedHough_getMaxBufferSize_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::GeneralizedHough*>(instance)->getMaxBufferSize();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_GeneralizedHoughBallard_setLevels_int(void* instance, int levels) {
		try {
			reinterpret_cast<cv::GeneralizedHoughBallard*>(instance)->setLevels(levels);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_GeneralizedHoughBallard_getLevels_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::GeneralizedHoughBallard*>(instance)->getLevels();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_GeneralizedHoughBallard_setVotesThreshold_int(void* instance, int votesThreshold) {
		try {
			reinterpret_cast<cv::GeneralizedHoughBallard*>(instance)->setVotesThreshold(votesThreshold);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_GeneralizedHoughBallard_getVotesThreshold_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::GeneralizedHoughBallard*>(instance)->getVotesThreshold();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_GeneralizedHoughGuil_setXi_double(void* instance, double xi) {
		try {
			reinterpret_cast<cv::GeneralizedHoughGuil*>(instance)->setXi(xi);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_GeneralizedHoughGuil_getXi_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::GeneralizedHoughGuil*>(instance)->getXi();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_GeneralizedHoughGuil_setLevels_int(void* instance, int levels) {
		try {
			reinterpret_cast<cv::GeneralizedHoughGuil*>(instance)->setLevels(levels);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_GeneralizedHoughGuil_getLevels_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::GeneralizedHoughGuil*>(instance)->getLevels();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_GeneralizedHoughGuil_setAngleEpsilon_double(void* instance, double angleEpsilon) {
		try {
			reinterpret_cast<cv::GeneralizedHoughGuil*>(instance)->setAngleEpsilon(angleEpsilon);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_GeneralizedHoughGuil_getAngleEpsilon_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::GeneralizedHoughGuil*>(instance)->getAngleEpsilon();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_GeneralizedHoughGuil_setMinAngle_double(void* instance, double minAngle) {
		try {
			reinterpret_cast<cv::GeneralizedHoughGuil*>(instance)->setMinAngle(minAngle);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_GeneralizedHoughGuil_getMinAngle_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::GeneralizedHoughGuil*>(instance)->getMinAngle();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_GeneralizedHoughGuil_setMaxAngle_double(void* instance, double maxAngle) {
		try {
			reinterpret_cast<cv::GeneralizedHoughGuil*>(instance)->setMaxAngle(maxAngle);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_GeneralizedHoughGuil_getMaxAngle_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::GeneralizedHoughGuil*>(instance)->getMaxAngle();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_GeneralizedHoughGuil_setAngleStep_double(void* instance, double angleStep) {
		try {
			reinterpret_cast<cv::GeneralizedHoughGuil*>(instance)->setAngleStep(angleStep);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_GeneralizedHoughGuil_getAngleStep_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::GeneralizedHoughGuil*>(instance)->getAngleStep();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_GeneralizedHoughGuil_setAngleThresh_int(void* instance, int angleThresh) {
		try {
			reinterpret_cast<cv::GeneralizedHoughGuil*>(instance)->setAngleThresh(angleThresh);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_GeneralizedHoughGuil_getAngleThresh_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::GeneralizedHoughGuil*>(instance)->getAngleThresh();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_GeneralizedHoughGuil_setMinScale_double(void* instance, double minScale) {
		try {
			reinterpret_cast<cv::GeneralizedHoughGuil*>(instance)->setMinScale(minScale);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_GeneralizedHoughGuil_getMinScale_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::GeneralizedHoughGuil*>(instance)->getMinScale();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_GeneralizedHoughGuil_setMaxScale_double(void* instance, double maxScale) {
		try {
			reinterpret_cast<cv::GeneralizedHoughGuil*>(instance)->setMaxScale(maxScale);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_GeneralizedHoughGuil_getMaxScale_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::GeneralizedHoughGuil*>(instance)->getMaxScale();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_GeneralizedHoughGuil_setScaleStep_double(void* instance, double scaleStep) {
		try {
			reinterpret_cast<cv::GeneralizedHoughGuil*>(instance)->setScaleStep(scaleStep);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_GeneralizedHoughGuil_getScaleStep_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::GeneralizedHoughGuil*>(instance)->getScaleStep();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_GeneralizedHoughGuil_setScaleThresh_int(void* instance, int scaleThresh) {
		try {
			reinterpret_cast<cv::GeneralizedHoughGuil*>(instance)->setScaleThresh(scaleThresh);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_GeneralizedHoughGuil_getScaleThresh_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::GeneralizedHoughGuil*>(instance)->getScaleThresh();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_GeneralizedHoughGuil_setPosThresh_int(void* instance, int posThresh) {
		try {
			reinterpret_cast<cv::GeneralizedHoughGuil*>(instance)->setPosThresh(posThresh);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_GeneralizedHoughGuil_getPosThresh_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::GeneralizedHoughGuil*>(instance)->getPosThresh();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<unsigned char*> cv_LineIterator_ptr(void* instance) {
		try {
			unsigned char* ret = reinterpret_cast<cv::LineIterator*>(instance)->ptr;
			return Ok<unsigned char*>(ret);
		} OCVRS_CATCH(Result<unsigned char*>)
	}
	
	Result_void cv_LineIterator_setPtr_unsigned_charX(void* instance, unsigned char* val) {
		try {
			reinterpret_cast<cv::LineIterator*>(instance)->ptr = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<const unsigned char*> cv_LineIterator_ptr0_const(void* instance) {
		try {
			const unsigned char* ret = reinterpret_cast<cv::LineIterator*>(instance)->ptr0;
			return Ok<const unsigned char*>(ret);
		} OCVRS_CATCH(Result<const unsigned char*>)
	}
	
	Result<int> cv_LineIterator_step_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::LineIterator*>(instance)->step;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_LineIterator_setStep_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::LineIterator*>(instance)->step = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_LineIterator_elemSize_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::LineIterator*>(instance)->elemSize;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_LineIterator_setElemSize_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::LineIterator*>(instance)->elemSize = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_LineIterator_err_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::LineIterator*>(instance)->err;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_LineIterator_setErr_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::LineIterator*>(instance)->err = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_LineIterator_count_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::LineIterator*>(instance)->count;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_LineIterator_setCount_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::LineIterator*>(instance)->count = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_LineIterator_minusDelta_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::LineIterator*>(instance)->minusDelta;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_LineIterator_setMinusDelta_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::LineIterator*>(instance)->minusDelta = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_LineIterator_plusDelta_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::LineIterator*>(instance)->plusDelta;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_LineIterator_setPlusDelta_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::LineIterator*>(instance)->plusDelta = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_LineIterator_minusStep_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::LineIterator*>(instance)->minusStep;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_LineIterator_setMinusStep_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::LineIterator*>(instance)->minusStep = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_LineIterator_plusStep_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::LineIterator*>(instance)->plusStep;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_LineIterator_setPlusStep_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::LineIterator*>(instance)->plusStep = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_LineIterator_delete(cv::LineIterator* instance) {
		delete instance;
	}
	Result<void*> cv_LineIterator_LineIterator_const_MatX_Point_Point_int_bool(void* img, cv::Point pt1, cv::Point pt2, int connectivity, bool leftToRight) {
		try {
			cv::LineIterator* ret = new cv::LineIterator(*reinterpret_cast<const cv::Mat*>(img), pt1, pt2, connectivity, leftToRight);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<cv::Point> cv_LineIterator_pos_const(void* instance) {
		try {
			cv::Point ret = reinterpret_cast<cv::LineIterator*>(instance)->pos();
			return Ok<cv::Point>(ret);
		} OCVRS_CATCH(Result<cv::Point>)
	}
	
	Result_void cv_LineSegmentDetector_detect_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX(void* instance, void* _image, void* _lines, void* width, void* prec, void* nfa) {
		try {
			reinterpret_cast<cv::LineSegmentDetector*>(instance)->detect(*reinterpret_cast<const cv::_InputArray*>(_image), *reinterpret_cast<const cv::_OutputArray*>(_lines), *reinterpret_cast<const cv::_OutputArray*>(width), *reinterpret_cast<const cv::_OutputArray*>(prec), *reinterpret_cast<const cv::_OutputArray*>(nfa));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_LineSegmentDetector_drawSegments_const__InputOutputArrayX_const__InputArrayX(void* instance, void* _image, void* lines) {
		try {
			reinterpret_cast<cv::LineSegmentDetector*>(instance)->drawSegments(*reinterpret_cast<const cv::_InputOutputArray*>(_image), *reinterpret_cast<const cv::_InputArray*>(lines));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_LineSegmentDetector_compareSegments_const_SizeX_const__InputArrayX_const__InputArrayX_const__InputOutputArrayX(void* instance, const cv::Size* size, void* lines1, void* lines2, void* _image) {
		try {
			int ret = reinterpret_cast<cv::LineSegmentDetector*>(instance)->compareSegments(*size, *reinterpret_cast<const cv::_InputArray*>(lines1), *reinterpret_cast<const cv::_InputArray*>(lines2), *reinterpret_cast<const cv::_InputOutputArray*>(_image));
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	void cv_Subdiv2D_delete(cv::Subdiv2D* instance) {
		delete instance;
	}
	Result<void*> cv_Subdiv2D_Subdiv2D() {
		try {
			cv::Subdiv2D* ret = new cv::Subdiv2D();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_Subdiv2D_Subdiv2D_Rect(cv::Rect rect) {
		try {
			cv::Subdiv2D* ret = new cv::Subdiv2D(rect);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_Subdiv2D_initDelaunay_Rect(void* instance, cv::Rect rect) {
		try {
			reinterpret_cast<cv::Subdiv2D*>(instance)->initDelaunay(rect);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_Subdiv2D_insert_Point2f(void* instance, cv::Point2f pt) {
		try {
			int ret = reinterpret_cast<cv::Subdiv2D*>(instance)->insert(pt);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_Subdiv2D_insert_const_vector_Point2f_X(void* instance, void* ptvec) {
		try {
			reinterpret_cast<cv::Subdiv2D*>(instance)->insert(*reinterpret_cast<const std::vector<cv::Point2f>*>(ptvec));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_Subdiv2D_locate_Point2f_intX_intX(void* instance, cv::Point2f pt, int* edge, int* vertex) {
		try {
			int ret = reinterpret_cast<cv::Subdiv2D*>(instance)->locate(pt, *edge, *vertex);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_Subdiv2D_findNearest_Point2f_Point2fX(void* instance, cv::Point2f pt, cv::Point2f* nearestPt) {
		try {
			int ret = reinterpret_cast<cv::Subdiv2D*>(instance)->findNearest(pt, nearestPt);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_Subdiv2D_getEdgeList_const_vector_Vec4f_X(void* instance, void* edgeList) {
		try {
			reinterpret_cast<cv::Subdiv2D*>(instance)->getEdgeList(*reinterpret_cast<std::vector<cv::Vec4f>*>(edgeList));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_Subdiv2D_getLeadingEdgeList_const_vector_int_X(void* instance, void* leadingEdgeList) {
		try {
			reinterpret_cast<cv::Subdiv2D*>(instance)->getLeadingEdgeList(*reinterpret_cast<std::vector<int>*>(leadingEdgeList));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_Subdiv2D_getTriangleList_const_vector_Vec6f_X(void* instance, void* triangleList) {
		try {
			reinterpret_cast<cv::Subdiv2D*>(instance)->getTriangleList(*reinterpret_cast<std::vector<cv::Vec6f>*>(triangleList));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_Subdiv2D_getVoronoiFacetList_const_vector_int_X_vector_vector_Point2f__X_vector_Point2f_X(void* instance, void* idx, void* facetList, void* facetCenters) {
		try {
			reinterpret_cast<cv::Subdiv2D*>(instance)->getVoronoiFacetList(*reinterpret_cast<const std::vector<int>*>(idx), *reinterpret_cast<std::vector<std::vector<cv::Point2f>>*>(facetList), *reinterpret_cast<std::vector<cv::Point2f>*>(facetCenters));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Point2f> cv_Subdiv2D_getVertex_const_int_intX(void* instance, int vertex, int* firstEdge) {
		try {
			cv::Point2f ret = reinterpret_cast<cv::Subdiv2D*>(instance)->getVertex(vertex, firstEdge);
			return Ok<cv::Point2f>(ret);
		} OCVRS_CATCH(Result<cv::Point2f>)
	}
	
	Result<int> cv_Subdiv2D_getEdge_const_int_int(void* instance, int edge, int nextEdgeType) {
		try {
			int ret = reinterpret_cast<cv::Subdiv2D*>(instance)->getEdge(edge, nextEdgeType);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_Subdiv2D_nextEdge_const_int(void* instance, int edge) {
		try {
			int ret = reinterpret_cast<cv::Subdiv2D*>(instance)->nextEdge(edge);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_Subdiv2D_rotateEdge_const_int_int(void* instance, int edge, int rotate) {
		try {
			int ret = reinterpret_cast<cv::Subdiv2D*>(instance)->rotateEdge(edge, rotate);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_Subdiv2D_symEdge_const_int(void* instance, int edge) {
		try {
			int ret = reinterpret_cast<cv::Subdiv2D*>(instance)->symEdge(edge);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_Subdiv2D_edgeOrg_const_int_Point2fX(void* instance, int edge, cv::Point2f* orgpt) {
		try {
			int ret = reinterpret_cast<cv::Subdiv2D*>(instance)->edgeOrg(edge, orgpt);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_Subdiv2D_edgeDst_const_int_Point2fX(void* instance, int edge, cv::Point2f* dstpt) {
		try {
			int ret = reinterpret_cast<cv::Subdiv2D*>(instance)->edgeDst(edge, dstpt);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
}
