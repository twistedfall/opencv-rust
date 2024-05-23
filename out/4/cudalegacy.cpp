#include "ocvrs_common.hpp"
#include <opencv2/cudalegacy.hpp>
#include "cudalegacy_types.hpp"

extern "C" {
	// cv::cuda::calcOpticalFlowBM(TraitClass, TraitClass, SimpleClass, SimpleClass, SimpleClass, Primitive, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:196
	// ("cv::cuda::calcOpticalFlowBM", vec![(pred!(mut, ["prev", "curr", "block_size", "shift_size", "max_range", "use_previous", "velx", "vely", "buf"], ["const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "cv::Size", "cv::Size", "cv::Size", "bool", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*"]), _)]),
	void cv_cuda_calcOpticalFlowBM_const_GpuMatR_const_GpuMatR_Size_Size_Size_bool_GpuMatR_GpuMatR_GpuMatR(const cv::cuda::GpuMat* prev, const cv::cuda::GpuMat* curr, cv::Size* block_size, cv::Size* shift_size, cv::Size* max_range, bool use_previous, cv::cuda::GpuMat* velx, cv::cuda::GpuMat* vely, cv::cuda::GpuMat* buf, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::calcOpticalFlowBM(*prev, *curr, *block_size, *shift_size, *max_range, use_previous, *velx, *vely, *buf);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// calcOpticalFlowBM(const GpuMat &, const GpuMat &, Size, Size, Size, bool, GpuMat &, GpuMat &, GpuMat &, Stream &)(TraitClass, TraitClass, SimpleClass, SimpleClass, SimpleClass, Primitive, TraitClass, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:196
	// ("cv::cuda::calcOpticalFlowBM", vec![(pred!(mut, ["prev", "curr", "block_size", "shift_size", "max_range", "use_previous", "velx", "vely", "buf", "stream"], ["const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "cv::Size", "cv::Size", "cv::Size", "bool", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_calcOpticalFlowBM_const_GpuMatR_const_GpuMatR_Size_Size_Size_bool_GpuMatR_GpuMatR_GpuMatR_StreamR(const cv::cuda::GpuMat* prev, const cv::cuda::GpuMat* curr, cv::Size* block_size, cv::Size* shift_size, cv::Size* max_range, bool use_previous, cv::cuda::GpuMat* velx, cv::cuda::GpuMat* vely, cv::cuda::GpuMat* buf, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::calcOpticalFlowBM(*prev, *curr, *block_size, *shift_size, *max_range, use_previous, *velx, *vely, *buf, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::connectivityMask(TraitClass, TraitClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:251
	// ("cv::cuda::connectivityMask", vec![(pred!(mut, ["image", "mask", "lo", "hi"], ["const cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "const cv::Scalar*", "const cv::Scalar*"]), _)]),
	void cv_cuda_connectivityMask_const_GpuMatR_GpuMatR_const_ScalarR_const_ScalarR(const cv::cuda::GpuMat* image, cv::cuda::GpuMat* mask, const cv::Scalar* lo, const cv::Scalar* hi, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::connectivityMask(*image, *mask, *lo, *hi);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// connectivityMask(const GpuMat &, GpuMat &, const cv::Scalar &, const cv::Scalar &, Stream &)(TraitClass, TraitClass, SimpleClass, SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:251
	// ("cv::cuda::connectivityMask", vec![(pred!(mut, ["image", "mask", "lo", "hi", "stream"], ["const cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "const cv::Scalar*", "const cv::Scalar*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_connectivityMask_const_GpuMatR_GpuMatR_const_ScalarR_const_ScalarR_StreamR(const cv::cuda::GpuMat* image, cv::cuda::GpuMat* mask, const cv::Scalar* lo, const cv::Scalar* hi, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::connectivityMask(*image, *mask, *lo, *hi, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::createBackgroundSubtractorFGD() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:189
	// ("cv::cuda::createBackgroundSubtractorFGD", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_createBackgroundSubtractorFGD(Result<cv::Ptr<cv::cuda::BackgroundSubtractorFGD>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::BackgroundSubtractorFGD> ret = cv::cuda::createBackgroundSubtractorFGD();
			Ok(new cv::Ptr<cv::cuda::BackgroundSubtractorFGD>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createBackgroundSubtractorFGD(const FGDParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:189
	// ("cv::cuda::createBackgroundSubtractorFGD", vec![(pred!(mut, ["params"], ["const cv::cuda::FGDParams*"]), _)]),
	void cv_cuda_createBackgroundSubtractorFGD_const_FGDParamsR(const cv::cuda::FGDParams* params, Result<cv::Ptr<cv::cuda::BackgroundSubtractorFGD>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::BackgroundSubtractorFGD> ret = cv::cuda::createBackgroundSubtractorFGD(*params);
			Ok(new cv::Ptr<cv::cuda::BackgroundSubtractorFGD>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::createBackgroundSubtractorGMG() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:133
	// ("cv::cuda::createBackgroundSubtractorGMG", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_createBackgroundSubtractorGMG(Result<cv::Ptr<cv::cuda::BackgroundSubtractorGMG>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::BackgroundSubtractorGMG> ret = cv::cuda::createBackgroundSubtractorGMG();
			Ok(new cv::Ptr<cv::cuda::BackgroundSubtractorGMG>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createBackgroundSubtractorGMG(int, double)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:133
	// ("cv::cuda::createBackgroundSubtractorGMG", vec![(pred!(mut, ["initializationFrames", "decisionThreshold"], ["int", "double"]), _)]),
	void cv_cuda_createBackgroundSubtractorGMG_int_double(int initializationFrames, double decisionThreshold, Result<cv::Ptr<cv::cuda::BackgroundSubtractorGMG>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::BackgroundSubtractorGMG> ret = cv::cuda::createBackgroundSubtractorGMG(initializationFrames, decisionThreshold);
			Ok(new cv::Ptr<cv::cuda::BackgroundSubtractorGMG>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::createImagePyramid(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:78
	// ("cv::cuda::createImagePyramid", vec![(pred!(mut, ["img"], ["const cv::_InputArray*"]), _)]),
	void cv_cuda_createImagePyramid_const__InputArrayR(const cv::_InputArray* img, Result<cv::Ptr<cv::cuda::ImagePyramid>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::ImagePyramid> ret = cv::cuda::createImagePyramid(*img);
			Ok(new cv::Ptr<cv::cuda::ImagePyramid>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createImagePyramid(InputArray, int, Stream &)(InputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:78
	// ("cv::cuda::createImagePyramid", vec![(pred!(mut, ["img", "nLayers", "stream"], ["const cv::_InputArray*", "int", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_createImagePyramid_const__InputArrayR_int_StreamR(const cv::_InputArray* img, int nLayers, cv::cuda::Stream* stream, Result<cv::Ptr<cv::cuda::ImagePyramid>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::ImagePyramid> ret = cv::cuda::createImagePyramid(*img, nLayers, *stream);
			Ok(new cv::Ptr<cv::cuda::ImagePyramid>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createOpticalFlowNeedleMap(const GpuMat &, const GpuMat &, GpuMat &, GpuMat &)(TraitClass, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:234
	// ("cv::cuda::createOpticalFlowNeedleMap", vec![(pred!(mut, ["u", "v", "vertex", "colors"], ["const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*"]), _)]),
	void cv_cuda_createOpticalFlowNeedleMap_const_GpuMatR_const_GpuMatR_GpuMatR_GpuMatR(const cv::cuda::GpuMat* u, const cv::cuda::GpuMat* v, cv::cuda::GpuMat* vertex, cv::cuda::GpuMat* colors, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::createOpticalFlowNeedleMap(*u, *v, *vertex, *colors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::graphcut(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:241
	// ("cv::cuda::graphcut", vec![(pred!(mut, ["terminals", "leftTransp", "rightTransp", "top", "bottom", "labels", "buf"], ["cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*"]), _)]),
	void cv_cuda_graphcut_GpuMatR_GpuMatR_GpuMatR_GpuMatR_GpuMatR_GpuMatR_GpuMatR(cv::cuda::GpuMat* terminals, cv::cuda::GpuMat* leftTransp, cv::cuda::GpuMat* rightTransp, cv::cuda::GpuMat* top, cv::cuda::GpuMat* bottom, cv::cuda::GpuMat* labels, cv::cuda::GpuMat* buf, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::graphcut(*terminals, *leftTransp, *rightTransp, *top, *bottom, *labels, *buf);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::graphcut(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:245
	// ("cv::cuda::graphcut", vec![(pred!(mut, ["terminals", "leftTransp", "rightTransp", "top", "topLeft", "topRight", "bottom", "bottomLeft", "bottomRight", "labels", "buf"], ["cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*"]), _)]),
	void cv_cuda_graphcut_GpuMatR_GpuMatR_GpuMatR_GpuMatR_GpuMatR_GpuMatR_GpuMatR_GpuMatR_GpuMatR_GpuMatR_GpuMatR(cv::cuda::GpuMat* terminals, cv::cuda::GpuMat* leftTransp, cv::cuda::GpuMat* rightTransp, cv::cuda::GpuMat* top, cv::cuda::GpuMat* topLeft, cv::cuda::GpuMat* topRight, cv::cuda::GpuMat* bottom, cv::cuda::GpuMat* bottomLeft, cv::cuda::GpuMat* bottomRight, cv::cuda::GpuMat* labels, cv::cuda::GpuMat* buf, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::graphcut(*terminals, *leftTransp, *rightTransp, *top, *topLeft, *topRight, *bottom, *bottomLeft, *bottomRight, *labels, *buf);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// graphcut(GpuMat &, GpuMat &, GpuMat &, GpuMat &, GpuMat &, GpuMat &, GpuMat &, GpuMat &, GpuMat &, GpuMat &, GpuMat &, Stream &)(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:245
	// ("cv::cuda::graphcut", vec![(pred!(mut, ["terminals", "leftTransp", "rightTransp", "top", "topLeft", "topRight", "bottom", "bottomLeft", "bottomRight", "labels", "buf", "stream"], ["cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_graphcut_GpuMatR_GpuMatR_GpuMatR_GpuMatR_GpuMatR_GpuMatR_GpuMatR_GpuMatR_GpuMatR_GpuMatR_GpuMatR_StreamR(cv::cuda::GpuMat* terminals, cv::cuda::GpuMat* leftTransp, cv::cuda::GpuMat* rightTransp, cv::cuda::GpuMat* top, cv::cuda::GpuMat* topLeft, cv::cuda::GpuMat* topRight, cv::cuda::GpuMat* bottom, cv::cuda::GpuMat* bottomLeft, cv::cuda::GpuMat* bottomRight, cv::cuda::GpuMat* labels, cv::cuda::GpuMat* buf, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::graphcut(*terminals, *leftTransp, *rightTransp, *top, *topLeft, *topRight, *bottom, *bottomLeft, *bottomRight, *labels, *buf, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// graphcut(GpuMat &, GpuMat &, GpuMat &, GpuMat &, GpuMat &, GpuMat &, GpuMat &, Stream &)(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:241
	// ("cv::cuda::graphcut", vec![(pred!(mut, ["terminals", "leftTransp", "rightTransp", "top", "bottom", "labels", "buf", "stream"], ["cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_graphcut_GpuMatR_GpuMatR_GpuMatR_GpuMatR_GpuMatR_GpuMatR_GpuMatR_StreamR(cv::cuda::GpuMat* terminals, cv::cuda::GpuMat* leftTransp, cv::cuda::GpuMat* rightTransp, cv::cuda::GpuMat* top, cv::cuda::GpuMat* bottom, cv::cuda::GpuMat* labels, cv::cuda::GpuMat* buf, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::graphcut(*terminals, *leftTransp, *rightTransp, *top, *bottom, *labels, *buf, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::interpolateFrames(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, Primitive, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:228
	// ("cv::cuda::interpolateFrames", vec![(pred!(mut, ["frame0", "frame1", "fu", "fv", "bu", "bv", "pos", "newFrame", "buf"], ["const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "float", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*"]), _)]),
	void cv_cuda_interpolateFrames_const_GpuMatR_const_GpuMatR_const_GpuMatR_const_GpuMatR_const_GpuMatR_const_GpuMatR_float_GpuMatR_GpuMatR(const cv::cuda::GpuMat* frame0, const cv::cuda::GpuMat* frame1, const cv::cuda::GpuMat* fu, const cv::cuda::GpuMat* fv, const cv::cuda::GpuMat* bu, const cv::cuda::GpuMat* bv, float pos, cv::cuda::GpuMat* newFrame, cv::cuda::GpuMat* buf, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::interpolateFrames(*frame0, *frame1, *fu, *fv, *bu, *bv, pos, *newFrame, *buf);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// interpolateFrames(const GpuMat &, const GpuMat &, const GpuMat &, const GpuMat &, const GpuMat &, const GpuMat &, float, GpuMat &, GpuMat &, Stream &)(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, Primitive, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:228
	// ("cv::cuda::interpolateFrames", vec![(pred!(mut, ["frame0", "frame1", "fu", "fv", "bu", "bv", "pos", "newFrame", "buf", "stream"], ["const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "float", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_interpolateFrames_const_GpuMatR_const_GpuMatR_const_GpuMatR_const_GpuMatR_const_GpuMatR_const_GpuMatR_float_GpuMatR_GpuMatR_StreamR(const cv::cuda::GpuMat* frame0, const cv::cuda::GpuMat* frame1, const cv::cuda::GpuMat* fu, const cv::cuda::GpuMat* fv, const cv::cuda::GpuMat* bu, const cv::cuda::GpuMat* bv, float pos, cv::cuda::GpuMat* newFrame, cv::cuda::GpuMat* buf, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::interpolateFrames(*frame0, *frame1, *fu, *fv, *bu, *bv, pos, *newFrame, *buf, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::labelComponents(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:254
	// ("cv::cuda::labelComponents", vec![(pred!(mut, ["mask", "components"], ["const cv::cuda::GpuMat*", "cv::cuda::GpuMat*"]), _)]),
	void cv_cuda_labelComponents_const_GpuMatR_GpuMatR(const cv::cuda::GpuMat* mask, cv::cuda::GpuMat* components, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::labelComponents(*mask, *components);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// labelComponents(const GpuMat &, GpuMat &, int, Stream &)(TraitClass, TraitClass, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:254
	// ("cv::cuda::labelComponents", vec![(pred!(mut, ["mask", "components", "flags", "stream"], ["const cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "int", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_labelComponents_const_GpuMatR_GpuMatR_int_StreamR(const cv::cuda::GpuMat* mask, cv::cuda::GpuMat* components, int flags, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::labelComponents(*mask, *components, flags, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::projectPoints(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:263
	// ("cv::cuda::projectPoints", vec![(pred!(mut, ["src", "rvec", "tvec", "camera_mat", "dist_coef", "dst"], ["const cv::cuda::GpuMat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "cv::cuda::GpuMat*"]), _)]),
	void cv_cuda_projectPoints_const_GpuMatR_const_MatR_const_MatR_const_MatR_const_MatR_GpuMatR(const cv::cuda::GpuMat* src, const cv::Mat* rvec, const cv::Mat* tvec, const cv::Mat* camera_mat, const cv::Mat* dist_coef, cv::cuda::GpuMat* dst, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::projectPoints(*src, *rvec, *tvec, *camera_mat, *dist_coef, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// projectPoints(const GpuMat &, const Mat &, const Mat &, const Mat &, const Mat &, GpuMat &, Stream &)(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:263
	// ("cv::cuda::projectPoints", vec![(pred!(mut, ["src", "rvec", "tvec", "camera_mat", "dist_coef", "dst", "stream"], ["const cv::cuda::GpuMat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "cv::cuda::GpuMat*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_projectPoints_const_GpuMatR_const_MatR_const_MatR_const_MatR_const_MatR_GpuMatR_StreamR(const cv::cuda::GpuMat* src, const cv::Mat* rvec, const cv::Mat* tvec, const cv::Mat* camera_mat, const cv::Mat* dist_coef, cv::cuda::GpuMat* dst, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::projectPoints(*src, *rvec, *tvec, *camera_mat, *dist_coef, *dst, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::solvePnPRansac(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:283
	// ("cv::cuda::solvePnPRansac", vec![(pred!(mut, ["object", "image", "camera_mat", "dist_coef", "rvec", "tvec"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "cv::Mat*", "cv::Mat*"]), _)]),
	void cv_cuda_solvePnPRansac_const_MatR_const_MatR_const_MatR_const_MatR_MatR_MatR(const cv::Mat* object, const cv::Mat* image, const cv::Mat* camera_mat, const cv::Mat* dist_coef, cv::Mat* rvec, cv::Mat* tvec, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::solvePnPRansac(*object, *image, *camera_mat, *dist_coef, *rvec, *tvec);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// solvePnPRansac(const Mat &, const Mat &, const Mat &, const Mat &, Mat &, Mat &, bool, int, float, int, std::vector<int> *)(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, Primitive, Primitive, Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:283
	// ("cv::cuda::solvePnPRansac", vec![(pred!(mut, ["object", "image", "camera_mat", "dist_coef", "rvec", "tvec", "use_extrinsic_guess", "num_iters", "max_dist", "min_inlier_count", "inliers"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "cv::Mat*", "cv::Mat*", "bool", "int", "float", "int", "std::vector<int>*"]), _)]),
	void cv_cuda_solvePnPRansac_const_MatR_const_MatR_const_MatR_const_MatR_MatR_MatR_bool_int_float_int_vectorLintGX(const cv::Mat* object, const cv::Mat* image, const cv::Mat* camera_mat, const cv::Mat* dist_coef, cv::Mat* rvec, cv::Mat* tvec, bool use_extrinsic_guess, int num_iters, float max_dist, int min_inlier_count, std::vector<int>* inliers, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::solvePnPRansac(*object, *image, *camera_mat, *dist_coef, *rvec, *tvec, use_extrinsic_guess, num_iters, max_dist, min_inlier_count, inliers);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::transformPoints(TraitClass, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:260
	// ("cv::cuda::transformPoints", vec![(pred!(mut, ["src", "rvec", "tvec", "dst"], ["const cv::cuda::GpuMat*", "const cv::Mat*", "const cv::Mat*", "cv::cuda::GpuMat*"]), _)]),
	void cv_cuda_transformPoints_const_GpuMatR_const_MatR_const_MatR_GpuMatR(const cv::cuda::GpuMat* src, const cv::Mat* rvec, const cv::Mat* tvec, cv::cuda::GpuMat* dst, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::transformPoints(*src, *rvec, *tvec, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// transformPoints(const GpuMat &, const Mat &, const Mat &, GpuMat &, Stream &)(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:260
	// ("cv::cuda::transformPoints", vec![(pred!(mut, ["src", "rvec", "tvec", "dst", "stream"], ["const cv::cuda::GpuMat*", "const cv::Mat*", "const cv::Mat*", "cv::cuda::GpuMat*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_transformPoints_const_GpuMatR_const_MatR_const_MatR_GpuMatR_StreamR(const cv::cuda::GpuMat* src, const cv::Mat* rvec, const cv::Mat* tvec, cv::cuda::GpuMat* dst, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			cv::cuda::transformPoints(*src, *rvec, *tvec, *dst, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getForegroundRegions(OutputArrayOfArrays)(OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:153
	// ("cv::cuda::BackgroundSubtractorFGD::getForegroundRegions", vec![(pred!(mut, ["foreground_regions"], ["const cv::_OutputArray*"]), _)]),
	void cv_cuda_BackgroundSubtractorFGD_getForegroundRegions_const__OutputArrayR(cv::cuda::BackgroundSubtractorFGD* instance, const cv::_OutputArray* foreground_regions, ResultVoid* ocvrs_return) {
		try {
			instance->getForegroundRegions(*foreground_regions);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::BackgroundSubtractorFGD::to_Algorithm() generated
	// ("cv::cuda::BackgroundSubtractorFGD::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_cuda_BackgroundSubtractorFGD_to_Algorithm(cv::cuda::BackgroundSubtractorFGD* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::cuda::BackgroundSubtractorFGD::to_BackgroundSubtractor() generated
	// ("cv::cuda::BackgroundSubtractorFGD::to_BackgroundSubtractor", vec![(pred!(mut, [], []), _)]),
	cv::BackgroundSubtractor* cv_cuda_BackgroundSubtractorFGD_to_BackgroundSubtractor(cv::cuda::BackgroundSubtractorFGD* instance) {
			return dynamic_cast<cv::BackgroundSubtractor*>(instance);
	}

	// cv::cuda::BackgroundSubtractorFGD::delete() generated
	// ("cv::cuda::BackgroundSubtractorFGD::delete", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_BackgroundSubtractorFGD_delete(cv::cuda::BackgroundSubtractorFGD* instance) {
			delete instance;
	}

	// apply(InputArray, OutputArray, double, Stream &)(InputArray, OutputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:94
	// ("cv::cuda::BackgroundSubtractorGMG::apply", vec![(pred!(mut, ["image", "fgmask", "learningRate", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_BackgroundSubtractorGMG_apply_const__InputArrayR_const__OutputArrayR_double_StreamR(cv::cuda::BackgroundSubtractorGMG* instance, const cv::_InputArray* image, const cv::_OutputArray* fgmask, double learningRate, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			instance->apply(*image, *fgmask, learningRate, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxFeatures()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:96
	// ("cv::cuda::BackgroundSubtractorGMG::getMaxFeatures", vec![(pred!(const, [], []), _)]),
	void cv_cuda_BackgroundSubtractorGMG_getMaxFeatures_const(const cv::cuda::BackgroundSubtractorGMG* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMaxFeatures();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxFeatures(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:97
	// ("cv::cuda::BackgroundSubtractorGMG::setMaxFeatures", vec![(pred!(mut, ["maxFeatures"], ["int"]), _)]),
	void cv_cuda_BackgroundSubtractorGMG_setMaxFeatures_int(cv::cuda::BackgroundSubtractorGMG* instance, int maxFeatures, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxFeatures(maxFeatures);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDefaultLearningRate()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:99
	// ("cv::cuda::BackgroundSubtractorGMG::getDefaultLearningRate", vec![(pred!(const, [], []), _)]),
	void cv_cuda_BackgroundSubtractorGMG_getDefaultLearningRate_const(const cv::cuda::BackgroundSubtractorGMG* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getDefaultLearningRate();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDefaultLearningRate(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:100
	// ("cv::cuda::BackgroundSubtractorGMG::setDefaultLearningRate", vec![(pred!(mut, ["lr"], ["double"]), _)]),
	void cv_cuda_BackgroundSubtractorGMG_setDefaultLearningRate_double(cv::cuda::BackgroundSubtractorGMG* instance, double lr, ResultVoid* ocvrs_return) {
		try {
			instance->setDefaultLearningRate(lr);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNumFrames()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:102
	// ("cv::cuda::BackgroundSubtractorGMG::getNumFrames", vec![(pred!(const, [], []), _)]),
	void cv_cuda_BackgroundSubtractorGMG_getNumFrames_const(const cv::cuda::BackgroundSubtractorGMG* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNumFrames();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNumFrames(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:103
	// ("cv::cuda::BackgroundSubtractorGMG::setNumFrames", vec![(pred!(mut, ["nframes"], ["int"]), _)]),
	void cv_cuda_BackgroundSubtractorGMG_setNumFrames_int(cv::cuda::BackgroundSubtractorGMG* instance, int nframes, ResultVoid* ocvrs_return) {
		try {
			instance->setNumFrames(nframes);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getQuantizationLevels()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:105
	// ("cv::cuda::BackgroundSubtractorGMG::getQuantizationLevels", vec![(pred!(const, [], []), _)]),
	void cv_cuda_BackgroundSubtractorGMG_getQuantizationLevels_const(const cv::cuda::BackgroundSubtractorGMG* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getQuantizationLevels();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setQuantizationLevels(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:106
	// ("cv::cuda::BackgroundSubtractorGMG::setQuantizationLevels", vec![(pred!(mut, ["nlevels"], ["int"]), _)]),
	void cv_cuda_BackgroundSubtractorGMG_setQuantizationLevels_int(cv::cuda::BackgroundSubtractorGMG* instance, int nlevels, ResultVoid* ocvrs_return) {
		try {
			instance->setQuantizationLevels(nlevels);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getBackgroundPrior()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:108
	// ("cv::cuda::BackgroundSubtractorGMG::getBackgroundPrior", vec![(pred!(const, [], []), _)]),
	void cv_cuda_BackgroundSubtractorGMG_getBackgroundPrior_const(const cv::cuda::BackgroundSubtractorGMG* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getBackgroundPrior();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setBackgroundPrior(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:109
	// ("cv::cuda::BackgroundSubtractorGMG::setBackgroundPrior", vec![(pred!(mut, ["bgprior"], ["double"]), _)]),
	void cv_cuda_BackgroundSubtractorGMG_setBackgroundPrior_double(cv::cuda::BackgroundSubtractorGMG* instance, double bgprior, ResultVoid* ocvrs_return) {
		try {
			instance->setBackgroundPrior(bgprior);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSmoothingRadius()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:111
	// ("cv::cuda::BackgroundSubtractorGMG::getSmoothingRadius", vec![(pred!(const, [], []), _)]),
	void cv_cuda_BackgroundSubtractorGMG_getSmoothingRadius_const(const cv::cuda::BackgroundSubtractorGMG* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getSmoothingRadius();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSmoothingRadius(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:112
	// ("cv::cuda::BackgroundSubtractorGMG::setSmoothingRadius", vec![(pred!(mut, ["radius"], ["int"]), _)]),
	void cv_cuda_BackgroundSubtractorGMG_setSmoothingRadius_int(cv::cuda::BackgroundSubtractorGMG* instance, int radius, ResultVoid* ocvrs_return) {
		try {
			instance->setSmoothingRadius(radius);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDecisionThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:114
	// ("cv::cuda::BackgroundSubtractorGMG::getDecisionThreshold", vec![(pred!(const, [], []), _)]),
	void cv_cuda_BackgroundSubtractorGMG_getDecisionThreshold_const(const cv::cuda::BackgroundSubtractorGMG* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getDecisionThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDecisionThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:115
	// ("cv::cuda::BackgroundSubtractorGMG::setDecisionThreshold", vec![(pred!(mut, ["thresh"], ["double"]), _)]),
	void cv_cuda_BackgroundSubtractorGMG_setDecisionThreshold_double(cv::cuda::BackgroundSubtractorGMG* instance, double thresh, ResultVoid* ocvrs_return) {
		try {
			instance->setDecisionThreshold(thresh);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getUpdateBackgroundModel()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:117
	// ("cv::cuda::BackgroundSubtractorGMG::getUpdateBackgroundModel", vec![(pred!(const, [], []), _)]),
	void cv_cuda_BackgroundSubtractorGMG_getUpdateBackgroundModel_const(const cv::cuda::BackgroundSubtractorGMG* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUpdateBackgroundModel();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setUpdateBackgroundModel(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:118
	// ("cv::cuda::BackgroundSubtractorGMG::setUpdateBackgroundModel", vec![(pred!(mut, ["update"], ["bool"]), _)]),
	void cv_cuda_BackgroundSubtractorGMG_setUpdateBackgroundModel_bool(cv::cuda::BackgroundSubtractorGMG* instance, bool update, ResultVoid* ocvrs_return) {
		try {
			instance->setUpdateBackgroundModel(update);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMinVal()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:120
	// ("cv::cuda::BackgroundSubtractorGMG::getMinVal", vec![(pred!(const, [], []), _)]),
	void cv_cuda_BackgroundSubtractorGMG_getMinVal_const(const cv::cuda::BackgroundSubtractorGMG* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMinVal();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMinVal(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:121
	// ("cv::cuda::BackgroundSubtractorGMG::setMinVal", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_cuda_BackgroundSubtractorGMG_setMinVal_double(cv::cuda::BackgroundSubtractorGMG* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setMinVal(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxVal()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:123
	// ("cv::cuda::BackgroundSubtractorGMG::getMaxVal", vec![(pred!(const, [], []), _)]),
	void cv_cuda_BackgroundSubtractorGMG_getMaxVal_const(const cv::cuda::BackgroundSubtractorGMG* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxVal();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxVal(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:124
	// ("cv::cuda::BackgroundSubtractorGMG::setMaxVal", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_cuda_BackgroundSubtractorGMG_setMaxVal_double(cv::cuda::BackgroundSubtractorGMG* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxVal(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::BackgroundSubtractorGMG::to_Algorithm() generated
	// ("cv::cuda::BackgroundSubtractorGMG::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_cuda_BackgroundSubtractorGMG_to_Algorithm(cv::cuda::BackgroundSubtractorGMG* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::cuda::BackgroundSubtractorGMG::to_BackgroundSubtractor() generated
	// ("cv::cuda::BackgroundSubtractorGMG::to_BackgroundSubtractor", vec![(pred!(mut, [], []), _)]),
	cv::BackgroundSubtractor* cv_cuda_BackgroundSubtractorGMG_to_BackgroundSubtractor(cv::cuda::BackgroundSubtractorGMG* instance) {
			return dynamic_cast<cv::BackgroundSubtractor*>(instance);
	}

	// cv::cuda::BackgroundSubtractorGMG::delete() generated
	// ("cv::cuda::BackgroundSubtractorGMG::delete", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_BackgroundSubtractorGMG_delete(cv::cuda::BackgroundSubtractorGMG* instance) {
			delete instance;
	}

	// FGDParams()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:181
	// ("cv::cuda::FGDParams::FGDParams", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_FGDParams_FGDParams(Result<cv::cuda::FGDParams*>* ocvrs_return) {
		try {
			cv::cuda::FGDParams* ret = new cv::cuda::FGDParams();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::FGDParams::Lc() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:158
	// ("cv::cuda::FGDParams::Lc", vec![(pred!(const, [], []), _)]),
	int cv_cuda_FGDParams_propLc_const(const cv::cuda::FGDParams* instance) {
			int ret = instance->Lc;
			return ret;
	}

	// cv::cuda::FGDParams::setLc(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:158
	// ("cv::cuda::FGDParams::setLc", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_cuda_FGDParams_propLc_const_int(cv::cuda::FGDParams* instance, const int val) {
			instance->Lc = val;
	}

	// cv::cuda::FGDParams::N1c() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:159
	// ("cv::cuda::FGDParams::N1c", vec![(pred!(const, [], []), _)]),
	int cv_cuda_FGDParams_propN1c_const(const cv::cuda::FGDParams* instance) {
			int ret = instance->N1c;
			return ret;
	}

	// cv::cuda::FGDParams::setN1c(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:159
	// ("cv::cuda::FGDParams::setN1c", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_cuda_FGDParams_propN1c_const_int(cv::cuda::FGDParams* instance, const int val) {
			instance->N1c = val;
	}

	// cv::cuda::FGDParams::N2c() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:160
	// ("cv::cuda::FGDParams::N2c", vec![(pred!(const, [], []), _)]),
	int cv_cuda_FGDParams_propN2c_const(const cv::cuda::FGDParams* instance) {
			int ret = instance->N2c;
			return ret;
	}

	// cv::cuda::FGDParams::setN2c(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:160
	// ("cv::cuda::FGDParams::setN2c", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_cuda_FGDParams_propN2c_const_int(cv::cuda::FGDParams* instance, const int val) {
			instance->N2c = val;
	}

	// cv::cuda::FGDParams::Lcc() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:163
	// ("cv::cuda::FGDParams::Lcc", vec![(pred!(const, [], []), _)]),
	int cv_cuda_FGDParams_propLcc_const(const cv::cuda::FGDParams* instance) {
			int ret = instance->Lcc;
			return ret;
	}

	// cv::cuda::FGDParams::setLcc(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:163
	// ("cv::cuda::FGDParams::setLcc", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_cuda_FGDParams_propLcc_const_int(cv::cuda::FGDParams* instance, const int val) {
			instance->Lcc = val;
	}

	// cv::cuda::FGDParams::N1cc() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:164
	// ("cv::cuda::FGDParams::N1cc", vec![(pred!(const, [], []), _)]),
	int cv_cuda_FGDParams_propN1cc_const(const cv::cuda::FGDParams* instance) {
			int ret = instance->N1cc;
			return ret;
	}

	// cv::cuda::FGDParams::setN1cc(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:164
	// ("cv::cuda::FGDParams::setN1cc", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_cuda_FGDParams_propN1cc_const_int(cv::cuda::FGDParams* instance, const int val) {
			instance->N1cc = val;
	}

	// cv::cuda::FGDParams::N2cc() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:165
	// ("cv::cuda::FGDParams::N2cc", vec![(pred!(const, [], []), _)]),
	int cv_cuda_FGDParams_propN2cc_const(const cv::cuda::FGDParams* instance) {
			int ret = instance->N2cc;
			return ret;
	}

	// cv::cuda::FGDParams::setN2cc(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:165
	// ("cv::cuda::FGDParams::setN2cc", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_cuda_FGDParams_propN2cc_const_int(cv::cuda::FGDParams* instance, const int val) {
			instance->N2cc = val;
	}

	// cv::cuda::FGDParams::is_obj_without_holes() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:168
	// ("cv::cuda::FGDParams::is_obj_without_holes", vec![(pred!(const, [], []), _)]),
	bool cv_cuda_FGDParams_propIs_obj_without_holes_const(const cv::cuda::FGDParams* instance) {
			bool ret = instance->is_obj_without_holes;
			return ret;
	}

	// cv::cuda::FGDParams::setIs_obj_without_holes(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:168
	// ("cv::cuda::FGDParams::setIs_obj_without_holes", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_cuda_FGDParams_propIs_obj_without_holes_const_bool(cv::cuda::FGDParams* instance, const bool val) {
			instance->is_obj_without_holes = val;
	}

	// cv::cuda::FGDParams::perform_morphing() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:169
	// ("cv::cuda::FGDParams::perform_morphing", vec![(pred!(const, [], []), _)]),
	int cv_cuda_FGDParams_propPerform_morphing_const(const cv::cuda::FGDParams* instance) {
			int ret = instance->perform_morphing;
			return ret;
	}

	// cv::cuda::FGDParams::setPerform_morphing(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:169
	// ("cv::cuda::FGDParams::setPerform_morphing", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_cuda_FGDParams_propPerform_morphing_const_int(cv::cuda::FGDParams* instance, const int val) {
			instance->perform_morphing = val;
	}

	// cv::cuda::FGDParams::alpha1() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:172
	// ("cv::cuda::FGDParams::alpha1", vec![(pred!(const, [], []), _)]),
	float cv_cuda_FGDParams_propAlpha1_const(const cv::cuda::FGDParams* instance) {
			float ret = instance->alpha1;
			return ret;
	}

	// cv::cuda::FGDParams::setAlpha1(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:172
	// ("cv::cuda::FGDParams::setAlpha1", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_cuda_FGDParams_propAlpha1_const_float(cv::cuda::FGDParams* instance, const float val) {
			instance->alpha1 = val;
	}

	// cv::cuda::FGDParams::alpha2() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:173
	// ("cv::cuda::FGDParams::alpha2", vec![(pred!(const, [], []), _)]),
	float cv_cuda_FGDParams_propAlpha2_const(const cv::cuda::FGDParams* instance) {
			float ret = instance->alpha2;
			return ret;
	}

	// cv::cuda::FGDParams::setAlpha2(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:173
	// ("cv::cuda::FGDParams::setAlpha2", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_cuda_FGDParams_propAlpha2_const_float(cv::cuda::FGDParams* instance, const float val) {
			instance->alpha2 = val;
	}

	// cv::cuda::FGDParams::alpha3() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:174
	// ("cv::cuda::FGDParams::alpha3", vec![(pred!(const, [], []), _)]),
	float cv_cuda_FGDParams_propAlpha3_const(const cv::cuda::FGDParams* instance) {
			float ret = instance->alpha3;
			return ret;
	}

	// cv::cuda::FGDParams::setAlpha3(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:174
	// ("cv::cuda::FGDParams::setAlpha3", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_cuda_FGDParams_propAlpha3_const_float(cv::cuda::FGDParams* instance, const float val) {
			instance->alpha3 = val;
	}

	// cv::cuda::FGDParams::delta() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:176
	// ("cv::cuda::FGDParams::delta", vec![(pred!(const, [], []), _)]),
	float cv_cuda_FGDParams_propDelta_const(const cv::cuda::FGDParams* instance) {
			float ret = instance->delta;
			return ret;
	}

	// cv::cuda::FGDParams::setDelta(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:176
	// ("cv::cuda::FGDParams::setDelta", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_cuda_FGDParams_propDelta_const_float(cv::cuda::FGDParams* instance, const float val) {
			instance->delta = val;
	}

	// cv::cuda::FGDParams::T() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:177
	// ("cv::cuda::FGDParams::T", vec![(pred!(const, [], []), _)]),
	float cv_cuda_FGDParams_propT_const(const cv::cuda::FGDParams* instance) {
			float ret = instance->T;
			return ret;
	}

	// cv::cuda::FGDParams::setT(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:177
	// ("cv::cuda::FGDParams::setT", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_cuda_FGDParams_propT_const_float(cv::cuda::FGDParams* instance, const float val) {
			instance->T = val;
	}

	// cv::cuda::FGDParams::minArea() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:178
	// ("cv::cuda::FGDParams::minArea", vec![(pred!(const, [], []), _)]),
	float cv_cuda_FGDParams_propMinArea_const(const cv::cuda::FGDParams* instance) {
			float ret = instance->minArea;
			return ret;
	}

	// cv::cuda::FGDParams::setMinArea(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:178
	// ("cv::cuda::FGDParams::setMinArea", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_cuda_FGDParams_propMinArea_const_float(cv::cuda::FGDParams* instance, const float val) {
			instance->minArea = val;
	}

	// cv::cuda::FGDParams::delete() generated
	// ("cv::cuda::FGDParams::delete", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_FGDParams_delete(cv::cuda::FGDParams* instance) {
			delete instance;
	}

	// operator()(const GpuMat &, const GpuMat &, GpuMat &, GpuMat &, int, int, Stream &)(TraitClass, TraitClass, TraitClass, TraitClass, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:204
	// ("cv::cuda::FastOpticalFlowBM::operator()", vec![(pred!(mut, ["I0", "I1", "flowx", "flowy", "search_window", "block_window", "s"], ["const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "int", "int", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_FastOpticalFlowBM_operator___const_GpuMatR_const_GpuMatR_GpuMatR_GpuMatR_int_int_StreamR(cv::cuda::FastOpticalFlowBM* instance, const cv::cuda::GpuMat* I0, const cv::cuda::GpuMat* I1, cv::cuda::GpuMat* flowx, cv::cuda::GpuMat* flowy, int search_window, int block_window, cv::cuda::Stream* s, ResultVoid* ocvrs_return) {
		try {
			instance->operator()(*I0, *I1, *flowx, *flowy, search_window, block_window, *s);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::FastOpticalFlowBM::operator()(TraitClass, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:204
	// ("cv::cuda::FastOpticalFlowBM::operator()", vec![(pred!(mut, ["I0", "I1", "flowx", "flowy"], ["const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*"]), _)]),
	void cv_cuda_FastOpticalFlowBM_operator___const_GpuMatR_const_GpuMatR_GpuMatR_GpuMatR(cv::cuda::FastOpticalFlowBM* instance, const cv::cuda::GpuMat* I0, const cv::cuda::GpuMat* I1, cv::cuda::GpuMat* flowx, cv::cuda::GpuMat* flowy, ResultVoid* ocvrs_return) {
		try {
			instance->operator()(*I0, *I1, *flowx, *flowy);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::FastOpticalFlowBM::defaultNew() generated
	// ("cv::cuda::FastOpticalFlowBM::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::cuda::FastOpticalFlowBM* cv_cuda_FastOpticalFlowBM_defaultNew_const() {
			cv::cuda::FastOpticalFlowBM* ret = new cv::cuda::FastOpticalFlowBM();
			return ret;
	}

	// cv::cuda::FastOpticalFlowBM::delete() generated
	// ("cv::cuda::FastOpticalFlowBM::delete", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_FastOpticalFlowBM_delete(cv::cuda::FastOpticalFlowBM* instance) {
			delete instance;
	}

	// getLayer(OutputArray, Size, Stream &)(OutputArray, SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:75
	// ("cv::cuda::ImagePyramid::getLayer", vec![(pred!(const, ["outImg", "outRoi", "stream"], ["const cv::_OutputArray*", "cv::Size", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_ImagePyramid_getLayer_const_const__OutputArrayR_Size_StreamR(const cv::cuda::ImagePyramid* instance, const cv::_OutputArray* outImg, cv::Size* outRoi, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			instance->getLayer(*outImg, *outRoi, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::ImagePyramid::getLayer(OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudalegacy.hpp:75
	// ("cv::cuda::ImagePyramid::getLayer", vec![(pred!(const, ["outImg", "outRoi"], ["const cv::_OutputArray*", "cv::Size"]), _)]),
	void cv_cuda_ImagePyramid_getLayer_const_const__OutputArrayR_Size(const cv::cuda::ImagePyramid* instance, const cv::_OutputArray* outImg, cv::Size* outRoi, ResultVoid* ocvrs_return) {
		try {
			instance->getLayer(*outImg, *outRoi);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::ImagePyramid::to_Algorithm() generated
	// ("cv::cuda::ImagePyramid::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_cuda_ImagePyramid_to_Algorithm(cv::cuda::ImagePyramid* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::cuda::ImagePyramid::delete() generated
	// ("cv::cuda::ImagePyramid::delete", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_ImagePyramid_delete(cv::cuda::ImagePyramid* instance) {
			delete instance;
	}

}
