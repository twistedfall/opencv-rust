#include "ocvrs_common.hpp"
#include <opencv2/imgproc.hpp>
#include "imgproc_types.hpp"

extern "C" {
	Result_void cv_Canny_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_double_bool(const cv::_InputArray* dx, const cv::_InputArray* dy, const cv::_OutputArray* edges, double threshold1, double threshold2, bool L2gradient) {
		try {
			cv::Canny(*dx, *dy, *edges, threshold1, threshold2, L2gradient);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_Canny_const__InputArrayR_const__OutputArrayR_double_double_int_bool(const cv::_InputArray* image, const cv::_OutputArray* edges, double threshold1, double threshold2, int apertureSize, bool L2gradient) {
		try {
			cv::Canny(*image, *edges, threshold1, threshold2, apertureSize, L2gradient);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_EMD_const__InputArrayR_const__InputArrayR_int_const__InputArrayR_floatX_const__OutputArrayR(const cv::_InputArray* signature1, const cv::_InputArray* signature2, int distType, const cv::_InputArray* cost, float* lowerBound, const cv::_OutputArray* flow) {
		try {
			float ret = cv::EMD(*signature1, *signature2, distType, *cost, lowerBound, *flow);
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_GaussianBlur_const__InputArrayR_const__OutputArrayR_Size_double_double_int(const cv::_InputArray* src, const cv::_OutputArray* dst, cv::Size* ksize, double sigmaX, double sigmaY, int borderType) {
		try {
			cv::GaussianBlur(*src, *dst, *ksize, sigmaX, sigmaY, borderType);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_HoughCircles_const__InputArrayR_const__OutputArrayR_int_double_double_double_double_int_int(const cv::_InputArray* image, const cv::_OutputArray* circles, int method, double dp, double minDist, double param1, double param2, int minRadius, int maxRadius) {
		try {
			cv::HoughCircles(*image, *circles, method, dp, minDist, param1, param2, minRadius, maxRadius);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_HoughLinesP_const__InputArrayR_const__OutputArrayR_double_double_int_double_double(const cv::_InputArray* image, const cv::_OutputArray* lines, double rho, double theta, int threshold, double minLineLength, double maxLineGap) {
		try {
			cv::HoughLinesP(*image, *lines, rho, theta, threshold, minLineLength, maxLineGap);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_HoughLinesPointSet_const__InputArrayR_const__OutputArrayR_int_int_double_double_double_double_double_double(const cv::_InputArray* _point, const cv::_OutputArray* _lines, int lines_max, int threshold, double min_rho, double max_rho, double rho_step, double min_theta, double max_theta, double theta_step) {
		try {
			cv::HoughLinesPointSet(*_point, *_lines, lines_max, threshold, min_rho, max_rho, rho_step, min_theta, max_theta, theta_step);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_HoughLines_const__InputArrayR_const__OutputArrayR_double_double_int_double_double_double_double(const cv::_InputArray* image, const cv::_OutputArray* lines, double rho, double theta, int threshold, double srn, double stn, double min_theta, double max_theta) {
		try {
			cv::HoughLines(*image, *lines, rho, theta, threshold, srn, stn, min_theta, max_theta);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_HuMoments_const_MomentsR_const__OutputArrayR(const cv::Moments* m, const cv::_OutputArray* hu) {
		try {
			cv::HuMoments(*m, *hu);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_HuMoments_const_MomentsR_double_X__7_(const cv::Moments* moments, double(*hu)[7]) {
		try {
			cv::HuMoments(*moments, *hu);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_Laplacian_const__InputArrayR_const__OutputArrayR_int_int_double_double_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int ddepth, int ksize, double scale, double delta, int borderType) {
		try {
			cv::Laplacian(*src, *dst, ddepth, ksize, scale, delta, borderType);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_Scharr_const__InputArrayR_const__OutputArrayR_int_int_int_double_double_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int ddepth, int dx, int dy, double scale, double delta, int borderType) {
		try {
			cv::Scharr(*src, *dst, ddepth, dx, dy, scale, delta, borderType);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_Sobel_const__InputArrayR_const__OutputArrayR_int_int_int_int_double_double_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int ddepth, int dx, int dy, int ksize, double scale, double delta, int borderType) {
		try {
			cv::Sobel(*src, *dst, ddepth, dx, dy, ksize, scale, delta, borderType);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_accumulateProduct_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_InputOutputArray* dst, const cv::_InputArray* mask) {
		try {
			cv::accumulateProduct(*src1, *src2, *dst, *mask);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_accumulateSquare_const__InputArrayR_const__InputOutputArrayR_const__InputArrayR(const cv::_InputArray* src, const cv::_InputOutputArray* dst, const cv::_InputArray* mask) {
		try {
			cv::accumulateSquare(*src, *dst, *mask);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_accumulateWeighted_const__InputArrayR_const__InputOutputArrayR_double_const__InputArrayR(const cv::_InputArray* src, const cv::_InputOutputArray* dst, double alpha, const cv::_InputArray* mask) {
		try {
			cv::accumulateWeighted(*src, *dst, alpha, *mask);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_accumulate_const__InputArrayR_const__InputOutputArrayR_const__InputArrayR(const cv::_InputArray* src, const cv::_InputOutputArray* dst, const cv::_InputArray* mask) {
		try {
			cv::accumulate(*src, *dst, *mask);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_adaptiveThreshold_const__InputArrayR_const__OutputArrayR_double_int_int_int_double(const cv::_InputArray* src, const cv::_OutputArray* dst, double maxValue, int adaptiveMethod, int thresholdType, int blockSize, double C) {
		try {
			cv::adaptiveThreshold(*src, *dst, maxValue, adaptiveMethod, thresholdType, blockSize, C);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_applyColorMap_const__InputArrayR_const__OutputArrayR_const__InputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* userColor) {
		try {
			cv::applyColorMap(*src, *dst, *userColor);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_applyColorMap_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int colormap) {
		try {
			cv::applyColorMap(*src, *dst, colormap);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_approxPolyDP_const__InputArrayR_const__OutputArrayR_double_bool(const cv::_InputArray* curve, const cv::_OutputArray* approxCurve, double epsilon, bool closed) {
		try {
			cv::approxPolyDP(*curve, *approxCurve, epsilon, closed);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_arcLength_const__InputArrayR_bool(const cv::_InputArray* curve, bool closed) {
		try {
			double ret = cv::arcLength(*curve, closed);
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_arrowedLine_const__InputOutputArrayR_Point_Point_const_ScalarR_int_int_int_double(const cv::_InputOutputArray* img, cv::Point* pt1, cv::Point* pt2, const cv::Scalar* color, int thickness, int line_type, int shift, double tipLength) {
		try {
			cv::arrowedLine(*img, *pt1, *pt2, *color, thickness, line_type, shift, tipLength);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_bilateralFilter_const__InputArrayR_const__OutputArrayR_int_double_double_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int d, double sigmaColor, double sigmaSpace, int borderType) {
		try {
			cv::bilateralFilter(*src, *dst, d, sigmaColor, sigmaSpace, borderType);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_blendLinear_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_InputArray* weights1, const cv::_InputArray* weights2, const cv::_OutputArray* dst) {
		try {
			cv::blendLinear(*src1, *src2, *weights1, *weights2, *dst);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_blur_const__InputArrayR_const__OutputArrayR_Size_Point_int(const cv::_InputArray* src, const cv::_OutputArray* dst, cv::Size* ksize, cv::Point* anchor, int borderType) {
		try {
			cv::blur(*src, *dst, *ksize, *anchor, borderType);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Rect> cv_boundingRect_const__InputArrayR(const cv::_InputArray* array) {
		try {
			cv::Rect ret = cv::boundingRect(*array);
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	Result_void cv_boxFilter_const__InputArrayR_const__OutputArrayR_int_Size_Point_bool_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int ddepth, cv::Size* ksize, cv::Point* anchor, bool normalize, int borderType) {
		try {
			cv::boxFilter(*src, *dst, ddepth, *ksize, *anchor, normalize, borderType);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_boxPoints_RotatedRect_const__OutputArrayR(cv::RotatedRect* box, const cv::_OutputArray* points) {
		try {
			cv::boxPoints(*box, *points);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_buildPyramid_const__InputArrayR_const__OutputArrayR_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int maxlevel, int borderType) {
		try {
			cv::buildPyramid(*src, *dst, maxlevel, borderType);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_calcBackProject_const__InputArrayR_const_vector_int_R_const__InputArrayR_const__OutputArrayR_const_vector_float_R_double(const cv::_InputArray* images, const std::vector<int>* channels, const cv::_InputArray* hist, const cv::_OutputArray* dst, const std::vector<float>* ranges, double scale) {
		try {
			cv::calcBackProject(*images, *channels, *hist, *dst, *ranges, scale);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_calcHist_const__InputArrayR_const_vector_int_R_const__InputArrayR_const__OutputArrayR_const_vector_int_R_const_vector_float_R_bool(const cv::_InputArray* images, const std::vector<int>* channels, const cv::_InputArray* mask, const cv::_OutputArray* hist, const std::vector<int>* histSize, const std::vector<float>* ranges, bool accumulate) {
		try {
			cv::calcHist(*images, *channels, *mask, *hist, *histSize, *ranges, accumulate);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_circle_const__InputOutputArrayR_Point_int_const_ScalarR_int_int_int(const cv::_InputOutputArray* img, cv::Point* center, int radius, const cv::Scalar* color, int thickness, int lineType, int shift) {
		try {
			cv::circle(*img, *center, radius, *color, thickness, lineType, shift);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_clipLine_Rect_PointR_PointR(cv::Rect* imgRect, cv::Point* pt1, cv::Point* pt2) {
		try {
			bool ret = cv::clipLine(*imgRect, *pt1, *pt2);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_clipLine_Size2l_Point2lR_Point2lR(cv::Size2l* imgSize, cv::Point2l* pt1, cv::Point2l* pt2) {
		try {
			bool ret = cv::clipLine(*imgSize, *pt1, *pt2);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_clipLine_Size_PointR_PointR(cv::Size* imgSize, cv::Point* pt1, cv::Point* pt2) {
		try {
			bool ret = cv::clipLine(*imgSize, *pt1, *pt2);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<double> cv_compareHist_const_SparseMatR_const_SparseMatR_int(const cv::SparseMat* H1, const cv::SparseMat* H2, int method) {
		try {
			double ret = cv::compareHist(*H1, *H2, method);
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result<double> cv_compareHist_const__InputArrayR_const__InputArrayR_int(const cv::_InputArray* H1, const cv::_InputArray* H2, int method) {
		try {
			double ret = cv::compareHist(*H1, *H2, method);
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result<int> cv_connectedComponentsWithStats_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_int(const cv::_InputArray* image, const cv::_OutputArray* labels, const cv::_OutputArray* stats, const cv::_OutputArray* centroids, int connectivity, int ltype) {
		try {
			int ret = cv::connectedComponentsWithStats(*image, *labels, *stats, *centroids, connectivity, ltype);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_connectedComponentsWithStats_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_int_int(const cv::_InputArray* image, const cv::_OutputArray* labels, const cv::_OutputArray* stats, const cv::_OutputArray* centroids, int connectivity, int ltype, int ccltype) {
		try {
			int ret = cv::connectedComponentsWithStats(*image, *labels, *stats, *centroids, connectivity, ltype, ccltype);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_connectedComponents_const__InputArrayR_const__OutputArrayR_int_int(const cv::_InputArray* image, const cv::_OutputArray* labels, int connectivity, int ltype) {
		try {
			int ret = cv::connectedComponents(*image, *labels, connectivity, ltype);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_connectedComponents_const__InputArrayR_const__OutputArrayR_int_int_int(const cv::_InputArray* image, const cv::_OutputArray* labels, int connectivity, int ltype, int ccltype) {
		try {
			int ret = cv::connectedComponents(*image, *labels, connectivity, ltype, ccltype);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<double> cv_contourArea_const__InputArrayR_bool(const cv::_InputArray* contour, bool oriented) {
		try {
			double ret = cv::contourArea(*contour, oriented);
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_convertMaps_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int_bool(const cv::_InputArray* map1, const cv::_InputArray* map2, const cv::_OutputArray* dstmap1, const cv::_OutputArray* dstmap2, int dstmap1type, bool nninterpolation) {
		try {
			cv::convertMaps(*map1, *map2, *dstmap1, *dstmap2, dstmap1type, nninterpolation);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_convexHull_const__InputArrayR_const__OutputArrayR_bool_bool(const cv::_InputArray* points, const cv::_OutputArray* hull, bool clockwise, bool returnPoints) {
		try {
			cv::convexHull(*points, *hull, clockwise, returnPoints);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_convexityDefects_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* contour, const cv::_InputArray* convexhull, const cv::_OutputArray* convexityDefects) {
		try {
			cv::convexityDefects(*contour, *convexhull, *convexityDefects);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cornerEigenValsAndVecs_const__InputArrayR_const__OutputArrayR_int_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int blockSize, int ksize, int borderType) {
		try {
			cv::cornerEigenValsAndVecs(*src, *dst, blockSize, ksize, borderType);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cornerHarris_const__InputArrayR_const__OutputArrayR_int_int_double_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int blockSize, int ksize, double k, int borderType) {
		try {
			cv::cornerHarris(*src, *dst, blockSize, ksize, k, borderType);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cornerMinEigenVal_const__InputArrayR_const__OutputArrayR_int_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int blockSize, int ksize, int borderType) {
		try {
			cv::cornerMinEigenVal(*src, *dst, blockSize, ksize, borderType);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cornerSubPix_const__InputArrayR_const__InputOutputArrayR_Size_Size_TermCriteria(const cv::_InputArray* image, const cv::_InputOutputArray* corners, cv::Size* winSize, cv::Size* zeroZone, cv::TermCriteria* criteria) {
		try {
			cv::cornerSubPix(*image, *corners, *winSize, *zeroZone, *criteria);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::CLAHE>*> cv_createCLAHE_double_Size(double clipLimit, cv::Size* tileGridSize) {
		try {
			cv::Ptr<cv::CLAHE> ret = cv::createCLAHE(clipLimit, *tileGridSize);
			return Ok(new cv::Ptr<cv::CLAHE>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::CLAHE>*>))
	}
	
	Result<cv::Ptr<cv::GeneralizedHoughBallard>*> cv_createGeneralizedHoughBallard() {
		try {
			cv::Ptr<cv::GeneralizedHoughBallard> ret = cv::createGeneralizedHoughBallard();
			return Ok(new cv::Ptr<cv::GeneralizedHoughBallard>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::GeneralizedHoughBallard>*>))
	}
	
	Result<cv::Ptr<cv::GeneralizedHoughGuil>*> cv_createGeneralizedHoughGuil() {
		try {
			cv::Ptr<cv::GeneralizedHoughGuil> ret = cv::createGeneralizedHoughGuil();
			return Ok(new cv::Ptr<cv::GeneralizedHoughGuil>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::GeneralizedHoughGuil>*>))
	}
	
	Result_void cv_createHanningWindow_const__OutputArrayR_Size_int(const cv::_OutputArray* dst, cv::Size* winSize, int type) {
		try {
			cv::createHanningWindow(*dst, *winSize, type);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::LineSegmentDetector>*> cv_createLineSegmentDetector_int_double_double_double_double_double_double_int(int _refine, double _scale, double _sigma_scale, double _quant, double _ang_th, double _log_eps, double _density_th, int _n_bins) {
		try {
			cv::Ptr<cv::LineSegmentDetector> ret = cv::createLineSegmentDetector(_refine, _scale, _sigma_scale, _quant, _ang_th, _log_eps, _density_th, _n_bins);
			return Ok(new cv::Ptr<cv::LineSegmentDetector>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::LineSegmentDetector>*>))
	}
	
	Result_void cv_cvtColorTwoPlane_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, int code) {
		try {
			cv::cvtColorTwoPlane(*src1, *src2, *dst, code);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cvtColor_const__InputArrayR_const__OutputArrayR_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int code, int dstCn) {
		try {
			cv::cvtColor(*src, *dst, code, dstCn);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_demosaicing_const__InputArrayR_const__OutputArrayR_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int code, int dstCn) {
		try {
			cv::demosaicing(*src, *dst, code, dstCn);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_dilate_const__InputArrayR_const__OutputArrayR_const__InputArrayR_Point_int_int_const_ScalarR(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* kernel, cv::Point* anchor, int iterations, int borderType, const cv::Scalar* borderValue) {
		try {
			cv::dilate(*src, *dst, *kernel, *anchor, iterations, borderType, *borderValue);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_distanceTransform_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_OutputArray* labels, int distanceType, int maskSize, int labelType) {
		try {
			cv::distanceTransform(*src, *dst, *labels, distanceType, maskSize, labelType);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_distanceTransform_const__InputArrayR_const__OutputArrayR_int_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int distanceType, int maskSize, int dstType) {
		try {
			cv::distanceTransform(*src, *dst, distanceType, maskSize, dstType);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_drawContours_const__InputOutputArrayR_const__InputArrayR_int_const_ScalarR_int_int_const__InputArrayR_int_Point(const cv::_InputOutputArray* image, const cv::_InputArray* contours, int contourIdx, const cv::Scalar* color, int thickness, int lineType, const cv::_InputArray* hierarchy, int maxLevel, cv::Point* offset) {
		try {
			cv::drawContours(*image, *contours, contourIdx, *color, thickness, lineType, *hierarchy, maxLevel, *offset);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_drawMarker_const__InputOutputArrayR_Point_const_ScalarR_int_int_int_int(const cv::_InputOutputArray* img, cv::Point* position, const cv::Scalar* color, int markerType, int markerSize, int thickness, int line_type) {
		try {
			cv::drawMarker(*img, *position, *color, markerType, markerSize, thickness, line_type);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_ellipse2Poly_Point2d_Size2d_int_int_int_int_vector_Point2d_R(cv::Point2d* center, cv::Size2d* axes, int angle, int arcStart, int arcEnd, int delta, std::vector<cv::Point2d>* pts) {
		try {
			cv::ellipse2Poly(*center, *axes, angle, arcStart, arcEnd, delta, *pts);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_ellipse2Poly_Point_Size_int_int_int_int_vector_Point_R(cv::Point* center, cv::Size* axes, int angle, int arcStart, int arcEnd, int delta, std::vector<cv::Point>* pts) {
		try {
			cv::ellipse2Poly(*center, *axes, angle, arcStart, arcEnd, delta, *pts);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_ellipse_const__InputOutputArrayR_Point_Size_double_double_double_const_ScalarR_int_int_int(const cv::_InputOutputArray* img, cv::Point* center, cv::Size* axes, double angle, double startAngle, double endAngle, const cv::Scalar* color, int thickness, int lineType, int shift) {
		try {
			cv::ellipse(*img, *center, *axes, angle, startAngle, endAngle, *color, thickness, lineType, shift);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_ellipse_const__InputOutputArrayR_const_RotatedRectR_const_ScalarR_int_int(const cv::_InputOutputArray* img, const cv::RotatedRect* box, const cv::Scalar* color, int thickness, int lineType) {
		try {
			cv::ellipse(*img, *box, *color, thickness, lineType);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_equalizeHist_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst) {
		try {
			cv::equalizeHist(*src, *dst);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_erode_const__InputArrayR_const__OutputArrayR_const__InputArrayR_Point_int_int_const_ScalarR(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* kernel, cv::Point* anchor, int iterations, int borderType, const cv::Scalar* borderValue) {
		try {
			cv::erode(*src, *dst, *kernel, *anchor, iterations, borderType, *borderValue);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_fillConvexPoly_const__InputOutputArrayR_const__InputArrayR_const_ScalarR_int_int(const cv::_InputOutputArray* img, const cv::_InputArray* points, const cv::Scalar* color, int lineType, int shift) {
		try {
			cv::fillConvexPoly(*img, *points, *color, lineType, shift);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_fillPoly_const__InputOutputArrayR_const__InputArrayR_const_ScalarR_int_int_Point(const cv::_InputOutputArray* img, const cv::_InputArray* pts, const cv::Scalar* color, int lineType, int shift, cv::Point* offset) {
		try {
			cv::fillPoly(*img, *pts, *color, lineType, shift, *offset);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_filter2D_const__InputArrayR_const__OutputArrayR_int_const__InputArrayR_Point_double_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int ddepth, const cv::_InputArray* kernel, cv::Point* anchor, double delta, int borderType) {
		try {
			cv::filter2D(*src, *dst, ddepth, *kernel, *anchor, delta, borderType);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_findContours_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int_int_Point(const cv::_InputArray* image, const cv::_OutputArray* contours, const cv::_OutputArray* hierarchy, int mode, int method, cv::Point* offset) {
		try {
			cv::findContours(*image, *contours, *hierarchy, mode, method, *offset);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_findContours_const__InputArrayR_const__OutputArrayR_int_int_Point(const cv::_InputArray* image, const cv::_OutputArray* contours, int mode, int method, cv::Point* offset) {
		try {
			cv::findContours(*image, *contours, mode, method, *offset);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::RotatedRect*> cv_fitEllipseAMS_const__InputArrayR(const cv::_InputArray* points) {
		try {
			cv::RotatedRect ret = cv::fitEllipseAMS(*points);
			return Ok(new cv::RotatedRect(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::RotatedRect*>))
	}
	
	Result<cv::RotatedRect*> cv_fitEllipseDirect_const__InputArrayR(const cv::_InputArray* points) {
		try {
			cv::RotatedRect ret = cv::fitEllipseDirect(*points);
			return Ok(new cv::RotatedRect(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::RotatedRect*>))
	}
	
	Result<cv::RotatedRect*> cv_fitEllipse_const__InputArrayR(const cv::_InputArray* points) {
		try {
			cv::RotatedRect ret = cv::fitEllipse(*points);
			return Ok(new cv::RotatedRect(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::RotatedRect*>))
	}
	
	Result_void cv_fitLine_const__InputArrayR_const__OutputArrayR_int_double_double_double(const cv::_InputArray* points, const cv::_OutputArray* line, int distType, double param, double reps, double aeps) {
		try {
			cv::fitLine(*points, *line, distType, param, reps, aeps);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_floodFill_const__InputOutputArrayR_Point_Scalar_RectX_Scalar_Scalar_int(const cv::_InputOutputArray* image, cv::Point* seedPoint, cv::Scalar* newVal, cv::Rect* rect, cv::Scalar* loDiff, cv::Scalar* upDiff, int flags) {
		try {
			int ret = cv::floodFill(*image, *seedPoint, *newVal, rect, *loDiff, *upDiff, flags);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_floodFill_const__InputOutputArrayR_const__InputOutputArrayR_Point_Scalar_RectX_Scalar_Scalar_int(const cv::_InputOutputArray* image, const cv::_InputOutputArray* mask, cv::Point* seedPoint, cv::Scalar* newVal, cv::Rect* rect, cv::Scalar* loDiff, cv::Scalar* upDiff, int flags) {
		try {
			int ret = cv::floodFill(*image, *mask, *seedPoint, *newVal, rect, *loDiff, *upDiff, flags);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<cv::Mat*> cv_getAffineTransform_const_Point2fX_const_Point2fX(const cv::Point2f* src, const cv::Point2f* dst) {
		try {
			cv::Mat ret = cv::getAffineTransform(src, dst);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv_getAffineTransform_const__InputArrayR_const__InputArrayR(const cv::_InputArray* src, const cv::_InputArray* dst) {
		try {
			cv::Mat ret = cv::getAffineTransform(*src, *dst);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_getDerivKernels_const__OutputArrayR_const__OutputArrayR_int_int_int_bool_int(const cv::_OutputArray* kx, const cv::_OutputArray* ky, int dx, int dy, int ksize, bool normalize, int ktype) {
		try {
			cv::getDerivKernels(*kx, *ky, dx, dy, ksize, normalize, ktype);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_getFontScaleFromHeight_const_int_const_int_const_int(const int fontFace, const int pixelHeight, const int thickness) {
		try {
			double ret = cv::getFontScaleFromHeight(fontFace, pixelHeight, thickness);
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result<cv::Mat*> cv_getGaborKernel_Size_double_double_double_double_double_int(cv::Size* ksize, double sigma, double theta, double lambd, double gamma, double psi, int ktype) {
		try {
			cv::Mat ret = cv::getGaborKernel(*ksize, sigma, theta, lambd, gamma, psi, ktype);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv_getGaussianKernel_int_double_int(int ksize, double sigma, int ktype) {
		try {
			cv::Mat ret = cv::getGaussianKernel(ksize, sigma, ktype);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv_getPerspectiveTransform_const_Point2fX_const_Point2fX_int(const cv::Point2f* src, const cv::Point2f* dst, int solveMethod) {
		try {
			cv::Mat ret = cv::getPerspectiveTransform(src, dst, solveMethod);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv_getPerspectiveTransform_const__InputArrayR_const__InputArrayR_int(const cv::_InputArray* src, const cv::_InputArray* dst, int solveMethod) {
		try {
			cv::Mat ret = cv::getPerspectiveTransform(*src, *dst, solveMethod);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_getRectSubPix_const__InputArrayR_Size_Point2f_const__OutputArrayR_int(const cv::_InputArray* image, cv::Size* patchSize, cv::Point2f* center, const cv::_OutputArray* patch, int patchType) {
		try {
			cv::getRectSubPix(*image, *patchSize, *center, *patch, patchType);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_getRotationMatrix2D_Point2f_double_double(cv::Point2f* center, double angle, double scale) {
		try {
			cv::Mat ret = cv::getRotationMatrix2D(*center, angle, scale);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	#if !defined(OCVRS_TARGET_OS_WINDOWS)
	Result<cv::Matx23d> cv_getRotationMatrix2D__Point2f_double_double(cv::Point2f* center, double angle, double scale) {
		try {
			cv::Matx23d ret = cv::getRotationMatrix2D_(*center, angle, scale);
			return Ok<cv::Matx23d>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Matx23d>))
	}
	#endif
	
	Result<cv::Mat*> cv_getStructuringElement_int_Size_Point(int shape, cv::Size* ksize, cv::Point* anchor) {
		try {
			cv::Mat ret = cv::getStructuringElement(shape, *ksize, *anchor);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Size> cv_getTextSize_const_StringR_int_double_int_intX(const char* text, int fontFace, double fontScale, int thickness, int* baseLine) {
		try {
			cv::Size ret = cv::getTextSize(std::string(text), fontFace, fontScale, thickness, baseLine);
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	Result_void cv_goodFeaturesToTrack_const__InputArrayR_const__OutputArrayR_int_double_double_const__InputArrayR_const__OutputArrayR_int_int_bool_double(const cv::_InputArray* image, const cv::_OutputArray* corners, int maxCorners, double qualityLevel, double minDistance, const cv::_InputArray* mask, const cv::_OutputArray* cornersQuality, int blockSize, int gradientSize, bool useHarrisDetector, double k) {
		try {
			cv::goodFeaturesToTrack(*image, *corners, maxCorners, qualityLevel, minDistance, *mask, *cornersQuality, blockSize, gradientSize, useHarrisDetector, k);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_goodFeaturesToTrack_const__InputArrayR_const__OutputArrayR_int_double_double_const__InputArrayR_int_bool_double(const cv::_InputArray* image, const cv::_OutputArray* corners, int maxCorners, double qualityLevel, double minDistance, const cv::_InputArray* mask, int blockSize, bool useHarrisDetector, double k) {
		try {
			cv::goodFeaturesToTrack(*image, *corners, maxCorners, qualityLevel, minDistance, *mask, blockSize, useHarrisDetector, k);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_goodFeaturesToTrack_const__InputArrayR_const__OutputArrayR_int_double_double_const__InputArrayR_int_int_bool_double(const cv::_InputArray* image, const cv::_OutputArray* corners, int maxCorners, double qualityLevel, double minDistance, const cv::_InputArray* mask, int blockSize, int gradientSize, bool useHarrisDetector, double k) {
		try {
			cv::goodFeaturesToTrack(*image, *corners, maxCorners, qualityLevel, minDistance, *mask, blockSize, gradientSize, useHarrisDetector, k);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_grabCut_const__InputArrayR_const__InputOutputArrayR_Rect_const__InputOutputArrayR_const__InputOutputArrayR_int_int(const cv::_InputArray* img, const cv::_InputOutputArray* mask, cv::Rect* rect, const cv::_InputOutputArray* bgdModel, const cv::_InputOutputArray* fgdModel, int iterCount, int mode) {
		try {
			cv::grabCut(*img, *mask, *rect, *bgdModel, *fgdModel, iterCount, mode);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_integral_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int_int(const cv::_InputArray* src, const cv::_OutputArray* sum, const cv::_OutputArray* sqsum, const cv::_OutputArray* tilted, int sdepth, int sqdepth) {
		try {
			cv::integral(*src, *sum, *sqsum, *tilted, sdepth, sqdepth);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_integral_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int_int(const cv::_InputArray* src, const cv::_OutputArray* sum, const cv::_OutputArray* sqsum, int sdepth, int sqdepth) {
		try {
			cv::integral(*src, *sum, *sqsum, sdepth, sqdepth);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_integral_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* sum, int sdepth) {
		try {
			cv::integral(*src, *sum, sdepth);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_intersectConvexConvex_const__InputArrayR_const__InputArrayR_const__OutputArrayR_bool(const cv::_InputArray* _p1, const cv::_InputArray* _p2, const cv::_OutputArray* _p12, bool handleNested) {
		try {
			float ret = cv::intersectConvexConvex(*_p1, *_p2, *_p12, handleNested);
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_invertAffineTransform_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* M, const cv::_OutputArray* iM) {
		try {
			cv::invertAffineTransform(*M, *iM);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_isContourConvex_const__InputArrayR(const cv::_InputArray* contour) {
		try {
			bool ret = cv::isContourConvex(*contour);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_line_const__InputOutputArrayR_Point_Point_const_ScalarR_int_int_int(const cv::_InputOutputArray* img, cv::Point* pt1, cv::Point* pt2, const cv::Scalar* color, int thickness, int lineType, int shift) {
		try {
			cv::line(*img, *pt1, *pt2, *color, thickness, lineType, shift);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_linearPolar_const__InputArrayR_const__OutputArrayR_Point2f_double_int(const cv::_InputArray* src, const cv::_OutputArray* dst, cv::Point2f* center, double maxRadius, int flags) {
		try {
			cv::linearPolar(*src, *dst, *center, maxRadius, flags);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_logPolar_const__InputArrayR_const__OutputArrayR_Point2f_double_int(const cv::_InputArray* src, const cv::_OutputArray* dst, cv::Point2f* center, double M, int flags) {
		try {
			cv::logPolar(*src, *dst, *center, M, flags);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_matchShapes_const__InputArrayR_const__InputArrayR_int_double(const cv::_InputArray* contour1, const cv::_InputArray* contour2, int method, double parameter) {
		try {
			double ret = cv::matchShapes(*contour1, *contour2, method, parameter);
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_matchTemplate_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_const__InputArrayR(const cv::_InputArray* image, const cv::_InputArray* templ, const cv::_OutputArray* result, int method, const cv::_InputArray* mask) {
		try {
			cv::matchTemplate(*image, *templ, *result, method, *mask);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_medianBlur_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int ksize) {
		try {
			cv::medianBlur(*src, *dst, ksize);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::RotatedRect*> cv_minAreaRect_const__InputArrayR(const cv::_InputArray* points) {
		try {
			cv::RotatedRect ret = cv::minAreaRect(*points);
			return Ok(new cv::RotatedRect(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::RotatedRect*>))
	}
	
	Result_void cv_minEnclosingCircle_const__InputArrayR_Point2fR_floatR(const cv::_InputArray* points, cv::Point2f* center, float* radius) {
		try {
			cv::minEnclosingCircle(*points, *center, *radius);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_minEnclosingTriangle_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* points, const cv::_OutputArray* triangle) {
		try {
			double ret = cv::minEnclosingTriangle(*points, *triangle);
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result<cv::Moments> cv_moments_const__InputArrayR_bool(const cv::_InputArray* array, bool binaryImage) {
		try {
			cv::Moments ret = cv::moments(*array, binaryImage);
			return Ok<cv::Moments>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Moments>))
	}
	
	Result<cv::Scalar> cv_morphologyDefaultBorderValue() {
		try {
			cv::Scalar ret = cv::morphologyDefaultBorderValue();
			return Ok<cv::Scalar>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Scalar>))
	}
	
	Result_void cv_morphologyEx_const__InputArrayR_const__OutputArrayR_int_const__InputArrayR_Point_int_int_const_ScalarR(const cv::_InputArray* src, const cv::_OutputArray* dst, int op, const cv::_InputArray* kernel, cv::Point* anchor, int iterations, int borderType, const cv::Scalar* borderValue) {
		try {
			cv::morphologyEx(*src, *dst, op, *kernel, *anchor, iterations, borderType, *borderValue);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Point2d> cv_phaseCorrelate_const__InputArrayR_const__InputArrayR_const__InputArrayR_doubleX(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_InputArray* window, double* response) {
		try {
			cv::Point2d ret = cv::phaseCorrelate(*src1, *src2, *window, response);
			return Ok<cv::Point2d>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point2d>))
	}
	
	Result<double> cv_pointPolygonTest_const__InputArrayR_Point2f_bool(const cv::_InputArray* contour, cv::Point2f* pt, bool measureDist) {
		try {
			double ret = cv::pointPolygonTest(*contour, *pt, measureDist);
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_polylines_const__InputOutputArrayR_const__InputArrayR_bool_const_ScalarR_int_int_int(const cv::_InputOutputArray* img, const cv::_InputArray* pts, bool isClosed, const cv::Scalar* color, int thickness, int lineType, int shift) {
		try {
			cv::polylines(*img, *pts, isClosed, *color, thickness, lineType, shift);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_preCornerDetect_const__InputArrayR_const__OutputArrayR_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int ksize, int borderType) {
		try {
			cv::preCornerDetect(*src, *dst, ksize, borderType);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_putText_const__InputOutputArrayR_const_StringR_Point_int_double_Scalar_int_int_bool(const cv::_InputOutputArray* img, const char* text, cv::Point* org, int fontFace, double fontScale, cv::Scalar* color, int thickness, int lineType, bool bottomLeftOrigin) {
		try {
			cv::putText(*img, std::string(text), *org, fontFace, fontScale, *color, thickness, lineType, bottomLeftOrigin);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_pyrDown_const__InputArrayR_const__OutputArrayR_const_SizeR_int(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::Size* dstsize, int borderType) {
		try {
			cv::pyrDown(*src, *dst, *dstsize, borderType);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_pyrMeanShiftFiltering_const__InputArrayR_const__OutputArrayR_double_double_int_TermCriteria(const cv::_InputArray* src, const cv::_OutputArray* dst, double sp, double sr, int maxLevel, cv::TermCriteria* termcrit) {
		try {
			cv::pyrMeanShiftFiltering(*src, *dst, sp, sr, maxLevel, *termcrit);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_pyrUp_const__InputArrayR_const__OutputArrayR_const_SizeR_int(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::Size* dstsize, int borderType) {
		try {
			cv::pyrUp(*src, *dst, *dstsize, borderType);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_rectangle_const__InputOutputArrayR_Point_Point_const_ScalarR_int_int_int(const cv::_InputOutputArray* img, cv::Point* pt1, cv::Point* pt2, const cv::Scalar* color, int thickness, int lineType, int shift) {
		try {
			cv::rectangle(*img, *pt1, *pt2, *color, thickness, lineType, shift);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_rectangle_const__InputOutputArrayR_Rect_const_ScalarR_int_int_int(const cv::_InputOutputArray* img, cv::Rect* rec, const cv::Scalar* color, int thickness, int lineType, int shift) {
		try {
			cv::rectangle(*img, *rec, *color, thickness, lineType, shift);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_remap_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_int_int_const_ScalarR(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* map1, const cv::_InputArray* map2, int interpolation, int borderMode, const cv::Scalar* borderValue) {
		try {
			cv::remap(*src, *dst, *map1, *map2, interpolation, borderMode, *borderValue);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_resize_const__InputArrayR_const__OutputArrayR_Size_double_double_int(const cv::_InputArray* src, const cv::_OutputArray* dst, cv::Size* dsize, double fx, double fy, int interpolation) {
		try {
			cv::resize(*src, *dst, *dsize, fx, fy, interpolation);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_rotatedRectangleIntersection_const_RotatedRectR_const_RotatedRectR_const__OutputArrayR(const cv::RotatedRect* rect1, const cv::RotatedRect* rect2, const cv::_OutputArray* intersectingRegion) {
		try {
			int ret = cv::rotatedRectangleIntersection(*rect1, *rect2, *intersectingRegion);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_sepFilter2D_const__InputArrayR_const__OutputArrayR_int_const__InputArrayR_const__InputArrayR_Point_double_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int ddepth, const cv::_InputArray* kernelX, const cv::_InputArray* kernelY, cv::Point* anchor, double delta, int borderType) {
		try {
			cv::sepFilter2D(*src, *dst, ddepth, *kernelX, *kernelY, *anchor, delta, borderType);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_spatialGradient_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int_int(const cv::_InputArray* src, const cv::_OutputArray* dx, const cv::_OutputArray* dy, int ksize, int borderType) {
		try {
			cv::spatialGradient(*src, *dx, *dy, ksize, borderType);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_sqrBoxFilter_const__InputArrayR_const__OutputArrayR_int_Size_Point_bool_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int ddepth, cv::Size* ksize, cv::Point* anchor, bool normalize, int borderType) {
		try {
			cv::sqrBoxFilter(*src, *dst, ddepth, *ksize, *anchor, normalize, borderType);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_threshold_const__InputArrayR_const__OutputArrayR_double_double_int(const cv::_InputArray* src, const cv::_OutputArray* dst, double thresh, double maxval, int type) {
		try {
			double ret = cv::threshold(*src, *dst, thresh, maxval, type);
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_warpAffine_const__InputArrayR_const__OutputArrayR_const__InputArrayR_Size_int_int_const_ScalarR(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* M, cv::Size* dsize, int flags, int borderMode, const cv::Scalar* borderValue) {
		try {
			cv::warpAffine(*src, *dst, *M, *dsize, flags, borderMode, *borderValue);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_warpPerspective_const__InputArrayR_const__OutputArrayR_const__InputArrayR_Size_int_int_const_ScalarR(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* M, cv::Size* dsize, int flags, int borderMode, const cv::Scalar* borderValue) {
		try {
			cv::warpPerspective(*src, *dst, *M, *dsize, flags, borderMode, *borderValue);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_warpPolar_const__InputArrayR_const__OutputArrayR_Size_Point2f_double_int(const cv::_InputArray* src, const cv::_OutputArray* dst, cv::Size* dsize, cv::Point2f* center, double maxRadius, int flags) {
		try {
			cv::warpPolar(*src, *dst, *dsize, *center, maxRadius, flags);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_watershed_const__InputArrayR_const__InputOutputArrayR(const cv::_InputArray* image, const cv::_InputOutputArray* markers) {
		try {
			cv::watershed(*image, *markers);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_wrapperEMD_const__InputArrayR_const__InputArrayR_int_const__InputArrayR_Ptr_float__const__OutputArrayR(const cv::_InputArray* signature1, const cv::_InputArray* signature2, int distType, const cv::_InputArray* cost, cv::Ptr<float>* lowerBound, const cv::_OutputArray* flow) {
		try {
			float ret = cv::wrapperEMD(*signature1, *signature2, distType, *cost, *lowerBound, *flow);
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_CLAHE_apply_const__InputArrayR_const__OutputArrayR(cv::CLAHE* instance, const cv::_InputArray* src, const cv::_OutputArray* dst) {
		try {
			instance->apply(*src, *dst);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_CLAHE_setClipLimit_double(cv::CLAHE* instance, double clipLimit) {
		try {
			instance->setClipLimit(clipLimit);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_CLAHE_getClipLimit_const(const cv::CLAHE* instance) {
		try {
			double ret = instance->getClipLimit();
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_CLAHE_setTilesGridSize_Size(cv::CLAHE* instance, cv::Size* tileGridSize) {
		try {
			instance->setTilesGridSize(*tileGridSize);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Size> cv_CLAHE_getTilesGridSize_const(const cv::CLAHE* instance) {
		try {
			cv::Size ret = instance->getTilesGridSize();
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	Result_void cv_CLAHE_collectGarbage(cv::CLAHE* instance) {
		try {
			instance->collectGarbage();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_GeneralizedHough_setTemplate_const__InputArrayR_Point(cv::GeneralizedHough* instance, const cv::_InputArray* templ, cv::Point* templCenter) {
		try {
			instance->setTemplate(*templ, *templCenter);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_GeneralizedHough_setTemplate_const__InputArrayR_const__InputArrayR_const__InputArrayR_Point(cv::GeneralizedHough* instance, const cv::_InputArray* edges, const cv::_InputArray* dx, const cv::_InputArray* dy, cv::Point* templCenter) {
		try {
			instance->setTemplate(*edges, *dx, *dy, *templCenter);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_GeneralizedHough_detect_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::GeneralizedHough* instance, const cv::_InputArray* image, const cv::_OutputArray* positions, const cv::_OutputArray* votes) {
		try {
			instance->detect(*image, *positions, *votes);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_GeneralizedHough_detect_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::GeneralizedHough* instance, const cv::_InputArray* edges, const cv::_InputArray* dx, const cv::_InputArray* dy, const cv::_OutputArray* positions, const cv::_OutputArray* votes) {
		try {
			instance->detect(*edges, *dx, *dy, *positions, *votes);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_GeneralizedHough_setCannyLowThresh_int(cv::GeneralizedHough* instance, int cannyLowThresh) {
		try {
			instance->setCannyLowThresh(cannyLowThresh);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_GeneralizedHough_getCannyLowThresh_const(const cv::GeneralizedHough* instance) {
		try {
			int ret = instance->getCannyLowThresh();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_GeneralizedHough_setCannyHighThresh_int(cv::GeneralizedHough* instance, int cannyHighThresh) {
		try {
			instance->setCannyHighThresh(cannyHighThresh);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_GeneralizedHough_getCannyHighThresh_const(const cv::GeneralizedHough* instance) {
		try {
			int ret = instance->getCannyHighThresh();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_GeneralizedHough_setMinDist_double(cv::GeneralizedHough* instance, double minDist) {
		try {
			instance->setMinDist(minDist);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_GeneralizedHough_getMinDist_const(const cv::GeneralizedHough* instance) {
		try {
			double ret = instance->getMinDist();
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_GeneralizedHough_setDp_double(cv::GeneralizedHough* instance, double dp) {
		try {
			instance->setDp(dp);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_GeneralizedHough_getDp_const(const cv::GeneralizedHough* instance) {
		try {
			double ret = instance->getDp();
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_GeneralizedHough_setMaxBufferSize_int(cv::GeneralizedHough* instance, int maxBufferSize) {
		try {
			instance->setMaxBufferSize(maxBufferSize);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_GeneralizedHough_getMaxBufferSize_const(const cv::GeneralizedHough* instance) {
		try {
			int ret = instance->getMaxBufferSize();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_GeneralizedHoughBallard_setLevels_int(cv::GeneralizedHoughBallard* instance, int levels) {
		try {
			instance->setLevels(levels);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_GeneralizedHoughBallard_getLevels_const(const cv::GeneralizedHoughBallard* instance) {
		try {
			int ret = instance->getLevels();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_GeneralizedHoughBallard_setVotesThreshold_int(cv::GeneralizedHoughBallard* instance, int votesThreshold) {
		try {
			instance->setVotesThreshold(votesThreshold);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_GeneralizedHoughBallard_getVotesThreshold_const(const cv::GeneralizedHoughBallard* instance) {
		try {
			int ret = instance->getVotesThreshold();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_GeneralizedHoughGuil_setXi_double(cv::GeneralizedHoughGuil* instance, double xi) {
		try {
			instance->setXi(xi);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_GeneralizedHoughGuil_getXi_const(const cv::GeneralizedHoughGuil* instance) {
		try {
			double ret = instance->getXi();
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_GeneralizedHoughGuil_setLevels_int(cv::GeneralizedHoughGuil* instance, int levels) {
		try {
			instance->setLevels(levels);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_GeneralizedHoughGuil_getLevels_const(const cv::GeneralizedHoughGuil* instance) {
		try {
			int ret = instance->getLevels();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_GeneralizedHoughGuil_setAngleEpsilon_double(cv::GeneralizedHoughGuil* instance, double angleEpsilon) {
		try {
			instance->setAngleEpsilon(angleEpsilon);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_GeneralizedHoughGuil_getAngleEpsilon_const(const cv::GeneralizedHoughGuil* instance) {
		try {
			double ret = instance->getAngleEpsilon();
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_GeneralizedHoughGuil_setMinAngle_double(cv::GeneralizedHoughGuil* instance, double minAngle) {
		try {
			instance->setMinAngle(minAngle);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_GeneralizedHoughGuil_getMinAngle_const(const cv::GeneralizedHoughGuil* instance) {
		try {
			double ret = instance->getMinAngle();
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_GeneralizedHoughGuil_setMaxAngle_double(cv::GeneralizedHoughGuil* instance, double maxAngle) {
		try {
			instance->setMaxAngle(maxAngle);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_GeneralizedHoughGuil_getMaxAngle_const(const cv::GeneralizedHoughGuil* instance) {
		try {
			double ret = instance->getMaxAngle();
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_GeneralizedHoughGuil_setAngleStep_double(cv::GeneralizedHoughGuil* instance, double angleStep) {
		try {
			instance->setAngleStep(angleStep);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_GeneralizedHoughGuil_getAngleStep_const(const cv::GeneralizedHoughGuil* instance) {
		try {
			double ret = instance->getAngleStep();
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_GeneralizedHoughGuil_setAngleThresh_int(cv::GeneralizedHoughGuil* instance, int angleThresh) {
		try {
			instance->setAngleThresh(angleThresh);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_GeneralizedHoughGuil_getAngleThresh_const(const cv::GeneralizedHoughGuil* instance) {
		try {
			int ret = instance->getAngleThresh();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_GeneralizedHoughGuil_setMinScale_double(cv::GeneralizedHoughGuil* instance, double minScale) {
		try {
			instance->setMinScale(minScale);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_GeneralizedHoughGuil_getMinScale_const(const cv::GeneralizedHoughGuil* instance) {
		try {
			double ret = instance->getMinScale();
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_GeneralizedHoughGuil_setMaxScale_double(cv::GeneralizedHoughGuil* instance, double maxScale) {
		try {
			instance->setMaxScale(maxScale);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_GeneralizedHoughGuil_getMaxScale_const(const cv::GeneralizedHoughGuil* instance) {
		try {
			double ret = instance->getMaxScale();
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_GeneralizedHoughGuil_setScaleStep_double(cv::GeneralizedHoughGuil* instance, double scaleStep) {
		try {
			instance->setScaleStep(scaleStep);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_GeneralizedHoughGuil_getScaleStep_const(const cv::GeneralizedHoughGuil* instance) {
		try {
			double ret = instance->getScaleStep();
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_GeneralizedHoughGuil_setScaleThresh_int(cv::GeneralizedHoughGuil* instance, int scaleThresh) {
		try {
			instance->setScaleThresh(scaleThresh);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_GeneralizedHoughGuil_getScaleThresh_const(const cv::GeneralizedHoughGuil* instance) {
		try {
			int ret = instance->getScaleThresh();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_GeneralizedHoughGuil_setPosThresh_int(cv::GeneralizedHoughGuil* instance, int posThresh) {
		try {
			instance->setPosThresh(posThresh);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_GeneralizedHoughGuil_getPosThresh_const(const cv::GeneralizedHoughGuil* instance) {
		try {
			int ret = instance->getPosThresh();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<unsigned char*> cv_LineIterator_getPropPtr(cv::LineIterator* instance) {
		try {
			unsigned char* ret = instance->ptr;
			return Ok<unsigned char*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned char*>))
	}
	
	Result_void cv_LineIterator_setPropPtr_unsigned_charX(cv::LineIterator* instance, unsigned char* val) {
		try {
			instance->ptr = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<const unsigned char*> cv_LineIterator_getPropPtr0_const(const cv::LineIterator* instance) {
		try {
			const unsigned char* ret = instance->ptr0;
			return Ok<const unsigned char*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<const unsigned char*>))
	}
	
	Result<int> cv_LineIterator_getPropStep_const(const cv::LineIterator* instance) {
		try {
			int ret = instance->step;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_LineIterator_setPropStep_int(cv::LineIterator* instance, int val) {
		try {
			instance->step = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_LineIterator_getPropElemSize_const(const cv::LineIterator* instance) {
		try {
			int ret = instance->elemSize;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_LineIterator_setPropElemSize_int(cv::LineIterator* instance, int val) {
		try {
			instance->elemSize = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_LineIterator_getPropErr_const(const cv::LineIterator* instance) {
		try {
			int ret = instance->err;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_LineIterator_setPropErr_int(cv::LineIterator* instance, int val) {
		try {
			instance->err = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_LineIterator_getPropCount_const(const cv::LineIterator* instance) {
		try {
			int ret = instance->count;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_LineIterator_setPropCount_int(cv::LineIterator* instance, int val) {
		try {
			instance->count = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_LineIterator_getPropMinusDelta_const(const cv::LineIterator* instance) {
		try {
			int ret = instance->minusDelta;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_LineIterator_setPropMinusDelta_int(cv::LineIterator* instance, int val) {
		try {
			instance->minusDelta = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_LineIterator_getPropPlusDelta_const(const cv::LineIterator* instance) {
		try {
			int ret = instance->plusDelta;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_LineIterator_setPropPlusDelta_int(cv::LineIterator* instance, int val) {
		try {
			instance->plusDelta = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_LineIterator_getPropMinusStep_const(const cv::LineIterator* instance) {
		try {
			int ret = instance->minusStep;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_LineIterator_setPropMinusStep_int(cv::LineIterator* instance, int val) {
		try {
			instance->minusStep = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_LineIterator_getPropPlusStep_const(const cv::LineIterator* instance) {
		try {
			int ret = instance->plusStep;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_LineIterator_setPropPlusStep_int(cv::LineIterator* instance, int val) {
		try {
			instance->plusStep = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_LineIterator_getPropMinusShift_const(const cv::LineIterator* instance) {
		try {
			int ret = instance->minusShift;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_LineIterator_setPropMinusShift_int(cv::LineIterator* instance, int val) {
		try {
			instance->minusShift = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_LineIterator_getPropPlusShift_const(const cv::LineIterator* instance) {
		try {
			int ret = instance->plusShift;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_LineIterator_setPropPlusShift_int(cv::LineIterator* instance, int val) {
		try {
			instance->plusShift = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Point> cv_LineIterator_getPropP_const(const cv::LineIterator* instance) {
		try {
			cv::Point ret = instance->p;
			return Ok<cv::Point>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point>))
	}
	
	Result_void cv_LineIterator_setPropP_Point(cv::LineIterator* instance, cv::Point* val) {
		try {
			instance->p = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_LineIterator_getPropPtmode_const(const cv::LineIterator* instance) {
		try {
			bool ret = instance->ptmode;
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_LineIterator_setPropPtmode_bool(cv::LineIterator* instance, bool val) {
		try {
			instance->ptmode = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_LineIterator_delete(cv::LineIterator* instance) {
		delete instance;
	}
	Result<cv::LineIterator*> cv_LineIterator_LineIterator_const_MatR_Point_Point_int_bool(const cv::Mat* img, cv::Point* pt1, cv::Point* pt2, int connectivity, bool leftToRight) {
		try {
			cv::LineIterator* ret = new cv::LineIterator(*img, *pt1, *pt2, connectivity, leftToRight);
			return Ok<cv::LineIterator*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::LineIterator*>))
	}
	
	Result<cv::LineIterator*> cv_LineIterator_LineIterator_Point_Point_int_bool(cv::Point* pt1, cv::Point* pt2, int connectivity, bool leftToRight) {
		try {
			cv::LineIterator* ret = new cv::LineIterator(*pt1, *pt2, connectivity, leftToRight);
			return Ok<cv::LineIterator*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::LineIterator*>))
	}
	
	Result<cv::LineIterator*> cv_LineIterator_LineIterator_Size_Point_Point_int_bool(cv::Size* boundingAreaSize, cv::Point* pt1, cv::Point* pt2, int connectivity, bool leftToRight) {
		try {
			cv::LineIterator* ret = new cv::LineIterator(*boundingAreaSize, *pt1, *pt2, connectivity, leftToRight);
			return Ok<cv::LineIterator*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::LineIterator*>))
	}
	
	Result<cv::LineIterator*> cv_LineIterator_LineIterator_Rect_Point_Point_int_bool(cv::Rect* boundingAreaRect, cv::Point* pt1, cv::Point* pt2, int connectivity, bool leftToRight) {
		try {
			cv::LineIterator* ret = new cv::LineIterator(*boundingAreaRect, *pt1, *pt2, connectivity, leftToRight);
			return Ok<cv::LineIterator*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::LineIterator*>))
	}
	
	Result_void cv_LineIterator_init_const_MatX_Rect_Point_Point_int_bool(cv::LineIterator* instance, const cv::Mat* img, cv::Rect* boundingAreaRect, cv::Point* pt1, cv::Point* pt2, int connectivity, bool leftToRight) {
		try {
			instance->init(img, *boundingAreaRect, *pt1, *pt2, connectivity, leftToRight);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<unsigned char*> cv_LineIterator_operatorX(cv::LineIterator* instance) {
		try {
			unsigned char* ret = instance->operator*();
			return Ok<unsigned char*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned char*>))
	}
	
	Result<cv::Point> cv_LineIterator_pos_const(const cv::LineIterator* instance) {
		try {
			cv::Point ret = instance->pos();
			return Ok<cv::Point>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point>))
	}
	
	Result_void cv_LineSegmentDetector_detect_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(cv::LineSegmentDetector* instance, const cv::_InputArray* _image, const cv::_OutputArray* _lines, const cv::_OutputArray* width, const cv::_OutputArray* prec, const cv::_OutputArray* nfa) {
		try {
			instance->detect(*_image, *_lines, *width, *prec, *nfa);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_LineSegmentDetector_drawSegments_const__InputOutputArrayR_const__InputArrayR(cv::LineSegmentDetector* instance, const cv::_InputOutputArray* _image, const cv::_InputArray* lines) {
		try {
			instance->drawSegments(*_image, *lines);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_LineSegmentDetector_compareSegments_const_SizeR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR(cv::LineSegmentDetector* instance, const cv::Size* size, const cv::_InputArray* lines1, const cv::_InputArray* lines2, const cv::_InputOutputArray* _image) {
		try {
			int ret = instance->compareSegments(*size, *lines1, *lines2, *_image);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	void cv_Subdiv2D_delete(cv::Subdiv2D* instance) {
		delete instance;
	}
	Result<cv::Subdiv2D*> cv_Subdiv2D_Subdiv2D() {
		try {
			cv::Subdiv2D* ret = new cv::Subdiv2D();
			return Ok<cv::Subdiv2D*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Subdiv2D*>))
	}
	
	Result<cv::Subdiv2D*> cv_Subdiv2D_Subdiv2D_Rect(cv::Rect* rect) {
		try {
			cv::Subdiv2D* ret = new cv::Subdiv2D(*rect);
			return Ok<cv::Subdiv2D*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Subdiv2D*>))
	}
	
	Result_void cv_Subdiv2D_initDelaunay_Rect(cv::Subdiv2D* instance, cv::Rect* rect) {
		try {
			instance->initDelaunay(*rect);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_Subdiv2D_insert_Point2f(cv::Subdiv2D* instance, cv::Point2f* pt) {
		try {
			int ret = instance->insert(*pt);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_Subdiv2D_insert_const_vector_Point2f_R(cv::Subdiv2D* instance, const std::vector<cv::Point2f>* ptvec) {
		try {
			instance->insert(*ptvec);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_Subdiv2D_locate_Point2f_intR_intR(cv::Subdiv2D* instance, cv::Point2f* pt, int* edge, int* vertex) {
		try {
			int ret = instance->locate(*pt, *edge, *vertex);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_Subdiv2D_findNearest_Point2f_Point2fX(cv::Subdiv2D* instance, cv::Point2f* pt, cv::Point2f* nearestPt) {
		try {
			int ret = instance->findNearest(*pt, nearestPt);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_Subdiv2D_getEdgeList_const_vector_Vec4f_R(const cv::Subdiv2D* instance, std::vector<cv::Vec4f>* edgeList) {
		try {
			instance->getEdgeList(*edgeList);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_Subdiv2D_getLeadingEdgeList_const_vector_int_R(const cv::Subdiv2D* instance, std::vector<int>* leadingEdgeList) {
		try {
			instance->getLeadingEdgeList(*leadingEdgeList);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_Subdiv2D_getTriangleList_const_vector_Vec6f_R(const cv::Subdiv2D* instance, std::vector<cv::Vec6f>* triangleList) {
		try {
			instance->getTriangleList(*triangleList);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_Subdiv2D_getVoronoiFacetList_const_vector_int_R_vector_vector_Point2f__R_vector_Point2f_R(cv::Subdiv2D* instance, const std::vector<int>* idx, std::vector<std::vector<cv::Point2f>>* facetList, std::vector<cv::Point2f>* facetCenters) {
		try {
			instance->getVoronoiFacetList(*idx, *facetList, *facetCenters);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Point2f> cv_Subdiv2D_getVertex_const_int_intX(const cv::Subdiv2D* instance, int vertex, int* firstEdge) {
		try {
			cv::Point2f ret = instance->getVertex(vertex, firstEdge);
			return Ok<cv::Point2f>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point2f>))
	}
	
	Result<int> cv_Subdiv2D_getEdge_const_int_int(const cv::Subdiv2D* instance, int edge, int nextEdgeType) {
		try {
			int ret = instance->getEdge(edge, nextEdgeType);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_Subdiv2D_nextEdge_const_int(const cv::Subdiv2D* instance, int edge) {
		try {
			int ret = instance->nextEdge(edge);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_Subdiv2D_rotateEdge_const_int_int(const cv::Subdiv2D* instance, int edge, int rotate) {
		try {
			int ret = instance->rotateEdge(edge, rotate);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_Subdiv2D_symEdge_const_int(const cv::Subdiv2D* instance, int edge) {
		try {
			int ret = instance->symEdge(edge);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_Subdiv2D_edgeOrg_const_int_Point2fX(const cv::Subdiv2D* instance, int edge, cv::Point2f* orgpt) {
		try {
			int ret = instance->edgeOrg(edge, orgpt);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_Subdiv2D_edgeDst_const_int_Point2fX(const cv::Subdiv2D* instance, int edge, cv::Point2f* dstpt) {
		try {
			int ret = instance->edgeDst(edge, dstpt);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	void cv_IntelligentScissorsMB_delete(cv::segmentation::IntelligentScissorsMB* instance) {
		delete instance;
	}
	Result<cv::segmentation::IntelligentScissorsMB*> cv_segmentation_IntelligentScissorsMB_IntelligentScissorsMB() {
		try {
			cv::segmentation::IntelligentScissorsMB* ret = new cv::segmentation::IntelligentScissorsMB();
			return Ok<cv::segmentation::IntelligentScissorsMB*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::segmentation::IntelligentScissorsMB*>))
	}
	
	Result<cv::segmentation::IntelligentScissorsMB*> cv_segmentation_IntelligentScissorsMB_setWeights_float_float_float(cv::segmentation::IntelligentScissorsMB* instance, float weight_non_edge, float weight_gradient_direction, float weight_gradient_magnitude) {
		try {
			cv::segmentation::IntelligentScissorsMB ret = instance->setWeights(weight_non_edge, weight_gradient_direction, weight_gradient_magnitude);
			return Ok(new cv::segmentation::IntelligentScissorsMB(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::segmentation::IntelligentScissorsMB*>))
	}
	
	Result<cv::segmentation::IntelligentScissorsMB*> cv_segmentation_IntelligentScissorsMB_setGradientMagnitudeMaxLimit_float(cv::segmentation::IntelligentScissorsMB* instance, float gradient_magnitude_threshold_max) {
		try {
			cv::segmentation::IntelligentScissorsMB ret = instance->setGradientMagnitudeMaxLimit(gradient_magnitude_threshold_max);
			return Ok(new cv::segmentation::IntelligentScissorsMB(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::segmentation::IntelligentScissorsMB*>))
	}
	
	Result<cv::segmentation::IntelligentScissorsMB*> cv_segmentation_IntelligentScissorsMB_setEdgeFeatureZeroCrossingParameters_float(cv::segmentation::IntelligentScissorsMB* instance, float gradient_magnitude_min_value) {
		try {
			cv::segmentation::IntelligentScissorsMB ret = instance->setEdgeFeatureZeroCrossingParameters(gradient_magnitude_min_value);
			return Ok(new cv::segmentation::IntelligentScissorsMB(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::segmentation::IntelligentScissorsMB*>))
	}
	
	Result<cv::segmentation::IntelligentScissorsMB*> cv_segmentation_IntelligentScissorsMB_setEdgeFeatureCannyParameters_double_double_int_bool(cv::segmentation::IntelligentScissorsMB* instance, double threshold1, double threshold2, int apertureSize, bool L2gradient) {
		try {
			cv::segmentation::IntelligentScissorsMB ret = instance->setEdgeFeatureCannyParameters(threshold1, threshold2, apertureSize, L2gradient);
			return Ok(new cv::segmentation::IntelligentScissorsMB(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::segmentation::IntelligentScissorsMB*>))
	}
	
	Result<cv::segmentation::IntelligentScissorsMB*> cv_segmentation_IntelligentScissorsMB_applyImage_const__InputArrayR(cv::segmentation::IntelligentScissorsMB* instance, const cv::_InputArray* image) {
		try {
			cv::segmentation::IntelligentScissorsMB ret = instance->applyImage(*image);
			return Ok(new cv::segmentation::IntelligentScissorsMB(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::segmentation::IntelligentScissorsMB*>))
	}
	
	Result<cv::segmentation::IntelligentScissorsMB*> cv_segmentation_IntelligentScissorsMB_applyImageFeatures_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(cv::segmentation::IntelligentScissorsMB* instance, const cv::_InputArray* non_edge, const cv::_InputArray* gradient_direction, const cv::_InputArray* gradient_magnitude, const cv::_InputArray* image) {
		try {
			cv::segmentation::IntelligentScissorsMB ret = instance->applyImageFeatures(*non_edge, *gradient_direction, *gradient_magnitude, *image);
			return Ok(new cv::segmentation::IntelligentScissorsMB(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::segmentation::IntelligentScissorsMB*>))
	}
	
	Result_void cv_segmentation_IntelligentScissorsMB_buildMap_const_PointR(cv::segmentation::IntelligentScissorsMB* instance, const cv::Point* sourcePt) {
		try {
			instance->buildMap(*sourcePt);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_segmentation_IntelligentScissorsMB_getContour_const_const_PointR_const__OutputArrayR_bool(const cv::segmentation::IntelligentScissorsMB* instance, const cv::Point* targetPt, const cv::_OutputArray* contour, bool backward) {
		try {
			instance->getContour(*targetPt, *contour, backward);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
}
