#include "imgproc.hpp"
#include "imgproc_types.hpp"

extern "C" {
	Result_void cv_Canny_const__InputArrayX_const__InputArrayX_const__OutputArrayX_double_double_bool(const cv::_InputArray* dx, const cv::_InputArray* dy, const cv::_OutputArray* edges, double threshold1, double threshold2, bool L2gradient) {
		try {
			cv::Canny(*dx, *dy, *edges, threshold1, threshold2, L2gradient);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_Canny_const__InputArrayX_const__OutputArrayX_double_double_int_bool(const cv::_InputArray* image, const cv::_OutputArray* edges, double threshold1, double threshold2, int apertureSize, bool L2gradient) {
		try {
			cv::Canny(*image, *edges, threshold1, threshold2, apertureSize, L2gradient);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_EMD_const__InputArrayX_const__InputArrayX_int_const__InputArrayX_floatX_const__OutputArrayX(const cv::_InputArray* signature1, const cv::_InputArray* signature2, int distType, const cv::_InputArray* cost, float* lowerBound, const cv::_OutputArray* flow) {
		try {
			float ret = cv::EMD(*signature1, *signature2, distType, *cost, lowerBound, *flow);
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_GaussianBlur_const__InputArrayX_const__OutputArrayX_Size_double_double_int(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::Size* ksize, double sigmaX, double sigmaY, int borderType) {
		try {
			cv::GaussianBlur(*src, *dst, *ksize, sigmaX, sigmaY, borderType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_HoughCircles_const__InputArrayX_const__OutputArrayX_int_double_double_double_double_int_int(const cv::_InputArray* image, const cv::_OutputArray* circles, int method, double dp, double minDist, double param1, double param2, int minRadius, int maxRadius) {
		try {
			cv::HoughCircles(*image, *circles, method, dp, minDist, param1, param2, minRadius, maxRadius);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_HoughLinesP_const__InputArrayX_const__OutputArrayX_double_double_int_double_double(const cv::_InputArray* image, const cv::_OutputArray* lines, double rho, double theta, int threshold, double minLineLength, double maxLineGap) {
		try {
			cv::HoughLinesP(*image, *lines, rho, theta, threshold, minLineLength, maxLineGap);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_HoughLinesPointSet_const__InputArrayX_const__OutputArrayX_int_int_double_double_double_double_double_double(const cv::_InputArray* _point, const cv::_OutputArray* _lines, int lines_max, int threshold, double min_rho, double max_rho, double rho_step, double min_theta, double max_theta, double theta_step) {
		try {
			cv::HoughLinesPointSet(*_point, *_lines, lines_max, threshold, min_rho, max_rho, rho_step, min_theta, max_theta, theta_step);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_HoughLines_const__InputArrayX_const__OutputArrayX_double_double_int_double_double_double_double(const cv::_InputArray* image, const cv::_OutputArray* lines, double rho, double theta, int threshold, double srn, double stn, double min_theta, double max_theta) {
		try {
			cv::HoughLines(*image, *lines, rho, theta, threshold, srn, stn, min_theta, max_theta);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_HuMoments_const_MomentsX_const__OutputArrayX(const cv::Moments* m, const cv::_OutputArray* hu) {
		try {
			cv::HuMoments(*m, *hu);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_HuMoments_const_MomentsX_double_X__7_(const cv::Moments* moments, double(*hu)[7]) {
		try {
			cv::HuMoments(*moments, *hu);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_Laplacian_const__InputArrayX_const__OutputArrayX_int_int_double_double_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int ddepth, int ksize, double scale, double delta, int borderType) {
		try {
			cv::Laplacian(*src, *dst, ddepth, ksize, scale, delta, borderType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_Scharr_const__InputArrayX_const__OutputArrayX_int_int_int_double_double_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int ddepth, int dx, int dy, double scale, double delta, int borderType) {
		try {
			cv::Scharr(*src, *dst, ddepth, dx, dy, scale, delta, borderType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_Sobel_const__InputArrayX_const__OutputArrayX_int_int_int_int_double_double_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int ddepth, int dx, int dy, int ksize, double scale, double delta, int borderType) {
		try {
			cv::Sobel(*src, *dst, ddepth, dx, dy, ksize, scale, delta, borderType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_accumulateProduct_const__InputArrayX_const__InputArrayX_const__InputOutputArrayX_const__InputArrayX(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_InputOutputArray* dst, const cv::_InputArray* mask) {
		try {
			cv::accumulateProduct(*src1, *src2, *dst, *mask);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_accumulateSquare_const__InputArrayX_const__InputOutputArrayX_const__InputArrayX(const cv::_InputArray* src, const cv::_InputOutputArray* dst, const cv::_InputArray* mask) {
		try {
			cv::accumulateSquare(*src, *dst, *mask);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_accumulateWeighted_const__InputArrayX_const__InputOutputArrayX_double_const__InputArrayX(const cv::_InputArray* src, const cv::_InputOutputArray* dst, double alpha, const cv::_InputArray* mask) {
		try {
			cv::accumulateWeighted(*src, *dst, alpha, *mask);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_accumulate_const__InputArrayX_const__InputOutputArrayX_const__InputArrayX(const cv::_InputArray* src, const cv::_InputOutputArray* dst, const cv::_InputArray* mask) {
		try {
			cv::accumulate(*src, *dst, *mask);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_adaptiveThreshold_const__InputArrayX_const__OutputArrayX_double_int_int_int_double(const cv::_InputArray* src, const cv::_OutputArray* dst, double maxValue, int adaptiveMethod, int thresholdType, int blockSize, double C) {
		try {
			cv::adaptiveThreshold(*src, *dst, maxValue, adaptiveMethod, thresholdType, blockSize, C);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_applyColorMap_const__InputArrayX_const__OutputArrayX_const__InputArrayX(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* userColor) {
		try {
			cv::applyColorMap(*src, *dst, *userColor);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_applyColorMap_const__InputArrayX_const__OutputArrayX_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int colormap) {
		try {
			cv::applyColorMap(*src, *dst, colormap);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_approxPolyDP_const__InputArrayX_const__OutputArrayX_double_bool(const cv::_InputArray* curve, const cv::_OutputArray* approxCurve, double epsilon, bool closed) {
		try {
			cv::approxPolyDP(*curve, *approxCurve, epsilon, closed);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_arcLength_const__InputArrayX_bool(const cv::_InputArray* curve, bool closed) {
		try {
			double ret = cv::arcLength(*curve, closed);
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_arrowedLine_const__InputOutputArrayX_Point_Point_const_ScalarX_int_int_int_double(const cv::_InputOutputArray* img, const cv::Point* pt1, const cv::Point* pt2, const cv::Scalar* color, int thickness, int line_type, int shift, double tipLength) {
		try {
			cv::arrowedLine(*img, *pt1, *pt2, *color, thickness, line_type, shift, tipLength);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bilateralFilter_const__InputArrayX_const__OutputArrayX_int_double_double_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int d, double sigmaColor, double sigmaSpace, int borderType) {
		try {
			cv::bilateralFilter(*src, *dst, d, sigmaColor, sigmaSpace, borderType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_blendLinear_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__OutputArrayX(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_InputArray* weights1, const cv::_InputArray* weights2, const cv::_OutputArray* dst) {
		try {
			cv::blendLinear(*src1, *src2, *weights1, *weights2, *dst);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_blur_const__InputArrayX_const__OutputArrayX_Size_Point_int(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::Size* ksize, const cv::Point* anchor, int borderType) {
		try {
			cv::blur(*src, *dst, *ksize, *anchor, borderType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Rect> cv_boundingRect_const__InputArrayX(const cv::_InputArray* array) {
		try {
			cv::Rect ret = cv::boundingRect(*array);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Rect>)
	}
	
	Result_void cv_boxFilter_const__InputArrayX_const__OutputArrayX_int_Size_Point_bool_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int ddepth, const cv::Size* ksize, const cv::Point* anchor, bool normalize, int borderType) {
		try {
			cv::boxFilter(*src, *dst, ddepth, *ksize, *anchor, normalize, borderType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_boxPoints_RotatedRect_const__OutputArrayX(cv::RotatedRect* box, const cv::_OutputArray* points) {
		try {
			cv::boxPoints(*box, *points);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_buildPyramid_const__InputArrayX_const__OutputArrayX_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int maxlevel, int borderType) {
		try {
			cv::buildPyramid(*src, *dst, maxlevel, borderType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_calcBackProject_const__InputArrayX_const_vector_int_X_const__InputArrayX_const__OutputArrayX_const_vector_float_X_double(const cv::_InputArray* images, const std::vector<int>* channels, const cv::_InputArray* hist, const cv::_OutputArray* dst, const std::vector<float>* ranges, double scale) {
		try {
			cv::calcBackProject(*images, *channels, *hist, *dst, *ranges, scale);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_calcHist_const__InputArrayX_const_vector_int_X_const__InputArrayX_const__OutputArrayX_const_vector_int_X_const_vector_float_X_bool(const cv::_InputArray* images, const std::vector<int>* channels, const cv::_InputArray* mask, const cv::_OutputArray* hist, const std::vector<int>* histSize, const std::vector<float>* ranges, bool accumulate) {
		try {
			cv::calcHist(*images, *channels, *mask, *hist, *histSize, *ranges, accumulate);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_circle_const__InputOutputArrayX_Point_int_const_ScalarX_int_int_int(const cv::_InputOutputArray* img, const cv::Point* center, int radius, const cv::Scalar* color, int thickness, int lineType, int shift) {
		try {
			cv::circle(*img, *center, radius, *color, thickness, lineType, shift);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_clipLine_Rect_PointX_PointX(const cv::Rect* imgRect, cv::Point* pt1, cv::Point* pt2) {
		try {
			bool ret = cv::clipLine(*imgRect, *pt1, *pt2);
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_clipLine_Size2l_Point2lX_Point2lX(const cv::Size2l* imgSize, cv::Point2l* pt1, cv::Point2l* pt2) {
		try {
			bool ret = cv::clipLine(*imgSize, *pt1, *pt2);
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_clipLine_Size_PointX_PointX(const cv::Size* imgSize, cv::Point* pt1, cv::Point* pt2) {
		try {
			bool ret = cv::clipLine(*imgSize, *pt1, *pt2);
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<double> cv_compareHist_const_SparseMatX_const_SparseMatX_int(const cv::SparseMat* H1, const cv::SparseMat* H2, int method) {
		try {
			double ret = cv::compareHist(*H1, *H2, method);
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result<double> cv_compareHist_const__InputArrayX_const__InputArrayX_int(const cv::_InputArray* H1, const cv::_InputArray* H2, int method) {
		try {
			double ret = cv::compareHist(*H1, *H2, method);
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result<int> cv_connectedComponentsWithStats_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_int_int(const cv::_InputArray* image, const cv::_OutputArray* labels, const cv::_OutputArray* stats, const cv::_OutputArray* centroids, int connectivity, int ltype) {
		try {
			int ret = cv::connectedComponentsWithStats(*image, *labels, *stats, *centroids, connectivity, ltype);
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_connectedComponentsWithStats_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_int_int_int(const cv::_InputArray* image, const cv::_OutputArray* labels, const cv::_OutputArray* stats, const cv::_OutputArray* centroids, int connectivity, int ltype, int ccltype) {
		try {
			int ret = cv::connectedComponentsWithStats(*image, *labels, *stats, *centroids, connectivity, ltype, ccltype);
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_connectedComponents_const__InputArrayX_const__OutputArrayX_int_int(const cv::_InputArray* image, const cv::_OutputArray* labels, int connectivity, int ltype) {
		try {
			int ret = cv::connectedComponents(*image, *labels, connectivity, ltype);
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_connectedComponents_const__InputArrayX_const__OutputArrayX_int_int_int(const cv::_InputArray* image, const cv::_OutputArray* labels, int connectivity, int ltype, int ccltype) {
		try {
			int ret = cv::connectedComponents(*image, *labels, connectivity, ltype, ccltype);
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<double> cv_contourArea_const__InputArrayX_bool(const cv::_InputArray* contour, bool oriented) {
		try {
			double ret = cv::contourArea(*contour, oriented);
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_convertMaps_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_int_bool(const cv::_InputArray* map1, const cv::_InputArray* map2, const cv::_OutputArray* dstmap1, const cv::_OutputArray* dstmap2, int dstmap1type, bool nninterpolation) {
		try {
			cv::convertMaps(*map1, *map2, *dstmap1, *dstmap2, dstmap1type, nninterpolation);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_convexHull_const__InputArrayX_const__OutputArrayX_bool_bool(const cv::_InputArray* points, const cv::_OutputArray* hull, bool clockwise, bool returnPoints) {
		try {
			cv::convexHull(*points, *hull, clockwise, returnPoints);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_convexityDefects_const__InputArrayX_const__InputArrayX_const__OutputArrayX(const cv::_InputArray* contour, const cv::_InputArray* convexhull, const cv::_OutputArray* convexityDefects) {
		try {
			cv::convexityDefects(*contour, *convexhull, *convexityDefects);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_cornerEigenValsAndVecs_const__InputArrayX_const__OutputArrayX_int_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int blockSize, int ksize, int borderType) {
		try {
			cv::cornerEigenValsAndVecs(*src, *dst, blockSize, ksize, borderType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_cornerHarris_const__InputArrayX_const__OutputArrayX_int_int_double_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int blockSize, int ksize, double k, int borderType) {
		try {
			cv::cornerHarris(*src, *dst, blockSize, ksize, k, borderType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_cornerMinEigenVal_const__InputArrayX_const__OutputArrayX_int_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int blockSize, int ksize, int borderType) {
		try {
			cv::cornerMinEigenVal(*src, *dst, blockSize, ksize, borderType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_cornerSubPix_const__InputArrayX_const__InputOutputArrayX_Size_Size_TermCriteria(const cv::_InputArray* image, const cv::_InputOutputArray* corners, const cv::Size* winSize, const cv::Size* zeroZone, const cv::TermCriteria* criteria) {
		try {
			cv::cornerSubPix(*image, *corners, *winSize, *zeroZone, *criteria);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Ptr<cv::CLAHE>*> cv_createCLAHE_double_Size(double clipLimit, const cv::Size* tileGridSize) {
		try {
			cv::Ptr<cv::CLAHE> ret = cv::createCLAHE(clipLimit, *tileGridSize);
			return Ok(new cv::Ptr<cv::CLAHE>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::CLAHE>*>)
	}
	
	Result<cv::Ptr<cv::GeneralizedHoughBallard>*> cv_createGeneralizedHoughBallard() {
		try {
			cv::Ptr<cv::GeneralizedHoughBallard> ret = cv::createGeneralizedHoughBallard();
			return Ok(new cv::Ptr<cv::GeneralizedHoughBallard>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::GeneralizedHoughBallard>*>)
	}
	
	Result<cv::Ptr<cv::GeneralizedHoughGuil>*> cv_createGeneralizedHoughGuil() {
		try {
			cv::Ptr<cv::GeneralizedHoughGuil> ret = cv::createGeneralizedHoughGuil();
			return Ok(new cv::Ptr<cv::GeneralizedHoughGuil>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::GeneralizedHoughGuil>*>)
	}
	
	Result_void cv_createHanningWindow_const__OutputArrayX_Size_int(const cv::_OutputArray* dst, const cv::Size* winSize, int type) {
		try {
			cv::createHanningWindow(*dst, *winSize, type);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Ptr<cv::LineSegmentDetector>*> cv_createLineSegmentDetector_int_double_double_double_double_double_double_int(int _refine, double _scale, double _sigma_scale, double _quant, double _ang_th, double _log_eps, double _density_th, int _n_bins) {
		try {
			cv::Ptr<cv::LineSegmentDetector> ret = cv::createLineSegmentDetector(_refine, _scale, _sigma_scale, _quant, _ang_th, _log_eps, _density_th, _n_bins);
			return Ok(new cv::Ptr<cv::LineSegmentDetector>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::LineSegmentDetector>*>)
	}
	
	Result_void cv_cvtColorTwoPlane_const__InputArrayX_const__InputArrayX_const__OutputArrayX_int(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, int code) {
		try {
			cv::cvtColorTwoPlane(*src1, *src2, *dst, code);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_cvtColor_const__InputArrayX_const__OutputArrayX_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int code, int dstCn) {
		try {
			cv::cvtColor(*src, *dst, code, dstCn);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_demosaicing_const__InputArrayX_const__OutputArrayX_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int code, int dstCn) {
		try {
			cv::demosaicing(*src, *dst, code, dstCn);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_dilate_const__InputArrayX_const__OutputArrayX_const__InputArrayX_Point_int_int_const_ScalarX(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* kernel, const cv::Point* anchor, int iterations, int borderType, const cv::Scalar* borderValue) {
		try {
			cv::dilate(*src, *dst, *kernel, *anchor, iterations, borderType, *borderValue);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_distanceTransform_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_int_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_OutputArray* labels, int distanceType, int maskSize, int labelType) {
		try {
			cv::distanceTransform(*src, *dst, *labels, distanceType, maskSize, labelType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_distanceTransform_const__InputArrayX_const__OutputArrayX_int_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int distanceType, int maskSize, int dstType) {
		try {
			cv::distanceTransform(*src, *dst, distanceType, maskSize, dstType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_drawContours_const__InputOutputArrayX_const__InputArrayX_int_const_ScalarX_int_int_const__InputArrayX_int_Point(const cv::_InputOutputArray* image, const cv::_InputArray* contours, int contourIdx, const cv::Scalar* color, int thickness, int lineType, const cv::_InputArray* hierarchy, int maxLevel, const cv::Point* offset) {
		try {
			cv::drawContours(*image, *contours, contourIdx, *color, thickness, lineType, *hierarchy, maxLevel, *offset);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_drawMarker_const__InputOutputArrayX_Point_const_ScalarX_int_int_int_int(const cv::_InputOutputArray* img, const cv::Point* position, const cv::Scalar* color, int markerType, int markerSize, int thickness, int line_type) {
		try {
			cv::drawMarker(*img, *position, *color, markerType, markerSize, thickness, line_type);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ellipse2Poly_Point2d_Size2d_int_int_int_int_vector_Point2d_X(const cv::Point2d* center, const cv::Size2d* axes, int angle, int arcStart, int arcEnd, int delta, std::vector<cv::Point2d>* pts) {
		try {
			cv::ellipse2Poly(*center, *axes, angle, arcStart, arcEnd, delta, *pts);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ellipse2Poly_Point_Size_int_int_int_int_vector_Point_X(const cv::Point* center, const cv::Size* axes, int angle, int arcStart, int arcEnd, int delta, std::vector<cv::Point>* pts) {
		try {
			cv::ellipse2Poly(*center, *axes, angle, arcStart, arcEnd, delta, *pts);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ellipse_const__InputOutputArrayX_Point_Size_double_double_double_const_ScalarX_int_int_int(const cv::_InputOutputArray* img, const cv::Point* center, const cv::Size* axes, double angle, double startAngle, double endAngle, const cv::Scalar* color, int thickness, int lineType, int shift) {
		try {
			cv::ellipse(*img, *center, *axes, angle, startAngle, endAngle, *color, thickness, lineType, shift);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ellipse_const__InputOutputArrayX_const_RotatedRectX_const_ScalarX_int_int(const cv::_InputOutputArray* img, const cv::RotatedRect* box, const cv::Scalar* color, int thickness, int lineType) {
		try {
			cv::ellipse(*img, *box, *color, thickness, lineType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_equalizeHist_const__InputArrayX_const__OutputArrayX(const cv::_InputArray* src, const cv::_OutputArray* dst) {
		try {
			cv::equalizeHist(*src, *dst);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_erode_const__InputArrayX_const__OutputArrayX_const__InputArrayX_Point_int_int_const_ScalarX(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* kernel, const cv::Point* anchor, int iterations, int borderType, const cv::Scalar* borderValue) {
		try {
			cv::erode(*src, *dst, *kernel, *anchor, iterations, borderType, *borderValue);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_fillConvexPoly_const__InputOutputArrayX_const__InputArrayX_const_ScalarX_int_int(const cv::_InputOutputArray* img, const cv::_InputArray* points, const cv::Scalar* color, int lineType, int shift) {
		try {
			cv::fillConvexPoly(*img, *points, *color, lineType, shift);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_fillPoly_const__InputOutputArrayX_const__InputArrayX_const_ScalarX_int_int_Point(const cv::_InputOutputArray* img, const cv::_InputArray* pts, const cv::Scalar* color, int lineType, int shift, const cv::Point* offset) {
		try {
			cv::fillPoly(*img, *pts, *color, lineType, shift, *offset);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_filter2D_const__InputArrayX_const__OutputArrayX_int_const__InputArrayX_Point_double_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int ddepth, const cv::_InputArray* kernel, const cv::Point* anchor, double delta, int borderType) {
		try {
			cv::filter2D(*src, *dst, ddepth, *kernel, *anchor, delta, borderType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_findContours_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_int_int_Point(const cv::_InputArray* image, const cv::_OutputArray* contours, const cv::_OutputArray* hierarchy, int mode, int method, const cv::Point* offset) {
		try {
			cv::findContours(*image, *contours, *hierarchy, mode, method, *offset);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_findContours_const__InputArrayX_const__OutputArrayX_int_int_Point(const cv::_InputArray* image, const cv::_OutputArray* contours, int mode, int method, const cv::Point* offset) {
		try {
			cv::findContours(*image, *contours, mode, method, *offset);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::RotatedRect*> cv_fitEllipseAMS_const__InputArrayX(const cv::_InputArray* points) {
		try {
			cv::RotatedRect ret = cv::fitEllipseAMS(*points);
			return Ok(new cv::RotatedRect(ret));
		} OCVRS_CATCH(Result<cv::RotatedRect*>)
	}
	
	Result<cv::RotatedRect*> cv_fitEllipseDirect_const__InputArrayX(const cv::_InputArray* points) {
		try {
			cv::RotatedRect ret = cv::fitEllipseDirect(*points);
			return Ok(new cv::RotatedRect(ret));
		} OCVRS_CATCH(Result<cv::RotatedRect*>)
	}
	
	Result<cv::RotatedRect*> cv_fitEllipse_const__InputArrayX(const cv::_InputArray* points) {
		try {
			cv::RotatedRect ret = cv::fitEllipse(*points);
			return Ok(new cv::RotatedRect(ret));
		} OCVRS_CATCH(Result<cv::RotatedRect*>)
	}
	
	Result_void cv_fitLine_const__InputArrayX_const__OutputArrayX_int_double_double_double(const cv::_InputArray* points, const cv::_OutputArray* line, int distType, double param, double reps, double aeps) {
		try {
			cv::fitLine(*points, *line, distType, param, reps, aeps);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_floodFill_const__InputOutputArrayX_Point_Scalar_RectX_Scalar_Scalar_int(const cv::_InputOutputArray* image, const cv::Point* seedPoint, const cv::Scalar* newVal, cv::Rect* rect, const cv::Scalar* loDiff, const cv::Scalar* upDiff, int flags) {
		try {
			int ret = cv::floodFill(*image, *seedPoint, *newVal, rect, *loDiff, *upDiff, flags);
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_floodFill_const__InputOutputArrayX_const__InputOutputArrayX_Point_Scalar_RectX_Scalar_Scalar_int(const cv::_InputOutputArray* image, const cv::_InputOutputArray* mask, const cv::Point* seedPoint, const cv::Scalar* newVal, cv::Rect* rect, const cv::Scalar* loDiff, const cv::Scalar* upDiff, int flags) {
		try {
			int ret = cv::floodFill(*image, *mask, *seedPoint, *newVal, rect, *loDiff, *upDiff, flags);
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<cv::Mat*> cv_getAffineTransform_const_Point2fX_const_Point2fX(const cv::Point2f* src, const cv::Point2f* dst) {
		try {
			cv::Mat ret = cv::getAffineTransform(src, dst);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result<cv::Mat*> cv_getAffineTransform_const__InputArrayX_const__InputArrayX(const cv::_InputArray* src, const cv::_InputArray* dst) {
		try {
			cv::Mat ret = cv::getAffineTransform(*src, *dst);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result_void cv_getDerivKernels_const__OutputArrayX_const__OutputArrayX_int_int_int_bool_int(const cv::_OutputArray* kx, const cv::_OutputArray* ky, int dx, int dy, int ksize, bool normalize, int ktype) {
		try {
			cv::getDerivKernels(*kx, *ky, dx, dy, ksize, normalize, ktype);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_getFontScaleFromHeight_int_int_int(int fontFace, int pixelHeight, int thickness) {
		try {
			double ret = cv::getFontScaleFromHeight(fontFace, pixelHeight, thickness);
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result<cv::Mat*> cv_getGaborKernel_Size_double_double_double_double_double_int(const cv::Size* ksize, double sigma, double theta, double lambd, double gamma, double psi, int ktype) {
		try {
			cv::Mat ret = cv::getGaborKernel(*ksize, sigma, theta, lambd, gamma, psi, ktype);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result<cv::Mat*> cv_getGaussianKernel_int_double_int(int ksize, double sigma, int ktype) {
		try {
			cv::Mat ret = cv::getGaussianKernel(ksize, sigma, ktype);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result<cv::Mat*> cv_getPerspectiveTransform_const_Point2fX_const_Point2fX_int(const cv::Point2f* src, const cv::Point2f* dst, int solveMethod) {
		try {
			cv::Mat ret = cv::getPerspectiveTransform(src, dst, solveMethod);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result<cv::Mat*> cv_getPerspectiveTransform_const__InputArrayX_const__InputArrayX_int(const cv::_InputArray* src, const cv::_InputArray* dst, int solveMethod) {
		try {
			cv::Mat ret = cv::getPerspectiveTransform(*src, *dst, solveMethod);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result_void cv_getRectSubPix_const__InputArrayX_Size_Point2f_const__OutputArrayX_int(const cv::_InputArray* image, const cv::Size* patchSize, const cv::Point2f* center, const cv::_OutputArray* patch, int patchType) {
		try {
			cv::getRectSubPix(*image, *patchSize, *center, *patch, patchType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Mat*> cv_getRotationMatrix2D_Point2f_double_double(const cv::Point2f* center, double angle, double scale) {
		try {
			cv::Mat ret = cv::getRotationMatrix2D(*center, angle, scale);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result<cv::Matx23d> cv_getRotationMatrix2D__Point2f_double_double(const cv::Point2f* center, double angle, double scale) {
		try {
			cv::Matx23d ret = cv::getRotationMatrix2D_(*center, angle, scale);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Matx23d>)
	}
	
	Result<cv::Mat*> cv_getStructuringElement_int_Size_Point(int shape, const cv::Size* ksize, const cv::Point* anchor) {
		try {
			cv::Mat ret = cv::getStructuringElement(shape, *ksize, *anchor);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result<cv::Size> cv_getTextSize_const_StringX_int_double_int_intX(const char* text, int fontFace, double fontScale, int thickness, int* baseLine) {
		try {
			cv::Size ret = cv::getTextSize(std::string(text), fontFace, fontScale, thickness, baseLine);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_goodFeaturesToTrack_const__InputArrayX_const__OutputArrayX_int_double_double_const__InputArrayX_int_bool_double(const cv::_InputArray* image, const cv::_OutputArray* corners, int maxCorners, double qualityLevel, double minDistance, const cv::_InputArray* mask, int blockSize, bool useHarrisDetector, double k) {
		try {
			cv::goodFeaturesToTrack(*image, *corners, maxCorners, qualityLevel, minDistance, *mask, blockSize, useHarrisDetector, k);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_goodFeaturesToTrack_const__InputArrayX_const__OutputArrayX_int_double_double_const__InputArrayX_int_int_bool_double(const cv::_InputArray* image, const cv::_OutputArray* corners, int maxCorners, double qualityLevel, double minDistance, const cv::_InputArray* mask, int blockSize, int gradientSize, bool useHarrisDetector, double k) {
		try {
			cv::goodFeaturesToTrack(*image, *corners, maxCorners, qualityLevel, minDistance, *mask, blockSize, gradientSize, useHarrisDetector, k);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_grabCut_const__InputArrayX_const__InputOutputArrayX_Rect_const__InputOutputArrayX_const__InputOutputArrayX_int_int(const cv::_InputArray* img, const cv::_InputOutputArray* mask, const cv::Rect* rect, const cv::_InputOutputArray* bgdModel, const cv::_InputOutputArray* fgdModel, int iterCount, int mode) {
		try {
			cv::grabCut(*img, *mask, *rect, *bgdModel, *fgdModel, iterCount, mode);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_integral_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_int_int(const cv::_InputArray* src, const cv::_OutputArray* sum, const cv::_OutputArray* sqsum, const cv::_OutputArray* tilted, int sdepth, int sqdepth) {
		try {
			cv::integral(*src, *sum, *sqsum, *tilted, sdepth, sqdepth);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_integral_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_int_int(const cv::_InputArray* src, const cv::_OutputArray* sum, const cv::_OutputArray* sqsum, int sdepth, int sqdepth) {
		try {
			cv::integral(*src, *sum, *sqsum, sdepth, sqdepth);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_integral_const__InputArrayX_const__OutputArrayX_int(const cv::_InputArray* src, const cv::_OutputArray* sum, int sdepth) {
		try {
			cv::integral(*src, *sum, sdepth);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_intersectConvexConvex_const__InputArrayX_const__InputArrayX_const__OutputArrayX_bool(const cv::_InputArray* _p1, const cv::_InputArray* _p2, const cv::_OutputArray* _p12, bool handleNested) {
		try {
			float ret = cv::intersectConvexConvex(*_p1, *_p2, *_p12, handleNested);
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_invertAffineTransform_const__InputArrayX_const__OutputArrayX(const cv::_InputArray* M, const cv::_OutputArray* iM) {
		try {
			cv::invertAffineTransform(*M, *iM);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_isContourConvex_const__InputArrayX(const cv::_InputArray* contour) {
		try {
			bool ret = cv::isContourConvex(*contour);
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_line_const__InputOutputArrayX_Point_Point_const_ScalarX_int_int_int(const cv::_InputOutputArray* img, const cv::Point* pt1, const cv::Point* pt2, const cv::Scalar* color, int thickness, int lineType, int shift) {
		try {
			cv::line(*img, *pt1, *pt2, *color, thickness, lineType, shift);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_linearPolar_const__InputArrayX_const__OutputArrayX_Point2f_double_int(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::Point2f* center, double maxRadius, int flags) {
		try {
			cv::linearPolar(*src, *dst, *center, maxRadius, flags);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_logPolar_const__InputArrayX_const__OutputArrayX_Point2f_double_int(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::Point2f* center, double M, int flags) {
		try {
			cv::logPolar(*src, *dst, *center, M, flags);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_matchShapes_const__InputArrayX_const__InputArrayX_int_double(const cv::_InputArray* contour1, const cv::_InputArray* contour2, int method, double parameter) {
		try {
			double ret = cv::matchShapes(*contour1, *contour2, method, parameter);
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_matchTemplate_const__InputArrayX_const__InputArrayX_const__OutputArrayX_int_const__InputArrayX(const cv::_InputArray* image, const cv::_InputArray* templ, const cv::_OutputArray* result, int method, const cv::_InputArray* mask) {
		try {
			cv::matchTemplate(*image, *templ, *result, method, *mask);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_medianBlur_const__InputArrayX_const__OutputArrayX_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int ksize) {
		try {
			cv::medianBlur(*src, *dst, ksize);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::RotatedRect*> cv_minAreaRect_const__InputArrayX(const cv::_InputArray* points) {
		try {
			cv::RotatedRect ret = cv::minAreaRect(*points);
			return Ok(new cv::RotatedRect(ret));
		} OCVRS_CATCH(Result<cv::RotatedRect*>)
	}
	
	Result_void cv_minEnclosingCircle_const__InputArrayX_Point2fX_floatX(const cv::_InputArray* points, cv::Point2f* center, float* radius) {
		try {
			cv::minEnclosingCircle(*points, *center, *radius);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_minEnclosingTriangle_const__InputArrayX_const__OutputArrayX(const cv::_InputArray* points, const cv::_OutputArray* triangle) {
		try {
			double ret = cv::minEnclosingTriangle(*points, *triangle);
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result<cv::Moments> cv_moments_const__InputArrayX_bool(const cv::_InputArray* array, bool binaryImage) {
		try {
			cv::Moments ret = cv::moments(*array, binaryImage);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Moments>)
	}
	
	Result<cv::Scalar> cv_morphologyDefaultBorderValue() {
		try {
			cv::Scalar ret = cv::morphologyDefaultBorderValue();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Scalar>)
	}
	
	Result_void cv_morphologyEx_const__InputArrayX_const__OutputArrayX_int_const__InputArrayX_Point_int_int_const_ScalarX(const cv::_InputArray* src, const cv::_OutputArray* dst, int op, const cv::_InputArray* kernel, const cv::Point* anchor, int iterations, int borderType, const cv::Scalar* borderValue) {
		try {
			cv::morphologyEx(*src, *dst, op, *kernel, *anchor, iterations, borderType, *borderValue);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Point2d> cv_phaseCorrelate_const__InputArrayX_const__InputArrayX_const__InputArrayX_doubleX(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_InputArray* window, double* response) {
		try {
			cv::Point2d ret = cv::phaseCorrelate(*src1, *src2, *window, response);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Point2d>)
	}
	
	Result<double> cv_pointPolygonTest_const__InputArrayX_Point2f_bool(const cv::_InputArray* contour, const cv::Point2f* pt, bool measureDist) {
		try {
			double ret = cv::pointPolygonTest(*contour, *pt, measureDist);
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_polylines_const__InputOutputArrayX_const__InputArrayX_bool_const_ScalarX_int_int_int(const cv::_InputOutputArray* img, const cv::_InputArray* pts, bool isClosed, const cv::Scalar* color, int thickness, int lineType, int shift) {
		try {
			cv::polylines(*img, *pts, isClosed, *color, thickness, lineType, shift);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_preCornerDetect_const__InputArrayX_const__OutputArrayX_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int ksize, int borderType) {
		try {
			cv::preCornerDetect(*src, *dst, ksize, borderType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_putText_const__InputOutputArrayX_const_StringX_Point_int_double_Scalar_int_int_bool(const cv::_InputOutputArray* img, const char* text, const cv::Point* org, int fontFace, double fontScale, const cv::Scalar* color, int thickness, int lineType, bool bottomLeftOrigin) {
		try {
			cv::putText(*img, std::string(text), *org, fontFace, fontScale, *color, thickness, lineType, bottomLeftOrigin);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_pyrDown_const__InputArrayX_const__OutputArrayX_const_SizeX_int(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::Size* dstsize, int borderType) {
		try {
			cv::pyrDown(*src, *dst, *dstsize, borderType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_pyrMeanShiftFiltering_const__InputArrayX_const__OutputArrayX_double_double_int_TermCriteria(const cv::_InputArray* src, const cv::_OutputArray* dst, double sp, double sr, int maxLevel, const cv::TermCriteria* termcrit) {
		try {
			cv::pyrMeanShiftFiltering(*src, *dst, sp, sr, maxLevel, *termcrit);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_pyrUp_const__InputArrayX_const__OutputArrayX_const_SizeX_int(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::Size* dstsize, int borderType) {
		try {
			cv::pyrUp(*src, *dst, *dstsize, borderType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_rectangle_const__InputOutputArrayX_Point_Point_const_ScalarX_int_int_int(const cv::_InputOutputArray* img, const cv::Point* pt1, const cv::Point* pt2, const cv::Scalar* color, int thickness, int lineType, int shift) {
		try {
			cv::rectangle(*img, *pt1, *pt2, *color, thickness, lineType, shift);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_rectangle_const__InputOutputArrayX_Rect_const_ScalarX_int_int_int(const cv::_InputOutputArray* img, const cv::Rect* rec, const cv::Scalar* color, int thickness, int lineType, int shift) {
		try {
			cv::rectangle(*img, *rec, *color, thickness, lineType, shift);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_remap_const__InputArrayX_const__OutputArrayX_const__InputArrayX_const__InputArrayX_int_int_const_ScalarX(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* map1, const cv::_InputArray* map2, int interpolation, int borderMode, const cv::Scalar* borderValue) {
		try {
			cv::remap(*src, *dst, *map1, *map2, interpolation, borderMode, *borderValue);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_resize_const__InputArrayX_const__OutputArrayX_Size_double_double_int(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::Size* dsize, double fx, double fy, int interpolation) {
		try {
			cv::resize(*src, *dst, *dsize, fx, fy, interpolation);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_rotatedRectangleIntersection_const_RotatedRectX_const_RotatedRectX_const__OutputArrayX(const cv::RotatedRect* rect1, const cv::RotatedRect* rect2, const cv::_OutputArray* intersectingRegion) {
		try {
			int ret = cv::rotatedRectangleIntersection(*rect1, *rect2, *intersectingRegion);
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_sepFilter2D_const__InputArrayX_const__OutputArrayX_int_const__InputArrayX_const__InputArrayX_Point_double_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int ddepth, const cv::_InputArray* kernelX, const cv::_InputArray* kernelY, const cv::Point* anchor, double delta, int borderType) {
		try {
			cv::sepFilter2D(*src, *dst, ddepth, *kernelX, *kernelY, *anchor, delta, borderType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_spatialGradient_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_int_int(const cv::_InputArray* src, const cv::_OutputArray* dx, const cv::_OutputArray* dy, int ksize, int borderType) {
		try {
			cv::spatialGradient(*src, *dx, *dy, ksize, borderType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_sqrBoxFilter_const__InputArrayX_const__OutputArrayX_int_Size_Point_bool_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int ddepth, const cv::Size* ksize, const cv::Point* anchor, bool normalize, int borderType) {
		try {
			cv::sqrBoxFilter(*src, *dst, ddepth, *ksize, *anchor, normalize, borderType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_threshold_const__InputArrayX_const__OutputArrayX_double_double_int(const cv::_InputArray* src, const cv::_OutputArray* dst, double thresh, double maxval, int type) {
		try {
			double ret = cv::threshold(*src, *dst, thresh, maxval, type);
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_warpAffine_const__InputArrayX_const__OutputArrayX_const__InputArrayX_Size_int_int_const_ScalarX(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* M, const cv::Size* dsize, int flags, int borderMode, const cv::Scalar* borderValue) {
		try {
			cv::warpAffine(*src, *dst, *M, *dsize, flags, borderMode, *borderValue);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_warpPerspective_const__InputArrayX_const__OutputArrayX_const__InputArrayX_Size_int_int_const_ScalarX(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* M, const cv::Size* dsize, int flags, int borderMode, const cv::Scalar* borderValue) {
		try {
			cv::warpPerspective(*src, *dst, *M, *dsize, flags, borderMode, *borderValue);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_warpPolar_const__InputArrayX_const__OutputArrayX_Size_Point2f_double_int(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::Size* dsize, const cv::Point2f* center, double maxRadius, int flags) {
		try {
			cv::warpPolar(*src, *dst, *dsize, *center, maxRadius, flags);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_watershed_const__InputArrayX_const__InputOutputArrayX(const cv::_InputArray* image, const cv::_InputOutputArray* markers) {
		try {
			cv::watershed(*image, *markers);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_wrapperEMD_const__InputArrayX_const__InputArrayX_int_const__InputArrayX_Ptr_float__const__OutputArrayX(const cv::_InputArray* signature1, const cv::_InputArray* signature2, int distType, const cv::_InputArray* cost, cv::Ptr<float>* lowerBound, const cv::_OutputArray* flow) {
		try {
			float ret = cv::wrapperEMD(*signature1, *signature2, distType, *cost, *lowerBound, *flow);
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_CLAHE_apply_const__InputArrayX_const__OutputArrayX(cv::CLAHE* instance, const cv::_InputArray* src, const cv::_OutputArray* dst) {
		try {
			instance->apply(*src, *dst);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_CLAHE_setClipLimit_double(cv::CLAHE* instance, double clipLimit) {
		try {
			instance->setClipLimit(clipLimit);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_CLAHE_getClipLimit_const(const cv::CLAHE* instance) {
		try {
			double ret = instance->getClipLimit();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_CLAHE_setTilesGridSize_Size(cv::CLAHE* instance, const cv::Size* tileGridSize) {
		try {
			instance->setTilesGridSize(*tileGridSize);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Size> cv_CLAHE_getTilesGridSize_const(const cv::CLAHE* instance) {
		try {
			cv::Size ret = instance->getTilesGridSize();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_CLAHE_collectGarbage(cv::CLAHE* instance) {
		try {
			instance->collectGarbage();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_GeneralizedHough_setTemplate_const__InputArrayX_Point(cv::GeneralizedHough* instance, const cv::_InputArray* templ, const cv::Point* templCenter) {
		try {
			instance->setTemplate(*templ, *templCenter);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_GeneralizedHough_setTemplate_const__InputArrayX_const__InputArrayX_const__InputArrayX_Point(cv::GeneralizedHough* instance, const cv::_InputArray* edges, const cv::_InputArray* dx, const cv::_InputArray* dy, const cv::Point* templCenter) {
		try {
			instance->setTemplate(*edges, *dx, *dy, *templCenter);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_GeneralizedHough_detect_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(cv::GeneralizedHough* instance, const cv::_InputArray* image, const cv::_OutputArray* positions, const cv::_OutputArray* votes) {
		try {
			instance->detect(*image, *positions, *votes);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_GeneralizedHough_detect_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(cv::GeneralizedHough* instance, const cv::_InputArray* edges, const cv::_InputArray* dx, const cv::_InputArray* dy, const cv::_OutputArray* positions, const cv::_OutputArray* votes) {
		try {
			instance->detect(*edges, *dx, *dy, *positions, *votes);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_GeneralizedHough_setCannyLowThresh_int(cv::GeneralizedHough* instance, int cannyLowThresh) {
		try {
			instance->setCannyLowThresh(cannyLowThresh);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_GeneralizedHough_getCannyLowThresh_const(const cv::GeneralizedHough* instance) {
		try {
			int ret = instance->getCannyLowThresh();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_GeneralizedHough_setCannyHighThresh_int(cv::GeneralizedHough* instance, int cannyHighThresh) {
		try {
			instance->setCannyHighThresh(cannyHighThresh);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_GeneralizedHough_getCannyHighThresh_const(const cv::GeneralizedHough* instance) {
		try {
			int ret = instance->getCannyHighThresh();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_GeneralizedHough_setMinDist_double(cv::GeneralizedHough* instance, double minDist) {
		try {
			instance->setMinDist(minDist);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_GeneralizedHough_getMinDist_const(const cv::GeneralizedHough* instance) {
		try {
			double ret = instance->getMinDist();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_GeneralizedHough_setDp_double(cv::GeneralizedHough* instance, double dp) {
		try {
			instance->setDp(dp);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_GeneralizedHough_getDp_const(const cv::GeneralizedHough* instance) {
		try {
			double ret = instance->getDp();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_GeneralizedHough_setMaxBufferSize_int(cv::GeneralizedHough* instance, int maxBufferSize) {
		try {
			instance->setMaxBufferSize(maxBufferSize);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_GeneralizedHough_getMaxBufferSize_const(const cv::GeneralizedHough* instance) {
		try {
			int ret = instance->getMaxBufferSize();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_GeneralizedHoughBallard_setLevels_int(cv::GeneralizedHoughBallard* instance, int levels) {
		try {
			instance->setLevels(levels);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_GeneralizedHoughBallard_getLevels_const(const cv::GeneralizedHoughBallard* instance) {
		try {
			int ret = instance->getLevels();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_GeneralizedHoughBallard_setVotesThreshold_int(cv::GeneralizedHoughBallard* instance, int votesThreshold) {
		try {
			instance->setVotesThreshold(votesThreshold);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_GeneralizedHoughBallard_getVotesThreshold_const(const cv::GeneralizedHoughBallard* instance) {
		try {
			int ret = instance->getVotesThreshold();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_GeneralizedHoughGuil_setXi_double(cv::GeneralizedHoughGuil* instance, double xi) {
		try {
			instance->setXi(xi);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_GeneralizedHoughGuil_getXi_const(const cv::GeneralizedHoughGuil* instance) {
		try {
			double ret = instance->getXi();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_GeneralizedHoughGuil_setLevels_int(cv::GeneralizedHoughGuil* instance, int levels) {
		try {
			instance->setLevels(levels);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_GeneralizedHoughGuil_getLevels_const(const cv::GeneralizedHoughGuil* instance) {
		try {
			int ret = instance->getLevels();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_GeneralizedHoughGuil_setAngleEpsilon_double(cv::GeneralizedHoughGuil* instance, double angleEpsilon) {
		try {
			instance->setAngleEpsilon(angleEpsilon);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_GeneralizedHoughGuil_getAngleEpsilon_const(const cv::GeneralizedHoughGuil* instance) {
		try {
			double ret = instance->getAngleEpsilon();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_GeneralizedHoughGuil_setMinAngle_double(cv::GeneralizedHoughGuil* instance, double minAngle) {
		try {
			instance->setMinAngle(minAngle);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_GeneralizedHoughGuil_getMinAngle_const(const cv::GeneralizedHoughGuil* instance) {
		try {
			double ret = instance->getMinAngle();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_GeneralizedHoughGuil_setMaxAngle_double(cv::GeneralizedHoughGuil* instance, double maxAngle) {
		try {
			instance->setMaxAngle(maxAngle);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_GeneralizedHoughGuil_getMaxAngle_const(const cv::GeneralizedHoughGuil* instance) {
		try {
			double ret = instance->getMaxAngle();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_GeneralizedHoughGuil_setAngleStep_double(cv::GeneralizedHoughGuil* instance, double angleStep) {
		try {
			instance->setAngleStep(angleStep);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_GeneralizedHoughGuil_getAngleStep_const(const cv::GeneralizedHoughGuil* instance) {
		try {
			double ret = instance->getAngleStep();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_GeneralizedHoughGuil_setAngleThresh_int(cv::GeneralizedHoughGuil* instance, int angleThresh) {
		try {
			instance->setAngleThresh(angleThresh);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_GeneralizedHoughGuil_getAngleThresh_const(const cv::GeneralizedHoughGuil* instance) {
		try {
			int ret = instance->getAngleThresh();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_GeneralizedHoughGuil_setMinScale_double(cv::GeneralizedHoughGuil* instance, double minScale) {
		try {
			instance->setMinScale(minScale);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_GeneralizedHoughGuil_getMinScale_const(const cv::GeneralizedHoughGuil* instance) {
		try {
			double ret = instance->getMinScale();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_GeneralizedHoughGuil_setMaxScale_double(cv::GeneralizedHoughGuil* instance, double maxScale) {
		try {
			instance->setMaxScale(maxScale);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_GeneralizedHoughGuil_getMaxScale_const(const cv::GeneralizedHoughGuil* instance) {
		try {
			double ret = instance->getMaxScale();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_GeneralizedHoughGuil_setScaleStep_double(cv::GeneralizedHoughGuil* instance, double scaleStep) {
		try {
			instance->setScaleStep(scaleStep);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_GeneralizedHoughGuil_getScaleStep_const(const cv::GeneralizedHoughGuil* instance) {
		try {
			double ret = instance->getScaleStep();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_GeneralizedHoughGuil_setScaleThresh_int(cv::GeneralizedHoughGuil* instance, int scaleThresh) {
		try {
			instance->setScaleThresh(scaleThresh);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_GeneralizedHoughGuil_getScaleThresh_const(const cv::GeneralizedHoughGuil* instance) {
		try {
			int ret = instance->getScaleThresh();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_GeneralizedHoughGuil_setPosThresh_int(cv::GeneralizedHoughGuil* instance, int posThresh) {
		try {
			instance->setPosThresh(posThresh);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_GeneralizedHoughGuil_getPosThresh_const(const cv::GeneralizedHoughGuil* instance) {
		try {
			int ret = instance->getPosThresh();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<unsigned char*> cv_LineIterator_ptr(cv::LineIterator* instance) {
		try {
			unsigned char* ret = instance->ptr;
			return Ok(ret);
		} OCVRS_CATCH(Result<unsigned char*>)
	}
	
	Result_void cv_LineIterator_setPtr_unsigned_charX(cv::LineIterator* instance, unsigned char* val) {
		try {
			instance->ptr = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<const unsigned char*> cv_LineIterator_ptr0_const(const cv::LineIterator* instance) {
		try {
			const unsigned char* ret = instance->ptr0;
			return Ok(ret);
		} OCVRS_CATCH(Result<const unsigned char*>)
	}
	
	Result<int> cv_LineIterator_step_const(const cv::LineIterator* instance) {
		try {
			int ret = instance->step;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_LineIterator_setStep_int(cv::LineIterator* instance, int val) {
		try {
			instance->step = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_LineIterator_elemSize_const(const cv::LineIterator* instance) {
		try {
			int ret = instance->elemSize;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_LineIterator_setElemSize_int(cv::LineIterator* instance, int val) {
		try {
			instance->elemSize = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_LineIterator_err_const(const cv::LineIterator* instance) {
		try {
			int ret = instance->err;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_LineIterator_setErr_int(cv::LineIterator* instance, int val) {
		try {
			instance->err = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_LineIterator_count_const(const cv::LineIterator* instance) {
		try {
			int ret = instance->count;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_LineIterator_setCount_int(cv::LineIterator* instance, int val) {
		try {
			instance->count = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_LineIterator_minusDelta_const(const cv::LineIterator* instance) {
		try {
			int ret = instance->minusDelta;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_LineIterator_setMinusDelta_int(cv::LineIterator* instance, int val) {
		try {
			instance->minusDelta = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_LineIterator_plusDelta_const(const cv::LineIterator* instance) {
		try {
			int ret = instance->plusDelta;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_LineIterator_setPlusDelta_int(cv::LineIterator* instance, int val) {
		try {
			instance->plusDelta = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_LineIterator_minusStep_const(const cv::LineIterator* instance) {
		try {
			int ret = instance->minusStep;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_LineIterator_setMinusStep_int(cv::LineIterator* instance, int val) {
		try {
			instance->minusStep = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_LineIterator_plusStep_const(const cv::LineIterator* instance) {
		try {
			int ret = instance->plusStep;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_LineIterator_setPlusStep_int(cv::LineIterator* instance, int val) {
		try {
			instance->plusStep = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_LineIterator_delete(cv::LineIterator* instance) {
		delete instance;
	}
	Result<cv::LineIterator*> cv_LineIterator_LineIterator_const_MatX_Point_Point_int_bool(const cv::Mat* img, const cv::Point* pt1, const cv::Point* pt2, int connectivity, bool leftToRight) {
		try {
			cv::LineIterator* ret = new cv::LineIterator(*img, *pt1, *pt2, connectivity, leftToRight);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::LineIterator*>)
	}
	
	Result<unsigned char*> cv_LineIterator_operatorX(cv::LineIterator* instance) {
		try {
			unsigned char* ret = instance->operator*();
			return Ok(ret);
		} OCVRS_CATCH(Result<unsigned char*>)
	}
	
	Result<cv::Point> cv_LineIterator_pos_const(const cv::LineIterator* instance) {
		try {
			cv::Point ret = instance->pos();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Point>)
	}
	
	Result_void cv_LineSegmentDetector_detect_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX(cv::LineSegmentDetector* instance, const cv::_InputArray* _image, const cv::_OutputArray* _lines, const cv::_OutputArray* width, const cv::_OutputArray* prec, const cv::_OutputArray* nfa) {
		try {
			instance->detect(*_image, *_lines, *width, *prec, *nfa);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_LineSegmentDetector_drawSegments_const__InputOutputArrayX_const__InputArrayX(cv::LineSegmentDetector* instance, const cv::_InputOutputArray* _image, const cv::_InputArray* lines) {
		try {
			instance->drawSegments(*_image, *lines);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_LineSegmentDetector_compareSegments_const_SizeX_const__InputArrayX_const__InputArrayX_const__InputOutputArrayX(cv::LineSegmentDetector* instance, const cv::Size* size, const cv::_InputArray* lines1, const cv::_InputArray* lines2, const cv::_InputOutputArray* _image) {
		try {
			int ret = instance->compareSegments(*size, *lines1, *lines2, *_image);
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	void cv_Subdiv2D_delete(cv::Subdiv2D* instance) {
		delete instance;
	}
	Result<cv::Subdiv2D*> cv_Subdiv2D_Subdiv2D() {
		try {
			cv::Subdiv2D* ret = new cv::Subdiv2D();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Subdiv2D*>)
	}
	
	Result<cv::Subdiv2D*> cv_Subdiv2D_Subdiv2D_Rect(const cv::Rect* rect) {
		try {
			cv::Subdiv2D* ret = new cv::Subdiv2D(*rect);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Subdiv2D*>)
	}
	
	Result_void cv_Subdiv2D_initDelaunay_Rect(cv::Subdiv2D* instance, const cv::Rect* rect) {
		try {
			instance->initDelaunay(*rect);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_Subdiv2D_insert_Point2f(cv::Subdiv2D* instance, const cv::Point2f* pt) {
		try {
			int ret = instance->insert(*pt);
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_Subdiv2D_insert_const_vector_Point2f_X(cv::Subdiv2D* instance, const std::vector<cv::Point2f>* ptvec) {
		try {
			instance->insert(*ptvec);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_Subdiv2D_locate_Point2f_intX_intX(cv::Subdiv2D* instance, const cv::Point2f* pt, int* edge, int* vertex) {
		try {
			int ret = instance->locate(*pt, *edge, *vertex);
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_Subdiv2D_findNearest_Point2f_Point2fX(cv::Subdiv2D* instance, const cv::Point2f* pt, cv::Point2f* nearestPt) {
		try {
			int ret = instance->findNearest(*pt, nearestPt);
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_Subdiv2D_getEdgeList_const_vector_Vec4f_X(const cv::Subdiv2D* instance, std::vector<cv::Vec4f>* edgeList) {
		try {
			instance->getEdgeList(*edgeList);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_Subdiv2D_getLeadingEdgeList_const_vector_int_X(const cv::Subdiv2D* instance, std::vector<int>* leadingEdgeList) {
		try {
			instance->getLeadingEdgeList(*leadingEdgeList);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_Subdiv2D_getTriangleList_const_vector_Vec6f_X(const cv::Subdiv2D* instance, std::vector<cv::Vec6f>* triangleList) {
		try {
			instance->getTriangleList(*triangleList);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_Subdiv2D_getVoronoiFacetList_const_vector_int_X_vector_vector_Point2f__X_vector_Point2f_X(cv::Subdiv2D* instance, const std::vector<int>* idx, std::vector<std::vector<cv::Point2f>>* facetList, std::vector<cv::Point2f>* facetCenters) {
		try {
			instance->getVoronoiFacetList(*idx, *facetList, *facetCenters);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Point2f> cv_Subdiv2D_getVertex_const_int_intX(const cv::Subdiv2D* instance, int vertex, int* firstEdge) {
		try {
			cv::Point2f ret = instance->getVertex(vertex, firstEdge);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Point2f>)
	}
	
	Result<int> cv_Subdiv2D_getEdge_const_int_int(const cv::Subdiv2D* instance, int edge, int nextEdgeType) {
		try {
			int ret = instance->getEdge(edge, nextEdgeType);
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_Subdiv2D_nextEdge_const_int(const cv::Subdiv2D* instance, int edge) {
		try {
			int ret = instance->nextEdge(edge);
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_Subdiv2D_rotateEdge_const_int_int(const cv::Subdiv2D* instance, int edge, int rotate) {
		try {
			int ret = instance->rotateEdge(edge, rotate);
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_Subdiv2D_symEdge_const_int(const cv::Subdiv2D* instance, int edge) {
		try {
			int ret = instance->symEdge(edge);
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_Subdiv2D_edgeOrg_const_int_Point2fX(const cv::Subdiv2D* instance, int edge, cv::Point2f* orgpt) {
		try {
			int ret = instance->edgeOrg(edge, orgpt);
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_Subdiv2D_edgeDst_const_int_Point2fX(const cv::Subdiv2D* instance, int edge, cv::Point2f* dstpt) {
		try {
			int ret = instance->edgeDst(edge, dstpt);
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
}
