#include "common.hpp"
#include <opencv2/cudaimgproc.hpp>
#include "cudaimgproc_types.hpp"

extern "C" {
	Result_void cv_cuda_alphaComp_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_StreamR(const cv::_InputArray* img1, const cv::_InputArray* img2, const cv::_OutputArray* dst, int alpha_op, cv::cuda::Stream* stream) {
		try {
			cv::cuda::alphaComp(*img1, *img2, *dst, alpha_op, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_bilateralFilter_const__InputArrayR_const__OutputArrayR_int_float_float_int_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, int kernel_size, float sigma_color, float sigma_spatial, int borderMode, cv::cuda::Stream* stream) {
		try {
			cv::cuda::bilateralFilter(*src, *dst, kernel_size, sigma_color, sigma_spatial, borderMode, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_blendLinear_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_StreamR(const cv::_InputArray* img1, const cv::_InputArray* img2, const cv::_InputArray* weights1, const cv::_InputArray* weights2, const cv::_OutputArray* result, cv::cuda::Stream* stream) {
		try {
			cv::cuda::blendLinear(*img1, *img2, *weights1, *weights2, *result, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_calcHist_const__InputArrayR_const__InputArrayR_const__OutputArrayR_StreamR(const cv::_InputArray* src, const cv::_InputArray* mask, const cv::_OutputArray* hist, cv::cuda::Stream* stream) {
		try {
			cv::cuda::calcHist(*src, *mask, *hist, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_calcHist_const__InputArrayR_const__OutputArrayR_StreamR(const cv::_InputArray* src, const cv::_OutputArray* hist, cv::cuda::Stream* stream) {
		try {
			cv::cuda::calcHist(*src, *hist, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::cuda::CLAHE>*> cv_cuda_createCLAHE_double_Size(double clipLimit, cv::Size* tileGridSize) {
		try {
			cv::Ptr<cv::cuda::CLAHE> ret = cv::cuda::createCLAHE(clipLimit, *tileGridSize);
			return Ok(new cv::Ptr<cv::cuda::CLAHE>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::cuda::CLAHE>*>))
	}
	
	Result<cv::Ptr<cv::cuda::CannyEdgeDetector>*> cv_cuda_createCannyEdgeDetector_double_double_int_bool(double low_thresh, double high_thresh, int apperture_size, bool L2gradient) {
		try {
			cv::Ptr<cv::cuda::CannyEdgeDetector> ret = cv::cuda::createCannyEdgeDetector(low_thresh, high_thresh, apperture_size, L2gradient);
			return Ok(new cv::Ptr<cv::cuda::CannyEdgeDetector>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::cuda::CannyEdgeDetector>*>))
	}
	
	Result<cv::Ptr<cv::GeneralizedHoughBallard>*> cv_cuda_createGeneralizedHoughBallard() {
		try {
			cv::Ptr<cv::GeneralizedHoughBallard> ret = cv::cuda::createGeneralizedHoughBallard();
			return Ok(new cv::Ptr<cv::GeneralizedHoughBallard>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::GeneralizedHoughBallard>*>))
	}
	
	Result<cv::Ptr<cv::GeneralizedHoughGuil>*> cv_cuda_createGeneralizedHoughGuil() {
		try {
			cv::Ptr<cv::GeneralizedHoughGuil> ret = cv::cuda::createGeneralizedHoughGuil();
			return Ok(new cv::Ptr<cv::GeneralizedHoughGuil>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::GeneralizedHoughGuil>*>))
	}
	
	Result<cv::Ptr<cv::cuda::CornersDetector>*> cv_cuda_createGoodFeaturesToTrackDetector_int_int_double_double_int_bool_double(int srcType, int maxCorners, double qualityLevel, double minDistance, int blockSize, bool useHarrisDetector, double harrisK) {
		try {
			cv::Ptr<cv::cuda::CornersDetector> ret = cv::cuda::createGoodFeaturesToTrackDetector(srcType, maxCorners, qualityLevel, minDistance, blockSize, useHarrisDetector, harrisK);
			return Ok(new cv::Ptr<cv::cuda::CornersDetector>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::cuda::CornersDetector>*>))
	}
	
	Result<cv::Ptr<cv::cuda::CornernessCriteria>*> cv_cuda_createHarrisCorner_int_int_int_double_int(int srcType, int blockSize, int ksize, double k, int borderType) {
		try {
			cv::Ptr<cv::cuda::CornernessCriteria> ret = cv::cuda::createHarrisCorner(srcType, blockSize, ksize, k, borderType);
			return Ok(new cv::Ptr<cv::cuda::CornernessCriteria>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::cuda::CornernessCriteria>*>))
	}
	
	Result<cv::Ptr<cv::cuda::HoughCirclesDetector>*> cv_cuda_createHoughCirclesDetector_float_float_int_int_int_int_int(float dp, float minDist, int cannyThreshold, int votesThreshold, int minRadius, int maxRadius, int maxCircles) {
		try {
			cv::Ptr<cv::cuda::HoughCirclesDetector> ret = cv::cuda::createHoughCirclesDetector(dp, minDist, cannyThreshold, votesThreshold, minRadius, maxRadius, maxCircles);
			return Ok(new cv::Ptr<cv::cuda::HoughCirclesDetector>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::cuda::HoughCirclesDetector>*>))
	}
	
	Result<cv::Ptr<cv::cuda::HoughLinesDetector>*> cv_cuda_createHoughLinesDetector_float_float_int_bool_int(float rho, float theta, int threshold, bool doSort, int maxLines) {
		try {
			cv::Ptr<cv::cuda::HoughLinesDetector> ret = cv::cuda::createHoughLinesDetector(rho, theta, threshold, doSort, maxLines);
			return Ok(new cv::Ptr<cv::cuda::HoughLinesDetector>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::cuda::HoughLinesDetector>*>))
	}
	
	Result<cv::Ptr<cv::cuda::HoughSegmentDetector>*> cv_cuda_createHoughSegmentDetector_float_float_int_int_int(float rho, float theta, int minLineLength, int maxLineGap, int maxLines) {
		try {
			cv::Ptr<cv::cuda::HoughSegmentDetector> ret = cv::cuda::createHoughSegmentDetector(rho, theta, minLineLength, maxLineGap, maxLines);
			return Ok(new cv::Ptr<cv::cuda::HoughSegmentDetector>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::cuda::HoughSegmentDetector>*>))
	}
	
	Result<cv::Ptr<cv::cuda::CornernessCriteria>*> cv_cuda_createMinEigenValCorner_int_int_int_int(int srcType, int blockSize, int ksize, int borderType) {
		try {
			cv::Ptr<cv::cuda::CornernessCriteria> ret = cv::cuda::createMinEigenValCorner(srcType, blockSize, ksize, borderType);
			return Ok(new cv::Ptr<cv::cuda::CornernessCriteria>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::cuda::CornernessCriteria>*>))
	}
	
	Result<cv::Ptr<cv::cuda::TemplateMatching>*> cv_cuda_createTemplateMatching_int_int_Size(int srcType, int method, cv::Size* user_block_size) {
		try {
			cv::Ptr<cv::cuda::TemplateMatching> ret = cv::cuda::createTemplateMatching(srcType, method, *user_block_size);
			return Ok(new cv::Ptr<cv::cuda::TemplateMatching>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::cuda::TemplateMatching>*>))
	}
	
	Result_void cv_cuda_cvtColor_const__InputArrayR_const__OutputArrayR_int_int_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, int code, int dcn, cv::cuda::Stream* stream) {
		try {
			cv::cuda::cvtColor(*src, *dst, code, dcn, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_demosaicing_const__InputArrayR_const__OutputArrayR_int_int_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, int code, int dcn, cv::cuda::Stream* stream) {
		try {
			cv::cuda::demosaicing(*src, *dst, code, dcn, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_equalizeHist_const__InputArrayR_const__OutputArrayR_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, cv::cuda::Stream* stream) {
		try {
			cv::cuda::equalizeHist(*src, *dst, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_evenLevels_const__OutputArrayR_int_int_int_StreamR(const cv::_OutputArray* levels, int nLevels, int lowerLevel, int upperLevel, cv::cuda::Stream* stream) {
		try {
			cv::cuda::evenLevels(*levels, nLevels, lowerLevel, upperLevel, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_gammaCorrection_const__InputArrayR_const__OutputArrayR_bool_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, bool forward, cv::cuda::Stream* stream) {
		try {
			cv::cuda::gammaCorrection(*src, *dst, forward, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_histEven_const__InputArrayR_const__OutputArrayR_int_int_int_StreamR(const cv::_InputArray* src, const cv::_OutputArray* hist, int histSize, int lowerLevel, int upperLevel, cv::cuda::Stream* stream) {
		try {
			cv::cuda::histEven(*src, *hist, histSize, lowerLevel, upperLevel, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_histRange_const__InputArrayR_const__OutputArrayR_const__InputArrayR_StreamR(const cv::_InputArray* src, const cv::_OutputArray* hist, const cv::_InputArray* levels, cv::cuda::Stream* stream) {
		try {
			cv::cuda::histRange(*src, *hist, *levels, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_meanShiftFiltering_const__InputArrayR_const__OutputArrayR_int_int_TermCriteria_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, int sp, int sr, cv::TermCriteria* criteria, cv::cuda::Stream* stream) {
		try {
			cv::cuda::meanShiftFiltering(*src, *dst, sp, sr, *criteria, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_meanShiftProc_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int_int_TermCriteria_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dstr, const cv::_OutputArray* dstsp, int sp, int sr, cv::TermCriteria* criteria, cv::cuda::Stream* stream) {
		try {
			cv::cuda::meanShiftProc(*src, *dstr, *dstsp, sp, sr, *criteria, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_meanShiftSegmentation_const__InputArrayR_const__OutputArrayR_int_int_int_TermCriteria_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, int sp, int sr, int minsize, cv::TermCriteria* criteria, cv::cuda::Stream* stream) {
		try {
			cv::cuda::meanShiftSegmentation(*src, *dst, sp, sr, minsize, *criteria, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_swapChannels_const__InputOutputArrayR_const_int_X__4__StreamR(const cv::_InputOutputArray* image, const int(*dstOrder)[4], cv::cuda::Stream* stream) {
		try {
			cv::cuda::swapChannels(*image, *dstOrder, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_CLAHE_apply_const__InputArrayR_const__OutputArrayR_StreamR(cv::cuda::CLAHE* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, cv::cuda::Stream* stream) {
		try {
			instance->apply(*src, *dst, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_CannyEdgeDetector_detect_const__InputArrayR_const__OutputArrayR_StreamR(cv::cuda::CannyEdgeDetector* instance, const cv::_InputArray* image, const cv::_OutputArray* edges, cv::cuda::Stream* stream) {
		try {
			instance->detect(*image, *edges, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_CannyEdgeDetector_detect_const__InputArrayR_const__InputArrayR_const__OutputArrayR_StreamR(cv::cuda::CannyEdgeDetector* instance, const cv::_InputArray* dx, const cv::_InputArray* dy, const cv::_OutputArray* edges, cv::cuda::Stream* stream) {
		try {
			instance->detect(*dx, *dy, *edges, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_CannyEdgeDetector_setLowThreshold_double(cv::cuda::CannyEdgeDetector* instance, double low_thresh) {
		try {
			instance->setLowThreshold(low_thresh);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_cuda_CannyEdgeDetector_getLowThreshold_const(const cv::cuda::CannyEdgeDetector* instance) {
		try {
			double ret = instance->getLowThreshold();
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_cuda_CannyEdgeDetector_setHighThreshold_double(cv::cuda::CannyEdgeDetector* instance, double high_thresh) {
		try {
			instance->setHighThreshold(high_thresh);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_cuda_CannyEdgeDetector_getHighThreshold_const(const cv::cuda::CannyEdgeDetector* instance) {
		try {
			double ret = instance->getHighThreshold();
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_cuda_CannyEdgeDetector_setAppertureSize_int(cv::cuda::CannyEdgeDetector* instance, int apperture_size) {
		try {
			instance->setAppertureSize(apperture_size);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cuda_CannyEdgeDetector_getAppertureSize_const(const cv::cuda::CannyEdgeDetector* instance) {
		try {
			int ret = instance->getAppertureSize();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cuda_CannyEdgeDetector_setL2Gradient_bool(cv::cuda::CannyEdgeDetector* instance, bool L2gradient) {
		try {
			instance->setL2Gradient(L2gradient);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_cuda_CannyEdgeDetector_getL2Gradient_const(const cv::cuda::CannyEdgeDetector* instance) {
		try {
			bool ret = instance->getL2Gradient();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_cuda_CornernessCriteria_compute_const__InputArrayR_const__OutputArrayR_StreamR(cv::cuda::CornernessCriteria* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, cv::cuda::Stream* stream) {
		try {
			instance->compute(*src, *dst, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_CornersDetector_detect_const__InputArrayR_const__OutputArrayR_const__InputArrayR_StreamR(cv::cuda::CornersDetector* instance, const cv::_InputArray* image, const cv::_OutputArray* corners, const cv::_InputArray* mask, cv::cuda::Stream* stream) {
		try {
			instance->detect(*image, *corners, *mask, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_HoughCirclesDetector_detect_const__InputArrayR_const__OutputArrayR_StreamR(cv::cuda::HoughCirclesDetector* instance, const cv::_InputArray* src, const cv::_OutputArray* circles, cv::cuda::Stream* stream) {
		try {
			instance->detect(*src, *circles, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_HoughCirclesDetector_setDp_float(cv::cuda::HoughCirclesDetector* instance, float dp) {
		try {
			instance->setDp(dp);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_cuda_HoughCirclesDetector_getDp_const(const cv::cuda::HoughCirclesDetector* instance) {
		try {
			float ret = instance->getDp();
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_cuda_HoughCirclesDetector_setMinDist_float(cv::cuda::HoughCirclesDetector* instance, float minDist) {
		try {
			instance->setMinDist(minDist);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_cuda_HoughCirclesDetector_getMinDist_const(const cv::cuda::HoughCirclesDetector* instance) {
		try {
			float ret = instance->getMinDist();
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_cuda_HoughCirclesDetector_setCannyThreshold_int(cv::cuda::HoughCirclesDetector* instance, int cannyThreshold) {
		try {
			instance->setCannyThreshold(cannyThreshold);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cuda_HoughCirclesDetector_getCannyThreshold_const(const cv::cuda::HoughCirclesDetector* instance) {
		try {
			int ret = instance->getCannyThreshold();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cuda_HoughCirclesDetector_setVotesThreshold_int(cv::cuda::HoughCirclesDetector* instance, int votesThreshold) {
		try {
			instance->setVotesThreshold(votesThreshold);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cuda_HoughCirclesDetector_getVotesThreshold_const(const cv::cuda::HoughCirclesDetector* instance) {
		try {
			int ret = instance->getVotesThreshold();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cuda_HoughCirclesDetector_setMinRadius_int(cv::cuda::HoughCirclesDetector* instance, int minRadius) {
		try {
			instance->setMinRadius(minRadius);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cuda_HoughCirclesDetector_getMinRadius_const(const cv::cuda::HoughCirclesDetector* instance) {
		try {
			int ret = instance->getMinRadius();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cuda_HoughCirclesDetector_setMaxRadius_int(cv::cuda::HoughCirclesDetector* instance, int maxRadius) {
		try {
			instance->setMaxRadius(maxRadius);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cuda_HoughCirclesDetector_getMaxRadius_const(const cv::cuda::HoughCirclesDetector* instance) {
		try {
			int ret = instance->getMaxRadius();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cuda_HoughCirclesDetector_setMaxCircles_int(cv::cuda::HoughCirclesDetector* instance, int maxCircles) {
		try {
			instance->setMaxCircles(maxCircles);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cuda_HoughCirclesDetector_getMaxCircles_const(const cv::cuda::HoughCirclesDetector* instance) {
		try {
			int ret = instance->getMaxCircles();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cuda_HoughLinesDetector_detect_const__InputArrayR_const__OutputArrayR_StreamR(cv::cuda::HoughLinesDetector* instance, const cv::_InputArray* src, const cv::_OutputArray* lines, cv::cuda::Stream* stream) {
		try {
			instance->detect(*src, *lines, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_HoughLinesDetector_downloadResults_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_StreamR(cv::cuda::HoughLinesDetector* instance, const cv::_InputArray* d_lines, const cv::_OutputArray* h_lines, const cv::_OutputArray* h_votes, cv::cuda::Stream* stream) {
		try {
			instance->downloadResults(*d_lines, *h_lines, *h_votes, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_HoughLinesDetector_setRho_float(cv::cuda::HoughLinesDetector* instance, float rho) {
		try {
			instance->setRho(rho);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_cuda_HoughLinesDetector_getRho_const(const cv::cuda::HoughLinesDetector* instance) {
		try {
			float ret = instance->getRho();
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_cuda_HoughLinesDetector_setTheta_float(cv::cuda::HoughLinesDetector* instance, float theta) {
		try {
			instance->setTheta(theta);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_cuda_HoughLinesDetector_getTheta_const(const cv::cuda::HoughLinesDetector* instance) {
		try {
			float ret = instance->getTheta();
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_cuda_HoughLinesDetector_setThreshold_int(cv::cuda::HoughLinesDetector* instance, int threshold) {
		try {
			instance->setThreshold(threshold);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cuda_HoughLinesDetector_getThreshold_const(const cv::cuda::HoughLinesDetector* instance) {
		try {
			int ret = instance->getThreshold();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cuda_HoughLinesDetector_setDoSort_bool(cv::cuda::HoughLinesDetector* instance, bool doSort) {
		try {
			instance->setDoSort(doSort);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_cuda_HoughLinesDetector_getDoSort_const(const cv::cuda::HoughLinesDetector* instance) {
		try {
			bool ret = instance->getDoSort();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_cuda_HoughLinesDetector_setMaxLines_int(cv::cuda::HoughLinesDetector* instance, int maxLines) {
		try {
			instance->setMaxLines(maxLines);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cuda_HoughLinesDetector_getMaxLines_const(const cv::cuda::HoughLinesDetector* instance) {
		try {
			int ret = instance->getMaxLines();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cuda_HoughSegmentDetector_detect_const__InputArrayR_const__OutputArrayR_StreamR(cv::cuda::HoughSegmentDetector* instance, const cv::_InputArray* src, const cv::_OutputArray* lines, cv::cuda::Stream* stream) {
		try {
			instance->detect(*src, *lines, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cuda_HoughSegmentDetector_setRho_float(cv::cuda::HoughSegmentDetector* instance, float rho) {
		try {
			instance->setRho(rho);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_cuda_HoughSegmentDetector_getRho_const(const cv::cuda::HoughSegmentDetector* instance) {
		try {
			float ret = instance->getRho();
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_cuda_HoughSegmentDetector_setTheta_float(cv::cuda::HoughSegmentDetector* instance, float theta) {
		try {
			instance->setTheta(theta);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_cuda_HoughSegmentDetector_getTheta_const(const cv::cuda::HoughSegmentDetector* instance) {
		try {
			float ret = instance->getTheta();
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_cuda_HoughSegmentDetector_setMinLineLength_int(cv::cuda::HoughSegmentDetector* instance, int minLineLength) {
		try {
			instance->setMinLineLength(minLineLength);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cuda_HoughSegmentDetector_getMinLineLength_const(const cv::cuda::HoughSegmentDetector* instance) {
		try {
			int ret = instance->getMinLineLength();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cuda_HoughSegmentDetector_setMaxLineGap_int(cv::cuda::HoughSegmentDetector* instance, int maxLineGap) {
		try {
			instance->setMaxLineGap(maxLineGap);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cuda_HoughSegmentDetector_getMaxLineGap_const(const cv::cuda::HoughSegmentDetector* instance) {
		try {
			int ret = instance->getMaxLineGap();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cuda_HoughSegmentDetector_setMaxLines_int(cv::cuda::HoughSegmentDetector* instance, int maxLines) {
		try {
			instance->setMaxLines(maxLines);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cuda_HoughSegmentDetector_getMaxLines_const(const cv::cuda::HoughSegmentDetector* instance) {
		try {
			int ret = instance->getMaxLines();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cuda_TemplateMatching_match_const__InputArrayR_const__InputArrayR_const__OutputArrayR_StreamR(cv::cuda::TemplateMatching* instance, const cv::_InputArray* image, const cv::_InputArray* templ, const cv::_OutputArray* result, cv::cuda::Stream* stream) {
		try {
			instance->match(*image, *templ, *result, *stream);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
}
