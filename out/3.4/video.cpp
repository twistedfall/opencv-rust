#include "ocvrs_common.hpp"
#include <opencv2/video.hpp>
#include "video_types.hpp"

extern "C" {
	// CamShift(InputArray, Rect &, TermCriteria)(InputArray, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:79
	// ("cv::CamShift", vec![(pred!(mut, ["probImage", "window", "criteria"], ["const cv::_InputArray*", "cv::Rect*", "cv::TermCriteria"]), _)]),
	void cv_CamShift_const__InputArrayR_RectR_TermCriteria(const cv::_InputArray* probImage, cv::Rect* window, cv::TermCriteria* criteria, Result<cv::RotatedRect>* ocvrs_return) {
		try {
			cv::RotatedRect ret = cv::CamShift(*probImage, *window, *criteria);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::buildOpticalFlowPyramid(InputArray, OutputArray, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:121
	// ("cv::buildOpticalFlowPyramid", vec![(pred!(mut, ["img", "pyramid", "winSize", "maxLevel"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::Size", "int"]), _)]),
	void cv_buildOpticalFlowPyramid_const__InputArrayR_const__OutputArrayR_Size_int(const cv::_InputArray* img, const cv::_OutputArray* pyramid, cv::Size* winSize, int maxLevel, Result<int>* ocvrs_return) {
		try {
			int ret = cv::buildOpticalFlowPyramid(*img, *pyramid, *winSize, maxLevel);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// buildOpticalFlowPyramid(InputArray, OutputArrayOfArrays, Size, int, bool, int, int, bool)(InputArray, OutputArray, SimpleClass, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:121
	// ("cv::buildOpticalFlowPyramid", vec![(pred!(mut, ["img", "pyramid", "winSize", "maxLevel", "withDerivatives", "pyrBorder", "derivBorder", "tryReuseInputImage"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::Size", "int", "bool", "int", "int", "bool"]), _)]),
	void cv_buildOpticalFlowPyramid_const__InputArrayR_const__OutputArrayR_Size_int_bool_int_int_bool(const cv::_InputArray* img, const cv::_OutputArray* pyramid, cv::Size* winSize, int maxLevel, bool withDerivatives, int pyrBorder, int derivBorder, bool tryReuseInputImage, Result<int>* ocvrs_return) {
		try {
			int ret = cv::buildOpticalFlowPyramid(*img, *pyramid, *winSize, maxLevel, withDerivatives, pyrBorder, derivBorder, tryReuseInputImage);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// calcOpticalFlowFarneback(InputArray, InputArray, InputOutputArray, double, int, int, int, int, double, int)(InputArray, InputArray, InputOutputArray, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:223
	// ("cv::calcOpticalFlowFarneback", vec![(pred!(mut, ["prev", "next", "flow", "pyr_scale", "levels", "winsize", "iterations", "poly_n", "poly_sigma", "flags"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "double", "int", "int", "int", "int", "double", "int"]), _)]),
	void cv_calcOpticalFlowFarneback_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_double_int_int_int_int_double_int(const cv::_InputArray* prev, const cv::_InputArray* next, const cv::_InputOutputArray* flow, double pyr_scale, int levels, int winsize, int iterations, int poly_n, double poly_sigma, int flags, ResultVoid* ocvrs_return) {
		try {
			cv::calcOpticalFlowFarneback(*prev, *next, *flow, pyr_scale, levels, winsize, iterations, poly_n, poly_sigma, flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::calcOpticalFlowPyrLK(InputArray, InputArray, InputArray, InputOutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:178
	// ("cv::calcOpticalFlowPyrLK", vec![(pred!(mut, ["prevImg", "nextImg", "prevPts", "nextPts", "status", "err"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_calcOpticalFlowPyrLK_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* prevImg, const cv::_InputArray* nextImg, const cv::_InputArray* prevPts, const cv::_InputOutputArray* nextPts, const cv::_OutputArray* status, const cv::_OutputArray* err, ResultVoid* ocvrs_return) {
		try {
			cv::calcOpticalFlowPyrLK(*prevImg, *nextImg, *prevPts, *nextPts, *status, *err);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// calcOpticalFlowPyrLK(InputArray, InputArray, InputArray, InputOutputArray, OutputArray, OutputArray, Size, int, TermCriteria, int, double)(InputArray, InputArray, InputArray, InputOutputArray, OutputArray, OutputArray, SimpleClass, Primitive, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:178
	// ("cv::calcOpticalFlowPyrLK", vec![(pred!(mut, ["prevImg", "nextImg", "prevPts", "nextPts", "status", "err", "winSize", "maxLevel", "criteria", "flags", "minEigThreshold"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "cv::Size", "int", "cv::TermCriteria", "int", "double"]), _)]),
	void cv_calcOpticalFlowPyrLK_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_Size_int_TermCriteria_int_double(const cv::_InputArray* prevImg, const cv::_InputArray* nextImg, const cv::_InputArray* prevPts, const cv::_InputOutputArray* nextPts, const cv::_OutputArray* status, const cv::_OutputArray* err, cv::Size* winSize, int maxLevel, cv::TermCriteria* criteria, int flags, double minEigThreshold, ResultVoid* ocvrs_return) {
		try {
			cv::calcOpticalFlowPyrLK(*prevImg, *nextImg, *prevPts, *nextPts, *status, *err, *winSize, maxLevel, *criteria, flags, minEigThreshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::computeECC(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:279
	// ("cv::computeECC", vec![(pred!(mut, ["templateImage", "inputImage"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_computeECC_const__InputArrayR_const__InputArrayR(const cv::_InputArray* templateImage, const cv::_InputArray* inputImage, Result<double>* ocvrs_return) {
		try {
			double ret = cv::computeECC(*templateImage, *inputImage);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// computeECC(InputArray, InputArray, InputArray)(InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:279
	// ("cv::computeECC", vec![(pred!(mut, ["templateImage", "inputImage", "inputMask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_computeECC_const__InputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputArray* templateImage, const cv::_InputArray* inputImage, const cv::_InputArray* inputMask, Result<double>* ocvrs_return) {
		try {
			double ret = cv::computeECC(*templateImage, *inputImage, *inputMask);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::createBackgroundSubtractorKNN() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:310
	// ("cv::createBackgroundSubtractorKNN", vec![(pred!(mut, [], []), _)]),
	void cv_createBackgroundSubtractorKNN(Result<cv::Ptr<cv::BackgroundSubtractorKNN>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::BackgroundSubtractorKNN> ret = cv::createBackgroundSubtractorKNN();
			Ok(new cv::Ptr<cv::BackgroundSubtractorKNN>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createBackgroundSubtractorKNN(int, double, bool)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:310
	// ("cv::createBackgroundSubtractorKNN", vec![(pred!(mut, ["history", "dist2Threshold", "detectShadows"], ["int", "double", "bool"]), _)]),
	void cv_createBackgroundSubtractorKNN_int_double_bool(int history, double dist2Threshold, bool detectShadows, Result<cv::Ptr<cv::BackgroundSubtractorKNN>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::BackgroundSubtractorKNN> ret = cv::createBackgroundSubtractorKNN(history, dist2Threshold, detectShadows);
			Ok(new cv::Ptr<cv::BackgroundSubtractorKNN>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::createBackgroundSubtractorMOG2() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:221
	// ("cv::createBackgroundSubtractorMOG2", vec![(pred!(mut, [], []), _)]),
	void cv_createBackgroundSubtractorMOG2(Result<cv::Ptr<cv::BackgroundSubtractorMOG2>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::BackgroundSubtractorMOG2> ret = cv::createBackgroundSubtractorMOG2();
			Ok(new cv::Ptr<cv::BackgroundSubtractorMOG2>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createBackgroundSubtractorMOG2(int, double, bool)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:221
	// ("cv::createBackgroundSubtractorMOG2", vec![(pred!(mut, ["history", "varThreshold", "detectShadows"], ["int", "double", "bool"]), _)]),
	void cv_createBackgroundSubtractorMOG2_int_double_bool(int history, double varThreshold, bool detectShadows, Result<cv::Ptr<cv::BackgroundSubtractorMOG2>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::BackgroundSubtractorMOG2> ret = cv::createBackgroundSubtractorMOG2(history, varThreshold, detectShadows);
			Ok(new cv::Ptr<cv::BackgroundSubtractorMOG2>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createOptFlow_DualTVL1()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:573
	// ("cv::createOptFlow_DualTVL1", vec![(pred!(mut, [], []), _)]),
	void cv_createOptFlow_DualTVL1(Result<cv::Ptr<cv::DualTVL1OpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::DualTVL1OpticalFlow> ret = cv::createOptFlow_DualTVL1();
			Ok(new cv::Ptr<cv::DualTVL1OpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// estimateRigidTransform(InputArray, InputArray, bool)(InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:255
	// ("cv::estimateRigidTransform", vec![(pred!(mut, ["src", "dst", "fullAffine"], ["const cv::_InputArray*", "const cv::_InputArray*", "bool"]), _)]),
	void cv_estimateRigidTransform_const__InputArrayR_const__InputArrayR_bool(const cv::_InputArray* src, const cv::_InputArray* dst, bool fullAffine, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::estimateRigidTransform(*src, *dst, fullAffine);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// estimateRigidTransform(InputArray, InputArray, bool, int, double, int)(InputArray, InputArray, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:256
	// ("cv::estimateRigidTransform", vec![(pred!(mut, ["src", "dst", "fullAffine", "ransacMaxIters", "ransacGoodRatio", "ransacSize0"], ["const cv::_InputArray*", "const cv::_InputArray*", "bool", "int", "double", "int"]), _)]),
	void cv_estimateRigidTransform_const__InputArrayR_const__InputArrayR_bool_int_double_int(const cv::_InputArray* src, const cv::_InputArray* dst, bool fullAffine, int ransacMaxIters, double ransacGoodRatio, int ransacSize0, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::estimateRigidTransform(*src, *dst, fullAffine, ransacMaxIters, ransacGoodRatio, ransacSize0);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::findTransformECC(InputArray, InputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:343
	// ("cv::findTransformECC", vec![(pred!(mut, ["templateImage", "inputImage", "warpMatrix"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*"]), _)]),
	void cv_findTransformECC_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR(const cv::_InputArray* templateImage, const cv::_InputArray* inputImage, const cv::_InputOutputArray* warpMatrix, Result<double>* ocvrs_return) {
		try {
			double ret = cv::findTransformECC(*templateImage, *inputImage, *warpMatrix);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// findTransformECC(InputArray, InputArray, InputOutputArray, int, TermCriteria, InputArray)(InputArray, InputArray, InputOutputArray, Primitive, SimpleClass, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:343
	// ("cv::findTransformECC", vec![(pred!(mut, ["templateImage", "inputImage", "warpMatrix", "motionType", "criteria", "inputMask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "int", "cv::TermCriteria", "const cv::_InputArray*"]), _)]),
	void cv_findTransformECC_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_int_TermCriteria_const__InputArrayR(const cv::_InputArray* templateImage, const cv::_InputArray* inputImage, const cv::_InputOutputArray* warpMatrix, int motionType, cv::TermCriteria* criteria, const cv::_InputArray* inputMask, Result<double>* ocvrs_return) {
		try {
			double ret = cv::findTransformECC(*templateImage, *inputImage, *warpMatrix, motionType, *criteria, *inputMask);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// findTransformECC(InputArray, InputArray, InputOutputArray, int, TermCriteria, InputArray, int)(InputArray, InputArray, InputOutputArray, Primitive, SimpleClass, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:336
	// ("cv::findTransformECC", vec![(pred!(mut, ["templateImage", "inputImage", "warpMatrix", "motionType", "criteria", "inputMask", "gaussFiltSize"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "int", "cv::TermCriteria", "const cv::_InputArray*", "int"]), _)]),
	void cv_findTransformECC_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_int_TermCriteria_const__InputArrayR_int(const cv::_InputArray* templateImage, const cv::_InputArray* inputImage, const cv::_InputOutputArray* warpMatrix, int motionType, cv::TermCriteria* criteria, const cv::_InputArray* inputMask, int gaussFiltSize, Result<double>* ocvrs_return) {
		try {
			double ret = cv::findTransformECC(*templateImage, *inputImage, *warpMatrix, motionType, *criteria, *inputMask, gaussFiltSize);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// meanShift(InputArray, Rect &, TermCriteria)(InputArray, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:104
	// ("cv::meanShift", vec![(pred!(mut, ["probImage", "window", "criteria"], ["const cv::_InputArray*", "cv::Rect*", "cv::TermCriteria"]), _)]),
	void cv_meanShift_const__InputArrayR_RectR_TermCriteria(const cv::_InputArray* probImage, cv::Rect* window, cv::TermCriteria* criteria, Result<int>* ocvrs_return) {
		try {
			int ret = cv::meanShift(*probImage, *window, *criteria);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// apply(InputArray, OutputArray, double)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:72
	// ("cv::BackgroundSubtractor::apply", vec![(pred!(mut, ["image", "fgmask", "learningRate"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double"]), _)]),
	void cv_BackgroundSubtractor_apply_const__InputArrayR_const__OutputArrayR_double(cv::BackgroundSubtractor* instance, const cv::_InputArray* image, const cv::_OutputArray* fgmask, double learningRate, ResultVoid* ocvrs_return) {
		try {
			instance->apply(*image, *fgmask, learningRate);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::BackgroundSubtractor::apply(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:72
	// ("cv::BackgroundSubtractor::apply", vec![(pred!(mut, ["image", "fgmask"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_BackgroundSubtractor_apply_const__InputArrayR_const__OutputArrayR(cv::BackgroundSubtractor* instance, const cv::_InputArray* image, const cv::_OutputArray* fgmask, ResultVoid* ocvrs_return) {
		try {
			instance->apply(*image, *fgmask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getBackgroundImage(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:81
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

	// getHistory()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:234
	// ("cv::BackgroundSubtractorKNN::getHistory", vec![(pred!(const, [], []), _)]),
	void cv_BackgroundSubtractorKNN_getHistory_const(const cv::BackgroundSubtractorKNN* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getHistory();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setHistory(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:237
	// ("cv::BackgroundSubtractorKNN::setHistory", vec![(pred!(mut, ["history"], ["int"]), _)]),
	void cv_BackgroundSubtractorKNN_setHistory_int(cv::BackgroundSubtractorKNN* instance, int history, ResultVoid* ocvrs_return) {
		try {
			instance->setHistory(history);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNSamples()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:241
	// ("cv::BackgroundSubtractorKNN::getNSamples", vec![(pred!(const, [], []), _)]),
	void cv_BackgroundSubtractorKNN_getNSamples_const(const cv::BackgroundSubtractorKNN* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNSamples();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNSamples(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:246
	// ("cv::BackgroundSubtractorKNN::setNSamples", vec![(pred!(mut, ["_nN"], ["int"]), _)]),
	void cv_BackgroundSubtractorKNN_setNSamples_int(cv::BackgroundSubtractorKNN* instance, int _nN, ResultVoid* ocvrs_return) {
		try {
			instance->setNSamples(_nN);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDist2Threshold()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:253
	// ("cv::BackgroundSubtractorKNN::getDist2Threshold", vec![(pred!(const, [], []), _)]),
	void cv_BackgroundSubtractorKNN_getDist2Threshold_const(const cv::BackgroundSubtractorKNN* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getDist2Threshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDist2Threshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:256
	// ("cv::BackgroundSubtractorKNN::setDist2Threshold", vec![(pred!(mut, ["_dist2Threshold"], ["double"]), _)]),
	void cv_BackgroundSubtractorKNN_setDist2Threshold_double(cv::BackgroundSubtractorKNN* instance, double _dist2Threshold, ResultVoid* ocvrs_return) {
		try {
			instance->setDist2Threshold(_dist2Threshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getkNNSamples()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:263
	// ("cv::BackgroundSubtractorKNN::getkNNSamples", vec![(pred!(const, [], []), _)]),
	void cv_BackgroundSubtractorKNN_getkNNSamples_const(const cv::BackgroundSubtractorKNN* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getkNNSamples();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setkNNSamples(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:266
	// ("cv::BackgroundSubtractorKNN::setkNNSamples", vec![(pred!(mut, ["_nkNN"], ["int"]), _)]),
	void cv_BackgroundSubtractorKNN_setkNNSamples_int(cv::BackgroundSubtractorKNN* instance, int _nkNN, ResultVoid* ocvrs_return) {
		try {
			instance->setkNNSamples(_nkNN);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDetectShadows()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:273
	// ("cv::BackgroundSubtractorKNN::getDetectShadows", vec![(pred!(const, [], []), _)]),
	void cv_BackgroundSubtractorKNN_getDetectShadows_const(const cv::BackgroundSubtractorKNN* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getDetectShadows();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDetectShadows(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:276
	// ("cv::BackgroundSubtractorKNN::setDetectShadows", vec![(pred!(mut, ["detectShadows"], ["bool"]), _)]),
	void cv_BackgroundSubtractorKNN_setDetectShadows_bool(cv::BackgroundSubtractorKNN* instance, bool detectShadows, ResultVoid* ocvrs_return) {
		try {
			instance->setDetectShadows(detectShadows);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getShadowValue()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:283
	// ("cv::BackgroundSubtractorKNN::getShadowValue", vec![(pred!(const, [], []), _)]),
	void cv_BackgroundSubtractorKNN_getShadowValue_const(const cv::BackgroundSubtractorKNN* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getShadowValue();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setShadowValue(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:286
	// ("cv::BackgroundSubtractorKNN::setShadowValue", vec![(pred!(mut, ["value"], ["int"]), _)]),
	void cv_BackgroundSubtractorKNN_setShadowValue_int(cv::BackgroundSubtractorKNN* instance, int value, ResultVoid* ocvrs_return) {
		try {
			instance->setShadowValue(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getShadowThreshold()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:295
	// ("cv::BackgroundSubtractorKNN::getShadowThreshold", vec![(pred!(const, [], []), _)]),
	void cv_BackgroundSubtractorKNN_getShadowThreshold_const(const cv::BackgroundSubtractorKNN* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getShadowThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setShadowThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:298
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

	// getHistory()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:95
	// ("cv::BackgroundSubtractorMOG2::getHistory", vec![(pred!(const, [], []), _)]),
	void cv_BackgroundSubtractorMOG2_getHistory_const(const cv::BackgroundSubtractorMOG2* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getHistory();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setHistory(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:98
	// ("cv::BackgroundSubtractorMOG2::setHistory", vec![(pred!(mut, ["history"], ["int"]), _)]),
	void cv_BackgroundSubtractorMOG2_setHistory_int(cv::BackgroundSubtractorMOG2* instance, int history, ResultVoid* ocvrs_return) {
		try {
			instance->setHistory(history);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNMixtures()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:102
	// ("cv::BackgroundSubtractorMOG2::getNMixtures", vec![(pred!(const, [], []), _)]),
	void cv_BackgroundSubtractorMOG2_getNMixtures_const(const cv::BackgroundSubtractorMOG2* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNMixtures();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNMixtures(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:107
	// ("cv::BackgroundSubtractorMOG2::setNMixtures", vec![(pred!(mut, ["nmixtures"], ["int"]), _)]),
	void cv_BackgroundSubtractorMOG2_setNMixtures_int(cv::BackgroundSubtractorMOG2* instance, int nmixtures, ResultVoid* ocvrs_return) {
		try {
			instance->setNMixtures(nmixtures);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getBackgroundRatio()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:115
	// ("cv::BackgroundSubtractorMOG2::getBackgroundRatio", vec![(pred!(const, [], []), _)]),
	void cv_BackgroundSubtractorMOG2_getBackgroundRatio_const(const cv::BackgroundSubtractorMOG2* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getBackgroundRatio();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setBackgroundRatio(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:118
	// ("cv::BackgroundSubtractorMOG2::setBackgroundRatio", vec![(pred!(mut, ["ratio"], ["double"]), _)]),
	void cv_BackgroundSubtractorMOG2_setBackgroundRatio_double(cv::BackgroundSubtractorMOG2* instance, double ratio, ResultVoid* ocvrs_return) {
		try {
			instance->setBackgroundRatio(ratio);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getVarThreshold()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:125
	// ("cv::BackgroundSubtractorMOG2::getVarThreshold", vec![(pred!(const, [], []), _)]),
	void cv_BackgroundSubtractorMOG2_getVarThreshold_const(const cv::BackgroundSubtractorMOG2* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getVarThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setVarThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:128
	// ("cv::BackgroundSubtractorMOG2::setVarThreshold", vec![(pred!(mut, ["varThreshold"], ["double"]), _)]),
	void cv_BackgroundSubtractorMOG2_setVarThreshold_double(cv::BackgroundSubtractorMOG2* instance, double varThreshold, ResultVoid* ocvrs_return) {
		try {
			instance->setVarThreshold(varThreshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getVarThresholdGen()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:138
	// ("cv::BackgroundSubtractorMOG2::getVarThresholdGen", vec![(pred!(const, [], []), _)]),
	void cv_BackgroundSubtractorMOG2_getVarThresholdGen_const(const cv::BackgroundSubtractorMOG2* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getVarThresholdGen();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setVarThresholdGen(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:141
	// ("cv::BackgroundSubtractorMOG2::setVarThresholdGen", vec![(pred!(mut, ["varThresholdGen"], ["double"]), _)]),
	void cv_BackgroundSubtractorMOG2_setVarThresholdGen_double(cv::BackgroundSubtractorMOG2* instance, double varThresholdGen, ResultVoid* ocvrs_return) {
		try {
			instance->setVarThresholdGen(varThresholdGen);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getVarInit()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:145
	// ("cv::BackgroundSubtractorMOG2::getVarInit", vec![(pred!(const, [], []), _)]),
	void cv_BackgroundSubtractorMOG2_getVarInit_const(const cv::BackgroundSubtractorMOG2* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getVarInit();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setVarInit(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:148
	// ("cv::BackgroundSubtractorMOG2::setVarInit", vec![(pred!(mut, ["varInit"], ["double"]), _)]),
	void cv_BackgroundSubtractorMOG2_setVarInit_double(cv::BackgroundSubtractorMOG2* instance, double varInit, ResultVoid* ocvrs_return) {
		try {
			instance->setVarInit(varInit);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getVarMin()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:150
	// ("cv::BackgroundSubtractorMOG2::getVarMin", vec![(pred!(const, [], []), _)]),
	void cv_BackgroundSubtractorMOG2_getVarMin_const(const cv::BackgroundSubtractorMOG2* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getVarMin();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setVarMin(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:151
	// ("cv::BackgroundSubtractorMOG2::setVarMin", vec![(pred!(mut, ["varMin"], ["double"]), _)]),
	void cv_BackgroundSubtractorMOG2_setVarMin_double(cv::BackgroundSubtractorMOG2* instance, double varMin, ResultVoid* ocvrs_return) {
		try {
			instance->setVarMin(varMin);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getVarMax()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:153
	// ("cv::BackgroundSubtractorMOG2::getVarMax", vec![(pred!(const, [], []), _)]),
	void cv_BackgroundSubtractorMOG2_getVarMax_const(const cv::BackgroundSubtractorMOG2* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getVarMax();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setVarMax(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:154
	// ("cv::BackgroundSubtractorMOG2::setVarMax", vec![(pred!(mut, ["varMax"], ["double"]), _)]),
	void cv_BackgroundSubtractorMOG2_setVarMax_double(cv::BackgroundSubtractorMOG2* instance, double varMax, ResultVoid* ocvrs_return) {
		try {
			instance->setVarMax(varMax);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getComplexityReductionThreshold()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:162
	// ("cv::BackgroundSubtractorMOG2::getComplexityReductionThreshold", vec![(pred!(const, [], []), _)]),
	void cv_BackgroundSubtractorMOG2_getComplexityReductionThreshold_const(const cv::BackgroundSubtractorMOG2* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getComplexityReductionThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setComplexityReductionThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:165
	// ("cv::BackgroundSubtractorMOG2::setComplexityReductionThreshold", vec![(pred!(mut, ["ct"], ["double"]), _)]),
	void cv_BackgroundSubtractorMOG2_setComplexityReductionThreshold_double(cv::BackgroundSubtractorMOG2* instance, double ct, ResultVoid* ocvrs_return) {
		try {
			instance->setComplexityReductionThreshold(ct);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDetectShadows()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:172
	// ("cv::BackgroundSubtractorMOG2::getDetectShadows", vec![(pred!(const, [], []), _)]),
	void cv_BackgroundSubtractorMOG2_getDetectShadows_const(const cv::BackgroundSubtractorMOG2* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getDetectShadows();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDetectShadows(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:175
	// ("cv::BackgroundSubtractorMOG2::setDetectShadows", vec![(pred!(mut, ["detectShadows"], ["bool"]), _)]),
	void cv_BackgroundSubtractorMOG2_setDetectShadows_bool(cv::BackgroundSubtractorMOG2* instance, bool detectShadows, ResultVoid* ocvrs_return) {
		try {
			instance->setDetectShadows(detectShadows);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getShadowValue()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:182
	// ("cv::BackgroundSubtractorMOG2::getShadowValue", vec![(pred!(const, [], []), _)]),
	void cv_BackgroundSubtractorMOG2_getShadowValue_const(const cv::BackgroundSubtractorMOG2* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getShadowValue();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setShadowValue(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:185
	// ("cv::BackgroundSubtractorMOG2::setShadowValue", vec![(pred!(mut, ["value"], ["int"]), _)]),
	void cv_BackgroundSubtractorMOG2_setShadowValue_int(cv::BackgroundSubtractorMOG2* instance, int value, ResultVoid* ocvrs_return) {
		try {
			instance->setShadowValue(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getShadowThreshold()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:194
	// ("cv::BackgroundSubtractorMOG2::getShadowThreshold", vec![(pred!(const, [], []), _)]),
	void cv_BackgroundSubtractorMOG2_getShadowThreshold_const(const cv::BackgroundSubtractorMOG2* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getShadowThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setShadowThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:197
	// ("cv::BackgroundSubtractorMOG2::setShadowThreshold", vec![(pred!(mut, ["threshold"], ["double"]), _)]),
	void cv_BackgroundSubtractorMOG2_setShadowThreshold_double(cv::BackgroundSubtractorMOG2* instance, double threshold, ResultVoid* ocvrs_return) {
		try {
			instance->setShadowThreshold(threshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// apply(InputArray, OutputArray, double)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:208
	// ("cv::BackgroundSubtractorMOG2::apply", vec![(pred!(mut, ["image", "fgmask", "learningRate"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double"]), _)]),
	void cv_BackgroundSubtractorMOG2_apply_const__InputArrayR_const__OutputArrayR_double(cv::BackgroundSubtractorMOG2* instance, const cv::_InputArray* image, const cv::_OutputArray* fgmask, double learningRate, ResultVoid* ocvrs_return) {
		try {
			instance->apply(*image, *fgmask, learningRate);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::BackgroundSubtractorMOG2::apply(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/background_segm.hpp:208
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

	// calc(InputArray, InputArray, InputOutputArray)(InputArray, InputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:422
	// ("cv::DenseOpticalFlow::calc", vec![(pred!(mut, ["I0", "I1", "flow"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*"]), _)]),
	void cv_DenseOpticalFlow_calc_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR(cv::DenseOpticalFlow* instance, const cv::_InputArray* I0, const cv::_InputArray* I1, const cv::_InputOutputArray* flow, ResultVoid* ocvrs_return) {
		try {
			instance->calc(*I0, *I1, *flow);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// collectGarbage()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:425
	// ("cv::DenseOpticalFlow::collectGarbage", vec![(pred!(mut, [], []), _)]),
	void cv_DenseOpticalFlow_collectGarbage(cv::DenseOpticalFlow* instance, ResultVoid* ocvrs_return) {
		try {
			instance->collectGarbage();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::DenseOpticalFlow::to_DualTVL1OpticalFlow() generated
	// ("cv::DenseOpticalFlow::to_DualTVL1OpticalFlow", vec![(pred!(mut, [], []), _)]),
	cv::DualTVL1OpticalFlow* cv_DenseOpticalFlow_to_DualTVL1OpticalFlow(cv::DenseOpticalFlow* instance) {
			return dynamic_cast<cv::DualTVL1OpticalFlow*>(instance);
	}

	// cv::DenseOpticalFlow::to_FarnebackOpticalFlow() generated
	// ("cv::DenseOpticalFlow::to_FarnebackOpticalFlow", vec![(pred!(mut, [], []), _)]),
	cv::FarnebackOpticalFlow* cv_DenseOpticalFlow_to_FarnebackOpticalFlow(cv::DenseOpticalFlow* instance) {
			return dynamic_cast<cv::FarnebackOpticalFlow*>(instance);
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

	// getTau()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:496
	// ("cv::DualTVL1OpticalFlow::getTau", vec![(pred!(const, [], []), _)]),
	void cv_DualTVL1OpticalFlow_getTau_const(const cv::DualTVL1OpticalFlow* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getTau();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTau(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:498
	// ("cv::DualTVL1OpticalFlow::setTau", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_DualTVL1OpticalFlow_setTau_double(cv::DualTVL1OpticalFlow* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setTau(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLambda()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:501
	// ("cv::DualTVL1OpticalFlow::getLambda", vec![(pred!(const, [], []), _)]),
	void cv_DualTVL1OpticalFlow_getLambda_const(const cv::DualTVL1OpticalFlow* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getLambda();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setLambda(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:503
	// ("cv::DualTVL1OpticalFlow::setLambda", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_DualTVL1OpticalFlow_setLambda_double(cv::DualTVL1OpticalFlow* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setLambda(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTheta()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:506
	// ("cv::DualTVL1OpticalFlow::getTheta", vec![(pred!(const, [], []), _)]),
	void cv_DualTVL1OpticalFlow_getTheta_const(const cv::DualTVL1OpticalFlow* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getTheta();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTheta(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:508
	// ("cv::DualTVL1OpticalFlow::setTheta", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_DualTVL1OpticalFlow_setTheta_double(cv::DualTVL1OpticalFlow* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setTheta(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getGamma()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:511
	// ("cv::DualTVL1OpticalFlow::getGamma", vec![(pred!(const, [], []), _)]),
	void cv_DualTVL1OpticalFlow_getGamma_const(const cv::DualTVL1OpticalFlow* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getGamma();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setGamma(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:513
	// ("cv::DualTVL1OpticalFlow::setGamma", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_DualTVL1OpticalFlow_setGamma_double(cv::DualTVL1OpticalFlow* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setGamma(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getScalesNumber()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:516
	// ("cv::DualTVL1OpticalFlow::getScalesNumber", vec![(pred!(const, [], []), _)]),
	void cv_DualTVL1OpticalFlow_getScalesNumber_const(const cv::DualTVL1OpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getScalesNumber();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setScalesNumber(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:518
	// ("cv::DualTVL1OpticalFlow::setScalesNumber", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_DualTVL1OpticalFlow_setScalesNumber_int(cv::DualTVL1OpticalFlow* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setScalesNumber(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getWarpingsNumber()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:521
	// ("cv::DualTVL1OpticalFlow::getWarpingsNumber", vec![(pred!(const, [], []), _)]),
	void cv_DualTVL1OpticalFlow_getWarpingsNumber_const(const cv::DualTVL1OpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getWarpingsNumber();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWarpingsNumber(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:523
	// ("cv::DualTVL1OpticalFlow::setWarpingsNumber", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_DualTVL1OpticalFlow_setWarpingsNumber_int(cv::DualTVL1OpticalFlow* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setWarpingsNumber(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getEpsilon()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:526
	// ("cv::DualTVL1OpticalFlow::getEpsilon", vec![(pred!(const, [], []), _)]),
	void cv_DualTVL1OpticalFlow_getEpsilon_const(const cv::DualTVL1OpticalFlow* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getEpsilon();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setEpsilon(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:528
	// ("cv::DualTVL1OpticalFlow::setEpsilon", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_DualTVL1OpticalFlow_setEpsilon_double(cv::DualTVL1OpticalFlow* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setEpsilon(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getInnerIterations()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:531
	// ("cv::DualTVL1OpticalFlow::getInnerIterations", vec![(pred!(const, [], []), _)]),
	void cv_DualTVL1OpticalFlow_getInnerIterations_const(const cv::DualTVL1OpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getInnerIterations();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setInnerIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:533
	// ("cv::DualTVL1OpticalFlow::setInnerIterations", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_DualTVL1OpticalFlow_setInnerIterations_int(cv::DualTVL1OpticalFlow* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setInnerIterations(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getOuterIterations()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:536
	// ("cv::DualTVL1OpticalFlow::getOuterIterations", vec![(pred!(const, [], []), _)]),
	void cv_DualTVL1OpticalFlow_getOuterIterations_const(const cv::DualTVL1OpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getOuterIterations();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setOuterIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:538
	// ("cv::DualTVL1OpticalFlow::setOuterIterations", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_DualTVL1OpticalFlow_setOuterIterations_int(cv::DualTVL1OpticalFlow* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setOuterIterations(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getUseInitialFlow()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:541
	// ("cv::DualTVL1OpticalFlow::getUseInitialFlow", vec![(pred!(const, [], []), _)]),
	void cv_DualTVL1OpticalFlow_getUseInitialFlow_const(const cv::DualTVL1OpticalFlow* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUseInitialFlow();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setUseInitialFlow(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:543
	// ("cv::DualTVL1OpticalFlow::setUseInitialFlow", vec![(pred!(mut, ["val"], ["bool"]), _)]),
	void cv_DualTVL1OpticalFlow_setUseInitialFlow_bool(cv::DualTVL1OpticalFlow* instance, bool val, ResultVoid* ocvrs_return) {
		try {
			instance->setUseInitialFlow(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getScaleStep()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:546
	// ("cv::DualTVL1OpticalFlow::getScaleStep", vec![(pred!(const, [], []), _)]),
	void cv_DualTVL1OpticalFlow_getScaleStep_const(const cv::DualTVL1OpticalFlow* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getScaleStep();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setScaleStep(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:548
	// ("cv::DualTVL1OpticalFlow::setScaleStep", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_DualTVL1OpticalFlow_setScaleStep_double(cv::DualTVL1OpticalFlow* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setScaleStep(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMedianFiltering()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:551
	// ("cv::DualTVL1OpticalFlow::getMedianFiltering", vec![(pred!(const, [], []), _)]),
	void cv_DualTVL1OpticalFlow_getMedianFiltering_const(const cv::DualTVL1OpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMedianFiltering();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMedianFiltering(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:553
	// ("cv::DualTVL1OpticalFlow::setMedianFiltering", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_DualTVL1OpticalFlow_setMedianFiltering_int(cv::DualTVL1OpticalFlow* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setMedianFiltering(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(double, double, double, int, int, double, int, int, double, double, int, bool)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:556
	// ("cv::DualTVL1OpticalFlow::create", vec![(pred!(mut, ["tau", "lambda", "theta", "nscales", "warps", "epsilon", "innnerIterations", "outerIterations", "scaleStep", "gamma", "medianFiltering", "useInitialFlow"], ["double", "double", "double", "int", "int", "double", "int", "int", "double", "double", "int", "bool"]), _)]),
	void cv_DualTVL1OpticalFlow_create_double_double_double_int_int_double_int_int_double_double_int_bool(double tau, double lambda, double theta, int nscales, int warps, double epsilon, int innnerIterations, int outerIterations, double scaleStep, double gamma, int medianFiltering, bool useInitialFlow, Result<cv::Ptr<cv::DualTVL1OpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::DualTVL1OpticalFlow> ret = cv::DualTVL1OpticalFlow::create(tau, lambda, theta, nscales, warps, epsilon, innnerIterations, outerIterations, scaleStep, gamma, medianFiltering, useInitialFlow);
			Ok(new cv::Ptr<cv::DualTVL1OpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::DualTVL1OpticalFlow::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:556
	// ("cv::DualTVL1OpticalFlow::create", vec![(pred!(mut, [], []), _)]),
	void cv_DualTVL1OpticalFlow_create(Result<cv::Ptr<cv::DualTVL1OpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::DualTVL1OpticalFlow> ret = cv::DualTVL1OpticalFlow::create();
			Ok(new cv::Ptr<cv::DualTVL1OpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::DualTVL1OpticalFlow::to_Algorithm() generated
	// ("cv::DualTVL1OpticalFlow::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_DualTVL1OpticalFlow_to_Algorithm(cv::DualTVL1OpticalFlow* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::DualTVL1OpticalFlow::to_DenseOpticalFlow() generated
	// ("cv::DualTVL1OpticalFlow::to_DenseOpticalFlow", vec![(pred!(mut, [], []), _)]),
	cv::DenseOpticalFlow* cv_DualTVL1OpticalFlow_to_DenseOpticalFlow(cv::DualTVL1OpticalFlow* instance) {
			return dynamic_cast<cv::DenseOpticalFlow*>(instance);
	}

	// cv::DualTVL1OpticalFlow::delete() generated
	// ("cv::DualTVL1OpticalFlow::delete", vec![(pred!(mut, [], []), _)]),
	void cv_DualTVL1OpticalFlow_delete(cv::DualTVL1OpticalFlow* instance) {
			delete instance;
	}

	// getNumLevels()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:580
	// ("cv::FarnebackOpticalFlow::getNumLevels", vec![(pred!(const, [], []), _)]),
	void cv_FarnebackOpticalFlow_getNumLevels_const(const cv::FarnebackOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNumLevels();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNumLevels(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:581
	// ("cv::FarnebackOpticalFlow::setNumLevels", vec![(pred!(mut, ["numLevels"], ["int"]), _)]),
	void cv_FarnebackOpticalFlow_setNumLevels_int(cv::FarnebackOpticalFlow* instance, int numLevels, ResultVoid* ocvrs_return) {
		try {
			instance->setNumLevels(numLevels);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPyrScale()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:583
	// ("cv::FarnebackOpticalFlow::getPyrScale", vec![(pred!(const, [], []), _)]),
	void cv_FarnebackOpticalFlow_getPyrScale_const(const cv::FarnebackOpticalFlow* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getPyrScale();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setPyrScale(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:584
	// ("cv::FarnebackOpticalFlow::setPyrScale", vec![(pred!(mut, ["pyrScale"], ["double"]), _)]),
	void cv_FarnebackOpticalFlow_setPyrScale_double(cv::FarnebackOpticalFlow* instance, double pyrScale, ResultVoid* ocvrs_return) {
		try {
			instance->setPyrScale(pyrScale);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getFastPyramids()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:586
	// ("cv::FarnebackOpticalFlow::getFastPyramids", vec![(pred!(const, [], []), _)]),
	void cv_FarnebackOpticalFlow_getFastPyramids_const(const cv::FarnebackOpticalFlow* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getFastPyramids();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setFastPyramids(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:587
	// ("cv::FarnebackOpticalFlow::setFastPyramids", vec![(pred!(mut, ["fastPyramids"], ["bool"]), _)]),
	void cv_FarnebackOpticalFlow_setFastPyramids_bool(cv::FarnebackOpticalFlow* instance, bool fastPyramids, ResultVoid* ocvrs_return) {
		try {
			instance->setFastPyramids(fastPyramids);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getWinSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:589
	// ("cv::FarnebackOpticalFlow::getWinSize", vec![(pred!(const, [], []), _)]),
	void cv_FarnebackOpticalFlow_getWinSize_const(const cv::FarnebackOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getWinSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWinSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:590
	// ("cv::FarnebackOpticalFlow::setWinSize", vec![(pred!(mut, ["winSize"], ["int"]), _)]),
	void cv_FarnebackOpticalFlow_setWinSize_int(cv::FarnebackOpticalFlow* instance, int winSize, ResultVoid* ocvrs_return) {
		try {
			instance->setWinSize(winSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNumIters()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:592
	// ("cv::FarnebackOpticalFlow::getNumIters", vec![(pred!(const, [], []), _)]),
	void cv_FarnebackOpticalFlow_getNumIters_const(const cv::FarnebackOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNumIters();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNumIters(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:593
	// ("cv::FarnebackOpticalFlow::setNumIters", vec![(pred!(mut, ["numIters"], ["int"]), _)]),
	void cv_FarnebackOpticalFlow_setNumIters_int(cv::FarnebackOpticalFlow* instance, int numIters, ResultVoid* ocvrs_return) {
		try {
			instance->setNumIters(numIters);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPolyN()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:595
	// ("cv::FarnebackOpticalFlow::getPolyN", vec![(pred!(const, [], []), _)]),
	void cv_FarnebackOpticalFlow_getPolyN_const(const cv::FarnebackOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getPolyN();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setPolyN(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:596
	// ("cv::FarnebackOpticalFlow::setPolyN", vec![(pred!(mut, ["polyN"], ["int"]), _)]),
	void cv_FarnebackOpticalFlow_setPolyN_int(cv::FarnebackOpticalFlow* instance, int polyN, ResultVoid* ocvrs_return) {
		try {
			instance->setPolyN(polyN);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPolySigma()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:598
	// ("cv::FarnebackOpticalFlow::getPolySigma", vec![(pred!(const, [], []), _)]),
	void cv_FarnebackOpticalFlow_getPolySigma_const(const cv::FarnebackOpticalFlow* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getPolySigma();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setPolySigma(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:599
	// ("cv::FarnebackOpticalFlow::setPolySigma", vec![(pred!(mut, ["polySigma"], ["double"]), _)]),
	void cv_FarnebackOpticalFlow_setPolySigma_double(cv::FarnebackOpticalFlow* instance, double polySigma, ResultVoid* ocvrs_return) {
		try {
			instance->setPolySigma(polySigma);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getFlags()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:601
	// ("cv::FarnebackOpticalFlow::getFlags", vec![(pred!(const, [], []), _)]),
	void cv_FarnebackOpticalFlow_getFlags_const(const cv::FarnebackOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getFlags();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setFlags(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:602
	// ("cv::FarnebackOpticalFlow::setFlags", vec![(pred!(mut, ["flags"], ["int"]), _)]),
	void cv_FarnebackOpticalFlow_setFlags_int(cv::FarnebackOpticalFlow* instance, int flags, ResultVoid* ocvrs_return) {
		try {
			instance->setFlags(flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(int, double, bool, int, int, int, double, int)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:604
	// ("cv::FarnebackOpticalFlow::create", vec![(pred!(mut, ["numLevels", "pyrScale", "fastPyramids", "winSize", "numIters", "polyN", "polySigma", "flags"], ["int", "double", "bool", "int", "int", "int", "double", "int"]), _)]),
	void cv_FarnebackOpticalFlow_create_int_double_bool_int_int_int_double_int(int numLevels, double pyrScale, bool fastPyramids, int winSize, int numIters, int polyN, double polySigma, int flags, Result<cv::Ptr<cv::FarnebackOpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::FarnebackOpticalFlow> ret = cv::FarnebackOpticalFlow::create(numLevels, pyrScale, fastPyramids, winSize, numIters, polyN, polySigma, flags);
			Ok(new cv::Ptr<cv::FarnebackOpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::FarnebackOpticalFlow::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:604
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

	// KalmanFilter()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:363
	// ("cv::KalmanFilter::KalmanFilter", vec![(pred!(mut, [], []), _)]),
	void cv_KalmanFilter_KalmanFilter(Result<cv::KalmanFilter*>* ocvrs_return) {
		try {
			cv::KalmanFilter* ret = new cv::KalmanFilter();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// KalmanFilter(int, int, int, int)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:370
	// ("cv::KalmanFilter::KalmanFilter", vec![(pred!(mut, ["dynamParams", "measureParams", "controlParams", "type"], ["int", "int", "int", "int"]), _)]),
	void cv_KalmanFilter_KalmanFilter_int_int_int_int(int dynamParams, int measureParams, int controlParams, int type, Result<cv::KalmanFilter*>* ocvrs_return) {
		try {
			cv::KalmanFilter* ret = new cv::KalmanFilter(dynamParams, measureParams, controlParams, type);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::KalmanFilter::KalmanFilter(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:370
	// ("cv::KalmanFilter::KalmanFilter", vec![(pred!(mut, ["dynamParams", "measureParams"], ["int", "int"]), _)]),
	void cv_KalmanFilter_KalmanFilter_int_int(int dynamParams, int measureParams, Result<cv::KalmanFilter*>* ocvrs_return) {
		try {
			cv::KalmanFilter* ret = new cv::KalmanFilter(dynamParams, measureParams);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// init(int, int, int, int)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:379
	// ("cv::KalmanFilter::init", vec![(pred!(mut, ["dynamParams", "measureParams", "controlParams", "type"], ["int", "int", "int", "int"]), _)]),
	void cv_KalmanFilter_init_int_int_int_int(cv::KalmanFilter* instance, int dynamParams, int measureParams, int controlParams, int type, ResultVoid* ocvrs_return) {
		try {
			instance->init(dynamParams, measureParams, controlParams, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::KalmanFilter::init(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:379
	// ("cv::KalmanFilter::init", vec![(pred!(mut, ["dynamParams", "measureParams"], ["int", "int"]), _)]),
	void cv_KalmanFilter_init_int_int(cv::KalmanFilter* instance, int dynamParams, int measureParams, ResultVoid* ocvrs_return) {
		try {
			instance->init(dynamParams, measureParams);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// predict(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:385
	// ("cv::KalmanFilter::predict", vec![(pred!(mut, ["control"], ["const cv::Mat*"]), _)]),
	void cv_KalmanFilter_predict_const_MatR(cv::KalmanFilter* instance, const cv::Mat* control, Result<cv::Mat*>* ocvrs_return) {
		try {
			const cv::Mat ret = instance->predict(*control);
			Ok(new const cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::KalmanFilter::predict() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:385
	// ("cv::KalmanFilter::predict", vec![(pred!(mut, [], []), _)]),
	void cv_KalmanFilter_predict(cv::KalmanFilter* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			const cv::Mat ret = instance->predict();
			Ok(new const cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// correct(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:391
	// ("cv::KalmanFilter::correct", vec![(pred!(mut, ["measurement"], ["const cv::Mat*"]), _)]),
	void cv_KalmanFilter_correct_const_MatR(cv::KalmanFilter* instance, const cv::Mat* measurement, Result<cv::Mat*>* ocvrs_return) {
		try {
			const cv::Mat ret = instance->correct(*measurement);
			Ok(new const cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::KalmanFilter::statePre() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:393
	// ("cv::KalmanFilter::statePre", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_KalmanFilter_propStatePre_const(const cv::KalmanFilter* instance) {
			cv::Mat ret = instance->statePre;
			return new cv::Mat(ret);
	}

	// cv::KalmanFilter::setStatePre(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:393
	// ("cv::KalmanFilter::setStatePre", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_KalmanFilter_propStatePre_const_Mat(cv::KalmanFilter* instance, const cv::Mat* val) {
			instance->statePre = *val;
	}

	// cv::KalmanFilter::statePost() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:394
	// ("cv::KalmanFilter::statePost", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_KalmanFilter_propStatePost_const(const cv::KalmanFilter* instance) {
			cv::Mat ret = instance->statePost;
			return new cv::Mat(ret);
	}

	// cv::KalmanFilter::setStatePost(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:394
	// ("cv::KalmanFilter::setStatePost", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_KalmanFilter_propStatePost_const_Mat(cv::KalmanFilter* instance, const cv::Mat* val) {
			instance->statePost = *val;
	}

	// cv::KalmanFilter::transitionMatrix() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:395
	// ("cv::KalmanFilter::transitionMatrix", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_KalmanFilter_propTransitionMatrix_const(const cv::KalmanFilter* instance) {
			cv::Mat ret = instance->transitionMatrix;
			return new cv::Mat(ret);
	}

	// cv::KalmanFilter::setTransitionMatrix(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:395
	// ("cv::KalmanFilter::setTransitionMatrix", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_KalmanFilter_propTransitionMatrix_const_Mat(cv::KalmanFilter* instance, const cv::Mat* val) {
			instance->transitionMatrix = *val;
	}

	// cv::KalmanFilter::controlMatrix() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:396
	// ("cv::KalmanFilter::controlMatrix", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_KalmanFilter_propControlMatrix_const(const cv::KalmanFilter* instance) {
			cv::Mat ret = instance->controlMatrix;
			return new cv::Mat(ret);
	}

	// cv::KalmanFilter::setControlMatrix(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:396
	// ("cv::KalmanFilter::setControlMatrix", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_KalmanFilter_propControlMatrix_const_Mat(cv::KalmanFilter* instance, const cv::Mat* val) {
			instance->controlMatrix = *val;
	}

	// cv::KalmanFilter::measurementMatrix() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:397
	// ("cv::KalmanFilter::measurementMatrix", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_KalmanFilter_propMeasurementMatrix_const(const cv::KalmanFilter* instance) {
			cv::Mat ret = instance->measurementMatrix;
			return new cv::Mat(ret);
	}

	// cv::KalmanFilter::setMeasurementMatrix(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:397
	// ("cv::KalmanFilter::setMeasurementMatrix", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_KalmanFilter_propMeasurementMatrix_const_Mat(cv::KalmanFilter* instance, const cv::Mat* val) {
			instance->measurementMatrix = *val;
	}

	// cv::KalmanFilter::processNoiseCov() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:398
	// ("cv::KalmanFilter::processNoiseCov", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_KalmanFilter_propProcessNoiseCov_const(const cv::KalmanFilter* instance) {
			cv::Mat ret = instance->processNoiseCov;
			return new cv::Mat(ret);
	}

	// cv::KalmanFilter::setProcessNoiseCov(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:398
	// ("cv::KalmanFilter::setProcessNoiseCov", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_KalmanFilter_propProcessNoiseCov_const_Mat(cv::KalmanFilter* instance, const cv::Mat* val) {
			instance->processNoiseCov = *val;
	}

	// cv::KalmanFilter::measurementNoiseCov() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:399
	// ("cv::KalmanFilter::measurementNoiseCov", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_KalmanFilter_propMeasurementNoiseCov_const(const cv::KalmanFilter* instance) {
			cv::Mat ret = instance->measurementNoiseCov;
			return new cv::Mat(ret);
	}

	// cv::KalmanFilter::setMeasurementNoiseCov(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:399
	// ("cv::KalmanFilter::setMeasurementNoiseCov", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_KalmanFilter_propMeasurementNoiseCov_const_Mat(cv::KalmanFilter* instance, const cv::Mat* val) {
			instance->measurementNoiseCov = *val;
	}

	// cv::KalmanFilter::errorCovPre() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:400
	// ("cv::KalmanFilter::errorCovPre", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_KalmanFilter_propErrorCovPre_const(const cv::KalmanFilter* instance) {
			cv::Mat ret = instance->errorCovPre;
			return new cv::Mat(ret);
	}

	// cv::KalmanFilter::setErrorCovPre(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:400
	// ("cv::KalmanFilter::setErrorCovPre", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_KalmanFilter_propErrorCovPre_const_Mat(cv::KalmanFilter* instance, const cv::Mat* val) {
			instance->errorCovPre = *val;
	}

	// cv::KalmanFilter::gain() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:401
	// ("cv::KalmanFilter::gain", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_KalmanFilter_propGain_const(const cv::KalmanFilter* instance) {
			cv::Mat ret = instance->gain;
			return new cv::Mat(ret);
	}

	// cv::KalmanFilter::setGain(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:401
	// ("cv::KalmanFilter::setGain", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_KalmanFilter_propGain_const_Mat(cv::KalmanFilter* instance, const cv::Mat* val) {
			instance->gain = *val;
	}

	// cv::KalmanFilter::errorCovPost() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:402
	// ("cv::KalmanFilter::errorCovPost", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_KalmanFilter_propErrorCovPost_const(const cv::KalmanFilter* instance) {
			cv::Mat ret = instance->errorCovPost;
			return new cv::Mat(ret);
	}

	// cv::KalmanFilter::setErrorCovPost(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:402
	// ("cv::KalmanFilter::setErrorCovPost", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_KalmanFilter_propErrorCovPost_const_Mat(cv::KalmanFilter* instance, const cv::Mat* val) {
			instance->errorCovPost = *val;
	}

	// cv::KalmanFilter::temp1() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:405
	// ("cv::KalmanFilter::temp1", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_KalmanFilter_propTemp1_const(const cv::KalmanFilter* instance) {
			cv::Mat ret = instance->temp1;
			return new cv::Mat(ret);
	}

	// cv::KalmanFilter::setTemp1(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:405
	// ("cv::KalmanFilter::setTemp1", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_KalmanFilter_propTemp1_const_Mat(cv::KalmanFilter* instance, const cv::Mat* val) {
			instance->temp1 = *val;
	}

	// cv::KalmanFilter::temp2() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:406
	// ("cv::KalmanFilter::temp2", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_KalmanFilter_propTemp2_const(const cv::KalmanFilter* instance) {
			cv::Mat ret = instance->temp2;
			return new cv::Mat(ret);
	}

	// cv::KalmanFilter::setTemp2(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:406
	// ("cv::KalmanFilter::setTemp2", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_KalmanFilter_propTemp2_const_Mat(cv::KalmanFilter* instance, const cv::Mat* val) {
			instance->temp2 = *val;
	}

	// cv::KalmanFilter::temp3() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:407
	// ("cv::KalmanFilter::temp3", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_KalmanFilter_propTemp3_const(const cv::KalmanFilter* instance) {
			cv::Mat ret = instance->temp3;
			return new cv::Mat(ret);
	}

	// cv::KalmanFilter::setTemp3(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:407
	// ("cv::KalmanFilter::setTemp3", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_KalmanFilter_propTemp3_const_Mat(cv::KalmanFilter* instance, const cv::Mat* val) {
			instance->temp3 = *val;
	}

	// cv::KalmanFilter::temp4() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:408
	// ("cv::KalmanFilter::temp4", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_KalmanFilter_propTemp4_const(const cv::KalmanFilter* instance) {
			cv::Mat ret = instance->temp4;
			return new cv::Mat(ret);
	}

	// cv::KalmanFilter::setTemp4(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:408
	// ("cv::KalmanFilter::setTemp4", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_KalmanFilter_propTemp4_const_Mat(cv::KalmanFilter* instance, const cv::Mat* val) {
			instance->temp4 = *val;
	}

	// cv::KalmanFilter::temp5() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:409
	// ("cv::KalmanFilter::temp5", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_KalmanFilter_propTemp5_const(const cv::KalmanFilter* instance) {
			cv::Mat ret = instance->temp5;
			return new cv::Mat(ret);
	}

	// cv::KalmanFilter::setTemp5(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:409
	// ("cv::KalmanFilter::setTemp5", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_KalmanFilter_propTemp5_const_Mat(cv::KalmanFilter* instance, const cv::Mat* val) {
			instance->temp5 = *val;
	}

	// cv::KalmanFilter::delete() generated
	// ("cv::KalmanFilter::delete", vec![(pred!(mut, [], []), _)]),
	void cv_KalmanFilter_delete(cv::KalmanFilter* instance) {
			delete instance;
	}

	// calc(InputArray, InputArray, InputArray, InputOutputArray, OutputArray, OutputArray)(InputArray, InputArray, InputArray, InputOutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:443
	// ("cv::SparseOpticalFlow::calc", vec![(pred!(mut, ["prevImg", "nextImg", "prevPts", "nextPts", "status", "err"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_SparseOpticalFlow_calc_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR(cv::SparseOpticalFlow* instance, const cv::_InputArray* prevImg, const cv::_InputArray* nextImg, const cv::_InputArray* prevPts, const cv::_InputOutputArray* nextPts, const cv::_OutputArray* status, const cv::_OutputArray* err, ResultVoid* ocvrs_return) {
		try {
			instance->calc(*prevImg, *nextImg, *prevPts, *nextPts, *status, *err);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::SparseOpticalFlow::calc(InputArray, InputArray, InputArray, InputOutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:443
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

	// getWinSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:627
	// ("cv::SparsePyrLKOpticalFlow::getWinSize", vec![(pred!(const, [], []), _)]),
	void cv_SparsePyrLKOpticalFlow_getWinSize_const(const cv::SparsePyrLKOpticalFlow* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->getWinSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWinSize(Size)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:628
	// ("cv::SparsePyrLKOpticalFlow::setWinSize", vec![(pred!(mut, ["winSize"], ["cv::Size"]), _)]),
	void cv_SparsePyrLKOpticalFlow_setWinSize_Size(cv::SparsePyrLKOpticalFlow* instance, cv::Size* winSize, ResultVoid* ocvrs_return) {
		try {
			instance->setWinSize(*winSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxLevel()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:630
	// ("cv::SparsePyrLKOpticalFlow::getMaxLevel", vec![(pred!(const, [], []), _)]),
	void cv_SparsePyrLKOpticalFlow_getMaxLevel_const(const cv::SparsePyrLKOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMaxLevel();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxLevel(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:631
	// ("cv::SparsePyrLKOpticalFlow::setMaxLevel", vec![(pred!(mut, ["maxLevel"], ["int"]), _)]),
	void cv_SparsePyrLKOpticalFlow_setMaxLevel_int(cv::SparsePyrLKOpticalFlow* instance, int maxLevel, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxLevel(maxLevel);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTermCriteria()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:633
	// ("cv::SparsePyrLKOpticalFlow::getTermCriteria", vec![(pred!(const, [], []), _)]),
	void cv_SparsePyrLKOpticalFlow_getTermCriteria_const(const cv::SparsePyrLKOpticalFlow* instance, Result<cv::TermCriteria>* ocvrs_return) {
		try {
			cv::TermCriteria ret = instance->getTermCriteria();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTermCriteria(TermCriteria &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:634
	// ("cv::SparsePyrLKOpticalFlow::setTermCriteria", vec![(pred!(mut, ["crit"], ["cv::TermCriteria*"]), _)]),
	void cv_SparsePyrLKOpticalFlow_setTermCriteria_TermCriteriaR(cv::SparsePyrLKOpticalFlow* instance, cv::TermCriteria* crit, ResultVoid* ocvrs_return) {
		try {
			instance->setTermCriteria(*crit);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getFlags()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:636
	// ("cv::SparsePyrLKOpticalFlow::getFlags", vec![(pred!(const, [], []), _)]),
	void cv_SparsePyrLKOpticalFlow_getFlags_const(const cv::SparsePyrLKOpticalFlow* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getFlags();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setFlags(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:637
	// ("cv::SparsePyrLKOpticalFlow::setFlags", vec![(pred!(mut, ["flags"], ["int"]), _)]),
	void cv_SparsePyrLKOpticalFlow_setFlags_int(cv::SparsePyrLKOpticalFlow* instance, int flags, ResultVoid* ocvrs_return) {
		try {
			instance->setFlags(flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMinEigThreshold()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:639
	// ("cv::SparsePyrLKOpticalFlow::getMinEigThreshold", vec![(pred!(const, [], []), _)]),
	void cv_SparsePyrLKOpticalFlow_getMinEigThreshold_const(const cv::SparsePyrLKOpticalFlow* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMinEigThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMinEigThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:640
	// ("cv::SparsePyrLKOpticalFlow::setMinEigThreshold", vec![(pred!(mut, ["minEigThreshold"], ["double"]), _)]),
	void cv_SparsePyrLKOpticalFlow_setMinEigThreshold_double(cv::SparsePyrLKOpticalFlow* instance, double minEigThreshold, ResultVoid* ocvrs_return) {
		try {
			instance->setMinEigThreshold(minEigThreshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(Size, int, TermCriteria, int, double)(SimpleClass, Primitive, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:642
	// ("cv::SparsePyrLKOpticalFlow::create", vec![(pred!(mut, ["winSize", "maxLevel", "crit", "flags", "minEigThreshold"], ["cv::Size", "int", "cv::TermCriteria", "int", "double"]), _)]),
	void cv_SparsePyrLKOpticalFlow_create_Size_int_TermCriteria_int_double(cv::Size* winSize, int maxLevel, cv::TermCriteria* crit, int flags, double minEigThreshold, Result<cv::Ptr<cv::SparsePyrLKOpticalFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::SparsePyrLKOpticalFlow> ret = cv::SparsePyrLKOpticalFlow::create(*winSize, maxLevel, *crit, flags, minEigThreshold);
			Ok(new cv::Ptr<cv::SparsePyrLKOpticalFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::SparsePyrLKOpticalFlow::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/video/tracking.hpp:642
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

}
