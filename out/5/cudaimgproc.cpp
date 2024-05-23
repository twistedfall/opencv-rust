#include "ocvrs_common.hpp"
#include <opencv2/cudaimgproc.hpp>
#include "cudaimgproc_types.hpp"

extern "C" {
	// cv::cuda::alphaComp(InputArray, InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:188
	// ("cv::cuda::alphaComp", vec![(pred!(mut, ["img1", "img2", "dst", "alpha_op"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_cuda_alphaComp_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* img1, const cv::_InputArray* img2, const cv::_OutputArray* dst, int alpha_op, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::alphaComp(*img1, *img2, *dst, alpha_op);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// alphaComp(InputArray, InputArray, OutputArray, int, Stream &)(InputArray, InputArray, OutputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:188
	// ("cv::cuda::alphaComp", vec![(pred!(mut, ["img1", "img2", "dst", "alpha_op", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_alphaComp_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_StreamR(const cv::_InputArray* img1, const cv::_InputArray* img2, const cv::_OutputArray* dst, int alpha_op, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::alphaComp(*img1, *img2, *dst, alpha_op, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::bilateralFilter(InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:724
	// ("cv::cuda::bilateralFilter", vec![(pred!(mut, ["src", "dst", "kernel_size", "sigma_color", "sigma_spatial"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "float", "float"]), _)]),
	void cv_cuda_bilateralFilter_const__InputArrayR_const__OutputArrayR_int_float_float(const cv::_InputArray* src, const cv::_OutputArray* dst, int kernel_size, float sigma_color, float sigma_spatial, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::bilateralFilter(*src, *dst, kernel_size, sigma_color, sigma_spatial);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// bilateralFilter(InputArray, OutputArray, int, float, float, int, Stream &)(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:724
	// ("cv::cuda::bilateralFilter", vec![(pred!(mut, ["src", "dst", "kernel_size", "sigma_color", "sigma_spatial", "borderMode", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "float", "float", "int", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_bilateralFilter_const__InputArrayR_const__OutputArrayR_int_float_float_int_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, int kernel_size, float sigma_color, float sigma_spatial, int borderMode, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::bilateralFilter(*src, *dst, kernel_size, sigma_color, sigma_spatial, borderMode, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::blendLinear(InputArray, InputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:740
	// ("cv::cuda::blendLinear", vec![(pred!(mut, ["img1", "img2", "weights1", "weights2", "result"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_blendLinear_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* img1, const cv::_InputArray* img2, const cv::_InputArray* weights1, const cv::_InputArray* weights2, const cv::_OutputArray* result, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::blendLinear(*img1, *img2, *weights1, *weights2, *result);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// blendLinear(InputArray, InputArray, InputArray, InputArray, OutputArray, Stream &)(InputArray, InputArray, InputArray, InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:740
	// ("cv::cuda::blendLinear", vec![(pred!(mut, ["img1", "img2", "weights1", "weights2", "result", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_blendLinear_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_StreamR(const cv::_InputArray* img1, const cv::_InputArray* img2, const cv::_InputArray* weights1, const cv::_InputArray* weights2, const cv::_OutputArray* result, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::blendLinear(*img1, *img2, *weights1, *weights2, *result, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::calcHist(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:212
	// ("cv::cuda::calcHist", vec![(pred!(mut, ["src", "mask", "hist"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_calcHist_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_InputArray* mask, const cv::_OutputArray* hist, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::calcHist(*src, *mask, *hist);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// calcHist(InputArray, InputArray, OutputArray, Stream &)(InputArray, InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:212
	// ("cv::cuda::calcHist", vec![(pred!(mut, ["src", "mask", "hist", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_calcHist_const__InputArrayR_const__InputArrayR_const__OutputArrayR_StreamR(const cv::_InputArray* src, const cv::_InputArray* mask, const cv::_OutputArray* hist, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::calcHist(*src, *mask, *hist, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::calcHist(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:203
	// ("cv::cuda::calcHist", vec![(pred!(mut, ["src", "hist"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_calcHist_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* hist, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::calcHist(*src, *hist);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// calcHist(InputArray, OutputArray, Stream &)(InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:203
	// ("cv::cuda::calcHist", vec![(pred!(mut, ["src", "hist", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_calcHist_const__InputArrayR_const__OutputArrayR_StreamR(const cv::_InputArray* src, const cv::_OutputArray* hist, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::calcHist(*src, *hist, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::connectedComponents(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:783
	// ("cv::cuda::connectedComponents", vec![(pred!(mut, ["image", "labels"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_connectedComponents_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* image, const cv::_OutputArray* labels, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::connectedComponents(*image, *labels);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// connectedComponents(InputArray, OutputArray, int, int)(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:783
	// ("cv::cuda::connectedComponents", vec![(pred!(mut, ["image", "labels", "connectivity", "ltype"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int"]), _)]),
	void cv_cuda_connectedComponents_const__InputArrayR_const__OutputArrayR_int_int(const cv::_InputArray* image, const cv::_OutputArray* labels, int connectivity, int ltype, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::connectedComponents(*image, *labels, connectivity, ltype);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// connectedComponents(InputArray, OutputArray, int, int, cv::cuda::ConnectedComponentsAlgorithmsTypes)(InputArray, OutputArray, Primitive, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:772
	// ("cv::cuda::connectedComponents", vec![(pred!(mut, ["image", "labels", "connectivity", "ltype", "ccltype"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "cv::cuda::ConnectedComponentsAlgorithmsTypes"]), _)]),
	void cv_cuda_connectedComponents_const__InputArrayR_const__OutputArrayR_int_int_ConnectedComponentsAlgorithmsTypes(const cv::_InputArray* image, const cv::_OutputArray* labels, int connectivity, int ltype, cv::cuda::ConnectedComponentsAlgorithmsTypes ccltype, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::connectedComponents(*image, *labels, connectivity, ltype, ccltype);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// convertSpatialMoments(Mat, const MomentsOrder, const int)(TraitClass, Enum, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:819
	// ("cv::cuda::convertSpatialMoments", vec![(pred!(mut, ["spatialMoments", "order", "momentsType"], ["cv::Mat", "const cv::cuda::MomentsOrder", "const int"]), _)]),
	void cv_cuda_convertSpatialMoments_Mat_const_MomentsOrder_const_int(cv::Mat* spatialMoments, const cv::cuda::MomentsOrder order, const int momentsType, Result<cv::Moments>* ocvrs_return) {
		try {
			cv::Moments ret = cv::cuda::convertSpatialMoments(*spatialMoments, order, momentsType);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::createCLAHE() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:245
	// ("cv::cuda::createCLAHE", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_createCLAHE(Result<cv::Ptr<cv::cuda::CLAHE>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::CLAHE> ret = cv::cuda::createCLAHE();
			Ok(new cv::Ptr<cv::cuda::CLAHE>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createCLAHE(double, Size)(Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:245
	// ("cv::cuda::createCLAHE", vec![(pred!(mut, ["clipLimit", "tileGridSize"], ["double", "cv::Size"]), _)]),
	void cv_cuda_createCLAHE_double_Size(double clipLimit, cv::Size* tileGridSize, Result<cv::Ptr<cv::cuda::CLAHE>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::CLAHE> ret = cv::cuda::createCLAHE(clipLimit, *tileGridSize);
			Ok(new cv::Ptr<cv::cuda::CLAHE>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::createCannyEdgeDetector(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:330
	// ("cv::cuda::createCannyEdgeDetector", vec![(pred!(mut, ["low_thresh", "high_thresh"], ["double", "double"]), _)]),
	void cv_cuda_createCannyEdgeDetector_double_double(double low_thresh, double high_thresh, Result<cv::Ptr<cv::cuda::CannyEdgeDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::CannyEdgeDetector> ret = cv::cuda::createCannyEdgeDetector(low_thresh, high_thresh);
			Ok(new cv::Ptr<cv::cuda::CannyEdgeDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createCannyEdgeDetector(double, double, int, bool)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:330
	// ("cv::cuda::createCannyEdgeDetector", vec![(pred!(mut, ["low_thresh", "high_thresh", "apperture_size", "L2gradient"], ["double", "double", "int", "bool"]), _)]),
	void cv_cuda_createCannyEdgeDetector_double_double_int_bool(double low_thresh, double high_thresh, int apperture_size, bool L2gradient, Result<cv::Ptr<cv::cuda::CannyEdgeDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::CannyEdgeDetector> ret = cv::cuda::createCannyEdgeDetector(low_thresh, high_thresh, apperture_size, L2gradient);
			Ok(new cv::Ptr<cv::cuda::CannyEdgeDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createGeneralizedHoughBallard()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:510
	// ("cv::cuda::createGeneralizedHoughBallard", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_createGeneralizedHoughBallard(Result<cv::Ptr<cv::GeneralizedHoughBallard>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::GeneralizedHoughBallard> ret = cv::cuda::createGeneralizedHoughBallard();
			Ok(new cv::Ptr<cv::GeneralizedHoughBallard>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createGeneralizedHoughGuil()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:514
	// ("cv::cuda::createGeneralizedHoughGuil", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_createGeneralizedHoughGuil(Result<cv::Ptr<cv::GeneralizedHoughGuil>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::GeneralizedHoughGuil> ret = cv::cuda::createGeneralizedHoughGuil();
			Ok(new cv::Ptr<cv::GeneralizedHoughGuil>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::createGoodFeaturesToTrackDetector(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:604
	// ("cv::cuda::createGoodFeaturesToTrackDetector", vec![(pred!(mut, ["srcType"], ["int"]), _)]),
	void cv_cuda_createGoodFeaturesToTrackDetector_int(int srcType, Result<cv::Ptr<cv::cuda::CornersDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::CornersDetector> ret = cv::cuda::createGoodFeaturesToTrackDetector(srcType);
			Ok(new cv::Ptr<cv::cuda::CornersDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createGoodFeaturesToTrackDetector(int, int, double, double, int, bool, double)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:604
	// ("cv::cuda::createGoodFeaturesToTrackDetector", vec![(pred!(mut, ["srcType", "maxCorners", "qualityLevel", "minDistance", "blockSize", "useHarrisDetector", "harrisK"], ["int", "int", "double", "double", "int", "bool", "double"]), _)]),
	void cv_cuda_createGoodFeaturesToTrackDetector_int_int_double_double_int_bool_double(int srcType, int maxCorners, double qualityLevel, double minDistance, int blockSize, bool useHarrisDetector, double harrisK, Result<cv::Ptr<cv::cuda::CornersDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::CornersDetector> ret = cv::cuda::createGoodFeaturesToTrackDetector(srcType, maxCorners, qualityLevel, minDistance, blockSize, useHarrisDetector, harrisK);
			Ok(new cv::Ptr<cv::cuda::CornersDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::createHarrisCorner(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:549
	// ("cv::cuda::createHarrisCorner", vec![(pred!(mut, ["srcType", "blockSize", "ksize", "k"], ["int", "int", "int", "double"]), _)]),
	void cv_cuda_createHarrisCorner_int_int_int_double(int srcType, int blockSize, int ksize, double k, Result<cv::Ptr<cv::cuda::CornernessCriteria>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::CornernessCriteria> ret = cv::cuda::createHarrisCorner(srcType, blockSize, ksize, k);
			Ok(new cv::Ptr<cv::cuda::CornernessCriteria>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createHarrisCorner(int, int, int, double, int)(Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:549
	// ("cv::cuda::createHarrisCorner", vec![(pred!(mut, ["srcType", "blockSize", "ksize", "k", "borderType"], ["int", "int", "int", "double", "int"]), _)]),
	void cv_cuda_createHarrisCorner_int_int_int_double_int(int srcType, int blockSize, int ksize, double k, int borderType, Result<cv::Ptr<cv::cuda::CornernessCriteria>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::CornernessCriteria> ret = cv::cuda::createHarrisCorner(srcType, blockSize, ksize, k, borderType);
			Ok(new cv::Ptr<cv::cuda::CornernessCriteria>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::createHoughCirclesDetector(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:503
	// ("cv::cuda::createHoughCirclesDetector", vec![(pred!(mut, ["dp", "minDist", "cannyThreshold", "votesThreshold", "minRadius", "maxRadius"], ["float", "float", "int", "int", "int", "int"]), _)]),
	void cv_cuda_createHoughCirclesDetector_float_float_int_int_int_int(float dp, float minDist, int cannyThreshold, int votesThreshold, int minRadius, int maxRadius, Result<cv::Ptr<cv::cuda::HoughCirclesDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::HoughCirclesDetector> ret = cv::cuda::createHoughCirclesDetector(dp, minDist, cannyThreshold, votesThreshold, minRadius, maxRadius);
			Ok(new cv::Ptr<cv::cuda::HoughCirclesDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createHoughCirclesDetector(float, float, int, int, int, int, int)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:503
	// ("cv::cuda::createHoughCirclesDetector", vec![(pred!(mut, ["dp", "minDist", "cannyThreshold", "votesThreshold", "minRadius", "maxRadius", "maxCircles"], ["float", "float", "int", "int", "int", "int", "int"]), _)]),
	void cv_cuda_createHoughCirclesDetector_float_float_int_int_int_int_int(float dp, float minDist, int cannyThreshold, int votesThreshold, int minRadius, int maxRadius, int maxCircles, Result<cv::Ptr<cv::cuda::HoughCirclesDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::HoughCirclesDetector> ret = cv::cuda::createHoughCirclesDetector(dp, minDist, cannyThreshold, votesThreshold, minRadius, maxRadius, maxCircles);
			Ok(new cv::Ptr<cv::cuda::HoughCirclesDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::createHoughLinesDetector(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:392
	// ("cv::cuda::createHoughLinesDetector", vec![(pred!(mut, ["rho", "theta", "threshold"], ["float", "float", "int"]), _)]),
	void cv_cuda_createHoughLinesDetector_float_float_int(float rho, float theta, int threshold, Result<cv::Ptr<cv::cuda::HoughLinesDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::HoughLinesDetector> ret = cv::cuda::createHoughLinesDetector(rho, theta, threshold);
			Ok(new cv::Ptr<cv::cuda::HoughLinesDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createHoughLinesDetector(float, float, int, bool, int)(Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:392
	// ("cv::cuda::createHoughLinesDetector", vec![(pred!(mut, ["rho", "theta", "threshold", "doSort", "maxLines"], ["float", "float", "int", "bool", "int"]), _)]),
	void cv_cuda_createHoughLinesDetector_float_float_int_bool_int(float rho, float theta, int threshold, bool doSort, int maxLines, Result<cv::Ptr<cv::cuda::HoughLinesDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::HoughLinesDetector> ret = cv::cuda::createHoughLinesDetector(rho, theta, threshold, doSort, maxLines);
			Ok(new cv::Ptr<cv::cuda::HoughLinesDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::createHoughSegmentDetector(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:444
	// ("cv::cuda::createHoughSegmentDetector", vec![(pred!(mut, ["rho", "theta", "minLineLength", "maxLineGap"], ["float", "float", "int", "int"]), _)]),
	void cv_cuda_createHoughSegmentDetector_float_float_int_int(float rho, float theta, int minLineLength, int maxLineGap, Result<cv::Ptr<cv::cuda::HoughSegmentDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::HoughSegmentDetector> ret = cv::cuda::createHoughSegmentDetector(rho, theta, minLineLength, maxLineGap);
			Ok(new cv::Ptr<cv::cuda::HoughSegmentDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createHoughSegmentDetector(float, float, int, int, int, int)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:444
	// ("cv::cuda::createHoughSegmentDetector", vec![(pred!(mut, ["rho", "theta", "minLineLength", "maxLineGap", "maxLines", "threshold"], ["float", "float", "int", "int", "int", "int"]), _)]),
	void cv_cuda_createHoughSegmentDetector_float_float_int_int_int_int(float rho, float theta, int minLineLength, int maxLineGap, int maxLines, int threshold, Result<cv::Ptr<cv::cuda::HoughSegmentDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::HoughSegmentDetector> ret = cv::cuda::createHoughSegmentDetector(rho, theta, minLineLength, maxLineGap, maxLines, threshold);
			Ok(new cv::Ptr<cv::cuda::HoughSegmentDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::createMinEigenValCorner(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:562
	// ("cv::cuda::createMinEigenValCorner", vec![(pred!(mut, ["srcType", "blockSize", "ksize"], ["int", "int", "int"]), _)]),
	void cv_cuda_createMinEigenValCorner_int_int_int(int srcType, int blockSize, int ksize, Result<cv::Ptr<cv::cuda::CornernessCriteria>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::CornernessCriteria> ret = cv::cuda::createMinEigenValCorner(srcType, blockSize, ksize);
			Ok(new cv::Ptr<cv::cuda::CornernessCriteria>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createMinEigenValCorner(int, int, int, int)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:562
	// ("cv::cuda::createMinEigenValCorner", vec![(pred!(mut, ["srcType", "blockSize", "ksize", "borderType"], ["int", "int", "int", "int"]), _)]),
	void cv_cuda_createMinEigenValCorner_int_int_int_int(int srcType, int blockSize, int ksize, int borderType, Result<cv::Ptr<cv::cuda::CornernessCriteria>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::CornernessCriteria> ret = cv::cuda::createMinEigenValCorner(srcType, blockSize, ksize, borderType);
			Ok(new cv::Ptr<cv::cuda::CornernessCriteria>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::createTemplateMatching(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:706
	// ("cv::cuda::createTemplateMatching", vec![(pred!(mut, ["srcType", "method"], ["int", "int"]), _)]),
	void cv_cuda_createTemplateMatching_int_int(int srcType, int method, Result<cv::Ptr<cv::cuda::TemplateMatching>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::TemplateMatching> ret = cv::cuda::createTemplateMatching(srcType, method);
			Ok(new cv::Ptr<cv::cuda::TemplateMatching>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createTemplateMatching(int, int, Size)(Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:706
	// ("cv::cuda::createTemplateMatching", vec![(pred!(mut, ["srcType", "method", "user_block_size"], ["int", "int", "cv::Size"]), _)]),
	void cv_cuda_createTemplateMatching_int_int_Size(int srcType, int method, cv::Size* user_block_size, Result<cv::Ptr<cv::cuda::TemplateMatching>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::TemplateMatching> ret = cv::cuda::createTemplateMatching(srcType, method, *user_block_size);
			Ok(new cv::Ptr<cv::cuda::TemplateMatching>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::cvtColor(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:91
	// ("cv::cuda::cvtColor", vec![(pred!(mut, ["src", "dst", "code"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_cuda_cvtColor_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int code, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::cvtColor(*src, *dst, code);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cvtColor(InputArray, OutputArray, int, int, Stream &)(InputArray, OutputArray, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:91
	// ("cv::cuda::cvtColor", vec![(pred!(mut, ["src", "dst", "code", "dcn", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_cvtColor_const__InputArrayR_const__OutputArrayR_int_int_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, int code, int dcn, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::cvtColor(*src, *dst, code, dcn, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::demosaicing(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:137
	// ("cv::cuda::demosaicing", vec![(pred!(mut, ["src", "dst", "code"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_cuda_demosaicing_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int code, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::demosaicing(*src, *dst, code);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// demosaicing(InputArray, OutputArray, int, int, Stream &)(InputArray, OutputArray, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:137
	// ("cv::cuda::demosaicing", vec![(pred!(mut, ["src", "dst", "code", "dcn", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_demosaicing_const__InputArrayR_const__OutputArrayR_int_int_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, int code, int dcn, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::demosaicing(*src, *dst, code, dcn, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::equalizeHist(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:222
	// ("cv::cuda::equalizeHist", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_equalizeHist_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::equalizeHist(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// equalizeHist(InputArray, OutputArray, Stream &)(InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:222
	// ("cv::cuda::equalizeHist", vec![(pred!(mut, ["src", "dst", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_equalizeHist_const__InputArrayR_const__OutputArrayR_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::equalizeHist(*src, *dst, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::evenLevels(OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:255
	// ("cv::cuda::evenLevels", vec![(pred!(mut, ["levels", "nLevels", "lowerLevel", "upperLevel"], ["const cv::_OutputArray*", "int", "int", "int"]), _)]),
	void cv_cuda_evenLevels_const__OutputArrayR_int_int_int(const cv::_OutputArray* levels, int nLevels, int lowerLevel, int upperLevel, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::evenLevels(*levels, nLevels, lowerLevel, upperLevel);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// evenLevels(OutputArray, int, int, int, Stream &)(OutputArray, Primitive, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:255
	// ("cv::cuda::evenLevels", vec![(pred!(mut, ["levels", "nLevels", "lowerLevel", "upperLevel", "stream"], ["const cv::_OutputArray*", "int", "int", "int", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_evenLevels_const__OutputArrayR_int_int_int_StreamR(const cv::_OutputArray* levels, int nLevels, int lowerLevel, int upperLevel, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::evenLevels(*levels, nLevels, lowerLevel, upperLevel, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::gammaCorrection(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:158
	// ("cv::cuda::gammaCorrection", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_gammaCorrection_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::gammaCorrection(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// gammaCorrection(InputArray, OutputArray, bool, Stream &)(InputArray, OutputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:158
	// ("cv::cuda::gammaCorrection", vec![(pred!(mut, ["src", "dst", "forward", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "bool", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_gammaCorrection_const__InputArrayR_const__OutputArrayR_bool_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, bool forward, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::gammaCorrection(*src, *dst, forward, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::histEven(InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:267
	// ("cv::cuda::histEven", vec![(pred!(mut, ["src", "hist", "histSize", "lowerLevel", "upperLevel"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "int"]), _)]),
	void cv_cuda_histEven_const__InputArrayR_const__OutputArrayR_int_int_int(const cv::_InputArray* src, const cv::_OutputArray* hist, int histSize, int lowerLevel, int upperLevel, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::histEven(*src, *hist, histSize, lowerLevel, upperLevel);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// histEven(InputArray, OutputArray, int, int, int, Stream &)(InputArray, OutputArray, Primitive, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:267
	// ("cv::cuda::histEven", vec![(pred!(mut, ["src", "hist", "histSize", "lowerLevel", "upperLevel", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "int", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_histEven_const__InputArrayR_const__OutputArrayR_int_int_int_StreamR(const cv::_InputArray* src, const cv::_OutputArray* hist, int histSize, int lowerLevel, int upperLevel, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::histEven(*src, *hist, histSize, lowerLevel, upperLevel, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::histRange(InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:279
	// ("cv::cuda::histRange", vec![(pred!(mut, ["src", "hist", "levels"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
	void cv_cuda_histRange_const__InputArrayR_const__OutputArrayR_const__InputArrayR(const cv::_InputArray* src, const cv::_OutputArray* hist, const cv::_InputArray* levels, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::histRange(*src, *hist, *levels);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// histRange(InputArray, OutputArray, InputArray, Stream &)(InputArray, OutputArray, InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:279
	// ("cv::cuda::histRange", vec![(pred!(mut, ["src", "hist", "levels", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_histRange_const__InputArrayR_const__OutputArrayR_const__InputArrayR_StreamR(const cv::_InputArray* src, const cv::_OutputArray* hist, const cv::_InputArray* levels, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::histRange(*src, *hist, *levels, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::meanShiftFiltering(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:625
	// ("cv::cuda::meanShiftFiltering", vec![(pred!(mut, ["src", "dst", "sp", "sr"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int"]), _)]),
	void cv_cuda_meanShiftFiltering_const__InputArrayR_const__OutputArrayR_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int sp, int sr, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::meanShiftFiltering(*src, *dst, sp, sr);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// meanShiftFiltering(InputArray, OutputArray, int, int, TermCriteria, Stream &)(InputArray, OutputArray, Primitive, Primitive, SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:625
	// ("cv::cuda::meanShiftFiltering", vec![(pred!(mut, ["src", "dst", "sp", "sr", "criteria", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "cv::TermCriteria", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_meanShiftFiltering_const__InputArrayR_const__OutputArrayR_int_int_TermCriteria_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, int sp, int sr, cv::TermCriteria* criteria, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::meanShiftFiltering(*src, *dst, sp, sr, *criteria, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::meanShiftProc(InputArray, OutputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:644
	// ("cv::cuda::meanShiftProc", vec![(pred!(mut, ["src", "dstr", "dstsp", "sp", "sr"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "int"]), _)]),
	void cv_cuda_meanShiftProc_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int_int(const cv::_InputArray* src, const cv::_OutputArray* dstr, const cv::_OutputArray* dstsp, int sp, int sr, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::meanShiftProc(*src, *dstr, *dstsp, sp, sr);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// meanShiftProc(InputArray, OutputArray, OutputArray, int, int, TermCriteria, Stream &)(InputArray, OutputArray, OutputArray, Primitive, Primitive, SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:644
	// ("cv::cuda::meanShiftProc", vec![(pred!(mut, ["src", "dstr", "dstsp", "sp", "sr", "criteria", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int", "int", "cv::TermCriteria", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_meanShiftProc_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int_int_TermCriteria_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dstr, const cv::_OutputArray* dstsp, int sp, int sr, cv::TermCriteria* criteria, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::meanShiftProc(*src, *dstr, *dstsp, sp, sr, *criteria, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::meanShiftSegmentation(InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:658
	// ("cv::cuda::meanShiftSegmentation", vec![(pred!(mut, ["src", "dst", "sp", "sr", "minsize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "int"]), _)]),
	void cv_cuda_meanShiftSegmentation_const__InputArrayR_const__OutputArrayR_int_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int sp, int sr, int minsize, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::meanShiftSegmentation(*src, *dst, sp, sr, minsize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// meanShiftSegmentation(InputArray, OutputArray, int, int, int, TermCriteria, Stream &)(InputArray, OutputArray, Primitive, Primitive, Primitive, SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:658
	// ("cv::cuda::meanShiftSegmentation", vec![(pred!(mut, ["src", "dst", "sp", "sr", "minsize", "criteria", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "int", "cv::TermCriteria", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_meanShiftSegmentation_const__InputArrayR_const__OutputArrayR_int_int_int_TermCriteria_StreamR(const cv::_InputArray* src, const cv::_OutputArray* dst, int sp, int sr, int minsize, cv::TermCriteria* criteria, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::meanShiftSegmentation(*src, *dst, sp, sr, minsize, *criteria, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::moments(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:871
	// ("cv::cuda::moments", vec![(pred!(mut, ["src"], ["const cv::_InputArray*"]), _)]),
	void cv_cuda_moments_const__InputArrayR(const cv::_InputArray* src, Result<cv::Moments>* ocvrs_return) {
		try {
			cv::Moments ret = cv::cuda::moments(*src);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// moments(InputArray, const bool, const MomentsOrder, const int)(InputArray, Primitive, Enum, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:871
	// ("cv::cuda::moments", vec![(pred!(mut, ["src", "binaryImage", "order", "momentsType"], ["const cv::_InputArray*", "const bool", "const cv::cuda::MomentsOrder", "const int"]), _)]),
	void cv_cuda_moments_const__InputArrayR_const_bool_const_MomentsOrder_const_int(const cv::_InputArray* src, const bool binaryImage, const cv::cuda::MomentsOrder order, const int momentsType, Result<cv::Moments>* ocvrs_return) {
		try {
			cv::Moments ret = cv::cuda::moments(*src, binaryImage, order, momentsType);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// numMoments(const MomentsOrder)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:808
	// ("cv::cuda::numMoments", vec![(pred!(mut, ["order"], ["const cv::cuda::MomentsOrder"]), _)]),
	void cv_cuda_numMoments_const_MomentsOrder(const cv::cuda::MomentsOrder order, Result<int>* ocvrs_return) {
		try {
			int ret = cv::cuda::numMoments(order);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::spatialMoments(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:849
	// ("cv::cuda::spatialMoments", vec![(pred!(mut, ["src", "moments"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_spatialMoments_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* moments, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::spatialMoments(*src, *moments);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// spatialMoments(InputArray, OutputArray, const bool, const MomentsOrder, const int, Stream &)(InputArray, OutputArray, Primitive, Enum, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:849
	// ("cv::cuda::spatialMoments", vec![(pred!(mut, ["src", "moments", "binaryImage", "order", "momentsType", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const bool", "const cv::cuda::MomentsOrder", "const int", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_spatialMoments_const__InputArrayR_const__OutputArrayR_const_bool_const_MomentsOrder_const_int_StreamR(const cv::_InputArray* src, const cv::_OutputArray* moments, const bool binaryImage, const cv::cuda::MomentsOrder order, const int momentsType, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::spatialMoments(*src, *moments, binaryImage, order, momentsType, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::swapChannels(InputOutputArray, FixedArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:149
	// ("cv::cuda::swapChannels", vec![(pred!(mut, ["image", "dstOrder"], ["const cv::_InputOutputArray*", "const int**"]), _)]),
	void cv_cuda_swapChannels_const__InputOutputArrayR_const_intXX(const cv::_InputOutputArray* image, const int(*dstOrder)[4], ResultVoid* ocvrs_return) {
		try {
			cv::cuda::swapChannels(*image, *dstOrder);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// swapChannels(InputOutputArray, const int *, Stream &)(InputOutputArray, FixedArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:149
	// ("cv::cuda::swapChannels", vec![(pred!(mut, ["image", "dstOrder", "stream"], ["const cv::_InputOutputArray*", "const int**", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_swapChannels_const__InputOutputArrayR_const_intXX_StreamR(const cv::_InputOutputArray* image, const int(*dstOrder)[4], cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::swapChannels(*image, *dstOrder, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// apply(InputArray, OutputArray, Stream &)(InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:236
	// ("cv::cuda::CLAHE::apply", vec![(pred!(mut, ["src", "dst", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_CLAHE_apply_const__InputArrayR_const__OutputArrayR_StreamR(cv::cuda::CLAHE* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			instance->apply(*src, *dst, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::CLAHE::to_Algorithm() generated
	// ("cv::cuda::CLAHE::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_cuda_CLAHE_to_Algorithm(cv::cuda::CLAHE* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::cuda::CLAHE::to_CLAHE() generated
	// ("cv::cuda::CLAHE::to_CLAHE", vec![(pred!(mut, [], []), _)]),
	cv::CLAHE* cv_cuda_CLAHE_to_CLAHE(cv::cuda::CLAHE* instance) {
			return dynamic_cast<cv::CLAHE*>(instance);
	}

	// cv::cuda::CLAHE::delete() generated
	// ("cv::cuda::CLAHE::delete", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_CLAHE_delete(cv::cuda::CLAHE* instance) {
			delete instance;
	}

	// detect(InputArray, OutputArray, Stream &)(InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:298
	// ("cv::cuda::CannyEdgeDetector::detect", vec![(pred!(mut, ["image", "edges", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_CannyEdgeDetector_detect_const__InputArrayR_const__OutputArrayR_StreamR(cv::cuda::CannyEdgeDetector* instance, const cv::_InputArray* image, const cv::_OutputArray* edges, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*image, *edges, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::CannyEdgeDetector::detect(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:298
	// ("cv::cuda::CannyEdgeDetector::detect", vec![(pred!(mut, ["image", "edges"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_CannyEdgeDetector_detect_const__InputArrayR_const__OutputArrayR(cv::cuda::CannyEdgeDetector* instance, const cv::_InputArray* image, const cv::_OutputArray* edges, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*image, *edges);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detect(InputArray, InputArray, OutputArray, Stream &)(InputArray, InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:305
	// ("cv::cuda::CannyEdgeDetector::detect", vec![(pred!(mut, ["dx", "dy", "edges", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_CannyEdgeDetector_detect_const__InputArrayR_const__InputArrayR_const__OutputArrayR_StreamR(cv::cuda::CannyEdgeDetector* instance, const cv::_InputArray* dx, const cv::_InputArray* dy, const cv::_OutputArray* edges, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*dx, *dy, *edges, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::CannyEdgeDetector::detect(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:305
	// ("cv::cuda::CannyEdgeDetector::detect", vec![(pred!(mut, ["dx", "dy", "edges"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_CannyEdgeDetector_detect_const__InputArrayR_const__InputArrayR_const__OutputArrayR(cv::cuda::CannyEdgeDetector* instance, const cv::_InputArray* dx, const cv::_InputArray* dy, const cv::_OutputArray* edges, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*dx, *dy, *edges);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setLowThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:307
	// ("cv::cuda::CannyEdgeDetector::setLowThreshold", vec![(pred!(mut, ["low_thresh"], ["double"]), _)]),
	void cv_cuda_CannyEdgeDetector_setLowThreshold_double(cv::cuda::CannyEdgeDetector* instance, double low_thresh, ResultVoid* ocvrs_return) {
		try {
			instance->setLowThreshold(low_thresh);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLowThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:308
	// ("cv::cuda::CannyEdgeDetector::getLowThreshold", vec![(pred!(const, [], []), _)]),
	void cv_cuda_CannyEdgeDetector_getLowThreshold_const(const cv::cuda::CannyEdgeDetector* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getLowThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setHighThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:310
	// ("cv::cuda::CannyEdgeDetector::setHighThreshold", vec![(pred!(mut, ["high_thresh"], ["double"]), _)]),
	void cv_cuda_CannyEdgeDetector_setHighThreshold_double(cv::cuda::CannyEdgeDetector* instance, double high_thresh, ResultVoid* ocvrs_return) {
		try {
			instance->setHighThreshold(high_thresh);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getHighThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:311
	// ("cv::cuda::CannyEdgeDetector::getHighThreshold", vec![(pred!(const, [], []), _)]),
	void cv_cuda_CannyEdgeDetector_getHighThreshold_const(const cv::cuda::CannyEdgeDetector* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getHighThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setAppertureSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:313
	// ("cv::cuda::CannyEdgeDetector::setAppertureSize", vec![(pred!(mut, ["apperture_size"], ["int"]), _)]),
	void cv_cuda_CannyEdgeDetector_setAppertureSize_int(cv::cuda::CannyEdgeDetector* instance, int apperture_size, ResultVoid* ocvrs_return) {
		try {
			instance->setAppertureSize(apperture_size);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getAppertureSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:314
	// ("cv::cuda::CannyEdgeDetector::getAppertureSize", vec![(pred!(const, [], []), _)]),
	void cv_cuda_CannyEdgeDetector_getAppertureSize_const(const cv::cuda::CannyEdgeDetector* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getAppertureSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setL2Gradient(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:316
	// ("cv::cuda::CannyEdgeDetector::setL2Gradient", vec![(pred!(mut, ["L2gradient"], ["bool"]), _)]),
	void cv_cuda_CannyEdgeDetector_setL2Gradient_bool(cv::cuda::CannyEdgeDetector* instance, bool L2gradient, ResultVoid* ocvrs_return) {
		try {
			instance->setL2Gradient(L2gradient);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getL2Gradient()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:317
	// ("cv::cuda::CannyEdgeDetector::getL2Gradient", vec![(pred!(const, [], []), _)]),
	void cv_cuda_CannyEdgeDetector_getL2Gradient_const(const cv::cuda::CannyEdgeDetector* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getL2Gradient();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::CannyEdgeDetector::to_Algorithm() generated
	// ("cv::cuda::CannyEdgeDetector::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_cuda_CannyEdgeDetector_to_Algorithm(cv::cuda::CannyEdgeDetector* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::cuda::CannyEdgeDetector::delete() generated
	// ("cv::cuda::CannyEdgeDetector::delete", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_CannyEdgeDetector_delete(cv::cuda::CannyEdgeDetector* instance) {
			delete instance;
	}

	// compute(InputArray, OutputArray, Stream &)(InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:535
	// ("cv::cuda::CornernessCriteria::compute", vec![(pred!(mut, ["src", "dst", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_CornernessCriteria_compute_const__InputArrayR_const__OutputArrayR_StreamR(cv::cuda::CornernessCriteria* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			instance->compute(*src, *dst, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::CornernessCriteria::compute(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:535
	// ("cv::cuda::CornernessCriteria::compute", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_CornernessCriteria_compute_const__InputArrayR_const__OutputArrayR(cv::cuda::CornernessCriteria* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			instance->compute(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::CornernessCriteria::to_Algorithm() generated
	// ("cv::cuda::CornernessCriteria::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_cuda_CornernessCriteria_to_Algorithm(cv::cuda::CornernessCriteria* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::cuda::CornernessCriteria::delete() generated
	// ("cv::cuda::CornernessCriteria::delete", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_CornernessCriteria_delete(cv::cuda::CornernessCriteria* instance) {
			delete instance;
	}

	// detect(InputArray, OutputArray, InputArray, Stream &)(InputArray, OutputArray, InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:580
	// ("cv::cuda::CornersDetector::detect", vec![(pred!(mut, ["image", "corners", "mask", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_CornersDetector_detect_const__InputArrayR_const__OutputArrayR_const__InputArrayR_StreamR(cv::cuda::CornersDetector* instance, const cv::_InputArray* image, const cv::_OutputArray* corners, const cv::_InputArray* mask, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*image, *corners, *mask, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::CornersDetector::detect(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:580
	// ("cv::cuda::CornersDetector::detect", vec![(pred!(mut, ["image", "corners"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_CornersDetector_detect_const__InputArrayR_const__OutputArrayR(cv::cuda::CornersDetector* instance, const cv::_InputArray* image, const cv::_OutputArray* corners, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*image, *corners);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxCorners(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:582
	// ("cv::cuda::CornersDetector::setMaxCorners", vec![(pred!(mut, ["maxCorners"], ["int"]), _)]),
	void cv_cuda_CornersDetector_setMaxCorners_int(cv::cuda::CornersDetector* instance, int maxCorners, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxCorners(maxCorners);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMinDistance(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:583
	// ("cv::cuda::CornersDetector::setMinDistance", vec![(pred!(mut, ["minDistance"], ["double"]), _)]),
	void cv_cuda_CornersDetector_setMinDistance_double(cv::cuda::CornersDetector* instance, double minDistance, ResultVoid* ocvrs_return) {
		try {
			instance->setMinDistance(minDistance);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::CornersDetector::to_Algorithm() generated
	// ("cv::cuda::CornersDetector::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_cuda_CornersDetector_to_Algorithm(cv::cuda::CornersDetector* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::cuda::CornersDetector::delete() generated
	// ("cv::cuda::CornersDetector::delete", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_CornersDetector_delete(cv::cuda::CornersDetector* instance) {
			delete instance;
	}

	// detect(InputArray, OutputArray, Stream &)(InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:463
	// ("cv::cuda::HoughCirclesDetector::detect", vec![(pred!(mut, ["src", "circles", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_HoughCirclesDetector_detect_const__InputArrayR_const__OutputArrayR_StreamR(cv::cuda::HoughCirclesDetector* instance, const cv::_InputArray* src, const cv::_OutputArray* circles, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*src, *circles, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::HoughCirclesDetector::detect(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:463
	// ("cv::cuda::HoughCirclesDetector::detect", vec![(pred!(mut, ["src", "circles"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_HoughCirclesDetector_detect_const__InputArrayR_const__OutputArrayR(cv::cuda::HoughCirclesDetector* instance, const cv::_InputArray* src, const cv::_OutputArray* circles, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*src, *circles);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDp(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:465
	// ("cv::cuda::HoughCirclesDetector::setDp", vec![(pred!(mut, ["dp"], ["float"]), _)]),
	void cv_cuda_HoughCirclesDetector_setDp_float(cv::cuda::HoughCirclesDetector* instance, float dp, ResultVoid* ocvrs_return) {
		try {
			instance->setDp(dp);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDp()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:466
	// ("cv::cuda::HoughCirclesDetector::getDp", vec![(pred!(const, [], []), _)]),
	void cv_cuda_HoughCirclesDetector_getDp_const(const cv::cuda::HoughCirclesDetector* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getDp();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMinDist(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:468
	// ("cv::cuda::HoughCirclesDetector::setMinDist", vec![(pred!(mut, ["minDist"], ["float"]), _)]),
	void cv_cuda_HoughCirclesDetector_setMinDist_float(cv::cuda::HoughCirclesDetector* instance, float minDist, ResultVoid* ocvrs_return) {
		try {
			instance->setMinDist(minDist);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMinDist()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:469
	// ("cv::cuda::HoughCirclesDetector::getMinDist", vec![(pred!(const, [], []), _)]),
	void cv_cuda_HoughCirclesDetector_getMinDist_const(const cv::cuda::HoughCirclesDetector* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getMinDist();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setCannyThreshold(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:471
	// ("cv::cuda::HoughCirclesDetector::setCannyThreshold", vec![(pred!(mut, ["cannyThreshold"], ["int"]), _)]),
	void cv_cuda_HoughCirclesDetector_setCannyThreshold_int(cv::cuda::HoughCirclesDetector* instance, int cannyThreshold, ResultVoid* ocvrs_return) {
		try {
			instance->setCannyThreshold(cannyThreshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getCannyThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:472
	// ("cv::cuda::HoughCirclesDetector::getCannyThreshold", vec![(pred!(const, [], []), _)]),
	void cv_cuda_HoughCirclesDetector_getCannyThreshold_const(const cv::cuda::HoughCirclesDetector* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getCannyThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setVotesThreshold(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:474
	// ("cv::cuda::HoughCirclesDetector::setVotesThreshold", vec![(pred!(mut, ["votesThreshold"], ["int"]), _)]),
	void cv_cuda_HoughCirclesDetector_setVotesThreshold_int(cv::cuda::HoughCirclesDetector* instance, int votesThreshold, ResultVoid* ocvrs_return) {
		try {
			instance->setVotesThreshold(votesThreshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getVotesThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:475
	// ("cv::cuda::HoughCirclesDetector::getVotesThreshold", vec![(pred!(const, [], []), _)]),
	void cv_cuda_HoughCirclesDetector_getVotesThreshold_const(const cv::cuda::HoughCirclesDetector* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getVotesThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMinRadius(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:477
	// ("cv::cuda::HoughCirclesDetector::setMinRadius", vec![(pred!(mut, ["minRadius"], ["int"]), _)]),
	void cv_cuda_HoughCirclesDetector_setMinRadius_int(cv::cuda::HoughCirclesDetector* instance, int minRadius, ResultVoid* ocvrs_return) {
		try {
			instance->setMinRadius(minRadius);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMinRadius()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:478
	// ("cv::cuda::HoughCirclesDetector::getMinRadius", vec![(pred!(const, [], []), _)]),
	void cv_cuda_HoughCirclesDetector_getMinRadius_const(const cv::cuda::HoughCirclesDetector* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMinRadius();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxRadius(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:480
	// ("cv::cuda::HoughCirclesDetector::setMaxRadius", vec![(pred!(mut, ["maxRadius"], ["int"]), _)]),
	void cv_cuda_HoughCirclesDetector_setMaxRadius_int(cv::cuda::HoughCirclesDetector* instance, int maxRadius, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxRadius(maxRadius);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxRadius()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:481
	// ("cv::cuda::HoughCirclesDetector::getMaxRadius", vec![(pred!(const, [], []), _)]),
	void cv_cuda_HoughCirclesDetector_getMaxRadius_const(const cv::cuda::HoughCirclesDetector* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMaxRadius();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxCircles(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:483
	// ("cv::cuda::HoughCirclesDetector::setMaxCircles", vec![(pred!(mut, ["maxCircles"], ["int"]), _)]),
	void cv_cuda_HoughCirclesDetector_setMaxCircles_int(cv::cuda::HoughCirclesDetector* instance, int maxCircles, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxCircles(maxCircles);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxCircles()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:484
	// ("cv::cuda::HoughCirclesDetector::getMaxCircles", vec![(pred!(const, [], []), _)]),
	void cv_cuda_HoughCirclesDetector_getMaxCircles_const(const cv::cuda::HoughCirclesDetector* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMaxCircles();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::HoughCirclesDetector::to_Algorithm() generated
	// ("cv::cuda::HoughCirclesDetector::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_cuda_HoughCirclesDetector_to_Algorithm(cv::cuda::HoughCirclesDetector* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::cuda::HoughCirclesDetector::delete() generated
	// ("cv::cuda::HoughCirclesDetector::delete", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_HoughCirclesDetector_delete(cv::cuda::HoughCirclesDetector* instance) {
			delete instance;
	}

	// detect(InputArray, OutputArray, Stream &)(InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:356
	// ("cv::cuda::HoughLinesDetector::detect", vec![(pred!(mut, ["src", "lines", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_HoughLinesDetector_detect_const__InputArrayR_const__OutputArrayR_StreamR(cv::cuda::HoughLinesDetector* instance, const cv::_InputArray* src, const cv::_OutputArray* lines, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*src, *lines, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::HoughLinesDetector::detect(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:356
	// ("cv::cuda::HoughLinesDetector::detect", vec![(pred!(mut, ["src", "lines"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_HoughLinesDetector_detect_const__InputArrayR_const__OutputArrayR(cv::cuda::HoughLinesDetector* instance, const cv::_InputArray* src, const cv::_OutputArray* lines, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*src, *lines);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// downloadResults(InputArray, OutputArray, OutputArray, Stream &)(InputArray, OutputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:365
	// ("cv::cuda::HoughLinesDetector::downloadResults", vec![(pred!(mut, ["d_lines", "h_lines", "h_votes", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_HoughLinesDetector_downloadResults_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_StreamR(cv::cuda::HoughLinesDetector* instance, const cv::_InputArray* d_lines, const cv::_OutputArray* h_lines, const cv::_OutputArray* h_votes, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			instance->downloadResults(*d_lines, *h_lines, *h_votes, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::HoughLinesDetector::downloadResults(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:365
	// ("cv::cuda::HoughLinesDetector::downloadResults", vec![(pred!(mut, ["d_lines", "h_lines"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_HoughLinesDetector_downloadResults_const__InputArrayR_const__OutputArrayR(cv::cuda::HoughLinesDetector* instance, const cv::_InputArray* d_lines, const cv::_OutputArray* h_lines, ResultVoid* ocvrs_return) {
		try {
			instance->downloadResults(*d_lines, *h_lines);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setRho(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:367
	// ("cv::cuda::HoughLinesDetector::setRho", vec![(pred!(mut, ["rho"], ["float"]), _)]),
	void cv_cuda_HoughLinesDetector_setRho_float(cv::cuda::HoughLinesDetector* instance, float rho, ResultVoid* ocvrs_return) {
		try {
			instance->setRho(rho);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getRho()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:368
	// ("cv::cuda::HoughLinesDetector::getRho", vec![(pred!(const, [], []), _)]),
	void cv_cuda_HoughLinesDetector_getRho_const(const cv::cuda::HoughLinesDetector* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getRho();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTheta(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:370
	// ("cv::cuda::HoughLinesDetector::setTheta", vec![(pred!(mut, ["theta"], ["float"]), _)]),
	void cv_cuda_HoughLinesDetector_setTheta_float(cv::cuda::HoughLinesDetector* instance, float theta, ResultVoid* ocvrs_return) {
		try {
			instance->setTheta(theta);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTheta()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:371
	// ("cv::cuda::HoughLinesDetector::getTheta", vec![(pred!(const, [], []), _)]),
	void cv_cuda_HoughLinesDetector_getTheta_const(const cv::cuda::HoughLinesDetector* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getTheta();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setThreshold(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:373
	// ("cv::cuda::HoughLinesDetector::setThreshold", vec![(pred!(mut, ["threshold"], ["int"]), _)]),
	void cv_cuda_HoughLinesDetector_setThreshold_int(cv::cuda::HoughLinesDetector* instance, int threshold, ResultVoid* ocvrs_return) {
		try {
			instance->setThreshold(threshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:374
	// ("cv::cuda::HoughLinesDetector::getThreshold", vec![(pred!(const, [], []), _)]),
	void cv_cuda_HoughLinesDetector_getThreshold_const(const cv::cuda::HoughLinesDetector* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDoSort(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:376
	// ("cv::cuda::HoughLinesDetector::setDoSort", vec![(pred!(mut, ["doSort"], ["bool"]), _)]),
	void cv_cuda_HoughLinesDetector_setDoSort_bool(cv::cuda::HoughLinesDetector* instance, bool doSort, ResultVoid* ocvrs_return) {
		try {
			instance->setDoSort(doSort);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDoSort()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:377
	// ("cv::cuda::HoughLinesDetector::getDoSort", vec![(pred!(const, [], []), _)]),
	void cv_cuda_HoughLinesDetector_getDoSort_const(const cv::cuda::HoughLinesDetector* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getDoSort();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxLines(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:379
	// ("cv::cuda::HoughLinesDetector::setMaxLines", vec![(pred!(mut, ["maxLines"], ["int"]), _)]),
	void cv_cuda_HoughLinesDetector_setMaxLines_int(cv::cuda::HoughLinesDetector* instance, int maxLines, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxLines(maxLines);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxLines()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:380
	// ("cv::cuda::HoughLinesDetector::getMaxLines", vec![(pred!(const, [], []), _)]),
	void cv_cuda_HoughLinesDetector_getMaxLines_const(const cv::cuda::HoughLinesDetector* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMaxLines();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::HoughLinesDetector::to_Algorithm() generated
	// ("cv::cuda::HoughLinesDetector::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_cuda_HoughLinesDetector_to_Algorithm(cv::cuda::HoughLinesDetector* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::cuda::HoughLinesDetector::delete() generated
	// ("cv::cuda::HoughLinesDetector::delete", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_HoughLinesDetector_delete(cv::cuda::HoughLinesDetector* instance) {
			delete instance;
	}

	// detect(InputArray, OutputArray, Stream &)(InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:413
	// ("cv::cuda::HoughSegmentDetector::detect", vec![(pred!(mut, ["src", "lines", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_HoughSegmentDetector_detect_const__InputArrayR_const__OutputArrayR_StreamR(cv::cuda::HoughSegmentDetector* instance, const cv::_InputArray* src, const cv::_OutputArray* lines, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*src, *lines, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::HoughSegmentDetector::detect(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:413
	// ("cv::cuda::HoughSegmentDetector::detect", vec![(pred!(mut, ["src", "lines"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_HoughSegmentDetector_detect_const__InputArrayR_const__OutputArrayR(cv::cuda::HoughSegmentDetector* instance, const cv::_InputArray* src, const cv::_OutputArray* lines, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*src, *lines);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setRho(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:415
	// ("cv::cuda::HoughSegmentDetector::setRho", vec![(pred!(mut, ["rho"], ["float"]), _)]),
	void cv_cuda_HoughSegmentDetector_setRho_float(cv::cuda::HoughSegmentDetector* instance, float rho, ResultVoid* ocvrs_return) {
		try {
			instance->setRho(rho);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getRho()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:416
	// ("cv::cuda::HoughSegmentDetector::getRho", vec![(pred!(const, [], []), _)]),
	void cv_cuda_HoughSegmentDetector_getRho_const(const cv::cuda::HoughSegmentDetector* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getRho();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTheta(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:418
	// ("cv::cuda::HoughSegmentDetector::setTheta", vec![(pred!(mut, ["theta"], ["float"]), _)]),
	void cv_cuda_HoughSegmentDetector_setTheta_float(cv::cuda::HoughSegmentDetector* instance, float theta, ResultVoid* ocvrs_return) {
		try {
			instance->setTheta(theta);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTheta()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:419
	// ("cv::cuda::HoughSegmentDetector::getTheta", vec![(pred!(const, [], []), _)]),
	void cv_cuda_HoughSegmentDetector_getTheta_const(const cv::cuda::HoughSegmentDetector* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getTheta();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMinLineLength(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:421
	// ("cv::cuda::HoughSegmentDetector::setMinLineLength", vec![(pred!(mut, ["minLineLength"], ["int"]), _)]),
	void cv_cuda_HoughSegmentDetector_setMinLineLength_int(cv::cuda::HoughSegmentDetector* instance, int minLineLength, ResultVoid* ocvrs_return) {
		try {
			instance->setMinLineLength(minLineLength);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMinLineLength()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:422
	// ("cv::cuda::HoughSegmentDetector::getMinLineLength", vec![(pred!(const, [], []), _)]),
	void cv_cuda_HoughSegmentDetector_getMinLineLength_const(const cv::cuda::HoughSegmentDetector* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMinLineLength();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxLineGap(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:424
	// ("cv::cuda::HoughSegmentDetector::setMaxLineGap", vec![(pred!(mut, ["maxLineGap"], ["int"]), _)]),
	void cv_cuda_HoughSegmentDetector_setMaxLineGap_int(cv::cuda::HoughSegmentDetector* instance, int maxLineGap, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxLineGap(maxLineGap);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxLineGap()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:425
	// ("cv::cuda::HoughSegmentDetector::getMaxLineGap", vec![(pred!(const, [], []), _)]),
	void cv_cuda_HoughSegmentDetector_getMaxLineGap_const(const cv::cuda::HoughSegmentDetector* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMaxLineGap();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxLines(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:427
	// ("cv::cuda::HoughSegmentDetector::setMaxLines", vec![(pred!(mut, ["maxLines"], ["int"]), _)]),
	void cv_cuda_HoughSegmentDetector_setMaxLines_int(cv::cuda::HoughSegmentDetector* instance, int maxLines, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxLines(maxLines);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxLines()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:428
	// ("cv::cuda::HoughSegmentDetector::getMaxLines", vec![(pred!(const, [], []), _)]),
	void cv_cuda_HoughSegmentDetector_getMaxLines_const(const cv::cuda::HoughSegmentDetector* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMaxLines();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setThreshold(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:430
	// ("cv::cuda::HoughSegmentDetector::setThreshold", vec![(pred!(mut, ["threshold"], ["int"]), _)]),
	void cv_cuda_HoughSegmentDetector_setThreshold_int(cv::cuda::HoughSegmentDetector* instance, int threshold, ResultVoid* ocvrs_return) {
		try {
			instance->setThreshold(threshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:431
	// ("cv::cuda::HoughSegmentDetector::getThreshold", vec![(pred!(const, [], []), _)]),
	void cv_cuda_HoughSegmentDetector_getThreshold_const(const cv::cuda::HoughSegmentDetector* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::HoughSegmentDetector::to_Algorithm() generated
	// ("cv::cuda::HoughSegmentDetector::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_cuda_HoughSegmentDetector_to_Algorithm(cv::cuda::HoughSegmentDetector* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::cuda::HoughSegmentDetector::delete() generated
	// ("cv::cuda::HoughSegmentDetector::delete", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_HoughSegmentDetector_delete(cv::cuda::HoughSegmentDetector* instance) {
			delete instance;
	}

	// match(InputArray, InputArray, OutputArray, Stream &)(InputArray, InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:677
	// ("cv::cuda::TemplateMatching::match", vec![(pred!(mut, ["image", "templ", "result", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_TemplateMatching_match_const__InputArrayR_const__InputArrayR_const__OutputArrayR_StreamR(cv::cuda::TemplateMatching* instance, const cv::_InputArray* image, const cv::_InputArray* templ, const cv::_OutputArray* result, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			instance->match(*image, *templ, *result, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::TemplateMatching::match(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaimgproc.hpp:677
	// ("cv::cuda::TemplateMatching::match", vec![(pred!(mut, ["image", "templ", "result"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_TemplateMatching_match_const__InputArrayR_const__InputArrayR_const__OutputArrayR(cv::cuda::TemplateMatching* instance, const cv::_InputArray* image, const cv::_InputArray* templ, const cv::_OutputArray* result, ResultVoid* ocvrs_return) {
		try {
			instance->match(*image, *templ, *result);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::TemplateMatching::to_Algorithm() generated
	// ("cv::cuda::TemplateMatching::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_cuda_TemplateMatching_to_Algorithm(cv::cuda::TemplateMatching* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::cuda::TemplateMatching::delete() generated
	// ("cv::cuda::TemplateMatching::delete", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_TemplateMatching_delete(cv::cuda::TemplateMatching* instance) {
			delete instance;
	}

}
