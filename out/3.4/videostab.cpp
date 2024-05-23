#include "ocvrs_common.hpp"
#include <opencv2/videostab.hpp>
#include "videostab_types.hpp"

extern "C" {
	// calcBlurriness(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/deblurring.hpp:57
	// ("cv::videostab::calcBlurriness", vec![(pred!(mut, ["frame"], ["const cv::Mat*"]), _)]),
	void cv_videostab_calcBlurriness_const_MatR(const cv::Mat* frame, Result<float>* ocvrs_return) {
		try {
			float ret = cv::videostab::calcBlurriness(*frame);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// calcFlowMask(const Mat &, const Mat &, const Mat &, float, const Mat &, const Mat &, Mat &)(TraitClass, TraitClass, TraitClass, Primitive, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/inpainting.hpp:199
	// ("cv::videostab::calcFlowMask", vec![(pred!(mut, ["flowX", "flowY", "errors", "maxError", "mask0", "mask1", "flowMask"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "float", "const cv::Mat*", "const cv::Mat*", "cv::Mat*"]), _)]),
	void cv_videostab_calcFlowMask_const_MatR_const_MatR_const_MatR_float_const_MatR_const_MatR_MatR(const cv::Mat* flowX, const cv::Mat* flowY, const cv::Mat* errors, float maxError, const cv::Mat* mask0, const cv::Mat* mask1, cv::Mat* flowMask, ResultVoid* ocvrs_return) {
		try {
			cv::videostab::calcFlowMask(*flowX, *flowY, *errors, maxError, *mask0, *mask1, *flowMask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// completeFrameAccordingToFlow(const Mat &, const Mat &, const Mat &, const Mat &, const Mat &, float, Mat &, Mat &)(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/inpainting.hpp:203
	// ("cv::videostab::completeFrameAccordingToFlow", vec![(pred!(mut, ["flowMask", "flowX", "flowY", "frame1", "mask1", "distThresh", "frame0", "mask0"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "float", "cv::Mat*", "cv::Mat*"]), _)]),
	void cv_videostab_completeFrameAccordingToFlow_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_float_MatR_MatR(const cv::Mat* flowMask, const cv::Mat* flowX, const cv::Mat* flowY, const cv::Mat* frame1, const cv::Mat* mask1, float distThresh, cv::Mat* frame0, cv::Mat* mask0, ResultVoid* ocvrs_return) {
		try {
			cv::videostab::completeFrameAccordingToFlow(*flowMask, *flowX, *flowY, *frame1, *mask1, distThresh, *frame0, *mask0);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ensureInclusionConstraint(const Mat &, Size, float)(TraitClass, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/motion_stabilizing.hpp:165
	// ("cv::videostab::ensureInclusionConstraint", vec![(pred!(mut, ["M", "size", "trimRatio"], ["const cv::Mat*", "cv::Size", "float"]), _)]),
	void cv_videostab_ensureInclusionConstraint_const_MatR_Size_float(const cv::Mat* M, cv::Size* size, float trimRatio, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::videostab::ensureInclusionConstraint(*M, *size, trimRatio);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::estimateGlobalMotionLeastSquares(InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/global_motion.hpp:77
	// ("cv::videostab::estimateGlobalMotionLeastSquares", vec![(pred!(mut, ["points0", "points1"], ["const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
	void cv_videostab_estimateGlobalMotionLeastSquares_const__InputOutputArrayR_const__InputOutputArrayR(const cv::_InputOutputArray* points0, const cv::_InputOutputArray* points1, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::videostab::estimateGlobalMotionLeastSquares(*points0, *points1);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// estimateGlobalMotionLeastSquares(InputOutputArray, InputOutputArray, int, float *)(InputOutputArray, InputOutputArray, Primitive, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/global_motion.hpp:77
	// ("cv::videostab::estimateGlobalMotionLeastSquares", vec![(pred!(mut, ["points0", "points1", "model", "rmse"], ["const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "int", "float*"]), _)]),
	void cv_videostab_estimateGlobalMotionLeastSquares_const__InputOutputArrayR_const__InputOutputArrayR_int_floatX(const cv::_InputOutputArray* points0, const cv::_InputOutputArray* points1, int model, float* rmse, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::videostab::estimateGlobalMotionLeastSquares(*points0, *points1, model, rmse);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::estimateGlobalMotionRansac(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/global_motion.hpp:90
	// ("cv::videostab::estimateGlobalMotionRansac", vec![(pred!(mut, ["points0", "points1"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_videostab_estimateGlobalMotionRansac_const__InputArrayR_const__InputArrayR(const cv::_InputArray* points0, const cv::_InputArray* points1, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::videostab::estimateGlobalMotionRansac(*points0, *points1);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// estimateGlobalMotionRansac(InputArray, InputArray, int, const RansacParams &, float *, int *)(InputArray, InputArray, Primitive, TraitClass, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/global_motion.hpp:90
	// ("cv::videostab::estimateGlobalMotionRansac", vec![(pred!(mut, ["points0", "points1", "model", "params", "rmse", "ninliers"], ["const cv::_InputArray*", "const cv::_InputArray*", "int", "const cv::videostab::RansacParams*", "float*", "int*"]), _)]),
	void cv_videostab_estimateGlobalMotionRansac_const__InputArrayR_const__InputArrayR_int_const_RansacParamsR_floatX_intX(const cv::_InputArray* points0, const cv::_InputArray* points1, int model, const cv::videostab::RansacParams* params, float* rmse, int* ninliers, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::videostab::estimateGlobalMotionRansac(*points0, *points1, model, *params, rmse, ninliers);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// estimateOptimalTrimRatio(const Mat &, Size)(TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/motion_stabilizing.hpp:167
	// ("cv::videostab::estimateOptimalTrimRatio", vec![(pred!(mut, ["M", "size"], ["const cv::Mat*", "cv::Size"]), _)]),
	void cv_videostab_estimateOptimalTrimRatio_const_MatR_Size(const cv::Mat* M, cv::Size* size, Result<float>* ocvrs_return) {
		try {
			float ret = cv::videostab::estimateOptimalTrimRatio(*M, *size);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMotion(int, int, const std::vector<Mat> &)(Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/global_motion.hpp:293
	// ("cv::videostab::getMotion", vec![(pred!(mut, ["from", "to", "motions"], ["int", "int", "const std::vector<cv::Mat>*"]), _)]),
	void cv_videostab_getMotion_int_int_const_vectorLMatGR(int from, int to, const std::vector<cv::Mat>* motions, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::videostab::getMotion(from, to, *motions);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// inpaint(int, Mat &, Mat &)(Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/inpainting.hpp:177
	// ("cv::videostab::ColorAverageInpainter::inpaint", vec![(pred!(mut, ["idx", "frame", "mask"], ["int", "cv::Mat*", "cv::Mat*"]), _)]),
	void cv_videostab_ColorAverageInpainter_inpaint_int_MatR_MatR(cv::videostab::ColorAverageInpainter* instance, int idx, cv::Mat* frame, cv::Mat* mask, ResultVoid* ocvrs_return) {
		try {
			instance->inpaint(idx, *frame, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::ColorAverageInpainter::defaultNew() generated
	// ("cv::videostab::ColorAverageInpainter::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::videostab::ColorAverageInpainter* cv_videostab_ColorAverageInpainter_defaultNew_const() {
			cv::videostab::ColorAverageInpainter* ret = new cv::videostab::ColorAverageInpainter();
			return ret;
	}

	// cv::videostab::ColorAverageInpainter::to_InpainterBase() generated
	// ("cv::videostab::ColorAverageInpainter::to_InpainterBase", vec![(pred!(mut, [], []), _)]),
	cv::videostab::InpainterBase* cv_videostab_ColorAverageInpainter_to_InpainterBase(cv::videostab::ColorAverageInpainter* instance) {
			return dynamic_cast<cv::videostab::InpainterBase*>(instance);
	}

	// cv::videostab::ColorAverageInpainter::delete() generated
	// ("cv::videostab::ColorAverageInpainter::delete", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_ColorAverageInpainter_delete(cv::videostab::ColorAverageInpainter* instance) {
			delete instance;
	}

	// ColorInpainter(int, double)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/inpainting.hpp:186
	// ("cv::videostab::ColorInpainter::ColorInpainter", vec![(pred!(mut, ["method", "radius"], ["int", "double"]), _)]),
	void cv_videostab_ColorInpainter_ColorInpainter_int_double(int method, double radius, Result<cv::videostab::ColorInpainter*>* ocvrs_return) {
		try {
			cv::videostab::ColorInpainter* ret = new cv::videostab::ColorInpainter(method, radius);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::ColorInpainter::ColorInpainter() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/inpainting.hpp:186
	// ("cv::videostab::ColorInpainter::ColorInpainter", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_ColorInpainter_ColorInpainter(Result<cv::videostab::ColorInpainter*>* ocvrs_return) {
		try {
			cv::videostab::ColorInpainter* ret = new cv::videostab::ColorInpainter();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// inpaint(int, Mat &, Mat &)(Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/inpainting.hpp:188
	// ("cv::videostab::ColorInpainter::inpaint", vec![(pred!(mut, ["idx", "frame", "mask"], ["int", "cv::Mat*", "cv::Mat*"]), _)]),
	void cv_videostab_ColorInpainter_inpaint_int_MatR_MatR(cv::videostab::ColorInpainter* instance, int idx, cv::Mat* frame, cv::Mat* mask, ResultVoid* ocvrs_return) {
		try {
			instance->inpaint(idx, *frame, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::ColorInpainter::to_InpainterBase() generated
	// ("cv::videostab::ColorInpainter::to_InpainterBase", vec![(pred!(mut, [], []), _)]),
	cv::videostab::InpainterBase* cv_videostab_ColorInpainter_to_InpainterBase(cv::videostab::ColorInpainter* instance) {
			return dynamic_cast<cv::videostab::InpainterBase*>(instance);
	}

	// cv::videostab::ColorInpainter::delete() generated
	// ("cv::videostab::ColorInpainter::delete", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_ColorInpainter_delete(cv::videostab::ColorInpainter* instance) {
			delete instance;
	}

	// ConsistentMosaicInpainter()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/inpainting.hpp:130
	// ("cv::videostab::ConsistentMosaicInpainter::ConsistentMosaicInpainter", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_ConsistentMosaicInpainter_ConsistentMosaicInpainter(Result<cv::videostab::ConsistentMosaicInpainter*>* ocvrs_return) {
		try {
			cv::videostab::ConsistentMosaicInpainter* ret = new cv::videostab::ConsistentMosaicInpainter();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setStdevThresh(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/inpainting.hpp:132
	// ("cv::videostab::ConsistentMosaicInpainter::setStdevThresh", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_videostab_ConsistentMosaicInpainter_setStdevThresh_float(cv::videostab::ConsistentMosaicInpainter* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setStdevThresh(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// stdevThresh()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/inpainting.hpp:133
	// ("cv::videostab::ConsistentMosaicInpainter::stdevThresh", vec![(pred!(const, [], []), _)]),
	void cv_videostab_ConsistentMosaicInpainter_stdevThresh_const(const cv::videostab::ConsistentMosaicInpainter* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->stdevThresh();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// inpaint(int, Mat &, Mat &)(Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/inpainting.hpp:135
	// ("cv::videostab::ConsistentMosaicInpainter::inpaint", vec![(pred!(mut, ["idx", "frame", "mask"], ["int", "cv::Mat*", "cv::Mat*"]), _)]),
	void cv_videostab_ConsistentMosaicInpainter_inpaint_int_MatR_MatR(cv::videostab::ConsistentMosaicInpainter* instance, int idx, cv::Mat* frame, cv::Mat* mask, ResultVoid* ocvrs_return) {
		try {
			instance->inpaint(idx, *frame, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::ConsistentMosaicInpainter::to_InpainterBase() generated
	// ("cv::videostab::ConsistentMosaicInpainter::to_InpainterBase", vec![(pred!(mut, [], []), _)]),
	cv::videostab::InpainterBase* cv_videostab_ConsistentMosaicInpainter_to_InpainterBase(cv::videostab::ConsistentMosaicInpainter* instance) {
			return dynamic_cast<cv::videostab::InpainterBase*>(instance);
	}

	// cv::videostab::ConsistentMosaicInpainter::delete() generated
	// ("cv::videostab::ConsistentMosaicInpainter::delete", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_ConsistentMosaicInpainter_delete(cv::videostab::ConsistentMosaicInpainter* instance) {
			delete instance;
	}

	// setRadius(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/deblurring.hpp:66
	// ("cv::videostab::DeblurerBase::setRadius", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_videostab_DeblurerBase_setRadius_int(cv::videostab::DeblurerBase* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setRadius(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// radius()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/deblurring.hpp:67
	// ("cv::videostab::DeblurerBase::radius", vec![(pred!(const, [], []), _)]),
	void cv_videostab_DeblurerBase_radius_const(const cv::videostab::DeblurerBase* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->radius();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// deblur(int, Mat &)(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/deblurring.hpp:69
	// ("cv::videostab::DeblurerBase::deblur", vec![(pred!(mut, ["idx", "frame"], ["int", "cv::Mat*"]), _)]),
	void cv_videostab_DeblurerBase_deblur_int_MatR(cv::videostab::DeblurerBase* instance, int idx, cv::Mat* frame, ResultVoid* ocvrs_return) {
		try {
			instance->deblur(idx, *frame);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setFrames(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/deblurring.hpp:74
	// ("cv::videostab::DeblurerBase::setFrames", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>*"]), _)]),
	void cv_videostab_DeblurerBase_setFrames_const_vectorLMatGR(cv::videostab::DeblurerBase* instance, const std::vector<cv::Mat>* val, ResultVoid* ocvrs_return) {
		try {
			instance->setFrames(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// frames()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/deblurring.hpp:75
	// ("cv::videostab::DeblurerBase::frames", vec![(pred!(const, [], []), _)]),
	void cv_videostab_DeblurerBase_frames_const(const cv::videostab::DeblurerBase* instance, Result<std::vector<cv::Mat>*>* ocvrs_return) {
		try {
			const std::vector<cv::Mat> ret = instance->frames();
			Ok(new const std::vector<cv::Mat>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMotions(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/deblurring.hpp:77
	// ("cv::videostab::DeblurerBase::setMotions", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>*"]), _)]),
	void cv_videostab_DeblurerBase_setMotions_const_vectorLMatGR(cv::videostab::DeblurerBase* instance, const std::vector<cv::Mat>* val, ResultVoid* ocvrs_return) {
		try {
			instance->setMotions(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// motions()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/deblurring.hpp:78
	// ("cv::videostab::DeblurerBase::motions", vec![(pred!(const, [], []), _)]),
	void cv_videostab_DeblurerBase_motions_const(const cv::videostab::DeblurerBase* instance, Result<std::vector<cv::Mat>*>* ocvrs_return) {
		try {
			const std::vector<cv::Mat> ret = instance->motions();
			Ok(new const std::vector<cv::Mat>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setBlurrinessRates(const std::vector<float> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/deblurring.hpp:80
	// ("cv::videostab::DeblurerBase::setBlurrinessRates", vec![(pred!(mut, ["val"], ["const std::vector<float>*"]), _)]),
	void cv_videostab_DeblurerBase_setBlurrinessRates_const_vectorLfloatGR(cv::videostab::DeblurerBase* instance, const std::vector<float>* val, ResultVoid* ocvrs_return) {
		try {
			instance->setBlurrinessRates(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// blurrinessRates()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/deblurring.hpp:81
	// ("cv::videostab::DeblurerBase::blurrinessRates", vec![(pred!(const, [], []), _)]),
	void cv_videostab_DeblurerBase_blurrinessRates_const(const cv::videostab::DeblurerBase* instance, Result<std::vector<float>*>* ocvrs_return) {
		try {
			const std::vector<float> ret = instance->blurrinessRates();
			Ok(new const std::vector<float>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::DeblurerBase::to_NullDeblurer() generated
	// ("cv::videostab::DeblurerBase::to_NullDeblurer", vec![(pred!(mut, [], []), _)]),
	cv::videostab::NullDeblurer* cv_videostab_DeblurerBase_to_NullDeblurer(cv::videostab::DeblurerBase* instance) {
			return dynamic_cast<cv::videostab::NullDeblurer*>(instance);
	}

	// cv::videostab::DeblurerBase::to_WeightingDeblurer() generated
	// ("cv::videostab::DeblurerBase::to_WeightingDeblurer", vec![(pred!(mut, [], []), _)]),
	cv::videostab::WeightingDeblurer* cv_videostab_DeblurerBase_to_WeightingDeblurer(cv::videostab::DeblurerBase* instance) {
			return dynamic_cast<cv::videostab::WeightingDeblurer*>(instance);
	}

	// cv::videostab::DeblurerBase::delete() generated
	// ("cv::videostab::DeblurerBase::delete", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_DeblurerBase_delete(cv::videostab::DeblurerBase* instance) {
			delete instance;
	}

	// FastMarchingMethod()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/fast_marching.hpp:66
	// ("cv::videostab::FastMarchingMethod::FastMarchingMethod", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_FastMarchingMethod_FastMarchingMethod(Result<cv::videostab::FastMarchingMethod*>* ocvrs_return) {
		try {
			cv::videostab::FastMarchingMethod* ret = new cv::videostab::FastMarchingMethod();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// distanceMap()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/fast_marching.hpp:81
	// ("cv::videostab::FastMarchingMethod::distanceMap", vec![(pred!(const, [], []), _)]),
	void cv_videostab_FastMarchingMethod_distanceMap_const(const cv::videostab::FastMarchingMethod* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->distanceMap();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::FastMarchingMethod::delete() generated
	// ("cv::videostab::FastMarchingMethod::delete", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_FastMarchingMethod_delete(cv::videostab::FastMarchingMethod* instance) {
			delete instance;
	}

	// FromFileMotionReader(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/global_motion.hpp:195
	// ("cv::videostab::FromFileMotionReader::FromFileMotionReader", vec![(pred!(mut, ["path"], ["const cv::String*"]), _)]),
	void cv_videostab_FromFileMotionReader_FromFileMotionReader_const_StringR(const char* path, Result<cv::videostab::FromFileMotionReader*>* ocvrs_return) {
		try {
			cv::videostab::FromFileMotionReader* ret = new cv::videostab::FromFileMotionReader(cv::String(path));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// estimate(const Mat &, const Mat &, bool *)(TraitClass, TraitClass, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/global_motion.hpp:197
	// ("cv::videostab::FromFileMotionReader::estimate", vec![(pred!(mut, ["frame0", "frame1", "ok"], ["const cv::Mat*", "const cv::Mat*", "bool*"]), _)]),
	void cv_videostab_FromFileMotionReader_estimate_const_MatR_const_MatR_boolX(cv::videostab::FromFileMotionReader* instance, const cv::Mat* frame0, const cv::Mat* frame1, bool* ok, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->estimate(*frame0, *frame1, ok);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::FromFileMotionReader::estimate(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/global_motion.hpp:197
	// ("cv::videostab::FromFileMotionReader::estimate", vec![(pred!(mut, ["frame0", "frame1"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
	void cv_videostab_FromFileMotionReader_estimate_const_MatR_const_MatR(cv::videostab::FromFileMotionReader* instance, const cv::Mat* frame0, const cv::Mat* frame1, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->estimate(*frame0, *frame1);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::FromFileMotionReader::to_ImageMotionEstimatorBase() generated
	// ("cv::videostab::FromFileMotionReader::to_ImageMotionEstimatorBase", vec![(pred!(mut, [], []), _)]),
	cv::videostab::ImageMotionEstimatorBase* cv_videostab_FromFileMotionReader_to_ImageMotionEstimatorBase(cv::videostab::FromFileMotionReader* instance) {
			return dynamic_cast<cv::videostab::ImageMotionEstimatorBase*>(instance);
	}

	// cv::videostab::FromFileMotionReader::delete() generated
	// ("cv::videostab::FromFileMotionReader::delete", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_FromFileMotionReader_delete(cv::videostab::FromFileMotionReader* instance) {
			delete instance;
	}

	// GaussianMotionFilter(int, float)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/motion_stabilizing.hpp:100
	// ("cv::videostab::GaussianMotionFilter::GaussianMotionFilter", vec![(pred!(mut, ["radius", "stdev"], ["int", "float"]), _)]),
	void cv_videostab_GaussianMotionFilter_GaussianMotionFilter_int_float(int radius, float stdev, Result<cv::videostab::GaussianMotionFilter*>* ocvrs_return) {
		try {
			cv::videostab::GaussianMotionFilter* ret = new cv::videostab::GaussianMotionFilter(radius, stdev);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::GaussianMotionFilter::GaussianMotionFilter() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/motion_stabilizing.hpp:100
	// ("cv::videostab::GaussianMotionFilter::GaussianMotionFilter", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_GaussianMotionFilter_GaussianMotionFilter(Result<cv::videostab::GaussianMotionFilter*>* ocvrs_return) {
		try {
			cv::videostab::GaussianMotionFilter* ret = new cv::videostab::GaussianMotionFilter();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setParams(int, float)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/motion_stabilizing.hpp:102
	// ("cv::videostab::GaussianMotionFilter::setParams", vec![(pred!(mut, ["radius", "stdev"], ["int", "float"]), _)]),
	void cv_videostab_GaussianMotionFilter_setParams_int_float(cv::videostab::GaussianMotionFilter* instance, int radius, float stdev, ResultVoid* ocvrs_return) {
		try {
			instance->setParams(radius, stdev);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::GaussianMotionFilter::setParams(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/motion_stabilizing.hpp:102
	// ("cv::videostab::GaussianMotionFilter::setParams", vec![(pred!(mut, ["radius"], ["int"]), _)]),
	void cv_videostab_GaussianMotionFilter_setParams_int(cv::videostab::GaussianMotionFilter* instance, int radius, ResultVoid* ocvrs_return) {
		try {
			instance->setParams(radius);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// radius()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/motion_stabilizing.hpp:103
	// ("cv::videostab::GaussianMotionFilter::radius", vec![(pred!(const, [], []), _)]),
	void cv_videostab_GaussianMotionFilter_radius_const(const cv::videostab::GaussianMotionFilter* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->radius();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// stdev()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/motion_stabilizing.hpp:104
	// ("cv::videostab::GaussianMotionFilter::stdev", vec![(pred!(const, [], []), _)]),
	void cv_videostab_GaussianMotionFilter_stdev_const(const cv::videostab::GaussianMotionFilter* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->stdev();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// stabilize(int, const std::vector<Mat> &, std::pair<int, int>)(Primitive, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/motion_stabilizing.hpp:106
	// ("cv::videostab::GaussianMotionFilter::stabilize", vec![(pred!(mut, ["idx", "motions", "range"], ["int", "const std::vector<cv::Mat>*", "std::pair<int, int>"]), _)]),
	void cv_videostab_GaussianMotionFilter_stabilize_int_const_vectorLMatGR_pairLint__intG(cv::videostab::GaussianMotionFilter* instance, int idx, const std::vector<cv::Mat>* motions, std::pair<int, int>* range, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->stabilize(idx, *motions, *range);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::GaussianMotionFilter::to_IMotionStabilizer() generated
	// ("cv::videostab::GaussianMotionFilter::to_IMotionStabilizer", vec![(pred!(mut, [], []), _)]),
	cv::videostab::IMotionStabilizer* cv_videostab_GaussianMotionFilter_to_IMotionStabilizer(cv::videostab::GaussianMotionFilter* instance) {
			return dynamic_cast<cv::videostab::IMotionStabilizer*>(instance);
	}

	// cv::videostab::GaussianMotionFilter::to_MotionFilterBase() generated
	// ("cv::videostab::GaussianMotionFilter::to_MotionFilterBase", vec![(pred!(mut, [], []), _)]),
	cv::videostab::MotionFilterBase* cv_videostab_GaussianMotionFilter_to_MotionFilterBase(cv::videostab::GaussianMotionFilter* instance) {
			return dynamic_cast<cv::videostab::MotionFilterBase*>(instance);
	}

	// cv::videostab::GaussianMotionFilter::delete() generated
	// ("cv::videostab::GaussianMotionFilter::delete", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_GaussianMotionFilter_delete(cv::videostab::GaussianMotionFilter* instance) {
			delete instance;
	}

	// run(InputArray, InputArray, InputOutputArray, InputOutputArray, OutputArray)(InputArray, InputArray, InputOutputArray, InputOutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/optical_flow.hpp:74
	// ("cv::videostab::IDenseOptFlowEstimator::run", vec![(pred!(mut, ["frame0", "frame1", "flowX", "flowY", "errors"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_videostab_IDenseOptFlowEstimator_run_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__OutputArrayR(cv::videostab::IDenseOptFlowEstimator* instance, const cv::_InputArray* frame0, const cv::_InputArray* frame1, const cv::_InputOutputArray* flowX, const cv::_InputOutputArray* flowY, const cv::_OutputArray* errors, ResultVoid* ocvrs_return) {
		try {
			instance->run(*frame0, *frame1, *flowX, *flowY, *errors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::IDenseOptFlowEstimator::delete() generated
	// ("cv::videostab::IDenseOptFlowEstimator::delete", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_IDenseOptFlowEstimator_delete(cv::videostab::IDenseOptFlowEstimator* instance) {
			delete instance;
	}

	// reset()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/frame_source.hpp:61
	// ("cv::videostab::IFrameSource::reset", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_IFrameSource_reset(cv::videostab::IFrameSource* instance, ResultVoid* ocvrs_return) {
		try {
			instance->reset();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// nextFrame()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/frame_source.hpp:62
	// ("cv::videostab::IFrameSource::nextFrame", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_IFrameSource_nextFrame(cv::videostab::IFrameSource* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->nextFrame();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::IFrameSource::to_NullFrameSource() generated
	// ("cv::videostab::IFrameSource::to_NullFrameSource", vec![(pred!(mut, [], []), _)]),
	cv::videostab::NullFrameSource* cv_videostab_IFrameSource_to_NullFrameSource(cv::videostab::IFrameSource* instance) {
			return dynamic_cast<cv::videostab::NullFrameSource*>(instance);
	}

	// cv::videostab::IFrameSource::to_OnePassStabilizer() generated
	// ("cv::videostab::IFrameSource::to_OnePassStabilizer", vec![(pred!(mut, [], []), _)]),
	cv::videostab::OnePassStabilizer* cv_videostab_IFrameSource_to_OnePassStabilizer(cv::videostab::IFrameSource* instance) {
			return dynamic_cast<cv::videostab::OnePassStabilizer*>(instance);
	}

	// cv::videostab::IFrameSource::to_TwoPassStabilizer() generated
	// ("cv::videostab::IFrameSource::to_TwoPassStabilizer", vec![(pred!(mut, [], []), _)]),
	cv::videostab::TwoPassStabilizer* cv_videostab_IFrameSource_to_TwoPassStabilizer(cv::videostab::IFrameSource* instance) {
			return dynamic_cast<cv::videostab::TwoPassStabilizer*>(instance);
	}

	// cv::videostab::IFrameSource::to_VideoFileSource() generated
	// ("cv::videostab::IFrameSource::to_VideoFileSource", vec![(pred!(mut, [], []), _)]),
	cv::videostab::VideoFileSource* cv_videostab_IFrameSource_to_VideoFileSource(cv::videostab::IFrameSource* instance) {
			return dynamic_cast<cv::videostab::VideoFileSource*>(instance);
	}

	// cv::videostab::IFrameSource::delete() generated
	// ("cv::videostab::IFrameSource::delete", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_IFrameSource_delete(cv::videostab::IFrameSource* instance) {
			delete instance;
	}

	// print(const char *, ...)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/log.hpp:60
	// ("cv::videostab::ILog::print", vec![(pred!(mut, ["format"], ["const char*"]), _)]),
	void cv_videostab_ILog_print_const_charX(cv::videostab::ILog* instance, const char* format, ResultVoid* ocvrs_return) {
		try {
			instance->print(format);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::ILog::to_LogToStdout() generated
	// ("cv::videostab::ILog::to_LogToStdout", vec![(pred!(mut, [], []), _)]),
	cv::videostab::LogToStdout* cv_videostab_ILog_to_LogToStdout(cv::videostab::ILog* instance) {
			return dynamic_cast<cv::videostab::LogToStdout*>(instance);
	}

	// cv::videostab::ILog::to_NullLog() generated
	// ("cv::videostab::ILog::to_NullLog", vec![(pred!(mut, [], []), _)]),
	cv::videostab::NullLog* cv_videostab_ILog_to_NullLog(cv::videostab::ILog* instance) {
			return dynamic_cast<cv::videostab::NullLog*>(instance);
	}

	// cv::videostab::ILog::delete() generated
	// ("cv::videostab::ILog::delete", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_ILog_delete(cv::videostab::ILog* instance) {
			delete instance;
	}

	// stabilize(int, const std::vector<Mat> &, std::pair<int, int>, Mat *)(Primitive, CppPassByVoidPtr, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/motion_stabilizing.hpp:65
	// ("cv::videostab::IMotionStabilizer::stabilize", vec![(pred!(mut, ["size", "motions", "range", "stabilizationMotions"], ["int", "const std::vector<cv::Mat>*", "std::pair<int, int>", "cv::Mat*"]), _)]),
	void cv_videostab_IMotionStabilizer_stabilize_int_const_vectorLMatGR_pairLint__intG_MatX(cv::videostab::IMotionStabilizer* instance, int size, const std::vector<cv::Mat>* motions, std::pair<int, int>* range, cv::Mat* stabilizationMotions, ResultVoid* ocvrs_return) {
		try {
			instance->stabilize(size, *motions, *range, stabilizationMotions);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::IMotionStabilizer::to_GaussianMotionFilter() generated
	// ("cv::videostab::IMotionStabilizer::to_GaussianMotionFilter", vec![(pred!(mut, [], []), _)]),
	cv::videostab::GaussianMotionFilter* cv_videostab_IMotionStabilizer_to_GaussianMotionFilter(cv::videostab::IMotionStabilizer* instance) {
			return dynamic_cast<cv::videostab::GaussianMotionFilter*>(instance);
	}

	// cv::videostab::IMotionStabilizer::to_LpMotionStabilizer() generated
	// ("cv::videostab::IMotionStabilizer::to_LpMotionStabilizer", vec![(pred!(mut, [], []), _)]),
	cv::videostab::LpMotionStabilizer* cv_videostab_IMotionStabilizer_to_LpMotionStabilizer(cv::videostab::IMotionStabilizer* instance) {
			return dynamic_cast<cv::videostab::LpMotionStabilizer*>(instance);
	}

	// cv::videostab::IMotionStabilizer::to_MotionFilterBase() generated
	// ("cv::videostab::IMotionStabilizer::to_MotionFilterBase", vec![(pred!(mut, [], []), _)]),
	cv::videostab::MotionFilterBase* cv_videostab_IMotionStabilizer_to_MotionFilterBase(cv::videostab::IMotionStabilizer* instance) {
			return dynamic_cast<cv::videostab::MotionFilterBase*>(instance);
	}

	// cv::videostab::IMotionStabilizer::to_MotionStabilizationPipeline() generated
	// ("cv::videostab::IMotionStabilizer::to_MotionStabilizationPipeline", vec![(pred!(mut, [], []), _)]),
	cv::videostab::MotionStabilizationPipeline* cv_videostab_IMotionStabilizer_to_MotionStabilizationPipeline(cv::videostab::IMotionStabilizer* instance) {
			return dynamic_cast<cv::videostab::MotionStabilizationPipeline*>(instance);
	}

	// cv::videostab::IMotionStabilizer::delete() generated
	// ("cv::videostab::IMotionStabilizer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_IMotionStabilizer_delete(cv::videostab::IMotionStabilizer* instance) {
			delete instance;
	}

	// process(Size, InputArray, InputArray, OutputArray)(SimpleClass, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/outlier_rejection.hpp:63
	// ("cv::videostab::IOutlierRejector::process", vec![(pred!(mut, ["frameSize", "points0", "points1", "mask"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_videostab_IOutlierRejector_process_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR(cv::videostab::IOutlierRejector* instance, cv::Size* frameSize, const cv::_InputArray* points0, const cv::_InputArray* points1, const cv::_OutputArray* mask, ResultVoid* ocvrs_return) {
		try {
			instance->process(*frameSize, *points0, *points1, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::IOutlierRejector::to_NullOutlierRejector() generated
	// ("cv::videostab::IOutlierRejector::to_NullOutlierRejector", vec![(pred!(mut, [], []), _)]),
	cv::videostab::NullOutlierRejector* cv_videostab_IOutlierRejector_to_NullOutlierRejector(cv::videostab::IOutlierRejector* instance) {
			return dynamic_cast<cv::videostab::NullOutlierRejector*>(instance);
	}

	// cv::videostab::IOutlierRejector::to_TranslationBasedLocalOutlierRejector() generated
	// ("cv::videostab::IOutlierRejector::to_TranslationBasedLocalOutlierRejector", vec![(pred!(mut, [], []), _)]),
	cv::videostab::TranslationBasedLocalOutlierRejector* cv_videostab_IOutlierRejector_to_TranslationBasedLocalOutlierRejector(cv::videostab::IOutlierRejector* instance) {
			return dynamic_cast<cv::videostab::TranslationBasedLocalOutlierRejector*>(instance);
	}

	// cv::videostab::IOutlierRejector::delete() generated
	// ("cv::videostab::IOutlierRejector::delete", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_IOutlierRejector_delete(cv::videostab::IOutlierRejector* instance) {
			delete instance;
	}

	// run(InputArray, InputArray, InputArray, InputOutputArray, OutputArray, OutputArray)(InputArray, InputArray, InputArray, InputOutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/optical_flow.hpp:65
	// ("cv::videostab::ISparseOptFlowEstimator::run", vec![(pred!(mut, ["frame0", "frame1", "points0", "points1", "status", "errors"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_videostab_ISparseOptFlowEstimator_run_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR(cv::videostab::ISparseOptFlowEstimator* instance, const cv::_InputArray* frame0, const cv::_InputArray* frame1, const cv::_InputArray* points0, const cv::_InputOutputArray* points1, const cv::_OutputArray* status, const cv::_OutputArray* errors, ResultVoid* ocvrs_return) {
		try {
			instance->run(*frame0, *frame1, *points0, *points1, *status, *errors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::ISparseOptFlowEstimator::to_SparsePyrLkOptFlowEstimator() generated
	// ("cv::videostab::ISparseOptFlowEstimator::to_SparsePyrLkOptFlowEstimator", vec![(pred!(mut, [], []), _)]),
	cv::videostab::SparsePyrLkOptFlowEstimator* cv_videostab_ISparseOptFlowEstimator_to_SparsePyrLkOptFlowEstimator(cv::videostab::ISparseOptFlowEstimator* instance) {
			return dynamic_cast<cv::videostab::SparsePyrLkOptFlowEstimator*>(instance);
	}

	// cv::videostab::ISparseOptFlowEstimator::delete() generated
	// ("cv::videostab::ISparseOptFlowEstimator::delete", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_ISparseOptFlowEstimator_delete(cv::videostab::ISparseOptFlowEstimator* instance) {
			delete instance;
	}

	// setMotionModel(MotionModel)(Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/global_motion.hpp:180
	// ("cv::videostab::ImageMotionEstimatorBase::setMotionModel", vec![(pred!(mut, ["val"], ["cv::videostab::MotionModel"]), _)]),
	void cv_videostab_ImageMotionEstimatorBase_setMotionModel_MotionModel(cv::videostab::ImageMotionEstimatorBase* instance, cv::videostab::MotionModel val, ResultVoid* ocvrs_return) {
		try {
			instance->setMotionModel(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// motionModel()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/global_motion.hpp:181
	// ("cv::videostab::ImageMotionEstimatorBase::motionModel", vec![(pred!(const, [], []), _)]),
	void cv_videostab_ImageMotionEstimatorBase_motionModel_const(const cv::videostab::ImageMotionEstimatorBase* instance, Result<cv::videostab::MotionModel>* ocvrs_return) {
		try {
			cv::videostab::MotionModel ret = instance->motionModel();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// estimate(const Mat &, const Mat &, bool *)(TraitClass, TraitClass, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/global_motion.hpp:183
	// ("cv::videostab::ImageMotionEstimatorBase::estimate", vec![(pred!(mut, ["frame0", "frame1", "ok"], ["const cv::Mat*", "const cv::Mat*", "bool*"]), _)]),
	void cv_videostab_ImageMotionEstimatorBase_estimate_const_MatR_const_MatR_boolX(cv::videostab::ImageMotionEstimatorBase* instance, const cv::Mat* frame0, const cv::Mat* frame1, bool* ok, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->estimate(*frame0, *frame1, ok);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::ImageMotionEstimatorBase::estimate(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/global_motion.hpp:183
	// ("cv::videostab::ImageMotionEstimatorBase::estimate", vec![(pred!(mut, ["frame0", "frame1"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
	void cv_videostab_ImageMotionEstimatorBase_estimate_const_MatR_const_MatR(cv::videostab::ImageMotionEstimatorBase* instance, const cv::Mat* frame0, const cv::Mat* frame1, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->estimate(*frame0, *frame1);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::ImageMotionEstimatorBase::to_FromFileMotionReader() generated
	// ("cv::videostab::ImageMotionEstimatorBase::to_FromFileMotionReader", vec![(pred!(mut, [], []), _)]),
	cv::videostab::FromFileMotionReader* cv_videostab_ImageMotionEstimatorBase_to_FromFileMotionReader(cv::videostab::ImageMotionEstimatorBase* instance) {
			return dynamic_cast<cv::videostab::FromFileMotionReader*>(instance);
	}

	// cv::videostab::ImageMotionEstimatorBase::to_KeypointBasedMotionEstimator() generated
	// ("cv::videostab::ImageMotionEstimatorBase::to_KeypointBasedMotionEstimator", vec![(pred!(mut, [], []), _)]),
	cv::videostab::KeypointBasedMotionEstimator* cv_videostab_ImageMotionEstimatorBase_to_KeypointBasedMotionEstimator(cv::videostab::ImageMotionEstimatorBase* instance) {
			return dynamic_cast<cv::videostab::KeypointBasedMotionEstimator*>(instance);
	}

	// cv::videostab::ImageMotionEstimatorBase::to_ToFileMotionWriter() generated
	// ("cv::videostab::ImageMotionEstimatorBase::to_ToFileMotionWriter", vec![(pred!(mut, [], []), _)]),
	cv::videostab::ToFileMotionWriter* cv_videostab_ImageMotionEstimatorBase_to_ToFileMotionWriter(cv::videostab::ImageMotionEstimatorBase* instance) {
			return dynamic_cast<cv::videostab::ToFileMotionWriter*>(instance);
	}

	// cv::videostab::ImageMotionEstimatorBase::delete() generated
	// ("cv::videostab::ImageMotionEstimatorBase::delete", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_ImageMotionEstimatorBase_delete(cv::videostab::ImageMotionEstimatorBase* instance) {
			delete instance;
	}

	// setRadius(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/inpainting.hpp:70
	// ("cv::videostab::InpainterBase::setRadius", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_videostab_InpainterBase_setRadius_int(cv::videostab::InpainterBase* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setRadius(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// radius()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/inpainting.hpp:71
	// ("cv::videostab::InpainterBase::radius", vec![(pred!(const, [], []), _)]),
	void cv_videostab_InpainterBase_radius_const(const cv::videostab::InpainterBase* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->radius();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMotionModel(MotionModel)(Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/inpainting.hpp:73
	// ("cv::videostab::InpainterBase::setMotionModel", vec![(pred!(mut, ["val"], ["cv::videostab::MotionModel"]), _)]),
	void cv_videostab_InpainterBase_setMotionModel_MotionModel(cv::videostab::InpainterBase* instance, cv::videostab::MotionModel val, ResultVoid* ocvrs_return) {
		try {
			instance->setMotionModel(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// motionModel()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/inpainting.hpp:74
	// ("cv::videostab::InpainterBase::motionModel", vec![(pred!(const, [], []), _)]),
	void cv_videostab_InpainterBase_motionModel_const(const cv::videostab::InpainterBase* instance, Result<cv::videostab::MotionModel>* ocvrs_return) {
		try {
			cv::videostab::MotionModel ret = instance->motionModel();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// inpaint(int, Mat &, Mat &)(Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/inpainting.hpp:76
	// ("cv::videostab::InpainterBase::inpaint", vec![(pred!(mut, ["idx", "frame", "mask"], ["int", "cv::Mat*", "cv::Mat*"]), _)]),
	void cv_videostab_InpainterBase_inpaint_int_MatR_MatR(cv::videostab::InpainterBase* instance, int idx, cv::Mat* frame, cv::Mat* mask, ResultVoid* ocvrs_return) {
		try {
			instance->inpaint(idx, *frame, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setFrames(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/inpainting.hpp:81
	// ("cv::videostab::InpainterBase::setFrames", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>*"]), _)]),
	void cv_videostab_InpainterBase_setFrames_const_vectorLMatGR(cv::videostab::InpainterBase* instance, const std::vector<cv::Mat>* val, ResultVoid* ocvrs_return) {
		try {
			instance->setFrames(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// frames()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/inpainting.hpp:82
	// ("cv::videostab::InpainterBase::frames", vec![(pred!(const, [], []), _)]),
	void cv_videostab_InpainterBase_frames_const(const cv::videostab::InpainterBase* instance, Result<std::vector<cv::Mat>*>* ocvrs_return) {
		try {
			const std::vector<cv::Mat> ret = instance->frames();
			Ok(new const std::vector<cv::Mat>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMotions(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/inpainting.hpp:84
	// ("cv::videostab::InpainterBase::setMotions", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>*"]), _)]),
	void cv_videostab_InpainterBase_setMotions_const_vectorLMatGR(cv::videostab::InpainterBase* instance, const std::vector<cv::Mat>* val, ResultVoid* ocvrs_return) {
		try {
			instance->setMotions(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// motions()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/inpainting.hpp:85
	// ("cv::videostab::InpainterBase::motions", vec![(pred!(const, [], []), _)]),
	void cv_videostab_InpainterBase_motions_const(const cv::videostab::InpainterBase* instance, Result<std::vector<cv::Mat>*>* ocvrs_return) {
		try {
			const std::vector<cv::Mat> ret = instance->motions();
			Ok(new const std::vector<cv::Mat>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setStabilizedFrames(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/inpainting.hpp:87
	// ("cv::videostab::InpainterBase::setStabilizedFrames", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>*"]), _)]),
	void cv_videostab_InpainterBase_setStabilizedFrames_const_vectorLMatGR(cv::videostab::InpainterBase* instance, const std::vector<cv::Mat>* val, ResultVoid* ocvrs_return) {
		try {
			instance->setStabilizedFrames(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// stabilizedFrames()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/inpainting.hpp:88
	// ("cv::videostab::InpainterBase::stabilizedFrames", vec![(pred!(const, [], []), _)]),
	void cv_videostab_InpainterBase_stabilizedFrames_const(const cv::videostab::InpainterBase* instance, Result<std::vector<cv::Mat>*>* ocvrs_return) {
		try {
			const std::vector<cv::Mat> ret = instance->stabilizedFrames();
			Ok(new const std::vector<cv::Mat>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setStabilizationMotions(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/inpainting.hpp:90
	// ("cv::videostab::InpainterBase::setStabilizationMotions", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>*"]), _)]),
	void cv_videostab_InpainterBase_setStabilizationMotions_const_vectorLMatGR(cv::videostab::InpainterBase* instance, const std::vector<cv::Mat>* val, ResultVoid* ocvrs_return) {
		try {
			instance->setStabilizationMotions(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// stabilizationMotions()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/inpainting.hpp:91
	// ("cv::videostab::InpainterBase::stabilizationMotions", vec![(pred!(const, [], []), _)]),
	void cv_videostab_InpainterBase_stabilizationMotions_const(const cv::videostab::InpainterBase* instance, Result<std::vector<cv::Mat>*>* ocvrs_return) {
		try {
			const std::vector<cv::Mat> ret = instance->stabilizationMotions();
			Ok(new const std::vector<cv::Mat>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::InpainterBase::to_ColorAverageInpainter() generated
	// ("cv::videostab::InpainterBase::to_ColorAverageInpainter", vec![(pred!(mut, [], []), _)]),
	cv::videostab::ColorAverageInpainter* cv_videostab_InpainterBase_to_ColorAverageInpainter(cv::videostab::InpainterBase* instance) {
			return dynamic_cast<cv::videostab::ColorAverageInpainter*>(instance);
	}

	// cv::videostab::InpainterBase::to_ColorInpainter() generated
	// ("cv::videostab::InpainterBase::to_ColorInpainter", vec![(pred!(mut, [], []), _)]),
	cv::videostab::ColorInpainter* cv_videostab_InpainterBase_to_ColorInpainter(cv::videostab::InpainterBase* instance) {
			return dynamic_cast<cv::videostab::ColorInpainter*>(instance);
	}

	// cv::videostab::InpainterBase::to_ConsistentMosaicInpainter() generated
	// ("cv::videostab::InpainterBase::to_ConsistentMosaicInpainter", vec![(pred!(mut, [], []), _)]),
	cv::videostab::ConsistentMosaicInpainter* cv_videostab_InpainterBase_to_ConsistentMosaicInpainter(cv::videostab::InpainterBase* instance) {
			return dynamic_cast<cv::videostab::ConsistentMosaicInpainter*>(instance);
	}

	// cv::videostab::InpainterBase::to_InpaintingPipeline() generated
	// ("cv::videostab::InpainterBase::to_InpaintingPipeline", vec![(pred!(mut, [], []), _)]),
	cv::videostab::InpaintingPipeline* cv_videostab_InpainterBase_to_InpaintingPipeline(cv::videostab::InpainterBase* instance) {
			return dynamic_cast<cv::videostab::InpaintingPipeline*>(instance);
	}

	// cv::videostab::InpainterBase::to_MotionInpainter() generated
	// ("cv::videostab::InpainterBase::to_MotionInpainter", vec![(pred!(mut, [], []), _)]),
	cv::videostab::MotionInpainter* cv_videostab_InpainterBase_to_MotionInpainter(cv::videostab::InpainterBase* instance) {
			return dynamic_cast<cv::videostab::MotionInpainter*>(instance);
	}

	// cv::videostab::InpainterBase::to_NullInpainter() generated
	// ("cv::videostab::InpainterBase::to_NullInpainter", vec![(pred!(mut, [], []), _)]),
	cv::videostab::NullInpainter* cv_videostab_InpainterBase_to_NullInpainter(cv::videostab::InpainterBase* instance) {
			return dynamic_cast<cv::videostab::NullInpainter*>(instance);
	}

	// cv::videostab::InpainterBase::delete() generated
	// ("cv::videostab::InpainterBase::delete", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_InpainterBase_delete(cv::videostab::InpainterBase* instance) {
			delete instance;
	}

	// pushBack(Ptr<InpainterBase>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/inpainting.hpp:111
	// ("cv::videostab::InpaintingPipeline::pushBack", vec![(pred!(mut, ["inpainter"], ["cv::Ptr<cv::videostab::InpainterBase>"]), _)]),
	void cv_videostab_InpaintingPipeline_pushBack_PtrLInpainterBaseG(cv::videostab::InpaintingPipeline* instance, cv::Ptr<cv::videostab::InpainterBase>* inpainter, ResultVoid* ocvrs_return) {
		try {
			instance->pushBack(*inpainter);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// empty()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/inpainting.hpp:112
	// ("cv::videostab::InpaintingPipeline::empty", vec![(pred!(const, [], []), _)]),
	void cv_videostab_InpaintingPipeline_empty_const(const cv::videostab::InpaintingPipeline* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setRadius(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/inpainting.hpp:114
	// ("cv::videostab::InpaintingPipeline::setRadius", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_videostab_InpaintingPipeline_setRadius_int(cv::videostab::InpaintingPipeline* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setRadius(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMotionModel(MotionModel)(Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/inpainting.hpp:115
	// ("cv::videostab::InpaintingPipeline::setMotionModel", vec![(pred!(mut, ["val"], ["cv::videostab::MotionModel"]), _)]),
	void cv_videostab_InpaintingPipeline_setMotionModel_MotionModel(cv::videostab::InpaintingPipeline* instance, cv::videostab::MotionModel val, ResultVoid* ocvrs_return) {
		try {
			instance->setMotionModel(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setFrames(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/inpainting.hpp:116
	// ("cv::videostab::InpaintingPipeline::setFrames", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>*"]), _)]),
	void cv_videostab_InpaintingPipeline_setFrames_const_vectorLMatGR(cv::videostab::InpaintingPipeline* instance, const std::vector<cv::Mat>* val, ResultVoid* ocvrs_return) {
		try {
			instance->setFrames(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMotions(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/inpainting.hpp:117
	// ("cv::videostab::InpaintingPipeline::setMotions", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>*"]), _)]),
	void cv_videostab_InpaintingPipeline_setMotions_const_vectorLMatGR(cv::videostab::InpaintingPipeline* instance, const std::vector<cv::Mat>* val, ResultVoid* ocvrs_return) {
		try {
			instance->setMotions(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setStabilizedFrames(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/inpainting.hpp:118
	// ("cv::videostab::InpaintingPipeline::setStabilizedFrames", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>*"]), _)]),
	void cv_videostab_InpaintingPipeline_setStabilizedFrames_const_vectorLMatGR(cv::videostab::InpaintingPipeline* instance, const std::vector<cv::Mat>* val, ResultVoid* ocvrs_return) {
		try {
			instance->setStabilizedFrames(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setStabilizationMotions(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/inpainting.hpp:119
	// ("cv::videostab::InpaintingPipeline::setStabilizationMotions", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>*"]), _)]),
	void cv_videostab_InpaintingPipeline_setStabilizationMotions_const_vectorLMatGR(cv::videostab::InpaintingPipeline* instance, const std::vector<cv::Mat>* val, ResultVoid* ocvrs_return) {
		try {
			instance->setStabilizationMotions(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// inpaint(int, Mat &, Mat &)(Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/inpainting.hpp:121
	// ("cv::videostab::InpaintingPipeline::inpaint", vec![(pred!(mut, ["idx", "frame", "mask"], ["int", "cv::Mat*", "cv::Mat*"]), _)]),
	void cv_videostab_InpaintingPipeline_inpaint_int_MatR_MatR(cv::videostab::InpaintingPipeline* instance, int idx, cv::Mat* frame, cv::Mat* mask, ResultVoid* ocvrs_return) {
		try {
			instance->inpaint(idx, *frame, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::InpaintingPipeline::defaultNew() generated
	// ("cv::videostab::InpaintingPipeline::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::videostab::InpaintingPipeline* cv_videostab_InpaintingPipeline_defaultNew_const() {
			cv::videostab::InpaintingPipeline* ret = new cv::videostab::InpaintingPipeline();
			return ret;
	}

	// cv::videostab::InpaintingPipeline::to_InpainterBase() generated
	// ("cv::videostab::InpaintingPipeline::to_InpainterBase", vec![(pred!(mut, [], []), _)]),
	cv::videostab::InpainterBase* cv_videostab_InpaintingPipeline_to_InpainterBase(cv::videostab::InpaintingPipeline* instance) {
			return dynamic_cast<cv::videostab::InpainterBase*>(instance);
	}

	// cv::videostab::InpaintingPipeline::delete() generated
	// ("cv::videostab::InpaintingPipeline::delete", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_InpaintingPipeline_delete(cv::videostab::InpaintingPipeline* instance) {
			delete instance;
	}

	// KeypointBasedMotionEstimator(Ptr<MotionEstimatorBase>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/global_motion.hpp:224
	// ("cv::videostab::KeypointBasedMotionEstimator::KeypointBasedMotionEstimator", vec![(pred!(mut, ["estimator"], ["cv::Ptr<cv::videostab::MotionEstimatorBase>"]), _)]),
	void cv_videostab_KeypointBasedMotionEstimator_KeypointBasedMotionEstimator_PtrLMotionEstimatorBaseG(cv::Ptr<cv::videostab::MotionEstimatorBase>* estimator, Result<cv::videostab::KeypointBasedMotionEstimator*>* ocvrs_return) {
		try {
			cv::videostab::KeypointBasedMotionEstimator* ret = new cv::videostab::KeypointBasedMotionEstimator(*estimator);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMotionModel(MotionModel)(Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/global_motion.hpp:226
	// ("cv::videostab::KeypointBasedMotionEstimator::setMotionModel", vec![(pred!(mut, ["val"], ["cv::videostab::MotionModel"]), _)]),
	void cv_videostab_KeypointBasedMotionEstimator_setMotionModel_MotionModel(cv::videostab::KeypointBasedMotionEstimator* instance, cv::videostab::MotionModel val, ResultVoid* ocvrs_return) {
		try {
			instance->setMotionModel(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// motionModel()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/global_motion.hpp:227
	// ("cv::videostab::KeypointBasedMotionEstimator::motionModel", vec![(pred!(const, [], []), _)]),
	void cv_videostab_KeypointBasedMotionEstimator_motionModel_const(const cv::videostab::KeypointBasedMotionEstimator* instance, Result<cv::videostab::MotionModel>* ocvrs_return) {
		try {
			cv::videostab::MotionModel ret = instance->motionModel();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDetector(Ptr<FeatureDetector>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/global_motion.hpp:229
	// ("cv::videostab::KeypointBasedMotionEstimator::setDetector", vec![(pred!(mut, ["val"], ["cv::Ptr<cv::Feature2D>"]), _)]),
	void cv_videostab_KeypointBasedMotionEstimator_setDetector_PtrLFeature2DG(cv::videostab::KeypointBasedMotionEstimator* instance, cv::Ptr<cv::Feature2D>* val, ResultVoid* ocvrs_return) {
		try {
			instance->setDetector(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detector()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/global_motion.hpp:230
	// ("cv::videostab::KeypointBasedMotionEstimator::detector", vec![(pred!(const, [], []), _)]),
	void cv_videostab_KeypointBasedMotionEstimator_detector_const(const cv::videostab::KeypointBasedMotionEstimator* instance, Result<cv::Ptr<cv::Feature2D>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::Feature2D> ret = instance->detector();
			Ok(new cv::Ptr<cv::Feature2D>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setOpticalFlowEstimator(Ptr<ISparseOptFlowEstimator>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/global_motion.hpp:232
	// ("cv::videostab::KeypointBasedMotionEstimator::setOpticalFlowEstimator", vec![(pred!(mut, ["val"], ["cv::Ptr<cv::videostab::ISparseOptFlowEstimator>"]), _)]),
	void cv_videostab_KeypointBasedMotionEstimator_setOpticalFlowEstimator_PtrLISparseOptFlowEstimatorG(cv::videostab::KeypointBasedMotionEstimator* instance, cv::Ptr<cv::videostab::ISparseOptFlowEstimator>* val, ResultVoid* ocvrs_return) {
		try {
			instance->setOpticalFlowEstimator(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// opticalFlowEstimator()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/global_motion.hpp:233
	// ("cv::videostab::KeypointBasedMotionEstimator::opticalFlowEstimator", vec![(pred!(const, [], []), _)]),
	void cv_videostab_KeypointBasedMotionEstimator_opticalFlowEstimator_const(const cv::videostab::KeypointBasedMotionEstimator* instance, Result<cv::Ptr<cv::videostab::ISparseOptFlowEstimator>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::videostab::ISparseOptFlowEstimator> ret = instance->opticalFlowEstimator();
			Ok(new cv::Ptr<cv::videostab::ISparseOptFlowEstimator>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setOutlierRejector(Ptr<IOutlierRejector>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/global_motion.hpp:235
	// ("cv::videostab::KeypointBasedMotionEstimator::setOutlierRejector", vec![(pred!(mut, ["val"], ["cv::Ptr<cv::videostab::IOutlierRejector>"]), _)]),
	void cv_videostab_KeypointBasedMotionEstimator_setOutlierRejector_PtrLIOutlierRejectorG(cv::videostab::KeypointBasedMotionEstimator* instance, cv::Ptr<cv::videostab::IOutlierRejector>* val, ResultVoid* ocvrs_return) {
		try {
			instance->setOutlierRejector(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// outlierRejector()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/global_motion.hpp:236
	// ("cv::videostab::KeypointBasedMotionEstimator::outlierRejector", vec![(pred!(const, [], []), _)]),
	void cv_videostab_KeypointBasedMotionEstimator_outlierRejector_const(const cv::videostab::KeypointBasedMotionEstimator* instance, Result<cv::Ptr<cv::videostab::IOutlierRejector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::videostab::IOutlierRejector> ret = instance->outlierRejector();
			Ok(new cv::Ptr<cv::videostab::IOutlierRejector>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// estimate(const Mat &, const Mat &, bool *)(TraitClass, TraitClass, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/global_motion.hpp:238
	// ("cv::videostab::KeypointBasedMotionEstimator::estimate", vec![(pred!(mut, ["frame0", "frame1", "ok"], ["const cv::Mat*", "const cv::Mat*", "bool*"]), _)]),
	void cv_videostab_KeypointBasedMotionEstimator_estimate_const_MatR_const_MatR_boolX(cv::videostab::KeypointBasedMotionEstimator* instance, const cv::Mat* frame0, const cv::Mat* frame1, bool* ok, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->estimate(*frame0, *frame1, ok);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::KeypointBasedMotionEstimator::estimate(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/global_motion.hpp:238
	// ("cv::videostab::KeypointBasedMotionEstimator::estimate", vec![(pred!(mut, ["frame0", "frame1"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
	void cv_videostab_KeypointBasedMotionEstimator_estimate_const_MatR_const_MatR(cv::videostab::KeypointBasedMotionEstimator* instance, const cv::Mat* frame0, const cv::Mat* frame1, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->estimate(*frame0, *frame1);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// estimate(InputArray, InputArray, bool *)(InputArray, InputArray, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/global_motion.hpp:239
	// ("cv::videostab::KeypointBasedMotionEstimator::estimate", vec![(pred!(mut, ["frame0", "frame1", "ok"], ["const cv::_InputArray*", "const cv::_InputArray*", "bool*"]), _)]),
	void cv_videostab_KeypointBasedMotionEstimator_estimate_const__InputArrayR_const__InputArrayR_boolX(cv::videostab::KeypointBasedMotionEstimator* instance, const cv::_InputArray* frame0, const cv::_InputArray* frame1, bool* ok, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->estimate(*frame0, *frame1, ok);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::KeypointBasedMotionEstimator::estimate(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/global_motion.hpp:239
	// ("cv::videostab::KeypointBasedMotionEstimator::estimate", vec![(pred!(mut, ["frame0", "frame1"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_videostab_KeypointBasedMotionEstimator_estimate_const__InputArrayR_const__InputArrayR(cv::videostab::KeypointBasedMotionEstimator* instance, const cv::_InputArray* frame0, const cv::_InputArray* frame1, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->estimate(*frame0, *frame1);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::KeypointBasedMotionEstimator::to_ImageMotionEstimatorBase() generated
	// ("cv::videostab::KeypointBasedMotionEstimator::to_ImageMotionEstimatorBase", vec![(pred!(mut, [], []), _)]),
	cv::videostab::ImageMotionEstimatorBase* cv_videostab_KeypointBasedMotionEstimator_to_ImageMotionEstimatorBase(cv::videostab::KeypointBasedMotionEstimator* instance) {
			return dynamic_cast<cv::videostab::ImageMotionEstimatorBase*>(instance);
	}

	// cv::videostab::KeypointBasedMotionEstimator::delete() generated
	// ("cv::videostab::KeypointBasedMotionEstimator::delete", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_KeypointBasedMotionEstimator_delete(cv::videostab::KeypointBasedMotionEstimator* instance) {
			delete instance;
	}

	// print(const char *, ...)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/log.hpp:72
	// ("cv::videostab::LogToStdout::print", vec![(pred!(mut, ["format"], ["const char*"]), _)]),
	void cv_videostab_LogToStdout_print_const_charX(cv::videostab::LogToStdout* instance, const char* format, ResultVoid* ocvrs_return) {
		try {
			instance->print(format);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::LogToStdout::defaultNew() generated
	// ("cv::videostab::LogToStdout::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::videostab::LogToStdout* cv_videostab_LogToStdout_defaultNew_const() {
			cv::videostab::LogToStdout* ret = new cv::videostab::LogToStdout();
			return ret;
	}

	// cv::videostab::LogToStdout::to_ILog() generated
	// ("cv::videostab::LogToStdout::to_ILog", vec![(pred!(mut, [], []), _)]),
	cv::videostab::ILog* cv_videostab_LogToStdout_to_ILog(cv::videostab::LogToStdout* instance) {
			return dynamic_cast<cv::videostab::ILog*>(instance);
	}

	// cv::videostab::LogToStdout::delete() generated
	// ("cv::videostab::LogToStdout::delete", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_LogToStdout_delete(cv::videostab::LogToStdout* instance) {
			delete instance;
	}

	// LpMotionStabilizer(MotionModel)(Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/motion_stabilizing.hpp:120
	// ("cv::videostab::LpMotionStabilizer::LpMotionStabilizer", vec![(pred!(mut, ["model"], ["cv::videostab::MotionModel"]), _)]),
	void cv_videostab_LpMotionStabilizer_LpMotionStabilizer_MotionModel(cv::videostab::MotionModel model, Result<cv::videostab::LpMotionStabilizer*>* ocvrs_return) {
		try {
			cv::videostab::LpMotionStabilizer* ret = new cv::videostab::LpMotionStabilizer(model);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::LpMotionStabilizer::LpMotionStabilizer() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/motion_stabilizing.hpp:120
	// ("cv::videostab::LpMotionStabilizer::LpMotionStabilizer", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_LpMotionStabilizer_LpMotionStabilizer(Result<cv::videostab::LpMotionStabilizer*>* ocvrs_return) {
		try {
			cv::videostab::LpMotionStabilizer* ret = new cv::videostab::LpMotionStabilizer();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMotionModel(MotionModel)(Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/motion_stabilizing.hpp:122
	// ("cv::videostab::LpMotionStabilizer::setMotionModel", vec![(pred!(mut, ["val"], ["cv::videostab::MotionModel"]), _)]),
	void cv_videostab_LpMotionStabilizer_setMotionModel_MotionModel(cv::videostab::LpMotionStabilizer* instance, cv::videostab::MotionModel val, ResultVoid* ocvrs_return) {
		try {
			instance->setMotionModel(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// motionModel()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/motion_stabilizing.hpp:123
	// ("cv::videostab::LpMotionStabilizer::motionModel", vec![(pred!(const, [], []), _)]),
	void cv_videostab_LpMotionStabilizer_motionModel_const(const cv::videostab::LpMotionStabilizer* instance, Result<cv::videostab::MotionModel>* ocvrs_return) {
		try {
			cv::videostab::MotionModel ret = instance->motionModel();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setFrameSize(Size)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/motion_stabilizing.hpp:125
	// ("cv::videostab::LpMotionStabilizer::setFrameSize", vec![(pred!(mut, ["val"], ["cv::Size"]), _)]),
	void cv_videostab_LpMotionStabilizer_setFrameSize_Size(cv::videostab::LpMotionStabilizer* instance, cv::Size* val, ResultVoid* ocvrs_return) {
		try {
			instance->setFrameSize(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// frameSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/motion_stabilizing.hpp:126
	// ("cv::videostab::LpMotionStabilizer::frameSize", vec![(pred!(const, [], []), _)]),
	void cv_videostab_LpMotionStabilizer_frameSize_const(const cv::videostab::LpMotionStabilizer* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->frameSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTrimRatio(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/motion_stabilizing.hpp:128
	// ("cv::videostab::LpMotionStabilizer::setTrimRatio", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_videostab_LpMotionStabilizer_setTrimRatio_float(cv::videostab::LpMotionStabilizer* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setTrimRatio(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// trimRatio()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/motion_stabilizing.hpp:129
	// ("cv::videostab::LpMotionStabilizer::trimRatio", vec![(pred!(const, [], []), _)]),
	void cv_videostab_LpMotionStabilizer_trimRatio_const(const cv::videostab::LpMotionStabilizer* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->trimRatio();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWeight1(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/motion_stabilizing.hpp:131
	// ("cv::videostab::LpMotionStabilizer::setWeight1", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_videostab_LpMotionStabilizer_setWeight1_float(cv::videostab::LpMotionStabilizer* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setWeight1(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// weight1()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/motion_stabilizing.hpp:132
	// ("cv::videostab::LpMotionStabilizer::weight1", vec![(pred!(const, [], []), _)]),
	void cv_videostab_LpMotionStabilizer_weight1_const(const cv::videostab::LpMotionStabilizer* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->weight1();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWeight2(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/motion_stabilizing.hpp:134
	// ("cv::videostab::LpMotionStabilizer::setWeight2", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_videostab_LpMotionStabilizer_setWeight2_float(cv::videostab::LpMotionStabilizer* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setWeight2(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// weight2()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/motion_stabilizing.hpp:135
	// ("cv::videostab::LpMotionStabilizer::weight2", vec![(pred!(const, [], []), _)]),
	void cv_videostab_LpMotionStabilizer_weight2_const(const cv::videostab::LpMotionStabilizer* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->weight2();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWeight3(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/motion_stabilizing.hpp:137
	// ("cv::videostab::LpMotionStabilizer::setWeight3", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_videostab_LpMotionStabilizer_setWeight3_float(cv::videostab::LpMotionStabilizer* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setWeight3(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// weight3()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/motion_stabilizing.hpp:138
	// ("cv::videostab::LpMotionStabilizer::weight3", vec![(pred!(const, [], []), _)]),
	void cv_videostab_LpMotionStabilizer_weight3_const(const cv::videostab::LpMotionStabilizer* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->weight3();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWeight4(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/motion_stabilizing.hpp:140
	// ("cv::videostab::LpMotionStabilizer::setWeight4", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_videostab_LpMotionStabilizer_setWeight4_float(cv::videostab::LpMotionStabilizer* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setWeight4(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// weight4()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/motion_stabilizing.hpp:141
	// ("cv::videostab::LpMotionStabilizer::weight4", vec![(pred!(const, [], []), _)]),
	void cv_videostab_LpMotionStabilizer_weight4_const(const cv::videostab::LpMotionStabilizer* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->weight4();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// stabilize(int, const std::vector<Mat> &, std::pair<int, int>, Mat *)(Primitive, CppPassByVoidPtr, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/motion_stabilizing.hpp:143
	// ("cv::videostab::LpMotionStabilizer::stabilize", vec![(pred!(mut, ["size", "motions", "range", "stabilizationMotions"], ["int", "const std::vector<cv::Mat>*", "std::pair<int, int>", "cv::Mat*"]), _)]),
	void cv_videostab_LpMotionStabilizer_stabilize_int_const_vectorLMatGR_pairLint__intG_MatX(cv::videostab::LpMotionStabilizer* instance, int size, const std::vector<cv::Mat>* motions, std::pair<int, int>* range, cv::Mat* stabilizationMotions, ResultVoid* ocvrs_return) {
		try {
			instance->stabilize(size, *motions, *range, stabilizationMotions);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::LpMotionStabilizer::to_IMotionStabilizer() generated
	// ("cv::videostab::LpMotionStabilizer::to_IMotionStabilizer", vec![(pred!(mut, [], []), _)]),
	cv::videostab::IMotionStabilizer* cv_videostab_LpMotionStabilizer_to_IMotionStabilizer(cv::videostab::LpMotionStabilizer* instance) {
			return dynamic_cast<cv::videostab::IMotionStabilizer*>(instance);
	}

	// cv::videostab::LpMotionStabilizer::delete() generated
	// ("cv::videostab::LpMotionStabilizer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_LpMotionStabilizer_delete(cv::videostab::LpMotionStabilizer* instance) {
			delete instance;
	}

	// suppress(int, const Mat &, Mat &)(Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/wobble_suppression.hpp:116
	// ("cv::videostab::MoreAccurateMotionWobbleSuppressor::suppress", vec![(pred!(mut, ["idx", "frame", "result"], ["int", "const cv::Mat*", "cv::Mat*"]), _)]),
	void cv_videostab_MoreAccurateMotionWobbleSuppressor_suppress_int_const_MatR_MatR(cv::videostab::MoreAccurateMotionWobbleSuppressor* instance, int idx, const cv::Mat* frame, cv::Mat* result, ResultVoid* ocvrs_return) {
		try {
			instance->suppress(idx, *frame, *result);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::MoreAccurateMotionWobbleSuppressor::defaultNew() generated
	// ("cv::videostab::MoreAccurateMotionWobbleSuppressor::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::videostab::MoreAccurateMotionWobbleSuppressor* cv_videostab_MoreAccurateMotionWobbleSuppressor_defaultNew_const() {
			cv::videostab::MoreAccurateMotionWobbleSuppressor* ret = new cv::videostab::MoreAccurateMotionWobbleSuppressor();
			return ret;
	}

	// cv::videostab::MoreAccurateMotionWobbleSuppressor::to_MoreAccurateMotionWobbleSuppressorBase() generated
	// ("cv::videostab::MoreAccurateMotionWobbleSuppressor::to_MoreAccurateMotionWobbleSuppressorBase", vec![(pred!(mut, [], []), _)]),
	cv::videostab::MoreAccurateMotionWobbleSuppressorBase* cv_videostab_MoreAccurateMotionWobbleSuppressor_to_MoreAccurateMotionWobbleSuppressorBase(cv::videostab::MoreAccurateMotionWobbleSuppressor* instance) {
			return dynamic_cast<cv::videostab::MoreAccurateMotionWobbleSuppressorBase*>(instance);
	}

	// cv::videostab::MoreAccurateMotionWobbleSuppressor::to_WobbleSuppressorBase() generated
	// ("cv::videostab::MoreAccurateMotionWobbleSuppressor::to_WobbleSuppressorBase", vec![(pred!(mut, [], []), _)]),
	cv::videostab::WobbleSuppressorBase* cv_videostab_MoreAccurateMotionWobbleSuppressor_to_WobbleSuppressorBase(cv::videostab::MoreAccurateMotionWobbleSuppressor* instance) {
			return dynamic_cast<cv::videostab::WobbleSuppressorBase*>(instance);
	}

	// cv::videostab::MoreAccurateMotionWobbleSuppressor::delete() generated
	// ("cv::videostab::MoreAccurateMotionWobbleSuppressor::delete", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_MoreAccurateMotionWobbleSuppressor_delete(cv::videostab::MoreAccurateMotionWobbleSuppressor* instance) {
			delete instance;
	}

	// setPeriod(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/wobble_suppression.hpp:104
	// ("cv::videostab::MoreAccurateMotionWobbleSuppressorBase::setPeriod", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_videostab_MoreAccurateMotionWobbleSuppressorBase_setPeriod_int(cv::videostab::MoreAccurateMotionWobbleSuppressorBase* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setPeriod(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// period()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/wobble_suppression.hpp:105
	// ("cv::videostab::MoreAccurateMotionWobbleSuppressorBase::period", vec![(pred!(const, [], []), _)]),
	void cv_videostab_MoreAccurateMotionWobbleSuppressorBase_period_const(const cv::videostab::MoreAccurateMotionWobbleSuppressorBase* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->period();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::MoreAccurateMotionWobbleSuppressorBase::to_MoreAccurateMotionWobbleSuppressor() generated
	// ("cv::videostab::MoreAccurateMotionWobbleSuppressorBase::to_MoreAccurateMotionWobbleSuppressor", vec![(pred!(mut, [], []), _)]),
	cv::videostab::MoreAccurateMotionWobbleSuppressor* cv_videostab_MoreAccurateMotionWobbleSuppressorBase_to_MoreAccurateMotionWobbleSuppressor(cv::videostab::MoreAccurateMotionWobbleSuppressorBase* instance) {
			return dynamic_cast<cv::videostab::MoreAccurateMotionWobbleSuppressor*>(instance);
	}

	// cv::videostab::MoreAccurateMotionWobbleSuppressorBase::to_WobbleSuppressorBase() generated
	// ("cv::videostab::MoreAccurateMotionWobbleSuppressorBase::to_WobbleSuppressorBase", vec![(pred!(mut, [], []), _)]),
	cv::videostab::WobbleSuppressorBase* cv_videostab_MoreAccurateMotionWobbleSuppressorBase_to_WobbleSuppressorBase(cv::videostab::MoreAccurateMotionWobbleSuppressorBase* instance) {
			return dynamic_cast<cv::videostab::WobbleSuppressorBase*>(instance);
	}

	// cv::videostab::MoreAccurateMotionWobbleSuppressorBase::delete() generated
	// ("cv::videostab::MoreAccurateMotionWobbleSuppressorBase::delete", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_MoreAccurateMotionWobbleSuppressorBase_delete(cv::videostab::MoreAccurateMotionWobbleSuppressorBase* instance) {
			delete instance;
	}

	// setMotionModel(MotionModel)(Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/global_motion.hpp:106
	// ("cv::videostab::MotionEstimatorBase::setMotionModel", vec![(pred!(mut, ["val"], ["cv::videostab::MotionModel"]), _)]),
	void cv_videostab_MotionEstimatorBase_setMotionModel_MotionModel(cv::videostab::MotionEstimatorBase* instance, cv::videostab::MotionModel val, ResultVoid* ocvrs_return) {
		try {
			instance->setMotionModel(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// motionModel()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/global_motion.hpp:111
	// ("cv::videostab::MotionEstimatorBase::motionModel", vec![(pred!(const, [], []), _)]),
	void cv_videostab_MotionEstimatorBase_motionModel_const(const cv::videostab::MotionEstimatorBase* instance, Result<cv::videostab::MotionModel>* ocvrs_return) {
		try {
			cv::videostab::MotionModel ret = instance->motionModel();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// estimate(InputArray, InputArray, bool *)(InputArray, InputArray, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/global_motion.hpp:120
	// ("cv::videostab::MotionEstimatorBase::estimate", vec![(pred!(mut, ["points0", "points1", "ok"], ["const cv::_InputArray*", "const cv::_InputArray*", "bool*"]), _)]),
	void cv_videostab_MotionEstimatorBase_estimate_const__InputArrayR_const__InputArrayR_boolX(cv::videostab::MotionEstimatorBase* instance, const cv::_InputArray* points0, const cv::_InputArray* points1, bool* ok, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->estimate(*points0, *points1, ok);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::MotionEstimatorBase::estimate(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/global_motion.hpp:120
	// ("cv::videostab::MotionEstimatorBase::estimate", vec![(pred!(mut, ["points0", "points1"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_videostab_MotionEstimatorBase_estimate_const__InputArrayR_const__InputArrayR(cv::videostab::MotionEstimatorBase* instance, const cv::_InputArray* points0, const cv::_InputArray* points1, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->estimate(*points0, *points1);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::MotionEstimatorBase::to_MotionEstimatorL1() generated
	// ("cv::videostab::MotionEstimatorBase::to_MotionEstimatorL1", vec![(pred!(mut, [], []), _)]),
	cv::videostab::MotionEstimatorL1* cv_videostab_MotionEstimatorBase_to_MotionEstimatorL1(cv::videostab::MotionEstimatorBase* instance) {
			return dynamic_cast<cv::videostab::MotionEstimatorL1*>(instance);
	}

	// cv::videostab::MotionEstimatorBase::to_MotionEstimatorRansacL2() generated
	// ("cv::videostab::MotionEstimatorBase::to_MotionEstimatorRansacL2", vec![(pred!(mut, [], []), _)]),
	cv::videostab::MotionEstimatorRansacL2* cv_videostab_MotionEstimatorBase_to_MotionEstimatorRansacL2(cv::videostab::MotionEstimatorBase* instance) {
			return dynamic_cast<cv::videostab::MotionEstimatorRansacL2*>(instance);
	}

	// cv::videostab::MotionEstimatorBase::delete() generated
	// ("cv::videostab::MotionEstimatorBase::delete", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_MotionEstimatorBase_delete(cv::videostab::MotionEstimatorBase* instance) {
			delete instance;
	}

	// MotionEstimatorL1(MotionModel)(Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/global_motion.hpp:156
	// ("cv::videostab::MotionEstimatorL1::MotionEstimatorL1", vec![(pred!(mut, ["model"], ["cv::videostab::MotionModel"]), _)]),
	void cv_videostab_MotionEstimatorL1_MotionEstimatorL1_MotionModel(cv::videostab::MotionModel model, Result<cv::videostab::MotionEstimatorL1*>* ocvrs_return) {
		try {
			cv::videostab::MotionEstimatorL1* ret = new cv::videostab::MotionEstimatorL1(model);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::MotionEstimatorL1::MotionEstimatorL1() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/global_motion.hpp:156
	// ("cv::videostab::MotionEstimatorL1::MotionEstimatorL1", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_MotionEstimatorL1_MotionEstimatorL1(Result<cv::videostab::MotionEstimatorL1*>* ocvrs_return) {
		try {
			cv::videostab::MotionEstimatorL1* ret = new cv::videostab::MotionEstimatorL1();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// estimate(InputArray, InputArray, bool *)(InputArray, InputArray, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/global_motion.hpp:158
	// ("cv::videostab::MotionEstimatorL1::estimate", vec![(pred!(mut, ["points0", "points1", "ok"], ["const cv::_InputArray*", "const cv::_InputArray*", "bool*"]), _)]),
	void cv_videostab_MotionEstimatorL1_estimate_const__InputArrayR_const__InputArrayR_boolX(cv::videostab::MotionEstimatorL1* instance, const cv::_InputArray* points0, const cv::_InputArray* points1, bool* ok, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->estimate(*points0, *points1, ok);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::MotionEstimatorL1::estimate(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/global_motion.hpp:158
	// ("cv::videostab::MotionEstimatorL1::estimate", vec![(pred!(mut, ["points0", "points1"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_videostab_MotionEstimatorL1_estimate_const__InputArrayR_const__InputArrayR(cv::videostab::MotionEstimatorL1* instance, const cv::_InputArray* points0, const cv::_InputArray* points1, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->estimate(*points0, *points1);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::MotionEstimatorL1::to_MotionEstimatorBase() generated
	// ("cv::videostab::MotionEstimatorL1::to_MotionEstimatorBase", vec![(pred!(mut, [], []), _)]),
	cv::videostab::MotionEstimatorBase* cv_videostab_MotionEstimatorL1_to_MotionEstimatorBase(cv::videostab::MotionEstimatorL1* instance) {
			return dynamic_cast<cv::videostab::MotionEstimatorBase*>(instance);
	}

	// cv::videostab::MotionEstimatorL1::delete() generated
	// ("cv::videostab::MotionEstimatorL1::delete", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_MotionEstimatorL1_delete(cv::videostab::MotionEstimatorL1* instance) {
			delete instance;
	}

	// MotionEstimatorRansacL2(MotionModel)(Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/global_motion.hpp:134
	// ("cv::videostab::MotionEstimatorRansacL2::MotionEstimatorRansacL2", vec![(pred!(mut, ["model"], ["cv::videostab::MotionModel"]), _)]),
	void cv_videostab_MotionEstimatorRansacL2_MotionEstimatorRansacL2_MotionModel(cv::videostab::MotionModel model, Result<cv::videostab::MotionEstimatorRansacL2*>* ocvrs_return) {
		try {
			cv::videostab::MotionEstimatorRansacL2* ret = new cv::videostab::MotionEstimatorRansacL2(model);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::MotionEstimatorRansacL2::MotionEstimatorRansacL2() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/global_motion.hpp:134
	// ("cv::videostab::MotionEstimatorRansacL2::MotionEstimatorRansacL2", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_MotionEstimatorRansacL2_MotionEstimatorRansacL2(Result<cv::videostab::MotionEstimatorRansacL2*>* ocvrs_return) {
		try {
			cv::videostab::MotionEstimatorRansacL2* ret = new cv::videostab::MotionEstimatorRansacL2();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setRansacParams(const RansacParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/global_motion.hpp:136
	// ("cv::videostab::MotionEstimatorRansacL2::setRansacParams", vec![(pred!(mut, ["val"], ["const cv::videostab::RansacParams*"]), _)]),
	void cv_videostab_MotionEstimatorRansacL2_setRansacParams_const_RansacParamsR(cv::videostab::MotionEstimatorRansacL2* instance, const cv::videostab::RansacParams* val, ResultVoid* ocvrs_return) {
		try {
			instance->setRansacParams(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ransacParams()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/global_motion.hpp:137
	// ("cv::videostab::MotionEstimatorRansacL2::ransacParams", vec![(pred!(const, [], []), _)]),
	void cv_videostab_MotionEstimatorRansacL2_ransacParams_const(const cv::videostab::MotionEstimatorRansacL2* instance, Result<cv::videostab::RansacParams*>* ocvrs_return) {
		try {
			cv::videostab::RansacParams ret = instance->ransacParams();
			Ok(new cv::videostab::RansacParams(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMinInlierRatio(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/global_motion.hpp:139
	// ("cv::videostab::MotionEstimatorRansacL2::setMinInlierRatio", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_videostab_MotionEstimatorRansacL2_setMinInlierRatio_float(cv::videostab::MotionEstimatorRansacL2* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setMinInlierRatio(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// minInlierRatio()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/global_motion.hpp:140
	// ("cv::videostab::MotionEstimatorRansacL2::minInlierRatio", vec![(pred!(const, [], []), _)]),
	void cv_videostab_MotionEstimatorRansacL2_minInlierRatio_const(const cv::videostab::MotionEstimatorRansacL2* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->minInlierRatio();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// estimate(InputArray, InputArray, bool *)(InputArray, InputArray, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/global_motion.hpp:142
	// ("cv::videostab::MotionEstimatorRansacL2::estimate", vec![(pred!(mut, ["points0", "points1", "ok"], ["const cv::_InputArray*", "const cv::_InputArray*", "bool*"]), _)]),
	void cv_videostab_MotionEstimatorRansacL2_estimate_const__InputArrayR_const__InputArrayR_boolX(cv::videostab::MotionEstimatorRansacL2* instance, const cv::_InputArray* points0, const cv::_InputArray* points1, bool* ok, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->estimate(*points0, *points1, ok);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::MotionEstimatorRansacL2::estimate(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/global_motion.hpp:142
	// ("cv::videostab::MotionEstimatorRansacL2::estimate", vec![(pred!(mut, ["points0", "points1"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_videostab_MotionEstimatorRansacL2_estimate_const__InputArrayR_const__InputArrayR(cv::videostab::MotionEstimatorRansacL2* instance, const cv::_InputArray* points0, const cv::_InputArray* points1, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->estimate(*points0, *points1);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::MotionEstimatorRansacL2::to_MotionEstimatorBase() generated
	// ("cv::videostab::MotionEstimatorRansacL2::to_MotionEstimatorBase", vec![(pred!(mut, [], []), _)]),
	cv::videostab::MotionEstimatorBase* cv_videostab_MotionEstimatorRansacL2_to_MotionEstimatorBase(cv::videostab::MotionEstimatorRansacL2* instance) {
			return dynamic_cast<cv::videostab::MotionEstimatorBase*>(instance);
	}

	// cv::videostab::MotionEstimatorRansacL2::delete() generated
	// ("cv::videostab::MotionEstimatorRansacL2::delete", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_MotionEstimatorRansacL2_delete(cv::videostab::MotionEstimatorRansacL2* instance) {
			delete instance;
	}

	// stabilize(int, const std::vector<Mat> &, std::pair<int, int>)(Primitive, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/motion_stabilizing.hpp:89
	// ("cv::videostab::MotionFilterBase::stabilize", vec![(pred!(mut, ["idx", "motions", "range"], ["int", "const std::vector<cv::Mat>*", "std::pair<int, int>"]), _)]),
	void cv_videostab_MotionFilterBase_stabilize_int_const_vectorLMatGR_pairLint__intG(cv::videostab::MotionFilterBase* instance, int idx, const std::vector<cv::Mat>* motions, std::pair<int, int>* range, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->stabilize(idx, *motions, *range);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// stabilize(int, const std::vector<Mat> &, std::pair<int, int>, Mat *)(Primitive, CppPassByVoidPtr, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/motion_stabilizing.hpp:92
	// ("cv::videostab::MotionFilterBase::stabilize", vec![(pred!(mut, ["size", "motions", "range", "stabilizationMotions"], ["int", "const std::vector<cv::Mat>*", "std::pair<int, int>", "cv::Mat*"]), _)]),
	void cv_videostab_MotionFilterBase_stabilize_int_const_vectorLMatGR_pairLint__intG_MatX(cv::videostab::MotionFilterBase* instance, int size, const std::vector<cv::Mat>* motions, std::pair<int, int>* range, cv::Mat* stabilizationMotions, ResultVoid* ocvrs_return) {
		try {
			instance->stabilize(size, *motions, *range, stabilizationMotions);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::MotionFilterBase::to_GaussianMotionFilter() generated
	// ("cv::videostab::MotionFilterBase::to_GaussianMotionFilter", vec![(pred!(mut, [], []), _)]),
	cv::videostab::GaussianMotionFilter* cv_videostab_MotionFilterBase_to_GaussianMotionFilter(cv::videostab::MotionFilterBase* instance) {
			return dynamic_cast<cv::videostab::GaussianMotionFilter*>(instance);
	}

	// cv::videostab::MotionFilterBase::to_IMotionStabilizer() generated
	// ("cv::videostab::MotionFilterBase::to_IMotionStabilizer", vec![(pred!(mut, [], []), _)]),
	cv::videostab::IMotionStabilizer* cv_videostab_MotionFilterBase_to_IMotionStabilizer(cv::videostab::MotionFilterBase* instance) {
			return dynamic_cast<cv::videostab::IMotionStabilizer*>(instance);
	}

	// cv::videostab::MotionFilterBase::delete() generated
	// ("cv::videostab::MotionFilterBase::delete", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_MotionFilterBase_delete(cv::videostab::MotionFilterBase* instance) {
			delete instance;
	}

	// MotionInpainter()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/inpainting.hpp:144
	// ("cv::videostab::MotionInpainter::MotionInpainter", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_MotionInpainter_MotionInpainter(Result<cv::videostab::MotionInpainter*>* ocvrs_return) {
		try {
			cv::videostab::MotionInpainter* ret = new cv::videostab::MotionInpainter();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setOptFlowEstimator(Ptr<IDenseOptFlowEstimator>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/inpainting.hpp:146
	// ("cv::videostab::MotionInpainter::setOptFlowEstimator", vec![(pred!(mut, ["val"], ["cv::Ptr<cv::videostab::IDenseOptFlowEstimator>"]), _)]),
	void cv_videostab_MotionInpainter_setOptFlowEstimator_PtrLIDenseOptFlowEstimatorG(cv::videostab::MotionInpainter* instance, cv::Ptr<cv::videostab::IDenseOptFlowEstimator>* val, ResultVoid* ocvrs_return) {
		try {
			instance->setOptFlowEstimator(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// optFlowEstimator()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/inpainting.hpp:147
	// ("cv::videostab::MotionInpainter::optFlowEstimator", vec![(pred!(const, [], []), _)]),
	void cv_videostab_MotionInpainter_optFlowEstimator_const(const cv::videostab::MotionInpainter* instance, Result<cv::Ptr<cv::videostab::IDenseOptFlowEstimator>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::videostab::IDenseOptFlowEstimator> ret = instance->optFlowEstimator();
			Ok(new cv::Ptr<cv::videostab::IDenseOptFlowEstimator>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setFlowErrorThreshold(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/inpainting.hpp:149
	// ("cv::videostab::MotionInpainter::setFlowErrorThreshold", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_videostab_MotionInpainter_setFlowErrorThreshold_float(cv::videostab::MotionInpainter* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setFlowErrorThreshold(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// flowErrorThreshold()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/inpainting.hpp:150
	// ("cv::videostab::MotionInpainter::flowErrorThreshold", vec![(pred!(const, [], []), _)]),
	void cv_videostab_MotionInpainter_flowErrorThreshold_const(const cv::videostab::MotionInpainter* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->flowErrorThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDistThreshold(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/inpainting.hpp:152
	// ("cv::videostab::MotionInpainter::setDistThreshold", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_videostab_MotionInpainter_setDistThreshold_float(cv::videostab::MotionInpainter* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setDistThreshold(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// distThresh()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/inpainting.hpp:153
	// ("cv::videostab::MotionInpainter::distThresh", vec![(pred!(const, [], []), _)]),
	void cv_videostab_MotionInpainter_distThresh_const(const cv::videostab::MotionInpainter* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->distThresh();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setBorderMode(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/inpainting.hpp:155
	// ("cv::videostab::MotionInpainter::setBorderMode", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_videostab_MotionInpainter_setBorderMode_int(cv::videostab::MotionInpainter* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setBorderMode(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// borderMode()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/inpainting.hpp:156
	// ("cv::videostab::MotionInpainter::borderMode", vec![(pred!(const, [], []), _)]),
	void cv_videostab_MotionInpainter_borderMode_const(const cv::videostab::MotionInpainter* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->borderMode();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// inpaint(int, Mat &, Mat &)(Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/inpainting.hpp:158
	// ("cv::videostab::MotionInpainter::inpaint", vec![(pred!(mut, ["idx", "frame", "mask"], ["int", "cv::Mat*", "cv::Mat*"]), _)]),
	void cv_videostab_MotionInpainter_inpaint_int_MatR_MatR(cv::videostab::MotionInpainter* instance, int idx, cv::Mat* frame, cv::Mat* mask, ResultVoid* ocvrs_return) {
		try {
			instance->inpaint(idx, *frame, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::MotionInpainter::to_InpainterBase() generated
	// ("cv::videostab::MotionInpainter::to_InpainterBase", vec![(pred!(mut, [], []), _)]),
	cv::videostab::InpainterBase* cv_videostab_MotionInpainter_to_InpainterBase(cv::videostab::MotionInpainter* instance) {
			return dynamic_cast<cv::videostab::InpainterBase*>(instance);
	}

	// cv::videostab::MotionInpainter::delete() generated
	// ("cv::videostab::MotionInpainter::delete", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_MotionInpainter_delete(cv::videostab::MotionInpainter* instance) {
			delete instance;
	}

	// pushBack(Ptr<IMotionStabilizer>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/motion_stabilizing.hpp:73
	// ("cv::videostab::MotionStabilizationPipeline::pushBack", vec![(pred!(mut, ["stabilizer"], ["cv::Ptr<cv::videostab::IMotionStabilizer>"]), _)]),
	void cv_videostab_MotionStabilizationPipeline_pushBack_PtrLIMotionStabilizerG(cv::videostab::MotionStabilizationPipeline* instance, cv::Ptr<cv::videostab::IMotionStabilizer>* stabilizer, ResultVoid* ocvrs_return) {
		try {
			instance->pushBack(*stabilizer);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// empty()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/motion_stabilizing.hpp:74
	// ("cv::videostab::MotionStabilizationPipeline::empty", vec![(pred!(const, [], []), _)]),
	void cv_videostab_MotionStabilizationPipeline_empty_const(const cv::videostab::MotionStabilizationPipeline* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// stabilize(int, const std::vector<Mat> &, std::pair<int, int>, Mat *)(Primitive, CppPassByVoidPtr, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/motion_stabilizing.hpp:76
	// ("cv::videostab::MotionStabilizationPipeline::stabilize", vec![(pred!(mut, ["size", "motions", "range", "stabilizationMotions"], ["int", "const std::vector<cv::Mat>*", "std::pair<int, int>", "cv::Mat*"]), _)]),
	void cv_videostab_MotionStabilizationPipeline_stabilize_int_const_vectorLMatGR_pairLint__intG_MatX(cv::videostab::MotionStabilizationPipeline* instance, int size, const std::vector<cv::Mat>* motions, std::pair<int, int>* range, cv::Mat* stabilizationMotions, ResultVoid* ocvrs_return) {
		try {
			instance->stabilize(size, *motions, *range, stabilizationMotions);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::MotionStabilizationPipeline::defaultNew() generated
	// ("cv::videostab::MotionStabilizationPipeline::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::videostab::MotionStabilizationPipeline* cv_videostab_MotionStabilizationPipeline_defaultNew_const() {
			cv::videostab::MotionStabilizationPipeline* ret = new cv::videostab::MotionStabilizationPipeline();
			return ret;
	}

	// cv::videostab::MotionStabilizationPipeline::to_IMotionStabilizer() generated
	// ("cv::videostab::MotionStabilizationPipeline::to_IMotionStabilizer", vec![(pred!(mut, [], []), _)]),
	cv::videostab::IMotionStabilizer* cv_videostab_MotionStabilizationPipeline_to_IMotionStabilizer(cv::videostab::MotionStabilizationPipeline* instance) {
			return dynamic_cast<cv::videostab::IMotionStabilizer*>(instance);
	}

	// cv::videostab::MotionStabilizationPipeline::delete() generated
	// ("cv::videostab::MotionStabilizationPipeline::delete", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_MotionStabilizationPipeline_delete(cv::videostab::MotionStabilizationPipeline* instance) {
			delete instance;
	}

	// deblur(int, Mat &)(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/deblurring.hpp:93
	// ("cv::videostab::NullDeblurer::deblur", vec![(pred!(mut, ["unnamed", "unnamed"], ["int", "cv::Mat*"]), _)]),
	void cv_videostab_NullDeblurer_deblur_int_MatR(cv::videostab::NullDeblurer* instance, int unnamed, cv::Mat* unnamed_1, ResultVoid* ocvrs_return) {
		try {
			instance->deblur(unnamed, *unnamed_1);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::NullDeblurer::defaultNew() generated
	// ("cv::videostab::NullDeblurer::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::videostab::NullDeblurer* cv_videostab_NullDeblurer_defaultNew_const() {
			cv::videostab::NullDeblurer* ret = new cv::videostab::NullDeblurer();
			return ret;
	}

	// cv::videostab::NullDeblurer::to_DeblurerBase() generated
	// ("cv::videostab::NullDeblurer::to_DeblurerBase", vec![(pred!(mut, [], []), _)]),
	cv::videostab::DeblurerBase* cv_videostab_NullDeblurer_to_DeblurerBase(cv::videostab::NullDeblurer* instance) {
			return dynamic_cast<cv::videostab::DeblurerBase*>(instance);
	}

	// cv::videostab::NullDeblurer::delete() generated
	// ("cv::videostab::NullDeblurer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_NullDeblurer_delete(cv::videostab::NullDeblurer* instance) {
			delete instance;
	}

	// reset()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/frame_source.hpp:68
	// ("cv::videostab::NullFrameSource::reset", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_NullFrameSource_reset(cv::videostab::NullFrameSource* instance, ResultVoid* ocvrs_return) {
		try {
			instance->reset();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// nextFrame()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/frame_source.hpp:69
	// ("cv::videostab::NullFrameSource::nextFrame", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_NullFrameSource_nextFrame(cv::videostab::NullFrameSource* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->nextFrame();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::NullFrameSource::defaultNew() generated
	// ("cv::videostab::NullFrameSource::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::videostab::NullFrameSource* cv_videostab_NullFrameSource_defaultNew_const() {
			cv::videostab::NullFrameSource* ret = new cv::videostab::NullFrameSource();
			return ret;
	}

	// cv::videostab::NullFrameSource::to_IFrameSource() generated
	// ("cv::videostab::NullFrameSource::to_IFrameSource", vec![(pred!(mut, [], []), _)]),
	cv::videostab::IFrameSource* cv_videostab_NullFrameSource_to_IFrameSource(cv::videostab::NullFrameSource* instance) {
			return dynamic_cast<cv::videostab::IFrameSource*>(instance);
	}

	// cv::videostab::NullFrameSource::delete() generated
	// ("cv::videostab::NullFrameSource::delete", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_NullFrameSource_delete(cv::videostab::NullFrameSource* instance) {
			delete instance;
	}

	// inpaint(int, Mat &, Mat &)(Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/inpainting.hpp:105
	// ("cv::videostab::NullInpainter::inpaint", vec![(pred!(mut, ["unnamed", "unnamed", "unnamed"], ["int", "cv::Mat*", "cv::Mat*"]), _)]),
	void cv_videostab_NullInpainter_inpaint_int_MatR_MatR(cv::videostab::NullInpainter* instance, int unnamed, cv::Mat* unnamed_1, cv::Mat* unnamed_2, ResultVoid* ocvrs_return) {
		try {
			instance->inpaint(unnamed, *unnamed_1, *unnamed_2);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::NullInpainter::defaultNew() generated
	// ("cv::videostab::NullInpainter::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::videostab::NullInpainter* cv_videostab_NullInpainter_defaultNew_const() {
			cv::videostab::NullInpainter* ret = new cv::videostab::NullInpainter();
			return ret;
	}

	// cv::videostab::NullInpainter::to_InpainterBase() generated
	// ("cv::videostab::NullInpainter::to_InpainterBase", vec![(pred!(mut, [], []), _)]),
	cv::videostab::InpainterBase* cv_videostab_NullInpainter_to_InpainterBase(cv::videostab::NullInpainter* instance) {
			return dynamic_cast<cv::videostab::InpainterBase*>(instance);
	}

	// cv::videostab::NullInpainter::delete() generated
	// ("cv::videostab::NullInpainter::delete", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_NullInpainter_delete(cv::videostab::NullInpainter* instance) {
			delete instance;
	}

	// print(const char *, ...)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/log.hpp:66
	// ("cv::videostab::NullLog::print", vec![(pred!(mut, ["unnamed"], ["const char*"]), _)]),
	void cv_videostab_NullLog_print_const_charX(cv::videostab::NullLog* instance, const char* unnamed, ResultVoid* ocvrs_return) {
		try {
			instance->print(unnamed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::NullLog::defaultNew() generated
	// ("cv::videostab::NullLog::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::videostab::NullLog* cv_videostab_NullLog_defaultNew_const() {
			cv::videostab::NullLog* ret = new cv::videostab::NullLog();
			return ret;
	}

	// cv::videostab::NullLog::to_ILog() generated
	// ("cv::videostab::NullLog::to_ILog", vec![(pred!(mut, [], []), _)]),
	cv::videostab::ILog* cv_videostab_NullLog_to_ILog(cv::videostab::NullLog* instance) {
			return dynamic_cast<cv::videostab::ILog*>(instance);
	}

	// cv::videostab::NullLog::delete() generated
	// ("cv::videostab::NullLog::delete", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_NullLog_delete(cv::videostab::NullLog* instance) {
			delete instance;
	}

	// process(Size, InputArray, InputArray, OutputArray)(SimpleClass, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/outlier_rejection.hpp:70
	// ("cv::videostab::NullOutlierRejector::process", vec![(pred!(mut, ["frameSize", "points0", "points1", "mask"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_videostab_NullOutlierRejector_process_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR(cv::videostab::NullOutlierRejector* instance, cv::Size* frameSize, const cv::_InputArray* points0, const cv::_InputArray* points1, const cv::_OutputArray* mask, ResultVoid* ocvrs_return) {
		try {
			instance->process(*frameSize, *points0, *points1, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::NullOutlierRejector::defaultNew() generated
	// ("cv::videostab::NullOutlierRejector::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::videostab::NullOutlierRejector* cv_videostab_NullOutlierRejector_defaultNew_const() {
			cv::videostab::NullOutlierRejector* ret = new cv::videostab::NullOutlierRejector();
			return ret;
	}

	// cv::videostab::NullOutlierRejector::to_IOutlierRejector() generated
	// ("cv::videostab::NullOutlierRejector::to_IOutlierRejector", vec![(pred!(mut, [], []), _)]),
	cv::videostab::IOutlierRejector* cv_videostab_NullOutlierRejector_to_IOutlierRejector(cv::videostab::NullOutlierRejector* instance) {
			return dynamic_cast<cv::videostab::IOutlierRejector*>(instance);
	}

	// cv::videostab::NullOutlierRejector::delete() generated
	// ("cv::videostab::NullOutlierRejector::delete", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_NullOutlierRejector_delete(cv::videostab::NullOutlierRejector* instance) {
			delete instance;
	}

	// suppress(int, const Mat &, Mat &)(Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/wobble_suppression.hpp:98
	// ("cv::videostab::NullWobbleSuppressor::suppress", vec![(pred!(mut, ["idx", "frame", "result"], ["int", "const cv::Mat*", "cv::Mat*"]), _)]),
	void cv_videostab_NullWobbleSuppressor_suppress_int_const_MatR_MatR(cv::videostab::NullWobbleSuppressor* instance, int idx, const cv::Mat* frame, cv::Mat* result, ResultVoid* ocvrs_return) {
		try {
			instance->suppress(idx, *frame, *result);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::NullWobbleSuppressor::defaultNew() generated
	// ("cv::videostab::NullWobbleSuppressor::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::videostab::NullWobbleSuppressor* cv_videostab_NullWobbleSuppressor_defaultNew_const() {
			cv::videostab::NullWobbleSuppressor* ret = new cv::videostab::NullWobbleSuppressor();
			return ret;
	}

	// cv::videostab::NullWobbleSuppressor::to_WobbleSuppressorBase() generated
	// ("cv::videostab::NullWobbleSuppressor::to_WobbleSuppressorBase", vec![(pred!(mut, [], []), _)]),
	cv::videostab::WobbleSuppressorBase* cv_videostab_NullWobbleSuppressor_to_WobbleSuppressorBase(cv::videostab::NullWobbleSuppressor* instance) {
			return dynamic_cast<cv::videostab::WobbleSuppressorBase*>(instance);
	}

	// cv::videostab::NullWobbleSuppressor::delete() generated
	// ("cv::videostab::NullWobbleSuppressor::delete", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_NullWobbleSuppressor_delete(cv::videostab::NullWobbleSuppressor* instance) {
			delete instance;
	}

	// OnePassStabilizer()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/stabilizer.hpp:142
	// ("cv::videostab::OnePassStabilizer::OnePassStabilizer", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_OnePassStabilizer_OnePassStabilizer(Result<cv::videostab::OnePassStabilizer*>* ocvrs_return) {
		try {
			cv::videostab::OnePassStabilizer* ret = new cv::videostab::OnePassStabilizer();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMotionFilter(Ptr<MotionFilterBase>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/stabilizer.hpp:144
	// ("cv::videostab::OnePassStabilizer::setMotionFilter", vec![(pred!(mut, ["val"], ["cv::Ptr<cv::videostab::MotionFilterBase>"]), _)]),
	void cv_videostab_OnePassStabilizer_setMotionFilter_PtrLMotionFilterBaseG(cv::videostab::OnePassStabilizer* instance, cv::Ptr<cv::videostab::MotionFilterBase>* val, ResultVoid* ocvrs_return) {
		try {
			instance->setMotionFilter(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// motionFilter()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/stabilizer.hpp:145
	// ("cv::videostab::OnePassStabilizer::motionFilter", vec![(pred!(const, [], []), _)]),
	void cv_videostab_OnePassStabilizer_motionFilter_const(const cv::videostab::OnePassStabilizer* instance, Result<cv::Ptr<cv::videostab::MotionFilterBase>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::videostab::MotionFilterBase> ret = instance->motionFilter();
			Ok(new cv::Ptr<cv::videostab::MotionFilterBase>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// reset()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/stabilizer.hpp:147
	// ("cv::videostab::OnePassStabilizer::reset", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_OnePassStabilizer_reset(cv::videostab::OnePassStabilizer* instance, ResultVoid* ocvrs_return) {
		try {
			instance->reset();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// nextFrame()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/stabilizer.hpp:148
	// ("cv::videostab::OnePassStabilizer::nextFrame", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_OnePassStabilizer_nextFrame(cv::videostab::OnePassStabilizer* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->nextFrame();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::OnePassStabilizer::to_IFrameSource() generated
	// ("cv::videostab::OnePassStabilizer::to_IFrameSource", vec![(pred!(mut, [], []), _)]),
	cv::videostab::IFrameSource* cv_videostab_OnePassStabilizer_to_IFrameSource(cv::videostab::OnePassStabilizer* instance) {
			return dynamic_cast<cv::videostab::IFrameSource*>(instance);
	}

	// cv::videostab::OnePassStabilizer::to_StabilizerBase() generated
	// ("cv::videostab::OnePassStabilizer::to_StabilizerBase", vec![(pred!(mut, [], []), _)]),
	cv::videostab::StabilizerBase* cv_videostab_OnePassStabilizer_to_StabilizerBase(cv::videostab::OnePassStabilizer* instance) {
			return dynamic_cast<cv::videostab::StabilizerBase*>(instance);
	}

	// cv::videostab::OnePassStabilizer::delete() generated
	// ("cv::videostab::OnePassStabilizer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_OnePassStabilizer_delete(cv::videostab::OnePassStabilizer* instance) {
			delete instance;
	}

	// PyrLkOptFlowEstimatorBase()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/optical_flow.hpp:82
	// ("cv::videostab::PyrLkOptFlowEstimatorBase::PyrLkOptFlowEstimatorBase", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_PyrLkOptFlowEstimatorBase_PyrLkOptFlowEstimatorBase(Result<cv::videostab::PyrLkOptFlowEstimatorBase*>* ocvrs_return) {
		try {
			cv::videostab::PyrLkOptFlowEstimatorBase* ret = new cv::videostab::PyrLkOptFlowEstimatorBase();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWinSize(Size)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/optical_flow.hpp:84
	// ("cv::videostab::PyrLkOptFlowEstimatorBase::setWinSize", vec![(pred!(mut, ["val"], ["cv::Size"]), _)]),
	void cv_videostab_PyrLkOptFlowEstimatorBase_setWinSize_Size(cv::videostab::PyrLkOptFlowEstimatorBase* instance, cv::Size* val, ResultVoid* ocvrs_return) {
		try {
			instance->setWinSize(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// winSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/optical_flow.hpp:85
	// ("cv::videostab::PyrLkOptFlowEstimatorBase::winSize", vec![(pred!(const, [], []), _)]),
	void cv_videostab_PyrLkOptFlowEstimatorBase_winSize_const(const cv::videostab::PyrLkOptFlowEstimatorBase* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->winSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxLevel(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/optical_flow.hpp:87
	// ("cv::videostab::PyrLkOptFlowEstimatorBase::setMaxLevel", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_videostab_PyrLkOptFlowEstimatorBase_setMaxLevel_int(cv::videostab::PyrLkOptFlowEstimatorBase* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxLevel(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// maxLevel()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/optical_flow.hpp:88
	// ("cv::videostab::PyrLkOptFlowEstimatorBase::maxLevel", vec![(pred!(const, [], []), _)]),
	void cv_videostab_PyrLkOptFlowEstimatorBase_maxLevel_const(const cv::videostab::PyrLkOptFlowEstimatorBase* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->maxLevel();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::PyrLkOptFlowEstimatorBase::to_SparsePyrLkOptFlowEstimator() generated
	// ("cv::videostab::PyrLkOptFlowEstimatorBase::to_SparsePyrLkOptFlowEstimator", vec![(pred!(mut, [], []), _)]),
	cv::videostab::SparsePyrLkOptFlowEstimator* cv_videostab_PyrLkOptFlowEstimatorBase_to_SparsePyrLkOptFlowEstimator(cv::videostab::PyrLkOptFlowEstimatorBase* instance) {
			return dynamic_cast<cv::videostab::SparsePyrLkOptFlowEstimator*>(instance);
	}

	// cv::videostab::PyrLkOptFlowEstimatorBase::delete() generated
	// ("cv::videostab::PyrLkOptFlowEstimatorBase::delete", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_PyrLkOptFlowEstimatorBase_delete(cv::videostab::PyrLkOptFlowEstimatorBase* instance) {
			delete instance;
	}

	// RansacParams()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/motion_core.hpp:80
	// ("cv::videostab::RansacParams::RansacParams", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_RansacParams_RansacParams(Result<cv::videostab::RansacParams*>* ocvrs_return) {
		try {
			cv::videostab::RansacParams* ret = new cv::videostab::RansacParams();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// RansacParams(int, float, float, float)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/motion_core.hpp:87
	// ("cv::videostab::RansacParams::RansacParams", vec![(pred!(mut, ["size", "thresh", "eps", "prob"], ["int", "float", "float", "float"]), _)]),
	void cv_videostab_RansacParams_RansacParams_int_float_float_float(int size, float thresh, float eps, float prob, Result<cv::videostab::RansacParams*>* ocvrs_return) {
		try {
			cv::videostab::RansacParams* ret = new cv::videostab::RansacParams(size, thresh, eps, prob);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// niters()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/motion_core.hpp:92
	// ("cv::videostab::RansacParams::niters", vec![(pred!(const, [], []), _)]),
	void cv_videostab_RansacParams_niters_const(const cv::videostab::RansacParams* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->niters();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// default2dMotion(MotionModel)(Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/motion_core.hpp:102
	// ("cv::videostab::RansacParams::default2dMotion", vec![(pred!(mut, ["model"], ["cv::videostab::MotionModel"]), _)]),
	void cv_videostab_RansacParams_default2dMotion_MotionModel(cv::videostab::MotionModel model, Result<cv::videostab::RansacParams*>* ocvrs_return) {
		try {
			cv::videostab::RansacParams ret = cv::videostab::RansacParams::default2dMotion(model);
			Ok(new cv::videostab::RansacParams(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::RansacParams::size() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/motion_core.hpp:75
	// ("cv::videostab::RansacParams::size", vec![(pred!(const, [], []), _)]),
	int cv_videostab_RansacParams_propSize_const(const cv::videostab::RansacParams* instance) {
			int ret = instance->size;
			return ret;
	}

	// cv::videostab::RansacParams::setSize(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/motion_core.hpp:75
	// ("cv::videostab::RansacParams::setSize", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_videostab_RansacParams_propSize_const_int(cv::videostab::RansacParams* instance, const int val) {
			instance->size = val;
	}

	// cv::videostab::RansacParams::thresh() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/motion_core.hpp:76
	// ("cv::videostab::RansacParams::thresh", vec![(pred!(const, [], []), _)]),
	float cv_videostab_RansacParams_propThresh_const(const cv::videostab::RansacParams* instance) {
			float ret = instance->thresh;
			return ret;
	}

	// cv::videostab::RansacParams::setThresh(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/motion_core.hpp:76
	// ("cv::videostab::RansacParams::setThresh", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_videostab_RansacParams_propThresh_const_float(cv::videostab::RansacParams* instance, const float val) {
			instance->thresh = val;
	}

	// cv::videostab::RansacParams::eps() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/motion_core.hpp:77
	// ("cv::videostab::RansacParams::eps", vec![(pred!(const, [], []), _)]),
	float cv_videostab_RansacParams_propEps_const(const cv::videostab::RansacParams* instance) {
			float ret = instance->eps;
			return ret;
	}

	// cv::videostab::RansacParams::setEps(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/motion_core.hpp:77
	// ("cv::videostab::RansacParams::setEps", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_videostab_RansacParams_propEps_const_float(cv::videostab::RansacParams* instance, const float val) {
			instance->eps = val;
	}

	// cv::videostab::RansacParams::prob() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/motion_core.hpp:78
	// ("cv::videostab::RansacParams::prob", vec![(pred!(const, [], []), _)]),
	float cv_videostab_RansacParams_propProb_const(const cv::videostab::RansacParams* instance) {
			float ret = instance->prob;
			return ret;
	}

	// cv::videostab::RansacParams::setProb(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/motion_core.hpp:78
	// ("cv::videostab::RansacParams::setProb", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_videostab_RansacParams_propProb_const_float(cv::videostab::RansacParams* instance, const float val) {
			instance->prob = val;
	}

	// cv::videostab::RansacParams::delete() generated
	// ("cv::videostab::RansacParams::delete", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_RansacParams_delete(cv::videostab::RansacParams* instance) {
			delete instance;
	}

	// run(InputArray, InputArray, InputArray, InputOutputArray, OutputArray, OutputArray)(InputArray, InputArray, InputArray, InputOutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/optical_flow.hpp:100
	// ("cv::videostab::SparsePyrLkOptFlowEstimator::run", vec![(pred!(mut, ["frame0", "frame1", "points0", "points1", "status", "errors"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_videostab_SparsePyrLkOptFlowEstimator_run_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR(cv::videostab::SparsePyrLkOptFlowEstimator* instance, const cv::_InputArray* frame0, const cv::_InputArray* frame1, const cv::_InputArray* points0, const cv::_InputOutputArray* points1, const cv::_OutputArray* status, const cv::_OutputArray* errors, ResultVoid* ocvrs_return) {
		try {
			instance->run(*frame0, *frame1, *points0, *points1, *status, *errors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::SparsePyrLkOptFlowEstimator::defaultNew() generated
	// ("cv::videostab::SparsePyrLkOptFlowEstimator::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::videostab::SparsePyrLkOptFlowEstimator* cv_videostab_SparsePyrLkOptFlowEstimator_defaultNew_const() {
			cv::videostab::SparsePyrLkOptFlowEstimator* ret = new cv::videostab::SparsePyrLkOptFlowEstimator();
			return ret;
	}

	// cv::videostab::SparsePyrLkOptFlowEstimator::to_ISparseOptFlowEstimator() generated
	// ("cv::videostab::SparsePyrLkOptFlowEstimator::to_ISparseOptFlowEstimator", vec![(pred!(mut, [], []), _)]),
	cv::videostab::ISparseOptFlowEstimator* cv_videostab_SparsePyrLkOptFlowEstimator_to_ISparseOptFlowEstimator(cv::videostab::SparsePyrLkOptFlowEstimator* instance) {
			return dynamic_cast<cv::videostab::ISparseOptFlowEstimator*>(instance);
	}

	// cv::videostab::SparsePyrLkOptFlowEstimator::to_PyrLkOptFlowEstimatorBase() generated
	// ("cv::videostab::SparsePyrLkOptFlowEstimator::to_PyrLkOptFlowEstimatorBase", vec![(pred!(mut, [], []), _)]),
	cv::videostab::PyrLkOptFlowEstimatorBase* cv_videostab_SparsePyrLkOptFlowEstimator_to_PyrLkOptFlowEstimatorBase(cv::videostab::SparsePyrLkOptFlowEstimator* instance) {
			return dynamic_cast<cv::videostab::PyrLkOptFlowEstimatorBase*>(instance);
	}

	// cv::videostab::SparsePyrLkOptFlowEstimator::delete() generated
	// ("cv::videostab::SparsePyrLkOptFlowEstimator::delete", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_SparsePyrLkOptFlowEstimator_delete(cv::videostab::SparsePyrLkOptFlowEstimator* instance) {
			delete instance;
	}

	// setLog(Ptr<ILog>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/stabilizer.hpp:71
	// ("cv::videostab::StabilizerBase::setLog", vec![(pred!(mut, ["ilog"], ["cv::Ptr<cv::videostab::ILog>"]), _)]),
	void cv_videostab_StabilizerBase_setLog_PtrLILogG(cv::videostab::StabilizerBase* instance, cv::Ptr<cv::videostab::ILog>* ilog, ResultVoid* ocvrs_return) {
		try {
			instance->setLog(*ilog);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// log()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/stabilizer.hpp:72
	// ("cv::videostab::StabilizerBase::log", vec![(pred!(const, [], []), _)]),
	void cv_videostab_StabilizerBase_log_const(const cv::videostab::StabilizerBase* instance, Result<cv::Ptr<cv::videostab::ILog>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::videostab::ILog> ret = instance->log();
			Ok(new cv::Ptr<cv::videostab::ILog>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setRadius(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/stabilizer.hpp:74
	// ("cv::videostab::StabilizerBase::setRadius", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_videostab_StabilizerBase_setRadius_int(cv::videostab::StabilizerBase* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setRadius(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// radius()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/stabilizer.hpp:75
	// ("cv::videostab::StabilizerBase::radius", vec![(pred!(const, [], []), _)]),
	void cv_videostab_StabilizerBase_radius_const(const cv::videostab::StabilizerBase* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->radius();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setFrameSource(Ptr<IFrameSource>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/stabilizer.hpp:77
	// ("cv::videostab::StabilizerBase::setFrameSource", vec![(pred!(mut, ["val"], ["cv::Ptr<cv::videostab::IFrameSource>"]), _)]),
	void cv_videostab_StabilizerBase_setFrameSource_PtrLIFrameSourceG(cv::videostab::StabilizerBase* instance, cv::Ptr<cv::videostab::IFrameSource>* val, ResultVoid* ocvrs_return) {
		try {
			instance->setFrameSource(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// frameSource()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/stabilizer.hpp:78
	// ("cv::videostab::StabilizerBase::frameSource", vec![(pred!(const, [], []), _)]),
	void cv_videostab_StabilizerBase_frameSource_const(const cv::videostab::StabilizerBase* instance, Result<cv::Ptr<cv::videostab::IFrameSource>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::videostab::IFrameSource> ret = instance->frameSource();
			Ok(new cv::Ptr<cv::videostab::IFrameSource>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMotionEstimator(Ptr<ImageMotionEstimatorBase>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/stabilizer.hpp:80
	// ("cv::videostab::StabilizerBase::setMotionEstimator", vec![(pred!(mut, ["val"], ["cv::Ptr<cv::videostab::ImageMotionEstimatorBase>"]), _)]),
	void cv_videostab_StabilizerBase_setMotionEstimator_PtrLImageMotionEstimatorBaseG(cv::videostab::StabilizerBase* instance, cv::Ptr<cv::videostab::ImageMotionEstimatorBase>* val, ResultVoid* ocvrs_return) {
		try {
			instance->setMotionEstimator(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// motionEstimator()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/stabilizer.hpp:81
	// ("cv::videostab::StabilizerBase::motionEstimator", vec![(pred!(const, [], []), _)]),
	void cv_videostab_StabilizerBase_motionEstimator_const(const cv::videostab::StabilizerBase* instance, Result<cv::Ptr<cv::videostab::ImageMotionEstimatorBase>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::videostab::ImageMotionEstimatorBase> ret = instance->motionEstimator();
			Ok(new cv::Ptr<cv::videostab::ImageMotionEstimatorBase>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDeblurer(Ptr<DeblurerBase>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/stabilizer.hpp:83
	// ("cv::videostab::StabilizerBase::setDeblurer", vec![(pred!(mut, ["val"], ["cv::Ptr<cv::videostab::DeblurerBase>"]), _)]),
	void cv_videostab_StabilizerBase_setDeblurer_PtrLDeblurerBaseG(cv::videostab::StabilizerBase* instance, cv::Ptr<cv::videostab::DeblurerBase>* val, ResultVoid* ocvrs_return) {
		try {
			instance->setDeblurer(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// deblurrer()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/stabilizer.hpp:84
	// ("cv::videostab::StabilizerBase::deblurrer", vec![(pred!(const, [], []), _)]),
	void cv_videostab_StabilizerBase_deblurrer_const(const cv::videostab::StabilizerBase* instance, Result<cv::Ptr<cv::videostab::DeblurerBase>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::videostab::DeblurerBase> ret = instance->deblurrer();
			Ok(new cv::Ptr<cv::videostab::DeblurerBase>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTrimRatio(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/stabilizer.hpp:86
	// ("cv::videostab::StabilizerBase::setTrimRatio", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_videostab_StabilizerBase_setTrimRatio_float(cv::videostab::StabilizerBase* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setTrimRatio(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// trimRatio()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/stabilizer.hpp:87
	// ("cv::videostab::StabilizerBase::trimRatio", vec![(pred!(const, [], []), _)]),
	void cv_videostab_StabilizerBase_trimRatio_const(const cv::videostab::StabilizerBase* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->trimRatio();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setCorrectionForInclusion(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/stabilizer.hpp:89
	// ("cv::videostab::StabilizerBase::setCorrectionForInclusion", vec![(pred!(mut, ["val"], ["bool"]), _)]),
	void cv_videostab_StabilizerBase_setCorrectionForInclusion_bool(cv::videostab::StabilizerBase* instance, bool val, ResultVoid* ocvrs_return) {
		try {
			instance->setCorrectionForInclusion(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// doCorrectionForInclusion()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/stabilizer.hpp:90
	// ("cv::videostab::StabilizerBase::doCorrectionForInclusion", vec![(pred!(const, [], []), _)]),
	void cv_videostab_StabilizerBase_doCorrectionForInclusion_const(const cv::videostab::StabilizerBase* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->doCorrectionForInclusion();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setBorderMode(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/stabilizer.hpp:92
	// ("cv::videostab::StabilizerBase::setBorderMode", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_videostab_StabilizerBase_setBorderMode_int(cv::videostab::StabilizerBase* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setBorderMode(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// borderMode()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/stabilizer.hpp:93
	// ("cv::videostab::StabilizerBase::borderMode", vec![(pred!(const, [], []), _)]),
	void cv_videostab_StabilizerBase_borderMode_const(const cv::videostab::StabilizerBase* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->borderMode();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setInpainter(Ptr<InpainterBase>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/stabilizer.hpp:95
	// ("cv::videostab::StabilizerBase::setInpainter", vec![(pred!(mut, ["val"], ["cv::Ptr<cv::videostab::InpainterBase>"]), _)]),
	void cv_videostab_StabilizerBase_setInpainter_PtrLInpainterBaseG(cv::videostab::StabilizerBase* instance, cv::Ptr<cv::videostab::InpainterBase>* val, ResultVoid* ocvrs_return) {
		try {
			instance->setInpainter(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// inpainter()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/stabilizer.hpp:96
	// ("cv::videostab::StabilizerBase::inpainter", vec![(pred!(const, [], []), _)]),
	void cv_videostab_StabilizerBase_inpainter_const(const cv::videostab::StabilizerBase* instance, Result<cv::Ptr<cv::videostab::InpainterBase>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::videostab::InpainterBase> ret = instance->inpainter();
			Ok(new cv::Ptr<cv::videostab::InpainterBase>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::StabilizerBase::to_OnePassStabilizer() generated
	// ("cv::videostab::StabilizerBase::to_OnePassStabilizer", vec![(pred!(mut, [], []), _)]),
	cv::videostab::OnePassStabilizer* cv_videostab_StabilizerBase_to_OnePassStabilizer(cv::videostab::StabilizerBase* instance) {
			return dynamic_cast<cv::videostab::OnePassStabilizer*>(instance);
	}

	// cv::videostab::StabilizerBase::to_TwoPassStabilizer() generated
	// ("cv::videostab::StabilizerBase::to_TwoPassStabilizer", vec![(pred!(mut, [], []), _)]),
	cv::videostab::TwoPassStabilizer* cv_videostab_StabilizerBase_to_TwoPassStabilizer(cv::videostab::StabilizerBase* instance) {
			return dynamic_cast<cv::videostab::TwoPassStabilizer*>(instance);
	}

	// cv::videostab::StabilizerBase::delete() generated
	// ("cv::videostab::StabilizerBase::delete", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_StabilizerBase_delete(cv::videostab::StabilizerBase* instance) {
			delete instance;
	}

	// ToFileMotionWriter(const String &, Ptr<ImageMotionEstimatorBase>)(InString, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/global_motion.hpp:206
	// ("cv::videostab::ToFileMotionWriter::ToFileMotionWriter", vec![(pred!(mut, ["path", "estimator"], ["const cv::String*", "cv::Ptr<cv::videostab::ImageMotionEstimatorBase>"]), _)]),
	void cv_videostab_ToFileMotionWriter_ToFileMotionWriter_const_StringR_PtrLImageMotionEstimatorBaseG(const char* path, cv::Ptr<cv::videostab::ImageMotionEstimatorBase>* estimator, Result<cv::videostab::ToFileMotionWriter*>* ocvrs_return) {
		try {
			cv::videostab::ToFileMotionWriter* ret = new cv::videostab::ToFileMotionWriter(cv::String(path), *estimator);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMotionModel(MotionModel)(Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/global_motion.hpp:208
	// ("cv::videostab::ToFileMotionWriter::setMotionModel", vec![(pred!(mut, ["val"], ["cv::videostab::MotionModel"]), _)]),
	void cv_videostab_ToFileMotionWriter_setMotionModel_MotionModel(cv::videostab::ToFileMotionWriter* instance, cv::videostab::MotionModel val, ResultVoid* ocvrs_return) {
		try {
			instance->setMotionModel(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// motionModel()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/global_motion.hpp:209
	// ("cv::videostab::ToFileMotionWriter::motionModel", vec![(pred!(const, [], []), _)]),
	void cv_videostab_ToFileMotionWriter_motionModel_const(const cv::videostab::ToFileMotionWriter* instance, Result<cv::videostab::MotionModel>* ocvrs_return) {
		try {
			cv::videostab::MotionModel ret = instance->motionModel();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// estimate(const Mat &, const Mat &, bool *)(TraitClass, TraitClass, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/global_motion.hpp:211
	// ("cv::videostab::ToFileMotionWriter::estimate", vec![(pred!(mut, ["frame0", "frame1", "ok"], ["const cv::Mat*", "const cv::Mat*", "bool*"]), _)]),
	void cv_videostab_ToFileMotionWriter_estimate_const_MatR_const_MatR_boolX(cv::videostab::ToFileMotionWriter* instance, const cv::Mat* frame0, const cv::Mat* frame1, bool* ok, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->estimate(*frame0, *frame1, ok);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::ToFileMotionWriter::estimate(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/global_motion.hpp:211
	// ("cv::videostab::ToFileMotionWriter::estimate", vec![(pred!(mut, ["frame0", "frame1"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
	void cv_videostab_ToFileMotionWriter_estimate_const_MatR_const_MatR(cv::videostab::ToFileMotionWriter* instance, const cv::Mat* frame0, const cv::Mat* frame1, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->estimate(*frame0, *frame1);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::ToFileMotionWriter::to_ImageMotionEstimatorBase() generated
	// ("cv::videostab::ToFileMotionWriter::to_ImageMotionEstimatorBase", vec![(pred!(mut, [], []), _)]),
	cv::videostab::ImageMotionEstimatorBase* cv_videostab_ToFileMotionWriter_to_ImageMotionEstimatorBase(cv::videostab::ToFileMotionWriter* instance) {
			return dynamic_cast<cv::videostab::ImageMotionEstimatorBase*>(instance);
	}

	// cv::videostab::ToFileMotionWriter::delete() generated
	// ("cv::videostab::ToFileMotionWriter::delete", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_ToFileMotionWriter_delete(cv::videostab::ToFileMotionWriter* instance) {
			delete instance;
	}

	// TranslationBasedLocalOutlierRejector()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/outlier_rejection.hpp:77
	// ("cv::videostab::TranslationBasedLocalOutlierRejector::TranslationBasedLocalOutlierRejector", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_TranslationBasedLocalOutlierRejector_TranslationBasedLocalOutlierRejector(Result<cv::videostab::TranslationBasedLocalOutlierRejector*>* ocvrs_return) {
		try {
			cv::videostab::TranslationBasedLocalOutlierRejector* ret = new cv::videostab::TranslationBasedLocalOutlierRejector();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setCellSize(Size)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/outlier_rejection.hpp:79
	// ("cv::videostab::TranslationBasedLocalOutlierRejector::setCellSize", vec![(pred!(mut, ["val"], ["cv::Size"]), _)]),
	void cv_videostab_TranslationBasedLocalOutlierRejector_setCellSize_Size(cv::videostab::TranslationBasedLocalOutlierRejector* instance, cv::Size* val, ResultVoid* ocvrs_return) {
		try {
			instance->setCellSize(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cellSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/outlier_rejection.hpp:80
	// ("cv::videostab::TranslationBasedLocalOutlierRejector::cellSize", vec![(pred!(const, [], []), _)]),
	void cv_videostab_TranslationBasedLocalOutlierRejector_cellSize_const(const cv::videostab::TranslationBasedLocalOutlierRejector* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->cellSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setRansacParams(RansacParams)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/outlier_rejection.hpp:82
	// ("cv::videostab::TranslationBasedLocalOutlierRejector::setRansacParams", vec![(pred!(mut, ["val"], ["cv::videostab::RansacParams"]), _)]),
	void cv_videostab_TranslationBasedLocalOutlierRejector_setRansacParams_RansacParams(cv::videostab::TranslationBasedLocalOutlierRejector* instance, cv::videostab::RansacParams* val, ResultVoid* ocvrs_return) {
		try {
			instance->setRansacParams(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ransacParams()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/outlier_rejection.hpp:83
	// ("cv::videostab::TranslationBasedLocalOutlierRejector::ransacParams", vec![(pred!(const, [], []), _)]),
	void cv_videostab_TranslationBasedLocalOutlierRejector_ransacParams_const(const cv::videostab::TranslationBasedLocalOutlierRejector* instance, Result<cv::videostab::RansacParams*>* ocvrs_return) {
		try {
			cv::videostab::RansacParams ret = instance->ransacParams();
			Ok(new cv::videostab::RansacParams(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// process(Size, InputArray, InputArray, OutputArray)(SimpleClass, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/outlier_rejection.hpp:85
	// ("cv::videostab::TranslationBasedLocalOutlierRejector::process", vec![(pred!(mut, ["frameSize", "points0", "points1", "mask"], ["cv::Size", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_videostab_TranslationBasedLocalOutlierRejector_process_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR(cv::videostab::TranslationBasedLocalOutlierRejector* instance, cv::Size* frameSize, const cv::_InputArray* points0, const cv::_InputArray* points1, const cv::_OutputArray* mask, ResultVoid* ocvrs_return) {
		try {
			instance->process(*frameSize, *points0, *points1, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::TranslationBasedLocalOutlierRejector::to_IOutlierRejector() generated
	// ("cv::videostab::TranslationBasedLocalOutlierRejector::to_IOutlierRejector", vec![(pred!(mut, [], []), _)]),
	cv::videostab::IOutlierRejector* cv_videostab_TranslationBasedLocalOutlierRejector_to_IOutlierRejector(cv::videostab::TranslationBasedLocalOutlierRejector* instance) {
			return dynamic_cast<cv::videostab::IOutlierRejector*>(instance);
	}

	// cv::videostab::TranslationBasedLocalOutlierRejector::delete() generated
	// ("cv::videostab::TranslationBasedLocalOutlierRejector::delete", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_TranslationBasedLocalOutlierRejector_delete(cv::videostab::TranslationBasedLocalOutlierRejector* instance) {
			delete instance;
	}

	// TwoPassStabilizer()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/stabilizer.hpp:162
	// ("cv::videostab::TwoPassStabilizer::TwoPassStabilizer", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_TwoPassStabilizer_TwoPassStabilizer(Result<cv::videostab::TwoPassStabilizer*>* ocvrs_return) {
		try {
			cv::videostab::TwoPassStabilizer* ret = new cv::videostab::TwoPassStabilizer();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMotionStabilizer(Ptr<IMotionStabilizer>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/stabilizer.hpp:164
	// ("cv::videostab::TwoPassStabilizer::setMotionStabilizer", vec![(pred!(mut, ["val"], ["cv::Ptr<cv::videostab::IMotionStabilizer>"]), _)]),
	void cv_videostab_TwoPassStabilizer_setMotionStabilizer_PtrLIMotionStabilizerG(cv::videostab::TwoPassStabilizer* instance, cv::Ptr<cv::videostab::IMotionStabilizer>* val, ResultVoid* ocvrs_return) {
		try {
			instance->setMotionStabilizer(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// motionStabilizer()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/stabilizer.hpp:165
	// ("cv::videostab::TwoPassStabilizer::motionStabilizer", vec![(pred!(const, [], []), _)]),
	void cv_videostab_TwoPassStabilizer_motionStabilizer_const(const cv::videostab::TwoPassStabilizer* instance, Result<cv::Ptr<cv::videostab::IMotionStabilizer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::videostab::IMotionStabilizer> ret = instance->motionStabilizer();
			Ok(new cv::Ptr<cv::videostab::IMotionStabilizer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWobbleSuppressor(Ptr<WobbleSuppressorBase>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/stabilizer.hpp:167
	// ("cv::videostab::TwoPassStabilizer::setWobbleSuppressor", vec![(pred!(mut, ["val"], ["cv::Ptr<cv::videostab::WobbleSuppressorBase>"]), _)]),
	void cv_videostab_TwoPassStabilizer_setWobbleSuppressor_PtrLWobbleSuppressorBaseG(cv::videostab::TwoPassStabilizer* instance, cv::Ptr<cv::videostab::WobbleSuppressorBase>* val, ResultVoid* ocvrs_return) {
		try {
			instance->setWobbleSuppressor(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// wobbleSuppressor()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/stabilizer.hpp:168
	// ("cv::videostab::TwoPassStabilizer::wobbleSuppressor", vec![(pred!(const, [], []), _)]),
	void cv_videostab_TwoPassStabilizer_wobbleSuppressor_const(const cv::videostab::TwoPassStabilizer* instance, Result<cv::Ptr<cv::videostab::WobbleSuppressorBase>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::videostab::WobbleSuppressorBase> ret = instance->wobbleSuppressor();
			Ok(new cv::Ptr<cv::videostab::WobbleSuppressorBase>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setEstimateTrimRatio(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/stabilizer.hpp:170
	// ("cv::videostab::TwoPassStabilizer::setEstimateTrimRatio", vec![(pred!(mut, ["val"], ["bool"]), _)]),
	void cv_videostab_TwoPassStabilizer_setEstimateTrimRatio_bool(cv::videostab::TwoPassStabilizer* instance, bool val, ResultVoid* ocvrs_return) {
		try {
			instance->setEstimateTrimRatio(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// mustEstimateTrimaRatio()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/stabilizer.hpp:171
	// ("cv::videostab::TwoPassStabilizer::mustEstimateTrimaRatio", vec![(pred!(const, [], []), _)]),
	void cv_videostab_TwoPassStabilizer_mustEstimateTrimaRatio_const(const cv::videostab::TwoPassStabilizer* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->mustEstimateTrimaRatio();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// reset()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/stabilizer.hpp:173
	// ("cv::videostab::TwoPassStabilizer::reset", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_TwoPassStabilizer_reset(cv::videostab::TwoPassStabilizer* instance, ResultVoid* ocvrs_return) {
		try {
			instance->reset();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// nextFrame()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/stabilizer.hpp:174
	// ("cv::videostab::TwoPassStabilizer::nextFrame", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_TwoPassStabilizer_nextFrame(cv::videostab::TwoPassStabilizer* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->nextFrame();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::TwoPassStabilizer::to_IFrameSource() generated
	// ("cv::videostab::TwoPassStabilizer::to_IFrameSource", vec![(pred!(mut, [], []), _)]),
	cv::videostab::IFrameSource* cv_videostab_TwoPassStabilizer_to_IFrameSource(cv::videostab::TwoPassStabilizer* instance) {
			return dynamic_cast<cv::videostab::IFrameSource*>(instance);
	}

	// cv::videostab::TwoPassStabilizer::to_StabilizerBase() generated
	// ("cv::videostab::TwoPassStabilizer::to_StabilizerBase", vec![(pred!(mut, [], []), _)]),
	cv::videostab::StabilizerBase* cv_videostab_TwoPassStabilizer_to_StabilizerBase(cv::videostab::TwoPassStabilizer* instance) {
			return dynamic_cast<cv::videostab::StabilizerBase*>(instance);
	}

	// cv::videostab::TwoPassStabilizer::delete() generated
	// ("cv::videostab::TwoPassStabilizer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_TwoPassStabilizer_delete(cv::videostab::TwoPassStabilizer* instance) {
			delete instance;
	}

	// VideoFileSource(const String &, bool)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/frame_source.hpp:75
	// ("cv::videostab::VideoFileSource::VideoFileSource", vec![(pred!(mut, ["path", "volatileFrame"], ["const cv::String*", "bool"]), _)]),
	void cv_videostab_VideoFileSource_VideoFileSource_const_StringR_bool(const char* path, bool volatileFrame, Result<cv::videostab::VideoFileSource*>* ocvrs_return) {
		try {
			cv::videostab::VideoFileSource* ret = new cv::videostab::VideoFileSource(cv::String(path), volatileFrame);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::VideoFileSource::VideoFileSource(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/frame_source.hpp:75
	// ("cv::videostab::VideoFileSource::VideoFileSource", vec![(pred!(mut, ["path"], ["const cv::String*"]), _)]),
	void cv_videostab_VideoFileSource_VideoFileSource_const_StringR(const char* path, Result<cv::videostab::VideoFileSource*>* ocvrs_return) {
		try {
			cv::videostab::VideoFileSource* ret = new cv::videostab::VideoFileSource(cv::String(path));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// reset()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/frame_source.hpp:77
	// ("cv::videostab::VideoFileSource::reset", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_VideoFileSource_reset(cv::videostab::VideoFileSource* instance, ResultVoid* ocvrs_return) {
		try {
			instance->reset();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// nextFrame()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/frame_source.hpp:78
	// ("cv::videostab::VideoFileSource::nextFrame", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_VideoFileSource_nextFrame(cv::videostab::VideoFileSource* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->nextFrame();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// width()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/frame_source.hpp:80
	// ("cv::videostab::VideoFileSource::width", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_VideoFileSource_width(cv::videostab::VideoFileSource* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->width();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// height()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/frame_source.hpp:81
	// ("cv::videostab::VideoFileSource::height", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_VideoFileSource_height(cv::videostab::VideoFileSource* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->height();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// count()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/frame_source.hpp:82
	// ("cv::videostab::VideoFileSource::count", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_VideoFileSource_count(cv::videostab::VideoFileSource* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->count();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// fps()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/frame_source.hpp:83
	// ("cv::videostab::VideoFileSource::fps", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_VideoFileSource_fps(cv::videostab::VideoFileSource* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->fps();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::VideoFileSource::to_IFrameSource() generated
	// ("cv::videostab::VideoFileSource::to_IFrameSource", vec![(pred!(mut, [], []), _)]),
	cv::videostab::IFrameSource* cv_videostab_VideoFileSource_to_IFrameSource(cv::videostab::VideoFileSource* instance) {
			return dynamic_cast<cv::videostab::IFrameSource*>(instance);
	}

	// cv::videostab::VideoFileSource::delete() generated
	// ("cv::videostab::VideoFileSource::delete", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_VideoFileSource_delete(cv::videostab::VideoFileSource* instance) {
			delete instance;
	}

	// WeightingDeblurer()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/deblurring.hpp:99
	// ("cv::videostab::WeightingDeblurer::WeightingDeblurer", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_WeightingDeblurer_WeightingDeblurer(Result<cv::videostab::WeightingDeblurer*>* ocvrs_return) {
		try {
			cv::videostab::WeightingDeblurer* ret = new cv::videostab::WeightingDeblurer();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSensitivity(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/deblurring.hpp:101
	// ("cv::videostab::WeightingDeblurer::setSensitivity", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_videostab_WeightingDeblurer_setSensitivity_float(cv::videostab::WeightingDeblurer* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setSensitivity(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// sensitivity()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/deblurring.hpp:102
	// ("cv::videostab::WeightingDeblurer::sensitivity", vec![(pred!(const, [], []), _)]),
	void cv_videostab_WeightingDeblurer_sensitivity_const(const cv::videostab::WeightingDeblurer* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->sensitivity();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// deblur(int, Mat &)(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/deblurring.hpp:104
	// ("cv::videostab::WeightingDeblurer::deblur", vec![(pred!(mut, ["idx", "frame"], ["int", "cv::Mat*"]), _)]),
	void cv_videostab_WeightingDeblurer_deblur_int_MatR(cv::videostab::WeightingDeblurer* instance, int idx, cv::Mat* frame, ResultVoid* ocvrs_return) {
		try {
			instance->deblur(idx, *frame);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::WeightingDeblurer::to_DeblurerBase() generated
	// ("cv::videostab::WeightingDeblurer::to_DeblurerBase", vec![(pred!(mut, [], []), _)]),
	cv::videostab::DeblurerBase* cv_videostab_WeightingDeblurer_to_DeblurerBase(cv::videostab::WeightingDeblurer* instance) {
			return dynamic_cast<cv::videostab::DeblurerBase*>(instance);
	}

	// cv::videostab::WeightingDeblurer::delete() generated
	// ("cv::videostab::WeightingDeblurer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_WeightingDeblurer_delete(cv::videostab::WeightingDeblurer* instance) {
			delete instance;
	}

	// setMotionEstimator(Ptr<ImageMotionEstimatorBase>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/wobble_suppression.hpp:67
	// ("cv::videostab::WobbleSuppressorBase::setMotionEstimator", vec![(pred!(mut, ["val"], ["cv::Ptr<cv::videostab::ImageMotionEstimatorBase>"]), _)]),
	void cv_videostab_WobbleSuppressorBase_setMotionEstimator_PtrLImageMotionEstimatorBaseG(cv::videostab::WobbleSuppressorBase* instance, cv::Ptr<cv::videostab::ImageMotionEstimatorBase>* val, ResultVoid* ocvrs_return) {
		try {
			instance->setMotionEstimator(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// motionEstimator()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/wobble_suppression.hpp:68
	// ("cv::videostab::WobbleSuppressorBase::motionEstimator", vec![(pred!(const, [], []), _)]),
	void cv_videostab_WobbleSuppressorBase_motionEstimator_const(const cv::videostab::WobbleSuppressorBase* instance, Result<cv::Ptr<cv::videostab::ImageMotionEstimatorBase>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::videostab::ImageMotionEstimatorBase> ret = instance->motionEstimator();
			Ok(new cv::Ptr<cv::videostab::ImageMotionEstimatorBase>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// suppress(int, const Mat &, Mat &)(Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/wobble_suppression.hpp:70
	// ("cv::videostab::WobbleSuppressorBase::suppress", vec![(pred!(mut, ["idx", "frame", "result"], ["int", "const cv::Mat*", "cv::Mat*"]), _)]),
	void cv_videostab_WobbleSuppressorBase_suppress_int_const_MatR_MatR(cv::videostab::WobbleSuppressorBase* instance, int idx, const cv::Mat* frame, cv::Mat* result, ResultVoid* ocvrs_return) {
		try {
			instance->suppress(idx, *frame, *result);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setFrameCount(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/wobble_suppression.hpp:75
	// ("cv::videostab::WobbleSuppressorBase::setFrameCount", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_videostab_WobbleSuppressorBase_setFrameCount_int(cv::videostab::WobbleSuppressorBase* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setFrameCount(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// frameCount()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/wobble_suppression.hpp:76
	// ("cv::videostab::WobbleSuppressorBase::frameCount", vec![(pred!(const, [], []), _)]),
	void cv_videostab_WobbleSuppressorBase_frameCount_const(const cv::videostab::WobbleSuppressorBase* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->frameCount();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMotions(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/wobble_suppression.hpp:78
	// ("cv::videostab::WobbleSuppressorBase::setMotions", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>*"]), _)]),
	void cv_videostab_WobbleSuppressorBase_setMotions_const_vectorLMatGR(cv::videostab::WobbleSuppressorBase* instance, const std::vector<cv::Mat>* val, ResultVoid* ocvrs_return) {
		try {
			instance->setMotions(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// motions()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/wobble_suppression.hpp:79
	// ("cv::videostab::WobbleSuppressorBase::motions", vec![(pred!(const, [], []), _)]),
	void cv_videostab_WobbleSuppressorBase_motions_const(const cv::videostab::WobbleSuppressorBase* instance, Result<std::vector<cv::Mat>*>* ocvrs_return) {
		try {
			const std::vector<cv::Mat> ret = instance->motions();
			Ok(new const std::vector<cv::Mat>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMotions2(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/wobble_suppression.hpp:81
	// ("cv::videostab::WobbleSuppressorBase::setMotions2", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>*"]), _)]),
	void cv_videostab_WobbleSuppressorBase_setMotions2_const_vectorLMatGR(cv::videostab::WobbleSuppressorBase* instance, const std::vector<cv::Mat>* val, ResultVoid* ocvrs_return) {
		try {
			instance->setMotions2(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// motions2()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/wobble_suppression.hpp:82
	// ("cv::videostab::WobbleSuppressorBase::motions2", vec![(pred!(const, [], []), _)]),
	void cv_videostab_WobbleSuppressorBase_motions2_const(const cv::videostab::WobbleSuppressorBase* instance, Result<std::vector<cv::Mat>*>* ocvrs_return) {
		try {
			const std::vector<cv::Mat> ret = instance->motions2();
			Ok(new const std::vector<cv::Mat>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setStabilizationMotions(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/wobble_suppression.hpp:84
	// ("cv::videostab::WobbleSuppressorBase::setStabilizationMotions", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>*"]), _)]),
	void cv_videostab_WobbleSuppressorBase_setStabilizationMotions_const_vectorLMatGR(cv::videostab::WobbleSuppressorBase* instance, const std::vector<cv::Mat>* val, ResultVoid* ocvrs_return) {
		try {
			instance->setStabilizationMotions(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// stabilizationMotions()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videostab/wobble_suppression.hpp:85
	// ("cv::videostab::WobbleSuppressorBase::stabilizationMotions", vec![(pred!(const, [], []), _)]),
	void cv_videostab_WobbleSuppressorBase_stabilizationMotions_const(const cv::videostab::WobbleSuppressorBase* instance, Result<std::vector<cv::Mat>*>* ocvrs_return) {
		try {
			const std::vector<cv::Mat> ret = instance->stabilizationMotions();
			Ok(new const std::vector<cv::Mat>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::videostab::WobbleSuppressorBase::to_MoreAccurateMotionWobbleSuppressor() generated
	// ("cv::videostab::WobbleSuppressorBase::to_MoreAccurateMotionWobbleSuppressor", vec![(pred!(mut, [], []), _)]),
	cv::videostab::MoreAccurateMotionWobbleSuppressor* cv_videostab_WobbleSuppressorBase_to_MoreAccurateMotionWobbleSuppressor(cv::videostab::WobbleSuppressorBase* instance) {
			return dynamic_cast<cv::videostab::MoreAccurateMotionWobbleSuppressor*>(instance);
	}

	// cv::videostab::WobbleSuppressorBase::to_MoreAccurateMotionWobbleSuppressorBase() generated
	// ("cv::videostab::WobbleSuppressorBase::to_MoreAccurateMotionWobbleSuppressorBase", vec![(pred!(mut, [], []), _)]),
	cv::videostab::MoreAccurateMotionWobbleSuppressorBase* cv_videostab_WobbleSuppressorBase_to_MoreAccurateMotionWobbleSuppressorBase(cv::videostab::WobbleSuppressorBase* instance) {
			return dynamic_cast<cv::videostab::MoreAccurateMotionWobbleSuppressorBase*>(instance);
	}

	// cv::videostab::WobbleSuppressorBase::to_NullWobbleSuppressor() generated
	// ("cv::videostab::WobbleSuppressorBase::to_NullWobbleSuppressor", vec![(pred!(mut, [], []), _)]),
	cv::videostab::NullWobbleSuppressor* cv_videostab_WobbleSuppressorBase_to_NullWobbleSuppressor(cv::videostab::WobbleSuppressorBase* instance) {
			return dynamic_cast<cv::videostab::NullWobbleSuppressor*>(instance);
	}

	// cv::videostab::WobbleSuppressorBase::delete() generated
	// ("cv::videostab::WobbleSuppressorBase::delete", vec![(pred!(mut, [], []), _)]),
	void cv_videostab_WobbleSuppressorBase_delete(cv::videostab::WobbleSuppressorBase* instance) {
			delete instance;
	}

}
