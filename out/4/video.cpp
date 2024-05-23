#include "ocvrs_common.hpp"
#include <opencv2/video.hpp>
#include "video_types.hpp"

extern "C" {
	// CamShift(InputArray, Rect &, TermCriteria)(InputArray, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:79
	// ("cv::CamShift", vec![(pred!(mut, ["probImage", "window", "criteria"], ["const cv::_InputArray*", "cv::Rect*", "cv::TermCriteria"]), _)]),
	void cv_CamShift_const__InputArrayR_RectR_TermCriteria(const cv::_InputArray* probImage, cv::Rect* window, cv::TermCriteria* criteria, Result<cv::RotatedRect>* ocvrs_return) {
		try {
			cv::RotatedRect ret = cv::CamShift(*probImage, *window, *criteria);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::buildOpticalFlowPyramid(InputArray, OutputArray, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:121
	// ("cv::buildOpticalFlowPyramid", vec![(pred!(mut, ["img", "pyramid", "winSize", "maxLevel"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::Size", "int"]), _)]),
	void cv_buildOpticalFlowPyramid_const__InputArrayR_const__OutputArrayR_Size_int(const cv::_InputArray* img, const cv::_OutputArray* pyramid, cv::Size* winSize, int maxLevel, Result<int>* ocvrs_return) {
		try {
			int ret = cv::buildOpticalFlowPyramid(*img, *pyramid, *winSize, maxLevel);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// buildOpticalFlowPyramid(InputArray, OutputArrayOfArrays, Size, int, bool, int, int, bool)(InputArray, OutputArray, SimpleClass, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:121
	// ("cv::buildOpticalFlowPyramid", vec![(pred!(mut, ["img", "pyramid", "winSize", "maxLevel", "withDerivatives", "pyrBorder", "derivBorder", "tryReuseInputImage"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::Size", "int", "bool", "int", "int", "bool"]), _)]),
	void cv_buildOpticalFlowPyramid_const__InputArrayR_const__OutputArrayR_Size_int_bool_int_int_bool(const cv::_InputArray* img, const cv::_OutputArray* pyramid, cv::Size* winSize, int maxLevel, bool withDerivatives, int pyrBorder, int derivBorder, bool tryReuseInputImage, Result<int>* ocvrs_return) {
		try {
			int ret = cv::buildOpticalFlowPyramid(*img, *pyramid, *winSize, maxLevel, withDerivatives, pyrBorder, derivBorder, tryReuseInputImage);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// calcOpticalFlowFarneback(InputArray, InputArray, InputOutputArray, double, int, int, int, int, double, int)(InputArray, InputArray, InputOutputArray, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:223
	// ("cv::calcOpticalFlowFarneback", vec![(pred!(mut, ["prev", "next", "flow", "pyr_scale", "levels", "winsize", "iterations", "poly_n", "poly_sigma", "flags"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "double", "int", "int", "int", "int", "double", "int"]), _)]),
	void cv_calcOpticalFlowFarneback_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_double_int_int_int_int_double_int(const cv::_InputArray* prev, const cv::_InputArray* next, const cv::_InputOutputArray* flow, double pyr_scale, int levels, int winsize, int iterations, int poly_n, double poly_sigma, int flags, ResultVoid* ocvrs_return) {
		try {
			cv::calcOpticalFlowFarneback(*prev, *next, *flow, pyr_scale, levels, winsize, iterations, poly_n, poly_sigma, flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::calcOpticalFlowPyrLK(InputArray, InputArray, InputArray, InputOutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:178
	// ("cv::calcOpticalFlowPyrLK", vec![(pred!(mut, ["prevImg", "nextImg", "prevPts", "nextPts", "status", "err"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_calcOpticalFlowPyrLK_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* prevImg, const cv::_InputArray* nextImg, const cv::_InputArray* prevPts, const cv::_InputOutputArray* nextPts, const cv::_OutputArray* status, const cv::_OutputArray* err, ResultVoid* ocvrs_return) {
		try {
			cv::calcOpticalFlowPyrLK(*prevImg, *nextImg, *prevPts, *nextPts, *status, *err);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// calcOpticalFlowPyrLK(InputArray, InputArray, InputArray, InputOutputArray, OutputArray, OutputArray, Size, int, TermCriteria, int, double)(InputArray, InputArray, InputArray, InputOutputArray, OutputArray, OutputArray, SimpleClass, Primitive, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:178
	// ("cv::calcOpticalFlowPyrLK", vec![(pred!(mut, ["prevImg", "nextImg", "prevPts", "nextPts", "status", "err", "winSize", "maxLevel", "criteria", "flags", "minEigThreshold"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "cv::Size", "int", "cv::TermCriteria", "int", "double"]), _)]),
	void cv_calcOpticalFlowPyrLK_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_Size_int_TermCriteria_int_double(const cv::_InputArray* prevImg, const cv::_InputArray* nextImg, const cv::_InputArray* prevPts, const cv::_InputOutputArray* nextPts, const cv::_OutputArray* status, const cv::_OutputArray* err, cv::Size* winSize, int maxLevel, cv::TermCriteria* criteria, int flags, double minEigThreshold, ResultVoid* ocvrs_return) {
		try {
			cv::calcOpticalFlowPyrLK(*prevImg, *nextImg, *prevPts, *nextPts, *status, *err, *winSize, maxLevel, *criteria, flags, minEigThreshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::computeECC(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:279
	// ("cv::computeECC", vec![(pred!(mut, ["templateImage", "inputImage"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_computeECC_const__InputArrayR_const__InputArrayR(const cv::_InputArray* templateImage, const cv::_InputArray* inputImage, Result<double>* ocvrs_return) {
		try {
			double ret = cv::computeECC(*templateImage, *inputImage);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// computeECC(InputArray, InputArray, InputArray)(InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:279
	// ("cv::computeECC", vec![(pred!(mut, ["templateImage", "inputImage", "inputMask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_computeECC_const__InputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputArray* templateImage, const cv::_InputArray* inputImage, const cv::_InputArray* inputMask, Result<double>* ocvrs_return) {
		try {
			double ret = cv::computeECC(*templateImage, *inputImage, *inputMask);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::createBackgroundSubtractorKNN() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:310
	// ("cv::createBackgroundSubtractorKNN", vec![(pred!(mut, [], []), _)]),
	void cv_createBackgroundSubtractorKNN(Result<cv::Ptr<cv::BackgroundSubtractorKNN>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::BackgroundSubtractorKNN> ret = cv::createBackgroundSubtractorKNN();
			Ok(new cv::Ptr<cv::BackgroundSubtractorKNN>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createBackgroundSubtractorKNN(int, double, bool)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:310
	// ("cv::createBackgroundSubtractorKNN", vec![(pred!(mut, ["history", "dist2Threshold", "detectShadows"], ["int", "double", "bool"]), _)]),
	void cv_createBackgroundSubtractorKNN_int_double_bool(int history, double dist2Threshold, bool detectShadows, Result<cv::Ptr<cv::BackgroundSubtractorKNN>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::BackgroundSubtractorKNN> ret = cv::createBackgroundSubtractorKNN(history, dist2Threshold, detectShadows);
			Ok(new cv::Ptr<cv::BackgroundSubtractorKNN>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::createBackgroundSubtractorMOG2() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:221
	// ("cv::createBackgroundSubtractorMOG2", vec![(pred!(mut, [], []), _)]),
	void cv_createBackgroundSubtractorMOG2(Result<cv::Ptr<cv::BackgroundSubtractorMOG2>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::BackgroundSubtractorMOG2> ret = cv::createBackgroundSubtractorMOG2();
			Ok(new cv::Ptr<cv::BackgroundSubtractorMOG2>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createBackgroundSubtractorMOG2(int, double, bool)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:221
	// ("cv::createBackgroundSubtractorMOG2", vec![(pred!(mut, ["history", "varThreshold", "detectShadows"], ["int", "double", "bool"]), _)]),
	void cv_createBackgroundSubtractorMOG2_int_double_bool(int history, double varThreshold, bool detectShadows, Result<cv::Ptr<cv::BackgroundSubtractorMOG2>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::BackgroundSubtractorMOG2> ret = cv::createBackgroundSubtractorMOG2(history, varThreshold, detectShadows);
			Ok(new cv::Ptr<cv::BackgroundSubtractorMOG2>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// estimateRigidTransform(InputArray, InputArray, bool)(InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:258
	// ("cv::estimateRigidTransform", vec![(pred!(mut, ["src", "dst", "fullAffine"], ["const cv::_InputArray*", "const cv::_InputArray*", "bool"]), _)]),
	void cv_estimateRigidTransform_const__InputArrayR_const__InputArrayR_bool(const cv::_InputArray* src, const cv::_InputArray* dst, bool fullAffine, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::estimateRigidTransform(*src, *dst, fullAffine);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::findTransformECC(InputArray, InputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:343
	// ("cv::findTransformECC", vec![(pred!(mut, ["templateImage", "inputImage", "warpMatrix"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*"]), _)]),
	void cv_findTransformECC_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR(const cv::_InputArray* templateImage, const cv::_InputArray* inputImage, const cv::_InputOutputArray* warpMatrix, Result<double>* ocvrs_return) {
		try {
			double ret = cv::findTransformECC(*templateImage, *inputImage, *warpMatrix);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// findTransformECC(InputArray, InputArray, InputOutputArray, int, TermCriteria, InputArray)(InputArray, InputArray, InputOutputArray, Primitive, SimpleClass, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:343
	// ("cv::findTransformECC", vec![(pred!(mut, ["templateImage", "inputImage", "warpMatrix", "motionType", "criteria", "inputMask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "int", "cv::TermCriteria", "const cv::_InputArray*"]), _)]),
	void cv_findTransformECC_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_int_TermCriteria_const__InputArrayR(const cv::_InputArray* templateImage, const cv::_InputArray* inputImage, const cv::_InputOutputArray* warpMatrix, int motionType, cv::TermCriteria* criteria, const cv::_InputArray* inputMask, Result<double>* ocvrs_return) {
		try {
			double ret = cv::findTransformECC(*templateImage, *inputImage, *warpMatrix, motionType, *criteria, *inputMask);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// findTransformECC(InputArray, InputArray, InputOutputArray, int, TermCriteria, InputArray, int)(InputArray, InputArray, InputOutputArray, Primitive, SimpleClass, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:336
	// ("cv::findTransformECC", vec![(pred!(mut, ["templateImage", "inputImage", "warpMatrix", "motionType", "criteria", "inputMask", "gaussFiltSize"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "int", "cv::TermCriteria", "const cv::_InputArray*", "int"]), _)]),
	void cv_findTransformECC_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_int_TermCriteria_const__InputArrayR_int(const cv::_InputArray* templateImage, const cv::_InputArray* inputImage, const cv::_InputOutputArray* warpMatrix, int motionType, cv::TermCriteria* criteria, const cv::_InputArray* inputMask, int gaussFiltSize, Result<double>* ocvrs_return) {
		try {
			double ret = cv::findTransformECC(*templateImage, *inputImage, *warpMatrix, motionType, *criteria, *inputMask, gaussFiltSize);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// meanShift(InputArray, Rect &, TermCriteria)(InputArray, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:104
	// ("cv::meanShift", vec![(pred!(mut, ["probImage", "window", "criteria"], ["const cv::_InputArray*", "cv::Rect*", "cv::TermCriteria"]), _)]),
	void cv_meanShift_const__InputArrayR_RectR_TermCriteria(const cv::_InputArray* probImage, cv::Rect* window, cv::TermCriteria* criteria, Result<int>* ocvrs_return) {
		try {
			int ret = cv::meanShift(*probImage, *window, *criteria);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readOpticalFlow(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:421
	// ("cv::readOpticalFlow", vec![(pred!(mut, ["path"], ["const cv::String*"]), _)]),
	void cv_readOpticalFlow_const_StringR(const char* path, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::readOpticalFlow(std::string(path));
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// writeOpticalFlow(const String &, InputArray)(InString, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:431
	// ("cv::writeOpticalFlow", vec![(pred!(mut, ["path", "flow"], ["const cv::String*", "const cv::_InputArray*"]), _)]),
	void cv_writeOpticalFlow_const_StringR_const__InputArrayR(const char* path, const cv::_InputArray* flow, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::writeOpticalFlow(std::string(path), *flow);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// apply(InputArray, OutputArray, double)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:72
	// ("cv::BackgroundSubtractor::apply", vec![(pred!(mut, ["image", "fgmask", "learningRate"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double"]), _)]),
	void cv_BackgroundSubtractor_apply_const__InputArrayR_const__OutputArrayR_double(cv::BackgroundSubtractor* instance, const cv::_InputArray* image, const cv::_OutputArray* fgmask, double learningRate, ResultVoid* ocvrs_return) {
		try {
			instance->apply(*image, *fgmask, learningRate);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::BackgroundSubtractor::apply(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:72
	// ("cv::BackgroundSubtractor::apply", vec![(pred!(mut, ["image", "fgmask"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_BackgroundSubtractor_apply_const__InputArrayR_const__OutputArrayR(cv::BackgroundSubtractor* instance, const cv::_InputArray* image, const cv::_OutputArray* fgmask, ResultVoid* ocvrs_return) {
		try {
			instance->apply(*image, *fgmask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getBackgroundImage(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:81
	// ("cv::BackgroundSubtractor::getBackgroundImage", vec![(pred!(const, ["backgroundImage"], ["const cv::_OutputArray*"]), _)]),
	void cv_BackgroundSubtractor_getBackgroundImage_const_const__OutputArrayR(const cv::BackgroundSubtractor* instance, const cv::_OutputArray* backgroundImage, ResultVoid* ocvrs_return) {
		try {
			instance->getBackgroundImage(*backgroundImage);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::BackgroundSubtractor::to_BackgroundSubtractorKNN() generated
	// ("cv::BackgroundSubtractor::to_BackgroundSubtractorKNN", vec![(pred!(mut, [], []), _)]),
	cv::BackgroundSubtractorKNN* cv_BackgroundSubtractor_to_BackgroundSubtractorKNN(cv::BackgroundSubtractor* instance) {
			return dynamic_cast<cv::BackgroundSubtractorKNN*>(instance);
	}

	// cv::BackgroundSubtractor::to_BackgroundSubtractorMOG2() generated
	// ("cv::BackgroundSubtractor::to_BackgroundSubtractorMOG2", vec![(pred!(mut, [], []), _)]),
	cv::BackgroundSubtractorMOG2* cv_BackgroundSubtractor_to_BackgroundSubtractorMOG2(cv::BackgroundSubtractor* instance) {
			return dynamic_cast<cv::BackgroundSubtractorMOG2*>(instance);
	}

	// cv::BackgroundSubtractor::to_Algorithm() generated
	// ("cv::BackgroundSubtractor::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_BackgroundSubtractor_to_Algorithm(cv::BackgroundSubtractor* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::BackgroundSubtractor::delete() generated
	// ("cv::BackgroundSubtractor::delete", vec![(pred!(mut, [], []), _)]),
	void cv_BackgroundSubtractor_delete(cv::BackgroundSubtractor* instance) {
			delete instance;
	}

	// getHistory()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:234
	// ("cv::BackgroundSubtractorKNN::getHistory", vec![(pred!(const, [], []), _)]),
	void cv_BackgroundSubtractorKNN_getHistory_const(const cv::BackgroundSubtractorKNN* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getHistory();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setHistory(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:237
	// ("cv::BackgroundSubtractorKNN::setHistory", vec![(pred!(mut, ["history"], ["int"]), _)]),
	void cv_BackgroundSubtractorKNN_setHistory_int(cv::BackgroundSubtractorKNN* instance, int history, ResultVoid* ocvrs_return) {
		try {
			instance->setHistory(history);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNSamples()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:241
	// ("cv::BackgroundSubtractorKNN::getNSamples", vec![(pred!(const, [], []), _)]),
	void cv_BackgroundSubtractorKNN_getNSamples_const(const cv::BackgroundSubtractorKNN* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNSamples();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNSamples(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:246
	// ("cv::BackgroundSubtractorKNN::setNSamples", vec![(pred!(mut, ["_nN"], ["int"]), _)]),
	void cv_BackgroundSubtractorKNN_setNSamples_int(cv::BackgroundSubtractorKNN* instance, int _nN, ResultVoid* ocvrs_return) {
		try {
			instance->setNSamples(_nN);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDist2Threshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:253
	// ("cv::BackgroundSubtractorKNN::getDist2Threshold", vec![(pred!(const, [], []), _)]),
	void cv_BackgroundSubtractorKNN_getDist2Threshold_const(const cv::BackgroundSubtractorKNN* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getDist2Threshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDist2Threshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:256
	// ("cv::BackgroundSubtractorKNN::setDist2Threshold", vec![(pred!(mut, ["_dist2Threshold"], ["double"]), _)]),
	void cv_BackgroundSubtractorKNN_setDist2Threshold_double(cv::BackgroundSubtractorKNN* instance, double _dist2Threshold, ResultVoid* ocvrs_return) {
		try {
			instance->setDist2Threshold(_dist2Threshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getkNNSamples()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:263
	// ("cv::BackgroundSubtractorKNN::getkNNSamples", vec![(pred!(const, [], []), _)]),
	void cv_BackgroundSubtractorKNN_getkNNSamples_const(const cv::BackgroundSubtractorKNN* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getkNNSamples();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setkNNSamples(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:266
	// ("cv::BackgroundSubtractorKNN::setkNNSamples", vec![(pred!(mut, ["_nkNN"], ["int"]), _)]),
	void cv_BackgroundSubtractorKNN_setkNNSamples_int(cv::BackgroundSubtractorKNN* instance, int _nkNN, ResultVoid* ocvrs_return) {
		try {
			instance->setkNNSamples(_nkNN);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDetectShadows()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:273
	// ("cv::BackgroundSubtractorKNN::getDetectShadows", vec![(pred!(const, [], []), _)]),
	void cv_BackgroundSubtractorKNN_getDetectShadows_const(const cv::BackgroundSubtractorKNN* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getDetectShadows();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDetectShadows(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:276
	// ("cv::BackgroundSubtractorKNN::setDetectShadows", vec![(pred!(mut, ["detectShadows"], ["bool"]), _)]),
	void cv_BackgroundSubtractorKNN_setDetectShadows_bool(cv::BackgroundSubtractorKNN* instance, bool detectShadows, ResultVoid* ocvrs_return) {
		try {
			instance->setDetectShadows(detectShadows);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getShadowValue()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:283
	// ("cv::BackgroundSubtractorKNN::getShadowValue", vec![(pred!(const, [], []), _)]),
	void cv_BackgroundSubtractorKNN_getShadowValue_const(const cv::BackgroundSubtractorKNN* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getShadowValue();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setShadowValue(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:286
	// ("cv::BackgroundSubtractorKNN::setShadowValue", vec![(pred!(mut, ["value"], ["int"]), _)]),
	void cv_BackgroundSubtractorKNN_setShadowValue_int(cv::BackgroundSubtractorKNN* instance, int value, ResultVoid* ocvrs_return) {
		try {
			instance->setShadowValue(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getShadowThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:295
	// ("cv::BackgroundSubtractorKNN::getShadowThreshold", vec![(pred!(const, [], []), _)]),
	void cv_BackgroundSubtractorKNN_getShadowThreshold_const(const cv::BackgroundSubtractorKNN* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getShadowThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setShadowThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:298
	// ("cv::BackgroundSubtractorKNN::setShadowThreshold", vec![(pred!(mut, ["threshold"], ["double"]), _)]),
	void cv_BackgroundSubtractorKNN_setShadowThreshold_double(cv::BackgroundSubtractorKNN* instance, double threshold, ResultVoid* ocvrs_return) {
		try {
			instance->setShadowThreshold(threshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::BackgroundSubtractorKNN::to_Algorithm() generated
	// ("cv::BackgroundSubtractorKNN::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_BackgroundSubtractorKNN_to_Algorithm(cv::BackgroundSubtractorKNN* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::BackgroundSubtractorKNN::to_BackgroundSubtractor() generated
	// ("cv::BackgroundSubtractorKNN::to_BackgroundSubtractor", vec![(pred!(mut, [], []), _)]),
	cv::BackgroundSubtractor* cv_BackgroundSubtractorKNN_to_BackgroundSubtractor(cv::BackgroundSubtractorKNN* instance) {
			return dynamic_cast<cv::BackgroundSubtractor*>(instance);
	}

	// cv::BackgroundSubtractorKNN::delete() generated
	// ("cv::BackgroundSubtractorKNN::delete", vec![(pred!(mut, [], []), _)]),
	void cv_BackgroundSubtractorKNN_delete(cv::BackgroundSubtractorKNN* instance) {
			delete instance;
	}

	// getHistory()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:95
	// ("cv::BackgroundSubtractorMOG2::getHistory", vec![(pred!(const, [], []), _)]),
	void cv_BackgroundSubtractorMOG2_getHistory_const(const cv::BackgroundSubtractorMOG2* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getHistory();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setHistory(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:98
	// ("cv::BackgroundSubtractorMOG2::setHistory", vec![(pred!(mut, ["history"], ["int"]), _)]),
	void cv_BackgroundSubtractorMOG2_setHistory_int(cv::BackgroundSubtractorMOG2* instance, int history, ResultVoid* ocvrs_return) {
		try {
			instance->setHistory(history);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNMixtures()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:102
	// ("cv::BackgroundSubtractorMOG2::getNMixtures", vec![(pred!(const, [], []), _)]),
	void cv_BackgroundSubtractorMOG2_getNMixtures_const(const cv::BackgroundSubtractorMOG2* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNMixtures();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNMixtures(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:107
	// ("cv::BackgroundSubtractorMOG2::setNMixtures", vec![(pred!(mut, ["nmixtures"], ["int"]), _)]),
	void cv_BackgroundSubtractorMOG2_setNMixtures_int(cv::BackgroundSubtractorMOG2* instance, int nmixtures, ResultVoid* ocvrs_return) {
		try {
			instance->setNMixtures(nmixtures);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getBackgroundRatio()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:115
	// ("cv::BackgroundSubtractorMOG2::getBackgroundRatio", vec![(pred!(const, [], []), _)]),
	void cv_BackgroundSubtractorMOG2_getBackgroundRatio_const(const cv::BackgroundSubtractorMOG2* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getBackgroundRatio();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setBackgroundRatio(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:118
	// ("cv::BackgroundSubtractorMOG2::setBackgroundRatio", vec![(pred!(mut, ["ratio"], ["double"]), _)]),
	void cv_BackgroundSubtractorMOG2_setBackgroundRatio_double(cv::BackgroundSubtractorMOG2* instance, double ratio, ResultVoid* ocvrs_return) {
		try {
			instance->setBackgroundRatio(ratio);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getVarThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:125
	// ("cv::BackgroundSubtractorMOG2::getVarThreshold", vec![(pred!(const, [], []), _)]),
	void cv_BackgroundSubtractorMOG2_getVarThreshold_const(const cv::BackgroundSubtractorMOG2* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getVarThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setVarThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:128
	// ("cv::BackgroundSubtractorMOG2::setVarThreshold", vec![(pred!(mut, ["varThreshold"], ["double"]), _)]),
	void cv_BackgroundSubtractorMOG2_setVarThreshold_double(cv::BackgroundSubtractorMOG2* instance, double varThreshold, ResultVoid* ocvrs_return) {
		try {
			instance->setVarThreshold(varThreshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getVarThresholdGen()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:138
	// ("cv::BackgroundSubtractorMOG2::getVarThresholdGen", vec![(pred!(const, [], []), _)]),
	void cv_BackgroundSubtractorMOG2_getVarThresholdGen_const(const cv::BackgroundSubtractorMOG2* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getVarThresholdGen();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setVarThresholdGen(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:141
	// ("cv::BackgroundSubtractorMOG2::setVarThresholdGen", vec![(pred!(mut, ["varThresholdGen"], ["double"]), _)]),
	void cv_BackgroundSubtractorMOG2_setVarThresholdGen_double(cv::BackgroundSubtractorMOG2* instance, double varThresholdGen, ResultVoid* ocvrs_return) {
		try {
			instance->setVarThresholdGen(varThresholdGen);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getVarInit()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:145
	// ("cv::BackgroundSubtractorMOG2::getVarInit", vec![(pred!(const, [], []), _)]),
	void cv_BackgroundSubtractorMOG2_getVarInit_const(const cv::BackgroundSubtractorMOG2* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getVarInit();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setVarInit(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:148
	// ("cv::BackgroundSubtractorMOG2::setVarInit", vec![(pred!(mut, ["varInit"], ["double"]), _)]),
	void cv_BackgroundSubtractorMOG2_setVarInit_double(cv::BackgroundSubtractorMOG2* instance, double varInit, ResultVoid* ocvrs_return) {
		try {
			instance->setVarInit(varInit);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getVarMin()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:150
	// ("cv::BackgroundSubtractorMOG2::getVarMin", vec![(pred!(const, [], []), _)]),
	void cv_BackgroundSubtractorMOG2_getVarMin_const(const cv::BackgroundSubtractorMOG2* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getVarMin();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setVarMin(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:151
	// ("cv::BackgroundSubtractorMOG2::setVarMin", vec![(pred!(mut, ["varMin"], ["double"]), _)]),
	void cv_BackgroundSubtractorMOG2_setVarMin_double(cv::BackgroundSubtractorMOG2* instance, double varMin, ResultVoid* ocvrs_return) {
		try {
			instance->setVarMin(varMin);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getVarMax()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:153
	// ("cv::BackgroundSubtractorMOG2::getVarMax", vec![(pred!(const, [], []), _)]),
	void cv_BackgroundSubtractorMOG2_getVarMax_const(const cv::BackgroundSubtractorMOG2* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getVarMax();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setVarMax(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:154
	// ("cv::BackgroundSubtractorMOG2::setVarMax", vec![(pred!(mut, ["varMax"], ["double"]), _)]),
	void cv_BackgroundSubtractorMOG2_setVarMax_double(cv::BackgroundSubtractorMOG2* instance, double varMax, ResultVoid* ocvrs_return) {
		try {
			instance->setVarMax(varMax);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getComplexityReductionThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:162
	// ("cv::BackgroundSubtractorMOG2::getComplexityReductionThreshold", vec![(pred!(const, [], []), _)]),
	void cv_BackgroundSubtractorMOG2_getComplexityReductionThreshold_const(const cv::BackgroundSubtractorMOG2* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getComplexityReductionThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setComplexityReductionThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:165
	// ("cv::BackgroundSubtractorMOG2::setComplexityReductionThreshold", vec![(pred!(mut, ["ct"], ["double"]), _)]),
	void cv_BackgroundSubtractorMOG2_setComplexityReductionThreshold_double(cv::BackgroundSubtractorMOG2* instance, double ct, ResultVoid* ocvrs_return) {
		try {
			instance->setComplexityReductionThreshold(ct);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDetectShadows()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:172
	// ("cv::BackgroundSubtractorMOG2::getDetectShadows", vec![(pred!(const, [], []), _)]),
	void cv_BackgroundSubtractorMOG2_getDetectShadows_const(const cv::BackgroundSubtractorMOG2* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getDetectShadows();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDetectShadows(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:175
	// ("cv::BackgroundSubtractorMOG2::setDetectShadows", vec![(pred!(mut, ["detectShadows"], ["bool"]), _)]),
	void cv_BackgroundSubtractorMOG2_setDetectShadows_bool(cv::BackgroundSubtractorMOG2* instance, bool detectShadows, ResultVoid* ocvrs_return) {
		try {
			instance->setDetectShadows(detectShadows);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getShadowValue()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:182
	// ("cv::BackgroundSubtractorMOG2::getShadowValue", vec![(pred!(const, [], []), _)]),
	void cv_BackgroundSubtractorMOG2_getShadowValue_const(const cv::BackgroundSubtractorMOG2* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getShadowValue();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setShadowValue(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:185
	// ("cv::BackgroundSubtractorMOG2::setShadowValue", vec![(pred!(mut, ["value"], ["int"]), _)]),
	void cv_BackgroundSubtractorMOG2_setShadowValue_int(cv::BackgroundSubtractorMOG2* instance, int value, ResultVoid* ocvrs_return) {
		try {
			instance->setShadowValue(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getShadowThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:194
	// ("cv::BackgroundSubtractorMOG2::getShadowThreshold", vec![(pred!(const, [], []), _)]),
	void cv_BackgroundSubtractorMOG2_getShadowThreshold_const(const cv::BackgroundSubtractorMOG2* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getShadowThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setShadowThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:197
	// ("cv::BackgroundSubtractorMOG2::setShadowThreshold", vec![(pred!(mut, ["threshold"], ["double"]), _)]),
	void cv_BackgroundSubtractorMOG2_setShadowThreshold_double(cv::BackgroundSubtractorMOG2* instance, double threshold, ResultVoid* ocvrs_return) {
		try {
			instance->setShadowThreshold(threshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// apply(InputArray, OutputArray, double)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:208
	// ("cv::BackgroundSubtractorMOG2::apply", vec![(pred!(mut, ["image", "fgmask", "learningRate"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double"]), _)]),
	void cv_BackgroundSubtractorMOG2_apply_const__InputArrayR_const__OutputArrayR_double(cv::BackgroundSubtractorMOG2* instance, const cv::_InputArray* image, const cv::_OutputArray* fgmask, double learningRate, ResultVoid* ocvrs_return) {
		try {
			instance->apply(*image, *fgmask, learningRate);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::BackgroundSubtractorMOG2::apply(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/background_segm.hpp:208
	// ("cv::BackgroundSubtractorMOG2::apply", vec![(pred!(mut, ["image", "fgmask"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_BackgroundSubtractorMOG2_apply_const__InputArrayR_const__OutputArrayR(cv::BackgroundSubtractorMOG2* instance, const cv::_InputArray* image, const cv::_OutputArray* fgmask, ResultVoid* ocvrs_return) {
		try {
			instance->apply(*image, *fgmask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::BackgroundSubtractorMOG2::to_Algorithm() generated
	// ("cv::BackgroundSubtractorMOG2::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_BackgroundSubtractorMOG2_to_Algorithm(cv::BackgroundSubtractorMOG2* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::BackgroundSubtractorMOG2::to_BackgroundSubtractor() generated
	// ("cv::BackgroundSubtractorMOG2::to_BackgroundSubtractor", vec![(pred!(mut, [], []), _)]),
	cv::BackgroundSubtractor* cv_BackgroundSubtractorMOG2_to_BackgroundSubtractor(cv::BackgroundSubtractorMOG2* instance) {
			return dynamic_cast<cv::BackgroundSubtractor*>(instance);
	}

	// cv::BackgroundSubtractorMOG2::delete() generated
	// ("cv::BackgroundSubtractorMOG2::delete", vec![(pred!(mut, [], []), _)]),
	void cv_BackgroundSubtractorMOG2_delete(cv::BackgroundSubtractorMOG2* instance) {
			delete instance;
	}

	// getFinestScale()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:603
	// ("cv::DISOpticalFlow::getFinestScale", vec![(pred!(const, [], []), _)]),
	void cv_DISOpticalFlow_getFinestScale_const(const cv::DISOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getFinestScale();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setFinestScale(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:605
	// ("cv::DISOpticalFlow::setFinestScale", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_DISOpticalFlow_setFinestScale_int(cv::DISOpticalFlow* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setFinestScale(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPatchSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:610
	// ("cv::DISOpticalFlow::getPatchSize", vec![(pred!(const, [], []), _)]),
	void cv_DISOpticalFlow_getPatchSize_const(const cv::DISOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getPatchSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setPatchSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:612
	// ("cv::DISOpticalFlow::setPatchSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_DISOpticalFlow_setPatchSize_int(cv::DISOpticalFlow* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setPatchSize(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPatchStride()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:617
	// ("cv::DISOpticalFlow::getPatchStride", vec![(pred!(const, [], []), _)]),
	void cv_DISOpticalFlow_getPatchStride_const(const cv::DISOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getPatchStride();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setPatchStride(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:619
	// ("cv::DISOpticalFlow::setPatchStride", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_DISOpticalFlow_setPatchStride_int(cv::DISOpticalFlow* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setPatchStride(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getGradientDescentIterations()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:624
	// ("cv::DISOpticalFlow::getGradientDescentIterations", vec![(pred!(const, [], []), _)]),
	void cv_DISOpticalFlow_getGradientDescentIterations_const(const cv::DISOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getGradientDescentIterations();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setGradientDescentIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:626
	// ("cv::DISOpticalFlow::setGradientDescentIterations", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_DISOpticalFlow_setGradientDescentIterations_int(cv::DISOpticalFlow* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setGradientDescentIterations(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getVariationalRefinementIterations()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:632
	// ("cv::DISOpticalFlow::getVariationalRefinementIterations", vec![(pred!(const, [], []), _)]),
	void cv_DISOpticalFlow_getVariationalRefinementIterations_const(const cv::DISOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getVariationalRefinementIterations();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setVariationalRefinementIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:634
	// ("cv::DISOpticalFlow::setVariationalRefinementIterations", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_DISOpticalFlow_setVariationalRefinementIterations_int(cv::DISOpticalFlow* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setVariationalRefinementIterations(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getVariationalRefinementAlpha()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:638
	// ("cv::DISOpticalFlow::getVariationalRefinementAlpha", vec![(pred!(const, [], []), _)]),
	void cv_DISOpticalFlow_getVariationalRefinementAlpha_const(const cv::DISOpticalFlow* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getVariationalRefinementAlpha();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setVariationalRefinementAlpha(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:640
	// ("cv::DISOpticalFlow::setVariationalRefinementAlpha", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_DISOpticalFlow_setVariationalRefinementAlpha_float(cv::DISOpticalFlow* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setVariationalRefinementAlpha(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getVariationalRefinementDelta()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:644
	// ("cv::DISOpticalFlow::getVariationalRefinementDelta", vec![(pred!(const, [], []), _)]),
	void cv_DISOpticalFlow_getVariationalRefinementDelta_const(const cv::DISOpticalFlow* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getVariationalRefinementDelta();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setVariationalRefinementDelta(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:646
	// ("cv::DISOpticalFlow::setVariationalRefinementDelta", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_DISOpticalFlow_setVariationalRefinementDelta_float(cv::DISOpticalFlow* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setVariationalRefinementDelta(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getVariationalRefinementGamma()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:650
	// ("cv::DISOpticalFlow::getVariationalRefinementGamma", vec![(pred!(const, [], []), _)]),
	void cv_DISOpticalFlow_getVariationalRefinementGamma_const(const cv::DISOpticalFlow* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getVariationalRefinementGamma();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setVariationalRefinementGamma(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:652
	// ("cv::DISOpticalFlow::setVariationalRefinementGamma", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_DISOpticalFlow_setVariationalRefinementGamma_float(cv::DISOpticalFlow* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setVariationalRefinementGamma(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getVariationalRefinementEpsilon()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:656
	// ("cv::DISOpticalFlow::getVariationalRefinementEpsilon", vec![(pred!(const, [], []), _)]),
	void cv_DISOpticalFlow_getVariationalRefinementEpsilon_const(const cv::DISOpticalFlow* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getVariationalRefinementEpsilon();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setVariationalRefinementEpsilon(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:658
	// ("cv::DISOpticalFlow::setVariationalRefinementEpsilon", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_DISOpticalFlow_setVariationalRefinementEpsilon_float(cv::DISOpticalFlow* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setVariationalRefinementEpsilon(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getUseMeanNormalization()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:666
	// ("cv::DISOpticalFlow::getUseMeanNormalization", vec![(pred!(const, [], []), _)]),
	void cv_DISOpticalFlow_getUseMeanNormalization_const(const cv::DISOpticalFlow* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUseMeanNormalization();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setUseMeanNormalization(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:668
	// ("cv::DISOpticalFlow::setUseMeanNormalization", vec![(pred!(mut, ["val"], ["bool"]), _)]),
	void cv_DISOpticalFlow_setUseMeanNormalization_bool(cv::DISOpticalFlow* instance, bool val, ResultVoid* ocvrs_return) {
		try {
			instance->setUseMeanNormalization(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getUseSpatialPropagation()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:675
	// ("cv::DISOpticalFlow::getUseSpatialPropagation", vec![(pred!(const, [], []), _)]),
	void cv_DISOpticalFlow_getUseSpatialPropagation_const(const cv::DISOpticalFlow* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUseSpatialPropagation();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setUseSpatialPropagation(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:677
	// ("cv::DISOpticalFlow::setUseSpatialPropagation", vec![(pred!(mut, ["val"], ["bool"]), _)]),
	void cv_DISOpticalFlow_setUseSpatialPropagation_bool(cv::DISOpticalFlow* instance, bool val, ResultVoid* ocvrs_return) {
		try {
			instance->setUseSpatialPropagation(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:683
	// ("cv::DISOpticalFlow::create", vec![(pred!(mut, ["preset"], ["int"]), _)]),
	void cv_DISOpticalFlow_create_int(int preset, Result<cv::Ptr<cv::DISOpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::DISOpticalFlow> ret = cv::DISOpticalFlow::create(preset);
			Ok(new cv::Ptr<cv::DISOpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::DISOpticalFlow::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:683
	// ("cv::DISOpticalFlow::create", vec![(pred!(mut, [], []), _)]),
	void cv_DISOpticalFlow_create(Result<cv::Ptr<cv::DISOpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::DISOpticalFlow> ret = cv::DISOpticalFlow::create();
			Ok(new cv::Ptr<cv::DISOpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::DISOpticalFlow::to_Algorithm() generated
	// ("cv::DISOpticalFlow::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_DISOpticalFlow_to_Algorithm(cv::DISOpticalFlow* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::DISOpticalFlow::to_DenseOpticalFlow() generated
	// ("cv::DISOpticalFlow::to_DenseOpticalFlow", vec![(pred!(mut, [], []), _)]),
	cv::DenseOpticalFlow* cv_DISOpticalFlow_to_DenseOpticalFlow(cv::DISOpticalFlow* instance) {
			return dynamic_cast<cv::DenseOpticalFlow*>(instance);
	}

	// cv::DISOpticalFlow::delete() generated
	// ("cv::DISOpticalFlow::delete", vec![(pred!(mut, [], []), _)]),
	void cv_DISOpticalFlow_delete(cv::DISOpticalFlow* instance) {
			delete instance;
	}

	// calc(InputArray, InputArray, InputOutputArray)(InputArray, InputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:445
	// ("cv::DenseOpticalFlow::calc", vec![(pred!(mut, ["I0", "I1", "flow"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*"]), _)]),
	void cv_DenseOpticalFlow_calc_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR(cv::DenseOpticalFlow* instance, const cv::_InputArray* I0, const cv::_InputArray* I1, const cv::_InputOutputArray* flow, ResultVoid* ocvrs_return) {
		try {
			instance->calc(*I0, *I1, *flow);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// collectGarbage()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:448
	// ("cv::DenseOpticalFlow::collectGarbage", vec![(pred!(mut, [], []), _)]),
	void cv_DenseOpticalFlow_collectGarbage(cv::DenseOpticalFlow* instance, ResultVoid* ocvrs_return) {
		try {
			instance->collectGarbage();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::DenseOpticalFlow::to_DISOpticalFlow() generated
	// ("cv::DenseOpticalFlow::to_DISOpticalFlow", vec![(pred!(mut, [], []), _)]),
	cv::DISOpticalFlow* cv_DenseOpticalFlow_to_DISOpticalFlow(cv::DenseOpticalFlow* instance) {
			return dynamic_cast<cv::DISOpticalFlow*>(instance);
	}

	// cv::DenseOpticalFlow::to_FarnebackOpticalFlow() generated
	// ("cv::DenseOpticalFlow::to_FarnebackOpticalFlow", vec![(pred!(mut, [], []), _)]),
	cv::FarnebackOpticalFlow* cv_DenseOpticalFlow_to_FarnebackOpticalFlow(cv::DenseOpticalFlow* instance) {
			return dynamic_cast<cv::FarnebackOpticalFlow*>(instance);
	}

	// cv::DenseOpticalFlow::to_VariationalRefinement() generated
	// ("cv::DenseOpticalFlow::to_VariationalRefinement", vec![(pred!(mut, [], []), _)]),
	cv::VariationalRefinement* cv_DenseOpticalFlow_to_VariationalRefinement(cv::DenseOpticalFlow* instance) {
			return dynamic_cast<cv::VariationalRefinement*>(instance);
	}

	// cv::DenseOpticalFlow::to_Algorithm() generated
	// ("cv::DenseOpticalFlow::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_DenseOpticalFlow_to_Algorithm(cv::DenseOpticalFlow* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::DenseOpticalFlow::delete() generated
	// ("cv::DenseOpticalFlow::delete", vec![(pred!(mut, [], []), _)]),
	void cv_DenseOpticalFlow_delete(cv::DenseOpticalFlow* instance) {
			delete instance;
	}

	// getNumLevels()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:478
	// ("cv::FarnebackOpticalFlow::getNumLevels", vec![(pred!(const, [], []), _)]),
	void cv_FarnebackOpticalFlow_getNumLevels_const(const cv::FarnebackOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNumLevels();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNumLevels(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:479
	// ("cv::FarnebackOpticalFlow::setNumLevels", vec![(pred!(mut, ["numLevels"], ["int"]), _)]),
	void cv_FarnebackOpticalFlow_setNumLevels_int(cv::FarnebackOpticalFlow* instance, int numLevels, ResultVoid* ocvrs_return) {
		try {
			instance->setNumLevels(numLevels);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPyrScale()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:481
	// ("cv::FarnebackOpticalFlow::getPyrScale", vec![(pred!(const, [], []), _)]),
	void cv_FarnebackOpticalFlow_getPyrScale_const(const cv::FarnebackOpticalFlow* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getPyrScale();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setPyrScale(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:482
	// ("cv::FarnebackOpticalFlow::setPyrScale", vec![(pred!(mut, ["pyrScale"], ["double"]), _)]),
	void cv_FarnebackOpticalFlow_setPyrScale_double(cv::FarnebackOpticalFlow* instance, double pyrScale, ResultVoid* ocvrs_return) {
		try {
			instance->setPyrScale(pyrScale);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getFastPyramids()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:484
	// ("cv::FarnebackOpticalFlow::getFastPyramids", vec![(pred!(const, [], []), _)]),
	void cv_FarnebackOpticalFlow_getFastPyramids_const(const cv::FarnebackOpticalFlow* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getFastPyramids();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setFastPyramids(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:485
	// ("cv::FarnebackOpticalFlow::setFastPyramids", vec![(pred!(mut, ["fastPyramids"], ["bool"]), _)]),
	void cv_FarnebackOpticalFlow_setFastPyramids_bool(cv::FarnebackOpticalFlow* instance, bool fastPyramids, ResultVoid* ocvrs_return) {
		try {
			instance->setFastPyramids(fastPyramids);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getWinSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:487
	// ("cv::FarnebackOpticalFlow::getWinSize", vec![(pred!(const, [], []), _)]),
	void cv_FarnebackOpticalFlow_getWinSize_const(const cv::FarnebackOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getWinSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWinSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:488
	// ("cv::FarnebackOpticalFlow::setWinSize", vec![(pred!(mut, ["winSize"], ["int"]), _)]),
	void cv_FarnebackOpticalFlow_setWinSize_int(cv::FarnebackOpticalFlow* instance, int winSize, ResultVoid* ocvrs_return) {
		try {
			instance->setWinSize(winSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNumIters()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:490
	// ("cv::FarnebackOpticalFlow::getNumIters", vec![(pred!(const, [], []), _)]),
	void cv_FarnebackOpticalFlow_getNumIters_const(const cv::FarnebackOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNumIters();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNumIters(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:491
	// ("cv::FarnebackOpticalFlow::setNumIters", vec![(pred!(mut, ["numIters"], ["int"]), _)]),
	void cv_FarnebackOpticalFlow_setNumIters_int(cv::FarnebackOpticalFlow* instance, int numIters, ResultVoid* ocvrs_return) {
		try {
			instance->setNumIters(numIters);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPolyN()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:493
	// ("cv::FarnebackOpticalFlow::getPolyN", vec![(pred!(const, [], []), _)]),
	void cv_FarnebackOpticalFlow_getPolyN_const(const cv::FarnebackOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getPolyN();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setPolyN(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:494
	// ("cv::FarnebackOpticalFlow::setPolyN", vec![(pred!(mut, ["polyN"], ["int"]), _)]),
	void cv_FarnebackOpticalFlow_setPolyN_int(cv::FarnebackOpticalFlow* instance, int polyN, ResultVoid* ocvrs_return) {
		try {
			instance->setPolyN(polyN);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPolySigma()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:496
	// ("cv::FarnebackOpticalFlow::getPolySigma", vec![(pred!(const, [], []), _)]),
	void cv_FarnebackOpticalFlow_getPolySigma_const(const cv::FarnebackOpticalFlow* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getPolySigma();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setPolySigma(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:497
	// ("cv::FarnebackOpticalFlow::setPolySigma", vec![(pred!(mut, ["polySigma"], ["double"]), _)]),
	void cv_FarnebackOpticalFlow_setPolySigma_double(cv::FarnebackOpticalFlow* instance, double polySigma, ResultVoid* ocvrs_return) {
		try {
			instance->setPolySigma(polySigma);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getFlags()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:499
	// ("cv::FarnebackOpticalFlow::getFlags", vec![(pred!(const, [], []), _)]),
	void cv_FarnebackOpticalFlow_getFlags_const(const cv::FarnebackOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getFlags();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setFlags(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:500
	// ("cv::FarnebackOpticalFlow::setFlags", vec![(pred!(mut, ["flags"], ["int"]), _)]),
	void cv_FarnebackOpticalFlow_setFlags_int(cv::FarnebackOpticalFlow* instance, int flags, ResultVoid* ocvrs_return) {
		try {
			instance->setFlags(flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(int, double, bool, int, int, int, double, int)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:502
	// ("cv::FarnebackOpticalFlow::create", vec![(pred!(mut, ["numLevels", "pyrScale", "fastPyramids", "winSize", "numIters", "polyN", "polySigma", "flags"], ["int", "double", "bool", "int", "int", "int", "double", "int"]), _)]),
	void cv_FarnebackOpticalFlow_create_int_double_bool_int_int_int_double_int(int numLevels, double pyrScale, bool fastPyramids, int winSize, int numIters, int polyN, double polySigma, int flags, Result<cv::Ptr<cv::FarnebackOpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::FarnebackOpticalFlow> ret = cv::FarnebackOpticalFlow::create(numLevels, pyrScale, fastPyramids, winSize, numIters, polyN, polySigma, flags);
			Ok(new cv::Ptr<cv::FarnebackOpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::FarnebackOpticalFlow::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:502
	// ("cv::FarnebackOpticalFlow::create", vec![(pred!(mut, [], []), _)]),
	void cv_FarnebackOpticalFlow_create(Result<cv::Ptr<cv::FarnebackOpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::FarnebackOpticalFlow> ret = cv::FarnebackOpticalFlow::create();
			Ok(new cv::Ptr<cv::FarnebackOpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::FarnebackOpticalFlow::to_Algorithm() generated
	// ("cv::FarnebackOpticalFlow::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_FarnebackOpticalFlow_to_Algorithm(cv::FarnebackOpticalFlow* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::FarnebackOpticalFlow::to_DenseOpticalFlow() generated
	// ("cv::FarnebackOpticalFlow::to_DenseOpticalFlow", vec![(pred!(mut, [], []), _)]),
	cv::DenseOpticalFlow* cv_FarnebackOpticalFlow_to_DenseOpticalFlow(cv::FarnebackOpticalFlow* instance) {
			return dynamic_cast<cv::DenseOpticalFlow*>(instance);
	}

	// cv::FarnebackOpticalFlow::delete() generated
	// ("cv::FarnebackOpticalFlow::delete", vec![(pred!(mut, [], []), _)]),
	void cv_FarnebackOpticalFlow_delete(cv::FarnebackOpticalFlow* instance) {
			delete instance;
	}

	// KalmanFilter()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:363
	// ("cv::KalmanFilter::KalmanFilter", vec![(pred!(mut, [], []), _)]),
	void cv_KalmanFilter_KalmanFilter(Result<cv::KalmanFilter*>* ocvrs_return) {
		try {
			cv::KalmanFilter* ret = new cv::KalmanFilter();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// KalmanFilter(int, int, int, int)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:370
	// ("cv::KalmanFilter::KalmanFilter", vec![(pred!(mut, ["dynamParams", "measureParams", "controlParams", "type"], ["int", "int", "int", "int"]), _)]),
	void cv_KalmanFilter_KalmanFilter_int_int_int_int(int dynamParams, int measureParams, int controlParams, int type, Result<cv::KalmanFilter*>* ocvrs_return) {
		try {
			cv::KalmanFilter* ret = new cv::KalmanFilter(dynamParams, measureParams, controlParams, type);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::KalmanFilter::KalmanFilter(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:370
	// ("cv::KalmanFilter::KalmanFilter", vec![(pred!(mut, ["dynamParams", "measureParams"], ["int", "int"]), _)]),
	void cv_KalmanFilter_KalmanFilter_int_int(int dynamParams, int measureParams, Result<cv::KalmanFilter*>* ocvrs_return) {
		try {
			cv::KalmanFilter* ret = new cv::KalmanFilter(dynamParams, measureParams);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// init(int, int, int, int)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:379
	// ("cv::KalmanFilter::init", vec![(pred!(mut, ["dynamParams", "measureParams", "controlParams", "type"], ["int", "int", "int", "int"]), _)]),
	void cv_KalmanFilter_init_int_int_int_int(cv::KalmanFilter* instance, int dynamParams, int measureParams, int controlParams, int type, ResultVoid* ocvrs_return) {
		try {
			instance->init(dynamParams, measureParams, controlParams, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::KalmanFilter::init(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:379
	// ("cv::KalmanFilter::init", vec![(pred!(mut, ["dynamParams", "measureParams"], ["int", "int"]), _)]),
	void cv_KalmanFilter_init_int_int(cv::KalmanFilter* instance, int dynamParams, int measureParams, ResultVoid* ocvrs_return) {
		try {
			instance->init(dynamParams, measureParams);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// predict(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:385
	// ("cv::KalmanFilter::predict", vec![(pred!(mut, ["control"], ["const cv::Mat*"]), _)]),
	void cv_KalmanFilter_predict_const_MatR(cv::KalmanFilter* instance, const cv::Mat* control, Result<cv::Mat*>* ocvrs_return) {
		try {
			const cv::Mat ret = instance->predict(*control);
			Ok(new const cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::KalmanFilter::predict() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:385
	// ("cv::KalmanFilter::predict", vec![(pred!(mut, [], []), _)]),
	void cv_KalmanFilter_predict(cv::KalmanFilter* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			const cv::Mat ret = instance->predict();
			Ok(new const cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// correct(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:391
	// ("cv::KalmanFilter::correct", vec![(pred!(mut, ["measurement"], ["const cv::Mat*"]), _)]),
	void cv_KalmanFilter_correct_const_MatR(cv::KalmanFilter* instance, const cv::Mat* measurement, Result<cv::Mat*>* ocvrs_return) {
		try {
			const cv::Mat ret = instance->correct(*measurement);
			Ok(new const cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::KalmanFilter::statePre() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:393
	// ("cv::KalmanFilter::statePre", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_KalmanFilter_propStatePre_const(const cv::KalmanFilter* instance) {
			cv::Mat ret = instance->statePre;
			return new cv::Mat(ret);
	}

	// cv::KalmanFilter::setStatePre(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:393
	// ("cv::KalmanFilter::setStatePre", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_KalmanFilter_propStatePre_const_Mat(cv::KalmanFilter* instance, const cv::Mat* val) {
			instance->statePre = *val;
	}

	// cv::KalmanFilter::statePost() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:394
	// ("cv::KalmanFilter::statePost", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_KalmanFilter_propStatePost_const(const cv::KalmanFilter* instance) {
			cv::Mat ret = instance->statePost;
			return new cv::Mat(ret);
	}

	// cv::KalmanFilter::setStatePost(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:394
	// ("cv::KalmanFilter::setStatePost", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_KalmanFilter_propStatePost_const_Mat(cv::KalmanFilter* instance, const cv::Mat* val) {
			instance->statePost = *val;
	}

	// cv::KalmanFilter::transitionMatrix() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:395
	// ("cv::KalmanFilter::transitionMatrix", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_KalmanFilter_propTransitionMatrix_const(const cv::KalmanFilter* instance) {
			cv::Mat ret = instance->transitionMatrix;
			return new cv::Mat(ret);
	}

	// cv::KalmanFilter::setTransitionMatrix(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:395
	// ("cv::KalmanFilter::setTransitionMatrix", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_KalmanFilter_propTransitionMatrix_const_Mat(cv::KalmanFilter* instance, const cv::Mat* val) {
			instance->transitionMatrix = *val;
	}

	// cv::KalmanFilter::controlMatrix() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:396
	// ("cv::KalmanFilter::controlMatrix", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_KalmanFilter_propControlMatrix_const(const cv::KalmanFilter* instance) {
			cv::Mat ret = instance->controlMatrix;
			return new cv::Mat(ret);
	}

	// cv::KalmanFilter::setControlMatrix(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:396
	// ("cv::KalmanFilter::setControlMatrix", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_KalmanFilter_propControlMatrix_const_Mat(cv::KalmanFilter* instance, const cv::Mat* val) {
			instance->controlMatrix = *val;
	}

	// cv::KalmanFilter::measurementMatrix() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:397
	// ("cv::KalmanFilter::measurementMatrix", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_KalmanFilter_propMeasurementMatrix_const(const cv::KalmanFilter* instance) {
			cv::Mat ret = instance->measurementMatrix;
			return new cv::Mat(ret);
	}

	// cv::KalmanFilter::setMeasurementMatrix(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:397
	// ("cv::KalmanFilter::setMeasurementMatrix", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_KalmanFilter_propMeasurementMatrix_const_Mat(cv::KalmanFilter* instance, const cv::Mat* val) {
			instance->measurementMatrix = *val;
	}

	// cv::KalmanFilter::processNoiseCov() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:398
	// ("cv::KalmanFilter::processNoiseCov", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_KalmanFilter_propProcessNoiseCov_const(const cv::KalmanFilter* instance) {
			cv::Mat ret = instance->processNoiseCov;
			return new cv::Mat(ret);
	}

	// cv::KalmanFilter::setProcessNoiseCov(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:398
	// ("cv::KalmanFilter::setProcessNoiseCov", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_KalmanFilter_propProcessNoiseCov_const_Mat(cv::KalmanFilter* instance, const cv::Mat* val) {
			instance->processNoiseCov = *val;
	}

	// cv::KalmanFilter::measurementNoiseCov() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:399
	// ("cv::KalmanFilter::measurementNoiseCov", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_KalmanFilter_propMeasurementNoiseCov_const(const cv::KalmanFilter* instance) {
			cv::Mat ret = instance->measurementNoiseCov;
			return new cv::Mat(ret);
	}

	// cv::KalmanFilter::setMeasurementNoiseCov(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:399
	// ("cv::KalmanFilter::setMeasurementNoiseCov", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_KalmanFilter_propMeasurementNoiseCov_const_Mat(cv::KalmanFilter* instance, const cv::Mat* val) {
			instance->measurementNoiseCov = *val;
	}

	// cv::KalmanFilter::errorCovPre() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:400
	// ("cv::KalmanFilter::errorCovPre", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_KalmanFilter_propErrorCovPre_const(const cv::KalmanFilter* instance) {
			cv::Mat ret = instance->errorCovPre;
			return new cv::Mat(ret);
	}

	// cv::KalmanFilter::setErrorCovPre(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:400
	// ("cv::KalmanFilter::setErrorCovPre", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_KalmanFilter_propErrorCovPre_const_Mat(cv::KalmanFilter* instance, const cv::Mat* val) {
			instance->errorCovPre = *val;
	}

	// cv::KalmanFilter::gain() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:401
	// ("cv::KalmanFilter::gain", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_KalmanFilter_propGain_const(const cv::KalmanFilter* instance) {
			cv::Mat ret = instance->gain;
			return new cv::Mat(ret);
	}

	// cv::KalmanFilter::setGain(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:401
	// ("cv::KalmanFilter::setGain", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_KalmanFilter_propGain_const_Mat(cv::KalmanFilter* instance, const cv::Mat* val) {
			instance->gain = *val;
	}

	// cv::KalmanFilter::errorCovPost() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:402
	// ("cv::KalmanFilter::errorCovPost", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_KalmanFilter_propErrorCovPost_const(const cv::KalmanFilter* instance) {
			cv::Mat ret = instance->errorCovPost;
			return new cv::Mat(ret);
	}

	// cv::KalmanFilter::setErrorCovPost(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:402
	// ("cv::KalmanFilter::setErrorCovPost", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_KalmanFilter_propErrorCovPost_const_Mat(cv::KalmanFilter* instance, const cv::Mat* val) {
			instance->errorCovPost = *val;
	}

	// cv::KalmanFilter::temp1() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:405
	// ("cv::KalmanFilter::temp1", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_KalmanFilter_propTemp1_const(const cv::KalmanFilter* instance) {
			cv::Mat ret = instance->temp1;
			return new cv::Mat(ret);
	}

	// cv::KalmanFilter::setTemp1(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:405
	// ("cv::KalmanFilter::setTemp1", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_KalmanFilter_propTemp1_const_Mat(cv::KalmanFilter* instance, const cv::Mat* val) {
			instance->temp1 = *val;
	}

	// cv::KalmanFilter::temp2() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:406
	// ("cv::KalmanFilter::temp2", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_KalmanFilter_propTemp2_const(const cv::KalmanFilter* instance) {
			cv::Mat ret = instance->temp2;
			return new cv::Mat(ret);
	}

	// cv::KalmanFilter::setTemp2(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:406
	// ("cv::KalmanFilter::setTemp2", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_KalmanFilter_propTemp2_const_Mat(cv::KalmanFilter* instance, const cv::Mat* val) {
			instance->temp2 = *val;
	}

	// cv::KalmanFilter::temp3() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:407
	// ("cv::KalmanFilter::temp3", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_KalmanFilter_propTemp3_const(const cv::KalmanFilter* instance) {
			cv::Mat ret = instance->temp3;
			return new cv::Mat(ret);
	}

	// cv::KalmanFilter::setTemp3(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:407
	// ("cv::KalmanFilter::setTemp3", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_KalmanFilter_propTemp3_const_Mat(cv::KalmanFilter* instance, const cv::Mat* val) {
			instance->temp3 = *val;
	}

	// cv::KalmanFilter::temp4() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:408
	// ("cv::KalmanFilter::temp4", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_KalmanFilter_propTemp4_const(const cv::KalmanFilter* instance) {
			cv::Mat ret = instance->temp4;
			return new cv::Mat(ret);
	}

	// cv::KalmanFilter::setTemp4(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:408
	// ("cv::KalmanFilter::setTemp4", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_KalmanFilter_propTemp4_const_Mat(cv::KalmanFilter* instance, const cv::Mat* val) {
			instance->temp4 = *val;
	}

	// cv::KalmanFilter::temp5() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:409
	// ("cv::KalmanFilter::temp5", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_KalmanFilter_propTemp5_const(const cv::KalmanFilter* instance) {
			cv::Mat ret = instance->temp5;
			return new cv::Mat(ret);
	}

	// cv::KalmanFilter::setTemp5(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:409
	// ("cv::KalmanFilter::setTemp5", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_KalmanFilter_propTemp5_const_Mat(cv::KalmanFilter* instance, const cv::Mat* val) {
			instance->temp5 = *val;
	}

	// cv::KalmanFilter::delete() generated
	// ("cv::KalmanFilter::delete", vec![(pred!(mut, [], []), _)]),
	void cv_KalmanFilter_delete(cv::KalmanFilter* instance) {
			delete instance;
	}

	// calc(InputArray, InputArray, InputArray, InputOutputArray, OutputArray, OutputArray)(InputArray, InputArray, InputArray, InputOutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:466
	// ("cv::SparseOpticalFlow::calc", vec![(pred!(mut, ["prevImg", "nextImg", "prevPts", "nextPts", "status", "err"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_SparseOpticalFlow_calc_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR(cv::SparseOpticalFlow* instance, const cv::_InputArray* prevImg, const cv::_InputArray* nextImg, const cv::_InputArray* prevPts, const cv::_InputOutputArray* nextPts, const cv::_OutputArray* status, const cv::_OutputArray* err, ResultVoid* ocvrs_return) {
		try {
			instance->calc(*prevImg, *nextImg, *prevPts, *nextPts, *status, *err);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::SparseOpticalFlow::calc(InputArray, InputArray, InputArray, InputOutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:466
	// ("cv::SparseOpticalFlow::calc", vec![(pred!(mut, ["prevImg", "nextImg", "prevPts", "nextPts", "status"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_SparseOpticalFlow_calc_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR(cv::SparseOpticalFlow* instance, const cv::_InputArray* prevImg, const cv::_InputArray* nextImg, const cv::_InputArray* prevPts, const cv::_InputOutputArray* nextPts, const cv::_OutputArray* status, ResultVoid* ocvrs_return) {
		try {
			instance->calc(*prevImg, *nextImg, *prevPts, *nextPts, *status);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::SparseOpticalFlow::to_SparsePyrLKOpticalFlow() generated
	// ("cv::SparseOpticalFlow::to_SparsePyrLKOpticalFlow", vec![(pred!(mut, [], []), _)]),
	cv::SparsePyrLKOpticalFlow* cv_SparseOpticalFlow_to_SparsePyrLKOpticalFlow(cv::SparseOpticalFlow* instance) {
			return dynamic_cast<cv::SparsePyrLKOpticalFlow*>(instance);
	}

	// cv::SparseOpticalFlow::to_Algorithm() generated
	// ("cv::SparseOpticalFlow::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_SparseOpticalFlow_to_Algorithm(cv::SparseOpticalFlow* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::SparseOpticalFlow::delete() generated
	// ("cv::SparseOpticalFlow::delete", vec![(pred!(mut, [], []), _)]),
	void cv_SparseOpticalFlow_delete(cv::SparseOpticalFlow* instance) {
			delete instance;
	}

	// getWinSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:697
	// ("cv::SparsePyrLKOpticalFlow::getWinSize", vec![(pred!(const, [], []), _)]),
	void cv_SparsePyrLKOpticalFlow_getWinSize_const(const cv::SparsePyrLKOpticalFlow* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->getWinSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWinSize(Size)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:698
	// ("cv::SparsePyrLKOpticalFlow::setWinSize", vec![(pred!(mut, ["winSize"], ["cv::Size"]), _)]),
	void cv_SparsePyrLKOpticalFlow_setWinSize_Size(cv::SparsePyrLKOpticalFlow* instance, cv::Size* winSize, ResultVoid* ocvrs_return) {
		try {
			instance->setWinSize(*winSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxLevel()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:700
	// ("cv::SparsePyrLKOpticalFlow::getMaxLevel", vec![(pred!(const, [], []), _)]),
	void cv_SparsePyrLKOpticalFlow_getMaxLevel_const(const cv::SparsePyrLKOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMaxLevel();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxLevel(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:701
	// ("cv::SparsePyrLKOpticalFlow::setMaxLevel", vec![(pred!(mut, ["maxLevel"], ["int"]), _)]),
	void cv_SparsePyrLKOpticalFlow_setMaxLevel_int(cv::SparsePyrLKOpticalFlow* instance, int maxLevel, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxLevel(maxLevel);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTermCriteria()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:703
	// ("cv::SparsePyrLKOpticalFlow::getTermCriteria", vec![(pred!(const, [], []), _)]),
	void cv_SparsePyrLKOpticalFlow_getTermCriteria_const(const cv::SparsePyrLKOpticalFlow* instance, Result<cv::TermCriteria>* ocvrs_return) {
		try {
			cv::TermCriteria ret = instance->getTermCriteria();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTermCriteria(TermCriteria &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:704
	// ("cv::SparsePyrLKOpticalFlow::setTermCriteria", vec![(pred!(mut, ["crit"], ["cv::TermCriteria*"]), _)]),
	void cv_SparsePyrLKOpticalFlow_setTermCriteria_TermCriteriaR(cv::SparsePyrLKOpticalFlow* instance, cv::TermCriteria* crit, ResultVoid* ocvrs_return) {
		try {
			instance->setTermCriteria(*crit);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getFlags()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:706
	// ("cv::SparsePyrLKOpticalFlow::getFlags", vec![(pred!(const, [], []), _)]),
	void cv_SparsePyrLKOpticalFlow_getFlags_const(const cv::SparsePyrLKOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getFlags();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setFlags(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:707
	// ("cv::SparsePyrLKOpticalFlow::setFlags", vec![(pred!(mut, ["flags"], ["int"]), _)]),
	void cv_SparsePyrLKOpticalFlow_setFlags_int(cv::SparsePyrLKOpticalFlow* instance, int flags, ResultVoid* ocvrs_return) {
		try {
			instance->setFlags(flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMinEigThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:709
	// ("cv::SparsePyrLKOpticalFlow::getMinEigThreshold", vec![(pred!(const, [], []), _)]),
	void cv_SparsePyrLKOpticalFlow_getMinEigThreshold_const(const cv::SparsePyrLKOpticalFlow* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMinEigThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMinEigThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:710
	// ("cv::SparsePyrLKOpticalFlow::setMinEigThreshold", vec![(pred!(mut, ["minEigThreshold"], ["double"]), _)]),
	void cv_SparsePyrLKOpticalFlow_setMinEigThreshold_double(cv::SparsePyrLKOpticalFlow* instance, double minEigThreshold, ResultVoid* ocvrs_return) {
		try {
			instance->setMinEigThreshold(minEigThreshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(Size, int, TermCriteria, int, double)(SimpleClass, Primitive, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:712
	// ("cv::SparsePyrLKOpticalFlow::create", vec![(pred!(mut, ["winSize", "maxLevel", "crit", "flags", "minEigThreshold"], ["cv::Size", "int", "cv::TermCriteria", "int", "double"]), _)]),
	void cv_SparsePyrLKOpticalFlow_create_Size_int_TermCriteria_int_double(cv::Size* winSize, int maxLevel, cv::TermCriteria* crit, int flags, double minEigThreshold, Result<cv::Ptr<cv::SparsePyrLKOpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::SparsePyrLKOpticalFlow> ret = cv::SparsePyrLKOpticalFlow::create(*winSize, maxLevel, *crit, flags, minEigThreshold);
			Ok(new cv::Ptr<cv::SparsePyrLKOpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::SparsePyrLKOpticalFlow::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:712
	// ("cv::SparsePyrLKOpticalFlow::create", vec![(pred!(mut, [], []), _)]),
	void cv_SparsePyrLKOpticalFlow_create(Result<cv::Ptr<cv::SparsePyrLKOpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::SparsePyrLKOpticalFlow> ret = cv::SparsePyrLKOpticalFlow::create();
			Ok(new cv::Ptr<cv::SparsePyrLKOpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::SparsePyrLKOpticalFlow::to_Algorithm() generated
	// ("cv::SparsePyrLKOpticalFlow::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_SparsePyrLKOpticalFlow_to_Algorithm(cv::SparsePyrLKOpticalFlow* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::SparsePyrLKOpticalFlow::to_SparseOpticalFlow() generated
	// ("cv::SparsePyrLKOpticalFlow::to_SparseOpticalFlow", vec![(pred!(mut, [], []), _)]),
	cv::SparseOpticalFlow* cv_SparsePyrLKOpticalFlow_to_SparseOpticalFlow(cv::SparsePyrLKOpticalFlow* instance) {
			return dynamic_cast<cv::SparseOpticalFlow*>(instance);
	}

	// cv::SparsePyrLKOpticalFlow::delete() generated
	// ("cv::SparsePyrLKOpticalFlow::delete", vec![(pred!(mut, [], []), _)]),
	void cv_SparsePyrLKOpticalFlow_delete(cv::SparsePyrLKOpticalFlow* instance) {
			delete instance;
	}

	// init(InputArray, const Rect &)(InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:737
	// ("cv::Tracker::init", vec![(pred!(mut, ["image", "boundingBox"], ["const cv::_InputArray*", "const cv::Rect*"]), _)]),
	void cv_Tracker_init_const__InputArrayR_const_RectR(cv::Tracker* instance, const cv::_InputArray* image, const cv::Rect* boundingBox, ResultVoid* ocvrs_return) {
		try {
			instance->init(*image, *boundingBox);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// update(InputArray, Rect &)(InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:749
	// ("cv::Tracker::update", vec![(pred!(mut, ["image", "boundingBox"], ["const cv::_InputArray*", "cv::Rect*"]), _)]),
	void cv_Tracker_update_const__InputArrayR_RectR(cv::Tracker* instance, const cv::_InputArray* image, cv::Rect* boundingBox, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->update(*image, *boundingBox);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Tracker::to_TrackerDaSiamRPN() generated
	// ("cv::Tracker::to_TrackerDaSiamRPN", vec![(pred!(mut, [], []), _)]),
	cv::TrackerDaSiamRPN* cv_Tracker_to_TrackerDaSiamRPN(cv::Tracker* instance) {
			return dynamic_cast<cv::TrackerDaSiamRPN*>(instance);
	}

	// cv::Tracker::to_TrackerGOTURN() generated
	// ("cv::Tracker::to_TrackerGOTURN", vec![(pred!(mut, [], []), _)]),
	cv::TrackerGOTURN* cv_Tracker_to_TrackerGOTURN(cv::Tracker* instance) {
			return dynamic_cast<cv::TrackerGOTURN*>(instance);
	}

	// cv::Tracker::to_TrackerMIL() generated
	// ("cv::Tracker::to_TrackerMIL", vec![(pred!(mut, [], []), _)]),
	cv::TrackerMIL* cv_Tracker_to_TrackerMIL(cv::Tracker* instance) {
			return dynamic_cast<cv::TrackerMIL*>(instance);
	}

	// cv::Tracker::to_TrackerNano() generated
	// ("cv::Tracker::to_TrackerNano", vec![(pred!(mut, [], []), _)]),
	cv::TrackerNano* cv_Tracker_to_TrackerNano(cv::Tracker* instance) {
			return dynamic_cast<cv::TrackerNano*>(instance);
	}

	// cv::Tracker::to_TrackerVit() generated
	// ("cv::Tracker::to_TrackerVit", vec![(pred!(mut, [], []), _)]),
	cv::TrackerVit* cv_Tracker_to_TrackerVit(cv::Tracker* instance) {
			return dynamic_cast<cv::TrackerVit*>(instance);
	}

	// cv::Tracker::delete() generated
	// ("cv::Tracker::delete", vec![(pred!(mut, [], []), _)]),
	void cv_Tracker_delete(cv::Tracker* instance) {
			delete instance;
	}

	// create(const TrackerDaSiamRPN::Params &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:854
	// ("cv::TrackerDaSiamRPN::create", vec![(pred!(mut, ["parameters"], ["const cv::TrackerDaSiamRPN::Params*"]), _)]),
	void cv_TrackerDaSiamRPN_create_const_ParamsR(const cv::TrackerDaSiamRPN::Params* parameters, Result<cv::Ptr<cv::TrackerDaSiamRPN>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::TrackerDaSiamRPN> ret = cv::TrackerDaSiamRPN::create(*parameters);
			Ok(new cv::Ptr<cv::TrackerDaSiamRPN>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TrackerDaSiamRPN::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:854
	// ("cv::TrackerDaSiamRPN::create", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerDaSiamRPN_create(Result<cv::Ptr<cv::TrackerDaSiamRPN>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::TrackerDaSiamRPN> ret = cv::TrackerDaSiamRPN::create();
			Ok(new cv::Ptr<cv::TrackerDaSiamRPN>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTrackingScore()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:858
	// ("cv::TrackerDaSiamRPN::getTrackingScore", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerDaSiamRPN_getTrackingScore(cv::TrackerDaSiamRPN* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getTrackingScore();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TrackerDaSiamRPN::to_Tracker() generated
	// ("cv::TrackerDaSiamRPN::to_Tracker", vec![(pred!(mut, [], []), _)]),
	cv::Tracker* cv_TrackerDaSiamRPN_to_Tracker(cv::TrackerDaSiamRPN* instance) {
			return dynamic_cast<cv::Tracker*>(instance);
	}

	// cv::TrackerDaSiamRPN::delete() generated
	// ("cv::TrackerDaSiamRPN::delete", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerDaSiamRPN_delete(cv::TrackerDaSiamRPN* instance) {
			delete instance;
	}

	// Params()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:842
	// ("cv::TrackerDaSiamRPN::Params::Params", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerDaSiamRPN_Params_Params(Result<cv::TrackerDaSiamRPN::Params*>* ocvrs_return) {
		try {
			cv::TrackerDaSiamRPN::Params* ret = new cv::TrackerDaSiamRPN::Params();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TrackerDaSiamRPN::Params::implicitClone() generated
	// ("cv::TrackerDaSiamRPN::Params::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::TrackerDaSiamRPN::Params* cv_TrackerDaSiamRPN_Params_implicitClone_const(const cv::TrackerDaSiamRPN::Params* instance) {
			return new cv::TrackerDaSiamRPN::Params(*instance);
	}

	// cv::TrackerDaSiamRPN::Params::model() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:843
	// ("cv::TrackerDaSiamRPN::Params::model", vec![(pred!(const, [], []), _)]),
	void* cv_TrackerDaSiamRPN_Params_propModel_const(const cv::TrackerDaSiamRPN::Params* instance) {
			std::string ret = instance->model;
			return ocvrs_create_string(ret.c_str());
	}

	// cv::TrackerDaSiamRPN::Params::setModel(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:843
	// ("cv::TrackerDaSiamRPN::Params::setModel", vec![(pred!(mut, ["val"], ["const std::string"]), _)]),
	void cv_TrackerDaSiamRPN_Params_propModel_const_string(cv::TrackerDaSiamRPN::Params* instance, const char* val) {
			instance->model = std::string(val);
	}

	// cv::TrackerDaSiamRPN::Params::kernel_cls1() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:844
	// ("cv::TrackerDaSiamRPN::Params::kernel_cls1", vec![(pred!(const, [], []), _)]),
	void* cv_TrackerDaSiamRPN_Params_propKernel_cls1_const(const cv::TrackerDaSiamRPN::Params* instance) {
			std::string ret = instance->kernel_cls1;
			return ocvrs_create_string(ret.c_str());
	}

	// cv::TrackerDaSiamRPN::Params::setKernel_cls1(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:844
	// ("cv::TrackerDaSiamRPN::Params::setKernel_cls1", vec![(pred!(mut, ["val"], ["const std::string"]), _)]),
	void cv_TrackerDaSiamRPN_Params_propKernel_cls1_const_string(cv::TrackerDaSiamRPN::Params* instance, const char* val) {
			instance->kernel_cls1 = std::string(val);
	}

	// cv::TrackerDaSiamRPN::Params::kernel_r1() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:845
	// ("cv::TrackerDaSiamRPN::Params::kernel_r1", vec![(pred!(const, [], []), _)]),
	void* cv_TrackerDaSiamRPN_Params_propKernel_r1_const(const cv::TrackerDaSiamRPN::Params* instance) {
			std::string ret = instance->kernel_r1;
			return ocvrs_create_string(ret.c_str());
	}

	// cv::TrackerDaSiamRPN::Params::setKernel_r1(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:845
	// ("cv::TrackerDaSiamRPN::Params::setKernel_r1", vec![(pred!(mut, ["val"], ["const std::string"]), _)]),
	void cv_TrackerDaSiamRPN_Params_propKernel_r1_const_string(cv::TrackerDaSiamRPN::Params* instance, const char* val) {
			instance->kernel_r1 = std::string(val);
	}

	// cv::TrackerDaSiamRPN::Params::backend() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:846
	// ("cv::TrackerDaSiamRPN::Params::backend", vec![(pred!(const, [], []), _)]),
	int cv_TrackerDaSiamRPN_Params_propBackend_const(const cv::TrackerDaSiamRPN::Params* instance) {
			int ret = instance->backend;
			return ret;
	}

	// cv::TrackerDaSiamRPN::Params::setBackend(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:846
	// ("cv::TrackerDaSiamRPN::Params::setBackend", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_TrackerDaSiamRPN_Params_propBackend_const_int(cv::TrackerDaSiamRPN::Params* instance, const int val) {
			instance->backend = val;
	}

	// cv::TrackerDaSiamRPN::Params::target() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:847
	// ("cv::TrackerDaSiamRPN::Params::target", vec![(pred!(const, [], []), _)]),
	int cv_TrackerDaSiamRPN_Params_propTarget_const(const cv::TrackerDaSiamRPN::Params* instance) {
			int ret = instance->target;
			return ret;
	}

	// cv::TrackerDaSiamRPN::Params::setTarget(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:847
	// ("cv::TrackerDaSiamRPN::Params::setTarget", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_TrackerDaSiamRPN_Params_propTarget_const_int(cv::TrackerDaSiamRPN::Params* instance, const int val) {
			instance->target = val;
	}

	// cv::TrackerDaSiamRPN::Params::delete() generated
	// ("cv::TrackerDaSiamRPN::Params::delete", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerDaSiamRPN_Params_delete(cv::TrackerDaSiamRPN::Params* instance) {
			delete instance;
	}

	// create(const TrackerGOTURN::Params &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:827
	// ("cv::TrackerGOTURN::create", vec![(pred!(mut, ["parameters"], ["const cv::TrackerGOTURN::Params*"]), _)]),
	void cv_TrackerGOTURN_create_const_ParamsR(const cv::TrackerGOTURN::Params* parameters, Result<cv::Ptr<cv::TrackerGOTURN>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::TrackerGOTURN> ret = cv::TrackerGOTURN::create(*parameters);
			Ok(new cv::Ptr<cv::TrackerGOTURN>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TrackerGOTURN::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:827
	// ("cv::TrackerGOTURN::create", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerGOTURN_create(Result<cv::Ptr<cv::TrackerGOTURN>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::TrackerGOTURN> ret = cv::TrackerGOTURN::create();
			Ok(new cv::Ptr<cv::TrackerGOTURN>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TrackerGOTURN::to_Tracker() generated
	// ("cv::TrackerGOTURN::to_Tracker", vec![(pred!(mut, [], []), _)]),
	cv::Tracker* cv_TrackerGOTURN_to_Tracker(cv::TrackerGOTURN* instance) {
			return dynamic_cast<cv::Tracker*>(instance);
	}

	// cv::TrackerGOTURN::delete() generated
	// ("cv::TrackerGOTURN::delete", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerGOTURN_delete(cv::TrackerGOTURN* instance) {
			delete instance;
	}

	// Params()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:818
	// ("cv::TrackerGOTURN::Params::Params", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerGOTURN_Params_Params(Result<cv::TrackerGOTURN::Params*>* ocvrs_return) {
		try {
			cv::TrackerGOTURN::Params* ret = new cv::TrackerGOTURN::Params();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TrackerGOTURN::Params::implicitClone() generated
	// ("cv::TrackerGOTURN::Params::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::TrackerGOTURN::Params* cv_TrackerGOTURN_Params_implicitClone_const(const cv::TrackerGOTURN::Params* instance) {
			return new cv::TrackerGOTURN::Params(*instance);
	}

	// cv::TrackerGOTURN::Params::modelTxt() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:819
	// ("cv::TrackerGOTURN::Params::modelTxt", vec![(pred!(const, [], []), _)]),
	void* cv_TrackerGOTURN_Params_propModelTxt_const(const cv::TrackerGOTURN::Params* instance) {
			std::string ret = instance->modelTxt;
			return ocvrs_create_string(ret.c_str());
	}

	// cv::TrackerGOTURN::Params::setModelTxt(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:819
	// ("cv::TrackerGOTURN::Params::setModelTxt", vec![(pred!(mut, ["val"], ["const std::string"]), _)]),
	void cv_TrackerGOTURN_Params_propModelTxt_const_string(cv::TrackerGOTURN::Params* instance, const char* val) {
			instance->modelTxt = std::string(val);
	}

	// cv::TrackerGOTURN::Params::modelBin() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:820
	// ("cv::TrackerGOTURN::Params::modelBin", vec![(pred!(const, [], []), _)]),
	void* cv_TrackerGOTURN_Params_propModelBin_const(const cv::TrackerGOTURN::Params* instance) {
			std::string ret = instance->modelBin;
			return ocvrs_create_string(ret.c_str());
	}

	// cv::TrackerGOTURN::Params::setModelBin(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:820
	// ("cv::TrackerGOTURN::Params::setModelBin", vec![(pred!(mut, ["val"], ["const std::string"]), _)]),
	void cv_TrackerGOTURN_Params_propModelBin_const_string(cv::TrackerGOTURN::Params* instance, const char* val) {
			instance->modelBin = std::string(val);
	}

	// cv::TrackerGOTURN::Params::delete() generated
	// ("cv::TrackerGOTURN::Params::delete", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerGOTURN_Params_delete(cv::TrackerGOTURN::Params* instance) {
			delete instance;
	}

	// create(const TrackerMIL::Params &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:786
	// ("cv::TrackerMIL::create", vec![(pred!(mut, ["parameters"], ["const cv::TrackerMIL::Params*"]), _)]),
	void cv_TrackerMIL_create_const_ParamsR(const cv::TrackerMIL::Params* parameters, Result<cv::Ptr<cv::TrackerMIL>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::TrackerMIL> ret = cv::TrackerMIL::create(*parameters);
			Ok(new cv::Ptr<cv::TrackerMIL>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TrackerMIL::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:786
	// ("cv::TrackerMIL::create", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerMIL_create(Result<cv::Ptr<cv::TrackerMIL>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::TrackerMIL> ret = cv::TrackerMIL::create();
			Ok(new cv::Ptr<cv::TrackerMIL>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TrackerMIL::to_Tracker() generated
	// ("cv::TrackerMIL::to_Tracker", vec![(pred!(mut, [], []), _)]),
	cv::Tracker* cv_TrackerMIL_to_Tracker(cv::TrackerMIL* instance) {
			return dynamic_cast<cv::Tracker*>(instance);
	}

	// cv::TrackerMIL::delete() generated
	// ("cv::TrackerMIL::delete", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerMIL_delete(cv::TrackerMIL* instance) {
			delete instance;
	}

	// Params()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:771
	// ("cv::TrackerMIL::Params::Params", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerMIL_Params_Params(Result<cv::TrackerMIL::Params>* ocvrs_return) {
		try {
			cv::TrackerMIL::Params ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const TrackerNano::Params &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:892
	// ("cv::TrackerNano::create", vec![(pred!(mut, ["parameters"], ["const cv::TrackerNano::Params*"]), _)]),
	void cv_TrackerNano_create_const_ParamsR(const cv::TrackerNano::Params* parameters, Result<cv::Ptr<cv::TrackerNano>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::TrackerNano> ret = cv::TrackerNano::create(*parameters);
			Ok(new cv::Ptr<cv::TrackerNano>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TrackerNano::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:892
	// ("cv::TrackerNano::create", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerNano_create(Result<cv::Ptr<cv::TrackerNano>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::TrackerNano> ret = cv::TrackerNano::create();
			Ok(new cv::Ptr<cv::TrackerNano>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTrackingScore()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:896
	// ("cv::TrackerNano::getTrackingScore", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerNano_getTrackingScore(cv::TrackerNano* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getTrackingScore();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TrackerNano::to_Tracker() generated
	// ("cv::TrackerNano::to_Tracker", vec![(pred!(mut, [], []), _)]),
	cv::Tracker* cv_TrackerNano_to_Tracker(cv::TrackerNano* instance) {
			return dynamic_cast<cv::Tracker*>(instance);
	}

	// cv::TrackerNano::delete() generated
	// ("cv::TrackerNano::delete", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerNano_delete(cv::TrackerNano* instance) {
			delete instance;
	}

	// Params()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:881
	// ("cv::TrackerNano::Params::Params", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerNano_Params_Params(Result<cv::TrackerNano::Params*>* ocvrs_return) {
		try {
			cv::TrackerNano::Params* ret = new cv::TrackerNano::Params();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TrackerNano::Params::implicitClone() generated
	// ("cv::TrackerNano::Params::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::TrackerNano::Params* cv_TrackerNano_Params_implicitClone_const(const cv::TrackerNano::Params* instance) {
			return new cv::TrackerNano::Params(*instance);
	}

	// cv::TrackerNano::Params::backbone() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:882
	// ("cv::TrackerNano::Params::backbone", vec![(pred!(const, [], []), _)]),
	void* cv_TrackerNano_Params_propBackbone_const(const cv::TrackerNano::Params* instance) {
			std::string ret = instance->backbone;
			return ocvrs_create_string(ret.c_str());
	}

	// cv::TrackerNano::Params::setBackbone(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:882
	// ("cv::TrackerNano::Params::setBackbone", vec![(pred!(mut, ["val"], ["const std::string"]), _)]),
	void cv_TrackerNano_Params_propBackbone_const_string(cv::TrackerNano::Params* instance, const char* val) {
			instance->backbone = std::string(val);
	}

	// cv::TrackerNano::Params::neckhead() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:883
	// ("cv::TrackerNano::Params::neckhead", vec![(pred!(const, [], []), _)]),
	void* cv_TrackerNano_Params_propNeckhead_const(const cv::TrackerNano::Params* instance) {
			std::string ret = instance->neckhead;
			return ocvrs_create_string(ret.c_str());
	}

	// cv::TrackerNano::Params::setNeckhead(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:883
	// ("cv::TrackerNano::Params::setNeckhead", vec![(pred!(mut, ["val"], ["const std::string"]), _)]),
	void cv_TrackerNano_Params_propNeckhead_const_string(cv::TrackerNano::Params* instance, const char* val) {
			instance->neckhead = std::string(val);
	}

	// cv::TrackerNano::Params::backend() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:884
	// ("cv::TrackerNano::Params::backend", vec![(pred!(const, [], []), _)]),
	int cv_TrackerNano_Params_propBackend_const(const cv::TrackerNano::Params* instance) {
			int ret = instance->backend;
			return ret;
	}

	// cv::TrackerNano::Params::setBackend(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:884
	// ("cv::TrackerNano::Params::setBackend", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_TrackerNano_Params_propBackend_const_int(cv::TrackerNano::Params* instance, const int val) {
			instance->backend = val;
	}

	// cv::TrackerNano::Params::target() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:885
	// ("cv::TrackerNano::Params::target", vec![(pred!(const, [], []), _)]),
	int cv_TrackerNano_Params_propTarget_const(const cv::TrackerNano::Params* instance) {
			int ret = instance->target;
			return ret;
	}

	// cv::TrackerNano::Params::setTarget(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:885
	// ("cv::TrackerNano::Params::setTarget", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_TrackerNano_Params_propTarget_const_int(cv::TrackerNano::Params* instance, const int val) {
			instance->target = val;
	}

	// cv::TrackerNano::Params::delete() generated
	// ("cv::TrackerNano::Params::delete", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerNano_Params_delete(cv::TrackerNano::Params* instance) {
			delete instance;
	}

	// create(const TrackerVit::Params &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:930
	// ("cv::TrackerVit::create", vec![(pred!(mut, ["parameters"], ["const cv::TrackerVit::Params*"]), _)]),
	void cv_TrackerVit_create_const_ParamsR(const cv::TrackerVit::Params* parameters, Result<cv::Ptr<cv::TrackerVit>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::TrackerVit> ret = cv::TrackerVit::create(*parameters);
			Ok(new cv::Ptr<cv::TrackerVit>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TrackerVit::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:930
	// ("cv::TrackerVit::create", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerVit_create(Result<cv::Ptr<cv::TrackerVit>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::TrackerVit> ret = cv::TrackerVit::create();
			Ok(new cv::Ptr<cv::TrackerVit>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTrackingScore()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:934
	// ("cv::TrackerVit::getTrackingScore", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerVit_getTrackingScore(cv::TrackerVit* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getTrackingScore();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TrackerVit::to_Tracker() generated
	// ("cv::TrackerVit::to_Tracker", vec![(pred!(mut, [], []), _)]),
	cv::Tracker* cv_TrackerVit_to_Tracker(cv::TrackerVit* instance) {
			return dynamic_cast<cv::Tracker*>(instance);
	}

	// cv::TrackerVit::delete() generated
	// ("cv::TrackerVit::delete", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerVit_delete(cv::TrackerVit* instance) {
			delete instance;
	}

	// Params()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:917
	// ("cv::TrackerVit::Params::Params", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerVit_Params_Params(Result<cv::TrackerVit::Params*>* ocvrs_return) {
		try {
			cv::TrackerVit::Params* ret = new cv::TrackerVit::Params();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TrackerVit::Params::implicitClone() generated
	// ("cv::TrackerVit::Params::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::TrackerVit::Params* cv_TrackerVit_Params_implicitClone_const(const cv::TrackerVit::Params* instance) {
			return new cv::TrackerVit::Params(*instance);
	}

	// cv::TrackerVit::Params::net() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:918
	// ("cv::TrackerVit::Params::net", vec![(pred!(const, [], []), _)]),
	void* cv_TrackerVit_Params_propNet_const(const cv::TrackerVit::Params* instance) {
			std::string ret = instance->net;
			return ocvrs_create_string(ret.c_str());
	}

	// cv::TrackerVit::Params::setNet(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:918
	// ("cv::TrackerVit::Params::setNet", vec![(pred!(mut, ["val"], ["const std::string"]), _)]),
	void cv_TrackerVit_Params_propNet_const_string(cv::TrackerVit::Params* instance, const char* val) {
			instance->net = std::string(val);
	}

	// cv::TrackerVit::Params::backend() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:919
	// ("cv::TrackerVit::Params::backend", vec![(pred!(const, [], []), _)]),
	int cv_TrackerVit_Params_propBackend_const(const cv::TrackerVit::Params* instance) {
			int ret = instance->backend;
			return ret;
	}

	// cv::TrackerVit::Params::setBackend(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:919
	// ("cv::TrackerVit::Params::setBackend", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_TrackerVit_Params_propBackend_const_int(cv::TrackerVit::Params* instance, const int val) {
			instance->backend = val;
	}

	// cv::TrackerVit::Params::target() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:920
	// ("cv::TrackerVit::Params::target", vec![(pred!(const, [], []), _)]),
	int cv_TrackerVit_Params_propTarget_const(const cv::TrackerVit::Params* instance) {
			int ret = instance->target;
			return ret;
	}

	// cv::TrackerVit::Params::setTarget(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:920
	// ("cv::TrackerVit::Params::setTarget", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_TrackerVit_Params_propTarget_const_int(cv::TrackerVit::Params* instance, const int val) {
			instance->target = val;
	}

	// cv::TrackerVit::Params::meanvalue() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:921
	// ("cv::TrackerVit::Params::meanvalue", vec![(pred!(const, [], []), _)]),
	void cv_TrackerVit_Params_propMeanvalue_const(const cv::TrackerVit::Params* instance, cv::Scalar* ocvrs_return) {
			cv::Scalar ret = instance->meanvalue;
			*ocvrs_return = ret;
	}

	// cv::TrackerVit::Params::setMeanvalue(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:921
	// ("cv::TrackerVit::Params::setMeanvalue", vec![(pred!(mut, ["val"], ["const cv::Scalar"]), _)]),
	void cv_TrackerVit_Params_propMeanvalue_const_Scalar(cv::TrackerVit::Params* instance, const cv::Scalar* val) {
			instance->meanvalue = *val;
	}

	// cv::TrackerVit::Params::stdvalue() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:922
	// ("cv::TrackerVit::Params::stdvalue", vec![(pred!(const, [], []), _)]),
	void cv_TrackerVit_Params_propStdvalue_const(const cv::TrackerVit::Params* instance, cv::Scalar* ocvrs_return) {
			cv::Scalar ret = instance->stdvalue;
			*ocvrs_return = ret;
	}

	// cv::TrackerVit::Params::setStdvalue(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:922
	// ("cv::TrackerVit::Params::setStdvalue", vec![(pred!(mut, ["val"], ["const cv::Scalar"]), _)]),
	void cv_TrackerVit_Params_propStdvalue_const_Scalar(cv::TrackerVit::Params* instance, const cv::Scalar* val) {
			instance->stdvalue = *val;
	}

	// cv::TrackerVit::Params::tracking_score_threshold() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:923
	// ("cv::TrackerVit::Params::tracking_score_threshold", vec![(pred!(const, [], []), _)]),
	float cv_TrackerVit_Params_propTracking_score_threshold_const(const cv::TrackerVit::Params* instance) {
			float ret = instance->tracking_score_threshold;
			return ret;
	}

	// cv::TrackerVit::Params::setTracking_score_threshold(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:923
	// ("cv::TrackerVit::Params::setTracking_score_threshold", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_TrackerVit_Params_propTracking_score_threshold_const_float(cv::TrackerVit::Params* instance, const float val) {
			instance->tracking_score_threshold = val;
	}

	// cv::TrackerVit::Params::delete() generated
	// ("cv::TrackerVit::Params::delete", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerVit_Params_delete(cv::TrackerVit::Params* instance) {
			delete instance;
	}

	// calcUV(InputArray, InputArray, InputOutputArray, InputOutputArray)(InputArray, InputArray, InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:528
	// ("cv::VariationalRefinement::calcUV", vec![(pred!(mut, ["I0", "I1", "flow_u", "flow_v"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
	void cv_VariationalRefinement_calcUV_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(cv::VariationalRefinement* instance, const cv::_InputArray* I0, const cv::_InputArray* I1, const cv::_InputOutputArray* flow_u, const cv::_InputOutputArray* flow_v, ResultVoid* ocvrs_return) {
		try {
			instance->calcUV(*I0, *I1, *flow_u, *flow_v);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getFixedPointIterations()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:532
	// ("cv::VariationalRefinement::getFixedPointIterations", vec![(pred!(const, [], []), _)]),
	void cv_VariationalRefinement_getFixedPointIterations_const(const cv::VariationalRefinement* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getFixedPointIterations();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setFixedPointIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:534
	// ("cv::VariationalRefinement::setFixedPointIterations", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_VariationalRefinement_setFixedPointIterations_int(cv::VariationalRefinement* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setFixedPointIterations(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSorIterations()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:539
	// ("cv::VariationalRefinement::getSorIterations", vec![(pred!(const, [], []), _)]),
	void cv_VariationalRefinement_getSorIterations_const(const cv::VariationalRefinement* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getSorIterations();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSorIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:541
	// ("cv::VariationalRefinement::setSorIterations", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_VariationalRefinement_setSorIterations_int(cv::VariationalRefinement* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setSorIterations(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getOmega()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:545
	// ("cv::VariationalRefinement::getOmega", vec![(pred!(const, [], []), _)]),
	void cv_VariationalRefinement_getOmega_const(const cv::VariationalRefinement* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getOmega();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setOmega(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:547
	// ("cv::VariationalRefinement::setOmega", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_VariationalRefinement_setOmega_float(cv::VariationalRefinement* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setOmega(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getAlpha()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:551
	// ("cv::VariationalRefinement::getAlpha", vec![(pred!(const, [], []), _)]),
	void cv_VariationalRefinement_getAlpha_const(const cv::VariationalRefinement* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getAlpha();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setAlpha(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:553
	// ("cv::VariationalRefinement::setAlpha", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_VariationalRefinement_setAlpha_float(cv::VariationalRefinement* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setAlpha(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDelta()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:557
	// ("cv::VariationalRefinement::getDelta", vec![(pred!(const, [], []), _)]),
	void cv_VariationalRefinement_getDelta_const(const cv::VariationalRefinement* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getDelta();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDelta(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:559
	// ("cv::VariationalRefinement::setDelta", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_VariationalRefinement_setDelta_float(cv::VariationalRefinement* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setDelta(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getGamma()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:563
	// ("cv::VariationalRefinement::getGamma", vec![(pred!(const, [], []), _)]),
	void cv_VariationalRefinement_getGamma_const(const cv::VariationalRefinement* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getGamma();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setGamma(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:565
	// ("cv::VariationalRefinement::setGamma", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_VariationalRefinement_setGamma_float(cv::VariationalRefinement* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setGamma(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getEpsilon()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:569
	// ("cv::VariationalRefinement::getEpsilon", vec![(pred!(const, [], []), _)]),
	void cv_VariationalRefinement_getEpsilon_const(const cv::VariationalRefinement* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getEpsilon();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setEpsilon(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:571
	// ("cv::VariationalRefinement::setEpsilon", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_VariationalRefinement_setEpsilon_float(cv::VariationalRefinement* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setEpsilon(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/video/tracking.hpp:575
	// ("cv::VariationalRefinement::create", vec![(pred!(mut, [], []), _)]),
	void cv_VariationalRefinement_create(Result<cv::Ptr<cv::VariationalRefinement>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::VariationalRefinement> ret = cv::VariationalRefinement::create();
			Ok(new cv::Ptr<cv::VariationalRefinement>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::VariationalRefinement::to_Algorithm() generated
	// ("cv::VariationalRefinement::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_VariationalRefinement_to_Algorithm(cv::VariationalRefinement* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::VariationalRefinement::to_DenseOpticalFlow() generated
	// ("cv::VariationalRefinement::to_DenseOpticalFlow", vec![(pred!(mut, [], []), _)]),
	cv::DenseOpticalFlow* cv_VariationalRefinement_to_DenseOpticalFlow(cv::VariationalRefinement* instance) {
			return dynamic_cast<cv::DenseOpticalFlow*>(instance);
	}

	// cv::VariationalRefinement::delete() generated
	// ("cv::VariationalRefinement::delete", vec![(pred!(mut, [], []), _)]),
	void cv_VariationalRefinement_delete(cv::VariationalRefinement* instance) {
			delete instance;
	}

}
