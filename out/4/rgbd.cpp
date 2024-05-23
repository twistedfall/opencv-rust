#include "ocvrs_common.hpp"
#include <opencv2/rgbd.hpp>
#include "rgbd_types.hpp"

extern "C" {
	// makeVolume(VolumeType, float, Matx44f, float, float, int, float, Vec3i)(Enum, Primitive, SimpleClass, Primitive, Primitive, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:122
	// ("cv::kinfu::makeVolume", vec![(pred!(mut, ["_volumeType", "_voxelSize", "_pose", "_raycastStepFactor", "_truncDist", "_maxWeight", "_truncateThreshold", "_resolution"], ["cv::kinfu::VolumeType", "float", "cv::Matx44f", "float", "float", "int", "float", "cv::Vec3i"]), _)]),
	void cv_kinfu_makeVolume_VolumeType_float_Matx44f_float_float_int_float_Vec3i(cv::kinfu::VolumeType _volumeType, float _voxelSize, cv::Matx44f* _pose, float _raycastStepFactor, float _truncDist, int _maxWeight, float _truncateThreshold, cv::Vec3i* _resolution, Result<cv::Ptr<cv::kinfu::Volume>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::kinfu::Volume> ret = cv::kinfu::makeVolume(_volumeType, _voxelSize, *_pose, _raycastStepFactor, _truncDist, _maxWeight, _truncateThreshold, *_resolution);
			Ok(new cv::Ptr<cv::kinfu::Volume>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// colormap(const Mat &, Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:245
	// ("cv::linemod::colormap", vec![(pred!(mut, ["quantized", "dst"], ["const cv::Mat*", "cv::Mat*"]), _)]),
	void cv_linemod_colormap_const_MatR_MatR(const cv::Mat* quantized, cv::Mat* dst, ResultVoid* ocvrs_return) {
		try {
			cv::linemod::colormap(*quantized, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::linemod::drawFeatures(InputOutputArray, CppPassByVoidPtr, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:254
	// ("cv::linemod::drawFeatures", vec![(pred!(mut, ["img", "templates", "tl"], ["const cv::_InputOutputArray*", "const std::vector<cv::linemod::Template>*", "const cv::Point2i*"]), _)]),
	void cv_linemod_drawFeatures_const__InputOutputArrayR_const_vectorLTemplateGR_const_Point2iR(const cv::_InputOutputArray* img, const std::vector<cv::linemod::Template>* templates, const cv::Point2i* tl, ResultVoid* ocvrs_return) {
		try {
			cv::linemod::drawFeatures(*img, *templates, *tl);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// drawFeatures(InputOutputArray, const std::vector<Template> &, const Point2i &, int)(InputOutputArray, CppPassByVoidPtr, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:254
	// ("cv::linemod::drawFeatures", vec![(pred!(mut, ["img", "templates", "tl", "size"], ["const cv::_InputOutputArray*", "const std::vector<cv::linemod::Template>*", "const cv::Point2i*", "int"]), _)]),
	void cv_linemod_drawFeatures_const__InputOutputArrayR_const_vectorLTemplateGR_const_Point2iR_int(const cv::_InputOutputArray* img, const std::vector<cv::linemod::Template>* templates, const cv::Point2i* tl, int size, ResultVoid* ocvrs_return) {
		try {
			cv::linemod::drawFeatures(*img, *templates, *tl, size);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDefaultLINE()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:420
	// ("cv::linemod::getDefaultLINE", vec![(pred!(mut, [], []), _)]),
	void cv_linemod_getDefaultLINE(Result<cv::Ptr<cv::linemod::Detector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::linemod::Detector> ret = cv::linemod::getDefaultLINE();
			Ok(new cv::Ptr<cv::linemod::Detector>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDefaultLINEMOD()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:428
	// ("cv::linemod::getDefaultLINEMOD", vec![(pred!(mut, [], []), _)]),
	void cv_linemod_getDefaultLINEMOD(Result<cv::Ptr<cv::linemod::Detector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::linemod::Detector> ret = cv::linemod::getDefaultLINEMOD();
			Ok(new cv::Ptr<cv::linemod::Detector>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// depthTo3dSparse(InputArray, InputArray, InputArray, OutputArray)(InputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:299
	// ("cv::rgbd::depthTo3dSparse", vec![(pred!(mut, ["depth", "in_K", "in_points", "points3d"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_rgbd_depthTo3dSparse_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* depth, const cv::_InputArray* in_K, const cv::_InputArray* in_points, const cv::_OutputArray* points3d, ResultVoid* ocvrs_return) {
		try {
			cv::rgbd::depthTo3dSparse(*depth, *in_K, *in_points, *points3d);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rgbd::depthTo3d(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:312
	// ("cv::rgbd::depthTo3d", vec![(pred!(mut, ["depth", "K", "points3d"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_rgbd_depthTo3d_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* depth, const cv::_InputArray* K, const cv::_OutputArray* points3d, ResultVoid* ocvrs_return) {
		try {
			cv::rgbd::depthTo3d(*depth, *K, *points3d);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// depthTo3d(InputArray, InputArray, OutputArray, InputArray)(InputArray, InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:312
	// ("cv::rgbd::depthTo3d", vec![(pred!(mut, ["depth", "K", "points3d", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
	void cv_rgbd_depthTo3d_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR(const cv::_InputArray* depth, const cv::_InputArray* K, const cv::_OutputArray* points3d, const cv::_InputArray* mask, ResultVoid* ocvrs_return) {
		try {
			cv::rgbd::depthTo3d(*depth, *K, *points3d, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isValidDepth(const double &)(Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:33
	// ("cv::rgbd::isValidDepth", vec![(pred!(mut, ["depth"], ["const double*"]), _)]),
	void cv_rgbd_isValidDepth_const_doubleR(const double* depth, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::rgbd::isValidDepth(*depth);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isValidDepth(const float &)(Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:27
	// ("cv::rgbd::isValidDepth", vec![(pred!(mut, ["depth"], ["const float*"]), _)]),
	void cv_rgbd_isValidDepth_const_floatR(const float* depth, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::rgbd::isValidDepth(*depth);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isValidDepth(const int &)(Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:52
	// ("cv::rgbd::isValidDepth", vec![(pred!(mut, ["depth"], ["const int*"]), _)]),
	void cv_rgbd_isValidDepth_const_intR(const int* depth, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::rgbd::isValidDepth(*depth);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isValidDepth(const short &)(Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:39
	// ("cv::rgbd::isValidDepth", vec![(pred!(mut, ["depth"], ["const short*"]), _)]),
	void cv_rgbd_isValidDepth_const_shortR(const short* depth, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::rgbd::isValidDepth(*depth);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isValidDepth(const unsigned int &)(Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:58
	// ("cv::rgbd::isValidDepth", vec![(pred!(mut, ["depth"], ["const unsigned int*"]), _)]),
	void cv_rgbd_isValidDepth_const_unsigned_intR(const unsigned int* depth, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::rgbd::isValidDepth(*depth);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isValidDepth(const unsigned short &)(Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:45
	// ("cv::rgbd::isValidDepth", vec![(pred!(mut, ["depth"], ["const unsigned short*"]), _)]),
	void cv_rgbd_isValidDepth_const_unsigned_shortR(const unsigned short* depth, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::rgbd::isValidDepth(*depth);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rgbd::registerDepth(InputArray, InputArray, InputArray, InputArray, InputArray, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:287
	// ("cv::rgbd::registerDepth", vec![(pred!(mut, ["unregisteredCameraMatrix", "registeredCameraMatrix", "registeredDistCoeffs", "Rt", "unregisteredDepth", "outputImagePlaneSize", "registeredDepth"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Size*", "const cv::_OutputArray*"]), _)]),
	void cv_rgbd_registerDepth_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR_const__OutputArrayR(const cv::_InputArray* unregisteredCameraMatrix, const cv::_InputArray* registeredCameraMatrix, const cv::_InputArray* registeredDistCoeffs, const cv::_InputArray* Rt, const cv::_InputArray* unregisteredDepth, const cv::Size* outputImagePlaneSize, const cv::_OutputArray* registeredDepth, ResultVoid* ocvrs_return) {
		try {
			cv::rgbd::registerDepth(*unregisteredCameraMatrix, *registeredCameraMatrix, *registeredDistCoeffs, *Rt, *unregisteredDepth, *outputImagePlaneSize, *registeredDepth);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// registerDepth(InputArray, InputArray, InputArray, InputArray, InputArray, const Size &, OutputArray, bool)(InputArray, InputArray, InputArray, InputArray, InputArray, SimpleClass, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:287
	// ("cv::rgbd::registerDepth", vec![(pred!(mut, ["unregisteredCameraMatrix", "registeredCameraMatrix", "registeredDistCoeffs", "Rt", "unregisteredDepth", "outputImagePlaneSize", "registeredDepth", "depthDilation"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Size*", "const cv::_OutputArray*", "bool"]), _)]),
	void cv_rgbd_registerDepth_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR_const__OutputArrayR_bool(const cv::_InputArray* unregisteredCameraMatrix, const cv::_InputArray* registeredCameraMatrix, const cv::_InputArray* registeredDistCoeffs, const cv::_InputArray* Rt, const cv::_InputArray* unregisteredDepth, const cv::Size* outputImagePlaneSize, const cv::_OutputArray* registeredDepth, bool depthDilation, ResultVoid* ocvrs_return) {
		try {
			cv::rgbd::registerDepth(*unregisteredCameraMatrix, *registeredCameraMatrix, *registeredDistCoeffs, *Rt, *unregisteredDepth, *outputImagePlaneSize, *registeredDepth, depthDilation);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rgbd::rescaleDepth(InputArray, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:325
	// ("cv::rgbd::rescaleDepth", vec![(pred!(mut, ["in", "depth", "out"], ["const cv::_InputArray*", "int", "const cv::_OutputArray*"]), _)]),
	void cv_rgbd_rescaleDepth_const__InputArrayR_int_const__OutputArrayR(const cv::_InputArray* in, int depth, const cv::_OutputArray* out, ResultVoid* ocvrs_return) {
		try {
			cv::rgbd::rescaleDepth(*in, depth, *out);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// rescaleDepth(InputArray, int, OutputArray, double)(InputArray, Primitive, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:325
	// ("cv::rgbd::rescaleDepth", vec![(pred!(mut, ["in", "depth", "out", "depth_factor"], ["const cv::_InputArray*", "int", "const cv::_OutputArray*", "double"]), _)]),
	void cv_rgbd_rescaleDepth_const__InputArrayR_int_const__OutputArrayR_double(const cv::_InputArray* in, int depth, const cv::_OutputArray* out, double depth_factor, ResultVoid* ocvrs_return) {
		try {
			cv::rgbd::rescaleDepth(*in, depth, *out, depth_factor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rgbd::warpFrame(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1179
	// ("cv::rgbd::warpFrame", vec![(pred!(mut, ["image", "depth", "mask", "Rt", "cameraMatrix", "distCoeff", "warpedImage"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::_OutputArray*"]), _)]),
	void cv_rgbd_warpFrame_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_const__OutputArrayR(const cv::Mat* image, const cv::Mat* depth, const cv::Mat* mask, const cv::Mat* Rt, const cv::Mat* cameraMatrix, const cv::Mat* distCoeff, const cv::_OutputArray* warpedImage, ResultVoid* ocvrs_return) {
		try {
			cv::rgbd::warpFrame(*image, *depth, *mask, *Rt, *cameraMatrix, *distCoeff, *warpedImage);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// warpFrame(const Mat &, const Mat &, const Mat &, const Mat &, const Mat &, const Mat &, OutputArray, OutputArray, OutputArray)(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1179
	// ("cv::rgbd::warpFrame", vec![(pred!(mut, ["image", "depth", "mask", "Rt", "cameraMatrix", "distCoeff", "warpedImage", "warpedDepth", "warpedMask"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_rgbd_warpFrame_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::Mat* image, const cv::Mat* depth, const cv::Mat* mask, const cv::Mat* Rt, const cv::Mat* cameraMatrix, const cv::Mat* distCoeff, const cv::_OutputArray* warpedImage, const cv::_OutputArray* warpedDepth, const cv::_OutputArray* warpedMask, ResultVoid* ocvrs_return) {
		try {
			cv::rgbd::warpFrame(*image, *depth, *mask, *Rt, *cameraMatrix, *distCoeff, *warpedImage, *warpedDepth, *warpedMask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const Ptr<Params> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:198
	// ("cv::colored_kinfu::ColoredKinFu::create", vec![(pred!(mut, ["_params"], ["const cv::Ptr<cv::colored_kinfu::Params>*"]), _)]),
	void cv_colored_kinfu_ColoredKinFu_create_const_PtrLParamsGR(const cv::Ptr<cv::colored_kinfu::Params>* _params, Result<cv::Ptr<cv::colored_kinfu::ColoredKinFu>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::colored_kinfu::ColoredKinFu> ret = cv::colored_kinfu::ColoredKinFu::create(*_params);
			Ok(new cv::Ptr<cv::colored_kinfu::ColoredKinFu>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getParams()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:202
	// ("cv::colored_kinfu::ColoredKinFu::getParams", vec![(pred!(const, [], []), _)]),
	void cv_colored_kinfu_ColoredKinFu_getParams_const(const cv::colored_kinfu::ColoredKinFu* instance, Result<cv::colored_kinfu::Params*>* ocvrs_return) {
		try {
			const cv::colored_kinfu::Params ret = instance->getParams();
			Ok(new const cv::colored_kinfu::Params(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// render(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:212
	// ("cv::colored_kinfu::ColoredKinFu::render", vec![(pred!(const, ["image"], ["const cv::_OutputArray*"]), _)]),
	void cv_colored_kinfu_ColoredKinFu_render_const_const__OutputArrayR(const cv::colored_kinfu::ColoredKinFu* instance, const cv::_OutputArray* image, ResultVoid* ocvrs_return) {
		try {
			instance->render(*image);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// render(OutputArray, const Matx44f &)(OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:224
	// ("cv::colored_kinfu::ColoredKinFu::render", vec![(pred!(const, ["image", "cameraPose"], ["const cv::_OutputArray*", "const cv::Matx44f*"]), _)]),
	void cv_colored_kinfu_ColoredKinFu_render_const_const__OutputArrayR_const_Matx44fR(const cv::colored_kinfu::ColoredKinFu* instance, const cv::_OutputArray* image, const cv::Matx44f* cameraPose, ResultVoid* ocvrs_return) {
		try {
			instance->render(*image, *cameraPose);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getCloud(OutputArray, OutputArray, OutputArray)(OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:235
	// ("cv::colored_kinfu::ColoredKinFu::getCloud", vec![(pred!(const, ["points", "normals", "colors"], ["const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_colored_kinfu_ColoredKinFu_getCloud_const_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::colored_kinfu::ColoredKinFu* instance, const cv::_OutputArray* points, const cv::_OutputArray* normals, const cv::_OutputArray* colors, ResultVoid* ocvrs_return) {
		try {
			instance->getCloud(*points, *normals, *colors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::colored_kinfu::ColoredKinFu::getCloud(OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:235
	// ("cv::colored_kinfu::ColoredKinFu::getCloud", vec![(pred!(const, ["points", "normals"], ["const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_colored_kinfu_ColoredKinFu_getCloud_const_const__OutputArrayR_const__OutputArrayR(const cv::colored_kinfu::ColoredKinFu* instance, const cv::_OutputArray* points, const cv::_OutputArray* normals, ResultVoid* ocvrs_return) {
		try {
			instance->getCloud(*points, *normals);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPoints(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:243
	// ("cv::colored_kinfu::ColoredKinFu::getPoints", vec![(pred!(const, ["points"], ["const cv::_OutputArray*"]), _)]),
	void cv_colored_kinfu_ColoredKinFu_getPoints_const_const__OutputArrayR(const cv::colored_kinfu::ColoredKinFu* instance, const cv::_OutputArray* points, ResultVoid* ocvrs_return) {
		try {
			instance->getPoints(*points);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNormals(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:249
	// ("cv::colored_kinfu::ColoredKinFu::getNormals", vec![(pred!(const, ["points", "normals"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_colored_kinfu_ColoredKinFu_getNormals_const_const__InputArrayR_const__OutputArrayR(const cv::colored_kinfu::ColoredKinFu* instance, const cv::_InputArray* points, const cv::_OutputArray* normals, ResultVoid* ocvrs_return) {
		try {
			instance->getNormals(*points, *normals);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// reset()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:255
	// ("cv::colored_kinfu::ColoredKinFu::reset", vec![(pred!(mut, [], []), _)]),
	void cv_colored_kinfu_ColoredKinFu_reset(cv::colored_kinfu::ColoredKinFu* instance, ResultVoid* ocvrs_return) {
		try {
			instance->reset();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPose()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:258
	// ("cv::colored_kinfu::ColoredKinFu::getPose", vec![(pred!(const, [], []), _)]),
	void cv_colored_kinfu_ColoredKinFu_getPose_const(const cv::colored_kinfu::ColoredKinFu* instance, Result<cv::Affine3f>* ocvrs_return) {
		try {
			cv::Affine3f ret = instance->getPose();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// update(InputArray, InputArray)(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:266
	// ("cv::colored_kinfu::ColoredKinFu::update", vec![(pred!(mut, ["depth", "rgb"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_colored_kinfu_ColoredKinFu_update_const__InputArrayR_const__InputArrayR(cv::colored_kinfu::ColoredKinFu* instance, const cv::_InputArray* depth, const cv::_InputArray* rgb, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->update(*depth, *rgb);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::colored_kinfu::ColoredKinFu::delete() generated
	// ("cv::colored_kinfu::ColoredKinFu::delete", vec![(pred!(mut, [], []), _)]),
	void cv_colored_kinfu_ColoredKinFu_delete(cv::colored_kinfu::ColoredKinFu* instance) {
			delete instance;
	}

	// Params()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:22
	// ("cv::colored_kinfu::Params::Params", vec![(pred!(mut, [], []), _)]),
	void cv_colored_kinfu_Params_Params(Result<cv::colored_kinfu::Params*>* ocvrs_return) {
		try {
			cv::colored_kinfu::Params* ret = new cv::colored_kinfu::Params();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Params(Matx33f, Vec3f)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:30
	// ("cv::colored_kinfu::Params::Params", vec![(pred!(mut, ["volumeInitialPoseRot", "volumeInitialPoseTransl"], ["cv::Matx33f", "cv::Vec3f"]), _)]),
	void cv_colored_kinfu_Params_Params_Matx33f_Vec3f(cv::Matx33f* volumeInitialPoseRot, cv::Vec3f* volumeInitialPoseTransl, Result<cv::colored_kinfu::Params*>* ocvrs_return) {
		try {
			cv::colored_kinfu::Params* ret = new cv::colored_kinfu::Params(*volumeInitialPoseRot, *volumeInitialPoseTransl);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Params(Matx44f)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:40
	// ("cv::colored_kinfu::Params::Params", vec![(pred!(mut, ["volumeInitialPose"], ["cv::Matx44f"]), _)]),
	void cv_colored_kinfu_Params_Params_Matx44f(cv::Matx44f* volumeInitialPose, Result<cv::colored_kinfu::Params*>* ocvrs_return) {
		try {
			cv::colored_kinfu::Params* ret = new cv::colored_kinfu::Params(*volumeInitialPose);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setInitialVolumePose(Matx33f, Vec3f)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:51
	// ("cv::colored_kinfu::Params::setInitialVolumePose", vec![(pred!(mut, ["R", "t"], ["cv::Matx33f", "cv::Vec3f"]), _)]),
	void cv_colored_kinfu_Params_setInitialVolumePose_Matx33f_Vec3f(cv::colored_kinfu::Params* instance, cv::Matx33f* R, cv::Vec3f* t, ResultVoid* ocvrs_return) {
		try {
			instance->setInitialVolumePose(*R, *t);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setInitialVolumePose(Matx44f)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:58
	// ("cv::colored_kinfu::Params::setInitialVolumePose", vec![(pred!(mut, ["homogen_tf"], ["cv::Matx44f"]), _)]),
	void cv_colored_kinfu_Params_setInitialVolumePose_Matx44f(cv::colored_kinfu::Params* instance, cv::Matx44f* homogen_tf, ResultVoid* ocvrs_return) {
		try {
			instance->setInitialVolumePose(*homogen_tf);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// defaultParams()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:64
	// ("cv::colored_kinfu::Params::defaultParams", vec![(pred!(mut, [], []), _)]),
	void cv_colored_kinfu_Params_defaultParams(Result<cv::Ptr<cv::colored_kinfu::Params>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::colored_kinfu::Params> ret = cv::colored_kinfu::Params::defaultParams();
			Ok(new cv::Ptr<cv::colored_kinfu::Params>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// coarseParams()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:70
	// ("cv::colored_kinfu::Params::coarseParams", vec![(pred!(mut, [], []), _)]),
	void cv_colored_kinfu_Params_coarseParams(Result<cv::Ptr<cv::colored_kinfu::Params>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::colored_kinfu::Params> ret = cv::colored_kinfu::Params::coarseParams();
			Ok(new cv::Ptr<cv::colored_kinfu::Params>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// hashTSDFParams(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:75
	// ("cv::colored_kinfu::Params::hashTSDFParams", vec![(pred!(mut, ["isCoarse"], ["bool"]), _)]),
	void cv_colored_kinfu_Params_hashTSDFParams_bool(bool isCoarse, Result<cv::Ptr<cv::colored_kinfu::Params>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::colored_kinfu::Params> ret = cv::colored_kinfu::Params::hashTSDFParams(isCoarse);
			Ok(new cv::Ptr<cv::colored_kinfu::Params>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// coloredTSDFParams(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:80
	// ("cv::colored_kinfu::Params::coloredTSDFParams", vec![(pred!(mut, ["isCoarse"], ["bool"]), _)]),
	void cv_colored_kinfu_Params_coloredTSDFParams_bool(bool isCoarse, Result<cv::Ptr<cv::colored_kinfu::Params>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::colored_kinfu::Params> ret = cv::colored_kinfu::Params::coloredTSDFParams(isCoarse);
			Ok(new cv::Ptr<cv::colored_kinfu::Params>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::colored_kinfu::Params::frameSize() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:83
	// ("cv::colored_kinfu::Params::frameSize", vec![(pred!(const, [], []), _)]),
	void cv_colored_kinfu_Params_propFrameSize_const(const cv::colored_kinfu::Params* instance, cv::Size* ocvrs_return) {
			cv::Size ret = instance->frameSize;
			*ocvrs_return = ret;
	}

	// cv::colored_kinfu::Params::setFrameSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:83
	// ("cv::colored_kinfu::Params::setFrameSize", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	void cv_colored_kinfu_Params_propFrameSize_const_Size(cv::colored_kinfu::Params* instance, const cv::Size* val) {
			instance->frameSize = *val;
	}

	// cv::colored_kinfu::Params::rgb_frameSize() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:86
	// ("cv::colored_kinfu::Params::rgb_frameSize", vec![(pred!(const, [], []), _)]),
	void cv_colored_kinfu_Params_propRgb_frameSize_const(const cv::colored_kinfu::Params* instance, cv::Size* ocvrs_return) {
			cv::Size ret = instance->rgb_frameSize;
			*ocvrs_return = ret;
	}

	// cv::colored_kinfu::Params::setRgb_frameSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:86
	// ("cv::colored_kinfu::Params::setRgb_frameSize", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	void cv_colored_kinfu_Params_propRgb_frameSize_const_Size(cv::colored_kinfu::Params* instance, const cv::Size* val) {
			instance->rgb_frameSize = *val;
	}

	// cv::colored_kinfu::Params::volumeType() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:88
	// ("cv::colored_kinfu::Params::volumeType", vec![(pred!(const, [], []), _)]),
	void cv_colored_kinfu_Params_propVolumeType_const(const cv::colored_kinfu::Params* instance, cv::kinfu::VolumeType* ocvrs_return) {
			cv::kinfu::VolumeType ret = instance->volumeType;
			*ocvrs_return = ret;
	}

	// cv::colored_kinfu::Params::setVolumeType(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:88
	// ("cv::colored_kinfu::Params::setVolumeType", vec![(pred!(mut, ["val"], ["const cv::kinfu::VolumeType"]), _)]),
	void cv_colored_kinfu_Params_propVolumeType_const_VolumeType(cv::colored_kinfu::Params* instance, const cv::kinfu::VolumeType val) {
			instance->volumeType = val;
	}

	// cv::colored_kinfu::Params::intr() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:91
	// ("cv::colored_kinfu::Params::intr", vec![(pred!(const, [], []), _)]),
	void cv_colored_kinfu_Params_propIntr_const(const cv::colored_kinfu::Params* instance, cv::Matx33f* ocvrs_return) {
			cv::Matx33f ret = instance->intr;
			*ocvrs_return = ret;
	}

	// cv::colored_kinfu::Params::setIntr(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:91
	// ("cv::colored_kinfu::Params::setIntr", vec![(pred!(mut, ["val"], ["const cv::Matx33f"]), _)]),
	void cv_colored_kinfu_Params_propIntr_const_Matx33f(cv::colored_kinfu::Params* instance, const cv::Matx33f* val) {
			instance->intr = *val;
	}

	// cv::colored_kinfu::Params::rgb_intr() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:94
	// ("cv::colored_kinfu::Params::rgb_intr", vec![(pred!(const, [], []), _)]),
	void cv_colored_kinfu_Params_propRgb_intr_const(const cv::colored_kinfu::Params* instance, cv::Matx33f* ocvrs_return) {
			cv::Matx33f ret = instance->rgb_intr;
			*ocvrs_return = ret;
	}

	// cv::colored_kinfu::Params::setRgb_intr(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:94
	// ("cv::colored_kinfu::Params::setRgb_intr", vec![(pred!(mut, ["val"], ["const cv::Matx33f"]), _)]),
	void cv_colored_kinfu_Params_propRgb_intr_const_Matx33f(cv::colored_kinfu::Params* instance, const cv::Matx33f* val) {
			instance->rgb_intr = *val;
	}

	// cv::colored_kinfu::Params::depthFactor() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:103
	// ("cv::colored_kinfu::Params::depthFactor", vec![(pred!(const, [], []), _)]),
	float cv_colored_kinfu_Params_propDepthFactor_const(const cv::colored_kinfu::Params* instance) {
			float ret = instance->depthFactor;
			return ret;
	}

	// cv::colored_kinfu::Params::setDepthFactor(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:103
	// ("cv::colored_kinfu::Params::setDepthFactor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_colored_kinfu_Params_propDepthFactor_const_float(cv::colored_kinfu::Params* instance, const float val) {
			instance->depthFactor = val;
	}

	// cv::colored_kinfu::Params::bilateral_sigma_depth() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:106
	// ("cv::colored_kinfu::Params::bilateral_sigma_depth", vec![(pred!(const, [], []), _)]),
	float cv_colored_kinfu_Params_propBilateral_sigma_depth_const(const cv::colored_kinfu::Params* instance) {
			float ret = instance->bilateral_sigma_depth;
			return ret;
	}

	// cv::colored_kinfu::Params::setBilateral_sigma_depth(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:106
	// ("cv::colored_kinfu::Params::setBilateral_sigma_depth", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_colored_kinfu_Params_propBilateral_sigma_depth_const_float(cv::colored_kinfu::Params* instance, const float val) {
			instance->bilateral_sigma_depth = val;
	}

	// cv::colored_kinfu::Params::bilateral_sigma_spatial() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:108
	// ("cv::colored_kinfu::Params::bilateral_sigma_spatial", vec![(pred!(const, [], []), _)]),
	float cv_colored_kinfu_Params_propBilateral_sigma_spatial_const(const cv::colored_kinfu::Params* instance) {
			float ret = instance->bilateral_sigma_spatial;
			return ret;
	}

	// cv::colored_kinfu::Params::setBilateral_sigma_spatial(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:108
	// ("cv::colored_kinfu::Params::setBilateral_sigma_spatial", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_colored_kinfu_Params_propBilateral_sigma_spatial_const_float(cv::colored_kinfu::Params* instance, const float val) {
			instance->bilateral_sigma_spatial = val;
	}

	// cv::colored_kinfu::Params::bilateral_kernel_size() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:110
	// ("cv::colored_kinfu::Params::bilateral_kernel_size", vec![(pred!(const, [], []), _)]),
	int cv_colored_kinfu_Params_propBilateral_kernel_size_const(const cv::colored_kinfu::Params* instance) {
			int ret = instance->bilateral_kernel_size;
			return ret;
	}

	// cv::colored_kinfu::Params::setBilateral_kernel_size(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:110
	// ("cv::colored_kinfu::Params::setBilateral_kernel_size", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_colored_kinfu_Params_propBilateral_kernel_size_const_int(cv::colored_kinfu::Params* instance, const int val) {
			instance->bilateral_kernel_size = val;
	}

	// cv::colored_kinfu::Params::pyramidLevels() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:113
	// ("cv::colored_kinfu::Params::pyramidLevels", vec![(pred!(const, [], []), _)]),
	int cv_colored_kinfu_Params_propPyramidLevels_const(const cv::colored_kinfu::Params* instance) {
			int ret = instance->pyramidLevels;
			return ret;
	}

	// cv::colored_kinfu::Params::setPyramidLevels(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:113
	// ("cv::colored_kinfu::Params::setPyramidLevels", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_colored_kinfu_Params_propPyramidLevels_const_int(cv::colored_kinfu::Params* instance, const int val) {
			instance->pyramidLevels = val;
	}

	// cv::colored_kinfu::Params::volumeDims() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:119
	// ("cv::colored_kinfu::Params::volumeDims", vec![(pred!(const, [], []), _)]),
	void cv_colored_kinfu_Params_propVolumeDims_const(const cv::colored_kinfu::Params* instance, cv::Vec3i* ocvrs_return) {
			cv::Vec3i ret = instance->volumeDims;
			*ocvrs_return = ret;
	}

	// cv::colored_kinfu::Params::setVolumeDims(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:119
	// ("cv::colored_kinfu::Params::setVolumeDims", vec![(pred!(mut, ["val"], ["const cv::Vec3i"]), _)]),
	void cv_colored_kinfu_Params_propVolumeDims_const_Vec3i(cv::colored_kinfu::Params* instance, const cv::Vec3i* val) {
			instance->volumeDims = *val;
	}

	// cv::colored_kinfu::Params::voxelSize() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:121
	// ("cv::colored_kinfu::Params::voxelSize", vec![(pred!(const, [], []), _)]),
	float cv_colored_kinfu_Params_propVoxelSize_const(const cv::colored_kinfu::Params* instance) {
			float ret = instance->voxelSize;
			return ret;
	}

	// cv::colored_kinfu::Params::setVoxelSize(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:121
	// ("cv::colored_kinfu::Params::setVoxelSize", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_colored_kinfu_Params_propVoxelSize_const_float(cv::colored_kinfu::Params* instance, const float val) {
			instance->voxelSize = val;
	}

	// cv::colored_kinfu::Params::tsdf_min_camera_movement() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:127
	// ("cv::colored_kinfu::Params::tsdf_min_camera_movement", vec![(pred!(const, [], []), _)]),
	float cv_colored_kinfu_Params_propTsdf_min_camera_movement_const(const cv::colored_kinfu::Params* instance) {
			float ret = instance->tsdf_min_camera_movement;
			return ret;
	}

	// cv::colored_kinfu::Params::setTsdf_min_camera_movement(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:127
	// ("cv::colored_kinfu::Params::setTsdf_min_camera_movement", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_colored_kinfu_Params_propTsdf_min_camera_movement_const_float(cv::colored_kinfu::Params* instance, const float val) {
			instance->tsdf_min_camera_movement = val;
	}

	// cv::colored_kinfu::Params::volumePose() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:130
	// ("cv::colored_kinfu::Params::volumePose", vec![(pred!(const, [], []), _)]),
	void cv_colored_kinfu_Params_propVolumePose_const(const cv::colored_kinfu::Params* instance, cv::Affine3f* ocvrs_return) {
			cv::Affine3f ret = instance->volumePose;
			*ocvrs_return = ret;
	}

	// cv::colored_kinfu::Params::setVolumePose(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:130
	// ("cv::colored_kinfu::Params::setVolumePose", vec![(pred!(mut, ["val"], ["const cv::Affine3f"]), _)]),
	void cv_colored_kinfu_Params_propVolumePose_const_Affine3f(cv::colored_kinfu::Params* instance, const cv::Affine3f* val) {
			instance->volumePose = *val;
	}

	// cv::colored_kinfu::Params::tsdf_trunc_dist() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:136
	// ("cv::colored_kinfu::Params::tsdf_trunc_dist", vec![(pred!(const, [], []), _)]),
	float cv_colored_kinfu_Params_propTsdf_trunc_dist_const(const cv::colored_kinfu::Params* instance) {
			float ret = instance->tsdf_trunc_dist;
			return ret;
	}

	// cv::colored_kinfu::Params::setTsdf_trunc_dist(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:136
	// ("cv::colored_kinfu::Params::setTsdf_trunc_dist", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_colored_kinfu_Params_propTsdf_trunc_dist_const_float(cv::colored_kinfu::Params* instance, const float val) {
			instance->tsdf_trunc_dist = val;
	}

	// cv::colored_kinfu::Params::tsdf_max_weight() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:142
	// ("cv::colored_kinfu::Params::tsdf_max_weight", vec![(pred!(const, [], []), _)]),
	int cv_colored_kinfu_Params_propTsdf_max_weight_const(const cv::colored_kinfu::Params* instance) {
			int ret = instance->tsdf_max_weight;
			return ret;
	}

	// cv::colored_kinfu::Params::setTsdf_max_weight(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:142
	// ("cv::colored_kinfu::Params::setTsdf_max_weight", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_colored_kinfu_Params_propTsdf_max_weight_const_int(cv::colored_kinfu::Params* instance, const int val) {
			instance->tsdf_max_weight = val;
	}

	// cv::colored_kinfu::Params::raycast_step_factor() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:148
	// ("cv::colored_kinfu::Params::raycast_step_factor", vec![(pred!(const, [], []), _)]),
	float cv_colored_kinfu_Params_propRaycast_step_factor_const(const cv::colored_kinfu::Params* instance) {
			float ret = instance->raycast_step_factor;
			return ret;
	}

	// cv::colored_kinfu::Params::setRaycast_step_factor(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:148
	// ("cv::colored_kinfu::Params::setRaycast_step_factor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_colored_kinfu_Params_propRaycast_step_factor_const_float(cv::colored_kinfu::Params* instance, const float val) {
			instance->raycast_step_factor = val;
	}

	// cv::colored_kinfu::Params::lightPose() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:155
	// ("cv::colored_kinfu::Params::lightPose", vec![(pred!(const, [], []), _)]),
	void cv_colored_kinfu_Params_propLightPose_const(const cv::colored_kinfu::Params* instance, cv::Vec3f* ocvrs_return) {
			cv::Vec3f ret = instance->lightPose;
			*ocvrs_return = ret;
	}

	// cv::colored_kinfu::Params::setLightPose(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:155
	// ("cv::colored_kinfu::Params::setLightPose", vec![(pred!(mut, ["val"], ["const cv::Vec3f"]), _)]),
	void cv_colored_kinfu_Params_propLightPose_const_Vec3f(cv::colored_kinfu::Params* instance, const cv::Vec3f* val) {
			instance->lightPose = *val;
	}

	// cv::colored_kinfu::Params::icpDistThresh() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:158
	// ("cv::colored_kinfu::Params::icpDistThresh", vec![(pred!(const, [], []), _)]),
	float cv_colored_kinfu_Params_propIcpDistThresh_const(const cv::colored_kinfu::Params* instance) {
			float ret = instance->icpDistThresh;
			return ret;
	}

	// cv::colored_kinfu::Params::setIcpDistThresh(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:158
	// ("cv::colored_kinfu::Params::setIcpDistThresh", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_colored_kinfu_Params_propIcpDistThresh_const_float(cv::colored_kinfu::Params* instance, const float val) {
			instance->icpDistThresh = val;
	}

	// cv::colored_kinfu::Params::icpAngleThresh() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:160
	// ("cv::colored_kinfu::Params::icpAngleThresh", vec![(pred!(const, [], []), _)]),
	float cv_colored_kinfu_Params_propIcpAngleThresh_const(const cv::colored_kinfu::Params* instance) {
			float ret = instance->icpAngleThresh;
			return ret;
	}

	// cv::colored_kinfu::Params::setIcpAngleThresh(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:160
	// ("cv::colored_kinfu::Params::setIcpAngleThresh", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_colored_kinfu_Params_propIcpAngleThresh_const_float(cv::colored_kinfu::Params* instance, const float val) {
			instance->icpAngleThresh = val;
	}

	// cv::colored_kinfu::Params::icpIterations() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:162
	// ("cv::colored_kinfu::Params::icpIterations", vec![(pred!(const, [], []), _)]),
	std::vector<int>* cv_colored_kinfu_Params_propIcpIterations_const(const cv::colored_kinfu::Params* instance) {
			std::vector<int> ret = instance->icpIterations;
			return new std::vector<int>(ret);
	}

	// cv::colored_kinfu::Params::setIcpIterations(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:162
	// ("cv::colored_kinfu::Params::setIcpIterations", vec![(pred!(mut, ["val"], ["const std::vector<int>"]), _)]),
	void cv_colored_kinfu_Params_propIcpIterations_const_vectorLintG(cv::colored_kinfu::Params* instance, const std::vector<int>* val) {
			instance->icpIterations = *val;
	}

	// cv::colored_kinfu::Params::truncateThreshold() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:168
	// ("cv::colored_kinfu::Params::truncateThreshold", vec![(pred!(const, [], []), _)]),
	float cv_colored_kinfu_Params_propTruncateThreshold_const(const cv::colored_kinfu::Params* instance) {
			float ret = instance->truncateThreshold;
			return ret;
	}

	// cv::colored_kinfu::Params::setTruncateThreshold(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/colored_kinfu.hpp:168
	// ("cv::colored_kinfu::Params::setTruncateThreshold", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_colored_kinfu_Params_propTruncateThreshold_const_float(cv::colored_kinfu::Params* instance, const float val) {
			instance->truncateThreshold = val;
	}

	// cv::colored_kinfu::Params::delete() generated
	// ("cv::colored_kinfu::Params::delete", vec![(pred!(mut, [], []), _)]),
	void cv_colored_kinfu_Params_delete(cv::colored_kinfu::Params* instance) {
			delete instance;
	}

	// create(const Ptr<kinfu::Params> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/dynafu.hpp:49
	// ("cv::dynafu::DynaFu::create", vec![(pred!(mut, ["_params"], ["const cv::Ptr<cv::kinfu::Params>*"]), _)]),
	void cv_dynafu_DynaFu_create_const_PtrLParamsGR(const cv::Ptr<cv::kinfu::Params>* _params, Result<cv::Ptr<cv::dynafu::DynaFu>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dynafu::DynaFu> ret = cv::dynafu::DynaFu::create(*_params);
			Ok(new cv::Ptr<cv::dynafu::DynaFu>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getParams()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/dynafu.hpp:53
	// ("cv::dynafu::DynaFu::getParams", vec![(pred!(const, [], []), _)]),
	void cv_dynafu_DynaFu_getParams_const(const cv::dynafu::DynaFu* instance, Result<cv::kinfu::Params*>* ocvrs_return) {
		try {
			const cv::kinfu::Params ret = instance->getParams();
			Ok(new const cv::kinfu::Params(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// render(OutputArray, const Matx44f &)(OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/dynafu.hpp:65
	// ("cv::dynafu::DynaFu::render", vec![(pred!(const, ["image", "cameraPose"], ["const cv::_OutputArray*", "const cv::Matx44f*"]), _)]),
	void cv_dynafu_DynaFu_render_const_const__OutputArrayR_const_Matx44fR(const cv::dynafu::DynaFu* instance, const cv::_OutputArray* image, const cv::Matx44f* cameraPose, ResultVoid* ocvrs_return) {
		try {
			instance->render(*image, *cameraPose);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dynafu::DynaFu::render(OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/dynafu.hpp:65
	// ("cv::dynafu::DynaFu::render", vec![(pred!(const, ["image"], ["const cv::_OutputArray*"]), _)]),
	void cv_dynafu_DynaFu_render_const_const__OutputArrayR(const cv::dynafu::DynaFu* instance, const cv::_OutputArray* image, ResultVoid* ocvrs_return) {
		try {
			instance->render(*image);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getCloud(OutputArray, OutputArray)(OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/dynafu.hpp:75
	// ("cv::dynafu::DynaFu::getCloud", vec![(pred!(const, ["points", "normals"], ["const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_dynafu_DynaFu_getCloud_const_const__OutputArrayR_const__OutputArrayR(const cv::dynafu::DynaFu* instance, const cv::_OutputArray* points, const cv::_OutputArray* normals, ResultVoid* ocvrs_return) {
		try {
			instance->getCloud(*points, *normals);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPoints(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/dynafu.hpp:83
	// ("cv::dynafu::DynaFu::getPoints", vec![(pred!(const, ["points"], ["const cv::_OutputArray*"]), _)]),
	void cv_dynafu_DynaFu_getPoints_const_const__OutputArrayR(const cv::dynafu::DynaFu* instance, const cv::_OutputArray* points, ResultVoid* ocvrs_return) {
		try {
			instance->getPoints(*points);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNormals(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/dynafu.hpp:89
	// ("cv::dynafu::DynaFu::getNormals", vec![(pred!(const, ["points", "normals"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_dynafu_DynaFu_getNormals_const_const__InputArrayR_const__OutputArrayR(const cv::dynafu::DynaFu* instance, const cv::_InputArray* points, const cv::_OutputArray* normals, ResultVoid* ocvrs_return) {
		try {
			instance->getNormals(*points, *normals);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// reset()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/dynafu.hpp:95
	// ("cv::dynafu::DynaFu::reset", vec![(pred!(mut, [], []), _)]),
	void cv_dynafu_DynaFu_reset(cv::dynafu::DynaFu* instance, ResultVoid* ocvrs_return) {
		try {
			instance->reset();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPose()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/dynafu.hpp:98
	// ("cv::dynafu::DynaFu::getPose", vec![(pred!(const, [], []), _)]),
	void cv_dynafu_DynaFu_getPose_const(const cv::dynafu::DynaFu* instance, Result<cv::Affine3f>* ocvrs_return) {
		try {
			cv::Affine3f ret = instance->getPose();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// update(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/dynafu.hpp:108
	// ("cv::dynafu::DynaFu::update", vec![(pred!(mut, ["depth"], ["const cv::_InputArray*"]), _)]),
	void cv_dynafu_DynaFu_update_const__InputArrayR(cv::dynafu::DynaFu* instance, const cv::_InputArray* depth, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->update(*depth);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNodesPos()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/dynafu.hpp:110
	// ("cv::dynafu::DynaFu::getNodesPos", vec![(pred!(const, [], []), _)]),
	void cv_dynafu_DynaFu_getNodesPos_const(const cv::dynafu::DynaFu* instance, Result<std::vector<cv::Point3f>*>* ocvrs_return) {
		try {
			std::vector<cv::Point3f> ret = instance->getNodesPos();
			Ok(new std::vector<cv::Point3f>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// marchCubes(OutputArray, OutputArray)(OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/dynafu.hpp:112
	// ("cv::dynafu::DynaFu::marchCubes", vec![(pred!(const, ["vertices", "edges"], ["const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_dynafu_DynaFu_marchCubes_const_const__OutputArrayR_const__OutputArrayR(const cv::dynafu::DynaFu* instance, const cv::_OutputArray* vertices, const cv::_OutputArray* edges, ResultVoid* ocvrs_return) {
		try {
			instance->marchCubes(*vertices, *edges);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// renderSurface(OutputArray, OutputArray, OutputArray, bool)(OutputArray, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/dynafu.hpp:114
	// ("cv::dynafu::DynaFu::renderSurface", vec![(pred!(mut, ["depthImage", "vertImage", "normImage", "warp"], ["const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "bool"]), _)]),
	void cv_dynafu_DynaFu_renderSurface_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_bool(cv::dynafu::DynaFu* instance, const cv::_OutputArray* depthImage, const cv::_OutputArray* vertImage, const cv::_OutputArray* normImage, bool warp, ResultVoid* ocvrs_return) {
		try {
			instance->renderSurface(*depthImage, *vertImage, *normImage, warp);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dynafu::DynaFu::renderSurface(OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/dynafu.hpp:114
	// ("cv::dynafu::DynaFu::renderSurface", vec![(pred!(mut, ["depthImage", "vertImage", "normImage"], ["const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_dynafu_DynaFu_renderSurface_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(cv::dynafu::DynaFu* instance, const cv::_OutputArray* depthImage, const cv::_OutputArray* vertImage, const cv::_OutputArray* normImage, ResultVoid* ocvrs_return) {
		try {
			instance->renderSurface(*depthImage, *vertImage, *normImage);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dynafu::DynaFu::delete() generated
	// ("cv::dynafu::DynaFu::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dynafu_DynaFu_delete(cv::dynafu::DynaFu* instance) {
			delete instance;
	}

	// Intr()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/intrinsics.hpp:61
	// ("cv::kinfu::Intr::Intr", vec![(pred!(mut, [], []), _)]),
	void cv_kinfu_Intr_Intr(Result<cv::kinfu::Intr>* ocvrs_return) {
		try {
			cv::kinfu::Intr ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Intr(float, float, float, float)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/intrinsics.hpp:62
	// ("cv::kinfu::Intr::Intr", vec![(pred!(mut, ["_fx", "_fy", "_cx", "_cy"], ["float", "float", "float", "float"]), _)]),
	void cv_kinfu_Intr_Intr_float_float_float_float(float _fx, float _fy, float _cx, float _cy, Result<cv::kinfu::Intr>* ocvrs_return) {
		try {
			cv::kinfu::Intr ret(_fx, _fy, _cx, _cy);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Intr(cv::Matx33f)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/intrinsics.hpp:63
	// ("cv::kinfu::Intr::Intr", vec![(pred!(mut, ["m"], ["cv::Matx33f"]), _)]),
	void cv_kinfu_Intr_Intr_Matx33f(cv::Matx33f* m, Result<cv::kinfu::Intr>* ocvrs_return) {
		try {
			cv::kinfu::Intr ret(*m);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// scale(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/intrinsics.hpp:65
	// ("cv::kinfu::Intr::scale", vec![(pred!(const, ["pyr"], ["int"]), _)]),
	void cv_kinfu_Intr_scale_const_int(const cv::kinfu::Intr* instance, int pyr, Result<cv::kinfu::Intr>* ocvrs_return) {
		try {
			cv::kinfu::Intr ret = instance->scale(pyr);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// makeReprojector()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/intrinsics.hpp:70
	// ("cv::kinfu::Intr::makeReprojector", vec![(pred!(const, [], []), _)]),
	void cv_kinfu_Intr_makeReprojector_const(const cv::kinfu::Intr* instance, Result<cv::kinfu::Intr::Reprojector>* ocvrs_return) {
		try {
			cv::kinfu::Intr::Reprojector ret = instance->makeReprojector();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// makeProjector()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/intrinsics.hpp:71
	// ("cv::kinfu::Intr::makeProjector", vec![(pred!(const, [], []), _)]),
	void cv_kinfu_Intr_makeProjector_const(const cv::kinfu::Intr* instance, Result<cv::kinfu::Intr::Projector>* ocvrs_return) {
		try {
			cv::kinfu::Intr::Projector ret = instance->makeProjector();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMat()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/intrinsics.hpp:73
	// ("cv::kinfu::Intr::getMat", vec![(pred!(const, [], []), _)]),
	void cv_kinfu_Intr_getMat_const(const cv::kinfu::Intr* instance, Result<cv::Matx33f>* ocvrs_return) {
		try {
			cv::Matx33f ret = instance->getMat();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Projector(Intr)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/intrinsics.hpp:41
	// ("cv::kinfu::Intr::Projector::Projector", vec![(pred!(mut, ["intr"], ["cv::kinfu::Intr"]), _)]),
	void cv_kinfu_Intr_Projector_Projector_Intr(cv::kinfu::Intr* intr, Result<cv::kinfu::Intr::Projector>* ocvrs_return) {
		try {
			cv::kinfu::Intr::Projector ret(*intr);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Reprojector()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/intrinsics.hpp:21
	// ("cv::kinfu::Intr::Reprojector::Reprojector", vec![(pred!(mut, [], []), _)]),
	void cv_kinfu_Intr_Reprojector_Reprojector(Result<cv::kinfu::Intr::Reprojector>* ocvrs_return) {
		try {
			cv::kinfu::Intr::Reprojector ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Reprojector(Intr)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/intrinsics.hpp:22
	// ("cv::kinfu::Intr::Reprojector::Reprojector", vec![(pred!(mut, ["intr"], ["cv::kinfu::Intr"]), _)]),
	void cv_kinfu_Intr_Reprojector_Reprojector_Intr(cv::kinfu::Intr* intr, Result<cv::kinfu::Intr::Reprojector>* ocvrs_return) {
		try {
			cv::kinfu::Intr::Reprojector ret(*intr);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const Ptr<Params> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:195
	// ("cv::kinfu::KinFu::create", vec![(pred!(mut, ["_params"], ["const cv::Ptr<cv::kinfu::Params>*"]), _)]),
	void cv_kinfu_KinFu_create_const_PtrLParamsGR(const cv::Ptr<cv::kinfu::Params>* _params, Result<cv::Ptr<cv::kinfu::KinFu>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::kinfu::KinFu> ret = cv::kinfu::KinFu::create(*_params);
			Ok(new cv::Ptr<cv::kinfu::KinFu>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getParams()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:199
	// ("cv::kinfu::KinFu::getParams", vec![(pred!(const, [], []), _)]),
	void cv_kinfu_KinFu_getParams_const(const cv::kinfu::KinFu* instance, Result<cv::kinfu::Params*>* ocvrs_return) {
		try {
			const cv::kinfu::Params ret = instance->getParams();
			Ok(new const cv::kinfu::Params(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// render(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:209
	// ("cv::kinfu::KinFu::render", vec![(pred!(const, ["image"], ["const cv::_OutputArray*"]), _)]),
	void cv_kinfu_KinFu_render_const_const__OutputArrayR(const cv::kinfu::KinFu* instance, const cv::_OutputArray* image, ResultVoid* ocvrs_return) {
		try {
			instance->render(*image);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// render(OutputArray, const Matx44f &)(OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:221
	// ("cv::kinfu::KinFu::render", vec![(pred!(const, ["image", "cameraPose"], ["const cv::_OutputArray*", "const cv::Matx44f*"]), _)]),
	void cv_kinfu_KinFu_render_const_const__OutputArrayR_const_Matx44fR(const cv::kinfu::KinFu* instance, const cv::_OutputArray* image, const cv::Matx44f* cameraPose, ResultVoid* ocvrs_return) {
		try {
			instance->render(*image, *cameraPose);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getCloud(OutputArray, OutputArray)(OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:231
	// ("cv::kinfu::KinFu::getCloud", vec![(pred!(const, ["points", "normals"], ["const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_kinfu_KinFu_getCloud_const_const__OutputArrayR_const__OutputArrayR(const cv::kinfu::KinFu* instance, const cv::_OutputArray* points, const cv::_OutputArray* normals, ResultVoid* ocvrs_return) {
		try {
			instance->getCloud(*points, *normals);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPoints(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:239
	// ("cv::kinfu::KinFu::getPoints", vec![(pred!(const, ["points"], ["const cv::_OutputArray*"]), _)]),
	void cv_kinfu_KinFu_getPoints_const_const__OutputArrayR(const cv::kinfu::KinFu* instance, const cv::_OutputArray* points, ResultVoid* ocvrs_return) {
		try {
			instance->getPoints(*points);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNormals(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:245
	// ("cv::kinfu::KinFu::getNormals", vec![(pred!(const, ["points", "normals"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_kinfu_KinFu_getNormals_const_const__InputArrayR_const__OutputArrayR(const cv::kinfu::KinFu* instance, const cv::_InputArray* points, const cv::_OutputArray* normals, ResultVoid* ocvrs_return) {
		try {
			instance->getNormals(*points, *normals);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// reset()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:251
	// ("cv::kinfu::KinFu::reset", vec![(pred!(mut, [], []), _)]),
	void cv_kinfu_KinFu_reset(cv::kinfu::KinFu* instance, ResultVoid* ocvrs_return) {
		try {
			instance->reset();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPose()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:254
	// ("cv::kinfu::KinFu::getPose", vec![(pred!(const, [], []), _)]),
	void cv_kinfu_KinFu_getPose_const(const cv::kinfu::KinFu* instance, Result<cv::Affine3f>* ocvrs_return) {
		try {
			cv::Affine3f ret = instance->getPose();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// update(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:264
	// ("cv::kinfu::KinFu::update", vec![(pred!(mut, ["depth"], ["const cv::_InputArray*"]), _)]),
	void cv_kinfu_KinFu_update_const__InputArrayR(cv::kinfu::KinFu* instance, const cv::_InputArray* depth, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->update(*depth);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::kinfu::KinFu::delete() generated
	// ("cv::kinfu::KinFu::delete", vec![(pred!(mut, [], []), _)]),
	void cv_kinfu_KinFu_delete(cv::kinfu::KinFu* instance) {
			delete instance;
	}

	// Params()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:22
	// ("cv::kinfu::Params::Params", vec![(pred!(mut, [], []), _)]),
	void cv_kinfu_Params_Params(Result<cv::kinfu::Params*>* ocvrs_return) {
		try {
			cv::kinfu::Params* ret = new cv::kinfu::Params();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Params(Matx33f, Vec3f)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:30
	// ("cv::kinfu::Params::Params", vec![(pred!(mut, ["volumeInitialPoseRot", "volumeInitialPoseTransl"], ["cv::Matx33f", "cv::Vec3f"]), _)]),
	void cv_kinfu_Params_Params_Matx33f_Vec3f(cv::Matx33f* volumeInitialPoseRot, cv::Vec3f* volumeInitialPoseTransl, Result<cv::kinfu::Params*>* ocvrs_return) {
		try {
			cv::kinfu::Params* ret = new cv::kinfu::Params(*volumeInitialPoseRot, *volumeInitialPoseTransl);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Params(Matx44f)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:40
	// ("cv::kinfu::Params::Params", vec![(pred!(mut, ["volumeInitialPose"], ["cv::Matx44f"]), _)]),
	void cv_kinfu_Params_Params_Matx44f(cv::Matx44f* volumeInitialPose, Result<cv::kinfu::Params*>* ocvrs_return) {
		try {
			cv::kinfu::Params* ret = new cv::kinfu::Params(*volumeInitialPose);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setInitialVolumePose(Matx33f, Vec3f)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:51
	// ("cv::kinfu::Params::setInitialVolumePose", vec![(pred!(mut, ["R", "t"], ["cv::Matx33f", "cv::Vec3f"]), _)]),
	void cv_kinfu_Params_setInitialVolumePose_Matx33f_Vec3f(cv::kinfu::Params* instance, cv::Matx33f* R, cv::Vec3f* t, ResultVoid* ocvrs_return) {
		try {
			instance->setInitialVolumePose(*R, *t);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setInitialVolumePose(Matx44f)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:58
	// ("cv::kinfu::Params::setInitialVolumePose", vec![(pred!(mut, ["homogen_tf"], ["cv::Matx44f"]), _)]),
	void cv_kinfu_Params_setInitialVolumePose_Matx44f(cv::kinfu::Params* instance, cv::Matx44f* homogen_tf, ResultVoid* ocvrs_return) {
		try {
			instance->setInitialVolumePose(*homogen_tf);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// defaultParams()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:64
	// ("cv::kinfu::Params::defaultParams", vec![(pred!(mut, [], []), _)]),
	void cv_kinfu_Params_defaultParams(Result<cv::Ptr<cv::kinfu::Params>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::kinfu::Params> ret = cv::kinfu::Params::defaultParams();
			Ok(new cv::Ptr<cv::kinfu::Params>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// coarseParams()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:70
	// ("cv::kinfu::Params::coarseParams", vec![(pred!(mut, [], []), _)]),
	void cv_kinfu_Params_coarseParams(Result<cv::Ptr<cv::kinfu::Params>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::kinfu::Params> ret = cv::kinfu::Params::coarseParams();
			Ok(new cv::Ptr<cv::kinfu::Params>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// hashTSDFParams(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:75
	// ("cv::kinfu::Params::hashTSDFParams", vec![(pred!(mut, ["isCoarse"], ["bool"]), _)]),
	void cv_kinfu_Params_hashTSDFParams_bool(bool isCoarse, Result<cv::Ptr<cv::kinfu::Params>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::kinfu::Params> ret = cv::kinfu::Params::hashTSDFParams(isCoarse);
			Ok(new cv::Ptr<cv::kinfu::Params>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// coloredTSDFParams(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:80
	// ("cv::kinfu::Params::coloredTSDFParams", vec![(pred!(mut, ["isCoarse"], ["bool"]), _)]),
	void cv_kinfu_Params_coloredTSDFParams_bool(bool isCoarse, Result<cv::Ptr<cv::kinfu::Params>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::kinfu::Params> ret = cv::kinfu::Params::coloredTSDFParams(isCoarse);
			Ok(new cv::Ptr<cv::kinfu::Params>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::kinfu::Params::frameSize() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:83
	// ("cv::kinfu::Params::frameSize", vec![(pred!(const, [], []), _)]),
	void cv_kinfu_Params_propFrameSize_const(const cv::kinfu::Params* instance, cv::Size* ocvrs_return) {
			cv::Size ret = instance->frameSize;
			*ocvrs_return = ret;
	}

	// cv::kinfu::Params::setFrameSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:83
	// ("cv::kinfu::Params::setFrameSize", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	void cv_kinfu_Params_propFrameSize_const_Size(cv::kinfu::Params* instance, const cv::Size* val) {
			instance->frameSize = *val;
	}

	// cv::kinfu::Params::volumeType() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:86
	// ("cv::kinfu::Params::volumeType", vec![(pred!(const, [], []), _)]),
	void cv_kinfu_Params_propVolumeType_const(const cv::kinfu::Params* instance, cv::kinfu::VolumeType* ocvrs_return) {
			cv::kinfu::VolumeType ret = instance->volumeType;
			*ocvrs_return = ret;
	}

	// cv::kinfu::Params::setVolumeType(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:86
	// ("cv::kinfu::Params::setVolumeType", vec![(pred!(mut, ["val"], ["const cv::kinfu::VolumeType"]), _)]),
	void cv_kinfu_Params_propVolumeType_const_VolumeType(cv::kinfu::Params* instance, const cv::kinfu::VolumeType val) {
			instance->volumeType = val;
	}

	// cv::kinfu::Params::intr() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:89
	// ("cv::kinfu::Params::intr", vec![(pred!(const, [], []), _)]),
	void cv_kinfu_Params_propIntr_const(const cv::kinfu::Params* instance, cv::Matx33f* ocvrs_return) {
			cv::Matx33f ret = instance->intr;
			*ocvrs_return = ret;
	}

	// cv::kinfu::Params::setIntr(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:89
	// ("cv::kinfu::Params::setIntr", vec![(pred!(mut, ["val"], ["const cv::Matx33f"]), _)]),
	void cv_kinfu_Params_propIntr_const_Matx33f(cv::kinfu::Params* instance, const cv::Matx33f* val) {
			instance->intr = *val;
	}

	// cv::kinfu::Params::rgb_intr() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:92
	// ("cv::kinfu::Params::rgb_intr", vec![(pred!(const, [], []), _)]),
	void cv_kinfu_Params_propRgb_intr_const(const cv::kinfu::Params* instance, cv::Matx33f* ocvrs_return) {
			cv::Matx33f ret = instance->rgb_intr;
			*ocvrs_return = ret;
	}

	// cv::kinfu::Params::setRgb_intr(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:92
	// ("cv::kinfu::Params::setRgb_intr", vec![(pred!(mut, ["val"], ["const cv::Matx33f"]), _)]),
	void cv_kinfu_Params_propRgb_intr_const_Matx33f(cv::kinfu::Params* instance, const cv::Matx33f* val) {
			instance->rgb_intr = *val;
	}

	// cv::kinfu::Params::depthFactor() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:100
	// ("cv::kinfu::Params::depthFactor", vec![(pred!(const, [], []), _)]),
	float cv_kinfu_Params_propDepthFactor_const(const cv::kinfu::Params* instance) {
			float ret = instance->depthFactor;
			return ret;
	}

	// cv::kinfu::Params::setDepthFactor(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:100
	// ("cv::kinfu::Params::setDepthFactor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_kinfu_Params_propDepthFactor_const_float(cv::kinfu::Params* instance, const float val) {
			instance->depthFactor = val;
	}

	// cv::kinfu::Params::bilateral_sigma_depth() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:103
	// ("cv::kinfu::Params::bilateral_sigma_depth", vec![(pred!(const, [], []), _)]),
	float cv_kinfu_Params_propBilateral_sigma_depth_const(const cv::kinfu::Params* instance) {
			float ret = instance->bilateral_sigma_depth;
			return ret;
	}

	// cv::kinfu::Params::setBilateral_sigma_depth(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:103
	// ("cv::kinfu::Params::setBilateral_sigma_depth", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_kinfu_Params_propBilateral_sigma_depth_const_float(cv::kinfu::Params* instance, const float val) {
			instance->bilateral_sigma_depth = val;
	}

	// cv::kinfu::Params::bilateral_sigma_spatial() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:105
	// ("cv::kinfu::Params::bilateral_sigma_spatial", vec![(pred!(const, [], []), _)]),
	float cv_kinfu_Params_propBilateral_sigma_spatial_const(const cv::kinfu::Params* instance) {
			float ret = instance->bilateral_sigma_spatial;
			return ret;
	}

	// cv::kinfu::Params::setBilateral_sigma_spatial(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:105
	// ("cv::kinfu::Params::setBilateral_sigma_spatial", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_kinfu_Params_propBilateral_sigma_spatial_const_float(cv::kinfu::Params* instance, const float val) {
			instance->bilateral_sigma_spatial = val;
	}

	// cv::kinfu::Params::bilateral_kernel_size() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:107
	// ("cv::kinfu::Params::bilateral_kernel_size", vec![(pred!(const, [], []), _)]),
	int cv_kinfu_Params_propBilateral_kernel_size_const(const cv::kinfu::Params* instance) {
			int ret = instance->bilateral_kernel_size;
			return ret;
	}

	// cv::kinfu::Params::setBilateral_kernel_size(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:107
	// ("cv::kinfu::Params::setBilateral_kernel_size", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_kinfu_Params_propBilateral_kernel_size_const_int(cv::kinfu::Params* instance, const int val) {
			instance->bilateral_kernel_size = val;
	}

	// cv::kinfu::Params::pyramidLevels() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:110
	// ("cv::kinfu::Params::pyramidLevels", vec![(pred!(const, [], []), _)]),
	int cv_kinfu_Params_propPyramidLevels_const(const cv::kinfu::Params* instance) {
			int ret = instance->pyramidLevels;
			return ret;
	}

	// cv::kinfu::Params::setPyramidLevels(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:110
	// ("cv::kinfu::Params::setPyramidLevels", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_kinfu_Params_propPyramidLevels_const_int(cv::kinfu::Params* instance, const int val) {
			instance->pyramidLevels = val;
	}

	// cv::kinfu::Params::volumeDims() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:116
	// ("cv::kinfu::Params::volumeDims", vec![(pred!(const, [], []), _)]),
	void cv_kinfu_Params_propVolumeDims_const(const cv::kinfu::Params* instance, cv::Vec3i* ocvrs_return) {
			cv::Vec3i ret = instance->volumeDims;
			*ocvrs_return = ret;
	}

	// cv::kinfu::Params::setVolumeDims(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:116
	// ("cv::kinfu::Params::setVolumeDims", vec![(pred!(mut, ["val"], ["const cv::Vec3i"]), _)]),
	void cv_kinfu_Params_propVolumeDims_const_Vec3i(cv::kinfu::Params* instance, const cv::Vec3i* val) {
			instance->volumeDims = *val;
	}

	// cv::kinfu::Params::voxelSize() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:118
	// ("cv::kinfu::Params::voxelSize", vec![(pred!(const, [], []), _)]),
	float cv_kinfu_Params_propVoxelSize_const(const cv::kinfu::Params* instance) {
			float ret = instance->voxelSize;
			return ret;
	}

	// cv::kinfu::Params::setVoxelSize(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:118
	// ("cv::kinfu::Params::setVoxelSize", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_kinfu_Params_propVoxelSize_const_float(cv::kinfu::Params* instance, const float val) {
			instance->voxelSize = val;
	}

	// cv::kinfu::Params::tsdf_min_camera_movement() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:124
	// ("cv::kinfu::Params::tsdf_min_camera_movement", vec![(pred!(const, [], []), _)]),
	float cv_kinfu_Params_propTsdf_min_camera_movement_const(const cv::kinfu::Params* instance) {
			float ret = instance->tsdf_min_camera_movement;
			return ret;
	}

	// cv::kinfu::Params::setTsdf_min_camera_movement(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:124
	// ("cv::kinfu::Params::setTsdf_min_camera_movement", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_kinfu_Params_propTsdf_min_camera_movement_const_float(cv::kinfu::Params* instance, const float val) {
			instance->tsdf_min_camera_movement = val;
	}

	// cv::kinfu::Params::volumePose() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:127
	// ("cv::kinfu::Params::volumePose", vec![(pred!(const, [], []), _)]),
	void cv_kinfu_Params_propVolumePose_const(const cv::kinfu::Params* instance, cv::Affine3f* ocvrs_return) {
			cv::Affine3f ret = instance->volumePose;
			*ocvrs_return = ret;
	}

	// cv::kinfu::Params::setVolumePose(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:127
	// ("cv::kinfu::Params::setVolumePose", vec![(pred!(mut, ["val"], ["const cv::Affine3f"]), _)]),
	void cv_kinfu_Params_propVolumePose_const_Affine3f(cv::kinfu::Params* instance, const cv::Affine3f* val) {
			instance->volumePose = *val;
	}

	// cv::kinfu::Params::tsdf_trunc_dist() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:133
	// ("cv::kinfu::Params::tsdf_trunc_dist", vec![(pred!(const, [], []), _)]),
	float cv_kinfu_Params_propTsdf_trunc_dist_const(const cv::kinfu::Params* instance) {
			float ret = instance->tsdf_trunc_dist;
			return ret;
	}

	// cv::kinfu::Params::setTsdf_trunc_dist(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:133
	// ("cv::kinfu::Params::setTsdf_trunc_dist", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_kinfu_Params_propTsdf_trunc_dist_const_float(cv::kinfu::Params* instance, const float val) {
			instance->tsdf_trunc_dist = val;
	}

	// cv::kinfu::Params::tsdf_max_weight() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:139
	// ("cv::kinfu::Params::tsdf_max_weight", vec![(pred!(const, [], []), _)]),
	int cv_kinfu_Params_propTsdf_max_weight_const(const cv::kinfu::Params* instance) {
			int ret = instance->tsdf_max_weight;
			return ret;
	}

	// cv::kinfu::Params::setTsdf_max_weight(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:139
	// ("cv::kinfu::Params::setTsdf_max_weight", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_kinfu_Params_propTsdf_max_weight_const_int(cv::kinfu::Params* instance, const int val) {
			instance->tsdf_max_weight = val;
	}

	// cv::kinfu::Params::raycast_step_factor() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:145
	// ("cv::kinfu::Params::raycast_step_factor", vec![(pred!(const, [], []), _)]),
	float cv_kinfu_Params_propRaycast_step_factor_const(const cv::kinfu::Params* instance) {
			float ret = instance->raycast_step_factor;
			return ret;
	}

	// cv::kinfu::Params::setRaycast_step_factor(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:145
	// ("cv::kinfu::Params::setRaycast_step_factor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_kinfu_Params_propRaycast_step_factor_const_float(cv::kinfu::Params* instance, const float val) {
			instance->raycast_step_factor = val;
	}

	// cv::kinfu::Params::lightPose() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:152
	// ("cv::kinfu::Params::lightPose", vec![(pred!(const, [], []), _)]),
	void cv_kinfu_Params_propLightPose_const(const cv::kinfu::Params* instance, cv::Vec3f* ocvrs_return) {
			cv::Vec3f ret = instance->lightPose;
			*ocvrs_return = ret;
	}

	// cv::kinfu::Params::setLightPose(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:152
	// ("cv::kinfu::Params::setLightPose", vec![(pred!(mut, ["val"], ["const cv::Vec3f"]), _)]),
	void cv_kinfu_Params_propLightPose_const_Vec3f(cv::kinfu::Params* instance, const cv::Vec3f* val) {
			instance->lightPose = *val;
	}

	// cv::kinfu::Params::icpDistThresh() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:155
	// ("cv::kinfu::Params::icpDistThresh", vec![(pred!(const, [], []), _)]),
	float cv_kinfu_Params_propIcpDistThresh_const(const cv::kinfu::Params* instance) {
			float ret = instance->icpDistThresh;
			return ret;
	}

	// cv::kinfu::Params::setIcpDistThresh(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:155
	// ("cv::kinfu::Params::setIcpDistThresh", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_kinfu_Params_propIcpDistThresh_const_float(cv::kinfu::Params* instance, const float val) {
			instance->icpDistThresh = val;
	}

	// cv::kinfu::Params::icpAngleThresh() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:157
	// ("cv::kinfu::Params::icpAngleThresh", vec![(pred!(const, [], []), _)]),
	float cv_kinfu_Params_propIcpAngleThresh_const(const cv::kinfu::Params* instance) {
			float ret = instance->icpAngleThresh;
			return ret;
	}

	// cv::kinfu::Params::setIcpAngleThresh(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:157
	// ("cv::kinfu::Params::setIcpAngleThresh", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_kinfu_Params_propIcpAngleThresh_const_float(cv::kinfu::Params* instance, const float val) {
			instance->icpAngleThresh = val;
	}

	// cv::kinfu::Params::icpIterations() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:159
	// ("cv::kinfu::Params::icpIterations", vec![(pred!(const, [], []), _)]),
	std::vector<int>* cv_kinfu_Params_propIcpIterations_const(const cv::kinfu::Params* instance) {
			std::vector<int> ret = instance->icpIterations;
			return new std::vector<int>(ret);
	}

	// cv::kinfu::Params::setIcpIterations(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:159
	// ("cv::kinfu::Params::setIcpIterations", vec![(pred!(mut, ["val"], ["const std::vector<int>"]), _)]),
	void cv_kinfu_Params_propIcpIterations_const_vectorLintG(cv::kinfu::Params* instance, const std::vector<int>* val) {
			instance->icpIterations = *val;
	}

	// cv::kinfu::Params::truncateThreshold() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:165
	// ("cv::kinfu::Params::truncateThreshold", vec![(pred!(const, [], []), _)]),
	float cv_kinfu_Params_propTruncateThreshold_const(const cv::kinfu::Params* instance) {
			float ret = instance->truncateThreshold;
			return ret;
	}

	// cv::kinfu::Params::setTruncateThreshold(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/kinfu.hpp:165
	// ("cv::kinfu::Params::setTruncateThreshold", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_kinfu_Params_propTruncateThreshold_const_float(cv::kinfu::Params* instance, const float val) {
			instance->truncateThreshold = val;
	}

	// cv::kinfu::Params::delete() generated
	// ("cv::kinfu::Params::delete", vec![(pred!(mut, [], []), _)]),
	void cv_kinfu_Params_delete(cv::kinfu::Params* instance) {
			delete instance;
	}

	// integrate(InputArray, float, const Matx44f &, const kinfu::Intr &, const int)(InputArray, Primitive, SimpleClass, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:31
	// ("cv::kinfu::Volume::integrate", vec![(pred!(mut, ["_depth", "depthFactor", "cameraPose", "intrinsics", "frameId"], ["const cv::_InputArray*", "float", "const cv::Matx44f*", "const cv::kinfu::Intr*", "const int"]), _)]),
	void cv_kinfu_Volume_integrate_const__InputArrayR_float_const_Matx44fR_const_IntrR_const_int(cv::kinfu::Volume* instance, const cv::_InputArray* _depth, float depthFactor, const cv::Matx44f* cameraPose, const cv::kinfu::Intr* intrinsics, const int frameId, ResultVoid* ocvrs_return) {
		try {
			instance->integrate(*_depth, depthFactor, *cameraPose, *intrinsics, frameId);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::kinfu::Volume::integrate(InputArray, Primitive, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:31
	// ("cv::kinfu::Volume::integrate", vec![(pred!(mut, ["_depth", "depthFactor", "cameraPose", "intrinsics"], ["const cv::_InputArray*", "float", "const cv::Matx44f*", "const cv::kinfu::Intr*"]), _)]),
	void cv_kinfu_Volume_integrate_const__InputArrayR_float_const_Matx44fR_const_IntrR(cv::kinfu::Volume* instance, const cv::_InputArray* _depth, float depthFactor, const cv::Matx44f* cameraPose, const cv::kinfu::Intr* intrinsics, ResultVoid* ocvrs_return) {
		try {
			instance->integrate(*_depth, depthFactor, *cameraPose, *intrinsics);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// integrate(InputArray, InputArray, float, const Matx44f &, const kinfu::Intr &, const Intr &, const int)(InputArray, InputArray, Primitive, SimpleClass, SimpleClass, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:33
	// ("cv::kinfu::Volume::integrate", vec![(pred!(mut, ["_depth", "_rgb", "depthFactor", "cameraPose", "intrinsics", "rgb_intrinsics", "frameId"], ["const cv::_InputArray*", "const cv::_InputArray*", "float", "const cv::Matx44f*", "const cv::kinfu::Intr*", "const cv::kinfu::Intr*", "const int"]), _)]),
	void cv_kinfu_Volume_integrate_const__InputArrayR_const__InputArrayR_float_const_Matx44fR_const_IntrR_const_IntrR_const_int(cv::kinfu::Volume* instance, const cv::_InputArray* _depth, const cv::_InputArray* _rgb, float depthFactor, const cv::Matx44f* cameraPose, const cv::kinfu::Intr* intrinsics, const cv::kinfu::Intr* rgb_intrinsics, const int frameId, ResultVoid* ocvrs_return) {
		try {
			instance->integrate(*_depth, *_rgb, depthFactor, *cameraPose, *intrinsics, *rgb_intrinsics, frameId);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::kinfu::Volume::integrate(InputArray, InputArray, Primitive, SimpleClass, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:33
	// ("cv::kinfu::Volume::integrate", vec![(pred!(mut, ["_depth", "_rgb", "depthFactor", "cameraPose", "intrinsics", "rgb_intrinsics"], ["const cv::_InputArray*", "const cv::_InputArray*", "float", "const cv::Matx44f*", "const cv::kinfu::Intr*", "const cv::kinfu::Intr*"]), _)]),
	void cv_kinfu_Volume_integrate_const__InputArrayR_const__InputArrayR_float_const_Matx44fR_const_IntrR_const_IntrR(cv::kinfu::Volume* instance, const cv::_InputArray* _depth, const cv::_InputArray* _rgb, float depthFactor, const cv::Matx44f* cameraPose, const cv::kinfu::Intr* intrinsics, const cv::kinfu::Intr* rgb_intrinsics, ResultVoid* ocvrs_return) {
		try {
			instance->integrate(*_depth, *_rgb, depthFactor, *cameraPose, *intrinsics, *rgb_intrinsics);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// raycast(const Matx44f &, const kinfu::Intr &, const Size &, OutputArray, OutputArray)(SimpleClass, SimpleClass, SimpleClass, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:36
	// ("cv::kinfu::Volume::raycast", vec![(pred!(const, ["cameraPose", "intrinsics", "frameSize", "points", "normals"], ["const cv::Matx44f*", "const cv::kinfu::Intr*", "const cv::Size*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_kinfu_Volume_raycast_const_const_Matx44fR_const_IntrR_const_SizeR_const__OutputArrayR_const__OutputArrayR(const cv::kinfu::Volume* instance, const cv::Matx44f* cameraPose, const cv::kinfu::Intr* intrinsics, const cv::Size* frameSize, const cv::_OutputArray* points, const cv::_OutputArray* normals, ResultVoid* ocvrs_return) {
		try {
			instance->raycast(*cameraPose, *intrinsics, *frameSize, *points, *normals);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// raycast(const Matx44f &, const kinfu::Intr &, const Size &, OutputArray, OutputArray, OutputArray)(SimpleClass, SimpleClass, SimpleClass, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:38
	// ("cv::kinfu::Volume::raycast", vec![(pred!(const, ["cameraPose", "intrinsics", "frameSize", "points", "normals", "colors"], ["const cv::Matx44f*", "const cv::kinfu::Intr*", "const cv::Size*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_kinfu_Volume_raycast_const_const_Matx44fR_const_IntrR_const_SizeR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::kinfu::Volume* instance, const cv::Matx44f* cameraPose, const cv::kinfu::Intr* intrinsics, const cv::Size* frameSize, const cv::_OutputArray* points, const cv::_OutputArray* normals, const cv::_OutputArray* colors, ResultVoid* ocvrs_return) {
		try {
			instance->raycast(*cameraPose, *intrinsics, *frameSize, *points, *normals, *colors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// fetchNormals(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:40
	// ("cv::kinfu::Volume::fetchNormals", vec![(pred!(const, ["points", "_normals"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_kinfu_Volume_fetchNormals_const_const__InputArrayR_const__OutputArrayR(const cv::kinfu::Volume* instance, const cv::_InputArray* points, const cv::_OutputArray* _normals, ResultVoid* ocvrs_return) {
		try {
			instance->fetchNormals(*points, *_normals);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// fetchPointsNormals(OutputArray, OutputArray)(OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:41
	// ("cv::kinfu::Volume::fetchPointsNormals", vec![(pred!(const, ["points", "normals"], ["const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_kinfu_Volume_fetchPointsNormals_const_const__OutputArrayR_const__OutputArrayR(const cv::kinfu::Volume* instance, const cv::_OutputArray* points, const cv::_OutputArray* normals, ResultVoid* ocvrs_return) {
		try {
			instance->fetchPointsNormals(*points, *normals);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// fetchPointsNormalsColors(OutputArray, OutputArray, OutputArray)(OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:42
	// ("cv::kinfu::Volume::fetchPointsNormalsColors", vec![(pred!(const, ["unnamed", "unnamed", "unnamed"], ["const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_kinfu_Volume_fetchPointsNormalsColors_const_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::kinfu::Volume* instance, const cv::_OutputArray* unnamed, const cv::_OutputArray* unnamed_1, const cv::_OutputArray* unnamed_2, ResultVoid* ocvrs_return) {
		try {
			instance->fetchPointsNormalsColors(*unnamed, *unnamed_1, *unnamed_2);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// reset()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:46
	// ("cv::kinfu::Volume::reset", vec![(pred!(mut, [], []), _)]),
	void cv_kinfu_Volume_reset(cv::kinfu::Volume* instance, ResultVoid* ocvrs_return) {
		try {
			instance->reset();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::kinfu::Volume::voxelSize() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:49
	// ("cv::kinfu::Volume::voxelSize", vec![(pred!(const, [], []), _)]),
	const float cv_kinfu_Volume_propVoxelSize_const(const cv::kinfu::Volume* instance) {
			const float ret = instance->voxelSize;
			return ret;
	}

	// cv::kinfu::Volume::voxelSizeInv() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:50
	// ("cv::kinfu::Volume::voxelSizeInv", vec![(pred!(const, [], []), _)]),
	const float cv_kinfu_Volume_propVoxelSizeInv_const(const cv::kinfu::Volume* instance) {
			const float ret = instance->voxelSizeInv;
			return ret;
	}

	// cv::kinfu::Volume::pose() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:51
	// ("cv::kinfu::Volume::pose", vec![(pred!(const, [], []), _)]),
	void cv_kinfu_Volume_propPose_const(const cv::kinfu::Volume* instance, cv::Affine3f* ocvrs_return) {
			const cv::Affine3f ret = instance->pose;
			*ocvrs_return = ret;
	}

	// cv::kinfu::Volume::raycastStepFactor() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:52
	// ("cv::kinfu::Volume::raycastStepFactor", vec![(pred!(const, [], []), _)]),
	const float cv_kinfu_Volume_propRaycastStepFactor_const(const cv::kinfu::Volume* instance) {
			const float ret = instance->raycastStepFactor;
			return ret;
	}

	// cv::kinfu::Volume::delete() generated
	// ("cv::kinfu::Volume::delete", vec![(pred!(mut, [], []), _)]),
	void cv_kinfu_Volume_delete(cv::kinfu::Volume* instance) {
			delete instance;
	}

	// defaultParams(VolumeType)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:112
	// ("cv::kinfu::VolumeParams::defaultParams", vec![(pred!(mut, ["_volumeType"], ["cv::kinfu::VolumeType"]), _)]),
	void cv_kinfu_VolumeParams_defaultParams_VolumeType(cv::kinfu::VolumeType _volumeType, Result<cv::Ptr<cv::kinfu::VolumeParams>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::kinfu::VolumeParams> ret = cv::kinfu::VolumeParams::defaultParams(_volumeType);
			Ok(new cv::Ptr<cv::kinfu::VolumeParams>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// coarseParams(VolumeType)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:117
	// ("cv::kinfu::VolumeParams::coarseParams", vec![(pred!(mut, ["_volumeType"], ["cv::kinfu::VolumeType"]), _)]),
	void cv_kinfu_VolumeParams_coarseParams_VolumeType(cv::kinfu::VolumeType _volumeType, Result<cv::Ptr<cv::kinfu::VolumeParams>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::kinfu::VolumeParams> ret = cv::kinfu::VolumeParams::coarseParams(_volumeType);
			Ok(new cv::Ptr<cv::kinfu::VolumeParams>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::kinfu::VolumeParams::defaultNew() generated
	// ("cv::kinfu::VolumeParams::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::kinfu::VolumeParams* cv_kinfu_VolumeParams_defaultNew_const() {
			cv::kinfu::VolumeParams* ret = new cv::kinfu::VolumeParams();
			return ret;
	}

	// cv::kinfu::VolumeParams::type() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:67
	// ("cv::kinfu::VolumeParams::type", vec![(pred!(const, [], []), _)]),
	void cv_kinfu_VolumeParams_propType_const(const cv::kinfu::VolumeParams* instance, cv::kinfu::VolumeType* ocvrs_return) {
			cv::kinfu::VolumeType ret = instance->type;
			*ocvrs_return = ret;
	}

	// cv::kinfu::VolumeParams::setType(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:67
	// ("cv::kinfu::VolumeParams::setType", vec![(pred!(mut, ["val"], ["const cv::kinfu::VolumeType"]), _)]),
	void cv_kinfu_VolumeParams_propType_const_VolumeType(cv::kinfu::VolumeParams* instance, const cv::kinfu::VolumeType val) {
			instance->type = val;
	}

	// cv::kinfu::VolumeParams::resolution() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:74
	// ("cv::kinfu::VolumeParams::resolution", vec![(pred!(const, [], []), _)]),
	void cv_kinfu_VolumeParams_propResolution_const(const cv::kinfu::VolumeParams* instance, cv::Vec3i* ocvrs_return) {
			cv::Vec3i ret = instance->resolution;
			*ocvrs_return = ret;
	}

	// cv::kinfu::VolumeParams::setResolution(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:74
	// ("cv::kinfu::VolumeParams::setResolution", vec![(pred!(mut, ["val"], ["const cv::Vec3i"]), _)]),
	void cv_kinfu_VolumeParams_propResolution_const_Vec3i(cv::kinfu::VolumeParams* instance, const cv::Vec3i* val) {
			instance->resolution = *val;
	}

	// cv::kinfu::VolumeParams::unitResolution() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:80
	// ("cv::kinfu::VolumeParams::unitResolution", vec![(pred!(const, [], []), _)]),
	int cv_kinfu_VolumeParams_propUnitResolution_const(const cv::kinfu::VolumeParams* instance) {
			int ret = instance->unitResolution;
			return ret;
	}

	// cv::kinfu::VolumeParams::setUnitResolution(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:80
	// ("cv::kinfu::VolumeParams::setUnitResolution", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_kinfu_VolumeParams_propUnitResolution_const_int(cv::kinfu::VolumeParams* instance, const int val) {
			instance->unitResolution = val;
	}

	// cv::kinfu::VolumeParams::pose() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:83
	// ("cv::kinfu::VolumeParams::pose", vec![(pred!(const, [], []), _)]),
	void cv_kinfu_VolumeParams_propPose_const(const cv::kinfu::VolumeParams* instance, cv::Affine3f* ocvrs_return) {
			cv::Affine3f ret = instance->pose;
			*ocvrs_return = ret;
	}

	// cv::kinfu::VolumeParams::setPose(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:83
	// ("cv::kinfu::VolumeParams::setPose", vec![(pred!(mut, ["val"], ["const cv::Affine3f"]), _)]),
	void cv_kinfu_VolumeParams_propPose_const_Affine3f(cv::kinfu::VolumeParams* instance, const cv::Affine3f* val) {
			instance->pose = *val;
	}

	// cv::kinfu::VolumeParams::voxelSize() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:86
	// ("cv::kinfu::VolumeParams::voxelSize", vec![(pred!(const, [], []), _)]),
	float cv_kinfu_VolumeParams_propVoxelSize_const(const cv::kinfu::VolumeParams* instance) {
			float ret = instance->voxelSize;
			return ret;
	}

	// cv::kinfu::VolumeParams::setVoxelSize(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:86
	// ("cv::kinfu::VolumeParams::setVoxelSize", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_kinfu_VolumeParams_propVoxelSize_const_float(cv::kinfu::VolumeParams* instance, const float val) {
			instance->voxelSize = val;
	}

	// cv::kinfu::VolumeParams::tsdfTruncDist() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:91
	// ("cv::kinfu::VolumeParams::tsdfTruncDist", vec![(pred!(const, [], []), _)]),
	float cv_kinfu_VolumeParams_propTsdfTruncDist_const(const cv::kinfu::VolumeParams* instance) {
			float ret = instance->tsdfTruncDist;
			return ret;
	}

	// cv::kinfu::VolumeParams::setTsdfTruncDist(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:91
	// ("cv::kinfu::VolumeParams::setTsdfTruncDist", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_kinfu_VolumeParams_propTsdfTruncDist_const_float(cv::kinfu::VolumeParams* instance, const float val) {
			instance->tsdfTruncDist = val;
	}

	// cv::kinfu::VolumeParams::maxWeight() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:97
	// ("cv::kinfu::VolumeParams::maxWeight", vec![(pred!(const, [], []), _)]),
	int cv_kinfu_VolumeParams_propMaxWeight_const(const cv::kinfu::VolumeParams* instance) {
			int ret = instance->maxWeight;
			return ret;
	}

	// cv::kinfu::VolumeParams::setMaxWeight(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:97
	// ("cv::kinfu::VolumeParams::setMaxWeight", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_kinfu_VolumeParams_propMaxWeight_const_int(cv::kinfu::VolumeParams* instance, const int val) {
			instance->maxWeight = val;
	}

	// cv::kinfu::VolumeParams::depthTruncThreshold() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:102
	// ("cv::kinfu::VolumeParams::depthTruncThreshold", vec![(pred!(const, [], []), _)]),
	float cv_kinfu_VolumeParams_propDepthTruncThreshold_const(const cv::kinfu::VolumeParams* instance) {
			float ret = instance->depthTruncThreshold;
			return ret;
	}

	// cv::kinfu::VolumeParams::setDepthTruncThreshold(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:102
	// ("cv::kinfu::VolumeParams::setDepthTruncThreshold", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_kinfu_VolumeParams_propDepthTruncThreshold_const_float(cv::kinfu::VolumeParams* instance, const float val) {
			instance->depthTruncThreshold = val;
	}

	// cv::kinfu::VolumeParams::raycastStepFactor() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:107
	// ("cv::kinfu::VolumeParams::raycastStepFactor", vec![(pred!(const, [], []), _)]),
	float cv_kinfu_VolumeParams_propRaycastStepFactor_const(const cv::kinfu::VolumeParams* instance) {
			float ret = instance->raycastStepFactor;
			return ret;
	}

	// cv::kinfu::VolumeParams::setRaycastStepFactor(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/volume.hpp:107
	// ("cv::kinfu::VolumeParams::setRaycastStepFactor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_kinfu_VolumeParams_propRaycastStepFactor_const_float(cv::kinfu::VolumeParams* instance, const float val) {
			instance->raycastStepFactor = val;
	}

	// cv::kinfu::VolumeParams::delete() generated
	// ("cv::kinfu::VolumeParams::delete", vec![(pred!(mut, [], []), _)]),
	void cv_kinfu_VolumeParams_delete(cv::kinfu::VolumeParams* instance) {
			delete instance;
	}

	// create()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/detail/pose_graph.hpp:29
	// ("cv::kinfu::detail::PoseGraph::create", vec![(pred!(mut, [], []), _)]),
	void cv_kinfu_detail_PoseGraph_create(Result<cv::Ptr<cv::kinfu::detail::PoseGraph>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::kinfu::detail::PoseGraph> ret = cv::kinfu::detail::PoseGraph::create();
			Ok(new cv::Ptr<cv::kinfu::detail::PoseGraph>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// addNode(size_t, const Affine3d &, bool)(Primitive, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/detail/pose_graph.hpp:33
	// ("cv::kinfu::detail::PoseGraph::addNode", vec![(pred!(mut, ["_nodeId", "_pose", "fixed"], ["size_t", "const cv::Affine3d*", "bool"]), _)]),
	void cv_kinfu_detail_PoseGraph_addNode_size_t_const_Affine3dR_bool(cv::kinfu::detail::PoseGraph* instance, size_t _nodeId, const cv::Affine3d* _pose, bool fixed, ResultVoid* ocvrs_return) {
		try {
			instance->addNode(_nodeId, *_pose, fixed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isNodeExist(size_t)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/detail/pose_graph.hpp:34
	// ("cv::kinfu::detail::PoseGraph::isNodeExist", vec![(pred!(const, ["nodeId"], ["size_t"]), _)]),
	void cv_kinfu_detail_PoseGraph_isNodeExist_const_size_t(const cv::kinfu::detail::PoseGraph* instance, size_t nodeId, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isNodeExist(nodeId);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNodeFixed(size_t, bool)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/detail/pose_graph.hpp:35
	// ("cv::kinfu::detail::PoseGraph::setNodeFixed", vec![(pred!(mut, ["nodeId", "fixed"], ["size_t", "bool"]), _)]),
	void cv_kinfu_detail_PoseGraph_setNodeFixed_size_t_bool(cv::kinfu::detail::PoseGraph* instance, size_t nodeId, bool fixed, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->setNodeFixed(nodeId, fixed);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isNodeFixed(size_t)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/detail/pose_graph.hpp:36
	// ("cv::kinfu::detail::PoseGraph::isNodeFixed", vec![(pred!(const, ["nodeId"], ["size_t"]), _)]),
	void cv_kinfu_detail_PoseGraph_isNodeFixed_const_size_t(const cv::kinfu::detail::PoseGraph* instance, size_t nodeId, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isNodeFixed(nodeId);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNodePose(size_t)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/detail/pose_graph.hpp:37
	// ("cv::kinfu::detail::PoseGraph::getNodePose", vec![(pred!(const, ["nodeId"], ["size_t"]), _)]),
	void cv_kinfu_detail_PoseGraph_getNodePose_const_size_t(const cv::kinfu::detail::PoseGraph* instance, size_t nodeId, Result<cv::Affine3d>* ocvrs_return) {
		try {
			cv::Affine3d ret = instance->getNodePose(nodeId);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNodesIds()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/detail/pose_graph.hpp:38
	// ("cv::kinfu::detail::PoseGraph::getNodesIds", vec![(pred!(const, [], []), _)]),
	void cv_kinfu_detail_PoseGraph_getNodesIds_const(const cv::kinfu::detail::PoseGraph* instance, Result<std::vector<size_t>*>* ocvrs_return) {
		try {
			std::vector<size_t> ret = instance->getNodesIds();
			Ok(new std::vector<size_t>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNumNodes()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/detail/pose_graph.hpp:39
	// ("cv::kinfu::detail::PoseGraph::getNumNodes", vec![(pred!(const, [], []), _)]),
	void cv_kinfu_detail_PoseGraph_getNumNodes_const(const cv::kinfu::detail::PoseGraph* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->getNumNodes();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// addEdge(size_t, size_t, const Affine3f &, const Matx66f &)(Primitive, Primitive, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/detail/pose_graph.hpp:42
	// ("cv::kinfu::detail::PoseGraph::addEdge", vec![(pred!(mut, ["_sourceNodeId", "_targetNodeId", "_transformation", "_information"], ["size_t", "size_t", "const cv::Affine3f*", "const cv::Matx66f*"]), _)]),
	void cv_kinfu_detail_PoseGraph_addEdge_size_t_size_t_const_Affine3fR_const_Matx66fR(cv::kinfu::detail::PoseGraph* instance, size_t _sourceNodeId, size_t _targetNodeId, const cv::Affine3f* _transformation, const cv::Matx66f* _information, ResultVoid* ocvrs_return) {
		try {
			instance->addEdge(_sourceNodeId, _targetNodeId, *_transformation, *_information);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::kinfu::detail::PoseGraph::addEdge(Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/detail/pose_graph.hpp:42
	// ("cv::kinfu::detail::PoseGraph::addEdge", vec![(pred!(mut, ["_sourceNodeId", "_targetNodeId", "_transformation"], ["size_t", "size_t", "const cv::Affine3f*"]), _)]),
	void cv_kinfu_detail_PoseGraph_addEdge_size_t_size_t_const_Affine3fR(cv::kinfu::detail::PoseGraph* instance, size_t _sourceNodeId, size_t _targetNodeId, const cv::Affine3f* _transformation, ResultVoid* ocvrs_return) {
		try {
			instance->addEdge(_sourceNodeId, _targetNodeId, *_transformation);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getEdgeStart(size_t)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/detail/pose_graph.hpp:44
	// ("cv::kinfu::detail::PoseGraph::getEdgeStart", vec![(pred!(const, ["i"], ["size_t"]), _)]),
	void cv_kinfu_detail_PoseGraph_getEdgeStart_const_size_t(const cv::kinfu::detail::PoseGraph* instance, size_t i, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->getEdgeStart(i);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getEdgeEnd(size_t)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/detail/pose_graph.hpp:45
	// ("cv::kinfu::detail::PoseGraph::getEdgeEnd", vec![(pred!(const, ["i"], ["size_t"]), _)]),
	void cv_kinfu_detail_PoseGraph_getEdgeEnd_const_size_t(const cv::kinfu::detail::PoseGraph* instance, size_t i, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->getEdgeEnd(i);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNumEdges()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/detail/pose_graph.hpp:46
	// ("cv::kinfu::detail::PoseGraph::getNumEdges", vec![(pred!(const, [], []), _)]),
	void cv_kinfu_detail_PoseGraph_getNumEdges_const(const cv::kinfu::detail::PoseGraph* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->getNumEdges();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isValid()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/detail/pose_graph.hpp:49
	// ("cv::kinfu::detail::PoseGraph::isValid", vec![(pred!(const, [], []), _)]),
	void cv_kinfu_detail_PoseGraph_isValid_const(const cv::kinfu::detail::PoseGraph* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isValid();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// optimize(const cv::TermCriteria &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/detail/pose_graph.hpp:53
	// ("cv::kinfu::detail::PoseGraph::optimize", vec![(pred!(mut, ["tc"], ["const cv::TermCriteria*"]), _)]),
	void cv_kinfu_detail_PoseGraph_optimize_const_TermCriteriaR(cv::kinfu::detail::PoseGraph* instance, const cv::TermCriteria* tc, Result<int>* ocvrs_return) {
		try {
			int ret = instance->optimize(*tc);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::kinfu::detail::PoseGraph::optimize() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/detail/pose_graph.hpp:53
	// ("cv::kinfu::detail::PoseGraph::optimize", vec![(pred!(mut, [], []), _)]),
	void cv_kinfu_detail_PoseGraph_optimize(cv::kinfu::detail::PoseGraph* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->optimize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// calcEnergy()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/detail/pose_graph.hpp:56
	// ("cv::kinfu::detail::PoseGraph::calcEnergy", vec![(pred!(const, [], []), _)]),
	void cv_kinfu_detail_PoseGraph_calcEnergy_const(const cv::kinfu::detail::PoseGraph* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->calcEnergy();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::kinfu::detail::PoseGraph::delete() generated
	// ("cv::kinfu::detail::PoseGraph::delete", vec![(pred!(mut, [], []), _)]),
	void cv_kinfu_detail_PoseGraph_delete(cv::kinfu::detail::PoseGraph* instance) {
			delete instance;
	}

	// create(const Ptr<Params> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:123
	// ("cv::large_kinfu::LargeKinfu::create", vec![(pred!(mut, ["_params"], ["const cv::Ptr<cv::large_kinfu::Params>*"]), _)]),
	void cv_large_kinfu_LargeKinfu_create_const_PtrLParamsGR(const cv::Ptr<cv::large_kinfu::Params>* _params, Result<cv::Ptr<cv::large_kinfu::LargeKinfu>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::large_kinfu::LargeKinfu> ret = cv::large_kinfu::LargeKinfu::create(*_params);
			Ok(new cv::Ptr<cv::large_kinfu::LargeKinfu>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getParams()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:126
	// ("cv::large_kinfu::LargeKinfu::getParams", vec![(pred!(const, [], []), _)]),
	void cv_large_kinfu_LargeKinfu_getParams_const(const cv::large_kinfu::LargeKinfu* instance, Result<cv::large_kinfu::Params*>* ocvrs_return) {
		try {
			const cv::large_kinfu::Params ret = instance->getParams();
			Ok(new const cv::large_kinfu::Params(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// render(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:128
	// ("cv::large_kinfu::LargeKinfu::render", vec![(pred!(const, ["image"], ["const cv::_OutputArray*"]), _)]),
	void cv_large_kinfu_LargeKinfu_render_const_const__OutputArrayR(const cv::large_kinfu::LargeKinfu* instance, const cv::_OutputArray* image, ResultVoid* ocvrs_return) {
		try {
			instance->render(*image);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// render(OutputArray, const Matx44f &)(OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:129
	// ("cv::large_kinfu::LargeKinfu::render", vec![(pred!(const, ["image", "cameraPose"], ["const cv::_OutputArray*", "const cv::Matx44f*"]), _)]),
	void cv_large_kinfu_LargeKinfu_render_const_const__OutputArrayR_const_Matx44fR(const cv::large_kinfu::LargeKinfu* instance, const cv::_OutputArray* image, const cv::Matx44f* cameraPose, ResultVoid* ocvrs_return) {
		try {
			instance->render(*image, *cameraPose);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getCloud(OutputArray, OutputArray)(OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:131
	// ("cv::large_kinfu::LargeKinfu::getCloud", vec![(pred!(const, ["points", "normals"], ["const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_large_kinfu_LargeKinfu_getCloud_const_const__OutputArrayR_const__OutputArrayR(const cv::large_kinfu::LargeKinfu* instance, const cv::_OutputArray* points, const cv::_OutputArray* normals, ResultVoid* ocvrs_return) {
		try {
			instance->getCloud(*points, *normals);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPoints(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:133
	// ("cv::large_kinfu::LargeKinfu::getPoints", vec![(pred!(const, ["points"], ["const cv::_OutputArray*"]), _)]),
	void cv_large_kinfu_LargeKinfu_getPoints_const_const__OutputArrayR(const cv::large_kinfu::LargeKinfu* instance, const cv::_OutputArray* points, ResultVoid* ocvrs_return) {
		try {
			instance->getPoints(*points);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNormals(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:135
	// ("cv::large_kinfu::LargeKinfu::getNormals", vec![(pred!(const, ["points", "normals"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_large_kinfu_LargeKinfu_getNormals_const_const__InputArrayR_const__OutputArrayR(const cv::large_kinfu::LargeKinfu* instance, const cv::_InputArray* points, const cv::_OutputArray* normals, ResultVoid* ocvrs_return) {
		try {
			instance->getNormals(*points, *normals);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// reset()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:137
	// ("cv::large_kinfu::LargeKinfu::reset", vec![(pred!(mut, [], []), _)]),
	void cv_large_kinfu_LargeKinfu_reset(cv::large_kinfu::LargeKinfu* instance, ResultVoid* ocvrs_return) {
		try {
			instance->reset();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPose()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:139
	// ("cv::large_kinfu::LargeKinfu::getPose", vec![(pred!(const, [], []), _)]),
	void cv_large_kinfu_LargeKinfu_getPose_const(const cv::large_kinfu::LargeKinfu* instance, Result<cv::Affine3f>* ocvrs_return) {
		try {
			cv::Affine3f ret = instance->getPose();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// update(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:141
	// ("cv::large_kinfu::LargeKinfu::update", vec![(pred!(mut, ["depth"], ["const cv::_InputArray*"]), _)]),
	void cv_large_kinfu_LargeKinfu_update_const__InputArrayR(cv::large_kinfu::LargeKinfu* instance, const cv::_InputArray* depth, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->update(*depth);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::large_kinfu::LargeKinfu::delete() generated
	// ("cv::large_kinfu::LargeKinfu::delete", vec![(pred!(mut, [], []), _)]),
	void cv_large_kinfu_LargeKinfu_delete(cv::large_kinfu::LargeKinfu* instance) {
			delete instance;
	}

	// defaultParams()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:25
	// ("cv::large_kinfu::Params::defaultParams", vec![(pred!(mut, [], []), _)]),
	void cv_large_kinfu_Params_defaultParams(Result<cv::Ptr<cv::large_kinfu::Params>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::large_kinfu::Params> ret = cv::large_kinfu::Params::defaultParams();
			Ok(new cv::Ptr<cv::large_kinfu::Params>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// coarseParams()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:31
	// ("cv::large_kinfu::Params::coarseParams", vec![(pred!(mut, [], []), _)]),
	void cv_large_kinfu_Params_coarseParams(Result<cv::Ptr<cv::large_kinfu::Params>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::large_kinfu::Params> ret = cv::large_kinfu::Params::coarseParams();
			Ok(new cv::Ptr<cv::large_kinfu::Params>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// hashTSDFParams(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:36
	// ("cv::large_kinfu::Params::hashTSDFParams", vec![(pred!(mut, ["isCoarse"], ["bool"]), _)]),
	void cv_large_kinfu_Params_hashTSDFParams_bool(bool isCoarse, Result<cv::Ptr<cv::large_kinfu::Params>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::large_kinfu::Params> ret = cv::large_kinfu::Params::hashTSDFParams(isCoarse);
			Ok(new cv::Ptr<cv::large_kinfu::Params>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::large_kinfu::Params::defaultNew() generated
	// ("cv::large_kinfu::Params::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::large_kinfu::Params* cv_large_kinfu_Params_defaultNew_const() {
			cv::large_kinfu::Params* ret = new cv::large_kinfu::Params();
			return ret;
	}

	// cv::large_kinfu::Params::frameSize() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:39
	// ("cv::large_kinfu::Params::frameSize", vec![(pred!(const, [], []), _)]),
	void cv_large_kinfu_Params_propFrameSize_const(const cv::large_kinfu::Params* instance, cv::Size* ocvrs_return) {
			cv::Size ret = instance->frameSize;
			*ocvrs_return = ret;
	}

	// cv::large_kinfu::Params::setFrameSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:39
	// ("cv::large_kinfu::Params::setFrameSize", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	void cv_large_kinfu_Params_propFrameSize_const_Size(cv::large_kinfu::Params* instance, const cv::Size* val) {
			instance->frameSize = *val;
	}

	// cv::large_kinfu::Params::intr() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:42
	// ("cv::large_kinfu::Params::intr", vec![(pred!(const, [], []), _)]),
	void cv_large_kinfu_Params_propIntr_const(const cv::large_kinfu::Params* instance, cv::Matx33f* ocvrs_return) {
			cv::Matx33f ret = instance->intr;
			*ocvrs_return = ret;
	}

	// cv::large_kinfu::Params::setIntr(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:42
	// ("cv::large_kinfu::Params::setIntr", vec![(pred!(mut, ["val"], ["const cv::Matx33f"]), _)]),
	void cv_large_kinfu_Params_propIntr_const_Matx33f(cv::large_kinfu::Params* instance, const cv::Matx33f* val) {
			instance->intr = *val;
	}

	// cv::large_kinfu::Params::rgb_intr() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:45
	// ("cv::large_kinfu::Params::rgb_intr", vec![(pred!(const, [], []), _)]),
	void cv_large_kinfu_Params_propRgb_intr_const(const cv::large_kinfu::Params* instance, cv::Matx33f* ocvrs_return) {
			cv::Matx33f ret = instance->rgb_intr;
			*ocvrs_return = ret;
	}

	// cv::large_kinfu::Params::setRgb_intr(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:45
	// ("cv::large_kinfu::Params::setRgb_intr", vec![(pred!(mut, ["val"], ["const cv::Matx33f"]), _)]),
	void cv_large_kinfu_Params_propRgb_intr_const_Matx33f(cv::large_kinfu::Params* instance, const cv::Matx33f* val) {
			instance->rgb_intr = *val;
	}

	// cv::large_kinfu::Params::depthFactor() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:53
	// ("cv::large_kinfu::Params::depthFactor", vec![(pred!(const, [], []), _)]),
	float cv_large_kinfu_Params_propDepthFactor_const(const cv::large_kinfu::Params* instance) {
			float ret = instance->depthFactor;
			return ret;
	}

	// cv::large_kinfu::Params::setDepthFactor(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:53
	// ("cv::large_kinfu::Params::setDepthFactor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_large_kinfu_Params_propDepthFactor_const_float(cv::large_kinfu::Params* instance, const float val) {
			instance->depthFactor = val;
	}

	// cv::large_kinfu::Params::bilateral_sigma_depth() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:56
	// ("cv::large_kinfu::Params::bilateral_sigma_depth", vec![(pred!(const, [], []), _)]),
	float cv_large_kinfu_Params_propBilateral_sigma_depth_const(const cv::large_kinfu::Params* instance) {
			float ret = instance->bilateral_sigma_depth;
			return ret;
	}

	// cv::large_kinfu::Params::setBilateral_sigma_depth(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:56
	// ("cv::large_kinfu::Params::setBilateral_sigma_depth", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_large_kinfu_Params_propBilateral_sigma_depth_const_float(cv::large_kinfu::Params* instance, const float val) {
			instance->bilateral_sigma_depth = val;
	}

	// cv::large_kinfu::Params::bilateral_sigma_spatial() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:58
	// ("cv::large_kinfu::Params::bilateral_sigma_spatial", vec![(pred!(const, [], []), _)]),
	float cv_large_kinfu_Params_propBilateral_sigma_spatial_const(const cv::large_kinfu::Params* instance) {
			float ret = instance->bilateral_sigma_spatial;
			return ret;
	}

	// cv::large_kinfu::Params::setBilateral_sigma_spatial(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:58
	// ("cv::large_kinfu::Params::setBilateral_sigma_spatial", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_large_kinfu_Params_propBilateral_sigma_spatial_const_float(cv::large_kinfu::Params* instance, const float val) {
			instance->bilateral_sigma_spatial = val;
	}

	// cv::large_kinfu::Params::bilateral_kernel_size() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:60
	// ("cv::large_kinfu::Params::bilateral_kernel_size", vec![(pred!(const, [], []), _)]),
	int cv_large_kinfu_Params_propBilateral_kernel_size_const(const cv::large_kinfu::Params* instance) {
			int ret = instance->bilateral_kernel_size;
			return ret;
	}

	// cv::large_kinfu::Params::setBilateral_kernel_size(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:60
	// ("cv::large_kinfu::Params::setBilateral_kernel_size", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_large_kinfu_Params_propBilateral_kernel_size_const_int(cv::large_kinfu::Params* instance, const int val) {
			instance->bilateral_kernel_size = val;
	}

	// cv::large_kinfu::Params::pyramidLevels() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:63
	// ("cv::large_kinfu::Params::pyramidLevels", vec![(pred!(const, [], []), _)]),
	int cv_large_kinfu_Params_propPyramidLevels_const(const cv::large_kinfu::Params* instance) {
			int ret = instance->pyramidLevels;
			return ret;
	}

	// cv::large_kinfu::Params::setPyramidLevels(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:63
	// ("cv::large_kinfu::Params::setPyramidLevels", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_large_kinfu_Params_propPyramidLevels_const_int(cv::large_kinfu::Params* instance, const int val) {
			instance->pyramidLevels = val;
	}

	// cv::large_kinfu::Params::tsdf_min_camera_movement() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:68
	// ("cv::large_kinfu::Params::tsdf_min_camera_movement", vec![(pred!(const, [], []), _)]),
	float cv_large_kinfu_Params_propTsdf_min_camera_movement_const(const cv::large_kinfu::Params* instance) {
			float ret = instance->tsdf_min_camera_movement;
			return ret;
	}

	// cv::large_kinfu::Params::setTsdf_min_camera_movement(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:68
	// ("cv::large_kinfu::Params::setTsdf_min_camera_movement", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_large_kinfu_Params_propTsdf_min_camera_movement_const_float(cv::large_kinfu::Params* instance, const float val) {
			instance->tsdf_min_camera_movement = val;
	}

	// cv::large_kinfu::Params::lightPose() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:71
	// ("cv::large_kinfu::Params::lightPose", vec![(pred!(const, [], []), _)]),
	void cv_large_kinfu_Params_propLightPose_const(const cv::large_kinfu::Params* instance, cv::Vec3f* ocvrs_return) {
			cv::Vec3f ret = instance->lightPose;
			*ocvrs_return = ret;
	}

	// cv::large_kinfu::Params::setLightPose(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:71
	// ("cv::large_kinfu::Params::setLightPose", vec![(pred!(mut, ["val"], ["const cv::Vec3f"]), _)]),
	void cv_large_kinfu_Params_propLightPose_const_Vec3f(cv::large_kinfu::Params* instance, const cv::Vec3f* val) {
			instance->lightPose = *val;
	}

	// cv::large_kinfu::Params::icpDistThresh() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:74
	// ("cv::large_kinfu::Params::icpDistThresh", vec![(pred!(const, [], []), _)]),
	float cv_large_kinfu_Params_propIcpDistThresh_const(const cv::large_kinfu::Params* instance) {
			float ret = instance->icpDistThresh;
			return ret;
	}

	// cv::large_kinfu::Params::setIcpDistThresh(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:74
	// ("cv::large_kinfu::Params::setIcpDistThresh", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_large_kinfu_Params_propIcpDistThresh_const_float(cv::large_kinfu::Params* instance, const float val) {
			instance->icpDistThresh = val;
	}

	// cv::large_kinfu::Params::icpAngleThresh() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:76
	// ("cv::large_kinfu::Params::icpAngleThresh", vec![(pred!(const, [], []), _)]),
	float cv_large_kinfu_Params_propIcpAngleThresh_const(const cv::large_kinfu::Params* instance) {
			float ret = instance->icpAngleThresh;
			return ret;
	}

	// cv::large_kinfu::Params::setIcpAngleThresh(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:76
	// ("cv::large_kinfu::Params::setIcpAngleThresh", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_large_kinfu_Params_propIcpAngleThresh_const_float(cv::large_kinfu::Params* instance, const float val) {
			instance->icpAngleThresh = val;
	}

	// cv::large_kinfu::Params::icpIterations() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:78
	// ("cv::large_kinfu::Params::icpIterations", vec![(pred!(const, [], []), _)]),
	std::vector<int>* cv_large_kinfu_Params_propIcpIterations_const(const cv::large_kinfu::Params* instance) {
			std::vector<int> ret = instance->icpIterations;
			return new std::vector<int>(ret);
	}

	// cv::large_kinfu::Params::setIcpIterations(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:78
	// ("cv::large_kinfu::Params::setIcpIterations", vec![(pred!(mut, ["val"], ["const std::vector<int>"]), _)]),
	void cv_large_kinfu_Params_propIcpIterations_const_vectorLintG(cv::large_kinfu::Params* instance, const std::vector<int>* val) {
			instance->icpIterations = *val;
	}

	// cv::large_kinfu::Params::truncateThreshold() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:83
	// ("cv::large_kinfu::Params::truncateThreshold", vec![(pred!(const, [], []), _)]),
	float cv_large_kinfu_Params_propTruncateThreshold_const(const cv::large_kinfu::Params* instance) {
			float ret = instance->truncateThreshold;
			return ret;
	}

	// cv::large_kinfu::Params::setTruncateThreshold(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:83
	// ("cv::large_kinfu::Params::setTruncateThreshold", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_large_kinfu_Params_propTruncateThreshold_const_float(cv::large_kinfu::Params* instance, const float val) {
			instance->truncateThreshold = val;
	}

	// cv::large_kinfu::Params::volumeParams() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:87
	// ("cv::large_kinfu::Params::volumeParams", vec![(pred!(const, [], []), _)]),
	cv::kinfu::VolumeParams* cv_large_kinfu_Params_propVolumeParams_const(const cv::large_kinfu::Params* instance) {
			cv::kinfu::VolumeParams ret = instance->volumeParams;
			return new cv::kinfu::VolumeParams(ret);
	}

	// cv::large_kinfu::Params::setVolumeParams(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/large_kinfu.hpp:87
	// ("cv::large_kinfu::Params::setVolumeParams", vec![(pred!(mut, ["val"], ["const cv::kinfu::VolumeParams"]), _)]),
	void cv_large_kinfu_Params_propVolumeParams_const_VolumeParams(cv::large_kinfu::Params* instance, const cv::kinfu::VolumeParams* val) {
			instance->volumeParams = *val;
	}

	// cv::large_kinfu::Params::delete() generated
	// ("cv::large_kinfu::Params::delete", vec![(pred!(mut, [], []), _)]),
	void cv_large_kinfu_Params_delete(cv::large_kinfu::Params* instance) {
			delete instance;
	}

	// ColorGradient()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:172
	// ("cv::linemod::ColorGradient::ColorGradient", vec![(pred!(mut, [], []), _)]),
	void cv_linemod_ColorGradient_ColorGradient(Result<cv::linemod::ColorGradient*>* ocvrs_return) {
		try {
			cv::linemod::ColorGradient* ret = new cv::linemod::ColorGradient();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ColorGradient(float, size_t, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:182
	// ("cv::linemod::ColorGradient::ColorGradient", vec![(pred!(mut, ["weak_threshold", "num_features", "strong_threshold"], ["float", "size_t", "float"]), _)]),
	void cv_linemod_ColorGradient_ColorGradient_float_size_t_float(float weak_threshold, size_t num_features, float strong_threshold, Result<cv::linemod::ColorGradient*>* ocvrs_return) {
		try {
			cv::linemod::ColorGradient* ret = new cv::linemod::ColorGradient(weak_threshold, num_features, strong_threshold);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(float, size_t, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:184
	// ("cv::linemod::ColorGradient::create", vec![(pred!(mut, ["weak_threshold", "num_features", "strong_threshold"], ["float", "size_t", "float"]), _)]),
	void cv_linemod_ColorGradient_create_float_size_t_float(float weak_threshold, size_t num_features, float strong_threshold, Result<cv::Ptr<cv::linemod::ColorGradient>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::linemod::ColorGradient> ret = cv::linemod::ColorGradient::create(weak_threshold, num_features, strong_threshold);
			Ok(new cv::Ptr<cv::linemod::ColorGradient>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// name()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:186
	// ("cv::linemod::ColorGradient::name", vec![(pred!(const, [], []), _)]),
	void cv_linemod_ColorGradient_name_const(const cv::linemod::ColorGradient* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->name();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:188
	// ("cv::linemod::ColorGradient::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	void cv_linemod_ColorGradient_read_const_FileNodeR(cv::linemod::ColorGradient* instance, const cv::FileNode* fn, ResultVoid* ocvrs_return) {
		try {
			instance->read(*fn);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:189
	// ("cv::linemod::ColorGradient::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	void cv_linemod_ColorGradient_write_const_FileStorageR(const cv::linemod::ColorGradient* instance, cv::FileStorage* fs, ResultVoid* ocvrs_return) {
		try {
			instance->write(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::linemod::ColorGradient::weak_threshold() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:191
	// ("cv::linemod::ColorGradient::weak_threshold", vec![(pred!(const, [], []), _)]),
	float cv_linemod_ColorGradient_propWeak_threshold_const(const cv::linemod::ColorGradient* instance) {
			float ret = instance->weak_threshold;
			return ret;
	}

	// cv::linemod::ColorGradient::setWeak_threshold(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:191
	// ("cv::linemod::ColorGradient::setWeak_threshold", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_linemod_ColorGradient_propWeak_threshold_const_float(cv::linemod::ColorGradient* instance, const float val) {
			instance->weak_threshold = val;
	}

	// cv::linemod::ColorGradient::num_features() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:192
	// ("cv::linemod::ColorGradient::num_features", vec![(pred!(const, [], []), _)]),
	size_t cv_linemod_ColorGradient_propNum_features_const(const cv::linemod::ColorGradient* instance) {
			size_t ret = instance->num_features;
			return ret;
	}

	// cv::linemod::ColorGradient::setNum_features(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:192
	// ("cv::linemod::ColorGradient::setNum_features", vec![(pred!(mut, ["val"], ["const size_t"]), _)]),
	void cv_linemod_ColorGradient_propNum_features_const_size_t(cv::linemod::ColorGradient* instance, const size_t val) {
			instance->num_features = val;
	}

	// cv::linemod::ColorGradient::strong_threshold() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:193
	// ("cv::linemod::ColorGradient::strong_threshold", vec![(pred!(const, [], []), _)]),
	float cv_linemod_ColorGradient_propStrong_threshold_const(const cv::linemod::ColorGradient* instance) {
			float ret = instance->strong_threshold;
			return ret;
	}

	// cv::linemod::ColorGradient::setStrong_threshold(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:193
	// ("cv::linemod::ColorGradient::setStrong_threshold", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_linemod_ColorGradient_propStrong_threshold_const_float(cv::linemod::ColorGradient* instance, const float val) {
			instance->strong_threshold = val;
	}

	// cv::linemod::ColorGradient::to_LineMod_Modality() generated
	// ("cv::linemod::ColorGradient::to_LineMod_Modality", vec![(pred!(mut, [], []), _)]),
	cv::linemod::Modality* cv_linemod_ColorGradient_to_LineMod_Modality(cv::linemod::ColorGradient* instance) {
			return dynamic_cast<cv::linemod::Modality*>(instance);
	}

	// cv::linemod::ColorGradient::delete() generated
	// ("cv::linemod::ColorGradient::delete", vec![(pred!(mut, [], []), _)]),
	void cv_linemod_ColorGradient_delete(cv::linemod::ColorGradient* instance) {
			delete instance;
	}

	// DepthNormal()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:209
	// ("cv::linemod::DepthNormal::DepthNormal", vec![(pred!(mut, [], []), _)]),
	void cv_linemod_DepthNormal_DepthNormal(Result<cv::linemod::DepthNormal*>* ocvrs_return) {
		try {
			cv::linemod::DepthNormal* ret = new cv::linemod::DepthNormal();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// DepthNormal(int, int, size_t, int)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:221
	// ("cv::linemod::DepthNormal::DepthNormal", vec![(pred!(mut, ["distance_threshold", "difference_threshold", "num_features", "extract_threshold"], ["int", "int", "size_t", "int"]), _)]),
	void cv_linemod_DepthNormal_DepthNormal_int_int_size_t_int(int distance_threshold, int difference_threshold, size_t num_features, int extract_threshold, Result<cv::linemod::DepthNormal*>* ocvrs_return) {
		try {
			cv::linemod::DepthNormal* ret = new cv::linemod::DepthNormal(distance_threshold, difference_threshold, num_features, extract_threshold);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(int, int, size_t, int)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:224
	// ("cv::linemod::DepthNormal::create", vec![(pred!(mut, ["distance_threshold", "difference_threshold", "num_features", "extract_threshold"], ["int", "int", "size_t", "int"]), _)]),
	void cv_linemod_DepthNormal_create_int_int_size_t_int(int distance_threshold, int difference_threshold, size_t num_features, int extract_threshold, Result<cv::Ptr<cv::linemod::DepthNormal>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::linemod::DepthNormal> ret = cv::linemod::DepthNormal::create(distance_threshold, difference_threshold, num_features, extract_threshold);
			Ok(new cv::Ptr<cv::linemod::DepthNormal>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// name()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:227
	// ("cv::linemod::DepthNormal::name", vec![(pred!(const, [], []), _)]),
	void cv_linemod_DepthNormal_name_const(const cv::linemod::DepthNormal* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->name();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:229
	// ("cv::linemod::DepthNormal::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	void cv_linemod_DepthNormal_read_const_FileNodeR(cv::linemod::DepthNormal* instance, const cv::FileNode* fn, ResultVoid* ocvrs_return) {
		try {
			instance->read(*fn);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:230
	// ("cv::linemod::DepthNormal::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	void cv_linemod_DepthNormal_write_const_FileStorageR(const cv::linemod::DepthNormal* instance, cv::FileStorage* fs, ResultVoid* ocvrs_return) {
		try {
			instance->write(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::linemod::DepthNormal::distance_threshold() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:232
	// ("cv::linemod::DepthNormal::distance_threshold", vec![(pred!(const, [], []), _)]),
	int cv_linemod_DepthNormal_propDistance_threshold_const(const cv::linemod::DepthNormal* instance) {
			int ret = instance->distance_threshold;
			return ret;
	}

	// cv::linemod::DepthNormal::setDistance_threshold(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:232
	// ("cv::linemod::DepthNormal::setDistance_threshold", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_linemod_DepthNormal_propDistance_threshold_const_int(cv::linemod::DepthNormal* instance, const int val) {
			instance->distance_threshold = val;
	}

	// cv::linemod::DepthNormal::difference_threshold() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:233
	// ("cv::linemod::DepthNormal::difference_threshold", vec![(pred!(const, [], []), _)]),
	int cv_linemod_DepthNormal_propDifference_threshold_const(const cv::linemod::DepthNormal* instance) {
			int ret = instance->difference_threshold;
			return ret;
	}

	// cv::linemod::DepthNormal::setDifference_threshold(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:233
	// ("cv::linemod::DepthNormal::setDifference_threshold", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_linemod_DepthNormal_propDifference_threshold_const_int(cv::linemod::DepthNormal* instance, const int val) {
			instance->difference_threshold = val;
	}

	// cv::linemod::DepthNormal::num_features() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:234
	// ("cv::linemod::DepthNormal::num_features", vec![(pred!(const, [], []), _)]),
	size_t cv_linemod_DepthNormal_propNum_features_const(const cv::linemod::DepthNormal* instance) {
			size_t ret = instance->num_features;
			return ret;
	}

	// cv::linemod::DepthNormal::setNum_features(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:234
	// ("cv::linemod::DepthNormal::setNum_features", vec![(pred!(mut, ["val"], ["const size_t"]), _)]),
	void cv_linemod_DepthNormal_propNum_features_const_size_t(cv::linemod::DepthNormal* instance, const size_t val) {
			instance->num_features = val;
	}

	// cv::linemod::DepthNormal::extract_threshold() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:235
	// ("cv::linemod::DepthNormal::extract_threshold", vec![(pred!(const, [], []), _)]),
	int cv_linemod_DepthNormal_propExtract_threshold_const(const cv::linemod::DepthNormal* instance) {
			int ret = instance->extract_threshold;
			return ret;
	}

	// cv::linemod::DepthNormal::setExtract_threshold(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:235
	// ("cv::linemod::DepthNormal::setExtract_threshold", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_linemod_DepthNormal_propExtract_threshold_const_int(cv::linemod::DepthNormal* instance, const int val) {
			instance->extract_threshold = val;
	}

	// cv::linemod::DepthNormal::to_LineMod_Modality() generated
	// ("cv::linemod::DepthNormal::to_LineMod_Modality", vec![(pred!(mut, [], []), _)]),
	cv::linemod::Modality* cv_linemod_DepthNormal_to_LineMod_Modality(cv::linemod::DepthNormal* instance) {
			return dynamic_cast<cv::linemod::Modality*>(instance);
	}

	// cv::linemod::DepthNormal::delete() generated
	// ("cv::linemod::DepthNormal::delete", vec![(pred!(mut, [], []), _)]),
	void cv_linemod_DepthNormal_delete(cv::linemod::DepthNormal* instance) {
			delete instance;
	}

	// Detector()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:304
	// ("cv::linemod::Detector::Detector", vec![(pred!(mut, [], []), _)]),
	void cv_linemod_Detector_Detector(Result<cv::linemod::Detector*>* ocvrs_return) {
		try {
			cv::linemod::Detector* ret = new cv::linemod::Detector();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Detector(const std::vector<Ptr<Modality>> &, const std::vector<int> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:313
	// ("cv::linemod::Detector::Detector", vec![(pred!(mut, ["modalities", "T_pyramid"], ["const std::vector<cv::Ptr<cv::linemod::Modality>>*", "const std::vector<int>*"]), _)]),
	void cv_linemod_Detector_Detector_const_vectorLPtrLModalityGGR_const_vectorLintGR(const std::vector<cv::Ptr<cv::linemod::Modality>>* modalities, const std::vector<int>* T_pyramid, Result<cv::linemod::Detector*>* ocvrs_return) {
		try {
			cv::linemod::Detector* ret = new cv::linemod::Detector(*modalities, *T_pyramid);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// match(const std::vector<Mat> &, float, std::vector<Match> &, const std::vector<String> &, OutputArrayOfArrays, const std::vector<Mat> &)(CppPassByVoidPtr, Primitive, CppPassByVoidPtr, CppPassByVoidPtr, OutputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:330
	// ("cv::linemod::Detector::match", vec![(pred!(const, ["sources", "threshold", "matches", "class_ids", "quantized_images", "masks"], ["const std::vector<cv::Mat>*", "float", "std::vector<cv::linemod::Match>*", "const std::vector<cv::String>*", "const cv::_OutputArray*", "const std::vector<cv::Mat>*"]), _)]),
	void cv_linemod_Detector_match_const_const_vectorLMatGR_float_vectorLMatchGR_const_vectorLStringGR_const__OutputArrayR_const_vectorLMatGR(const cv::linemod::Detector* instance, const std::vector<cv::Mat>* sources, float threshold, std::vector<cv::linemod::Match>* matches, const std::vector<cv::String>* class_ids, const cv::_OutputArray* quantized_images, const std::vector<cv::Mat>* masks, ResultVoid* ocvrs_return) {
		try {
			instance->match(*sources, threshold, *matches, *class_ids, *quantized_images, *masks);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::linemod::Detector::match(CppPassByVoidPtr, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:330
	// ("cv::linemod::Detector::match", vec![(pred!(const, ["sources", "threshold", "matches"], ["const std::vector<cv::Mat>*", "float", "std::vector<cv::linemod::Match>*"]), _)]),
	void cv_linemod_Detector_match_const_const_vectorLMatGR_float_vectorLMatchGR(const cv::linemod::Detector* instance, const std::vector<cv::Mat>* sources, float threshold, std::vector<cv::linemod::Match>* matches, ResultVoid* ocvrs_return) {
		try {
			instance->match(*sources, threshold, *matches);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// addTemplate(const std::vector<Mat> &, const String &, const Mat &, Rect *)(CppPassByVoidPtr, InString, TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:345
	// ("cv::linemod::Detector::addTemplate", vec![(pred!(mut, ["sources", "class_id", "object_mask", "bounding_box"], ["const std::vector<cv::Mat>*", "const cv::String*", "const cv::Mat*", "cv::Rect*"]), _)]),
	void cv_linemod_Detector_addTemplate_const_vectorLMatGR_const_StringR_const_MatR_RectX(cv::linemod::Detector* instance, const std::vector<cv::Mat>* sources, const char* class_id, const cv::Mat* object_mask, cv::Rect* bounding_box, Result<int>* ocvrs_return) {
		try {
			int ret = instance->addTemplate(*sources, std::string(class_id), *object_mask, bounding_box);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::linemod::Detector::addTemplate(CppPassByVoidPtr, InString, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:345
	// ("cv::linemod::Detector::addTemplate", vec![(pred!(mut, ["sources", "class_id", "object_mask"], ["const std::vector<cv::Mat>*", "const cv::String*", "const cv::Mat*"]), _)]),
	void cv_linemod_Detector_addTemplate_const_vectorLMatGR_const_StringR_const_MatR(cv::linemod::Detector* instance, const std::vector<cv::Mat>* sources, const char* class_id, const cv::Mat* object_mask, Result<int>* ocvrs_return) {
		try {
			int ret = instance->addTemplate(*sources, std::string(class_id), *object_mask);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// addSyntheticTemplate(const std::vector<Template> &, const String &)(CppPassByVoidPtr, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:351
	// ("cv::linemod::Detector::addSyntheticTemplate", vec![(pred!(mut, ["templates", "class_id"], ["const std::vector<cv::linemod::Template>*", "const cv::String*"]), _)]),
	void cv_linemod_Detector_addSyntheticTemplate_const_vectorLTemplateGR_const_StringR(cv::linemod::Detector* instance, const std::vector<cv::linemod::Template>* templates, const char* class_id, Result<int>* ocvrs_return) {
		try {
			int ret = instance->addSyntheticTemplate(*templates, std::string(class_id));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getModalities()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:359
	// ("cv::linemod::Detector::getModalities", vec![(pred!(const, [], []), _)]),
	void cv_linemod_Detector_getModalities_const(const cv::linemod::Detector* instance, Result<std::vector<cv::Ptr<cv::linemod::Modality>>*>* ocvrs_return) {
		try {
			const std::vector<cv::Ptr<cv::linemod::Modality>> ret = instance->getModalities();
			Ok(new const std::vector<cv::Ptr<cv::linemod::Modality>>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getT(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:364
	// ("cv::linemod::Detector::getT", vec![(pred!(const, ["pyramid_level"], ["int"]), _)]),
	void cv_linemod_Detector_getT_const_int(const cv::linemod::Detector* instance, int pyramid_level, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getT(pyramid_level);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// pyramidLevels()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:369
	// ("cv::linemod::Detector::pyramidLevels", vec![(pred!(const, [], []), _)]),
	void cv_linemod_Detector_pyramidLevels_const(const cv::linemod::Detector* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->pyramidLevels();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTemplates(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:377
	// ("cv::linemod::Detector::getTemplates", vec![(pred!(const, ["class_id", "template_id"], ["const cv::String*", "int"]), _)]),
	void cv_linemod_Detector_getTemplates_const_const_StringR_int(const cv::linemod::Detector* instance, const char* class_id, int template_id, Result<std::vector<cv::linemod::Template>*>* ocvrs_return) {
		try {
			const std::vector<cv::linemod::Template> ret = instance->getTemplates(std::string(class_id), template_id);
			Ok(new const std::vector<cv::linemod::Template>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// numTemplates()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:379
	// ("cv::linemod::Detector::numTemplates", vec![(pred!(const, [], []), _)]),
	void cv_linemod_Detector_numTemplates_const(const cv::linemod::Detector* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->numTemplates();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// numTemplates(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:380
	// ("cv::linemod::Detector::numTemplates", vec![(pred!(const, ["class_id"], ["const cv::String*"]), _)]),
	void cv_linemod_Detector_numTemplates_const_const_StringR(const cv::linemod::Detector* instance, const char* class_id, Result<int>* ocvrs_return) {
		try {
			int ret = instance->numTemplates(std::string(class_id));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// numClasses()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:381
	// ("cv::linemod::Detector::numClasses", vec![(pred!(const, [], []), _)]),
	void cv_linemod_Detector_numClasses_const(const cv::linemod::Detector* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->numClasses();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// classIds()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:383
	// ("cv::linemod::Detector::classIds", vec![(pred!(const, [], []), _)]),
	void cv_linemod_Detector_classIds_const(const cv::linemod::Detector* instance, Result<std::vector<cv::String>*>* ocvrs_return) {
		try {
			std::vector<cv::String> ret = instance->classIds();
			Ok(new std::vector<cv::String>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:385
	// ("cv::linemod::Detector::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	void cv_linemod_Detector_read_const_FileNodeR(cv::linemod::Detector* instance, const cv::FileNode* fn, ResultVoid* ocvrs_return) {
		try {
			instance->read(*fn);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:386
	// ("cv::linemod::Detector::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	void cv_linemod_Detector_write_const_FileStorageR(const cv::linemod::Detector* instance, cv::FileStorage* fs, ResultVoid* ocvrs_return) {
		try {
			instance->write(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readClass(const FileNode &, const String &)(TraitClass, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:388
	// ("cv::linemod::Detector::readClass", vec![(pred!(mut, ["fn", "class_id_override"], ["const cv::FileNode*", "const cv::String*"]), _)]),
	void cv_linemod_Detector_readClass_const_FileNodeR_const_StringR(cv::linemod::Detector* instance, const cv::FileNode* fn, const char* class_id_override, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->readClass(*fn, std::string(class_id_override));
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::linemod::Detector::readClass(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:388
	// ("cv::linemod::Detector::readClass", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	void cv_linemod_Detector_readClass_const_FileNodeR(cv::linemod::Detector* instance, const cv::FileNode* fn, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->readClass(*fn);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// writeClass(const String &, FileStorage &)(InString, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:389
	// ("cv::linemod::Detector::writeClass", vec![(pred!(const, ["class_id", "fs"], ["const cv::String*", "cv::FileStorage*"]), _)]),
	void cv_linemod_Detector_writeClass_const_const_StringR_FileStorageR(const cv::linemod::Detector* instance, const char* class_id, cv::FileStorage* fs, ResultVoid* ocvrs_return) {
		try {
			instance->writeClass(std::string(class_id), *fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readClasses(const std::vector<String> &, const String &)(CppPassByVoidPtr, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:391
	// ("cv::linemod::Detector::readClasses", vec![(pred!(mut, ["class_ids", "format"], ["const std::vector<cv::String>*", "const cv::String*"]), _)]),
	void cv_linemod_Detector_readClasses_const_vectorLStringGR_const_StringR(cv::linemod::Detector* instance, const std::vector<cv::String>* class_ids, const char* format, ResultVoid* ocvrs_return) {
		try {
			instance->readClasses(*class_ids, std::string(format));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::linemod::Detector::readClasses(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:391
	// ("cv::linemod::Detector::readClasses", vec![(pred!(mut, ["class_ids"], ["const std::vector<cv::String>*"]), _)]),
	void cv_linemod_Detector_readClasses_const_vectorLStringGR(cv::linemod::Detector* instance, const std::vector<cv::String>* class_ids, ResultVoid* ocvrs_return) {
		try {
			instance->readClasses(*class_ids);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// writeClasses(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:393
	// ("cv::linemod::Detector::writeClasses", vec![(pred!(const, ["format"], ["const cv::String*"]), _)]),
	void cv_linemod_Detector_writeClasses_const_const_StringR(const cv::linemod::Detector* instance, const char* format, ResultVoid* ocvrs_return) {
		try {
			instance->writeClasses(std::string(format));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::linemod::Detector::writeClasses() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:393
	// ("cv::linemod::Detector::writeClasses", vec![(pred!(const, [], []), _)]),
	void cv_linemod_Detector_writeClasses_const(const cv::linemod::Detector* instance, ResultVoid* ocvrs_return) {
		try {
			instance->writeClasses();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::linemod::Detector::delete() generated
	// ("cv::linemod::Detector::delete", vec![(pred!(mut, [], []), _)]),
	void cv_linemod_Detector_delete(cv::linemod::Detector* instance) {
			delete instance;
	}

	// Feature()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:32
	// ("cv::linemod::Feature::Feature", vec![(pred!(mut, [], []), _)]),
	void cv_linemod_Feature_Feature(Result<cv::linemod::Feature>* ocvrs_return) {
		try {
			cv::linemod::Feature ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Feature(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:33
	// ("cv::linemod::Feature::Feature", vec![(pred!(mut, ["x", "y", "label"], ["int", "int", "int"]), _)]),
	void cv_linemod_Feature_Feature_int_int_int(int x, int y, int label, Result<cv::linemod::Feature>* ocvrs_return) {
		try {
			cv::linemod::Feature ret(x, y, label);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:35
	// ("cv::linemod::Feature::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	void cv_linemod_Feature_read_const_FileNodeR(cv::linemod::Feature* instance, const cv::FileNode* fn, ResultVoid* ocvrs_return) {
		try {
			instance->read(*fn);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:36
	// ("cv::linemod::Feature::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	void cv_linemod_Feature_write_const_FileStorageR(const cv::linemod::Feature* instance, cv::FileStorage* fs, ResultVoid* ocvrs_return) {
		try {
			instance->write(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Match()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:261
	// ("cv::linemod::Match::Match", vec![(pred!(mut, [], []), _)]),
	void cv_linemod_Match_Match(Result<cv::linemod::Match*>* ocvrs_return) {
		try {
			cv::linemod::Match* ret = new cv::linemod::Match();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Match(int, int, float, const String &, int)(Primitive, Primitive, Primitive, InString, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:265
	// ("cv::linemod::Match::Match", vec![(pred!(mut, ["x", "y", "similarity", "class_id", "template_id"], ["int", "int", "float", "const cv::String*", "int"]), _)]),
	void cv_linemod_Match_Match_int_int_float_const_StringR_int(int x, int y, float similarity, const char* class_id, int template_id, Result<cv::linemod::Match*>* ocvrs_return) {
		try {
			cv::linemod::Match* ret = new cv::linemod::Match(x, y, similarity, std::string(class_id), template_id);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator<(const Match &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:268
	// ("cv::linemod::Match::operator<", vec![(pred!(const, ["rhs"], ["const cv::linemod::Match*"]), _)]),
	void cv_linemod_Match_operatorL_const_const_MatchR(const cv::linemod::Match* instance, const cv::linemod::Match* rhs, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->operator<(*rhs);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator==(const Match &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:277
	// ("cv::linemod::Match::operator==", vec![(pred!(const, ["rhs"], ["const cv::linemod::Match*"]), _)]),
	void cv_linemod_Match_operatorEQ_const_const_MatchR(const cv::linemod::Match* instance, const cv::linemod::Match* rhs, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->operator==(*rhs);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::linemod::Match::implicitClone() generated
	// ("cv::linemod::Match::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::linemod::Match* cv_linemod_Match_implicitClone_const(const cv::linemod::Match* instance) {
			return new cv::linemod::Match(*instance);
	}

	// cv::linemod::Match::x() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:282
	// ("cv::linemod::Match::x", vec![(pred!(const, [], []), _)]),
	int cv_linemod_Match_propX_const(const cv::linemod::Match* instance) {
			int ret = instance->x;
			return ret;
	}

	// cv::linemod::Match::setX(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:282
	// ("cv::linemod::Match::setX", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_linemod_Match_propX_const_int(cv::linemod::Match* instance, const int val) {
			instance->x = val;
	}

	// cv::linemod::Match::y() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:283
	// ("cv::linemod::Match::y", vec![(pred!(const, [], []), _)]),
	int cv_linemod_Match_propY_const(const cv::linemod::Match* instance) {
			int ret = instance->y;
			return ret;
	}

	// cv::linemod::Match::setY(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:283
	// ("cv::linemod::Match::setY", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_linemod_Match_propY_const_int(cv::linemod::Match* instance, const int val) {
			instance->y = val;
	}

	// cv::linemod::Match::similarity() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:284
	// ("cv::linemod::Match::similarity", vec![(pred!(const, [], []), _)]),
	float cv_linemod_Match_propSimilarity_const(const cv::linemod::Match* instance) {
			float ret = instance->similarity;
			return ret;
	}

	// cv::linemod::Match::setSimilarity(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:284
	// ("cv::linemod::Match::setSimilarity", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_linemod_Match_propSimilarity_const_float(cv::linemod::Match* instance, const float val) {
			instance->similarity = val;
	}

	// cv::linemod::Match::class_id() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:285
	// ("cv::linemod::Match::class_id", vec![(pred!(const, [], []), _)]),
	void* cv_linemod_Match_propClass_id_const(const cv::linemod::Match* instance) {
			cv::String ret = instance->class_id;
			return ocvrs_create_string(ret.c_str());
	}

	// cv::linemod::Match::setClass_id(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:285
	// ("cv::linemod::Match::setClass_id", vec![(pred!(mut, ["val"], ["const cv::String"]), _)]),
	void cv_linemod_Match_propClass_id_const_String(cv::linemod::Match* instance, const char* val) {
			instance->class_id = std::string(val);
	}

	// cv::linemod::Match::template_id() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:286
	// ("cv::linemod::Match::template_id", vec![(pred!(const, [], []), _)]),
	int cv_linemod_Match_propTemplate_id_const(const cv::linemod::Match* instance) {
			int ret = instance->template_id;
			return ret;
	}

	// cv::linemod::Match::setTemplate_id(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:286
	// ("cv::linemod::Match::setTemplate_id", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_linemod_Match_propTemplate_id_const_int(cv::linemod::Match* instance, const int val) {
			instance->template_id = val;
	}

	// cv::linemod::Match::delete() generated
	// ("cv::linemod::Match::delete", vec![(pred!(mut, [], []), _)]),
	void cv_linemod_Match_delete(cv::linemod::Match* instance) {
			delete instance;
	}

	// process(const Mat &, const Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:132
	// ("cv::linemod::Modality::process", vec![(pred!(const, ["src", "mask"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
	void cv_linemod_Modality_process_const_const_MatR_const_MatR(const cv::linemod::Modality* instance, const cv::Mat* src, const cv::Mat* mask, Result<cv::Ptr<cv::linemod::QuantizedPyramid>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::linemod::QuantizedPyramid> ret = instance->process(*src, *mask);
			Ok(new cv::Ptr<cv::linemod::QuantizedPyramid>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::linemod::Modality::process(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:132
	// ("cv::linemod::Modality::process", vec![(pred!(const, ["src"], ["const cv::Mat*"]), _)]),
	void cv_linemod_Modality_process_const_const_MatR(const cv::linemod::Modality* instance, const cv::Mat* src, Result<cv::Ptr<cv::linemod::QuantizedPyramid>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::linemod::QuantizedPyramid> ret = instance->process(*src);
			Ok(new cv::Ptr<cv::linemod::QuantizedPyramid>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// name()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:138
	// ("cv::linemod::Modality::name", vec![(pred!(const, [], []), _)]),
	void cv_linemod_Modality_name_const(const cv::linemod::Modality* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->name();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:140
	// ("cv::linemod::Modality::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	void cv_linemod_Modality_read_const_FileNodeR(cv::linemod::Modality* instance, const cv::FileNode* fn, ResultVoid* ocvrs_return) {
		try {
			instance->read(*fn);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:141
	// ("cv::linemod::Modality::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	void cv_linemod_Modality_write_const_FileStorageR(const cv::linemod::Modality* instance, cv::FileStorage* fs, ResultVoid* ocvrs_return) {
		try {
			instance->write(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:150
	// ("cv::linemod::Modality::create", vec![(pred!(mut, ["modality_type"], ["const cv::String*"]), _)]),
	void cv_linemod_Modality_create_const_StringR(const char* modality_type, Result<cv::Ptr<cv::linemod::Modality>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::linemod::Modality> ret = cv::linemod::Modality::create(std::string(modality_type));
			Ok(new cv::Ptr<cv::linemod::Modality>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:155
	// ("cv::linemod::Modality::create", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	void cv_linemod_Modality_create_const_FileNodeR(const cv::FileNode* fn, Result<cv::Ptr<cv::linemod::Modality>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::linemod::Modality> ret = cv::linemod::Modality::create(*fn);
			Ok(new cv::Ptr<cv::linemod::Modality>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::linemod::Modality::to_LineMod_ColorGradient() generated
	// ("cv::linemod::Modality::to_LineMod_ColorGradient", vec![(pred!(mut, [], []), _)]),
	cv::linemod::ColorGradient* cv_linemod_Modality_to_LineMod_ColorGradient(cv::linemod::Modality* instance) {
			return dynamic_cast<cv::linemod::ColorGradient*>(instance);
	}

	// cv::linemod::Modality::to_LineMod_DepthNormal() generated
	// ("cv::linemod::Modality::to_LineMod_DepthNormal", vec![(pred!(mut, [], []), _)]),
	cv::linemod::DepthNormal* cv_linemod_Modality_to_LineMod_DepthNormal(cv::linemod::Modality* instance) {
			return dynamic_cast<cv::linemod::DepthNormal*>(instance);
	}

	// cv::linemod::Modality::delete() generated
	// ("cv::linemod::Modality::delete", vec![(pred!(mut, [], []), _)]),
	void cv_linemod_Modality_delete(cv::linemod::Modality* instance) {
			delete instance;
	}

	// quantize(Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:67
	// ("cv::linemod::QuantizedPyramid::quantize", vec![(pred!(const, ["dst"], ["cv::Mat*"]), _)]),
	void cv_linemod_QuantizedPyramid_quantize_const_MatR(const cv::linemod::QuantizedPyramid* instance, cv::Mat* dst, ResultVoid* ocvrs_return) {
		try {
			instance->quantize(*dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// extractTemplate(Template &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:74
	// ("cv::linemod::QuantizedPyramid::extractTemplate", vec![(pred!(const, ["templ"], ["cv::linemod::Template*"]), _)]),
	void cv_linemod_QuantizedPyramid_extractTemplate_const_TemplateR(const cv::linemod::QuantizedPyramid* instance, cv::linemod::Template* templ, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->extractTemplate(*templ);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// pyrDown()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:81
	// ("cv::linemod::QuantizedPyramid::pyrDown", vec![(pred!(mut, [], []), _)]),
	void cv_linemod_QuantizedPyramid_pyrDown(cv::linemod::QuantizedPyramid* instance, ResultVoid* ocvrs_return) {
		try {
			instance->pyrDown();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::linemod::QuantizedPyramid::delete() generated
	// ("cv::linemod::QuantizedPyramid::delete", vec![(pred!(mut, [], []), _)]),
	void cv_linemod_QuantizedPyramid_delete(cv::linemod::QuantizedPyramid* instance) {
			delete instance;
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:48
	// ("cv::linemod::Template::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	void cv_linemod_Template_read_const_FileNodeR(cv::linemod::Template* instance, const cv::FileNode* fn, ResultVoid* ocvrs_return) {
		try {
			instance->read(*fn);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:49
	// ("cv::linemod::Template::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	void cv_linemod_Template_write_const_FileStorageR(const cv::linemod::Template* instance, cv::FileStorage* fs, ResultVoid* ocvrs_return) {
		try {
			instance->write(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::linemod::Template::implicitClone() generated
	// ("cv::linemod::Template::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::linemod::Template* cv_linemod_Template_implicitClone_const(const cv::linemod::Template* instance) {
			return new cv::linemod::Template(*instance);
	}

	// cv::linemod::Template::defaultNew() generated
	// ("cv::linemod::Template::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::linemod::Template* cv_linemod_Template_defaultNew_const() {
			cv::linemod::Template* ret = new cv::linemod::Template();
			return ret;
	}

	// cv::linemod::Template::width() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:43
	// ("cv::linemod::Template::width", vec![(pred!(const, [], []), _)]),
	int cv_linemod_Template_propWidth_const(const cv::linemod::Template* instance) {
			int ret = instance->width;
			return ret;
	}

	// cv::linemod::Template::setWidth(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:43
	// ("cv::linemod::Template::setWidth", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_linemod_Template_propWidth_const_int(cv::linemod::Template* instance, const int val) {
			instance->width = val;
	}

	// cv::linemod::Template::height() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:44
	// ("cv::linemod::Template::height", vec![(pred!(const, [], []), _)]),
	int cv_linemod_Template_propHeight_const(const cv::linemod::Template* instance) {
			int ret = instance->height;
			return ret;
	}

	// cv::linemod::Template::setHeight(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:44
	// ("cv::linemod::Template::setHeight", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_linemod_Template_propHeight_const_int(cv::linemod::Template* instance, const int val) {
			instance->height = val;
	}

	// cv::linemod::Template::pyramid_level() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:45
	// ("cv::linemod::Template::pyramid_level", vec![(pred!(const, [], []), _)]),
	int cv_linemod_Template_propPyramid_level_const(const cv::linemod::Template* instance) {
			int ret = instance->pyramid_level;
			return ret;
	}

	// cv::linemod::Template::setPyramid_level(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:45
	// ("cv::linemod::Template::setPyramid_level", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_linemod_Template_propPyramid_level_const_int(cv::linemod::Template* instance, const int val) {
			instance->pyramid_level = val;
	}

	// cv::linemod::Template::features() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:46
	// ("cv::linemod::Template::features", vec![(pred!(const, [], []), _)]),
	std::vector<cv::linemod::Feature>* cv_linemod_Template_propFeatures_const(const cv::linemod::Template* instance) {
			std::vector<cv::linemod::Feature> ret = instance->features;
			return new std::vector<cv::linemod::Feature>(ret);
	}

	// cv::linemod::Template::setFeatures(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/linemod.hpp:46
	// ("cv::linemod::Template::setFeatures", vec![(pred!(mut, ["val"], ["const std::vector<cv::linemod::Feature>"]), _)]),
	void cv_linemod_Template_propFeatures_const_vectorLFeatureG(cv::linemod::Template* instance, const std::vector<cv::linemod::Feature>* val) {
			instance->features = *val;
	}

	// cv::linemod::Template::delete() generated
	// ("cv::linemod::Template::delete", vec![(pred!(mut, [], []), _)]),
	void cv_linemod_Template_delete(cv::linemod::Template* instance) {
			delete instance;
	}

	// DepthCleaner()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:198
	// ("cv::rgbd::DepthCleaner::DepthCleaner", vec![(pred!(mut, [], []), _)]),
	void cv_rgbd_DepthCleaner_DepthCleaner(Result<cv::rgbd::DepthCleaner*>* ocvrs_return) {
		try {
			cv::rgbd::DepthCleaner* ret = new cv::rgbd::DepthCleaner();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// DepthCleaner(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:212
	// ("cv::rgbd::DepthCleaner::DepthCleaner", vec![(pred!(mut, ["depth", "window_size", "method"], ["int", "int", "int"]), _)]),
	void cv_rgbd_DepthCleaner_DepthCleaner_int_int_int(int depth, int window_size, int method, Result<cv::rgbd::DepthCleaner*>* ocvrs_return) {
		try {
			cv::rgbd::DepthCleaner* ret = new cv::rgbd::DepthCleaner(depth, window_size, method);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rgbd::DepthCleaner::DepthCleaner(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:212
	// ("cv::rgbd::DepthCleaner::DepthCleaner", vec![(pred!(mut, ["depth"], ["int"]), _)]),
	void cv_rgbd_DepthCleaner_DepthCleaner_int(int depth, Result<cv::rgbd::DepthCleaner*>* ocvrs_return) {
		try {
			cv::rgbd::DepthCleaner* ret = new cv::rgbd::DepthCleaner(depth);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:216
	// ("cv::rgbd::DepthCleaner::create", vec![(pred!(mut, ["depth", "window_size", "method"], ["int", "int", "int"]), _)]),
	void cv_rgbd_DepthCleaner_create_int_int_int(int depth, int window_size, int method, Result<cv::Ptr<cv::rgbd::DepthCleaner>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::rgbd::DepthCleaner> ret = cv::rgbd::DepthCleaner::create(depth, window_size, method);
			Ok(new cv::Ptr<cv::rgbd::DepthCleaner>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rgbd::DepthCleaner::create(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:216
	// ("cv::rgbd::DepthCleaner::create", vec![(pred!(mut, ["depth"], ["int"]), _)]),
	void cv_rgbd_DepthCleaner_create_int(int depth, Result<cv::Ptr<cv::rgbd::DepthCleaner>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::rgbd::DepthCleaner> ret = cv::rgbd::DepthCleaner::create(depth);
			Ok(new cv::Ptr<cv::rgbd::DepthCleaner>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator()(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:223
	// ("cv::rgbd::DepthCleaner::operator()", vec![(pred!(const, ["points", "depth"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_rgbd_DepthCleaner_operator___const_const__InputArrayR_const__OutputArrayR(const cv::rgbd::DepthCleaner* instance, const cv::_InputArray* points, const cv::_OutputArray* depth, ResultVoid* ocvrs_return) {
		try {
			instance->operator()(*points, *depth);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// initialize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:229
	// ("cv::rgbd::DepthCleaner::initialize", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_DepthCleaner_initialize_const(const cv::rgbd::DepthCleaner* instance, ResultVoid* ocvrs_return) {
		try {
			instance->initialize();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getWindowSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:231
	// ("cv::rgbd::DepthCleaner::getWindowSize", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_DepthCleaner_getWindowSize_const(const cv::rgbd::DepthCleaner* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getWindowSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWindowSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:235
	// ("cv::rgbd::DepthCleaner::setWindowSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_rgbd_DepthCleaner_setWindowSize_int(cv::rgbd::DepthCleaner* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setWindowSize(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDepth()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:239
	// ("cv::rgbd::DepthCleaner::getDepth", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_DepthCleaner_getDepth_const(const cv::rgbd::DepthCleaner* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getDepth();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDepth(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:243
	// ("cv::rgbd::DepthCleaner::setDepth", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_rgbd_DepthCleaner_setDepth_int(cv::rgbd::DepthCleaner* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setDepth(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMethod()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:247
	// ("cv::rgbd::DepthCleaner::getMethod", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_DepthCleaner_getMethod_const(const cv::rgbd::DepthCleaner* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMethod();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMethod(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:251
	// ("cv::rgbd::DepthCleaner::setMethod", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_rgbd_DepthCleaner_setMethod_int(cv::rgbd::DepthCleaner* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setMethod(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rgbd::DepthCleaner::to_Algorithm() generated
	// ("cv::rgbd::DepthCleaner::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_rgbd_DepthCleaner_to_Algorithm(cv::rgbd::DepthCleaner* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::rgbd::DepthCleaner::delete() generated
	// ("cv::rgbd::DepthCleaner::delete", vec![(pred!(mut, [], []), _)]),
	void cv_rgbd_DepthCleaner_delete(cv::rgbd::DepthCleaner* instance) {
			delete instance;
	}

	// FastICPOdometry()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1042
	// ("cv::rgbd::FastICPOdometry::FastICPOdometry", vec![(pred!(mut, [], []), _)]),
	void cv_rgbd_FastICPOdometry_FastICPOdometry(Result<cv::rgbd::FastICPOdometry*>* ocvrs_return) {
		try {
			cv::rgbd::FastICPOdometry* ret = new cv::rgbd::FastICPOdometry();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// FastICPOdometry(const Mat &, float, float, float, float, int, const std::vector<int> &)(TraitClass, Primitive, Primitive, Primitive, Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1054
	// ("cv::rgbd::FastICPOdometry::FastICPOdometry", vec![(pred!(mut, ["cameraMatrix", "maxDistDiff", "angleThreshold", "sigmaDepth", "sigmaSpatial", "kernelSize", "iterCounts"], ["const cv::Mat*", "float", "float", "float", "float", "int", "const std::vector<int>*"]), _)]),
	void cv_rgbd_FastICPOdometry_FastICPOdometry_const_MatR_float_float_float_float_int_const_vectorLintGR(const cv::Mat* cameraMatrix, float maxDistDiff, float angleThreshold, float sigmaDepth, float sigmaSpatial, int kernelSize, const std::vector<int>* iterCounts, Result<cv::rgbd::FastICPOdometry*>* ocvrs_return) {
		try {
			cv::rgbd::FastICPOdometry* ret = new cv::rgbd::FastICPOdometry(*cameraMatrix, maxDistDiff, angleThreshold, sigmaDepth, sigmaSpatial, kernelSize, *iterCounts);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rgbd::FastICPOdometry::FastICPOdometry(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1054
	// ("cv::rgbd::FastICPOdometry::FastICPOdometry", vec![(pred!(mut, ["cameraMatrix"], ["const cv::Mat*"]), _)]),
	void cv_rgbd_FastICPOdometry_FastICPOdometry_const_MatR(const cv::Mat* cameraMatrix, Result<cv::rgbd::FastICPOdometry*>* ocvrs_return) {
		try {
			cv::rgbd::FastICPOdometry* ret = new cv::rgbd::FastICPOdometry(*cameraMatrix);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const Mat &, float, float, float, float, int, const std::vector<int> &)(TraitClass, Primitive, Primitive, Primitive, Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1062
	// ("cv::rgbd::FastICPOdometry::create", vec![(pred!(mut, ["cameraMatrix", "maxDistDiff", "angleThreshold", "sigmaDepth", "sigmaSpatial", "kernelSize", "iterCounts"], ["const cv::Mat*", "float", "float", "float", "float", "int", "const std::vector<int>*"]), _)]),
	void cv_rgbd_FastICPOdometry_create_const_MatR_float_float_float_float_int_const_vectorLintGR(const cv::Mat* cameraMatrix, float maxDistDiff, float angleThreshold, float sigmaDepth, float sigmaSpatial, int kernelSize, const std::vector<int>* iterCounts, Result<cv::Ptr<cv::rgbd::FastICPOdometry>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::rgbd::FastICPOdometry> ret = cv::rgbd::FastICPOdometry::create(*cameraMatrix, maxDistDiff, angleThreshold, sigmaDepth, sigmaSpatial, kernelSize, *iterCounts);
			Ok(new cv::Ptr<cv::rgbd::FastICPOdometry>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rgbd::FastICPOdometry::create(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1062
	// ("cv::rgbd::FastICPOdometry::create", vec![(pred!(mut, ["cameraMatrix"], ["const cv::Mat*"]), _)]),
	void cv_rgbd_FastICPOdometry_create_const_MatR(const cv::Mat* cameraMatrix, Result<cv::Ptr<cv::rgbd::FastICPOdometry>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::rgbd::FastICPOdometry> ret = cv::rgbd::FastICPOdometry::create(*cameraMatrix);
			Ok(new cv::Ptr<cv::rgbd::FastICPOdometry>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// prepareFrameCache(Ptr<OdometryFrame> &, int)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1070
	// ("cv::rgbd::FastICPOdometry::prepareFrameCache", vec![(pred!(const, ["frame", "cacheType"], ["cv::Ptr<cv::rgbd::OdometryFrame>*", "int"]), _)]),
	void cv_rgbd_FastICPOdometry_prepareFrameCache_const_PtrLOdometryFrameGR_int(const cv::rgbd::FastICPOdometry* instance, cv::Ptr<cv::rgbd::OdometryFrame>* frame, int cacheType, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->prepareFrameCache(*frame, cacheType);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getCameraMatrix()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1072
	// ("cv::rgbd::FastICPOdometry::getCameraMatrix", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_FastICPOdometry_getCameraMatrix_const(const cv::rgbd::FastICPOdometry* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getCameraMatrix();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setCameraMatrix(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1076
	// ("cv::rgbd::FastICPOdometry::setCameraMatrix", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
	void cv_rgbd_FastICPOdometry_setCameraMatrix_const_MatR(cv::rgbd::FastICPOdometry* instance, const cv::Mat* val, ResultVoid* ocvrs_return) {
		try {
			instance->setCameraMatrix(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxDistDiff()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1080
	// ("cv::rgbd::FastICPOdometry::getMaxDistDiff", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_FastICPOdometry_getMaxDistDiff_const(const cv::rgbd::FastICPOdometry* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxDistDiff();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxDistDiff(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1084
	// ("cv::rgbd::FastICPOdometry::setMaxDistDiff", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_rgbd_FastICPOdometry_setMaxDistDiff_float(cv::rgbd::FastICPOdometry* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxDistDiff(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getAngleThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1088
	// ("cv::rgbd::FastICPOdometry::getAngleThreshold", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_FastICPOdometry_getAngleThreshold_const(const cv::rgbd::FastICPOdometry* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getAngleThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setAngleThreshold(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1092
	// ("cv::rgbd::FastICPOdometry::setAngleThreshold", vec![(pred!(mut, ["f"], ["float"]), _)]),
	void cv_rgbd_FastICPOdometry_setAngleThreshold_float(cv::rgbd::FastICPOdometry* instance, float f, ResultVoid* ocvrs_return) {
		try {
			instance->setAngleThreshold(f);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSigmaDepth()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1096
	// ("cv::rgbd::FastICPOdometry::getSigmaDepth", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_FastICPOdometry_getSigmaDepth_const(const cv::rgbd::FastICPOdometry* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getSigmaDepth();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSigmaDepth(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1100
	// ("cv::rgbd::FastICPOdometry::setSigmaDepth", vec![(pred!(mut, ["f"], ["float"]), _)]),
	void cv_rgbd_FastICPOdometry_setSigmaDepth_float(cv::rgbd::FastICPOdometry* instance, float f, ResultVoid* ocvrs_return) {
		try {
			instance->setSigmaDepth(f);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSigmaSpatial()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1104
	// ("cv::rgbd::FastICPOdometry::getSigmaSpatial", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_FastICPOdometry_getSigmaSpatial_const(const cv::rgbd::FastICPOdometry* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getSigmaSpatial();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSigmaSpatial(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1108
	// ("cv::rgbd::FastICPOdometry::setSigmaSpatial", vec![(pred!(mut, ["f"], ["float"]), _)]),
	void cv_rgbd_FastICPOdometry_setSigmaSpatial_float(cv::rgbd::FastICPOdometry* instance, float f, ResultVoid* ocvrs_return) {
		try {
			instance->setSigmaSpatial(f);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getKernelSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1112
	// ("cv::rgbd::FastICPOdometry::getKernelSize", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_FastICPOdometry_getKernelSize_const(const cv::rgbd::FastICPOdometry* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getKernelSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setKernelSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1116
	// ("cv::rgbd::FastICPOdometry::setKernelSize", vec![(pred!(mut, ["f"], ["int"]), _)]),
	void cv_rgbd_FastICPOdometry_setKernelSize_int(cv::rgbd::FastICPOdometry* instance, int f, ResultVoid* ocvrs_return) {
		try {
			instance->setKernelSize(f);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getIterationCounts()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1120
	// ("cv::rgbd::FastICPOdometry::getIterationCounts", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_FastICPOdometry_getIterationCounts_const(const cv::rgbd::FastICPOdometry* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getIterationCounts();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setIterationCounts(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1124
	// ("cv::rgbd::FastICPOdometry::setIterationCounts", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
	void cv_rgbd_FastICPOdometry_setIterationCounts_const_MatR(cv::rgbd::FastICPOdometry* instance, const cv::Mat* val, ResultVoid* ocvrs_return) {
		try {
			instance->setIterationCounts(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTransformType()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1128
	// ("cv::rgbd::FastICPOdometry::getTransformType", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_FastICPOdometry_getTransformType_const(const cv::rgbd::FastICPOdometry* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getTransformType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTransformType(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:1132
	// ("cv::rgbd::FastICPOdometry::setTransformType", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_rgbd_FastICPOdometry_setTransformType_int(cv::rgbd::FastICPOdometry* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setTransformType(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rgbd::FastICPOdometry::to_Algorithm() generated
	// ("cv::rgbd::FastICPOdometry::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_rgbd_FastICPOdometry_to_Algorithm(cv::rgbd::FastICPOdometry* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::rgbd::FastICPOdometry::to_Odometry() generated
	// ("cv::rgbd::FastICPOdometry::to_Odometry", vec![(pred!(mut, [], []), _)]),
	cv::rgbd::Odometry* cv_rgbd_FastICPOdometry_to_Odometry(cv::rgbd::FastICPOdometry* instance) {
			return dynamic_cast<cv::rgbd::Odometry*>(instance);
	}

	// cv::rgbd::FastICPOdometry::delete() generated
	// ("cv::rgbd::FastICPOdometry::delete", vec![(pred!(mut, [], []), _)]),
	void cv_rgbd_FastICPOdometry_delete(cv::rgbd::FastICPOdometry* instance) {
			delete instance;
	}

	// ICPOdometry()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:762
	// ("cv::rgbd::ICPOdometry::ICPOdometry", vec![(pred!(mut, [], []), _)]),
	void cv_rgbd_ICPOdometry_ICPOdometry(Result<cv::rgbd::ICPOdometry*>* ocvrs_return) {
		try {
			cv::rgbd::ICPOdometry* ret = new cv::rgbd::ICPOdometry();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ICPOdometry(const Mat &, float, float, float, float, const std::vector<int> &, int)(TraitClass, Primitive, Primitive, Primitive, Primitive, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:773
	// ("cv::rgbd::ICPOdometry::ICPOdometry", vec![(pred!(mut, ["cameraMatrix", "minDepth", "maxDepth", "maxDepthDiff", "maxPointsPart", "iterCounts", "transformType"], ["const cv::Mat*", "float", "float", "float", "float", "const std::vector<int>*", "int"]), _)]),
	void cv_rgbd_ICPOdometry_ICPOdometry_const_MatR_float_float_float_float_const_vectorLintGR_int(const cv::Mat* cameraMatrix, float minDepth, float maxDepth, float maxDepthDiff, float maxPointsPart, const std::vector<int>* iterCounts, int transformType, Result<cv::rgbd::ICPOdometry*>* ocvrs_return) {
		try {
			cv::rgbd::ICPOdometry* ret = new cv::rgbd::ICPOdometry(*cameraMatrix, minDepth, maxDepth, maxDepthDiff, maxPointsPart, *iterCounts, transformType);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rgbd::ICPOdometry::ICPOdometry(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:773
	// ("cv::rgbd::ICPOdometry::ICPOdometry", vec![(pred!(mut, ["cameraMatrix"], ["const cv::Mat*"]), _)]),
	void cv_rgbd_ICPOdometry_ICPOdometry_const_MatR(const cv::Mat* cameraMatrix, Result<cv::rgbd::ICPOdometry*>* ocvrs_return) {
		try {
			cv::rgbd::ICPOdometry* ret = new cv::rgbd::ICPOdometry(*cameraMatrix);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const Mat &, float, float, float, float, const std::vector<int> &, int)(TraitClass, Primitive, Primitive, Primitive, Primitive, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:777
	// ("cv::rgbd::ICPOdometry::create", vec![(pred!(mut, ["cameraMatrix", "minDepth", "maxDepth", "maxDepthDiff", "maxPointsPart", "iterCounts", "transformType"], ["const cv::Mat*", "float", "float", "float", "float", "const std::vector<int>*", "int"]), _)]),
	void cv_rgbd_ICPOdometry_create_const_MatR_float_float_float_float_const_vectorLintGR_int(const cv::Mat* cameraMatrix, float minDepth, float maxDepth, float maxDepthDiff, float maxPointsPart, const std::vector<int>* iterCounts, int transformType, Result<cv::Ptr<cv::rgbd::ICPOdometry>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::rgbd::ICPOdometry> ret = cv::rgbd::ICPOdometry::create(*cameraMatrix, minDepth, maxDepth, maxDepthDiff, maxPointsPart, *iterCounts, transformType);
			Ok(new cv::Ptr<cv::rgbd::ICPOdometry>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rgbd::ICPOdometry::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:777
	// ("cv::rgbd::ICPOdometry::create", vec![(pred!(mut, [], []), _)]),
	void cv_rgbd_ICPOdometry_create(Result<cv::Ptr<cv::rgbd::ICPOdometry>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::rgbd::ICPOdometry> ret = cv::rgbd::ICPOdometry::create();
			Ok(new cv::Ptr<cv::rgbd::ICPOdometry>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// prepareFrameCache(Ptr<OdometryFrame> &, int)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:781
	// ("cv::rgbd::ICPOdometry::prepareFrameCache", vec![(pred!(const, ["frame", "cacheType"], ["cv::Ptr<cv::rgbd::OdometryFrame>*", "int"]), _)]),
	void cv_rgbd_ICPOdometry_prepareFrameCache_const_PtrLOdometryFrameGR_int(const cv::rgbd::ICPOdometry* instance, cv::Ptr<cv::rgbd::OdometryFrame>* frame, int cacheType, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->prepareFrameCache(*frame, cacheType);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getCameraMatrix()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:783
	// ("cv::rgbd::ICPOdometry::getCameraMatrix", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_ICPOdometry_getCameraMatrix_const(const cv::rgbd::ICPOdometry* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getCameraMatrix();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setCameraMatrix(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:787
	// ("cv::rgbd::ICPOdometry::setCameraMatrix", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
	void cv_rgbd_ICPOdometry_setCameraMatrix_const_MatR(cv::rgbd::ICPOdometry* instance, const cv::Mat* val, ResultVoid* ocvrs_return) {
		try {
			instance->setCameraMatrix(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMinDepth()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:791
	// ("cv::rgbd::ICPOdometry::getMinDepth", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_ICPOdometry_getMinDepth_const(const cv::rgbd::ICPOdometry* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMinDepth();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMinDepth(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:795
	// ("cv::rgbd::ICPOdometry::setMinDepth", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_rgbd_ICPOdometry_setMinDepth_double(cv::rgbd::ICPOdometry* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setMinDepth(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxDepth()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:799
	// ("cv::rgbd::ICPOdometry::getMaxDepth", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_ICPOdometry_getMaxDepth_const(const cv::rgbd::ICPOdometry* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxDepth();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxDepth(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:803
	// ("cv::rgbd::ICPOdometry::setMaxDepth", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_rgbd_ICPOdometry_setMaxDepth_double(cv::rgbd::ICPOdometry* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxDepth(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxDepthDiff()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:807
	// ("cv::rgbd::ICPOdometry::getMaxDepthDiff", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_ICPOdometry_getMaxDepthDiff_const(const cv::rgbd::ICPOdometry* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxDepthDiff();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxDepthDiff(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:811
	// ("cv::rgbd::ICPOdometry::setMaxDepthDiff", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_rgbd_ICPOdometry_setMaxDepthDiff_double(cv::rgbd::ICPOdometry* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxDepthDiff(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getIterationCounts()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:815
	// ("cv::rgbd::ICPOdometry::getIterationCounts", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_ICPOdometry_getIterationCounts_const(const cv::rgbd::ICPOdometry* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getIterationCounts();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setIterationCounts(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:819
	// ("cv::rgbd::ICPOdometry::setIterationCounts", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
	void cv_rgbd_ICPOdometry_setIterationCounts_const_MatR(cv::rgbd::ICPOdometry* instance, const cv::Mat* val, ResultVoid* ocvrs_return) {
		try {
			instance->setIterationCounts(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxPointsPart()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:823
	// ("cv::rgbd::ICPOdometry::getMaxPointsPart", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_ICPOdometry_getMaxPointsPart_const(const cv::rgbd::ICPOdometry* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxPointsPart();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxPointsPart(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:827
	// ("cv::rgbd::ICPOdometry::setMaxPointsPart", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_rgbd_ICPOdometry_setMaxPointsPart_double(cv::rgbd::ICPOdometry* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxPointsPart(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTransformType()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:831
	// ("cv::rgbd::ICPOdometry::getTransformType", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_ICPOdometry_getTransformType_const(const cv::rgbd::ICPOdometry* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getTransformType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTransformType(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:835
	// ("cv::rgbd::ICPOdometry::setTransformType", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_rgbd_ICPOdometry_setTransformType_int(cv::rgbd::ICPOdometry* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setTransformType(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxTranslation()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:839
	// ("cv::rgbd::ICPOdometry::getMaxTranslation", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_ICPOdometry_getMaxTranslation_const(const cv::rgbd::ICPOdometry* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxTranslation();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxTranslation(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:843
	// ("cv::rgbd::ICPOdometry::setMaxTranslation", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_rgbd_ICPOdometry_setMaxTranslation_double(cv::rgbd::ICPOdometry* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxTranslation(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxRotation()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:847
	// ("cv::rgbd::ICPOdometry::getMaxRotation", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_ICPOdometry_getMaxRotation_const(const cv::rgbd::ICPOdometry* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxRotation();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxRotation(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:851
	// ("cv::rgbd::ICPOdometry::setMaxRotation", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_rgbd_ICPOdometry_setMaxRotation_double(cv::rgbd::ICPOdometry* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxRotation(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNormalsComputer()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:855
	// ("cv::rgbd::ICPOdometry::getNormalsComputer", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_ICPOdometry_getNormalsComputer_const(const cv::rgbd::ICPOdometry* instance, Result<cv::Ptr<cv::rgbd::RgbdNormals>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::rgbd::RgbdNormals> ret = instance->getNormalsComputer();
			Ok(new cv::Ptr<cv::rgbd::RgbdNormals>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rgbd::ICPOdometry::to_Algorithm() generated
	// ("cv::rgbd::ICPOdometry::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_rgbd_ICPOdometry_to_Algorithm(cv::rgbd::ICPOdometry* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::rgbd::ICPOdometry::to_Odometry() generated
	// ("cv::rgbd::ICPOdometry::to_Odometry", vec![(pred!(mut, [], []), _)]),
	cv::rgbd::Odometry* cv_rgbd_ICPOdometry_to_Odometry(cv::rgbd::ICPOdometry* instance) {
			return dynamic_cast<cv::rgbd::Odometry*>(instance);
	}

	// cv::rgbd::ICPOdometry::delete() generated
	// ("cv::rgbd::ICPOdometry::delete", vec![(pred!(mut, [], []), _)]),
	void cv_rgbd_ICPOdometry_delete(cv::rgbd::ICPOdometry* instance) {
			delete instance;
	}

	// DEFAULT_MIN_DEPTH()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:535
	// ("cv::rgbd::Odometry::DEFAULT_MIN_DEPTH", vec![(pred!(mut, [], []), _)]),
	void cv_rgbd_Odometry_DEFAULT_MIN_DEPTH(Result<float>* ocvrs_return) {
		try {
			float ret = cv::rgbd::Odometry::DEFAULT_MIN_DEPTH();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// DEFAULT_MAX_DEPTH()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:540
	// ("cv::rgbd::Odometry::DEFAULT_MAX_DEPTH", vec![(pred!(mut, [], []), _)]),
	void cv_rgbd_Odometry_DEFAULT_MAX_DEPTH(Result<float>* ocvrs_return) {
		try {
			float ret = cv::rgbd::Odometry::DEFAULT_MAX_DEPTH();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// DEFAULT_MAX_DEPTH_DIFF()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:545
	// ("cv::rgbd::Odometry::DEFAULT_MAX_DEPTH_DIFF", vec![(pred!(mut, [], []), _)]),
	void cv_rgbd_Odometry_DEFAULT_MAX_DEPTH_DIFF(Result<float>* ocvrs_return) {
		try {
			float ret = cv::rgbd::Odometry::DEFAULT_MAX_DEPTH_DIFF();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// DEFAULT_MAX_POINTS_PART()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:550
	// ("cv::rgbd::Odometry::DEFAULT_MAX_POINTS_PART", vec![(pred!(mut, [], []), _)]),
	void cv_rgbd_Odometry_DEFAULT_MAX_POINTS_PART(Result<float>* ocvrs_return) {
		try {
			float ret = cv::rgbd::Odometry::DEFAULT_MAX_POINTS_PART();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// DEFAULT_MAX_TRANSLATION()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:555
	// ("cv::rgbd::Odometry::DEFAULT_MAX_TRANSLATION", vec![(pred!(mut, [], []), _)]),
	void cv_rgbd_Odometry_DEFAULT_MAX_TRANSLATION(Result<float>* ocvrs_return) {
		try {
			float ret = cv::rgbd::Odometry::DEFAULT_MAX_TRANSLATION();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// DEFAULT_MAX_ROTATION()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:560
	// ("cv::rgbd::Odometry::DEFAULT_MAX_ROTATION", vec![(pred!(mut, [], []), _)]),
	void cv_rgbd_Odometry_DEFAULT_MAX_ROTATION(Result<float>* ocvrs_return) {
		try {
			float ret = cv::rgbd::Odometry::DEFAULT_MAX_ROTATION();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// compute(const Mat &, const Mat &, const Mat &, const Mat &, const Mat &, const Mat &, OutputArray, const Mat &)(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:584
	// ("cv::rgbd::Odometry::compute", vec![(pred!(const, ["srcImage", "srcDepth", "srcMask", "dstImage", "dstDepth", "dstMask", "Rt", "initRt"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::_OutputArray*", "const cv::Mat*"]), _)]),
	void cv_rgbd_Odometry_compute_const_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_const__OutputArrayR_const_MatR(const cv::rgbd::Odometry* instance, const cv::Mat* srcImage, const cv::Mat* srcDepth, const cv::Mat* srcMask, const cv::Mat* dstImage, const cv::Mat* dstDepth, const cv::Mat* dstMask, const cv::_OutputArray* Rt, const cv::Mat* initRt, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->compute(*srcImage, *srcDepth, *srcMask, *dstImage, *dstDepth, *dstMask, *Rt, *initRt);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rgbd::Odometry::compute(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:584
	// ("cv::rgbd::Odometry::compute", vec![(pred!(const, ["srcImage", "srcDepth", "srcMask", "dstImage", "dstDepth", "dstMask", "Rt"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::_OutputArray*"]), _)]),
	void cv_rgbd_Odometry_compute_const_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_const__OutputArrayR(const cv::rgbd::Odometry* instance, const cv::Mat* srcImage, const cv::Mat* srcDepth, const cv::Mat* srcMask, const cv::Mat* dstImage, const cv::Mat* dstDepth, const cv::Mat* dstMask, const cv::_OutputArray* Rt, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->compute(*srcImage, *srcDepth, *srcMask, *dstImage, *dstDepth, *dstMask, *Rt);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// compute(Ptr<OdometryFrame> &, Ptr<OdometryFrame> &, OutputArray, const Mat &)(CppPassByVoidPtr, CppPassByVoidPtr, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:591
	// ("cv::rgbd::Odometry::compute", vec![(pred!(const, ["srcFrame", "dstFrame", "Rt", "initRt"], ["cv::Ptr<cv::rgbd::OdometryFrame>*", "cv::Ptr<cv::rgbd::OdometryFrame>*", "const cv::_OutputArray*", "const cv::Mat*"]), _)]),
	void cv_rgbd_Odometry_compute_const_PtrLOdometryFrameGR_PtrLOdometryFrameGR_const__OutputArrayR_const_MatR(const cv::rgbd::Odometry* instance, cv::Ptr<cv::rgbd::OdometryFrame>* srcFrame, cv::Ptr<cv::rgbd::OdometryFrame>* dstFrame, const cv::_OutputArray* Rt, const cv::Mat* initRt, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->compute(*srcFrame, *dstFrame, *Rt, *initRt);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rgbd::Odometry::compute(CppPassByVoidPtr, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:591
	// ("cv::rgbd::Odometry::compute", vec![(pred!(const, ["srcFrame", "dstFrame", "Rt"], ["cv::Ptr<cv::rgbd::OdometryFrame>*", "cv::Ptr<cv::rgbd::OdometryFrame>*", "const cv::_OutputArray*"]), _)]),
	void cv_rgbd_Odometry_compute_const_PtrLOdometryFrameGR_PtrLOdometryFrameGR_const__OutputArrayR(const cv::rgbd::Odometry* instance, cv::Ptr<cv::rgbd::OdometryFrame>* srcFrame, cv::Ptr<cv::rgbd::OdometryFrame>* dstFrame, const cv::_OutputArray* Rt, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->compute(*srcFrame, *dstFrame, *Rt);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// prepareFrameCache(Ptr<OdometryFrame> &, int)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:599
	// ("cv::rgbd::Odometry::prepareFrameCache", vec![(pred!(const, ["frame", "cacheType"], ["cv::Ptr<cv::rgbd::OdometryFrame>*", "int"]), _)]),
	void cv_rgbd_Odometry_prepareFrameCache_const_PtrLOdometryFrameGR_int(const cv::rgbd::Odometry* instance, cv::Ptr<cv::rgbd::OdometryFrame>* frame, int cacheType, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->prepareFrameCache(*frame, cacheType);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:601
	// ("cv::rgbd::Odometry::create", vec![(pred!(mut, ["odometryType"], ["const cv::String*"]), _)]),
	void cv_rgbd_Odometry_create_const_StringR(const char* odometryType, Result<cv::Ptr<cv::rgbd::Odometry>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::rgbd::Odometry> ret = cv::rgbd::Odometry::create(std::string(odometryType));
			Ok(new cv::Ptr<cv::rgbd::Odometry>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getCameraMatrix()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:604
	// ("cv::rgbd::Odometry::getCameraMatrix", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_Odometry_getCameraMatrix_const(const cv::rgbd::Odometry* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getCameraMatrix();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setCameraMatrix(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:606
	// ("cv::rgbd::Odometry::setCameraMatrix", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
	void cv_rgbd_Odometry_setCameraMatrix_const_MatR(cv::rgbd::Odometry* instance, const cv::Mat* val, ResultVoid* ocvrs_return) {
		try {
			instance->setCameraMatrix(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTransformType()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:608
	// ("cv::rgbd::Odometry::getTransformType", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_Odometry_getTransformType_const(const cv::rgbd::Odometry* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getTransformType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTransformType(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:610
	// ("cv::rgbd::Odometry::setTransformType", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_rgbd_Odometry_setTransformType_int(cv::rgbd::Odometry* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setTransformType(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rgbd::Odometry::to_FastICPOdometry() generated
	// ("cv::rgbd::Odometry::to_FastICPOdometry", vec![(pred!(mut, [], []), _)]),
	cv::rgbd::FastICPOdometry* cv_rgbd_Odometry_to_FastICPOdometry(cv::rgbd::Odometry* instance) {
			return dynamic_cast<cv::rgbd::FastICPOdometry*>(instance);
	}

	// cv::rgbd::Odometry::to_ICPOdometry() generated
	// ("cv::rgbd::Odometry::to_ICPOdometry", vec![(pred!(mut, [], []), _)]),
	cv::rgbd::ICPOdometry* cv_rgbd_Odometry_to_ICPOdometry(cv::rgbd::Odometry* instance) {
			return dynamic_cast<cv::rgbd::ICPOdometry*>(instance);
	}

	// cv::rgbd::Odometry::to_RgbdICPOdometry() generated
	// ("cv::rgbd::Odometry::to_RgbdICPOdometry", vec![(pred!(mut, [], []), _)]),
	cv::rgbd::RgbdICPOdometry* cv_rgbd_Odometry_to_RgbdICPOdometry(cv::rgbd::Odometry* instance) {
			return dynamic_cast<cv::rgbd::RgbdICPOdometry*>(instance);
	}

	// cv::rgbd::Odometry::to_RgbdOdometry() generated
	// ("cv::rgbd::Odometry::to_RgbdOdometry", vec![(pred!(mut, [], []), _)]),
	cv::rgbd::RgbdOdometry* cv_rgbd_Odometry_to_RgbdOdometry(cv::rgbd::Odometry* instance) {
			return dynamic_cast<cv::rgbd::RgbdOdometry*>(instance);
	}

	// cv::rgbd::Odometry::to_Algorithm() generated
	// ("cv::rgbd::Odometry::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_rgbd_Odometry_to_Algorithm(cv::rgbd::Odometry* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::rgbd::Odometry::delete() generated
	// ("cv::rgbd::Odometry::delete", vec![(pred!(mut, [], []), _)]),
	void cv_rgbd_Odometry_delete(cv::rgbd::Odometry* instance) {
			delete instance;
	}

	// OdometryFrame()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:497
	// ("cv::rgbd::OdometryFrame::OdometryFrame", vec![(pred!(mut, [], []), _)]),
	void cv_rgbd_OdometryFrame_OdometryFrame(Result<cv::rgbd::OdometryFrame*>* ocvrs_return) {
		try {
			cv::rgbd::OdometryFrame* ret = new cv::rgbd::OdometryFrame();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// OdometryFrame(const Mat &, const Mat &, const Mat &, const Mat &, int)(TraitClass, TraitClass, TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:498
	// ("cv::rgbd::OdometryFrame::OdometryFrame", vec![(pred!(mut, ["image", "depth", "mask", "normals", "ID"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "int"]), _)]),
	void cv_rgbd_OdometryFrame_OdometryFrame_const_MatR_const_MatR_const_MatR_const_MatR_int(const cv::Mat* image, const cv::Mat* depth, const cv::Mat* mask, const cv::Mat* normals, int ID, Result<cv::rgbd::OdometryFrame*>* ocvrs_return) {
		try {
			cv::rgbd::OdometryFrame* ret = new cv::rgbd::OdometryFrame(*image, *depth, *mask, *normals, ID);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rgbd::OdometryFrame::OdometryFrame(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:498
	// ("cv::rgbd::OdometryFrame::OdometryFrame", vec![(pred!(mut, ["image", "depth"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
	void cv_rgbd_OdometryFrame_OdometryFrame_const_MatR_const_MatR(const cv::Mat* image, const cv::Mat* depth, Result<cv::rgbd::OdometryFrame*>* ocvrs_return) {
		try {
			cv::rgbd::OdometryFrame* ret = new cv::rgbd::OdometryFrame(*image, *depth);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const Mat &, const Mat &, const Mat &, const Mat &, int)(TraitClass, TraitClass, TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:500
	// ("cv::rgbd::OdometryFrame::create", vec![(pred!(mut, ["image", "depth", "mask", "normals", "ID"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "int"]), _)]),
	void cv_rgbd_OdometryFrame_create_const_MatR_const_MatR_const_MatR_const_MatR_int(const cv::Mat* image, const cv::Mat* depth, const cv::Mat* mask, const cv::Mat* normals, int ID, Result<cv::Ptr<cv::rgbd::OdometryFrame>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::rgbd::OdometryFrame> ret = cv::rgbd::OdometryFrame::create(*image, *depth, *mask, *normals, ID);
			Ok(new cv::Ptr<cv::rgbd::OdometryFrame>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rgbd::OdometryFrame::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:500
	// ("cv::rgbd::OdometryFrame::create", vec![(pred!(mut, [], []), _)]),
	void cv_rgbd_OdometryFrame_create(Result<cv::Ptr<cv::rgbd::OdometryFrame>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::rgbd::OdometryFrame> ret = cv::rgbd::OdometryFrame::create();
			Ok(new cv::Ptr<cv::rgbd::OdometryFrame>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// release()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:503
	// ("cv::rgbd::OdometryFrame::release", vec![(pred!(mut, [], []), _)]),
	void cv_rgbd_OdometryFrame_release(cv::rgbd::OdometryFrame* instance, ResultVoid* ocvrs_return) {
		try {
			instance->release();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// releasePyramids()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:506
	// ("cv::rgbd::OdometryFrame::releasePyramids", vec![(pred!(mut, [], []), _)]),
	void cv_rgbd_OdometryFrame_releasePyramids(cv::rgbd::OdometryFrame* instance, ResultVoid* ocvrs_return) {
		try {
			instance->releasePyramids();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rgbd::OdometryFrame::pyramidImage() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:508
	// ("cv::rgbd::OdometryFrame::pyramidImage", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Mat>* cv_rgbd_OdometryFrame_propPyramidImage_const(const cv::rgbd::OdometryFrame* instance) {
			std::vector<cv::Mat> ret = instance->pyramidImage;
			return new std::vector<cv::Mat>(ret);
	}

	// cv::rgbd::OdometryFrame::setPyramidImage(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:508
	// ("cv::rgbd::OdometryFrame::setPyramidImage", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
	void cv_rgbd_OdometryFrame_propPyramidImage_const_vectorLMatG(cv::rgbd::OdometryFrame* instance, const std::vector<cv::Mat>* val) {
			instance->pyramidImage = *val;
	}

	// cv::rgbd::OdometryFrame::pyramidDepth() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:509
	// ("cv::rgbd::OdometryFrame::pyramidDepth", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Mat>* cv_rgbd_OdometryFrame_propPyramidDepth_const(const cv::rgbd::OdometryFrame* instance) {
			std::vector<cv::Mat> ret = instance->pyramidDepth;
			return new std::vector<cv::Mat>(ret);
	}

	// cv::rgbd::OdometryFrame::setPyramidDepth(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:509
	// ("cv::rgbd::OdometryFrame::setPyramidDepth", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
	void cv_rgbd_OdometryFrame_propPyramidDepth_const_vectorLMatG(cv::rgbd::OdometryFrame* instance, const std::vector<cv::Mat>* val) {
			instance->pyramidDepth = *val;
	}

	// cv::rgbd::OdometryFrame::pyramidMask() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:510
	// ("cv::rgbd::OdometryFrame::pyramidMask", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Mat>* cv_rgbd_OdometryFrame_propPyramidMask_const(const cv::rgbd::OdometryFrame* instance) {
			std::vector<cv::Mat> ret = instance->pyramidMask;
			return new std::vector<cv::Mat>(ret);
	}

	// cv::rgbd::OdometryFrame::setPyramidMask(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:510
	// ("cv::rgbd::OdometryFrame::setPyramidMask", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
	void cv_rgbd_OdometryFrame_propPyramidMask_const_vectorLMatG(cv::rgbd::OdometryFrame* instance, const std::vector<cv::Mat>* val) {
			instance->pyramidMask = *val;
	}

	// cv::rgbd::OdometryFrame::pyramidCloud() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:512
	// ("cv::rgbd::OdometryFrame::pyramidCloud", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Mat>* cv_rgbd_OdometryFrame_propPyramidCloud_const(const cv::rgbd::OdometryFrame* instance) {
			std::vector<cv::Mat> ret = instance->pyramidCloud;
			return new std::vector<cv::Mat>(ret);
	}

	// cv::rgbd::OdometryFrame::setPyramidCloud(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:512
	// ("cv::rgbd::OdometryFrame::setPyramidCloud", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
	void cv_rgbd_OdometryFrame_propPyramidCloud_const_vectorLMatG(cv::rgbd::OdometryFrame* instance, const std::vector<cv::Mat>* val) {
			instance->pyramidCloud = *val;
	}

	// cv::rgbd::OdometryFrame::pyramid_dI_dx() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:514
	// ("cv::rgbd::OdometryFrame::pyramid_dI_dx", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Mat>* cv_rgbd_OdometryFrame_propPyramid_dI_dx_const(const cv::rgbd::OdometryFrame* instance) {
			std::vector<cv::Mat> ret = instance->pyramid_dI_dx;
			return new std::vector<cv::Mat>(ret);
	}

	// cv::rgbd::OdometryFrame::setPyramid_dI_dx(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:514
	// ("cv::rgbd::OdometryFrame::setPyramid_dI_dx", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
	void cv_rgbd_OdometryFrame_propPyramid_dI_dx_const_vectorLMatG(cv::rgbd::OdometryFrame* instance, const std::vector<cv::Mat>* val) {
			instance->pyramid_dI_dx = *val;
	}

	// cv::rgbd::OdometryFrame::pyramid_dI_dy() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:515
	// ("cv::rgbd::OdometryFrame::pyramid_dI_dy", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Mat>* cv_rgbd_OdometryFrame_propPyramid_dI_dy_const(const cv::rgbd::OdometryFrame* instance) {
			std::vector<cv::Mat> ret = instance->pyramid_dI_dy;
			return new std::vector<cv::Mat>(ret);
	}

	// cv::rgbd::OdometryFrame::setPyramid_dI_dy(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:515
	// ("cv::rgbd::OdometryFrame::setPyramid_dI_dy", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
	void cv_rgbd_OdometryFrame_propPyramid_dI_dy_const_vectorLMatG(cv::rgbd::OdometryFrame* instance, const std::vector<cv::Mat>* val) {
			instance->pyramid_dI_dy = *val;
	}

	// cv::rgbd::OdometryFrame::pyramidTexturedMask() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:516
	// ("cv::rgbd::OdometryFrame::pyramidTexturedMask", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Mat>* cv_rgbd_OdometryFrame_propPyramidTexturedMask_const(const cv::rgbd::OdometryFrame* instance) {
			std::vector<cv::Mat> ret = instance->pyramidTexturedMask;
			return new std::vector<cv::Mat>(ret);
	}

	// cv::rgbd::OdometryFrame::setPyramidTexturedMask(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:516
	// ("cv::rgbd::OdometryFrame::setPyramidTexturedMask", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
	void cv_rgbd_OdometryFrame_propPyramidTexturedMask_const_vectorLMatG(cv::rgbd::OdometryFrame* instance, const std::vector<cv::Mat>* val) {
			instance->pyramidTexturedMask = *val;
	}

	// cv::rgbd::OdometryFrame::pyramidNormals() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:518
	// ("cv::rgbd::OdometryFrame::pyramidNormals", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Mat>* cv_rgbd_OdometryFrame_propPyramidNormals_const(const cv::rgbd::OdometryFrame* instance) {
			std::vector<cv::Mat> ret = instance->pyramidNormals;
			return new std::vector<cv::Mat>(ret);
	}

	// cv::rgbd::OdometryFrame::setPyramidNormals(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:518
	// ("cv::rgbd::OdometryFrame::setPyramidNormals", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
	void cv_rgbd_OdometryFrame_propPyramidNormals_const_vectorLMatG(cv::rgbd::OdometryFrame* instance, const std::vector<cv::Mat>* val) {
			instance->pyramidNormals = *val;
	}

	// cv::rgbd::OdometryFrame::pyramidNormalsMask() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:519
	// ("cv::rgbd::OdometryFrame::pyramidNormalsMask", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Mat>* cv_rgbd_OdometryFrame_propPyramidNormalsMask_const(const cv::rgbd::OdometryFrame* instance) {
			std::vector<cv::Mat> ret = instance->pyramidNormalsMask;
			return new std::vector<cv::Mat>(ret);
	}

	// cv::rgbd::OdometryFrame::setPyramidNormalsMask(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:519
	// ("cv::rgbd::OdometryFrame::setPyramidNormalsMask", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
	void cv_rgbd_OdometryFrame_propPyramidNormalsMask_const_vectorLMatG(cv::rgbd::OdometryFrame* instance, const std::vector<cv::Mat>* val) {
			instance->pyramidNormalsMask = *val;
	}

	// cv::rgbd::OdometryFrame::to_RgbdFrame() generated
	// ("cv::rgbd::OdometryFrame::to_RgbdFrame", vec![(pred!(mut, [], []), _)]),
	cv::rgbd::RgbdFrame* cv_rgbd_OdometryFrame_to_RgbdFrame(cv::rgbd::OdometryFrame* instance) {
			return dynamic_cast<cv::rgbd::RgbdFrame*>(instance);
	}

	// cv::rgbd::OdometryFrame::delete() generated
	// ("cv::rgbd::OdometryFrame::delete", vec![(pred!(mut, [], []), _)]),
	void cv_rgbd_OdometryFrame_delete(cv::rgbd::OdometryFrame* instance) {
			delete instance;
	}

	// RgbdFrame()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:463
	// ("cv::rgbd::RgbdFrame::RgbdFrame", vec![(pred!(mut, [], []), _)]),
	void cv_rgbd_RgbdFrame_RgbdFrame(Result<cv::rgbd::RgbdFrame*>* ocvrs_return) {
		try {
			cv::rgbd::RgbdFrame* ret = new cv::rgbd::RgbdFrame();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// RgbdFrame(const Mat &, const Mat &, const Mat &, const Mat &, int)(TraitClass, TraitClass, TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:464
	// ("cv::rgbd::RgbdFrame::RgbdFrame", vec![(pred!(mut, ["image", "depth", "mask", "normals", "ID"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "int"]), _)]),
	void cv_rgbd_RgbdFrame_RgbdFrame_const_MatR_const_MatR_const_MatR_const_MatR_int(const cv::Mat* image, const cv::Mat* depth, const cv::Mat* mask, const cv::Mat* normals, int ID, Result<cv::rgbd::RgbdFrame*>* ocvrs_return) {
		try {
			cv::rgbd::RgbdFrame* ret = new cv::rgbd::RgbdFrame(*image, *depth, *mask, *normals, ID);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rgbd::RgbdFrame::RgbdFrame(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:464
	// ("cv::rgbd::RgbdFrame::RgbdFrame", vec![(pred!(mut, ["image", "depth"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
	void cv_rgbd_RgbdFrame_RgbdFrame_const_MatR_const_MatR(const cv::Mat* image, const cv::Mat* depth, Result<cv::rgbd::RgbdFrame*>* ocvrs_return) {
		try {
			cv::rgbd::RgbdFrame* ret = new cv::rgbd::RgbdFrame(*image, *depth);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const Mat &, const Mat &, const Mat &, const Mat &, int)(TraitClass, TraitClass, TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:467
	// ("cv::rgbd::RgbdFrame::create", vec![(pred!(mut, ["image", "depth", "mask", "normals", "ID"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "int"]), _)]),
	void cv_rgbd_RgbdFrame_create_const_MatR_const_MatR_const_MatR_const_MatR_int(const cv::Mat* image, const cv::Mat* depth, const cv::Mat* mask, const cv::Mat* normals, int ID, Result<cv::Ptr<cv::rgbd::RgbdFrame>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::rgbd::RgbdFrame> ret = cv::rgbd::RgbdFrame::create(*image, *depth, *mask, *normals, ID);
			Ok(new cv::Ptr<cv::rgbd::RgbdFrame>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rgbd::RgbdFrame::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:467
	// ("cv::rgbd::RgbdFrame::create", vec![(pred!(mut, [], []), _)]),
	void cv_rgbd_RgbdFrame_create(Result<cv::Ptr<cv::rgbd::RgbdFrame>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::rgbd::RgbdFrame> ret = cv::rgbd::RgbdFrame::create();
			Ok(new cv::Ptr<cv::rgbd::RgbdFrame>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// release()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:470
	// ("cv::rgbd::RgbdFrame::release", vec![(pred!(mut, [], []), _)]),
	void cv_rgbd_RgbdFrame_release(cv::rgbd::RgbdFrame* instance, ResultVoid* ocvrs_return) {
		try {
			instance->release();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rgbd::RgbdFrame::ID() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:472
	// ("cv::rgbd::RgbdFrame::ID", vec![(pred!(const, [], []), _)]),
	int cv_rgbd_RgbdFrame_propID_const(const cv::rgbd::RgbdFrame* instance) {
			int ret = instance->ID;
			return ret;
	}

	// cv::rgbd::RgbdFrame::setID(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:472
	// ("cv::rgbd::RgbdFrame::setID", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_rgbd_RgbdFrame_propID_const_int(cv::rgbd::RgbdFrame* instance, const int val) {
			instance->ID = val;
	}

	// cv::rgbd::RgbdFrame::image() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:473
	// ("cv::rgbd::RgbdFrame::image", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_rgbd_RgbdFrame_propImage_const(const cv::rgbd::RgbdFrame* instance) {
			cv::Mat ret = instance->image;
			return new cv::Mat(ret);
	}

	// cv::rgbd::RgbdFrame::setImage(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:473
	// ("cv::rgbd::RgbdFrame::setImage", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_rgbd_RgbdFrame_propImage_const_Mat(cv::rgbd::RgbdFrame* instance, const cv::Mat* val) {
			instance->image = *val;
	}

	// cv::rgbd::RgbdFrame::depth() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:474
	// ("cv::rgbd::RgbdFrame::depth", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_rgbd_RgbdFrame_propDepth_const(const cv::rgbd::RgbdFrame* instance) {
			cv::Mat ret = instance->depth;
			return new cv::Mat(ret);
	}

	// cv::rgbd::RgbdFrame::setDepth(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:474
	// ("cv::rgbd::RgbdFrame::setDepth", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_rgbd_RgbdFrame_propDepth_const_Mat(cv::rgbd::RgbdFrame* instance, const cv::Mat* val) {
			instance->depth = *val;
	}

	// cv::rgbd::RgbdFrame::mask() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:475
	// ("cv::rgbd::RgbdFrame::mask", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_rgbd_RgbdFrame_propMask_const(const cv::rgbd::RgbdFrame* instance) {
			cv::Mat ret = instance->mask;
			return new cv::Mat(ret);
	}

	// cv::rgbd::RgbdFrame::setMask(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:475
	// ("cv::rgbd::RgbdFrame::setMask", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_rgbd_RgbdFrame_propMask_const_Mat(cv::rgbd::RgbdFrame* instance, const cv::Mat* val) {
			instance->mask = *val;
	}

	// cv::rgbd::RgbdFrame::normals() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:476
	// ("cv::rgbd::RgbdFrame::normals", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_rgbd_RgbdFrame_propNormals_const(const cv::rgbd::RgbdFrame* instance) {
			cv::Mat ret = instance->normals;
			return new cv::Mat(ret);
	}

	// cv::rgbd::RgbdFrame::setNormals(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:476
	// ("cv::rgbd::RgbdFrame::setNormals", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_rgbd_RgbdFrame_propNormals_const_Mat(cv::rgbd::RgbdFrame* instance, const cv::Mat* val) {
			instance->normals = *val;
	}

	// cv::rgbd::RgbdFrame::to_OdometryFrame() generated
	// ("cv::rgbd::RgbdFrame::to_OdometryFrame", vec![(pred!(mut, [], []), _)]),
	cv::rgbd::OdometryFrame* cv_rgbd_RgbdFrame_to_OdometryFrame(cv::rgbd::RgbdFrame* instance) {
			return dynamic_cast<cv::rgbd::OdometryFrame*>(instance);
	}

	// cv::rgbd::RgbdFrame::delete() generated
	// ("cv::rgbd::RgbdFrame::delete", vec![(pred!(mut, [], []), _)]),
	void cv_rgbd_RgbdFrame_delete(cv::rgbd::RgbdFrame* instance) {
			delete instance;
	}

	// RgbdICPOdometry()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:890
	// ("cv::rgbd::RgbdICPOdometry::RgbdICPOdometry", vec![(pred!(mut, [], []), _)]),
	void cv_rgbd_RgbdICPOdometry_RgbdICPOdometry(Result<cv::rgbd::RgbdICPOdometry*>* ocvrs_return) {
		try {
			cv::rgbd::RgbdICPOdometry* ret = new cv::rgbd::RgbdICPOdometry();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// RgbdICPOdometry(const Mat &, float, float, float, float, const std::vector<int> &, const std::vector<float> &, int)(TraitClass, Primitive, Primitive, Primitive, Primitive, CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:903
	// ("cv::rgbd::RgbdICPOdometry::RgbdICPOdometry", vec![(pred!(mut, ["cameraMatrix", "minDepth", "maxDepth", "maxDepthDiff", "maxPointsPart", "iterCounts", "minGradientMagnitudes", "transformType"], ["const cv::Mat*", "float", "float", "float", "float", "const std::vector<int>*", "const std::vector<float>*", "int"]), _)]),
	void cv_rgbd_RgbdICPOdometry_RgbdICPOdometry_const_MatR_float_float_float_float_const_vectorLintGR_const_vectorLfloatGR_int(const cv::Mat* cameraMatrix, float minDepth, float maxDepth, float maxDepthDiff, float maxPointsPart, const std::vector<int>* iterCounts, const std::vector<float>* minGradientMagnitudes, int transformType, Result<cv::rgbd::RgbdICPOdometry*>* ocvrs_return) {
		try {
			cv::rgbd::RgbdICPOdometry* ret = new cv::rgbd::RgbdICPOdometry(*cameraMatrix, minDepth, maxDepth, maxDepthDiff, maxPointsPart, *iterCounts, *minGradientMagnitudes, transformType);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rgbd::RgbdICPOdometry::RgbdICPOdometry(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:903
	// ("cv::rgbd::RgbdICPOdometry::RgbdICPOdometry", vec![(pred!(mut, ["cameraMatrix"], ["const cv::Mat*"]), _)]),
	void cv_rgbd_RgbdICPOdometry_RgbdICPOdometry_const_MatR(const cv::Mat* cameraMatrix, Result<cv::rgbd::RgbdICPOdometry*>* ocvrs_return) {
		try {
			cv::rgbd::RgbdICPOdometry* ret = new cv::rgbd::RgbdICPOdometry(*cameraMatrix);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const Mat &, float, float, float, float, const std::vector<int> &, const std::vector<float> &, int)(TraitClass, Primitive, Primitive, Primitive, Primitive, CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:909
	// ("cv::rgbd::RgbdICPOdometry::create", vec![(pred!(mut, ["cameraMatrix", "minDepth", "maxDepth", "maxDepthDiff", "maxPointsPart", "iterCounts", "minGradientMagnitudes", "transformType"], ["const cv::Mat*", "float", "float", "float", "float", "const std::vector<int>*", "const std::vector<float>*", "int"]), _)]),
	void cv_rgbd_RgbdICPOdometry_create_const_MatR_float_float_float_float_const_vectorLintGR_const_vectorLfloatGR_int(const cv::Mat* cameraMatrix, float minDepth, float maxDepth, float maxDepthDiff, float maxPointsPart, const std::vector<int>* iterCounts, const std::vector<float>* minGradientMagnitudes, int transformType, Result<cv::Ptr<cv::rgbd::RgbdICPOdometry>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::rgbd::RgbdICPOdometry> ret = cv::rgbd::RgbdICPOdometry::create(*cameraMatrix, minDepth, maxDepth, maxDepthDiff, maxPointsPart, *iterCounts, *minGradientMagnitudes, transformType);
			Ok(new cv::Ptr<cv::rgbd::RgbdICPOdometry>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rgbd::RgbdICPOdometry::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:909
	// ("cv::rgbd::RgbdICPOdometry::create", vec![(pred!(mut, [], []), _)]),
	void cv_rgbd_RgbdICPOdometry_create(Result<cv::Ptr<cv::rgbd::RgbdICPOdometry>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::rgbd::RgbdICPOdometry> ret = cv::rgbd::RgbdICPOdometry::create();
			Ok(new cv::Ptr<cv::rgbd::RgbdICPOdometry>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// prepareFrameCache(Ptr<OdometryFrame> &, int)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:915
	// ("cv::rgbd::RgbdICPOdometry::prepareFrameCache", vec![(pred!(const, ["frame", "cacheType"], ["cv::Ptr<cv::rgbd::OdometryFrame>*", "int"]), _)]),
	void cv_rgbd_RgbdICPOdometry_prepareFrameCache_const_PtrLOdometryFrameGR_int(const cv::rgbd::RgbdICPOdometry* instance, cv::Ptr<cv::rgbd::OdometryFrame>* frame, int cacheType, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->prepareFrameCache(*frame, cacheType);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getCameraMatrix()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:917
	// ("cv::rgbd::RgbdICPOdometry::getCameraMatrix", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdICPOdometry_getCameraMatrix_const(const cv::rgbd::RgbdICPOdometry* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getCameraMatrix();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setCameraMatrix(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:921
	// ("cv::rgbd::RgbdICPOdometry::setCameraMatrix", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
	void cv_rgbd_RgbdICPOdometry_setCameraMatrix_const_MatR(cv::rgbd::RgbdICPOdometry* instance, const cv::Mat* val, ResultVoid* ocvrs_return) {
		try {
			instance->setCameraMatrix(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMinDepth()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:925
	// ("cv::rgbd::RgbdICPOdometry::getMinDepth", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdICPOdometry_getMinDepth_const(const cv::rgbd::RgbdICPOdometry* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMinDepth();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMinDepth(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:929
	// ("cv::rgbd::RgbdICPOdometry::setMinDepth", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_rgbd_RgbdICPOdometry_setMinDepth_double(cv::rgbd::RgbdICPOdometry* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setMinDepth(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxDepth()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:933
	// ("cv::rgbd::RgbdICPOdometry::getMaxDepth", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdICPOdometry_getMaxDepth_const(const cv::rgbd::RgbdICPOdometry* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxDepth();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxDepth(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:937
	// ("cv::rgbd::RgbdICPOdometry::setMaxDepth", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_rgbd_RgbdICPOdometry_setMaxDepth_double(cv::rgbd::RgbdICPOdometry* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxDepth(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxDepthDiff()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:941
	// ("cv::rgbd::RgbdICPOdometry::getMaxDepthDiff", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdICPOdometry_getMaxDepthDiff_const(const cv::rgbd::RgbdICPOdometry* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxDepthDiff();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxDepthDiff(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:945
	// ("cv::rgbd::RgbdICPOdometry::setMaxDepthDiff", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_rgbd_RgbdICPOdometry_setMaxDepthDiff_double(cv::rgbd::RgbdICPOdometry* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxDepthDiff(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxPointsPart()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:949
	// ("cv::rgbd::RgbdICPOdometry::getMaxPointsPart", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdICPOdometry_getMaxPointsPart_const(const cv::rgbd::RgbdICPOdometry* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxPointsPart();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxPointsPart(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:953
	// ("cv::rgbd::RgbdICPOdometry::setMaxPointsPart", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_rgbd_RgbdICPOdometry_setMaxPointsPart_double(cv::rgbd::RgbdICPOdometry* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxPointsPart(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getIterationCounts()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:957
	// ("cv::rgbd::RgbdICPOdometry::getIterationCounts", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdICPOdometry_getIterationCounts_const(const cv::rgbd::RgbdICPOdometry* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getIterationCounts();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setIterationCounts(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:961
	// ("cv::rgbd::RgbdICPOdometry::setIterationCounts", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
	void cv_rgbd_RgbdICPOdometry_setIterationCounts_const_MatR(cv::rgbd::RgbdICPOdometry* instance, const cv::Mat* val, ResultVoid* ocvrs_return) {
		try {
			instance->setIterationCounts(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMinGradientMagnitudes()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:965
	// ("cv::rgbd::RgbdICPOdometry::getMinGradientMagnitudes", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdICPOdometry_getMinGradientMagnitudes_const(const cv::rgbd::RgbdICPOdometry* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getMinGradientMagnitudes();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMinGradientMagnitudes(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:969
	// ("cv::rgbd::RgbdICPOdometry::setMinGradientMagnitudes", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
	void cv_rgbd_RgbdICPOdometry_setMinGradientMagnitudes_const_MatR(cv::rgbd::RgbdICPOdometry* instance, const cv::Mat* val, ResultVoid* ocvrs_return) {
		try {
			instance->setMinGradientMagnitudes(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTransformType()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:973
	// ("cv::rgbd::RgbdICPOdometry::getTransformType", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdICPOdometry_getTransformType_const(const cv::rgbd::RgbdICPOdometry* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getTransformType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTransformType(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:977
	// ("cv::rgbd::RgbdICPOdometry::setTransformType", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_rgbd_RgbdICPOdometry_setTransformType_int(cv::rgbd::RgbdICPOdometry* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setTransformType(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxTranslation()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:981
	// ("cv::rgbd::RgbdICPOdometry::getMaxTranslation", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdICPOdometry_getMaxTranslation_const(const cv::rgbd::RgbdICPOdometry* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxTranslation();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxTranslation(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:985
	// ("cv::rgbd::RgbdICPOdometry::setMaxTranslation", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_rgbd_RgbdICPOdometry_setMaxTranslation_double(cv::rgbd::RgbdICPOdometry* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxTranslation(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxRotation()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:989
	// ("cv::rgbd::RgbdICPOdometry::getMaxRotation", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdICPOdometry_getMaxRotation_const(const cv::rgbd::RgbdICPOdometry* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxRotation();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxRotation(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:993
	// ("cv::rgbd::RgbdICPOdometry::setMaxRotation", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_rgbd_RgbdICPOdometry_setMaxRotation_double(cv::rgbd::RgbdICPOdometry* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxRotation(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNormalsComputer()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:997
	// ("cv::rgbd::RgbdICPOdometry::getNormalsComputer", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdICPOdometry_getNormalsComputer_const(const cv::rgbd::RgbdICPOdometry* instance, Result<cv::Ptr<cv::rgbd::RgbdNormals>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::rgbd::RgbdNormals> ret = instance->getNormalsComputer();
			Ok(new cv::Ptr<cv::rgbd::RgbdNormals>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rgbd::RgbdICPOdometry::to_Algorithm() generated
	// ("cv::rgbd::RgbdICPOdometry::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_rgbd_RgbdICPOdometry_to_Algorithm(cv::rgbd::RgbdICPOdometry* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::rgbd::RgbdICPOdometry::to_Odometry() generated
	// ("cv::rgbd::RgbdICPOdometry::to_Odometry", vec![(pred!(mut, [], []), _)]),
	cv::rgbd::Odometry* cv_rgbd_RgbdICPOdometry_to_Odometry(cv::rgbd::RgbdICPOdometry* instance) {
			return dynamic_cast<cv::rgbd::Odometry*>(instance);
	}

	// cv::rgbd::RgbdICPOdometry::delete() generated
	// ("cv::rgbd::RgbdICPOdometry::delete", vec![(pred!(mut, [], []), _)]),
	void cv_rgbd_RgbdICPOdometry_delete(cv::rgbd::RgbdICPOdometry* instance) {
			delete instance;
	}

	// RgbdNormals()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:83
	// ("cv::rgbd::RgbdNormals::RgbdNormals", vec![(pred!(mut, [], []), _)]),
	void cv_rgbd_RgbdNormals_RgbdNormals(Result<cv::rgbd::RgbdNormals*>* ocvrs_return) {
		try {
			cv::rgbd::RgbdNormals* ret = new cv::rgbd::RgbdNormals();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// RgbdNormals(int, int, int, InputArray, int, int)(Primitive, Primitive, Primitive, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:103
	// ("cv::rgbd::RgbdNormals::RgbdNormals", vec![(pred!(mut, ["rows", "cols", "depth", "K", "window_size", "method"], ["int", "int", "int", "const cv::_InputArray*", "int", "int"]), _)]),
	void cv_rgbd_RgbdNormals_RgbdNormals_int_int_int_const__InputArrayR_int_int(int rows, int cols, int depth, const cv::_InputArray* K, int window_size, int method, Result<cv::rgbd::RgbdNormals*>* ocvrs_return) {
		try {
			cv::rgbd::RgbdNormals* ret = new cv::rgbd::RgbdNormals(rows, cols, depth, *K, window_size, method);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rgbd::RgbdNormals::RgbdNormals(Primitive, Primitive, Primitive, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:103
	// ("cv::rgbd::RgbdNormals::RgbdNormals", vec![(pred!(mut, ["rows", "cols", "depth", "K"], ["int", "int", "int", "const cv::_InputArray*"]), _)]),
	void cv_rgbd_RgbdNormals_RgbdNormals_int_int_int_const__InputArrayR(int rows, int cols, int depth, const cv::_InputArray* K, Result<cv::rgbd::RgbdNormals*>* ocvrs_return) {
		try {
			cv::rgbd::RgbdNormals* ret = new cv::rgbd::RgbdNormals(rows, cols, depth, *K);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(int, int, int, InputArray, int, int)(Primitive, Primitive, Primitive, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:108
	// ("cv::rgbd::RgbdNormals::create", vec![(pred!(mut, ["rows", "cols", "depth", "K", "window_size", "method"], ["int", "int", "int", "const cv::_InputArray*", "int", "int"]), _)]),
	void cv_rgbd_RgbdNormals_create_int_int_int_const__InputArrayR_int_int(int rows, int cols, int depth, const cv::_InputArray* K, int window_size, int method, Result<cv::Ptr<cv::rgbd::RgbdNormals>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::rgbd::RgbdNormals> ret = cv::rgbd::RgbdNormals::create(rows, cols, depth, *K, window_size, method);
			Ok(new cv::Ptr<cv::rgbd::RgbdNormals>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rgbd::RgbdNormals::create(Primitive, Primitive, Primitive, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:108
	// ("cv::rgbd::RgbdNormals::create", vec![(pred!(mut, ["rows", "cols", "depth", "K"], ["int", "int", "int", "const cv::_InputArray*"]), _)]),
	void cv_rgbd_RgbdNormals_create_int_int_int_const__InputArrayR(int rows, int cols, int depth, const cv::_InputArray* K, Result<cv::Ptr<cv::rgbd::RgbdNormals>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::rgbd::RgbdNormals> ret = cv::rgbd::RgbdNormals::create(rows, cols, depth, *K);
			Ok(new cv::Ptr<cv::rgbd::RgbdNormals>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator()(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:116
	// ("cv::rgbd::RgbdNormals::operator()", vec![(pred!(const, ["points", "normals"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_rgbd_RgbdNormals_operator___const_const__InputArrayR_const__OutputArrayR(const cv::rgbd::RgbdNormals* instance, const cv::_InputArray* points, const cv::_OutputArray* normals, ResultVoid* ocvrs_return) {
		try {
			instance->operator()(*points, *normals);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// initialize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:122
	// ("cv::rgbd::RgbdNormals::initialize", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdNormals_initialize_const(const cv::rgbd::RgbdNormals* instance, ResultVoid* ocvrs_return) {
		try {
			instance->initialize();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getRows()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:124
	// ("cv::rgbd::RgbdNormals::getRows", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdNormals_getRows_const(const cv::rgbd::RgbdNormals* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getRows();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setRows(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:128
	// ("cv::rgbd::RgbdNormals::setRows", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_rgbd_RgbdNormals_setRows_int(cv::rgbd::RgbdNormals* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setRows(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getCols()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:132
	// ("cv::rgbd::RgbdNormals::getCols", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdNormals_getCols_const(const cv::rgbd::RgbdNormals* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getCols();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setCols(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:136
	// ("cv::rgbd::RgbdNormals::setCols", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_rgbd_RgbdNormals_setCols_int(cv::rgbd::RgbdNormals* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setCols(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getWindowSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:140
	// ("cv::rgbd::RgbdNormals::getWindowSize", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdNormals_getWindowSize_const(const cv::rgbd::RgbdNormals* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getWindowSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWindowSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:144
	// ("cv::rgbd::RgbdNormals::setWindowSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_rgbd_RgbdNormals_setWindowSize_int(cv::rgbd::RgbdNormals* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setWindowSize(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDepth()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:148
	// ("cv::rgbd::RgbdNormals::getDepth", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdNormals_getDepth_const(const cv::rgbd::RgbdNormals* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getDepth();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDepth(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:152
	// ("cv::rgbd::RgbdNormals::setDepth", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_rgbd_RgbdNormals_setDepth_int(cv::rgbd::RgbdNormals* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setDepth(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getK()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:156
	// ("cv::rgbd::RgbdNormals::getK", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdNormals_getK_const(const cv::rgbd::RgbdNormals* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getK();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setK(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:160
	// ("cv::rgbd::RgbdNormals::setK", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
	void cv_rgbd_RgbdNormals_setK_const_MatR(cv::rgbd::RgbdNormals* instance, const cv::Mat* val, ResultVoid* ocvrs_return) {
		try {
			instance->setK(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMethod()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:164
	// ("cv::rgbd::RgbdNormals::getMethod", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdNormals_getMethod_const(const cv::rgbd::RgbdNormals* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMethod();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMethod(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:168
	// ("cv::rgbd::RgbdNormals::setMethod", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_rgbd_RgbdNormals_setMethod_int(cv::rgbd::RgbdNormals* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setMethod(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rgbd::RgbdNormals::to_Algorithm() generated
	// ("cv::rgbd::RgbdNormals::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_rgbd_RgbdNormals_to_Algorithm(cv::rgbd::RgbdNormals* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::rgbd::RgbdNormals::delete() generated
	// ("cv::rgbd::RgbdNormals::delete", vec![(pred!(mut, [], []), _)]),
	void cv_rgbd_RgbdNormals_delete(cv::rgbd::RgbdNormals* instance) {
			delete instance;
	}

	// RgbdOdometry()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:627
	// ("cv::rgbd::RgbdOdometry::RgbdOdometry", vec![(pred!(mut, [], []), _)]),
	void cv_rgbd_RgbdOdometry_RgbdOdometry(Result<cv::rgbd::RgbdOdometry*>* ocvrs_return) {
		try {
			cv::rgbd::RgbdOdometry* ret = new cv::rgbd::RgbdOdometry();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// RgbdOdometry(const Mat &, float, float, float, const std::vector<int> &, const std::vector<float> &, float, int)(TraitClass, Primitive, Primitive, Primitive, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:640
	// ("cv::rgbd::RgbdOdometry::RgbdOdometry", vec![(pred!(mut, ["cameraMatrix", "minDepth", "maxDepth", "maxDepthDiff", "iterCounts", "minGradientMagnitudes", "maxPointsPart", "transformType"], ["const cv::Mat*", "float", "float", "float", "const std::vector<int>*", "const std::vector<float>*", "float", "int"]), _)]),
	void cv_rgbd_RgbdOdometry_RgbdOdometry_const_MatR_float_float_float_const_vectorLintGR_const_vectorLfloatGR_float_int(const cv::Mat* cameraMatrix, float minDepth, float maxDepth, float maxDepthDiff, const std::vector<int>* iterCounts, const std::vector<float>* minGradientMagnitudes, float maxPointsPart, int transformType, Result<cv::rgbd::RgbdOdometry*>* ocvrs_return) {
		try {
			cv::rgbd::RgbdOdometry* ret = new cv::rgbd::RgbdOdometry(*cameraMatrix, minDepth, maxDepth, maxDepthDiff, *iterCounts, *minGradientMagnitudes, maxPointsPart, transformType);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rgbd::RgbdOdometry::RgbdOdometry(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:640
	// ("cv::rgbd::RgbdOdometry::RgbdOdometry", vec![(pred!(mut, ["cameraMatrix"], ["const cv::Mat*"]), _)]),
	void cv_rgbd_RgbdOdometry_RgbdOdometry_const_MatR(const cv::Mat* cameraMatrix, Result<cv::rgbd::RgbdOdometry*>* ocvrs_return) {
		try {
			cv::rgbd::RgbdOdometry* ret = new cv::rgbd::RgbdOdometry(*cameraMatrix);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const Mat &, float, float, float, const std::vector<int> &, const std::vector<float> &, float, int)(TraitClass, Primitive, Primitive, Primitive, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:645
	// ("cv::rgbd::RgbdOdometry::create", vec![(pred!(mut, ["cameraMatrix", "minDepth", "maxDepth", "maxDepthDiff", "iterCounts", "minGradientMagnitudes", "maxPointsPart", "transformType"], ["const cv::Mat*", "float", "float", "float", "const std::vector<int>*", "const std::vector<float>*", "float", "int"]), _)]),
	void cv_rgbd_RgbdOdometry_create_const_MatR_float_float_float_const_vectorLintGR_const_vectorLfloatGR_float_int(const cv::Mat* cameraMatrix, float minDepth, float maxDepth, float maxDepthDiff, const std::vector<int>* iterCounts, const std::vector<float>* minGradientMagnitudes, float maxPointsPart, int transformType, Result<cv::Ptr<cv::rgbd::RgbdOdometry>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::rgbd::RgbdOdometry> ret = cv::rgbd::RgbdOdometry::create(*cameraMatrix, minDepth, maxDepth, maxDepthDiff, *iterCounts, *minGradientMagnitudes, maxPointsPart, transformType);
			Ok(new cv::Ptr<cv::rgbd::RgbdOdometry>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rgbd::RgbdOdometry::create() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:645
	// ("cv::rgbd::RgbdOdometry::create", vec![(pred!(mut, [], []), _)]),
	void cv_rgbd_RgbdOdometry_create(Result<cv::Ptr<cv::rgbd::RgbdOdometry>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::rgbd::RgbdOdometry> ret = cv::rgbd::RgbdOdometry::create();
			Ok(new cv::Ptr<cv::rgbd::RgbdOdometry>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// prepareFrameCache(Ptr<OdometryFrame> &, int)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:650
	// ("cv::rgbd::RgbdOdometry::prepareFrameCache", vec![(pred!(const, ["frame", "cacheType"], ["cv::Ptr<cv::rgbd::OdometryFrame>*", "int"]), _)]),
	void cv_rgbd_RgbdOdometry_prepareFrameCache_const_PtrLOdometryFrameGR_int(const cv::rgbd::RgbdOdometry* instance, cv::Ptr<cv::rgbd::OdometryFrame>* frame, int cacheType, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->prepareFrameCache(*frame, cacheType);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getCameraMatrix()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:652
	// ("cv::rgbd::RgbdOdometry::getCameraMatrix", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdOdometry_getCameraMatrix_const(const cv::rgbd::RgbdOdometry* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getCameraMatrix();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setCameraMatrix(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:656
	// ("cv::rgbd::RgbdOdometry::setCameraMatrix", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
	void cv_rgbd_RgbdOdometry_setCameraMatrix_const_MatR(cv::rgbd::RgbdOdometry* instance, const cv::Mat* val, ResultVoid* ocvrs_return) {
		try {
			instance->setCameraMatrix(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMinDepth()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:660
	// ("cv::rgbd::RgbdOdometry::getMinDepth", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdOdometry_getMinDepth_const(const cv::rgbd::RgbdOdometry* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMinDepth();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMinDepth(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:664
	// ("cv::rgbd::RgbdOdometry::setMinDepth", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_rgbd_RgbdOdometry_setMinDepth_double(cv::rgbd::RgbdOdometry* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setMinDepth(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxDepth()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:668
	// ("cv::rgbd::RgbdOdometry::getMaxDepth", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdOdometry_getMaxDepth_const(const cv::rgbd::RgbdOdometry* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxDepth();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxDepth(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:672
	// ("cv::rgbd::RgbdOdometry::setMaxDepth", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_rgbd_RgbdOdometry_setMaxDepth_double(cv::rgbd::RgbdOdometry* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxDepth(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxDepthDiff()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:676
	// ("cv::rgbd::RgbdOdometry::getMaxDepthDiff", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdOdometry_getMaxDepthDiff_const(const cv::rgbd::RgbdOdometry* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxDepthDiff();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxDepthDiff(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:680
	// ("cv::rgbd::RgbdOdometry::setMaxDepthDiff", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_rgbd_RgbdOdometry_setMaxDepthDiff_double(cv::rgbd::RgbdOdometry* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxDepthDiff(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getIterationCounts()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:684
	// ("cv::rgbd::RgbdOdometry::getIterationCounts", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdOdometry_getIterationCounts_const(const cv::rgbd::RgbdOdometry* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getIterationCounts();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setIterationCounts(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:688
	// ("cv::rgbd::RgbdOdometry::setIterationCounts", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
	void cv_rgbd_RgbdOdometry_setIterationCounts_const_MatR(cv::rgbd::RgbdOdometry* instance, const cv::Mat* val, ResultVoid* ocvrs_return) {
		try {
			instance->setIterationCounts(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMinGradientMagnitudes()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:692
	// ("cv::rgbd::RgbdOdometry::getMinGradientMagnitudes", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdOdometry_getMinGradientMagnitudes_const(const cv::rgbd::RgbdOdometry* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getMinGradientMagnitudes();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMinGradientMagnitudes(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:696
	// ("cv::rgbd::RgbdOdometry::setMinGradientMagnitudes", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
	void cv_rgbd_RgbdOdometry_setMinGradientMagnitudes_const_MatR(cv::rgbd::RgbdOdometry* instance, const cv::Mat* val, ResultVoid* ocvrs_return) {
		try {
			instance->setMinGradientMagnitudes(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxPointsPart()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:700
	// ("cv::rgbd::RgbdOdometry::getMaxPointsPart", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdOdometry_getMaxPointsPart_const(const cv::rgbd::RgbdOdometry* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxPointsPart();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxPointsPart(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:704
	// ("cv::rgbd::RgbdOdometry::setMaxPointsPart", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_rgbd_RgbdOdometry_setMaxPointsPart_double(cv::rgbd::RgbdOdometry* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxPointsPart(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTransformType()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:708
	// ("cv::rgbd::RgbdOdometry::getTransformType", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdOdometry_getTransformType_const(const cv::rgbd::RgbdOdometry* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getTransformType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTransformType(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:712
	// ("cv::rgbd::RgbdOdometry::setTransformType", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_rgbd_RgbdOdometry_setTransformType_int(cv::rgbd::RgbdOdometry* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setTransformType(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxTranslation()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:716
	// ("cv::rgbd::RgbdOdometry::getMaxTranslation", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdOdometry_getMaxTranslation_const(const cv::rgbd::RgbdOdometry* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxTranslation();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxTranslation(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:720
	// ("cv::rgbd::RgbdOdometry::setMaxTranslation", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_rgbd_RgbdOdometry_setMaxTranslation_double(cv::rgbd::RgbdOdometry* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxTranslation(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxRotation()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:724
	// ("cv::rgbd::RgbdOdometry::getMaxRotation", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdOdometry_getMaxRotation_const(const cv::rgbd::RgbdOdometry* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxRotation();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxRotation(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:728
	// ("cv::rgbd::RgbdOdometry::setMaxRotation", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_rgbd_RgbdOdometry_setMaxRotation_double(cv::rgbd::RgbdOdometry* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxRotation(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rgbd::RgbdOdometry::to_Algorithm() generated
	// ("cv::rgbd::RgbdOdometry::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_rgbd_RgbdOdometry_to_Algorithm(cv::rgbd::RgbdOdometry* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::rgbd::RgbdOdometry::to_Odometry() generated
	// ("cv::rgbd::RgbdOdometry::to_Odometry", vec![(pred!(mut, [], []), _)]),
	cv::rgbd::Odometry* cv_rgbd_RgbdOdometry_to_Odometry(cv::rgbd::RgbdOdometry* instance) {
			return dynamic_cast<cv::rgbd::Odometry*>(instance);
	}

	// cv::rgbd::RgbdOdometry::delete() generated
	// ("cv::rgbd::RgbdOdometry::delete", vec![(pred!(mut, [], []), _)]),
	void cv_rgbd_RgbdOdometry_delete(cv::rgbd::RgbdOdometry* instance) {
			delete instance;
	}

	// RgbdPlane(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:337
	// ("cv::rgbd::RgbdPlane::RgbdPlane", vec![(pred!(mut, ["method"], ["int"]), _)]),
	void cv_rgbd_RgbdPlane_RgbdPlane_int(int method, Result<cv::rgbd::RgbdPlane*>* ocvrs_return) {
		try {
			cv::rgbd::RgbdPlane* ret = new cv::rgbd::RgbdPlane(method);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rgbd::RgbdPlane::RgbdPlane() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:337
	// ("cv::rgbd::RgbdPlane::RgbdPlane", vec![(pred!(mut, [], []), _)]),
	void cv_rgbd_RgbdPlane_RgbdPlane(Result<cv::rgbd::RgbdPlane*>* ocvrs_return) {
		try {
			cv::rgbd::RgbdPlane* ret = new cv::rgbd::RgbdPlane();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// RgbdPlane(int, int, int, double, double, double, double)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:358
	// ("cv::rgbd::RgbdPlane::RgbdPlane", vec![(pred!(mut, ["method", "block_size", "min_size", "threshold", "sensor_error_a", "sensor_error_b", "sensor_error_c"], ["int", "int", "int", "double", "double", "double", "double"]), _)]),
	void cv_rgbd_RgbdPlane_RgbdPlane_int_int_int_double_double_double_double(int method, int block_size, int min_size, double threshold, double sensor_error_a, double sensor_error_b, double sensor_error_c, Result<cv::rgbd::RgbdPlane*>* ocvrs_return) {
		try {
			cv::rgbd::RgbdPlane* ret = new cv::rgbd::RgbdPlane(method, block_size, min_size, threshold, sensor_error_a, sensor_error_b, sensor_error_c);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rgbd::RgbdPlane::RgbdPlane(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:358
	// ("cv::rgbd::RgbdPlane::RgbdPlane", vec![(pred!(mut, ["method", "block_size", "min_size", "threshold"], ["int", "int", "int", "double"]), _)]),
	void cv_rgbd_RgbdPlane_RgbdPlane_int_int_int_double(int method, int block_size, int min_size, double threshold, Result<cv::rgbd::RgbdPlane*>* ocvrs_return) {
		try {
			cv::rgbd::RgbdPlane* ret = new cv::rgbd::RgbdPlane(method, block_size, min_size, threshold);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(int, int, int, double, double, double, double)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:364
	// ("cv::rgbd::RgbdPlane::create", vec![(pred!(mut, ["method", "block_size", "min_size", "threshold", "sensor_error_a", "sensor_error_b", "sensor_error_c"], ["int", "int", "int", "double", "double", "double", "double"]), _)]),
	void cv_rgbd_RgbdPlane_create_int_int_int_double_double_double_double(int method, int block_size, int min_size, double threshold, double sensor_error_a, double sensor_error_b, double sensor_error_c, Result<cv::Ptr<cv::rgbd::RgbdPlane>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::rgbd::RgbdPlane> ret = cv::rgbd::RgbdPlane::create(method, block_size, min_size, threshold, sensor_error_a, sensor_error_b, sensor_error_c);
			Ok(new cv::Ptr<cv::rgbd::RgbdPlane>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rgbd::RgbdPlane::create(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:364
	// ("cv::rgbd::RgbdPlane::create", vec![(pred!(mut, ["method", "block_size", "min_size", "threshold"], ["int", "int", "int", "double"]), _)]),
	void cv_rgbd_RgbdPlane_create_int_int_int_double(int method, int block_size, int min_size, double threshold, Result<cv::Ptr<cv::rgbd::RgbdPlane>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::rgbd::RgbdPlane> ret = cv::rgbd::RgbdPlane::create(method, block_size, min_size, threshold);
			Ok(new cv::Ptr<cv::rgbd::RgbdPlane>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator()(InputArray, InputArray, OutputArray, OutputArray)(InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:377
	// ("cv::rgbd::RgbdPlane::operator()", vec![(pred!(mut, ["points3d", "normals", "mask", "plane_coefficients"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_rgbd_RgbdPlane_operator___const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::rgbd::RgbdPlane* instance, const cv::_InputArray* points3d, const cv::_InputArray* normals, const cv::_OutputArray* mask, const cv::_OutputArray* plane_coefficients, ResultVoid* ocvrs_return) {
		try {
			instance->operator()(*points3d, *normals, *mask, *plane_coefficients);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator()(InputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:387
	// ("cv::rgbd::RgbdPlane::operator()", vec![(pred!(mut, ["points3d", "mask", "plane_coefficients"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_rgbd_RgbdPlane_operator___const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::rgbd::RgbdPlane* instance, const cv::_InputArray* points3d, const cv::_OutputArray* mask, const cv::_OutputArray* plane_coefficients, ResultVoid* ocvrs_return) {
		try {
			instance->operator()(*points3d, *mask, *plane_coefficients);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getBlockSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:389
	// ("cv::rgbd::RgbdPlane::getBlockSize", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdPlane_getBlockSize_const(const cv::rgbd::RgbdPlane* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getBlockSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setBlockSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:393
	// ("cv::rgbd::RgbdPlane::setBlockSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_rgbd_RgbdPlane_setBlockSize_int(cv::rgbd::RgbdPlane* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setBlockSize(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMinSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:397
	// ("cv::rgbd::RgbdPlane::getMinSize", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdPlane_getMinSize_const(const cv::rgbd::RgbdPlane* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMinSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMinSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:401
	// ("cv::rgbd::RgbdPlane::setMinSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_rgbd_RgbdPlane_setMinSize_int(cv::rgbd::RgbdPlane* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setMinSize(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMethod()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:405
	// ("cv::rgbd::RgbdPlane::getMethod", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdPlane_getMethod_const(const cv::rgbd::RgbdPlane* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMethod();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMethod(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:409
	// ("cv::rgbd::RgbdPlane::setMethod", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_rgbd_RgbdPlane_setMethod_int(cv::rgbd::RgbdPlane* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setMethod(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getThreshold()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:413
	// ("cv::rgbd::RgbdPlane::getThreshold", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdPlane_getThreshold_const(const cv::rgbd::RgbdPlane* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:417
	// ("cv::rgbd::RgbdPlane::setThreshold", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_rgbd_RgbdPlane_setThreshold_double(cv::rgbd::RgbdPlane* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setThreshold(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSensorErrorA()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:421
	// ("cv::rgbd::RgbdPlane::getSensorErrorA", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdPlane_getSensorErrorA_const(const cv::rgbd::RgbdPlane* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getSensorErrorA();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSensorErrorA(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:425
	// ("cv::rgbd::RgbdPlane::setSensorErrorA", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_rgbd_RgbdPlane_setSensorErrorA_double(cv::rgbd::RgbdPlane* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setSensorErrorA(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSensorErrorB()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:429
	// ("cv::rgbd::RgbdPlane::getSensorErrorB", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdPlane_getSensorErrorB_const(const cv::rgbd::RgbdPlane* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getSensorErrorB();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSensorErrorB(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:433
	// ("cv::rgbd::RgbdPlane::setSensorErrorB", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_rgbd_RgbdPlane_setSensorErrorB_double(cv::rgbd::RgbdPlane* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setSensorErrorB(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSensorErrorC()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:437
	// ("cv::rgbd::RgbdPlane::getSensorErrorC", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdPlane_getSensorErrorC_const(const cv::rgbd::RgbdPlane* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getSensorErrorC();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSensorErrorC(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/rgbd/depth.hpp:441
	// ("cv::rgbd::RgbdPlane::setSensorErrorC", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_rgbd_RgbdPlane_setSensorErrorC_double(cv::rgbd::RgbdPlane* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setSensorErrorC(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rgbd::RgbdPlane::to_Algorithm() generated
	// ("cv::rgbd::RgbdPlane::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_rgbd_RgbdPlane_to_Algorithm(cv::rgbd::RgbdPlane* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::rgbd::RgbdPlane::delete() generated
	// ("cv::rgbd::RgbdPlane::delete", vec![(pred!(mut, [], []), _)]),
	void cv_rgbd_RgbdPlane_delete(cv::rgbd::RgbdPlane* instance) {
			delete instance;
	}

}
