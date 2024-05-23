#include "ocvrs_common.hpp"
#include <opencv2/rgbd.hpp>
#include "rgbd_types.hpp"

extern "C" {
	// colormap(const Mat &, Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:245
	// ("cv::linemod::colormap", vec![(pred!(mut, ["quantized", "dst"], ["const cv::Mat*", "cv::Mat*"]), _)]),
	void cv_linemod_colormap_const_MatR_MatR(const cv::Mat* quantized, cv::Mat* dst, ResultVoid* ocvrs_return) {
		try {
			cv::linemod::colormap(*quantized, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::linemod::drawFeatures(InputOutputArray, CppPassByVoidPtr, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:254
	// ("cv::linemod::drawFeatures", vec![(pred!(mut, ["img", "templates", "tl"], ["const cv::_InputOutputArray*", "const std::vector<cv::linemod::Template>*", "const cv::Point2i*"]), _)]),
	void cv_linemod_drawFeatures_const__InputOutputArrayR_const_vectorLTemplateGR_const_Point2iR(const cv::_InputOutputArray* img, const std::vector<cv::linemod::Template>* templates, const cv::Point2i* tl, ResultVoid* ocvrs_return) {
		try {
			cv::linemod::drawFeatures(*img, *templates, *tl);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// drawFeatures(InputOutputArray, const std::vector<Template> &, const Point2i &, int)(InputOutputArray, CppPassByVoidPtr, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:254
	// ("cv::linemod::drawFeatures", vec![(pred!(mut, ["img", "templates", "tl", "size"], ["const cv::_InputOutputArray*", "const std::vector<cv::linemod::Template>*", "const cv::Point2i*", "int"]), _)]),
	void cv_linemod_drawFeatures_const__InputOutputArrayR_const_vectorLTemplateGR_const_Point2iR_int(const cv::_InputOutputArray* img, const std::vector<cv::linemod::Template>* templates, const cv::Point2i* tl, int size, ResultVoid* ocvrs_return) {
		try {
			cv::linemod::drawFeatures(*img, *templates, *tl, size);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDefaultLINE()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:420
	// ("cv::linemod::getDefaultLINE", vec![(pred!(mut, [], []), _)]),
	void cv_linemod_getDefaultLINE(Result<cv::Ptr<cv::linemod::Detector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::linemod::Detector> ret = cv::linemod::getDefaultLINE();
			Ok(new cv::Ptr<cv::linemod::Detector>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDefaultLINEMOD()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:428
	// ("cv::linemod::getDefaultLINEMOD", vec![(pred!(mut, [], []), _)]),
	void cv_linemod_getDefaultLINEMOD(Result<cv::Ptr<cv::linemod::Detector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::linemod::Detector> ret = cv::linemod::getDefaultLINEMOD();
			Ok(new cv::Ptr<cv::linemod::Detector>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const Ptr<Params> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:200
	// ("cv::colored_kinfu::ColoredKinFu::create", vec![(pred!(mut, ["_params"], ["const cv::Ptr<cv::colored_kinfu::Params>*"]), _)]),
	void cv_colored_kinfu_ColoredKinFu_create_const_PtrLParamsGR(const cv::Ptr<cv::colored_kinfu::Params>* _params, Result<cv::Ptr<cv::colored_kinfu::ColoredKinFu>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::colored_kinfu::ColoredKinFu> ret = cv::colored_kinfu::ColoredKinFu::create(*_params);
			Ok(new cv::Ptr<cv::colored_kinfu::ColoredKinFu>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getParams()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:204
	// ("cv::colored_kinfu::ColoredKinFu::getParams", vec![(pred!(const, [], []), _)]),
	void cv_colored_kinfu_ColoredKinFu_getParams_const(const cv::colored_kinfu::ColoredKinFu* instance, Result<cv::colored_kinfu::Params*>* ocvrs_return) {
		try {
			const cv::colored_kinfu::Params ret = instance->getParams();
			Ok(new const cv::colored_kinfu::Params(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// render(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:214
	// ("cv::colored_kinfu::ColoredKinFu::render", vec![(pred!(const, ["image"], ["const cv::_OutputArray*"]), _)]),
	void cv_colored_kinfu_ColoredKinFu_render_const_const__OutputArrayR(const cv::colored_kinfu::ColoredKinFu* instance, const cv::_OutputArray* image, ResultVoid* ocvrs_return) {
		try {
			instance->render(*image);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// render(OutputArray, const Matx44f &)(OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:226
	// ("cv::colored_kinfu::ColoredKinFu::render", vec![(pred!(const, ["image", "cameraPose"], ["const cv::_OutputArray*", "const cv::Matx44f*"]), _)]),
	void cv_colored_kinfu_ColoredKinFu_render_const_const__OutputArrayR_const_Matx44fR(const cv::colored_kinfu::ColoredKinFu* instance, const cv::_OutputArray* image, const cv::Matx44f* cameraPose, ResultVoid* ocvrs_return) {
		try {
			instance->render(*image, *cameraPose);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getCloud(OutputArray, OutputArray)(OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:236
	// ("cv::colored_kinfu::ColoredKinFu::getCloud", vec![(pred!(const, ["points", "normals"], ["const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_colored_kinfu_ColoredKinFu_getCloud_const_const__OutputArrayR_const__OutputArrayR(const cv::colored_kinfu::ColoredKinFu* instance, const cv::_OutputArray* points, const cv::_OutputArray* normals, ResultVoid* ocvrs_return) {
		try {
			instance->getCloud(*points, *normals);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPoints(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:244
	// ("cv::colored_kinfu::ColoredKinFu::getPoints", vec![(pred!(const, ["points"], ["const cv::_OutputArray*"]), _)]),
	void cv_colored_kinfu_ColoredKinFu_getPoints_const_const__OutputArrayR(const cv::colored_kinfu::ColoredKinFu* instance, const cv::_OutputArray* points, ResultVoid* ocvrs_return) {
		try {
			instance->getPoints(*points);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNormals(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:250
	// ("cv::colored_kinfu::ColoredKinFu::getNormals", vec![(pred!(const, ["points", "normals"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_colored_kinfu_ColoredKinFu_getNormals_const_const__InputArrayR_const__OutputArrayR(const cv::colored_kinfu::ColoredKinFu* instance, const cv::_InputArray* points, const cv::_OutputArray* normals, ResultVoid* ocvrs_return) {
		try {
			instance->getNormals(*points, *normals);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// reset()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:256
	// ("cv::colored_kinfu::ColoredKinFu::reset", vec![(pred!(mut, [], []), _)]),
	void cv_colored_kinfu_ColoredKinFu_reset(cv::colored_kinfu::ColoredKinFu* instance, ResultVoid* ocvrs_return) {
		try {
			instance->reset();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPose()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:259
	// ("cv::colored_kinfu::ColoredKinFu::getPose", vec![(pred!(const, [], []), _)]),
	void cv_colored_kinfu_ColoredKinFu_getPose_const(const cv::colored_kinfu::ColoredKinFu* instance, Result<cv::Affine3f>* ocvrs_return) {
		try {
			cv::Affine3f ret = instance->getPose();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// update(InputArray, InputArray)(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:267
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

	// Params()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:21
	// ("cv::colored_kinfu::Params::Params", vec![(pred!(mut, [], []), _)]),
	void cv_colored_kinfu_Params_Params(Result<cv::colored_kinfu::Params*>* ocvrs_return) {
		try {
			cv::colored_kinfu::Params* ret = new cv::colored_kinfu::Params();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Params(Matx33f, Vec3f)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:32
	// ("cv::colored_kinfu::Params::Params", vec![(pred!(mut, ["volumeInitialPoseRot", "volumeInitialPoseTransl"], ["cv::Matx33f", "cv::Vec3f"]), _)]),
	void cv_colored_kinfu_Params_Params_Matx33f_Vec3f(cv::Matx33f* volumeInitialPoseRot, cv::Vec3f* volumeInitialPoseTransl, Result<cv::colored_kinfu::Params*>* ocvrs_return) {
		try {
			cv::colored_kinfu::Params* ret = new cv::colored_kinfu::Params(*volumeInitialPoseRot, *volumeInitialPoseTransl);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Params(Matx44f)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:42
	// ("cv::colored_kinfu::Params::Params", vec![(pred!(mut, ["volumeInitialPose"], ["cv::Matx44f"]), _)]),
	void cv_colored_kinfu_Params_Params_Matx44f(cv::Matx44f* volumeInitialPose, Result<cv::colored_kinfu::Params*>* ocvrs_return) {
		try {
			cv::colored_kinfu::Params* ret = new cv::colored_kinfu::Params(*volumeInitialPose);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setInitialVolumePose(Matx33f, Vec3f)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:53
	// ("cv::colored_kinfu::Params::setInitialVolumePose", vec![(pred!(mut, ["R", "t"], ["cv::Matx33f", "cv::Vec3f"]), _)]),
	void cv_colored_kinfu_Params_setInitialVolumePose_Matx33f_Vec3f(cv::colored_kinfu::Params* instance, cv::Matx33f* R, cv::Vec3f* t, ResultVoid* ocvrs_return) {
		try {
			instance->setInitialVolumePose(*R, *t);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setInitialVolumePose(Matx44f)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:60
	// ("cv::colored_kinfu::Params::setInitialVolumePose", vec![(pred!(mut, ["homogen_tf"], ["cv::Matx44f"]), _)]),
	void cv_colored_kinfu_Params_setInitialVolumePose_Matx44f(cv::colored_kinfu::Params* instance, cv::Matx44f* homogen_tf, ResultVoid* ocvrs_return) {
		try {
			instance->setInitialVolumePose(*homogen_tf);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// defaultParams()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:66
	// ("cv::colored_kinfu::Params::defaultParams", vec![(pred!(mut, [], []), _)]),
	void cv_colored_kinfu_Params_defaultParams(Result<cv::Ptr<cv::colored_kinfu::Params>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::colored_kinfu::Params> ret = cv::colored_kinfu::Params::defaultParams();
			Ok(new cv::Ptr<cv::colored_kinfu::Params>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// coarseParams()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:72
	// ("cv::colored_kinfu::Params::coarseParams", vec![(pred!(mut, [], []), _)]),
	void cv_colored_kinfu_Params_coarseParams(Result<cv::Ptr<cv::colored_kinfu::Params>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::colored_kinfu::Params> ret = cv::colored_kinfu::Params::coarseParams();
			Ok(new cv::Ptr<cv::colored_kinfu::Params>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// hashTSDFParams(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:77
	// ("cv::colored_kinfu::Params::hashTSDFParams", vec![(pred!(mut, ["isCoarse"], ["bool"]), _)]),
	void cv_colored_kinfu_Params_hashTSDFParams_bool(bool isCoarse, Result<cv::Ptr<cv::colored_kinfu::Params>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::colored_kinfu::Params> ret = cv::colored_kinfu::Params::hashTSDFParams(isCoarse);
			Ok(new cv::Ptr<cv::colored_kinfu::Params>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// coloredTSDFParams(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:82
	// ("cv::colored_kinfu::Params::coloredTSDFParams", vec![(pred!(mut, ["isCoarse"], ["bool"]), _)]),
	void cv_colored_kinfu_Params_coloredTSDFParams_bool(bool isCoarse, Result<cv::Ptr<cv::colored_kinfu::Params>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::colored_kinfu::Params> ret = cv::colored_kinfu::Params::coloredTSDFParams(isCoarse);
			Ok(new cv::Ptr<cv::colored_kinfu::Params>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::colored_kinfu::Params::frameSize() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:85
	// ("cv::colored_kinfu::Params::frameSize", vec![(pred!(const, [], []), _)]),
	void cv_colored_kinfu_Params_propFrameSize_const(const cv::colored_kinfu::Params* instance, cv::Size* ocvrs_return) {
			cv::Size ret = instance->frameSize;
			*ocvrs_return = ret;
	}

	// cv::colored_kinfu::Params::setFrameSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:85
	// ("cv::colored_kinfu::Params::setFrameSize", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	void cv_colored_kinfu_Params_propFrameSize_const_Size(cv::colored_kinfu::Params* instance, const cv::Size* val) {
			instance->frameSize = *val;
	}

	// cv::colored_kinfu::Params::rgb_frameSize() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:88
	// ("cv::colored_kinfu::Params::rgb_frameSize", vec![(pred!(const, [], []), _)]),
	void cv_colored_kinfu_Params_propRgb_frameSize_const(const cv::colored_kinfu::Params* instance, cv::Size* ocvrs_return) {
			cv::Size ret = instance->rgb_frameSize;
			*ocvrs_return = ret;
	}

	// cv::colored_kinfu::Params::setRgb_frameSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:88
	// ("cv::colored_kinfu::Params::setRgb_frameSize", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	void cv_colored_kinfu_Params_propRgb_frameSize_const_Size(cv::colored_kinfu::Params* instance, const cv::Size* val) {
			instance->rgb_frameSize = *val;
	}

	// cv::colored_kinfu::Params::volumeKind() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:90
	// ("cv::colored_kinfu::Params::volumeKind", vec![(pred!(const, [], []), _)]),
	void cv_colored_kinfu_Params_propVolumeKind_const(const cv::colored_kinfu::Params* instance, cv::VolumeType* ocvrs_return) {
			cv::VolumeType ret = instance->volumeKind;
			*ocvrs_return = ret;
	}

	// cv::colored_kinfu::Params::setVolumeKind(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:90
	// ("cv::colored_kinfu::Params::setVolumeKind", vec![(pred!(mut, ["val"], ["const cv::VolumeType"]), _)]),
	void cv_colored_kinfu_Params_propVolumeKind_const_VolumeType(cv::colored_kinfu::Params* instance, const cv::VolumeType val) {
			instance->volumeKind = val;
	}

	// cv::colored_kinfu::Params::intr() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:93
	// ("cv::colored_kinfu::Params::intr", vec![(pred!(const, [], []), _)]),
	void cv_colored_kinfu_Params_propIntr_const(const cv::colored_kinfu::Params* instance, cv::Matx33f* ocvrs_return) {
			cv::Matx33f ret = instance->intr;
			*ocvrs_return = ret;
	}

	// cv::colored_kinfu::Params::setIntr(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:93
	// ("cv::colored_kinfu::Params::setIntr", vec![(pred!(mut, ["val"], ["const cv::Matx33f"]), _)]),
	void cv_colored_kinfu_Params_propIntr_const_Matx33f(cv::colored_kinfu::Params* instance, const cv::Matx33f* val) {
			instance->intr = *val;
	}

	// cv::colored_kinfu::Params::rgb_intr() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:96
	// ("cv::colored_kinfu::Params::rgb_intr", vec![(pred!(const, [], []), _)]),
	void cv_colored_kinfu_Params_propRgb_intr_const(const cv::colored_kinfu::Params* instance, cv::Matx33f* ocvrs_return) {
			cv::Matx33f ret = instance->rgb_intr;
			*ocvrs_return = ret;
	}

	// cv::colored_kinfu::Params::setRgb_intr(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:96
	// ("cv::colored_kinfu::Params::setRgb_intr", vec![(pred!(mut, ["val"], ["const cv::Matx33f"]), _)]),
	void cv_colored_kinfu_Params_propRgb_intr_const_Matx33f(cv::colored_kinfu::Params* instance, const cv::Matx33f* val) {
			instance->rgb_intr = *val;
	}

	// cv::colored_kinfu::Params::depthFactor() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:105
	// ("cv::colored_kinfu::Params::depthFactor", vec![(pred!(const, [], []), _)]),
	float cv_colored_kinfu_Params_propDepthFactor_const(const cv::colored_kinfu::Params* instance) {
			float ret = instance->depthFactor;
			return ret;
	}

	// cv::colored_kinfu::Params::setDepthFactor(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:105
	// ("cv::colored_kinfu::Params::setDepthFactor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_colored_kinfu_Params_propDepthFactor_const_float(cv::colored_kinfu::Params* instance, const float val) {
			instance->depthFactor = val;
	}

	// cv::colored_kinfu::Params::bilateral_sigma_depth() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:108
	// ("cv::colored_kinfu::Params::bilateral_sigma_depth", vec![(pred!(const, [], []), _)]),
	float cv_colored_kinfu_Params_propBilateral_sigma_depth_const(const cv::colored_kinfu::Params* instance) {
			float ret = instance->bilateral_sigma_depth;
			return ret;
	}

	// cv::colored_kinfu::Params::setBilateral_sigma_depth(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:108
	// ("cv::colored_kinfu::Params::setBilateral_sigma_depth", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_colored_kinfu_Params_propBilateral_sigma_depth_const_float(cv::colored_kinfu::Params* instance, const float val) {
			instance->bilateral_sigma_depth = val;
	}

	// cv::colored_kinfu::Params::bilateral_sigma_spatial() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:110
	// ("cv::colored_kinfu::Params::bilateral_sigma_spatial", vec![(pred!(const, [], []), _)]),
	float cv_colored_kinfu_Params_propBilateral_sigma_spatial_const(const cv::colored_kinfu::Params* instance) {
			float ret = instance->bilateral_sigma_spatial;
			return ret;
	}

	// cv::colored_kinfu::Params::setBilateral_sigma_spatial(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:110
	// ("cv::colored_kinfu::Params::setBilateral_sigma_spatial", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_colored_kinfu_Params_propBilateral_sigma_spatial_const_float(cv::colored_kinfu::Params* instance, const float val) {
			instance->bilateral_sigma_spatial = val;
	}

	// cv::colored_kinfu::Params::bilateral_kernel_size() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:112
	// ("cv::colored_kinfu::Params::bilateral_kernel_size", vec![(pred!(const, [], []), _)]),
	int cv_colored_kinfu_Params_propBilateral_kernel_size_const(const cv::colored_kinfu::Params* instance) {
			int ret = instance->bilateral_kernel_size;
			return ret;
	}

	// cv::colored_kinfu::Params::setBilateral_kernel_size(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:112
	// ("cv::colored_kinfu::Params::setBilateral_kernel_size", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_colored_kinfu_Params_propBilateral_kernel_size_const_int(cv::colored_kinfu::Params* instance, const int val) {
			instance->bilateral_kernel_size = val;
	}

	// cv::colored_kinfu::Params::pyramidLevels() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:115
	// ("cv::colored_kinfu::Params::pyramidLevels", vec![(pred!(const, [], []), _)]),
	int cv_colored_kinfu_Params_propPyramidLevels_const(const cv::colored_kinfu::Params* instance) {
			int ret = instance->pyramidLevels;
			return ret;
	}

	// cv::colored_kinfu::Params::setPyramidLevels(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:115
	// ("cv::colored_kinfu::Params::setPyramidLevels", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_colored_kinfu_Params_propPyramidLevels_const_int(cv::colored_kinfu::Params* instance, const int val) {
			instance->pyramidLevels = val;
	}

	// cv::colored_kinfu::Params::volumeDims() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:121
	// ("cv::colored_kinfu::Params::volumeDims", vec![(pred!(const, [], []), _)]),
	void cv_colored_kinfu_Params_propVolumeDims_const(const cv::colored_kinfu::Params* instance, cv::Vec3i* ocvrs_return) {
			cv::Vec3i ret = instance->volumeDims;
			*ocvrs_return = ret;
	}

	// cv::colored_kinfu::Params::setVolumeDims(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:121
	// ("cv::colored_kinfu::Params::setVolumeDims", vec![(pred!(mut, ["val"], ["const cv::Vec3i"]), _)]),
	void cv_colored_kinfu_Params_propVolumeDims_const_Vec3i(cv::colored_kinfu::Params* instance, const cv::Vec3i* val) {
			instance->volumeDims = *val;
	}

	// cv::colored_kinfu::Params::voxelSize() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:123
	// ("cv::colored_kinfu::Params::voxelSize", vec![(pred!(const, [], []), _)]),
	float cv_colored_kinfu_Params_propVoxelSize_const(const cv::colored_kinfu::Params* instance) {
			float ret = instance->voxelSize;
			return ret;
	}

	// cv::colored_kinfu::Params::setVoxelSize(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:123
	// ("cv::colored_kinfu::Params::setVoxelSize", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_colored_kinfu_Params_propVoxelSize_const_float(cv::colored_kinfu::Params* instance, const float val) {
			instance->voxelSize = val;
	}

	// cv::colored_kinfu::Params::tsdf_min_camera_movement() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:129
	// ("cv::colored_kinfu::Params::tsdf_min_camera_movement", vec![(pred!(const, [], []), _)]),
	float cv_colored_kinfu_Params_propTsdf_min_camera_movement_const(const cv::colored_kinfu::Params* instance) {
			float ret = instance->tsdf_min_camera_movement;
			return ret;
	}

	// cv::colored_kinfu::Params::setTsdf_min_camera_movement(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:129
	// ("cv::colored_kinfu::Params::setTsdf_min_camera_movement", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_colored_kinfu_Params_propTsdf_min_camera_movement_const_float(cv::colored_kinfu::Params* instance, const float val) {
			instance->tsdf_min_camera_movement = val;
	}

	// cv::colored_kinfu::Params::volumePose() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:132
	// ("cv::colored_kinfu::Params::volumePose", vec![(pred!(const, [], []), _)]),
	void cv_colored_kinfu_Params_propVolumePose_const(const cv::colored_kinfu::Params* instance, cv::Matx44f* ocvrs_return) {
			cv::Matx44f ret = instance->volumePose;
			*ocvrs_return = ret;
	}

	// cv::colored_kinfu::Params::setVolumePose(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:132
	// ("cv::colored_kinfu::Params::setVolumePose", vec![(pred!(mut, ["val"], ["const cv::Matx44f"]), _)]),
	void cv_colored_kinfu_Params_propVolumePose_const_Matx44f(cv::colored_kinfu::Params* instance, const cv::Matx44f* val) {
			instance->volumePose = *val;
	}

	// cv::colored_kinfu::Params::tsdf_trunc_dist() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:138
	// ("cv::colored_kinfu::Params::tsdf_trunc_dist", vec![(pred!(const, [], []), _)]),
	float cv_colored_kinfu_Params_propTsdf_trunc_dist_const(const cv::colored_kinfu::Params* instance) {
			float ret = instance->tsdf_trunc_dist;
			return ret;
	}

	// cv::colored_kinfu::Params::setTsdf_trunc_dist(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:138
	// ("cv::colored_kinfu::Params::setTsdf_trunc_dist", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_colored_kinfu_Params_propTsdf_trunc_dist_const_float(cv::colored_kinfu::Params* instance, const float val) {
			instance->tsdf_trunc_dist = val;
	}

	// cv::colored_kinfu::Params::tsdf_max_weight() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:144
	// ("cv::colored_kinfu::Params::tsdf_max_weight", vec![(pred!(const, [], []), _)]),
	int cv_colored_kinfu_Params_propTsdf_max_weight_const(const cv::colored_kinfu::Params* instance) {
			int ret = instance->tsdf_max_weight;
			return ret;
	}

	// cv::colored_kinfu::Params::setTsdf_max_weight(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:144
	// ("cv::colored_kinfu::Params::setTsdf_max_weight", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_colored_kinfu_Params_propTsdf_max_weight_const_int(cv::colored_kinfu::Params* instance, const int val) {
			instance->tsdf_max_weight = val;
	}

	// cv::colored_kinfu::Params::raycast_step_factor() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:150
	// ("cv::colored_kinfu::Params::raycast_step_factor", vec![(pred!(const, [], []), _)]),
	float cv_colored_kinfu_Params_propRaycast_step_factor_const(const cv::colored_kinfu::Params* instance) {
			float ret = instance->raycast_step_factor;
			return ret;
	}

	// cv::colored_kinfu::Params::setRaycast_step_factor(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:150
	// ("cv::colored_kinfu::Params::setRaycast_step_factor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_colored_kinfu_Params_propRaycast_step_factor_const_float(cv::colored_kinfu::Params* instance, const float val) {
			instance->raycast_step_factor = val;
	}

	// cv::colored_kinfu::Params::lightPose() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:157
	// ("cv::colored_kinfu::Params::lightPose", vec![(pred!(const, [], []), _)]),
	void cv_colored_kinfu_Params_propLightPose_const(const cv::colored_kinfu::Params* instance, cv::Vec3f* ocvrs_return) {
			cv::Vec3f ret = instance->lightPose;
			*ocvrs_return = ret;
	}

	// cv::colored_kinfu::Params::setLightPose(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:157
	// ("cv::colored_kinfu::Params::setLightPose", vec![(pred!(mut, ["val"], ["const cv::Vec3f"]), _)]),
	void cv_colored_kinfu_Params_propLightPose_const_Vec3f(cv::colored_kinfu::Params* instance, const cv::Vec3f* val) {
			instance->lightPose = *val;
	}

	// cv::colored_kinfu::Params::icpDistThresh() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:160
	// ("cv::colored_kinfu::Params::icpDistThresh", vec![(pred!(const, [], []), _)]),
	float cv_colored_kinfu_Params_propIcpDistThresh_const(const cv::colored_kinfu::Params* instance) {
			float ret = instance->icpDistThresh;
			return ret;
	}

	// cv::colored_kinfu::Params::setIcpDistThresh(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:160
	// ("cv::colored_kinfu::Params::setIcpDistThresh", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_colored_kinfu_Params_propIcpDistThresh_const_float(cv::colored_kinfu::Params* instance, const float val) {
			instance->icpDistThresh = val;
	}

	// cv::colored_kinfu::Params::icpAngleThresh() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:162
	// ("cv::colored_kinfu::Params::icpAngleThresh", vec![(pred!(const, [], []), _)]),
	float cv_colored_kinfu_Params_propIcpAngleThresh_const(const cv::colored_kinfu::Params* instance) {
			float ret = instance->icpAngleThresh;
			return ret;
	}

	// cv::colored_kinfu::Params::setIcpAngleThresh(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:162
	// ("cv::colored_kinfu::Params::setIcpAngleThresh", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_colored_kinfu_Params_propIcpAngleThresh_const_float(cv::colored_kinfu::Params* instance, const float val) {
			instance->icpAngleThresh = val;
	}

	// cv::colored_kinfu::Params::icpIterations() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:164
	// ("cv::colored_kinfu::Params::icpIterations", vec![(pred!(const, [], []), _)]),
	std::vector<int>* cv_colored_kinfu_Params_propIcpIterations_const(const cv::colored_kinfu::Params* instance) {
			std::vector<int> ret = instance->icpIterations;
			return new std::vector<int>(ret);
	}

	// cv::colored_kinfu::Params::setIcpIterations(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:164
	// ("cv::colored_kinfu::Params::setIcpIterations", vec![(pred!(mut, ["val"], ["const std::vector<int>"]), _)]),
	void cv_colored_kinfu_Params_propIcpIterations_const_vectorLintG(cv::colored_kinfu::Params* instance, const std::vector<int>* val) {
			instance->icpIterations = *val;
	}

	// cv::colored_kinfu::Params::truncateThreshold() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:170
	// ("cv::colored_kinfu::Params::truncateThreshold", vec![(pred!(const, [], []), _)]),
	float cv_colored_kinfu_Params_propTruncateThreshold_const(const cv::colored_kinfu::Params* instance) {
			float ret = instance->truncateThreshold;
			return ret;
	}

	// cv::colored_kinfu::Params::setTruncateThreshold(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/colored_kinfu.hpp:170
	// ("cv::colored_kinfu::Params::setTruncateThreshold", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_colored_kinfu_Params_propTruncateThreshold_const_float(cv::colored_kinfu::Params* instance, const float val) {
			instance->truncateThreshold = val;
	}

	// cv::colored_kinfu::Params::delete() generated
	// ("cv::colored_kinfu::Params::delete", vec![(pred!(mut, [], []), _)]),
	void cv_colored_kinfu_Params_delete(cv::colored_kinfu::Params* instance) {
			delete instance;
	}

	// create(const Ptr<kinfu::Params> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/dynafu.hpp:49
	// ("cv::dynafu::DynaFu::create", vec![(pred!(mut, ["_params"], ["const cv::Ptr<cv::kinfu::Params>*"]), _)]),
	void cv_dynafu_DynaFu_create_const_PtrLParamsGR(const cv::Ptr<cv::kinfu::Params>* _params, Result<cv::Ptr<cv::dynafu::DynaFu>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dynafu::DynaFu> ret = cv::dynafu::DynaFu::create(*_params);
			Ok(new cv::Ptr<cv::dynafu::DynaFu>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getParams()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/dynafu.hpp:53
	// ("cv::dynafu::DynaFu::getParams", vec![(pred!(const, [], []), _)]),
	void cv_dynafu_DynaFu_getParams_const(const cv::dynafu::DynaFu* instance, Result<cv::kinfu::Params*>* ocvrs_return) {
		try {
			const cv::kinfu::Params ret = instance->getParams();
			Ok(new const cv::kinfu::Params(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// render(OutputArray, const Matx44f &)(OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/dynafu.hpp:65
	// ("cv::dynafu::DynaFu::render", vec![(pred!(const, ["image", "cameraPose"], ["const cv::_OutputArray*", "const cv::Matx44f*"]), _)]),
	void cv_dynafu_DynaFu_render_const_const__OutputArrayR_const_Matx44fR(const cv::dynafu::DynaFu* instance, const cv::_OutputArray* image, const cv::Matx44f* cameraPose, ResultVoid* ocvrs_return) {
		try {
			instance->render(*image, *cameraPose);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dynafu::DynaFu::render(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/dynafu.hpp:65
	// ("cv::dynafu::DynaFu::render", vec![(pred!(const, ["image"], ["const cv::_OutputArray*"]), _)]),
	void cv_dynafu_DynaFu_render_const_const__OutputArrayR(const cv::dynafu::DynaFu* instance, const cv::_OutputArray* image, ResultVoid* ocvrs_return) {
		try {
			instance->render(*image);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getCloud(OutputArray, OutputArray)(OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/dynafu.hpp:75
	// ("cv::dynafu::DynaFu::getCloud", vec![(pred!(const, ["points", "normals"], ["const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_dynafu_DynaFu_getCloud_const_const__OutputArrayR_const__OutputArrayR(const cv::dynafu::DynaFu* instance, const cv::_OutputArray* points, const cv::_OutputArray* normals, ResultVoid* ocvrs_return) {
		try {
			instance->getCloud(*points, *normals);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPoints(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/dynafu.hpp:83
	// ("cv::dynafu::DynaFu::getPoints", vec![(pred!(const, ["points"], ["const cv::_OutputArray*"]), _)]),
	void cv_dynafu_DynaFu_getPoints_const_const__OutputArrayR(const cv::dynafu::DynaFu* instance, const cv::_OutputArray* points, ResultVoid* ocvrs_return) {
		try {
			instance->getPoints(*points);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNormals(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/dynafu.hpp:89
	// ("cv::dynafu::DynaFu::getNormals", vec![(pred!(const, ["points", "normals"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_dynafu_DynaFu_getNormals_const_const__InputArrayR_const__OutputArrayR(const cv::dynafu::DynaFu* instance, const cv::_InputArray* points, const cv::_OutputArray* normals, ResultVoid* ocvrs_return) {
		try {
			instance->getNormals(*points, *normals);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// reset()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/dynafu.hpp:95
	// ("cv::dynafu::DynaFu::reset", vec![(pred!(mut, [], []), _)]),
	void cv_dynafu_DynaFu_reset(cv::dynafu::DynaFu* instance, ResultVoid* ocvrs_return) {
		try {
			instance->reset();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPose()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/dynafu.hpp:98
	// ("cv::dynafu::DynaFu::getPose", vec![(pred!(const, [], []), _)]),
	void cv_dynafu_DynaFu_getPose_const(const cv::dynafu::DynaFu* instance, Result<cv::Affine3f>* ocvrs_return) {
		try {
			cv::Affine3f ret = instance->getPose();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// update(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/dynafu.hpp:108
	// ("cv::dynafu::DynaFu::update", vec![(pred!(mut, ["depth"], ["const cv::_InputArray*"]), _)]),
	void cv_dynafu_DynaFu_update_const__InputArrayR(cv::dynafu::DynaFu* instance, const cv::_InputArray* depth, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->update(*depth);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNodesPos()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/dynafu.hpp:110
	// ("cv::dynafu::DynaFu::getNodesPos", vec![(pred!(const, [], []), _)]),
	void cv_dynafu_DynaFu_getNodesPos_const(const cv::dynafu::DynaFu* instance, Result<std::vector<cv::Point3f>*>* ocvrs_return) {
		try {
			std::vector<cv::Point3f> ret = instance->getNodesPos();
			Ok(new std::vector<cv::Point3f>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// marchCubes(OutputArray, OutputArray)(OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/dynafu.hpp:112
	// ("cv::dynafu::DynaFu::marchCubes", vec![(pred!(const, ["vertices", "edges"], ["const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_dynafu_DynaFu_marchCubes_const_const__OutputArrayR_const__OutputArrayR(const cv::dynafu::DynaFu* instance, const cv::_OutputArray* vertices, const cv::_OutputArray* edges, ResultVoid* ocvrs_return) {
		try {
			instance->marchCubes(*vertices, *edges);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// renderSurface(OutputArray, OutputArray, OutputArray, bool)(OutputArray, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/dynafu.hpp:114
	// ("cv::dynafu::DynaFu::renderSurface", vec![(pred!(mut, ["depthImage", "vertImage", "normImage", "warp"], ["const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "bool"]), _)]),
	void cv_dynafu_DynaFu_renderSurface_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_bool(cv::dynafu::DynaFu* instance, const cv::_OutputArray* depthImage, const cv::_OutputArray* vertImage, const cv::_OutputArray* normImage, bool warp, ResultVoid* ocvrs_return) {
		try {
			instance->renderSurface(*depthImage, *vertImage, *normImage, warp);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dynafu::DynaFu::renderSurface(OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/dynafu.hpp:114
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

	// create(const Ptr<Params> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:252
	// ("cv::kinfu::KinFu::create", vec![(pred!(mut, ["_params"], ["const cv::Ptr<cv::kinfu::Params>*"]), _)]),
	void cv_kinfu_KinFu_create_const_PtrLParamsGR(const cv::Ptr<cv::kinfu::Params>* _params, Result<cv::Ptr<cv::kinfu::KinFu>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::kinfu::KinFu> ret = cv::kinfu::KinFu::create(*_params);
			Ok(new cv::Ptr<cv::kinfu::KinFu>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getParams()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:256
	// ("cv::kinfu::KinFu::getParams", vec![(pred!(const, [], []), _)]),
	void cv_kinfu_KinFu_getParams_const(const cv::kinfu::KinFu* instance, Result<cv::kinfu::Params*>* ocvrs_return) {
		try {
			const cv::kinfu::Params ret = instance->getParams();
			Ok(new const cv::kinfu::Params(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// render(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:266
	// ("cv::kinfu::KinFu::render", vec![(pred!(const, ["image"], ["const cv::_OutputArray*"]), _)]),
	void cv_kinfu_KinFu_render_const_const__OutputArrayR(const cv::kinfu::KinFu* instance, const cv::_OutputArray* image, ResultVoid* ocvrs_return) {
		try {
			instance->render(*image);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// render(OutputArray, const Matx44f &)(OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:278
	// ("cv::kinfu::KinFu::render", vec![(pred!(const, ["image", "cameraPose"], ["const cv::_OutputArray*", "const cv::Matx44f*"]), _)]),
	void cv_kinfu_KinFu_render_const_const__OutputArrayR_const_Matx44fR(const cv::kinfu::KinFu* instance, const cv::_OutputArray* image, const cv::Matx44f* cameraPose, ResultVoid* ocvrs_return) {
		try {
			instance->render(*image, *cameraPose);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getCloud(OutputArray, OutputArray)(OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:288
	// ("cv::kinfu::KinFu::getCloud", vec![(pred!(const, ["points", "normals"], ["const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_kinfu_KinFu_getCloud_const_const__OutputArrayR_const__OutputArrayR(const cv::kinfu::KinFu* instance, const cv::_OutputArray* points, const cv::_OutputArray* normals, ResultVoid* ocvrs_return) {
		try {
			instance->getCloud(*points, *normals);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPoints(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:296
	// ("cv::kinfu::KinFu::getPoints", vec![(pred!(const, ["points"], ["const cv::_OutputArray*"]), _)]),
	void cv_kinfu_KinFu_getPoints_const_const__OutputArrayR(const cv::kinfu::KinFu* instance, const cv::_OutputArray* points, ResultVoid* ocvrs_return) {
		try {
			instance->getPoints(*points);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNormals(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:302
	// ("cv::kinfu::KinFu::getNormals", vec![(pred!(const, ["points", "normals"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_kinfu_KinFu_getNormals_const_const__InputArrayR_const__OutputArrayR(const cv::kinfu::KinFu* instance, const cv::_InputArray* points, const cv::_OutputArray* normals, ResultVoid* ocvrs_return) {
		try {
			instance->getNormals(*points, *normals);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// reset()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:308
	// ("cv::kinfu::KinFu::reset", vec![(pred!(mut, [], []), _)]),
	void cv_kinfu_KinFu_reset(cv::kinfu::KinFu* instance, ResultVoid* ocvrs_return) {
		try {
			instance->reset();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPose()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:311
	// ("cv::kinfu::KinFu::getPose", vec![(pred!(const, [], []), _)]),
	void cv_kinfu_KinFu_getPose_const(const cv::kinfu::KinFu* instance, Result<cv::Affine3f>* ocvrs_return) {
		try {
			cv::Affine3f ret = instance->getPose();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// update(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:321
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

	// Params()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:76
	// ("cv::kinfu::Params::Params", vec![(pred!(mut, [], []), _)]),
	void cv_kinfu_Params_Params(Result<cv::kinfu::Params*>* ocvrs_return) {
		try {
			cv::kinfu::Params* ret = new cv::kinfu::Params();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Params(Matx33f, Vec3f)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:87
	// ("cv::kinfu::Params::Params", vec![(pred!(mut, ["volumeInitialPoseRot", "volumeInitialPoseTransl"], ["cv::Matx33f", "cv::Vec3f"]), _)]),
	void cv_kinfu_Params_Params_Matx33f_Vec3f(cv::Matx33f* volumeInitialPoseRot, cv::Vec3f* volumeInitialPoseTransl, Result<cv::kinfu::Params*>* ocvrs_return) {
		try {
			cv::kinfu::Params* ret = new cv::kinfu::Params(*volumeInitialPoseRot, *volumeInitialPoseTransl);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Params(Matx44f)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:97
	// ("cv::kinfu::Params::Params", vec![(pred!(mut, ["volumeInitialPose"], ["cv::Matx44f"]), _)]),
	void cv_kinfu_Params_Params_Matx44f(cv::Matx44f* volumeInitialPose, Result<cv::kinfu::Params*>* ocvrs_return) {
		try {
			cv::kinfu::Params* ret = new cv::kinfu::Params(*volumeInitialPose);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setInitialVolumePose(Matx33f, Vec3f)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:108
	// ("cv::kinfu::Params::setInitialVolumePose", vec![(pred!(mut, ["R", "t"], ["cv::Matx33f", "cv::Vec3f"]), _)]),
	void cv_kinfu_Params_setInitialVolumePose_Matx33f_Vec3f(cv::kinfu::Params* instance, cv::Matx33f* R, cv::Vec3f* t, ResultVoid* ocvrs_return) {
		try {
			instance->setInitialVolumePose(*R, *t);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setInitialVolumePose(Matx44f)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:115
	// ("cv::kinfu::Params::setInitialVolumePose", vec![(pred!(mut, ["homogen_tf"], ["cv::Matx44f"]), _)]),
	void cv_kinfu_Params_setInitialVolumePose_Matx44f(cv::kinfu::Params* instance, cv::Matx44f* homogen_tf, ResultVoid* ocvrs_return) {
		try {
			instance->setInitialVolumePose(*homogen_tf);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// defaultParams()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:121
	// ("cv::kinfu::Params::defaultParams", vec![(pred!(mut, [], []), _)]),
	void cv_kinfu_Params_defaultParams(Result<cv::Ptr<cv::kinfu::Params>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::kinfu::Params> ret = cv::kinfu::Params::defaultParams();
			Ok(new cv::Ptr<cv::kinfu::Params>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// coarseParams()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:127
	// ("cv::kinfu::Params::coarseParams", vec![(pred!(mut, [], []), _)]),
	void cv_kinfu_Params_coarseParams(Result<cv::Ptr<cv::kinfu::Params>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::kinfu::Params> ret = cv::kinfu::Params::coarseParams();
			Ok(new cv::Ptr<cv::kinfu::Params>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// hashTSDFParams(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:132
	// ("cv::kinfu::Params::hashTSDFParams", vec![(pred!(mut, ["isCoarse"], ["bool"]), _)]),
	void cv_kinfu_Params_hashTSDFParams_bool(bool isCoarse, Result<cv::Ptr<cv::kinfu::Params>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::kinfu::Params> ret = cv::kinfu::Params::hashTSDFParams(isCoarse);
			Ok(new cv::Ptr<cv::kinfu::Params>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// coloredTSDFParams(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:137
	// ("cv::kinfu::Params::coloredTSDFParams", vec![(pred!(mut, ["isCoarse"], ["bool"]), _)]),
	void cv_kinfu_Params_coloredTSDFParams_bool(bool isCoarse, Result<cv::Ptr<cv::kinfu::Params>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::kinfu::Params> ret = cv::kinfu::Params::coloredTSDFParams(isCoarse);
			Ok(new cv::Ptr<cv::kinfu::Params>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::kinfu::Params::frameSize() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:140
	// ("cv::kinfu::Params::frameSize", vec![(pred!(const, [], []), _)]),
	void cv_kinfu_Params_propFrameSize_const(const cv::kinfu::Params* instance, cv::Size* ocvrs_return) {
			cv::Size ret = instance->frameSize;
			*ocvrs_return = ret;
	}

	// cv::kinfu::Params::setFrameSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:140
	// ("cv::kinfu::Params::setFrameSize", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	void cv_kinfu_Params_propFrameSize_const_Size(cv::kinfu::Params* instance, const cv::Size* val) {
			instance->frameSize = *val;
	}

	// cv::kinfu::Params::volumeKind() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:143
	// ("cv::kinfu::Params::volumeKind", vec![(pred!(const, [], []), _)]),
	void cv_kinfu_Params_propVolumeKind_const(const cv::kinfu::Params* instance, cv::VolumeType* ocvrs_return) {
			cv::VolumeType ret = instance->volumeKind;
			*ocvrs_return = ret;
	}

	// cv::kinfu::Params::setVolumeKind(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:143
	// ("cv::kinfu::Params::setVolumeKind", vec![(pred!(mut, ["val"], ["const cv::VolumeType"]), _)]),
	void cv_kinfu_Params_propVolumeKind_const_VolumeType(cv::kinfu::Params* instance, const cv::VolumeType val) {
			instance->volumeKind = val;
	}

	// cv::kinfu::Params::intr() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:146
	// ("cv::kinfu::Params::intr", vec![(pred!(const, [], []), _)]),
	void cv_kinfu_Params_propIntr_const(const cv::kinfu::Params* instance, cv::Matx33f* ocvrs_return) {
			cv::Matx33f ret = instance->intr;
			*ocvrs_return = ret;
	}

	// cv::kinfu::Params::setIntr(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:146
	// ("cv::kinfu::Params::setIntr", vec![(pred!(mut, ["val"], ["const cv::Matx33f"]), _)]),
	void cv_kinfu_Params_propIntr_const_Matx33f(cv::kinfu::Params* instance, const cv::Matx33f* val) {
			instance->intr = *val;
	}

	// cv::kinfu::Params::rgb_intr() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:149
	// ("cv::kinfu::Params::rgb_intr", vec![(pred!(const, [], []), _)]),
	void cv_kinfu_Params_propRgb_intr_const(const cv::kinfu::Params* instance, cv::Matx33f* ocvrs_return) {
			cv::Matx33f ret = instance->rgb_intr;
			*ocvrs_return = ret;
	}

	// cv::kinfu::Params::setRgb_intr(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:149
	// ("cv::kinfu::Params::setRgb_intr", vec![(pred!(mut, ["val"], ["const cv::Matx33f"]), _)]),
	void cv_kinfu_Params_propRgb_intr_const_Matx33f(cv::kinfu::Params* instance, const cv::Matx33f* val) {
			instance->rgb_intr = *val;
	}

	// cv::kinfu::Params::depthFactor() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:157
	// ("cv::kinfu::Params::depthFactor", vec![(pred!(const, [], []), _)]),
	float cv_kinfu_Params_propDepthFactor_const(const cv::kinfu::Params* instance) {
			float ret = instance->depthFactor;
			return ret;
	}

	// cv::kinfu::Params::setDepthFactor(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:157
	// ("cv::kinfu::Params::setDepthFactor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_kinfu_Params_propDepthFactor_const_float(cv::kinfu::Params* instance, const float val) {
			instance->depthFactor = val;
	}

	// cv::kinfu::Params::bilateral_sigma_depth() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:160
	// ("cv::kinfu::Params::bilateral_sigma_depth", vec![(pred!(const, [], []), _)]),
	float cv_kinfu_Params_propBilateral_sigma_depth_const(const cv::kinfu::Params* instance) {
			float ret = instance->bilateral_sigma_depth;
			return ret;
	}

	// cv::kinfu::Params::setBilateral_sigma_depth(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:160
	// ("cv::kinfu::Params::setBilateral_sigma_depth", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_kinfu_Params_propBilateral_sigma_depth_const_float(cv::kinfu::Params* instance, const float val) {
			instance->bilateral_sigma_depth = val;
	}

	// cv::kinfu::Params::bilateral_sigma_spatial() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:162
	// ("cv::kinfu::Params::bilateral_sigma_spatial", vec![(pred!(const, [], []), _)]),
	float cv_kinfu_Params_propBilateral_sigma_spatial_const(const cv::kinfu::Params* instance) {
			float ret = instance->bilateral_sigma_spatial;
			return ret;
	}

	// cv::kinfu::Params::setBilateral_sigma_spatial(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:162
	// ("cv::kinfu::Params::setBilateral_sigma_spatial", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_kinfu_Params_propBilateral_sigma_spatial_const_float(cv::kinfu::Params* instance, const float val) {
			instance->bilateral_sigma_spatial = val;
	}

	// cv::kinfu::Params::bilateral_kernel_size() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:164
	// ("cv::kinfu::Params::bilateral_kernel_size", vec![(pred!(const, [], []), _)]),
	int cv_kinfu_Params_propBilateral_kernel_size_const(const cv::kinfu::Params* instance) {
			int ret = instance->bilateral_kernel_size;
			return ret;
	}

	// cv::kinfu::Params::setBilateral_kernel_size(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:164
	// ("cv::kinfu::Params::setBilateral_kernel_size", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_kinfu_Params_propBilateral_kernel_size_const_int(cv::kinfu::Params* instance, const int val) {
			instance->bilateral_kernel_size = val;
	}

	// cv::kinfu::Params::pyramidLevels() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:167
	// ("cv::kinfu::Params::pyramidLevels", vec![(pred!(const, [], []), _)]),
	int cv_kinfu_Params_propPyramidLevels_const(const cv::kinfu::Params* instance) {
			int ret = instance->pyramidLevels;
			return ret;
	}

	// cv::kinfu::Params::setPyramidLevels(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:167
	// ("cv::kinfu::Params::setPyramidLevels", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_kinfu_Params_propPyramidLevels_const_int(cv::kinfu::Params* instance, const int val) {
			instance->pyramidLevels = val;
	}

	// cv::kinfu::Params::volumeDims() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:173
	// ("cv::kinfu::Params::volumeDims", vec![(pred!(const, [], []), _)]),
	void cv_kinfu_Params_propVolumeDims_const(const cv::kinfu::Params* instance, cv::Vec3i* ocvrs_return) {
			cv::Vec3i ret = instance->volumeDims;
			*ocvrs_return = ret;
	}

	// cv::kinfu::Params::setVolumeDims(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:173
	// ("cv::kinfu::Params::setVolumeDims", vec![(pred!(mut, ["val"], ["const cv::Vec3i"]), _)]),
	void cv_kinfu_Params_propVolumeDims_const_Vec3i(cv::kinfu::Params* instance, const cv::Vec3i* val) {
			instance->volumeDims = *val;
	}

	// cv::kinfu::Params::voxelSize() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:175
	// ("cv::kinfu::Params::voxelSize", vec![(pred!(const, [], []), _)]),
	float cv_kinfu_Params_propVoxelSize_const(const cv::kinfu::Params* instance) {
			float ret = instance->voxelSize;
			return ret;
	}

	// cv::kinfu::Params::setVoxelSize(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:175
	// ("cv::kinfu::Params::setVoxelSize", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_kinfu_Params_propVoxelSize_const_float(cv::kinfu::Params* instance, const float val) {
			instance->voxelSize = val;
	}

	// cv::kinfu::Params::tsdf_min_camera_movement() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:181
	// ("cv::kinfu::Params::tsdf_min_camera_movement", vec![(pred!(const, [], []), _)]),
	float cv_kinfu_Params_propTsdf_min_camera_movement_const(const cv::kinfu::Params* instance) {
			float ret = instance->tsdf_min_camera_movement;
			return ret;
	}

	// cv::kinfu::Params::setTsdf_min_camera_movement(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:181
	// ("cv::kinfu::Params::setTsdf_min_camera_movement", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_kinfu_Params_propTsdf_min_camera_movement_const_float(cv::kinfu::Params* instance, const float val) {
			instance->tsdf_min_camera_movement = val;
	}

	// cv::kinfu::Params::volumePose() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:184
	// ("cv::kinfu::Params::volumePose", vec![(pred!(const, [], []), _)]),
	void cv_kinfu_Params_propVolumePose_const(const cv::kinfu::Params* instance, cv::Matx44f* ocvrs_return) {
			cv::Matx44f ret = instance->volumePose;
			*ocvrs_return = ret;
	}

	// cv::kinfu::Params::setVolumePose(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:184
	// ("cv::kinfu::Params::setVolumePose", vec![(pred!(mut, ["val"], ["const cv::Matx44f"]), _)]),
	void cv_kinfu_Params_propVolumePose_const_Matx44f(cv::kinfu::Params* instance, const cv::Matx44f* val) {
			instance->volumePose = *val;
	}

	// cv::kinfu::Params::tsdf_trunc_dist() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:190
	// ("cv::kinfu::Params::tsdf_trunc_dist", vec![(pred!(const, [], []), _)]),
	float cv_kinfu_Params_propTsdf_trunc_dist_const(const cv::kinfu::Params* instance) {
			float ret = instance->tsdf_trunc_dist;
			return ret;
	}

	// cv::kinfu::Params::setTsdf_trunc_dist(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:190
	// ("cv::kinfu::Params::setTsdf_trunc_dist", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_kinfu_Params_propTsdf_trunc_dist_const_float(cv::kinfu::Params* instance, const float val) {
			instance->tsdf_trunc_dist = val;
	}

	// cv::kinfu::Params::tsdf_max_weight() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:196
	// ("cv::kinfu::Params::tsdf_max_weight", vec![(pred!(const, [], []), _)]),
	int cv_kinfu_Params_propTsdf_max_weight_const(const cv::kinfu::Params* instance) {
			int ret = instance->tsdf_max_weight;
			return ret;
	}

	// cv::kinfu::Params::setTsdf_max_weight(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:196
	// ("cv::kinfu::Params::setTsdf_max_weight", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_kinfu_Params_propTsdf_max_weight_const_int(cv::kinfu::Params* instance, const int val) {
			instance->tsdf_max_weight = val;
	}

	// cv::kinfu::Params::raycast_step_factor() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:202
	// ("cv::kinfu::Params::raycast_step_factor", vec![(pred!(const, [], []), _)]),
	float cv_kinfu_Params_propRaycast_step_factor_const(const cv::kinfu::Params* instance) {
			float ret = instance->raycast_step_factor;
			return ret;
	}

	// cv::kinfu::Params::setRaycast_step_factor(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:202
	// ("cv::kinfu::Params::setRaycast_step_factor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_kinfu_Params_propRaycast_step_factor_const_float(cv::kinfu::Params* instance, const float val) {
			instance->raycast_step_factor = val;
	}

	// cv::kinfu::Params::lightPose() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:209
	// ("cv::kinfu::Params::lightPose", vec![(pred!(const, [], []), _)]),
	void cv_kinfu_Params_propLightPose_const(const cv::kinfu::Params* instance, cv::Vec3f* ocvrs_return) {
			cv::Vec3f ret = instance->lightPose;
			*ocvrs_return = ret;
	}

	// cv::kinfu::Params::setLightPose(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:209
	// ("cv::kinfu::Params::setLightPose", vec![(pred!(mut, ["val"], ["const cv::Vec3f"]), _)]),
	void cv_kinfu_Params_propLightPose_const_Vec3f(cv::kinfu::Params* instance, const cv::Vec3f* val) {
			instance->lightPose = *val;
	}

	// cv::kinfu::Params::icpDistThresh() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:212
	// ("cv::kinfu::Params::icpDistThresh", vec![(pred!(const, [], []), _)]),
	float cv_kinfu_Params_propIcpDistThresh_const(const cv::kinfu::Params* instance) {
			float ret = instance->icpDistThresh;
			return ret;
	}

	// cv::kinfu::Params::setIcpDistThresh(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:212
	// ("cv::kinfu::Params::setIcpDistThresh", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_kinfu_Params_propIcpDistThresh_const_float(cv::kinfu::Params* instance, const float val) {
			instance->icpDistThresh = val;
	}

	// cv::kinfu::Params::icpAngleThresh() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:214
	// ("cv::kinfu::Params::icpAngleThresh", vec![(pred!(const, [], []), _)]),
	float cv_kinfu_Params_propIcpAngleThresh_const(const cv::kinfu::Params* instance) {
			float ret = instance->icpAngleThresh;
			return ret;
	}

	// cv::kinfu::Params::setIcpAngleThresh(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:214
	// ("cv::kinfu::Params::setIcpAngleThresh", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_kinfu_Params_propIcpAngleThresh_const_float(cv::kinfu::Params* instance, const float val) {
			instance->icpAngleThresh = val;
	}

	// cv::kinfu::Params::icpIterations() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:216
	// ("cv::kinfu::Params::icpIterations", vec![(pred!(const, [], []), _)]),
	std::vector<int>* cv_kinfu_Params_propIcpIterations_const(const cv::kinfu::Params* instance) {
			std::vector<int> ret = instance->icpIterations;
			return new std::vector<int>(ret);
	}

	// cv::kinfu::Params::setIcpIterations(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:216
	// ("cv::kinfu::Params::setIcpIterations", vec![(pred!(mut, ["val"], ["const std::vector<int>"]), _)]),
	void cv_kinfu_Params_propIcpIterations_const_vectorLintG(cv::kinfu::Params* instance, const std::vector<int>* val) {
			instance->icpIterations = *val;
	}

	// cv::kinfu::Params::truncateThreshold() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:222
	// ("cv::kinfu::Params::truncateThreshold", vec![(pred!(const, [], []), _)]),
	float cv_kinfu_Params_propTruncateThreshold_const(const cv::kinfu::Params* instance) {
			float ret = instance->truncateThreshold;
			return ret;
	}

	// cv::kinfu::Params::setTruncateThreshold(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:222
	// ("cv::kinfu::Params::setTruncateThreshold", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_kinfu_Params_propTruncateThreshold_const_float(cv::kinfu::Params* instance, const float val) {
			instance->truncateThreshold = val;
	}

	// cv::kinfu::Params::delete() generated
	// ("cv::kinfu::Params::delete", vec![(pred!(mut, [], []), _)]),
	void cv_kinfu_Params_delete(cv::kinfu::Params* instance) {
			delete instance;
	}

	// cv::kinfu::VolumeParams::defaultNew() generated
	// ("cv::kinfu::VolumeParams::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::kinfu::VolumeParams* cv_kinfu_VolumeParams_defaultNew_const() {
			cv::kinfu::VolumeParams* ret = new cv::kinfu::VolumeParams();
			return ret;
	}

	// cv::kinfu::VolumeParams::kind() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:24
	// ("cv::kinfu::VolumeParams::kind", vec![(pred!(const, [], []), _)]),
	void cv_kinfu_VolumeParams_propKind_const(const cv::kinfu::VolumeParams* instance, cv::VolumeType* ocvrs_return) {
			cv::VolumeType ret = instance->kind;
			*ocvrs_return = ret;
	}

	// cv::kinfu::VolumeParams::setKind(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:24
	// ("cv::kinfu::VolumeParams::setKind", vec![(pred!(mut, ["val"], ["const cv::VolumeType"]), _)]),
	void cv_kinfu_VolumeParams_propKind_const_VolumeType(cv::kinfu::VolumeParams* instance, const cv::VolumeType val) {
			instance->kind = val;
	}

	// cv::kinfu::VolumeParams::resolutionX() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:31
	// ("cv::kinfu::VolumeParams::resolutionX", vec![(pred!(const, [], []), _)]),
	int cv_kinfu_VolumeParams_propResolutionX_const(const cv::kinfu::VolumeParams* instance) {
			int ret = instance->resolutionX;
			return ret;
	}

	// cv::kinfu::VolumeParams::setResolutionX(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:31
	// ("cv::kinfu::VolumeParams::setResolutionX", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_kinfu_VolumeParams_propResolutionX_const_int(cv::kinfu::VolumeParams* instance, const int val) {
			instance->resolutionX = val;
	}

	// cv::kinfu::VolumeParams::resolutionY() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:32
	// ("cv::kinfu::VolumeParams::resolutionY", vec![(pred!(const, [], []), _)]),
	int cv_kinfu_VolumeParams_propResolutionY_const(const cv::kinfu::VolumeParams* instance) {
			int ret = instance->resolutionY;
			return ret;
	}

	// cv::kinfu::VolumeParams::setResolutionY(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:32
	// ("cv::kinfu::VolumeParams::setResolutionY", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_kinfu_VolumeParams_propResolutionY_const_int(cv::kinfu::VolumeParams* instance, const int val) {
			instance->resolutionY = val;
	}

	// cv::kinfu::VolumeParams::resolutionZ() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:33
	// ("cv::kinfu::VolumeParams::resolutionZ", vec![(pred!(const, [], []), _)]),
	int cv_kinfu_VolumeParams_propResolutionZ_const(const cv::kinfu::VolumeParams* instance) {
			int ret = instance->resolutionZ;
			return ret;
	}

	// cv::kinfu::VolumeParams::setResolutionZ(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:33
	// ("cv::kinfu::VolumeParams::setResolutionZ", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_kinfu_VolumeParams_propResolutionZ_const_int(cv::kinfu::VolumeParams* instance, const int val) {
			instance->resolutionZ = val;
	}

	// cv::kinfu::VolumeParams::unitResolution() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:39
	// ("cv::kinfu::VolumeParams::unitResolution", vec![(pred!(const, [], []), _)]),
	int cv_kinfu_VolumeParams_propUnitResolution_const(const cv::kinfu::VolumeParams* instance) {
			int ret = instance->unitResolution;
			return ret;
	}

	// cv::kinfu::VolumeParams::setUnitResolution(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:39
	// ("cv::kinfu::VolumeParams::setUnitResolution", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_kinfu_VolumeParams_propUnitResolution_const_int(cv::kinfu::VolumeParams* instance, const int val) {
			instance->unitResolution = val;
	}

	// cv::kinfu::VolumeParams::volumSize() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:42
	// ("cv::kinfu::VolumeParams::volumSize", vec![(pred!(const, [], []), _)]),
	float cv_kinfu_VolumeParams_propVolumSize_const(const cv::kinfu::VolumeParams* instance) {
			float ret = instance->volumSize;
			return ret;
	}

	// cv::kinfu::VolumeParams::setVolumSize(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:42
	// ("cv::kinfu::VolumeParams::setVolumSize", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_kinfu_VolumeParams_propVolumSize_const_float(cv::kinfu::VolumeParams* instance, const float val) {
			instance->volumSize = val;
	}

	// cv::kinfu::VolumeParams::pose() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:45
	// ("cv::kinfu::VolumeParams::pose", vec![(pred!(const, [], []), _)]),
	void cv_kinfu_VolumeParams_propPose_const(const cv::kinfu::VolumeParams* instance, cv::Matx44f* ocvrs_return) {
			cv::Matx44f ret = instance->pose;
			*ocvrs_return = ret;
	}

	// cv::kinfu::VolumeParams::setPose(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:45
	// ("cv::kinfu::VolumeParams::setPose", vec![(pred!(mut, ["val"], ["const cv::Matx44f"]), _)]),
	void cv_kinfu_VolumeParams_propPose_const_Matx44f(cv::kinfu::VolumeParams* instance, const cv::Matx44f* val) {
			instance->pose = *val;
	}

	// cv::kinfu::VolumeParams::voxelSize() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:48
	// ("cv::kinfu::VolumeParams::voxelSize", vec![(pred!(const, [], []), _)]),
	float cv_kinfu_VolumeParams_propVoxelSize_const(const cv::kinfu::VolumeParams* instance) {
			float ret = instance->voxelSize;
			return ret;
	}

	// cv::kinfu::VolumeParams::setVoxelSize(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:48
	// ("cv::kinfu::VolumeParams::setVoxelSize", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_kinfu_VolumeParams_propVoxelSize_const_float(cv::kinfu::VolumeParams* instance, const float val) {
			instance->voxelSize = val;
	}

	// cv::kinfu::VolumeParams::tsdfTruncDist() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:53
	// ("cv::kinfu::VolumeParams::tsdfTruncDist", vec![(pred!(const, [], []), _)]),
	float cv_kinfu_VolumeParams_propTsdfTruncDist_const(const cv::kinfu::VolumeParams* instance) {
			float ret = instance->tsdfTruncDist;
			return ret;
	}

	// cv::kinfu::VolumeParams::setTsdfTruncDist(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:53
	// ("cv::kinfu::VolumeParams::setTsdfTruncDist", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_kinfu_VolumeParams_propTsdfTruncDist_const_float(cv::kinfu::VolumeParams* instance, const float val) {
			instance->tsdfTruncDist = val;
	}

	// cv::kinfu::VolumeParams::maxWeight() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:59
	// ("cv::kinfu::VolumeParams::maxWeight", vec![(pred!(const, [], []), _)]),
	int cv_kinfu_VolumeParams_propMaxWeight_const(const cv::kinfu::VolumeParams* instance) {
			int ret = instance->maxWeight;
			return ret;
	}

	// cv::kinfu::VolumeParams::setMaxWeight(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:59
	// ("cv::kinfu::VolumeParams::setMaxWeight", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_kinfu_VolumeParams_propMaxWeight_const_int(cv::kinfu::VolumeParams* instance, const int val) {
			instance->maxWeight = val;
	}

	// cv::kinfu::VolumeParams::depthTruncThreshold() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:64
	// ("cv::kinfu::VolumeParams::depthTruncThreshold", vec![(pred!(const, [], []), _)]),
	float cv_kinfu_VolumeParams_propDepthTruncThreshold_const(const cv::kinfu::VolumeParams* instance) {
			float ret = instance->depthTruncThreshold;
			return ret;
	}

	// cv::kinfu::VolumeParams::setDepthTruncThreshold(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:64
	// ("cv::kinfu::VolumeParams::setDepthTruncThreshold", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_kinfu_VolumeParams_propDepthTruncThreshold_const_float(cv::kinfu::VolumeParams* instance, const float val) {
			instance->depthTruncThreshold = val;
	}

	// cv::kinfu::VolumeParams::raycastStepFactor() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:69
	// ("cv::kinfu::VolumeParams::raycastStepFactor", vec![(pred!(const, [], []), _)]),
	float cv_kinfu_VolumeParams_propRaycastStepFactor_const(const cv::kinfu::VolumeParams* instance) {
			float ret = instance->raycastStepFactor;
			return ret;
	}

	// cv::kinfu::VolumeParams::setRaycastStepFactor(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/kinfu.hpp:69
	// ("cv::kinfu::VolumeParams::setRaycastStepFactor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_kinfu_VolumeParams_propRaycastStepFactor_const_float(cv::kinfu::VolumeParams* instance, const float val) {
			instance->raycastStepFactor = val;
	}

	// cv::kinfu::VolumeParams::delete() generated
	// ("cv::kinfu::VolumeParams::delete", vec![(pred!(mut, [], []), _)]),
	void cv_kinfu_VolumeParams_delete(cv::kinfu::VolumeParams* instance) {
			delete instance;
	}

	// create(const Ptr<Params> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:195
	// ("cv::large_kinfu::LargeKinfu::create", vec![(pred!(mut, ["_params"], ["const cv::Ptr<cv::large_kinfu::Params>*"]), _)]),
	void cv_large_kinfu_LargeKinfu_create_const_PtrLParamsGR(const cv::Ptr<cv::large_kinfu::Params>* _params, Result<cv::Ptr<cv::large_kinfu::LargeKinfu>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::large_kinfu::LargeKinfu> ret = cv::large_kinfu::LargeKinfu::create(*_params);
			Ok(new cv::Ptr<cv::large_kinfu::LargeKinfu>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getParams()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:198
	// ("cv::large_kinfu::LargeKinfu::getParams", vec![(pred!(const, [], []), _)]),
	void cv_large_kinfu_LargeKinfu_getParams_const(const cv::large_kinfu::LargeKinfu* instance, Result<cv::large_kinfu::Params*>* ocvrs_return) {
		try {
			const cv::large_kinfu::Params ret = instance->getParams();
			Ok(new const cv::large_kinfu::Params(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// render(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:200
	// ("cv::large_kinfu::LargeKinfu::render", vec![(pred!(const, ["image"], ["const cv::_OutputArray*"]), _)]),
	void cv_large_kinfu_LargeKinfu_render_const_const__OutputArrayR(const cv::large_kinfu::LargeKinfu* instance, const cv::_OutputArray* image, ResultVoid* ocvrs_return) {
		try {
			instance->render(*image);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// render(OutputArray, const Matx44f &)(OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:201
	// ("cv::large_kinfu::LargeKinfu::render", vec![(pred!(const, ["image", "cameraPose"], ["const cv::_OutputArray*", "const cv::Matx44f*"]), _)]),
	void cv_large_kinfu_LargeKinfu_render_const_const__OutputArrayR_const_Matx44fR(const cv::large_kinfu::LargeKinfu* instance, const cv::_OutputArray* image, const cv::Matx44f* cameraPose, ResultVoid* ocvrs_return) {
		try {
			instance->render(*image, *cameraPose);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getCloud(OutputArray, OutputArray)(OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:203
	// ("cv::large_kinfu::LargeKinfu::getCloud", vec![(pred!(const, ["points", "normals"], ["const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_large_kinfu_LargeKinfu_getCloud_const_const__OutputArrayR_const__OutputArrayR(const cv::large_kinfu::LargeKinfu* instance, const cv::_OutputArray* points, const cv::_OutputArray* normals, ResultVoid* ocvrs_return) {
		try {
			instance->getCloud(*points, *normals);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPoints(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:205
	// ("cv::large_kinfu::LargeKinfu::getPoints", vec![(pred!(const, ["points"], ["const cv::_OutputArray*"]), _)]),
	void cv_large_kinfu_LargeKinfu_getPoints_const_const__OutputArrayR(const cv::large_kinfu::LargeKinfu* instance, const cv::_OutputArray* points, ResultVoid* ocvrs_return) {
		try {
			instance->getPoints(*points);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNormals(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:207
	// ("cv::large_kinfu::LargeKinfu::getNormals", vec![(pred!(const, ["points", "normals"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_large_kinfu_LargeKinfu_getNormals_const_const__InputArrayR_const__OutputArrayR(const cv::large_kinfu::LargeKinfu* instance, const cv::_InputArray* points, const cv::_OutputArray* normals, ResultVoid* ocvrs_return) {
		try {
			instance->getNormals(*points, *normals);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// reset()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:209
	// ("cv::large_kinfu::LargeKinfu::reset", vec![(pred!(mut, [], []), _)]),
	void cv_large_kinfu_LargeKinfu_reset(cv::large_kinfu::LargeKinfu* instance, ResultVoid* ocvrs_return) {
		try {
			instance->reset();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPose()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:211
	// ("cv::large_kinfu::LargeKinfu::getPose", vec![(pred!(const, [], []), _)]),
	void cv_large_kinfu_LargeKinfu_getPose_const(const cv::large_kinfu::LargeKinfu* instance, Result<cv::Affine3f>* ocvrs_return) {
		try {
			cv::Affine3f ret = instance->getPose();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// update(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:213
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

	// defaultParams()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:92
	// ("cv::large_kinfu::Params::defaultParams", vec![(pred!(mut, [], []), _)]),
	void cv_large_kinfu_Params_defaultParams(Result<cv::Ptr<cv::large_kinfu::Params>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::large_kinfu::Params> ret = cv::large_kinfu::Params::defaultParams();
			Ok(new cv::Ptr<cv::large_kinfu::Params>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// coarseParams()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:99
	// ("cv::large_kinfu::Params::coarseParams", vec![(pred!(mut, [], []), _)]),
	void cv_large_kinfu_Params_coarseParams(Result<cv::Ptr<cv::large_kinfu::Params>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::large_kinfu::Params> ret = cv::large_kinfu::Params::coarseParams();
			Ok(new cv::Ptr<cv::large_kinfu::Params>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// hashTSDFParams(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:105
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

	// cv::large_kinfu::Params::frameSize() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:108
	// ("cv::large_kinfu::Params::frameSize", vec![(pred!(const, [], []), _)]),
	void cv_large_kinfu_Params_propFrameSize_const(const cv::large_kinfu::Params* instance, cv::Size* ocvrs_return) {
			cv::Size ret = instance->frameSize;
			*ocvrs_return = ret;
	}

	// cv::large_kinfu::Params::setFrameSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:108
	// ("cv::large_kinfu::Params::setFrameSize", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	void cv_large_kinfu_Params_propFrameSize_const_Size(cv::large_kinfu::Params* instance, const cv::Size* val) {
			instance->frameSize = *val;
	}

	// cv::large_kinfu::Params::intr() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:111
	// ("cv::large_kinfu::Params::intr", vec![(pred!(const, [], []), _)]),
	void cv_large_kinfu_Params_propIntr_const(const cv::large_kinfu::Params* instance, cv::Matx33f* ocvrs_return) {
			cv::Matx33f ret = instance->intr;
			*ocvrs_return = ret;
	}

	// cv::large_kinfu::Params::setIntr(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:111
	// ("cv::large_kinfu::Params::setIntr", vec![(pred!(mut, ["val"], ["const cv::Matx33f"]), _)]),
	void cv_large_kinfu_Params_propIntr_const_Matx33f(cv::large_kinfu::Params* instance, const cv::Matx33f* val) {
			instance->intr = *val;
	}

	// cv::large_kinfu::Params::rgb_intr() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:114
	// ("cv::large_kinfu::Params::rgb_intr", vec![(pred!(const, [], []), _)]),
	void cv_large_kinfu_Params_propRgb_intr_const(const cv::large_kinfu::Params* instance, cv::Matx33f* ocvrs_return) {
			cv::Matx33f ret = instance->rgb_intr;
			*ocvrs_return = ret;
	}

	// cv::large_kinfu::Params::setRgb_intr(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:114
	// ("cv::large_kinfu::Params::setRgb_intr", vec![(pred!(mut, ["val"], ["const cv::Matx33f"]), _)]),
	void cv_large_kinfu_Params_propRgb_intr_const_Matx33f(cv::large_kinfu::Params* instance, const cv::Matx33f* val) {
			instance->rgb_intr = *val;
	}

	// cv::large_kinfu::Params::depthFactor() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:123
	// ("cv::large_kinfu::Params::depthFactor", vec![(pred!(const, [], []), _)]),
	float cv_large_kinfu_Params_propDepthFactor_const(const cv::large_kinfu::Params* instance) {
			float ret = instance->depthFactor;
			return ret;
	}

	// cv::large_kinfu::Params::setDepthFactor(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:123
	// ("cv::large_kinfu::Params::setDepthFactor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_large_kinfu_Params_propDepthFactor_const_float(cv::large_kinfu::Params* instance, const float val) {
			instance->depthFactor = val;
	}

	// cv::large_kinfu::Params::bilateral_sigma_depth() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:126
	// ("cv::large_kinfu::Params::bilateral_sigma_depth", vec![(pred!(const, [], []), _)]),
	float cv_large_kinfu_Params_propBilateral_sigma_depth_const(const cv::large_kinfu::Params* instance) {
			float ret = instance->bilateral_sigma_depth;
			return ret;
	}

	// cv::large_kinfu::Params::setBilateral_sigma_depth(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:126
	// ("cv::large_kinfu::Params::setBilateral_sigma_depth", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_large_kinfu_Params_propBilateral_sigma_depth_const_float(cv::large_kinfu::Params* instance, const float val) {
			instance->bilateral_sigma_depth = val;
	}

	// cv::large_kinfu::Params::bilateral_sigma_spatial() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:128
	// ("cv::large_kinfu::Params::bilateral_sigma_spatial", vec![(pred!(const, [], []), _)]),
	float cv_large_kinfu_Params_propBilateral_sigma_spatial_const(const cv::large_kinfu::Params* instance) {
			float ret = instance->bilateral_sigma_spatial;
			return ret;
	}

	// cv::large_kinfu::Params::setBilateral_sigma_spatial(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:128
	// ("cv::large_kinfu::Params::setBilateral_sigma_spatial", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_large_kinfu_Params_propBilateral_sigma_spatial_const_float(cv::large_kinfu::Params* instance, const float val) {
			instance->bilateral_sigma_spatial = val;
	}

	// cv::large_kinfu::Params::bilateral_kernel_size() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:130
	// ("cv::large_kinfu::Params::bilateral_kernel_size", vec![(pred!(const, [], []), _)]),
	int cv_large_kinfu_Params_propBilateral_kernel_size_const(const cv::large_kinfu::Params* instance) {
			int ret = instance->bilateral_kernel_size;
			return ret;
	}

	// cv::large_kinfu::Params::setBilateral_kernel_size(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:130
	// ("cv::large_kinfu::Params::setBilateral_kernel_size", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_large_kinfu_Params_propBilateral_kernel_size_const_int(cv::large_kinfu::Params* instance, const int val) {
			instance->bilateral_kernel_size = val;
	}

	// cv::large_kinfu::Params::pyramidLevels() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:133
	// ("cv::large_kinfu::Params::pyramidLevels", vec![(pred!(const, [], []), _)]),
	int cv_large_kinfu_Params_propPyramidLevels_const(const cv::large_kinfu::Params* instance) {
			int ret = instance->pyramidLevels;
			return ret;
	}

	// cv::large_kinfu::Params::setPyramidLevels(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:133
	// ("cv::large_kinfu::Params::setPyramidLevels", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_large_kinfu_Params_propPyramidLevels_const_int(cv::large_kinfu::Params* instance, const int val) {
			instance->pyramidLevels = val;
	}

	// cv::large_kinfu::Params::tsdf_min_camera_movement() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:139
	// ("cv::large_kinfu::Params::tsdf_min_camera_movement", vec![(pred!(const, [], []), _)]),
	float cv_large_kinfu_Params_propTsdf_min_camera_movement_const(const cv::large_kinfu::Params* instance) {
			float ret = instance->tsdf_min_camera_movement;
			return ret;
	}

	// cv::large_kinfu::Params::setTsdf_min_camera_movement(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:139
	// ("cv::large_kinfu::Params::setTsdf_min_camera_movement", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_large_kinfu_Params_propTsdf_min_camera_movement_const_float(cv::large_kinfu::Params* instance, const float val) {
			instance->tsdf_min_camera_movement = val;
	}

	// cv::large_kinfu::Params::lightPose() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:142
	// ("cv::large_kinfu::Params::lightPose", vec![(pred!(const, [], []), _)]),
	void cv_large_kinfu_Params_propLightPose_const(const cv::large_kinfu::Params* instance, cv::Vec3f* ocvrs_return) {
			cv::Vec3f ret = instance->lightPose;
			*ocvrs_return = ret;
	}

	// cv::large_kinfu::Params::setLightPose(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:142
	// ("cv::large_kinfu::Params::setLightPose", vec![(pred!(mut, ["val"], ["const cv::Vec3f"]), _)]),
	void cv_large_kinfu_Params_propLightPose_const_Vec3f(cv::large_kinfu::Params* instance, const cv::Vec3f* val) {
			instance->lightPose = *val;
	}

	// cv::large_kinfu::Params::icpDistThresh() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:145
	// ("cv::large_kinfu::Params::icpDistThresh", vec![(pred!(const, [], []), _)]),
	float cv_large_kinfu_Params_propIcpDistThresh_const(const cv::large_kinfu::Params* instance) {
			float ret = instance->icpDistThresh;
			return ret;
	}

	// cv::large_kinfu::Params::setIcpDistThresh(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:145
	// ("cv::large_kinfu::Params::setIcpDistThresh", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_large_kinfu_Params_propIcpDistThresh_const_float(cv::large_kinfu::Params* instance, const float val) {
			instance->icpDistThresh = val;
	}

	// cv::large_kinfu::Params::icpAngleThresh() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:147
	// ("cv::large_kinfu::Params::icpAngleThresh", vec![(pred!(const, [], []), _)]),
	float cv_large_kinfu_Params_propIcpAngleThresh_const(const cv::large_kinfu::Params* instance) {
			float ret = instance->icpAngleThresh;
			return ret;
	}

	// cv::large_kinfu::Params::setIcpAngleThresh(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:147
	// ("cv::large_kinfu::Params::setIcpAngleThresh", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_large_kinfu_Params_propIcpAngleThresh_const_float(cv::large_kinfu::Params* instance, const float val) {
			instance->icpAngleThresh = val;
	}

	// cv::large_kinfu::Params::icpIterations() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:149
	// ("cv::large_kinfu::Params::icpIterations", vec![(pred!(const, [], []), _)]),
	std::vector<int>* cv_large_kinfu_Params_propIcpIterations_const(const cv::large_kinfu::Params* instance) {
			std::vector<int> ret = instance->icpIterations;
			return new std::vector<int>(ret);
	}

	// cv::large_kinfu::Params::setIcpIterations(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:149
	// ("cv::large_kinfu::Params::setIcpIterations", vec![(pred!(mut, ["val"], ["const std::vector<int>"]), _)]),
	void cv_large_kinfu_Params_propIcpIterations_const_vectorLintG(cv::large_kinfu::Params* instance, const std::vector<int>* val) {
			instance->icpIterations = *val;
	}

	// cv::large_kinfu::Params::truncateThreshold() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:155
	// ("cv::large_kinfu::Params::truncateThreshold", vec![(pred!(const, [], []), _)]),
	float cv_large_kinfu_Params_propTruncateThreshold_const(const cv::large_kinfu::Params* instance) {
			float ret = instance->truncateThreshold;
			return ret;
	}

	// cv::large_kinfu::Params::setTruncateThreshold(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:155
	// ("cv::large_kinfu::Params::setTruncateThreshold", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_large_kinfu_Params_propTruncateThreshold_const_float(cv::large_kinfu::Params* instance, const float val) {
			instance->truncateThreshold = val;
	}

	// cv::large_kinfu::Params::volumeParams() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:159
	// ("cv::large_kinfu::Params::volumeParams", vec![(pred!(const, [], []), _)]),
	cv::large_kinfu::VolumeParams* cv_large_kinfu_Params_propVolumeParams_const(const cv::large_kinfu::Params* instance) {
			cv::large_kinfu::VolumeParams ret = instance->volumeParams;
			return new cv::large_kinfu::VolumeParams(ret);
	}

	// cv::large_kinfu::Params::setVolumeParams(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:159
	// ("cv::large_kinfu::Params::setVolumeParams", vec![(pred!(mut, ["val"], ["const cv::large_kinfu::VolumeParams"]), _)]),
	void cv_large_kinfu_Params_propVolumeParams_const_VolumeParams(cv::large_kinfu::Params* instance, const cv::large_kinfu::VolumeParams* val) {
			instance->volumeParams = *val;
	}

	// cv::large_kinfu::Params::delete() generated
	// ("cv::large_kinfu::Params::delete", vec![(pred!(mut, [], []), _)]),
	void cv_large_kinfu_Params_delete(cv::large_kinfu::Params* instance) {
			delete instance;
	}

	// defaultParams(VolumeType)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:80
	// ("cv::large_kinfu::VolumeParams::defaultParams", vec![(pred!(mut, ["volumeType"], ["cv::VolumeType"]), _)]),
	void cv_large_kinfu_VolumeParams_defaultParams_VolumeType(cv::VolumeType volumeType, Result<cv::Ptr<cv::large_kinfu::VolumeParams>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::large_kinfu::VolumeParams> ret = cv::large_kinfu::VolumeParams::defaultParams(volumeType);
			Ok(new cv::Ptr<cv::large_kinfu::VolumeParams>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// coarseParams(VolumeType)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:83
	// ("cv::large_kinfu::VolumeParams::coarseParams", vec![(pred!(mut, ["volumeType"], ["cv::VolumeType"]), _)]),
	void cv_large_kinfu_VolumeParams_coarseParams_VolumeType(cv::VolumeType volumeType, Result<cv::Ptr<cv::large_kinfu::VolumeParams>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::large_kinfu::VolumeParams> ret = cv::large_kinfu::VolumeParams::coarseParams(volumeType);
			Ok(new cv::Ptr<cv::large_kinfu::VolumeParams>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::large_kinfu::VolumeParams::defaultNew() generated
	// ("cv::large_kinfu::VolumeParams::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::large_kinfu::VolumeParams* cv_large_kinfu_VolumeParams_defaultNew_const() {
			cv::large_kinfu::VolumeParams* ret = new cv::large_kinfu::VolumeParams();
			return ret;
	}

	// cv::large_kinfu::VolumeParams::kind() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:26
	// ("cv::large_kinfu::VolumeParams::kind", vec![(pred!(const, [], []), _)]),
	void cv_large_kinfu_VolumeParams_propKind_const(const cv::large_kinfu::VolumeParams* instance, cv::VolumeType* ocvrs_return) {
			cv::VolumeType ret = instance->kind;
			*ocvrs_return = ret;
	}

	// cv::large_kinfu::VolumeParams::setKind(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:26
	// ("cv::large_kinfu::VolumeParams::setKind", vec![(pred!(mut, ["val"], ["const cv::VolumeType"]), _)]),
	void cv_large_kinfu_VolumeParams_propKind_const_VolumeType(cv::large_kinfu::VolumeParams* instance, const cv::VolumeType val) {
			instance->kind = val;
	}

	// cv::large_kinfu::VolumeParams::resolutionX() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:34
	// ("cv::large_kinfu::VolumeParams::resolutionX", vec![(pred!(const, [], []), _)]),
	int cv_large_kinfu_VolumeParams_propResolutionX_const(const cv::large_kinfu::VolumeParams* instance) {
			int ret = instance->resolutionX;
			return ret;
	}

	// cv::large_kinfu::VolumeParams::setResolutionX(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:34
	// ("cv::large_kinfu::VolumeParams::setResolutionX", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_large_kinfu_VolumeParams_propResolutionX_const_int(cv::large_kinfu::VolumeParams* instance, const int val) {
			instance->resolutionX = val;
	}

	// cv::large_kinfu::VolumeParams::resolutionY() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:35
	// ("cv::large_kinfu::VolumeParams::resolutionY", vec![(pred!(const, [], []), _)]),
	int cv_large_kinfu_VolumeParams_propResolutionY_const(const cv::large_kinfu::VolumeParams* instance) {
			int ret = instance->resolutionY;
			return ret;
	}

	// cv::large_kinfu::VolumeParams::setResolutionY(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:35
	// ("cv::large_kinfu::VolumeParams::setResolutionY", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_large_kinfu_VolumeParams_propResolutionY_const_int(cv::large_kinfu::VolumeParams* instance, const int val) {
			instance->resolutionY = val;
	}

	// cv::large_kinfu::VolumeParams::resolutionZ() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:36
	// ("cv::large_kinfu::VolumeParams::resolutionZ", vec![(pred!(const, [], []), _)]),
	int cv_large_kinfu_VolumeParams_propResolutionZ_const(const cv::large_kinfu::VolumeParams* instance) {
			int ret = instance->resolutionZ;
			return ret;
	}

	// cv::large_kinfu::VolumeParams::setResolutionZ(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:36
	// ("cv::large_kinfu::VolumeParams::setResolutionZ", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_large_kinfu_VolumeParams_propResolutionZ_const_int(cv::large_kinfu::VolumeParams* instance, const int val) {
			instance->resolutionZ = val;
	}

	// cv::large_kinfu::VolumeParams::unitResolution() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:43
	// ("cv::large_kinfu::VolumeParams::unitResolution", vec![(pred!(const, [], []), _)]),
	int cv_large_kinfu_VolumeParams_propUnitResolution_const(const cv::large_kinfu::VolumeParams* instance) {
			int ret = instance->unitResolution;
			return ret;
	}

	// cv::large_kinfu::VolumeParams::setUnitResolution(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:43
	// ("cv::large_kinfu::VolumeParams::setUnitResolution", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_large_kinfu_VolumeParams_propUnitResolution_const_int(cv::large_kinfu::VolumeParams* instance, const int val) {
			instance->unitResolution = val;
	}

	// cv::large_kinfu::VolumeParams::volumSize() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:46
	// ("cv::large_kinfu::VolumeParams::volumSize", vec![(pred!(const, [], []), _)]),
	float cv_large_kinfu_VolumeParams_propVolumSize_const(const cv::large_kinfu::VolumeParams* instance) {
			float ret = instance->volumSize;
			return ret;
	}

	// cv::large_kinfu::VolumeParams::setVolumSize(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:46
	// ("cv::large_kinfu::VolumeParams::setVolumSize", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_large_kinfu_VolumeParams_propVolumSize_const_float(cv::large_kinfu::VolumeParams* instance, const float val) {
			instance->volumSize = val;
	}

	// cv::large_kinfu::VolumeParams::pose() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:49
	// ("cv::large_kinfu::VolumeParams::pose", vec![(pred!(const, [], []), _)]),
	void cv_large_kinfu_VolumeParams_propPose_const(const cv::large_kinfu::VolumeParams* instance, cv::Matx44f* ocvrs_return) {
			cv::Matx44f ret = instance->pose;
			*ocvrs_return = ret;
	}

	// cv::large_kinfu::VolumeParams::setPose(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:49
	// ("cv::large_kinfu::VolumeParams::setPose", vec![(pred!(mut, ["val"], ["const cv::Matx44f"]), _)]),
	void cv_large_kinfu_VolumeParams_propPose_const_Matx44f(cv::large_kinfu::VolumeParams* instance, const cv::Matx44f* val) {
			instance->pose = *val;
	}

	// cv::large_kinfu::VolumeParams::voxelSize() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:52
	// ("cv::large_kinfu::VolumeParams::voxelSize", vec![(pred!(const, [], []), _)]),
	float cv_large_kinfu_VolumeParams_propVoxelSize_const(const cv::large_kinfu::VolumeParams* instance) {
			float ret = instance->voxelSize;
			return ret;
	}

	// cv::large_kinfu::VolumeParams::setVoxelSize(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:52
	// ("cv::large_kinfu::VolumeParams::setVoxelSize", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_large_kinfu_VolumeParams_propVoxelSize_const_float(cv::large_kinfu::VolumeParams* instance, const float val) {
			instance->voxelSize = val;
	}

	// cv::large_kinfu::VolumeParams::tsdfTruncDist() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:58
	// ("cv::large_kinfu::VolumeParams::tsdfTruncDist", vec![(pred!(const, [], []), _)]),
	float cv_large_kinfu_VolumeParams_propTsdfTruncDist_const(const cv::large_kinfu::VolumeParams* instance) {
			float ret = instance->tsdfTruncDist;
			return ret;
	}

	// cv::large_kinfu::VolumeParams::setTsdfTruncDist(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:58
	// ("cv::large_kinfu::VolumeParams::setTsdfTruncDist", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_large_kinfu_VolumeParams_propTsdfTruncDist_const_float(cv::large_kinfu::VolumeParams* instance, const float val) {
			instance->tsdfTruncDist = val;
	}

	// cv::large_kinfu::VolumeParams::maxWeight() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:65
	// ("cv::large_kinfu::VolumeParams::maxWeight", vec![(pred!(const, [], []), _)]),
	int cv_large_kinfu_VolumeParams_propMaxWeight_const(const cv::large_kinfu::VolumeParams* instance) {
			int ret = instance->maxWeight;
			return ret;
	}

	// cv::large_kinfu::VolumeParams::setMaxWeight(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:65
	// ("cv::large_kinfu::VolumeParams::setMaxWeight", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_large_kinfu_VolumeParams_propMaxWeight_const_int(cv::large_kinfu::VolumeParams* instance, const int val) {
			instance->maxWeight = val;
	}

	// cv::large_kinfu::VolumeParams::depthTruncThreshold() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:71
	// ("cv::large_kinfu::VolumeParams::depthTruncThreshold", vec![(pred!(const, [], []), _)]),
	float cv_large_kinfu_VolumeParams_propDepthTruncThreshold_const(const cv::large_kinfu::VolumeParams* instance) {
			float ret = instance->depthTruncThreshold;
			return ret;
	}

	// cv::large_kinfu::VolumeParams::setDepthTruncThreshold(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:71
	// ("cv::large_kinfu::VolumeParams::setDepthTruncThreshold", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_large_kinfu_VolumeParams_propDepthTruncThreshold_const_float(cv::large_kinfu::VolumeParams* instance, const float val) {
			instance->depthTruncThreshold = val;
	}

	// cv::large_kinfu::VolumeParams::raycastStepFactor() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:77
	// ("cv::large_kinfu::VolumeParams::raycastStepFactor", vec![(pred!(const, [], []), _)]),
	float cv_large_kinfu_VolumeParams_propRaycastStepFactor_const(const cv::large_kinfu::VolumeParams* instance) {
			float ret = instance->raycastStepFactor;
			return ret;
	}

	// cv::large_kinfu::VolumeParams::setRaycastStepFactor(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/large_kinfu.hpp:77
	// ("cv::large_kinfu::VolumeParams::setRaycastStepFactor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_large_kinfu_VolumeParams_propRaycastStepFactor_const_float(cv::large_kinfu::VolumeParams* instance, const float val) {
			instance->raycastStepFactor = val;
	}

	// cv::large_kinfu::VolumeParams::delete() generated
	// ("cv::large_kinfu::VolumeParams::delete", vec![(pred!(mut, [], []), _)]),
	void cv_large_kinfu_VolumeParams_delete(cv::large_kinfu::VolumeParams* instance) {
			delete instance;
	}

	// ColorGradient()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:172
	// ("cv::linemod::ColorGradient::ColorGradient", vec![(pred!(mut, [], []), _)]),
	void cv_linemod_ColorGradient_ColorGradient(Result<cv::linemod::ColorGradient*>* ocvrs_return) {
		try {
			cv::linemod::ColorGradient* ret = new cv::linemod::ColorGradient();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ColorGradient(float, size_t, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:182
	// ("cv::linemod::ColorGradient::ColorGradient", vec![(pred!(mut, ["weak_threshold", "num_features", "strong_threshold"], ["float", "size_t", "float"]), _)]),
	void cv_linemod_ColorGradient_ColorGradient_float_size_t_float(float weak_threshold, size_t num_features, float strong_threshold, Result<cv::linemod::ColorGradient*>* ocvrs_return) {
		try {
			cv::linemod::ColorGradient* ret = new cv::linemod::ColorGradient(weak_threshold, num_features, strong_threshold);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(float, size_t, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:184
	// ("cv::linemod::ColorGradient::create", vec![(pred!(mut, ["weak_threshold", "num_features", "strong_threshold"], ["float", "size_t", "float"]), _)]),
	void cv_linemod_ColorGradient_create_float_size_t_float(float weak_threshold, size_t num_features, float strong_threshold, Result<cv::Ptr<cv::linemod::ColorGradient>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::linemod::ColorGradient> ret = cv::linemod::ColorGradient::create(weak_threshold, num_features, strong_threshold);
			Ok(new cv::Ptr<cv::linemod::ColorGradient>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// name()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:186
	// ("cv::linemod::ColorGradient::name", vec![(pred!(const, [], []), _)]),
	void cv_linemod_ColorGradient_name_const(const cv::linemod::ColorGradient* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->name();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:188
	// ("cv::linemod::ColorGradient::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	void cv_linemod_ColorGradient_read_const_FileNodeR(cv::linemod::ColorGradient* instance, const cv::FileNode* fn, ResultVoid* ocvrs_return) {
		try {
			instance->read(*fn);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:189
	// ("cv::linemod::ColorGradient::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	void cv_linemod_ColorGradient_write_const_FileStorageR(const cv::linemod::ColorGradient* instance, cv::FileStorage* fs, ResultVoid* ocvrs_return) {
		try {
			instance->write(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::linemod::ColorGradient::weak_threshold() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:191
	// ("cv::linemod::ColorGradient::weak_threshold", vec![(pred!(const, [], []), _)]),
	float cv_linemod_ColorGradient_propWeak_threshold_const(const cv::linemod::ColorGradient* instance) {
			float ret = instance->weak_threshold;
			return ret;
	}

	// cv::linemod::ColorGradient::setWeak_threshold(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:191
	// ("cv::linemod::ColorGradient::setWeak_threshold", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_linemod_ColorGradient_propWeak_threshold_const_float(cv::linemod::ColorGradient* instance, const float val) {
			instance->weak_threshold = val;
	}

	// cv::linemod::ColorGradient::num_features() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:192
	// ("cv::linemod::ColorGradient::num_features", vec![(pred!(const, [], []), _)]),
	size_t cv_linemod_ColorGradient_propNum_features_const(const cv::linemod::ColorGradient* instance) {
			size_t ret = instance->num_features;
			return ret;
	}

	// cv::linemod::ColorGradient::setNum_features(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:192
	// ("cv::linemod::ColorGradient::setNum_features", vec![(pred!(mut, ["val"], ["const size_t"]), _)]),
	void cv_linemod_ColorGradient_propNum_features_const_size_t(cv::linemod::ColorGradient* instance, const size_t val) {
			instance->num_features = val;
	}

	// cv::linemod::ColorGradient::strong_threshold() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:193
	// ("cv::linemod::ColorGradient::strong_threshold", vec![(pred!(const, [], []), _)]),
	float cv_linemod_ColorGradient_propStrong_threshold_const(const cv::linemod::ColorGradient* instance) {
			float ret = instance->strong_threshold;
			return ret;
	}

	// cv::linemod::ColorGradient::setStrong_threshold(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:193
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

	// DepthNormal()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:209
	// ("cv::linemod::DepthNormal::DepthNormal", vec![(pred!(mut, [], []), _)]),
	void cv_linemod_DepthNormal_DepthNormal(Result<cv::linemod::DepthNormal*>* ocvrs_return) {
		try {
			cv::linemod::DepthNormal* ret = new cv::linemod::DepthNormal();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// DepthNormal(int, int, size_t, int)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:221
	// ("cv::linemod::DepthNormal::DepthNormal", vec![(pred!(mut, ["distance_threshold", "difference_threshold", "num_features", "extract_threshold"], ["int", "int", "size_t", "int"]), _)]),
	void cv_linemod_DepthNormal_DepthNormal_int_int_size_t_int(int distance_threshold, int difference_threshold, size_t num_features, int extract_threshold, Result<cv::linemod::DepthNormal*>* ocvrs_return) {
		try {
			cv::linemod::DepthNormal* ret = new cv::linemod::DepthNormal(distance_threshold, difference_threshold, num_features, extract_threshold);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(int, int, size_t, int)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:224
	// ("cv::linemod::DepthNormal::create", vec![(pred!(mut, ["distance_threshold", "difference_threshold", "num_features", "extract_threshold"], ["int", "int", "size_t", "int"]), _)]),
	void cv_linemod_DepthNormal_create_int_int_size_t_int(int distance_threshold, int difference_threshold, size_t num_features, int extract_threshold, Result<cv::Ptr<cv::linemod::DepthNormal>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::linemod::DepthNormal> ret = cv::linemod::DepthNormal::create(distance_threshold, difference_threshold, num_features, extract_threshold);
			Ok(new cv::Ptr<cv::linemod::DepthNormal>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// name()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:227
	// ("cv::linemod::DepthNormal::name", vec![(pred!(const, [], []), _)]),
	void cv_linemod_DepthNormal_name_const(const cv::linemod::DepthNormal* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->name();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:229
	// ("cv::linemod::DepthNormal::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	void cv_linemod_DepthNormal_read_const_FileNodeR(cv::linemod::DepthNormal* instance, const cv::FileNode* fn, ResultVoid* ocvrs_return) {
		try {
			instance->read(*fn);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:230
	// ("cv::linemod::DepthNormal::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	void cv_linemod_DepthNormal_write_const_FileStorageR(const cv::linemod::DepthNormal* instance, cv::FileStorage* fs, ResultVoid* ocvrs_return) {
		try {
			instance->write(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::linemod::DepthNormal::distance_threshold() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:232
	// ("cv::linemod::DepthNormal::distance_threshold", vec![(pred!(const, [], []), _)]),
	int cv_linemod_DepthNormal_propDistance_threshold_const(const cv::linemod::DepthNormal* instance) {
			int ret = instance->distance_threshold;
			return ret;
	}

	// cv::linemod::DepthNormal::setDistance_threshold(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:232
	// ("cv::linemod::DepthNormal::setDistance_threshold", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_linemod_DepthNormal_propDistance_threshold_const_int(cv::linemod::DepthNormal* instance, const int val) {
			instance->distance_threshold = val;
	}

	// cv::linemod::DepthNormal::difference_threshold() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:233
	// ("cv::linemod::DepthNormal::difference_threshold", vec![(pred!(const, [], []), _)]),
	int cv_linemod_DepthNormal_propDifference_threshold_const(const cv::linemod::DepthNormal* instance) {
			int ret = instance->difference_threshold;
			return ret;
	}

	// cv::linemod::DepthNormal::setDifference_threshold(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:233
	// ("cv::linemod::DepthNormal::setDifference_threshold", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_linemod_DepthNormal_propDifference_threshold_const_int(cv::linemod::DepthNormal* instance, const int val) {
			instance->difference_threshold = val;
	}

	// cv::linemod::DepthNormal::num_features() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:234
	// ("cv::linemod::DepthNormal::num_features", vec![(pred!(const, [], []), _)]),
	size_t cv_linemod_DepthNormal_propNum_features_const(const cv::linemod::DepthNormal* instance) {
			size_t ret = instance->num_features;
			return ret;
	}

	// cv::linemod::DepthNormal::setNum_features(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:234
	// ("cv::linemod::DepthNormal::setNum_features", vec![(pred!(mut, ["val"], ["const size_t"]), _)]),
	void cv_linemod_DepthNormal_propNum_features_const_size_t(cv::linemod::DepthNormal* instance, const size_t val) {
			instance->num_features = val;
	}

	// cv::linemod::DepthNormal::extract_threshold() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:235
	// ("cv::linemod::DepthNormal::extract_threshold", vec![(pred!(const, [], []), _)]),
	int cv_linemod_DepthNormal_propExtract_threshold_const(const cv::linemod::DepthNormal* instance) {
			int ret = instance->extract_threshold;
			return ret;
	}

	// cv::linemod::DepthNormal::setExtract_threshold(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:235
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

	// Detector()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:304
	// ("cv::linemod::Detector::Detector", vec![(pred!(mut, [], []), _)]),
	void cv_linemod_Detector_Detector(Result<cv::linemod::Detector*>* ocvrs_return) {
		try {
			cv::linemod::Detector* ret = new cv::linemod::Detector();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Detector(const std::vector<Ptr<Modality>> &, const std::vector<int> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:313
	// ("cv::linemod::Detector::Detector", vec![(pred!(mut, ["modalities", "T_pyramid"], ["const std::vector<cv::Ptr<cv::linemod::Modality>>*", "const std::vector<int>*"]), _)]),
	void cv_linemod_Detector_Detector_const_vectorLPtrLModalityGGR_const_vectorLintGR(const std::vector<cv::Ptr<cv::linemod::Modality>>* modalities, const std::vector<int>* T_pyramid, Result<cv::linemod::Detector*>* ocvrs_return) {
		try {
			cv::linemod::Detector* ret = new cv::linemod::Detector(*modalities, *T_pyramid);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// match(const std::vector<Mat> &, float, std::vector<Match> &, const std::vector<String> &, OutputArrayOfArrays, const std::vector<Mat> &)(CppPassByVoidPtr, Primitive, CppPassByVoidPtr, CppPassByVoidPtr, OutputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:330
	// ("cv::linemod::Detector::match", vec![(pred!(const, ["sources", "threshold", "matches", "class_ids", "quantized_images", "masks"], ["const std::vector<cv::Mat>*", "float", "std::vector<cv::linemod::Match>*", "const std::vector<cv::String>*", "const cv::_OutputArray*", "const std::vector<cv::Mat>*"]), _)]),
	void cv_linemod_Detector_match_const_const_vectorLMatGR_float_vectorLMatchGR_const_vectorLStringGR_const__OutputArrayR_const_vectorLMatGR(const cv::linemod::Detector* instance, const std::vector<cv::Mat>* sources, float threshold, std::vector<cv::linemod::Match>* matches, const std::vector<cv::String>* class_ids, const cv::_OutputArray* quantized_images, const std::vector<cv::Mat>* masks, ResultVoid* ocvrs_return) {
		try {
			instance->match(*sources, threshold, *matches, *class_ids, *quantized_images, *masks);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::linemod::Detector::match(CppPassByVoidPtr, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:330
	// ("cv::linemod::Detector::match", vec![(pred!(const, ["sources", "threshold", "matches"], ["const std::vector<cv::Mat>*", "float", "std::vector<cv::linemod::Match>*"]), _)]),
	void cv_linemod_Detector_match_const_const_vectorLMatGR_float_vectorLMatchGR(const cv::linemod::Detector* instance, const std::vector<cv::Mat>* sources, float threshold, std::vector<cv::linemod::Match>* matches, ResultVoid* ocvrs_return) {
		try {
			instance->match(*sources, threshold, *matches);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// addTemplate(const std::vector<Mat> &, const String &, const Mat &, Rect *)(CppPassByVoidPtr, InString, TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:345
	// ("cv::linemod::Detector::addTemplate", vec![(pred!(mut, ["sources", "class_id", "object_mask", "bounding_box"], ["const std::vector<cv::Mat>*", "const cv::String*", "const cv::Mat*", "cv::Rect*"]), _)]),
	void cv_linemod_Detector_addTemplate_const_vectorLMatGR_const_StringR_const_MatR_RectX(cv::linemod::Detector* instance, const std::vector<cv::Mat>* sources, const char* class_id, const cv::Mat* object_mask, cv::Rect* bounding_box, Result<int>* ocvrs_return) {
		try {
			int ret = instance->addTemplate(*sources, std::string(class_id), *object_mask, bounding_box);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::linemod::Detector::addTemplate(CppPassByVoidPtr, InString, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:345
	// ("cv::linemod::Detector::addTemplate", vec![(pred!(mut, ["sources", "class_id", "object_mask"], ["const std::vector<cv::Mat>*", "const cv::String*", "const cv::Mat*"]), _)]),
	void cv_linemod_Detector_addTemplate_const_vectorLMatGR_const_StringR_const_MatR(cv::linemod::Detector* instance, const std::vector<cv::Mat>* sources, const char* class_id, const cv::Mat* object_mask, Result<int>* ocvrs_return) {
		try {
			int ret = instance->addTemplate(*sources, std::string(class_id), *object_mask);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// addSyntheticTemplate(const std::vector<Template> &, const String &)(CppPassByVoidPtr, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:351
	// ("cv::linemod::Detector::addSyntheticTemplate", vec![(pred!(mut, ["templates", "class_id"], ["const std::vector<cv::linemod::Template>*", "const cv::String*"]), _)]),
	void cv_linemod_Detector_addSyntheticTemplate_const_vectorLTemplateGR_const_StringR(cv::linemod::Detector* instance, const std::vector<cv::linemod::Template>* templates, const char* class_id, Result<int>* ocvrs_return) {
		try {
			int ret = instance->addSyntheticTemplate(*templates, std::string(class_id));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getModalities()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:359
	// ("cv::linemod::Detector::getModalities", vec![(pred!(const, [], []), _)]),
	void cv_linemod_Detector_getModalities_const(const cv::linemod::Detector* instance, Result<std::vector<cv::Ptr<cv::linemod::Modality>>*>* ocvrs_return) {
		try {
			const std::vector<cv::Ptr<cv::linemod::Modality>> ret = instance->getModalities();
			Ok(new const std::vector<cv::Ptr<cv::linemod::Modality>>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getT(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:364
	// ("cv::linemod::Detector::getT", vec![(pred!(const, ["pyramid_level"], ["int"]), _)]),
	void cv_linemod_Detector_getT_const_int(const cv::linemod::Detector* instance, int pyramid_level, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getT(pyramid_level);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// pyramidLevels()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:369
	// ("cv::linemod::Detector::pyramidLevels", vec![(pred!(const, [], []), _)]),
	void cv_linemod_Detector_pyramidLevels_const(const cv::linemod::Detector* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->pyramidLevels();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTemplates(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:377
	// ("cv::linemod::Detector::getTemplates", vec![(pred!(const, ["class_id", "template_id"], ["const cv::String*", "int"]), _)]),
	void cv_linemod_Detector_getTemplates_const_const_StringR_int(const cv::linemod::Detector* instance, const char* class_id, int template_id, Result<std::vector<cv::linemod::Template>*>* ocvrs_return) {
		try {
			const std::vector<cv::linemod::Template> ret = instance->getTemplates(std::string(class_id), template_id);
			Ok(new const std::vector<cv::linemod::Template>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// numTemplates()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:379
	// ("cv::linemod::Detector::numTemplates", vec![(pred!(const, [], []), _)]),
	void cv_linemod_Detector_numTemplates_const(const cv::linemod::Detector* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->numTemplates();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// numTemplates(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:380
	// ("cv::linemod::Detector::numTemplates", vec![(pred!(const, ["class_id"], ["const cv::String*"]), _)]),
	void cv_linemod_Detector_numTemplates_const_const_StringR(const cv::linemod::Detector* instance, const char* class_id, Result<int>* ocvrs_return) {
		try {
			int ret = instance->numTemplates(std::string(class_id));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// numClasses()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:381
	// ("cv::linemod::Detector::numClasses", vec![(pred!(const, [], []), _)]),
	void cv_linemod_Detector_numClasses_const(const cv::linemod::Detector* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->numClasses();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// classIds()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:383
	// ("cv::linemod::Detector::classIds", vec![(pred!(const, [], []), _)]),
	void cv_linemod_Detector_classIds_const(const cv::linemod::Detector* instance, Result<std::vector<cv::String>*>* ocvrs_return) {
		try {
			std::vector<cv::String> ret = instance->classIds();
			Ok(new std::vector<cv::String>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:385
	// ("cv::linemod::Detector::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	void cv_linemod_Detector_read_const_FileNodeR(cv::linemod::Detector* instance, const cv::FileNode* fn, ResultVoid* ocvrs_return) {
		try {
			instance->read(*fn);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:386
	// ("cv::linemod::Detector::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	void cv_linemod_Detector_write_const_FileStorageR(const cv::linemod::Detector* instance, cv::FileStorage* fs, ResultVoid* ocvrs_return) {
		try {
			instance->write(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readClass(const FileNode &, const String &)(TraitClass, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:388
	// ("cv::linemod::Detector::readClass", vec![(pred!(mut, ["fn", "class_id_override"], ["const cv::FileNode*", "const cv::String*"]), _)]),
	void cv_linemod_Detector_readClass_const_FileNodeR_const_StringR(cv::linemod::Detector* instance, const cv::FileNode* fn, const char* class_id_override, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->readClass(*fn, std::string(class_id_override));
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::linemod::Detector::readClass(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:388
	// ("cv::linemod::Detector::readClass", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	void cv_linemod_Detector_readClass_const_FileNodeR(cv::linemod::Detector* instance, const cv::FileNode* fn, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->readClass(*fn);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// writeClass(const String &, FileStorage &)(InString, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:389
	// ("cv::linemod::Detector::writeClass", vec![(pred!(const, ["class_id", "fs"], ["const cv::String*", "cv::FileStorage*"]), _)]),
	void cv_linemod_Detector_writeClass_const_const_StringR_FileStorageR(const cv::linemod::Detector* instance, const char* class_id, cv::FileStorage* fs, ResultVoid* ocvrs_return) {
		try {
			instance->writeClass(std::string(class_id), *fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readClasses(const std::vector<String> &, const String &)(CppPassByVoidPtr, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:391
	// ("cv::linemod::Detector::readClasses", vec![(pred!(mut, ["class_ids", "format"], ["const std::vector<cv::String>*", "const cv::String*"]), _)]),
	void cv_linemod_Detector_readClasses_const_vectorLStringGR_const_StringR(cv::linemod::Detector* instance, const std::vector<cv::String>* class_ids, const char* format, ResultVoid* ocvrs_return) {
		try {
			instance->readClasses(*class_ids, std::string(format));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::linemod::Detector::readClasses(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:391
	// ("cv::linemod::Detector::readClasses", vec![(pred!(mut, ["class_ids"], ["const std::vector<cv::String>*"]), _)]),
	void cv_linemod_Detector_readClasses_const_vectorLStringGR(cv::linemod::Detector* instance, const std::vector<cv::String>* class_ids, ResultVoid* ocvrs_return) {
		try {
			instance->readClasses(*class_ids);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// writeClasses(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:393
	// ("cv::linemod::Detector::writeClasses", vec![(pred!(const, ["format"], ["const cv::String*"]), _)]),
	void cv_linemod_Detector_writeClasses_const_const_StringR(const cv::linemod::Detector* instance, const char* format, ResultVoid* ocvrs_return) {
		try {
			instance->writeClasses(std::string(format));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::linemod::Detector::writeClasses() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:393
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

	// Feature()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:32
	// ("cv::linemod::Feature::Feature", vec![(pred!(mut, [], []), _)]),
	void cv_linemod_Feature_Feature(Result<cv::linemod::Feature>* ocvrs_return) {
		try {
			cv::linemod::Feature ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Feature(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:33
	// ("cv::linemod::Feature::Feature", vec![(pred!(mut, ["x", "y", "label"], ["int", "int", "int"]), _)]),
	void cv_linemod_Feature_Feature_int_int_int(int x, int y, int label, Result<cv::linemod::Feature>* ocvrs_return) {
		try {
			cv::linemod::Feature ret(x, y, label);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:35
	// ("cv::linemod::Feature::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	void cv_linemod_Feature_read_const_FileNodeR(cv::linemod::Feature* instance, const cv::FileNode* fn, ResultVoid* ocvrs_return) {
		try {
			instance->read(*fn);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:36
	// ("cv::linemod::Feature::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	void cv_linemod_Feature_write_const_FileStorageR(const cv::linemod::Feature* instance, cv::FileStorage* fs, ResultVoid* ocvrs_return) {
		try {
			instance->write(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Match()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:261
	// ("cv::linemod::Match::Match", vec![(pred!(mut, [], []), _)]),
	void cv_linemod_Match_Match(Result<cv::linemod::Match*>* ocvrs_return) {
		try {
			cv::linemod::Match* ret = new cv::linemod::Match();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Match(int, int, float, const String &, int)(Primitive, Primitive, Primitive, InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:265
	// ("cv::linemod::Match::Match", vec![(pred!(mut, ["x", "y", "similarity", "class_id", "template_id"], ["int", "int", "float", "const cv::String*", "int"]), _)]),
	void cv_linemod_Match_Match_int_int_float_const_StringR_int(int x, int y, float similarity, const char* class_id, int template_id, Result<cv::linemod::Match*>* ocvrs_return) {
		try {
			cv::linemod::Match* ret = new cv::linemod::Match(x, y, similarity, std::string(class_id), template_id);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator<(const Match &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:268
	// ("cv::linemod::Match::operator<", vec![(pred!(const, ["rhs"], ["const cv::linemod::Match*"]), _)]),
	void cv_linemod_Match_operatorL_const_const_MatchR(const cv::linemod::Match* instance, const cv::linemod::Match* rhs, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->operator<(*rhs);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator==(const Match &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:277
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

	// cv::linemod::Match::x() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:282
	// ("cv::linemod::Match::x", vec![(pred!(const, [], []), _)]),
	int cv_linemod_Match_propX_const(const cv::linemod::Match* instance) {
			int ret = instance->x;
			return ret;
	}

	// cv::linemod::Match::setX(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:282
	// ("cv::linemod::Match::setX", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_linemod_Match_propX_const_int(cv::linemod::Match* instance, const int val) {
			instance->x = val;
	}

	// cv::linemod::Match::y() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:283
	// ("cv::linemod::Match::y", vec![(pred!(const, [], []), _)]),
	int cv_linemod_Match_propY_const(const cv::linemod::Match* instance) {
			int ret = instance->y;
			return ret;
	}

	// cv::linemod::Match::setY(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:283
	// ("cv::linemod::Match::setY", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_linemod_Match_propY_const_int(cv::linemod::Match* instance, const int val) {
			instance->y = val;
	}

	// cv::linemod::Match::similarity() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:284
	// ("cv::linemod::Match::similarity", vec![(pred!(const, [], []), _)]),
	float cv_linemod_Match_propSimilarity_const(const cv::linemod::Match* instance) {
			float ret = instance->similarity;
			return ret;
	}

	// cv::linemod::Match::setSimilarity(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:284
	// ("cv::linemod::Match::setSimilarity", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_linemod_Match_propSimilarity_const_float(cv::linemod::Match* instance, const float val) {
			instance->similarity = val;
	}

	// cv::linemod::Match::class_id() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:285
	// ("cv::linemod::Match::class_id", vec![(pred!(const, [], []), _)]),
	void* cv_linemod_Match_propClass_id_const(const cv::linemod::Match* instance) {
			cv::String ret = instance->class_id;
			return ocvrs_create_string(ret.c_str());
	}

	// cv::linemod::Match::setClass_id(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:285
	// ("cv::linemod::Match::setClass_id", vec![(pred!(mut, ["val"], ["const cv::String"]), _)]),
	void cv_linemod_Match_propClass_id_const_String(cv::linemod::Match* instance, const char* val) {
			instance->class_id = std::string(val);
	}

	// cv::linemod::Match::template_id() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:286
	// ("cv::linemod::Match::template_id", vec![(pred!(const, [], []), _)]),
	int cv_linemod_Match_propTemplate_id_const(const cv::linemod::Match* instance) {
			int ret = instance->template_id;
			return ret;
	}

	// cv::linemod::Match::setTemplate_id(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:286
	// ("cv::linemod::Match::setTemplate_id", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_linemod_Match_propTemplate_id_const_int(cv::linemod::Match* instance, const int val) {
			instance->template_id = val;
	}

	// cv::linemod::Match::delete() generated
	// ("cv::linemod::Match::delete", vec![(pred!(mut, [], []), _)]),
	void cv_linemod_Match_delete(cv::linemod::Match* instance) {
			delete instance;
	}

	// process(const Mat &, const Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:132
	// ("cv::linemod::Modality::process", vec![(pred!(const, ["src", "mask"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
	void cv_linemod_Modality_process_const_const_MatR_const_MatR(const cv::linemod::Modality* instance, const cv::Mat* src, const cv::Mat* mask, Result<cv::Ptr<cv::linemod::QuantizedPyramid>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::linemod::QuantizedPyramid> ret = instance->process(*src, *mask);
			Ok(new cv::Ptr<cv::linemod::QuantizedPyramid>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::linemod::Modality::process(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:132
	// ("cv::linemod::Modality::process", vec![(pred!(const, ["src"], ["const cv::Mat*"]), _)]),
	void cv_linemod_Modality_process_const_const_MatR(const cv::linemod::Modality* instance, const cv::Mat* src, Result<cv::Ptr<cv::linemod::QuantizedPyramid>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::linemod::QuantizedPyramid> ret = instance->process(*src);
			Ok(new cv::Ptr<cv::linemod::QuantizedPyramid>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// name()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:138
	// ("cv::linemod::Modality::name", vec![(pred!(const, [], []), _)]),
	void cv_linemod_Modality_name_const(const cv::linemod::Modality* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->name();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:140
	// ("cv::linemod::Modality::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	void cv_linemod_Modality_read_const_FileNodeR(cv::linemod::Modality* instance, const cv::FileNode* fn, ResultVoid* ocvrs_return) {
		try {
			instance->read(*fn);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:141
	// ("cv::linemod::Modality::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	void cv_linemod_Modality_write_const_FileStorageR(const cv::linemod::Modality* instance, cv::FileStorage* fs, ResultVoid* ocvrs_return) {
		try {
			instance->write(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:150
	// ("cv::linemod::Modality::create", vec![(pred!(mut, ["modality_type"], ["const cv::String*"]), _)]),
	void cv_linemod_Modality_create_const_StringR(const char* modality_type, Result<cv::Ptr<cv::linemod::Modality>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::linemod::Modality> ret = cv::linemod::Modality::create(std::string(modality_type));
			Ok(new cv::Ptr<cv::linemod::Modality>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:155
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

	// quantize(Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:67
	// ("cv::linemod::QuantizedPyramid::quantize", vec![(pred!(const, ["dst"], ["cv::Mat*"]), _)]),
	void cv_linemod_QuantizedPyramid_quantize_const_MatR(const cv::linemod::QuantizedPyramid* instance, cv::Mat* dst, ResultVoid* ocvrs_return) {
		try {
			instance->quantize(*dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// extractTemplate(Template &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:74
	// ("cv::linemod::QuantizedPyramid::extractTemplate", vec![(pred!(const, ["templ"], ["cv::linemod::Template*"]), _)]),
	void cv_linemod_QuantizedPyramid_extractTemplate_const_TemplateR(const cv::linemod::QuantizedPyramid* instance, cv::linemod::Template* templ, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->extractTemplate(*templ);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// pyrDown()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:81
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

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:48
	// ("cv::linemod::Template::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	void cv_linemod_Template_read_const_FileNodeR(cv::linemod::Template* instance, const cv::FileNode* fn, ResultVoid* ocvrs_return) {
		try {
			instance->read(*fn);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:49
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

	// cv::linemod::Template::width() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:43
	// ("cv::linemod::Template::width", vec![(pred!(const, [], []), _)]),
	int cv_linemod_Template_propWidth_const(const cv::linemod::Template* instance) {
			int ret = instance->width;
			return ret;
	}

	// cv::linemod::Template::setWidth(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:43
	// ("cv::linemod::Template::setWidth", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_linemod_Template_propWidth_const_int(cv::linemod::Template* instance, const int val) {
			instance->width = val;
	}

	// cv::linemod::Template::height() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:44
	// ("cv::linemod::Template::height", vec![(pred!(const, [], []), _)]),
	int cv_linemod_Template_propHeight_const(const cv::linemod::Template* instance) {
			int ret = instance->height;
			return ret;
	}

	// cv::linemod::Template::setHeight(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:44
	// ("cv::linemod::Template::setHeight", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_linemod_Template_propHeight_const_int(cv::linemod::Template* instance, const int val) {
			instance->height = val;
	}

	// cv::linemod::Template::pyramid_level() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:45
	// ("cv::linemod::Template::pyramid_level", vec![(pred!(const, [], []), _)]),
	int cv_linemod_Template_propPyramid_level_const(const cv::linemod::Template* instance) {
			int ret = instance->pyramid_level;
			return ret;
	}

	// cv::linemod::Template::setPyramid_level(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:45
	// ("cv::linemod::Template::setPyramid_level", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_linemod_Template_propPyramid_level_const_int(cv::linemod::Template* instance, const int val) {
			instance->pyramid_level = val;
	}

	// cv::linemod::Template::features() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:46
	// ("cv::linemod::Template::features", vec![(pred!(const, [], []), _)]),
	std::vector<cv::linemod::Feature>* cv_linemod_Template_propFeatures_const(const cv::linemod::Template* instance) {
			std::vector<cv::linemod::Feature> ret = instance->features;
			return new std::vector<cv::linemod::Feature>(ret);
	}

	// cv::linemod::Template::setFeatures(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/rgbd/linemod.hpp:46
	// ("cv::linemod::Template::setFeatures", vec![(pred!(mut, ["val"], ["const std::vector<cv::linemod::Feature>"]), _)]),
	void cv_linemod_Template_propFeatures_const_vectorLFeatureG(cv::linemod::Template* instance, const std::vector<cv::linemod::Feature>* val) {
			instance->features = *val;
	}

	// cv::linemod::Template::delete() generated
	// ("cv::linemod::Template::delete", vec![(pred!(mut, [], []), _)]),
	void cv_linemod_Template_delete(cv::linemod::Template* instance) {
			delete instance;
	}

}
