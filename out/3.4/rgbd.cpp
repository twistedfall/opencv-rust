#include "ocvrs_common.hpp"
#include <opencv2/rgbd.hpp>
#include "rgbd_types.hpp"

extern "C" {
	// colormap(const Mat &, Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:282
	// ("cv::linemod::colormap", vec![(pred!(mut, ["quantized", "dst"], ["const cv::Mat*", "cv::Mat*"]), _)]),
	void cv_linemod_colormap_const_MatR_MatR(const cv::Mat* quantized, cv::Mat* dst, ResultVoid* ocvrs_return) {
		try {
			cv::linemod::colormap(*quantized, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDefaultLINE()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:448
	// ("cv::linemod::getDefaultLINE", vec![(pred!(mut, [], []), _)]),
	void cv_linemod_getDefaultLINE(Result<cv::Ptr<cv::linemod::Detector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::linemod::Detector> ret = cv::linemod::getDefaultLINE();
			Ok(new cv::Ptr<cv::linemod::Detector>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDefaultLINEMOD()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:456
	// ("cv::linemod::getDefaultLINEMOD", vec![(pred!(mut, [], []), _)]),
	void cv_linemod_getDefaultLINEMOD(Result<cv::Ptr<cv::linemod::Detector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::linemod::Detector> ret = cv::linemod::getDefaultLINEMOD();
			Ok(new cv::Ptr<cv::linemod::Detector>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// depthTo3dSparse(InputArray, InputArray, InputArray, OutputArray)(InputArray, InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:333
	// ("cv::rgbd::depthTo3dSparse", vec![(pred!(mut, ["depth", "in_K", "in_points", "points3d"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_rgbd_depthTo3dSparse_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* depth, const cv::_InputArray* in_K, const cv::_InputArray* in_points, const cv::_OutputArray* points3d, ResultVoid* ocvrs_return) {
		try {
			cv::rgbd::depthTo3dSparse(*depth, *in_K, *in_points, *points3d);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rgbd::depthTo3d(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:346
	// ("cv::rgbd::depthTo3d", vec![(pred!(mut, ["depth", "K", "points3d"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_rgbd_depthTo3d_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* depth, const cv::_InputArray* K, const cv::_OutputArray* points3d, ResultVoid* ocvrs_return) {
		try {
			cv::rgbd::depthTo3d(*depth, *K, *points3d);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// depthTo3d(InputArray, InputArray, OutputArray, InputArray)(InputArray, InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:346
	// ("cv::rgbd::depthTo3d", vec![(pred!(mut, ["depth", "K", "points3d", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
	void cv_rgbd_depthTo3d_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR(const cv::_InputArray* depth, const cv::_InputArray* K, const cv::_OutputArray* points3d, const cv::_InputArray* mask, ResultVoid* ocvrs_return) {
		try {
			cv::rgbd::depthTo3d(*depth, *K, *points3d, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isValidDepth(const double &)(Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:67
	// ("cv::rgbd::isValidDepth", vec![(pred!(mut, ["depth"], ["const double*"]), _)]),
	void cv_rgbd_isValidDepth_const_doubleR(const double* depth, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::rgbd::isValidDepth(*depth);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isValidDepth(const float &)(Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:61
	// ("cv::rgbd::isValidDepth", vec![(pred!(mut, ["depth"], ["const float*"]), _)]),
	void cv_rgbd_isValidDepth_const_floatR(const float* depth, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::rgbd::isValidDepth(*depth);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isValidDepth(const int &)(Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:86
	// ("cv::rgbd::isValidDepth", vec![(pred!(mut, ["depth"], ["const int*"]), _)]),
	void cv_rgbd_isValidDepth_const_intR(const int* depth, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::rgbd::isValidDepth(*depth);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isValidDepth(const short &)(Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:73
	// ("cv::rgbd::isValidDepth", vec![(pred!(mut, ["depth"], ["const short*"]), _)]),
	void cv_rgbd_isValidDepth_const_shortR(const short* depth, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::rgbd::isValidDepth(*depth);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isValidDepth(const unsigned int &)(Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:92
	// ("cv::rgbd::isValidDepth", vec![(pred!(mut, ["depth"], ["const unsigned int*"]), _)]),
	void cv_rgbd_isValidDepth_const_unsigned_intR(const unsigned int* depth, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::rgbd::isValidDepth(*depth);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isValidDepth(const unsigned short &)(Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:79
	// ("cv::rgbd::isValidDepth", vec![(pred!(mut, ["depth"], ["const unsigned short*"]), _)]),
	void cv_rgbd_isValidDepth_const_unsigned_shortR(const unsigned short* depth, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::rgbd::isValidDepth(*depth);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rgbd::registerDepth(InputArray, InputArray, InputArray, InputArray, InputArray, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:321
	// ("cv::rgbd::registerDepth", vec![(pred!(mut, ["unregisteredCameraMatrix", "registeredCameraMatrix", "registeredDistCoeffs", "Rt", "unregisteredDepth", "outputImagePlaneSize", "registeredDepth"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Size*", "const cv::_OutputArray*"]), _)]),
	void cv_rgbd_registerDepth_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR_const__OutputArrayR(const cv::_InputArray* unregisteredCameraMatrix, const cv::_InputArray* registeredCameraMatrix, const cv::_InputArray* registeredDistCoeffs, const cv::_InputArray* Rt, const cv::_InputArray* unregisteredDepth, const cv::Size* outputImagePlaneSize, const cv::_OutputArray* registeredDepth, ResultVoid* ocvrs_return) {
		try {
			cv::rgbd::registerDepth(*unregisteredCameraMatrix, *registeredCameraMatrix, *registeredDistCoeffs, *Rt, *unregisteredDepth, *outputImagePlaneSize, *registeredDepth);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// registerDepth(InputArray, InputArray, InputArray, InputArray, InputArray, const Size &, OutputArray, bool)(InputArray, InputArray, InputArray, InputArray, InputArray, SimpleClass, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:321
	// ("cv::rgbd::registerDepth", vec![(pred!(mut, ["unregisteredCameraMatrix", "registeredCameraMatrix", "registeredDistCoeffs", "Rt", "unregisteredDepth", "outputImagePlaneSize", "registeredDepth", "depthDilation"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Size*", "const cv::_OutputArray*", "bool"]), _)]),
	void cv_rgbd_registerDepth_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR_const__OutputArrayR_bool(const cv::_InputArray* unregisteredCameraMatrix, const cv::_InputArray* registeredCameraMatrix, const cv::_InputArray* registeredDistCoeffs, const cv::_InputArray* Rt, const cv::_InputArray* unregisteredDepth, const cv::Size* outputImagePlaneSize, const cv::_OutputArray* registeredDepth, bool depthDilation, ResultVoid* ocvrs_return) {
		try {
			cv::rgbd::registerDepth(*unregisteredCameraMatrix, *registeredCameraMatrix, *registeredDistCoeffs, *Rt, *unregisteredDepth, *outputImagePlaneSize, *registeredDepth, depthDilation);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// rescaleDepth(InputArray, int, OutputArray)(InputArray, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:358
	// ("cv::rgbd::rescaleDepth", vec![(pred!(mut, ["in", "depth", "out"], ["const cv::_InputArray*", "int", "const cv::_OutputArray*"]), _)]),
	void cv_rgbd_rescaleDepth_const__InputArrayR_int_const__OutputArrayR(const cv::_InputArray* in, int depth, const cv::_OutputArray* out, ResultVoid* ocvrs_return) {
		try {
			cv::rgbd::rescaleDepth(*in, depth, *out);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rgbd::warpFrame(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:1076
	// ("cv::rgbd::warpFrame", vec![(pred!(mut, ["image", "depth", "mask", "Rt", "cameraMatrix", "distCoeff", "warpedImage"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::_OutputArray*"]), _)]),
	void cv_rgbd_warpFrame_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_const__OutputArrayR(const cv::Mat* image, const cv::Mat* depth, const cv::Mat* mask, const cv::Mat* Rt, const cv::Mat* cameraMatrix, const cv::Mat* distCoeff, const cv::_OutputArray* warpedImage, ResultVoid* ocvrs_return) {
		try {
			cv::rgbd::warpFrame(*image, *depth, *mask, *Rt, *cameraMatrix, *distCoeff, *warpedImage);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// warpFrame(const Mat &, const Mat &, const Mat &, const Mat &, const Mat &, const Mat &, OutputArray, OutputArray, OutputArray)(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:1076
	// ("cv::rgbd::warpFrame", vec![(pred!(mut, ["image", "depth", "mask", "Rt", "cameraMatrix", "distCoeff", "warpedImage", "warpedDepth", "warpedMask"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_rgbd_warpFrame_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::Mat* image, const cv::Mat* depth, const cv::Mat* mask, const cv::Mat* Rt, const cv::Mat* cameraMatrix, const cv::Mat* distCoeff, const cv::_OutputArray* warpedImage, const cv::_OutputArray* warpedDepth, const cv::_OutputArray* warpedMask, ResultVoid* ocvrs_return) {
		try {
			cv::rgbd::warpFrame(*image, *depth, *mask, *Rt, *cameraMatrix, *distCoeff, *warpedImage, *warpedDepth, *warpedMask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ColorGradient()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:209
	// ("cv::linemod::ColorGradient::ColorGradient", vec![(pred!(mut, [], []), _)]),
	void cv_linemod_ColorGradient_ColorGradient(Result<cv::linemod::ColorGradient*>* ocvrs_return) {
		try {
			cv::linemod::ColorGradient* ret = new cv::linemod::ColorGradient();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ColorGradient(float, size_t, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:219
	// ("cv::linemod::ColorGradient::ColorGradient", vec![(pred!(mut, ["weak_threshold", "num_features", "strong_threshold"], ["float", "size_t", "float"]), _)]),
	void cv_linemod_ColorGradient_ColorGradient_float_size_t_float(float weak_threshold, size_t num_features, float strong_threshold, Result<cv::linemod::ColorGradient*>* ocvrs_return) {
		try {
			cv::linemod::ColorGradient* ret = new cv::linemod::ColorGradient(weak_threshold, num_features, strong_threshold);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(float, size_t, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:221
	// ("cv::linemod::ColorGradient::create", vec![(pred!(mut, ["weak_threshold", "num_features", "strong_threshold"], ["float", "size_t", "float"]), _)]),
	void cv_linemod_ColorGradient_create_float_size_t_float(float weak_threshold, size_t num_features, float strong_threshold, Result<cv::Ptr<cv::linemod::ColorGradient>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::linemod::ColorGradient> ret = cv::linemod::ColorGradient::create(weak_threshold, num_features, strong_threshold);
			Ok(new cv::Ptr<cv::linemod::ColorGradient>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// name()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:223
	// ("cv::linemod::ColorGradient::name", vec![(pred!(const, [], []), _)]),
	void cv_linemod_ColorGradient_name_const(const cv::linemod::ColorGradient* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->name();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:225
	// ("cv::linemod::ColorGradient::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	void cv_linemod_ColorGradient_read_const_FileNodeR(cv::linemod::ColorGradient* instance, const cv::FileNode* fn, ResultVoid* ocvrs_return) {
		try {
			instance->read(*fn);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:226
	// ("cv::linemod::ColorGradient::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	void cv_linemod_ColorGradient_write_const_FileStorageR(const cv::linemod::ColorGradient* instance, cv::FileStorage* fs, ResultVoid* ocvrs_return) {
		try {
			instance->write(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::linemod::ColorGradient::weak_threshold() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:228
	// ("cv::linemod::ColorGradient::weak_threshold", vec![(pred!(const, [], []), _)]),
	float cv_linemod_ColorGradient_propWeak_threshold_const(const cv::linemod::ColorGradient* instance) {
			float ret = instance->weak_threshold;
			return ret;
	}

	// cv::linemod::ColorGradient::setWeak_threshold(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:228
	// ("cv::linemod::ColorGradient::setWeak_threshold", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_linemod_ColorGradient_propWeak_threshold_const_float(cv::linemod::ColorGradient* instance, const float val) {
			instance->weak_threshold = val;
	}

	// cv::linemod::ColorGradient::num_features() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:229
	// ("cv::linemod::ColorGradient::num_features", vec![(pred!(const, [], []), _)]),
	size_t cv_linemod_ColorGradient_propNum_features_const(const cv::linemod::ColorGradient* instance) {
			size_t ret = instance->num_features;
			return ret;
	}

	// cv::linemod::ColorGradient::setNum_features(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:229
	// ("cv::linemod::ColorGradient::setNum_features", vec![(pred!(mut, ["val"], ["const size_t"]), _)]),
	void cv_linemod_ColorGradient_propNum_features_const_size_t(cv::linemod::ColorGradient* instance, const size_t val) {
			instance->num_features = val;
	}

	// cv::linemod::ColorGradient::strong_threshold() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:230
	// ("cv::linemod::ColorGradient::strong_threshold", vec![(pred!(const, [], []), _)]),
	float cv_linemod_ColorGradient_propStrong_threshold_const(const cv::linemod::ColorGradient* instance) {
			float ret = instance->strong_threshold;
			return ret;
	}

	// cv::linemod::ColorGradient::setStrong_threshold(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:230
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

	// DepthNormal()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:246
	// ("cv::linemod::DepthNormal::DepthNormal", vec![(pred!(mut, [], []), _)]),
	void cv_linemod_DepthNormal_DepthNormal(Result<cv::linemod::DepthNormal*>* ocvrs_return) {
		try {
			cv::linemod::DepthNormal* ret = new cv::linemod::DepthNormal();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// DepthNormal(int, int, size_t, int)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:258
	// ("cv::linemod::DepthNormal::DepthNormal", vec![(pred!(mut, ["distance_threshold", "difference_threshold", "num_features", "extract_threshold"], ["int", "int", "size_t", "int"]), _)]),
	void cv_linemod_DepthNormal_DepthNormal_int_int_size_t_int(int distance_threshold, int difference_threshold, size_t num_features, int extract_threshold, Result<cv::linemod::DepthNormal*>* ocvrs_return) {
		try {
			cv::linemod::DepthNormal* ret = new cv::linemod::DepthNormal(distance_threshold, difference_threshold, num_features, extract_threshold);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(int, int, size_t, int)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:261
	// ("cv::linemod::DepthNormal::create", vec![(pred!(mut, ["distance_threshold", "difference_threshold", "num_features", "extract_threshold"], ["int", "int", "size_t", "int"]), _)]),
	void cv_linemod_DepthNormal_create_int_int_size_t_int(int distance_threshold, int difference_threshold, size_t num_features, int extract_threshold, Result<cv::Ptr<cv::linemod::DepthNormal>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::linemod::DepthNormal> ret = cv::linemod::DepthNormal::create(distance_threshold, difference_threshold, num_features, extract_threshold);
			Ok(new cv::Ptr<cv::linemod::DepthNormal>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// name()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:264
	// ("cv::linemod::DepthNormal::name", vec![(pred!(const, [], []), _)]),
	void cv_linemod_DepthNormal_name_const(const cv::linemod::DepthNormal* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->name();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:266
	// ("cv::linemod::DepthNormal::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	void cv_linemod_DepthNormal_read_const_FileNodeR(cv::linemod::DepthNormal* instance, const cv::FileNode* fn, ResultVoid* ocvrs_return) {
		try {
			instance->read(*fn);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:267
	// ("cv::linemod::DepthNormal::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	void cv_linemod_DepthNormal_write_const_FileStorageR(const cv::linemod::DepthNormal* instance, cv::FileStorage* fs, ResultVoid* ocvrs_return) {
		try {
			instance->write(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::linemod::DepthNormal::distance_threshold() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:269
	// ("cv::linemod::DepthNormal::distance_threshold", vec![(pred!(const, [], []), _)]),
	int cv_linemod_DepthNormal_propDistance_threshold_const(const cv::linemod::DepthNormal* instance) {
			int ret = instance->distance_threshold;
			return ret;
	}

	// cv::linemod::DepthNormal::setDistance_threshold(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:269
	// ("cv::linemod::DepthNormal::setDistance_threshold", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_linemod_DepthNormal_propDistance_threshold_const_int(cv::linemod::DepthNormal* instance, const int val) {
			instance->distance_threshold = val;
	}

	// cv::linemod::DepthNormal::difference_threshold() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:270
	// ("cv::linemod::DepthNormal::difference_threshold", vec![(pred!(const, [], []), _)]),
	int cv_linemod_DepthNormal_propDifference_threshold_const(const cv::linemod::DepthNormal* instance) {
			int ret = instance->difference_threshold;
			return ret;
	}

	// cv::linemod::DepthNormal::setDifference_threshold(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:270
	// ("cv::linemod::DepthNormal::setDifference_threshold", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_linemod_DepthNormal_propDifference_threshold_const_int(cv::linemod::DepthNormal* instance, const int val) {
			instance->difference_threshold = val;
	}

	// cv::linemod::DepthNormal::num_features() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:271
	// ("cv::linemod::DepthNormal::num_features", vec![(pred!(const, [], []), _)]),
	size_t cv_linemod_DepthNormal_propNum_features_const(const cv::linemod::DepthNormal* instance) {
			size_t ret = instance->num_features;
			return ret;
	}

	// cv::linemod::DepthNormal::setNum_features(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:271
	// ("cv::linemod::DepthNormal::setNum_features", vec![(pred!(mut, ["val"], ["const size_t"]), _)]),
	void cv_linemod_DepthNormal_propNum_features_const_size_t(cv::linemod::DepthNormal* instance, const size_t val) {
			instance->num_features = val;
	}

	// cv::linemod::DepthNormal::extract_threshold() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:272
	// ("cv::linemod::DepthNormal::extract_threshold", vec![(pred!(const, [], []), _)]),
	int cv_linemod_DepthNormal_propExtract_threshold_const(const cv::linemod::DepthNormal* instance) {
			int ret = instance->extract_threshold;
			return ret;
	}

	// cv::linemod::DepthNormal::setExtract_threshold(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:272
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

	// Detector()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:332
	// ("cv::linemod::Detector::Detector", vec![(pred!(mut, [], []), _)]),
	void cv_linemod_Detector_Detector(Result<cv::linemod::Detector*>* ocvrs_return) {
		try {
			cv::linemod::Detector* ret = new cv::linemod::Detector();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Detector(const std::vector<Ptr<Modality>> &, const std::vector<int> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:341
	// ("cv::linemod::Detector::Detector", vec![(pred!(mut, ["modalities", "T_pyramid"], ["const std::vector<cv::Ptr<cv::linemod::Modality>>*", "const std::vector<int>*"]), _)]),
	void cv_linemod_Detector_Detector_const_vectorLPtrLModalityGGR_const_vectorLintGR(const std::vector<cv::Ptr<cv::linemod::Modality>>* modalities, const std::vector<int>* T_pyramid, Result<cv::linemod::Detector*>* ocvrs_return) {
		try {
			cv::linemod::Detector* ret = new cv::linemod::Detector(*modalities, *T_pyramid);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// match(const std::vector<Mat> &, float, std::vector<Match> &, const std::vector<String> &, OutputArrayOfArrays, const std::vector<Mat> &)(CppPassByVoidPtr, Primitive, CppPassByVoidPtr, CppPassByVoidPtr, OutputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:358
	// ("cv::linemod::Detector::match", vec![(pred!(const, ["sources", "threshold", "matches", "class_ids", "quantized_images", "masks"], ["const std::vector<cv::Mat>*", "float", "std::vector<cv::linemod::Match>*", "const std::vector<cv::String>*", "const cv::_OutputArray*", "const std::vector<cv::Mat>*"]), _)]),
	void cv_linemod_Detector_match_const_const_vectorLMatGR_float_vectorLMatchGR_const_vectorLStringGR_const__OutputArrayR_const_vectorLMatGR(const cv::linemod::Detector* instance, const std::vector<cv::Mat>* sources, float threshold, std::vector<cv::linemod::Match>* matches, const std::vector<cv::String>* class_ids, const cv::_OutputArray* quantized_images, const std::vector<cv::Mat>* masks, ResultVoid* ocvrs_return) {
		try {
			instance->match(*sources, threshold, *matches, *class_ids, *quantized_images, *masks);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::linemod::Detector::match(CppPassByVoidPtr, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:358
	// ("cv::linemod::Detector::match", vec![(pred!(const, ["sources", "threshold", "matches"], ["const std::vector<cv::Mat>*", "float", "std::vector<cv::linemod::Match>*"]), _)]),
	void cv_linemod_Detector_match_const_const_vectorLMatGR_float_vectorLMatchGR(const cv::linemod::Detector* instance, const std::vector<cv::Mat>* sources, float threshold, std::vector<cv::linemod::Match>* matches, ResultVoid* ocvrs_return) {
		try {
			instance->match(*sources, threshold, *matches);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// addTemplate(const std::vector<Mat> &, const String &, const Mat &, Rect *)(CppPassByVoidPtr, InString, TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:373
	// ("cv::linemod::Detector::addTemplate", vec![(pred!(mut, ["sources", "class_id", "object_mask", "bounding_box"], ["const std::vector<cv::Mat>*", "const cv::String*", "const cv::Mat*", "cv::Rect*"]), _)]),
	void cv_linemod_Detector_addTemplate_const_vectorLMatGR_const_StringR_const_MatR_RectX(cv::linemod::Detector* instance, const std::vector<cv::Mat>* sources, const char* class_id, const cv::Mat* object_mask, cv::Rect* bounding_box, Result<int>* ocvrs_return) {
		try {
			int ret = instance->addTemplate(*sources, cv::String(class_id), *object_mask, bounding_box);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::linemod::Detector::addTemplate(CppPassByVoidPtr, InString, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:373
	// ("cv::linemod::Detector::addTemplate", vec![(pred!(mut, ["sources", "class_id", "object_mask"], ["const std::vector<cv::Mat>*", "const cv::String*", "const cv::Mat*"]), _)]),
	void cv_linemod_Detector_addTemplate_const_vectorLMatGR_const_StringR_const_MatR(cv::linemod::Detector* instance, const std::vector<cv::Mat>* sources, const char* class_id, const cv::Mat* object_mask, Result<int>* ocvrs_return) {
		try {
			int ret = instance->addTemplate(*sources, cv::String(class_id), *object_mask);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// addSyntheticTemplate(const std::vector<Template> &, const String &)(CppPassByVoidPtr, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:379
	// ("cv::linemod::Detector::addSyntheticTemplate", vec![(pred!(mut, ["templates", "class_id"], ["const std::vector<cv::linemod::Template>*", "const cv::String*"]), _)]),
	void cv_linemod_Detector_addSyntheticTemplate_const_vectorLTemplateGR_const_StringR(cv::linemod::Detector* instance, const std::vector<cv::linemod::Template>* templates, const char* class_id, Result<int>* ocvrs_return) {
		try {
			int ret = instance->addSyntheticTemplate(*templates, cv::String(class_id));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getModalities()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:387
	// ("cv::linemod::Detector::getModalities", vec![(pred!(const, [], []), _)]),
	void cv_linemod_Detector_getModalities_const(const cv::linemod::Detector* instance, Result<std::vector<cv::Ptr<cv::linemod::Modality>>*>* ocvrs_return) {
		try {
			const std::vector<cv::Ptr<cv::linemod::Modality>> ret = instance->getModalities();
			Ok(new const std::vector<cv::Ptr<cv::linemod::Modality>>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getT(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:392
	// ("cv::linemod::Detector::getT", vec![(pred!(const, ["pyramid_level"], ["int"]), _)]),
	void cv_linemod_Detector_getT_const_int(const cv::linemod::Detector* instance, int pyramid_level, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getT(pyramid_level);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// pyramidLevels()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:397
	// ("cv::linemod::Detector::pyramidLevels", vec![(pred!(const, [], []), _)]),
	void cv_linemod_Detector_pyramidLevels_const(const cv::linemod::Detector* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->pyramidLevels();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTemplates(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:405
	// ("cv::linemod::Detector::getTemplates", vec![(pred!(const, ["class_id", "template_id"], ["const cv::String*", "int"]), _)]),
	void cv_linemod_Detector_getTemplates_const_const_StringR_int(const cv::linemod::Detector* instance, const char* class_id, int template_id, Result<std::vector<cv::linemod::Template>*>* ocvrs_return) {
		try {
			const std::vector<cv::linemod::Template> ret = instance->getTemplates(cv::String(class_id), template_id);
			Ok(new const std::vector<cv::linemod::Template>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// numTemplates()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:407
	// ("cv::linemod::Detector::numTemplates", vec![(pred!(const, [], []), _)]),
	void cv_linemod_Detector_numTemplates_const(const cv::linemod::Detector* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->numTemplates();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// numTemplates(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:408
	// ("cv::linemod::Detector::numTemplates", vec![(pred!(const, ["class_id"], ["const cv::String*"]), _)]),
	void cv_linemod_Detector_numTemplates_const_const_StringR(const cv::linemod::Detector* instance, const char* class_id, Result<int>* ocvrs_return) {
		try {
			int ret = instance->numTemplates(cv::String(class_id));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// numClasses()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:409
	// ("cv::linemod::Detector::numClasses", vec![(pred!(const, [], []), _)]),
	void cv_linemod_Detector_numClasses_const(const cv::linemod::Detector* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->numClasses();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// classIds()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:411
	// ("cv::linemod::Detector::classIds", vec![(pred!(const, [], []), _)]),
	void cv_linemod_Detector_classIds_const(const cv::linemod::Detector* instance, Result<std::vector<cv::String>*>* ocvrs_return) {
		try {
			std::vector<cv::String> ret = instance->classIds();
			Ok(new std::vector<cv::String>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:413
	// ("cv::linemod::Detector::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	void cv_linemod_Detector_read_const_FileNodeR(cv::linemod::Detector* instance, const cv::FileNode* fn, ResultVoid* ocvrs_return) {
		try {
			instance->read(*fn);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:414
	// ("cv::linemod::Detector::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	void cv_linemod_Detector_write_const_FileStorageR(const cv::linemod::Detector* instance, cv::FileStorage* fs, ResultVoid* ocvrs_return) {
		try {
			instance->write(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readClass(const FileNode &, const String &)(TraitClass, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:416
	// ("cv::linemod::Detector::readClass", vec![(pred!(mut, ["fn", "class_id_override"], ["const cv::FileNode*", "const cv::String*"]), _)]),
	void cv_linemod_Detector_readClass_const_FileNodeR_const_StringR(cv::linemod::Detector* instance, const cv::FileNode* fn, const char* class_id_override, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->readClass(*fn, cv::String(class_id_override));
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::linemod::Detector::readClass(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:416
	// ("cv::linemod::Detector::readClass", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	void cv_linemod_Detector_readClass_const_FileNodeR(cv::linemod::Detector* instance, const cv::FileNode* fn, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->readClass(*fn);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// writeClass(const String &, FileStorage &)(InString, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:417
	// ("cv::linemod::Detector::writeClass", vec![(pred!(const, ["class_id", "fs"], ["const cv::String*", "cv::FileStorage*"]), _)]),
	void cv_linemod_Detector_writeClass_const_const_StringR_FileStorageR(const cv::linemod::Detector* instance, const char* class_id, cv::FileStorage* fs, ResultVoid* ocvrs_return) {
		try {
			instance->writeClass(cv::String(class_id), *fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// readClasses(const std::vector<String> &, const String &)(CppPassByVoidPtr, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:419
	// ("cv::linemod::Detector::readClasses", vec![(pred!(mut, ["class_ids", "format"], ["const std::vector<cv::String>*", "const cv::String*"]), _)]),
	void cv_linemod_Detector_readClasses_const_vectorLStringGR_const_StringR(cv::linemod::Detector* instance, const std::vector<cv::String>* class_ids, const char* format, ResultVoid* ocvrs_return) {
		try {
			instance->readClasses(*class_ids, cv::String(format));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::linemod::Detector::readClasses(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:419
	// ("cv::linemod::Detector::readClasses", vec![(pred!(mut, ["class_ids"], ["const std::vector<cv::String>*"]), _)]),
	void cv_linemod_Detector_readClasses_const_vectorLStringGR(cv::linemod::Detector* instance, const std::vector<cv::String>* class_ids, ResultVoid* ocvrs_return) {
		try {
			instance->readClasses(*class_ids);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// writeClasses(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:421
	// ("cv::linemod::Detector::writeClasses", vec![(pred!(const, ["format"], ["const cv::String*"]), _)]),
	void cv_linemod_Detector_writeClasses_const_const_StringR(const cv::linemod::Detector* instance, const char* format, ResultVoid* ocvrs_return) {
		try {
			instance->writeClasses(cv::String(format));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::linemod::Detector::writeClasses() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:421
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

	// Feature()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:69
	// ("cv::linemod::Feature::Feature", vec![(pred!(mut, [], []), _)]),
	void cv_linemod_Feature_Feature(Result<cv::linemod::Feature>* ocvrs_return) {
		try {
			cv::linemod::Feature ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Feature(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:70
	// ("cv::linemod::Feature::Feature", vec![(pred!(mut, ["x", "y", "label"], ["int", "int", "int"]), _)]),
	void cv_linemod_Feature_Feature_int_int_int(int x, int y, int label, Result<cv::linemod::Feature>* ocvrs_return) {
		try {
			cv::linemod::Feature ret(x, y, label);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:72
	// ("cv::linemod::Feature::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	void cv_linemod_Feature_read_const_FileNodeR(cv::linemod::Feature* instance, const cv::FileNode* fn, ResultVoid* ocvrs_return) {
		try {
			instance->read(*fn);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:73
	// ("cv::linemod::Feature::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	void cv_linemod_Feature_write_const_FileStorageR(const cv::linemod::Feature* instance, cv::FileStorage* fs, ResultVoid* ocvrs_return) {
		try {
			instance->write(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Match()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:289
	// ("cv::linemod::Match::Match", vec![(pred!(mut, [], []), _)]),
	void cv_linemod_Match_Match(Result<cv::linemod::Match*>* ocvrs_return) {
		try {
			cv::linemod::Match* ret = new cv::linemod::Match();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Match(int, int, float, const String &, int)(Primitive, Primitive, Primitive, InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:293
	// ("cv::linemod::Match::Match", vec![(pred!(mut, ["x", "y", "similarity", "class_id", "template_id"], ["int", "int", "float", "const cv::String*", "int"]), _)]),
	void cv_linemod_Match_Match_int_int_float_const_StringR_int(int x, int y, float similarity, const char* class_id, int template_id, Result<cv::linemod::Match*>* ocvrs_return) {
		try {
			cv::linemod::Match* ret = new cv::linemod::Match(x, y, similarity, cv::String(class_id), template_id);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator<(const Match &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:296
	// ("cv::linemod::Match::operator<", vec![(pred!(const, ["rhs"], ["const cv::linemod::Match*"]), _)]),
	void cv_linemod_Match_operatorL_const_const_MatchR(const cv::linemod::Match* instance, const cv::linemod::Match* rhs, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->operator<(*rhs);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator==(const Match &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:305
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

	// cv::linemod::Match::x() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:310
	// ("cv::linemod::Match::x", vec![(pred!(const, [], []), _)]),
	int cv_linemod_Match_propX_const(const cv::linemod::Match* instance) {
			int ret = instance->x;
			return ret;
	}

	// cv::linemod::Match::setX(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:310
	// ("cv::linemod::Match::setX", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_linemod_Match_propX_const_int(cv::linemod::Match* instance, const int val) {
			instance->x = val;
	}

	// cv::linemod::Match::y() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:311
	// ("cv::linemod::Match::y", vec![(pred!(const, [], []), _)]),
	int cv_linemod_Match_propY_const(const cv::linemod::Match* instance) {
			int ret = instance->y;
			return ret;
	}

	// cv::linemod::Match::setY(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:311
	// ("cv::linemod::Match::setY", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_linemod_Match_propY_const_int(cv::linemod::Match* instance, const int val) {
			instance->y = val;
	}

	// cv::linemod::Match::similarity() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:312
	// ("cv::linemod::Match::similarity", vec![(pred!(const, [], []), _)]),
	float cv_linemod_Match_propSimilarity_const(const cv::linemod::Match* instance) {
			float ret = instance->similarity;
			return ret;
	}

	// cv::linemod::Match::setSimilarity(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:312
	// ("cv::linemod::Match::setSimilarity", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_linemod_Match_propSimilarity_const_float(cv::linemod::Match* instance, const float val) {
			instance->similarity = val;
	}

	// cv::linemod::Match::class_id() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:313
	// ("cv::linemod::Match::class_id", vec![(pred!(const, [], []), _)]),
	void* cv_linemod_Match_propClass_id_const(const cv::linemod::Match* instance) {
			cv::String ret = instance->class_id;
			return ocvrs_create_string(ret.c_str());
	}

	// cv::linemod::Match::setClass_id(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:313
	// ("cv::linemod::Match::setClass_id", vec![(pred!(mut, ["val"], ["const cv::String"]), _)]),
	void cv_linemod_Match_propClass_id_const_String(cv::linemod::Match* instance, const char* val) {
			instance->class_id = cv::String(val);
	}

	// cv::linemod::Match::template_id() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:314
	// ("cv::linemod::Match::template_id", vec![(pred!(const, [], []), _)]),
	int cv_linemod_Match_propTemplate_id_const(const cv::linemod::Match* instance) {
			int ret = instance->template_id;
			return ret;
	}

	// cv::linemod::Match::setTemplate_id(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:314
	// ("cv::linemod::Match::setTemplate_id", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_linemod_Match_propTemplate_id_const_int(cv::linemod::Match* instance, const int val) {
			instance->template_id = val;
	}

	// cv::linemod::Match::delete() generated
	// ("cv::linemod::Match::delete", vec![(pred!(mut, [], []), _)]),
	void cv_linemod_Match_delete(cv::linemod::Match* instance) {
			delete instance;
	}

	// process(const Mat &, const Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:169
	// ("cv::linemod::Modality::process", vec![(pred!(const, ["src", "mask"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
	void cv_linemod_Modality_process_const_const_MatR_const_MatR(const cv::linemod::Modality* instance, const cv::Mat* src, const cv::Mat* mask, Result<cv::Ptr<cv::linemod::QuantizedPyramid>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::linemod::QuantizedPyramid> ret = instance->process(*src, *mask);
			Ok(new cv::Ptr<cv::linemod::QuantizedPyramid>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::linemod::Modality::process(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:169
	// ("cv::linemod::Modality::process", vec![(pred!(const, ["src"], ["const cv::Mat*"]), _)]),
	void cv_linemod_Modality_process_const_const_MatR(const cv::linemod::Modality* instance, const cv::Mat* src, Result<cv::Ptr<cv::linemod::QuantizedPyramid>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::linemod::QuantizedPyramid> ret = instance->process(*src);
			Ok(new cv::Ptr<cv::linemod::QuantizedPyramid>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// name()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:175
	// ("cv::linemod::Modality::name", vec![(pred!(const, [], []), _)]),
	void cv_linemod_Modality_name_const(const cv::linemod::Modality* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->name();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:177
	// ("cv::linemod::Modality::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	void cv_linemod_Modality_read_const_FileNodeR(cv::linemod::Modality* instance, const cv::FileNode* fn, ResultVoid* ocvrs_return) {
		try {
			instance->read(*fn);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:178
	// ("cv::linemod::Modality::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	void cv_linemod_Modality_write_const_FileStorageR(const cv::linemod::Modality* instance, cv::FileStorage* fs, ResultVoid* ocvrs_return) {
		try {
			instance->write(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:187
	// ("cv::linemod::Modality::create", vec![(pred!(mut, ["modality_type"], ["const cv::String*"]), _)]),
	void cv_linemod_Modality_create_const_StringR(const char* modality_type, Result<cv::Ptr<cv::linemod::Modality>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::linemod::Modality> ret = cv::linemod::Modality::create(cv::String(modality_type));
			Ok(new cv::Ptr<cv::linemod::Modality>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:192
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

	// quantize(Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:104
	// ("cv::linemod::QuantizedPyramid::quantize", vec![(pred!(const, ["dst"], ["cv::Mat*"]), _)]),
	void cv_linemod_QuantizedPyramid_quantize_const_MatR(const cv::linemod::QuantizedPyramid* instance, cv::Mat* dst, ResultVoid* ocvrs_return) {
		try {
			instance->quantize(*dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// extractTemplate(Template &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:111
	// ("cv::linemod::QuantizedPyramid::extractTemplate", vec![(pred!(const, ["templ"], ["cv::linemod::Template*"]), _)]),
	void cv_linemod_QuantizedPyramid_extractTemplate_const_TemplateR(const cv::linemod::QuantizedPyramid* instance, cv::linemod::Template* templ, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->extractTemplate(*templ);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// pyrDown()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:118
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

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:85
	// ("cv::linemod::Template::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	void cv_linemod_Template_read_const_FileNodeR(cv::linemod::Template* instance, const cv::FileNode* fn, ResultVoid* ocvrs_return) {
		try {
			instance->read(*fn);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:86
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

	// cv::linemod::Template::width() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:80
	// ("cv::linemod::Template::width", vec![(pred!(const, [], []), _)]),
	int cv_linemod_Template_propWidth_const(const cv::linemod::Template* instance) {
			int ret = instance->width;
			return ret;
	}

	// cv::linemod::Template::setWidth(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:80
	// ("cv::linemod::Template::setWidth", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_linemod_Template_propWidth_const_int(cv::linemod::Template* instance, const int val) {
			instance->width = val;
	}

	// cv::linemod::Template::height() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:81
	// ("cv::linemod::Template::height", vec![(pred!(const, [], []), _)]),
	int cv_linemod_Template_propHeight_const(const cv::linemod::Template* instance) {
			int ret = instance->height;
			return ret;
	}

	// cv::linemod::Template::setHeight(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:81
	// ("cv::linemod::Template::setHeight", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_linemod_Template_propHeight_const_int(cv::linemod::Template* instance, const int val) {
			instance->height = val;
	}

	// cv::linemod::Template::pyramid_level() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:82
	// ("cv::linemod::Template::pyramid_level", vec![(pred!(const, [], []), _)]),
	int cv_linemod_Template_propPyramid_level_const(const cv::linemod::Template* instance) {
			int ret = instance->pyramid_level;
			return ret;
	}

	// cv::linemod::Template::setPyramid_level(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:82
	// ("cv::linemod::Template::setPyramid_level", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_linemod_Template_propPyramid_level_const_int(cv::linemod::Template* instance, const int val) {
			instance->pyramid_level = val;
	}

	// cv::linemod::Template::features() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:83
	// ("cv::linemod::Template::features", vec![(pred!(const, [], []), _)]),
	std::vector<cv::linemod::Feature>* cv_linemod_Template_propFeatures_const(const cv::linemod::Template* instance) {
			std::vector<cv::linemod::Feature> ret = instance->features;
			return new std::vector<cv::linemod::Feature>(ret);
	}

	// cv::linemod::Template::setFeatures(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd/linemod.hpp:83
	// ("cv::linemod::Template::setFeatures", vec![(pred!(mut, ["val"], ["const std::vector<cv::linemod::Feature>"]), _)]),
	void cv_linemod_Template_propFeatures_const_vectorLFeatureG(cv::linemod::Template* instance, const std::vector<cv::linemod::Feature>* val) {
			instance->features = *val;
	}

	// cv::linemod::Template::delete() generated
	// ("cv::linemod::Template::delete", vec![(pred!(mut, [], []), _)]),
	void cv_linemod_Template_delete(cv::linemod::Template* instance) {
			delete instance;
	}

	// DepthCleaner()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:232
	// ("cv::rgbd::DepthCleaner::DepthCleaner", vec![(pred!(mut, [], []), _)]),
	void cv_rgbd_DepthCleaner_DepthCleaner(Result<cv::rgbd::DepthCleaner*>* ocvrs_return) {
		try {
			cv::rgbd::DepthCleaner* ret = new cv::rgbd::DepthCleaner();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// DepthCleaner(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:246
	// ("cv::rgbd::DepthCleaner::DepthCleaner", vec![(pred!(mut, ["depth", "window_size", "method"], ["int", "int", "int"]), _)]),
	void cv_rgbd_DepthCleaner_DepthCleaner_int_int_int(int depth, int window_size, int method, Result<cv::rgbd::DepthCleaner*>* ocvrs_return) {
		try {
			cv::rgbd::DepthCleaner* ret = new cv::rgbd::DepthCleaner(depth, window_size, method);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rgbd::DepthCleaner::DepthCleaner(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:246
	// ("cv::rgbd::DepthCleaner::DepthCleaner", vec![(pred!(mut, ["depth"], ["int"]), _)]),
	void cv_rgbd_DepthCleaner_DepthCleaner_int(int depth, Result<cv::rgbd::DepthCleaner*>* ocvrs_return) {
		try {
			cv::rgbd::DepthCleaner* ret = new cv::rgbd::DepthCleaner(depth);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(int, int, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:250
	// ("cv::rgbd::DepthCleaner::create", vec![(pred!(mut, ["depth", "window_size", "method"], ["int", "int", "int"]), _)]),
	void cv_rgbd_DepthCleaner_create_int_int_int(int depth, int window_size, int method, Result<cv::Ptr<cv::rgbd::DepthCleaner>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::rgbd::DepthCleaner> ret = cv::rgbd::DepthCleaner::create(depth, window_size, method);
			Ok(new cv::Ptr<cv::rgbd::DepthCleaner>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rgbd::DepthCleaner::create(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:250
	// ("cv::rgbd::DepthCleaner::create", vec![(pred!(mut, ["depth"], ["int"]), _)]),
	void cv_rgbd_DepthCleaner_create_int(int depth, Result<cv::Ptr<cv::rgbd::DepthCleaner>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::rgbd::DepthCleaner> ret = cv::rgbd::DepthCleaner::create(depth);
			Ok(new cv::Ptr<cv::rgbd::DepthCleaner>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator()(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:257
	// ("cv::rgbd::DepthCleaner::operator()", vec![(pred!(const, ["points", "depth"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_rgbd_DepthCleaner_operator___const_const__InputArrayR_const__OutputArrayR(const cv::rgbd::DepthCleaner* instance, const cv::_InputArray* points, const cv::_OutputArray* depth, ResultVoid* ocvrs_return) {
		try {
			instance->operator()(*points, *depth);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// initialize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:263
	// ("cv::rgbd::DepthCleaner::initialize", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_DepthCleaner_initialize_const(const cv::rgbd::DepthCleaner* instance, ResultVoid* ocvrs_return) {
		try {
			instance->initialize();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getWindowSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:265
	// ("cv::rgbd::DepthCleaner::getWindowSize", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_DepthCleaner_getWindowSize_const(const cv::rgbd::DepthCleaner* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getWindowSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWindowSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:269
	// ("cv::rgbd::DepthCleaner::setWindowSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_rgbd_DepthCleaner_setWindowSize_int(cv::rgbd::DepthCleaner* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setWindowSize(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDepth()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:273
	// ("cv::rgbd::DepthCleaner::getDepth", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_DepthCleaner_getDepth_const(const cv::rgbd::DepthCleaner* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getDepth();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDepth(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:277
	// ("cv::rgbd::DepthCleaner::setDepth", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_rgbd_DepthCleaner_setDepth_int(cv::rgbd::DepthCleaner* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setDepth(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMethod()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:281
	// ("cv::rgbd::DepthCleaner::getMethod", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_DepthCleaner_getMethod_const(const cv::rgbd::DepthCleaner* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMethod();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMethod(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:285
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

	// ICPOdometry()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:795
	// ("cv::rgbd::ICPOdometry::ICPOdometry", vec![(pred!(mut, [], []), _)]),
	void cv_rgbd_ICPOdometry_ICPOdometry(Result<cv::rgbd::ICPOdometry*>* ocvrs_return) {
		try {
			cv::rgbd::ICPOdometry* ret = new cv::rgbd::ICPOdometry();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ICPOdometry(const Mat &, float, float, float, float, const std::vector<int> &, int)(TraitClass, Primitive, Primitive, Primitive, Primitive, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:806
	// ("cv::rgbd::ICPOdometry::ICPOdometry", vec![(pred!(mut, ["cameraMatrix", "minDepth", "maxDepth", "maxDepthDiff", "maxPointsPart", "iterCounts", "transformType"], ["const cv::Mat*", "float", "float", "float", "float", "const std::vector<int>*", "int"]), _)]),
	void cv_rgbd_ICPOdometry_ICPOdometry_const_MatR_float_float_float_float_const_vectorLintGR_int(const cv::Mat* cameraMatrix, float minDepth, float maxDepth, float maxDepthDiff, float maxPointsPart, const std::vector<int>* iterCounts, int transformType, Result<cv::rgbd::ICPOdometry*>* ocvrs_return) {
		try {
			cv::rgbd::ICPOdometry* ret = new cv::rgbd::ICPOdometry(*cameraMatrix, minDepth, maxDepth, maxDepthDiff, maxPointsPart, *iterCounts, transformType);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rgbd::ICPOdometry::ICPOdometry(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:806
	// ("cv::rgbd::ICPOdometry::ICPOdometry", vec![(pred!(mut, ["cameraMatrix"], ["const cv::Mat*"]), _)]),
	void cv_rgbd_ICPOdometry_ICPOdometry_const_MatR(const cv::Mat* cameraMatrix, Result<cv::rgbd::ICPOdometry*>* ocvrs_return) {
		try {
			cv::rgbd::ICPOdometry* ret = new cv::rgbd::ICPOdometry(*cameraMatrix);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const Mat &, float, float, float, float, const std::vector<int> &, int)(TraitClass, Primitive, Primitive, Primitive, Primitive, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:810
	// ("cv::rgbd::ICPOdometry::create", vec![(pred!(mut, ["cameraMatrix", "minDepth", "maxDepth", "maxDepthDiff", "maxPointsPart", "iterCounts", "transformType"], ["const cv::Mat*", "float", "float", "float", "float", "const std::vector<int>*", "int"]), _)]),
	void cv_rgbd_ICPOdometry_create_const_MatR_float_float_float_float_const_vectorLintGR_int(const cv::Mat* cameraMatrix, float minDepth, float maxDepth, float maxDepthDiff, float maxPointsPart, const std::vector<int>* iterCounts, int transformType, Result<cv::Ptr<cv::rgbd::ICPOdometry>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::rgbd::ICPOdometry> ret = cv::rgbd::ICPOdometry::create(*cameraMatrix, minDepth, maxDepth, maxDepthDiff, maxPointsPart, *iterCounts, transformType);
			Ok(new cv::Ptr<cv::rgbd::ICPOdometry>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rgbd::ICPOdometry::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:810
	// ("cv::rgbd::ICPOdometry::create", vec![(pred!(mut, [], []), _)]),
	void cv_rgbd_ICPOdometry_create(Result<cv::Ptr<cv::rgbd::ICPOdometry>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::rgbd::ICPOdometry> ret = cv::rgbd::ICPOdometry::create();
			Ok(new cv::Ptr<cv::rgbd::ICPOdometry>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// prepareFrameCache(Ptr<OdometryFrame> &, int)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:814
	// ("cv::rgbd::ICPOdometry::prepareFrameCache", vec![(pred!(const, ["frame", "cacheType"], ["cv::Ptr<cv::rgbd::OdometryFrame>*", "int"]), _)]),
	void cv_rgbd_ICPOdometry_prepareFrameCache_const_PtrLOdometryFrameGR_int(const cv::rgbd::ICPOdometry* instance, cv::Ptr<cv::rgbd::OdometryFrame>* frame, int cacheType, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->prepareFrameCache(*frame, cacheType);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getCameraMatrix()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:816
	// ("cv::rgbd::ICPOdometry::getCameraMatrix", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_ICPOdometry_getCameraMatrix_const(const cv::rgbd::ICPOdometry* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getCameraMatrix();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setCameraMatrix(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:820
	// ("cv::rgbd::ICPOdometry::setCameraMatrix", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
	void cv_rgbd_ICPOdometry_setCameraMatrix_const_MatR(cv::rgbd::ICPOdometry* instance, const cv::Mat* val, ResultVoid* ocvrs_return) {
		try {
			instance->setCameraMatrix(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMinDepth()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:824
	// ("cv::rgbd::ICPOdometry::getMinDepth", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_ICPOdometry_getMinDepth_const(const cv::rgbd::ICPOdometry* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMinDepth();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMinDepth(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:828
	// ("cv::rgbd::ICPOdometry::setMinDepth", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_rgbd_ICPOdometry_setMinDepth_double(cv::rgbd::ICPOdometry* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setMinDepth(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxDepth()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:832
	// ("cv::rgbd::ICPOdometry::getMaxDepth", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_ICPOdometry_getMaxDepth_const(const cv::rgbd::ICPOdometry* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxDepth();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxDepth(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:836
	// ("cv::rgbd::ICPOdometry::setMaxDepth", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_rgbd_ICPOdometry_setMaxDepth_double(cv::rgbd::ICPOdometry* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxDepth(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxDepthDiff()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:840
	// ("cv::rgbd::ICPOdometry::getMaxDepthDiff", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_ICPOdometry_getMaxDepthDiff_const(const cv::rgbd::ICPOdometry* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxDepthDiff();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxDepthDiff(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:844
	// ("cv::rgbd::ICPOdometry::setMaxDepthDiff", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_rgbd_ICPOdometry_setMaxDepthDiff_double(cv::rgbd::ICPOdometry* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxDepthDiff(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getIterationCounts()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:848
	// ("cv::rgbd::ICPOdometry::getIterationCounts", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_ICPOdometry_getIterationCounts_const(const cv::rgbd::ICPOdometry* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getIterationCounts();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setIterationCounts(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:852
	// ("cv::rgbd::ICPOdometry::setIterationCounts", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
	void cv_rgbd_ICPOdometry_setIterationCounts_const_MatR(cv::rgbd::ICPOdometry* instance, const cv::Mat* val, ResultVoid* ocvrs_return) {
		try {
			instance->setIterationCounts(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxPointsPart()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:856
	// ("cv::rgbd::ICPOdometry::getMaxPointsPart", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_ICPOdometry_getMaxPointsPart_const(const cv::rgbd::ICPOdometry* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxPointsPart();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxPointsPart(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:860
	// ("cv::rgbd::ICPOdometry::setMaxPointsPart", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_rgbd_ICPOdometry_setMaxPointsPart_double(cv::rgbd::ICPOdometry* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxPointsPart(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTransformType()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:864
	// ("cv::rgbd::ICPOdometry::getTransformType", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_ICPOdometry_getTransformType_const(const cv::rgbd::ICPOdometry* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getTransformType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTransformType(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:868
	// ("cv::rgbd::ICPOdometry::setTransformType", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_rgbd_ICPOdometry_setTransformType_int(cv::rgbd::ICPOdometry* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setTransformType(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxTranslation()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:872
	// ("cv::rgbd::ICPOdometry::getMaxTranslation", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_ICPOdometry_getMaxTranslation_const(const cv::rgbd::ICPOdometry* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxTranslation();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxTranslation(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:876
	// ("cv::rgbd::ICPOdometry::setMaxTranslation", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_rgbd_ICPOdometry_setMaxTranslation_double(cv::rgbd::ICPOdometry* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxTranslation(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxRotation()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:880
	// ("cv::rgbd::ICPOdometry::getMaxRotation", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_ICPOdometry_getMaxRotation_const(const cv::rgbd::ICPOdometry* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxRotation();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxRotation(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:884
	// ("cv::rgbd::ICPOdometry::setMaxRotation", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_rgbd_ICPOdometry_setMaxRotation_double(cv::rgbd::ICPOdometry* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxRotation(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNormalsComputer()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:888
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

	// DEFAULT_MIN_DEPTH()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:568
	// ("cv::rgbd::Odometry::DEFAULT_MIN_DEPTH", vec![(pred!(mut, [], []), _)]),
	void cv_rgbd_Odometry_DEFAULT_MIN_DEPTH(Result<float>* ocvrs_return) {
		try {
			float ret = cv::rgbd::Odometry::DEFAULT_MIN_DEPTH();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// DEFAULT_MAX_DEPTH()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:573
	// ("cv::rgbd::Odometry::DEFAULT_MAX_DEPTH", vec![(pred!(mut, [], []), _)]),
	void cv_rgbd_Odometry_DEFAULT_MAX_DEPTH(Result<float>* ocvrs_return) {
		try {
			float ret = cv::rgbd::Odometry::DEFAULT_MAX_DEPTH();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// DEFAULT_MAX_DEPTH_DIFF()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:578
	// ("cv::rgbd::Odometry::DEFAULT_MAX_DEPTH_DIFF", vec![(pred!(mut, [], []), _)]),
	void cv_rgbd_Odometry_DEFAULT_MAX_DEPTH_DIFF(Result<float>* ocvrs_return) {
		try {
			float ret = cv::rgbd::Odometry::DEFAULT_MAX_DEPTH_DIFF();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// DEFAULT_MAX_POINTS_PART()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:583
	// ("cv::rgbd::Odometry::DEFAULT_MAX_POINTS_PART", vec![(pred!(mut, [], []), _)]),
	void cv_rgbd_Odometry_DEFAULT_MAX_POINTS_PART(Result<float>* ocvrs_return) {
		try {
			float ret = cv::rgbd::Odometry::DEFAULT_MAX_POINTS_PART();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// DEFAULT_MAX_TRANSLATION()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:588
	// ("cv::rgbd::Odometry::DEFAULT_MAX_TRANSLATION", vec![(pred!(mut, [], []), _)]),
	void cv_rgbd_Odometry_DEFAULT_MAX_TRANSLATION(Result<float>* ocvrs_return) {
		try {
			float ret = cv::rgbd::Odometry::DEFAULT_MAX_TRANSLATION();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// DEFAULT_MAX_ROTATION()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:593
	// ("cv::rgbd::Odometry::DEFAULT_MAX_ROTATION", vec![(pred!(mut, [], []), _)]),
	void cv_rgbd_Odometry_DEFAULT_MAX_ROTATION(Result<float>* ocvrs_return) {
		try {
			float ret = cv::rgbd::Odometry::DEFAULT_MAX_ROTATION();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// compute(const Mat &, const Mat &, const Mat &, const Mat &, const Mat &, const Mat &, OutputArray, const Mat &)(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:617
	// ("cv::rgbd::Odometry::compute", vec![(pred!(const, ["srcImage", "srcDepth", "srcMask", "dstImage", "dstDepth", "dstMask", "Rt", "initRt"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::_OutputArray*", "const cv::Mat*"]), _)]),
	void cv_rgbd_Odometry_compute_const_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_const__OutputArrayR_const_MatR(const cv::rgbd::Odometry* instance, const cv::Mat* srcImage, const cv::Mat* srcDepth, const cv::Mat* srcMask, const cv::Mat* dstImage, const cv::Mat* dstDepth, const cv::Mat* dstMask, const cv::_OutputArray* Rt, const cv::Mat* initRt, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->compute(*srcImage, *srcDepth, *srcMask, *dstImage, *dstDepth, *dstMask, *Rt, *initRt);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rgbd::Odometry::compute(TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:617
	// ("cv::rgbd::Odometry::compute", vec![(pred!(const, ["srcImage", "srcDepth", "srcMask", "dstImage", "dstDepth", "dstMask", "Rt"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::_OutputArray*"]), _)]),
	void cv_rgbd_Odometry_compute_const_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_const_MatR_const__OutputArrayR(const cv::rgbd::Odometry* instance, const cv::Mat* srcImage, const cv::Mat* srcDepth, const cv::Mat* srcMask, const cv::Mat* dstImage, const cv::Mat* dstDepth, const cv::Mat* dstMask, const cv::_OutputArray* Rt, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->compute(*srcImage, *srcDepth, *srcMask, *dstImage, *dstDepth, *dstMask, *Rt);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// compute(Ptr<OdometryFrame> &, Ptr<OdometryFrame> &, OutputArray, const Mat &)(CppPassByVoidPtr, CppPassByVoidPtr, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:624
	// ("cv::rgbd::Odometry::compute", vec![(pred!(const, ["srcFrame", "dstFrame", "Rt", "initRt"], ["cv::Ptr<cv::rgbd::OdometryFrame>*", "cv::Ptr<cv::rgbd::OdometryFrame>*", "const cv::_OutputArray*", "const cv::Mat*"]), _)]),
	void cv_rgbd_Odometry_compute_const_PtrLOdometryFrameGR_PtrLOdometryFrameGR_const__OutputArrayR_const_MatR(const cv::rgbd::Odometry* instance, cv::Ptr<cv::rgbd::OdometryFrame>* srcFrame, cv::Ptr<cv::rgbd::OdometryFrame>* dstFrame, const cv::_OutputArray* Rt, const cv::Mat* initRt, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->compute(*srcFrame, *dstFrame, *Rt, *initRt);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rgbd::Odometry::compute(CppPassByVoidPtr, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:624
	// ("cv::rgbd::Odometry::compute", vec![(pred!(const, ["srcFrame", "dstFrame", "Rt"], ["cv::Ptr<cv::rgbd::OdometryFrame>*", "cv::Ptr<cv::rgbd::OdometryFrame>*", "const cv::_OutputArray*"]), _)]),
	void cv_rgbd_Odometry_compute_const_PtrLOdometryFrameGR_PtrLOdometryFrameGR_const__OutputArrayR(const cv::rgbd::Odometry* instance, cv::Ptr<cv::rgbd::OdometryFrame>* srcFrame, cv::Ptr<cv::rgbd::OdometryFrame>* dstFrame, const cv::_OutputArray* Rt, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->compute(*srcFrame, *dstFrame, *Rt);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// prepareFrameCache(Ptr<OdometryFrame> &, int)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:632
	// ("cv::rgbd::Odometry::prepareFrameCache", vec![(pred!(const, ["frame", "cacheType"], ["cv::Ptr<cv::rgbd::OdometryFrame>*", "int"]), _)]),
	void cv_rgbd_Odometry_prepareFrameCache_const_PtrLOdometryFrameGR_int(const cv::rgbd::Odometry* instance, cv::Ptr<cv::rgbd::OdometryFrame>* frame, int cacheType, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->prepareFrameCache(*frame, cacheType);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:634
	// ("cv::rgbd::Odometry::create", vec![(pred!(mut, ["odometryType"], ["const cv::String*"]), _)]),
	void cv_rgbd_Odometry_create_const_StringR(const char* odometryType, Result<cv::Ptr<cv::rgbd::Odometry>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::rgbd::Odometry> ret = cv::rgbd::Odometry::create(cv::String(odometryType));
			Ok(new cv::Ptr<cv::rgbd::Odometry>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getCameraMatrix()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:637
	// ("cv::rgbd::Odometry::getCameraMatrix", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_Odometry_getCameraMatrix_const(const cv::rgbd::Odometry* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getCameraMatrix();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setCameraMatrix(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:639
	// ("cv::rgbd::Odometry::setCameraMatrix", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
	void cv_rgbd_Odometry_setCameraMatrix_const_MatR(cv::rgbd::Odometry* instance, const cv::Mat* val, ResultVoid* ocvrs_return) {
		try {
			instance->setCameraMatrix(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTransformType()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:641
	// ("cv::rgbd::Odometry::getTransformType", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_Odometry_getTransformType_const(const cv::rgbd::Odometry* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getTransformType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTransformType(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:643
	// ("cv::rgbd::Odometry::setTransformType", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_rgbd_Odometry_setTransformType_int(cv::rgbd::Odometry* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setTransformType(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
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

	// OdometryFrame()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:530
	// ("cv::rgbd::OdometryFrame::OdometryFrame", vec![(pred!(mut, [], []), _)]),
	void cv_rgbd_OdometryFrame_OdometryFrame(Result<cv::rgbd::OdometryFrame*>* ocvrs_return) {
		try {
			cv::rgbd::OdometryFrame* ret = new cv::rgbd::OdometryFrame();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// OdometryFrame(const Mat &, const Mat &, const Mat &, const Mat &, int)(TraitClass, TraitClass, TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:531
	// ("cv::rgbd::OdometryFrame::OdometryFrame", vec![(pred!(mut, ["image", "depth", "mask", "normals", "ID"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "int"]), _)]),
	void cv_rgbd_OdometryFrame_OdometryFrame_const_MatR_const_MatR_const_MatR_const_MatR_int(const cv::Mat* image, const cv::Mat* depth, const cv::Mat* mask, const cv::Mat* normals, int ID, Result<cv::rgbd::OdometryFrame*>* ocvrs_return) {
		try {
			cv::rgbd::OdometryFrame* ret = new cv::rgbd::OdometryFrame(*image, *depth, *mask, *normals, ID);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rgbd::OdometryFrame::OdometryFrame(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:531
	// ("cv::rgbd::OdometryFrame::OdometryFrame", vec![(pred!(mut, ["image", "depth"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
	void cv_rgbd_OdometryFrame_OdometryFrame_const_MatR_const_MatR(const cv::Mat* image, const cv::Mat* depth, Result<cv::rgbd::OdometryFrame*>* ocvrs_return) {
		try {
			cv::rgbd::OdometryFrame* ret = new cv::rgbd::OdometryFrame(*image, *depth);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const Mat &, const Mat &, const Mat &, const Mat &, int)(TraitClass, TraitClass, TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:533
	// ("cv::rgbd::OdometryFrame::create", vec![(pred!(mut, ["image", "depth", "mask", "normals", "ID"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "int"]), _)]),
	void cv_rgbd_OdometryFrame_create_const_MatR_const_MatR_const_MatR_const_MatR_int(const cv::Mat* image, const cv::Mat* depth, const cv::Mat* mask, const cv::Mat* normals, int ID, Result<cv::Ptr<cv::rgbd::OdometryFrame>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::rgbd::OdometryFrame> ret = cv::rgbd::OdometryFrame::create(*image, *depth, *mask, *normals, ID);
			Ok(new cv::Ptr<cv::rgbd::OdometryFrame>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rgbd::OdometryFrame::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:533
	// ("cv::rgbd::OdometryFrame::create", vec![(pred!(mut, [], []), _)]),
	void cv_rgbd_OdometryFrame_create(Result<cv::Ptr<cv::rgbd::OdometryFrame>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::rgbd::OdometryFrame> ret = cv::rgbd::OdometryFrame::create();
			Ok(new cv::Ptr<cv::rgbd::OdometryFrame>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// release()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:536
	// ("cv::rgbd::OdometryFrame::release", vec![(pred!(mut, [], []), _)]),
	void cv_rgbd_OdometryFrame_release(cv::rgbd::OdometryFrame* instance, ResultVoid* ocvrs_return) {
		try {
			instance->release();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// releasePyramids()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:539
	// ("cv::rgbd::OdometryFrame::releasePyramids", vec![(pred!(mut, [], []), _)]),
	void cv_rgbd_OdometryFrame_releasePyramids(cv::rgbd::OdometryFrame* instance, ResultVoid* ocvrs_return) {
		try {
			instance->releasePyramids();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rgbd::OdometryFrame::pyramidImage() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:541
	// ("cv::rgbd::OdometryFrame::pyramidImage", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Mat>* cv_rgbd_OdometryFrame_propPyramidImage_const(const cv::rgbd::OdometryFrame* instance) {
			std::vector<cv::Mat> ret = instance->pyramidImage;
			return new std::vector<cv::Mat>(ret);
	}

	// cv::rgbd::OdometryFrame::setPyramidImage(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:541
	// ("cv::rgbd::OdometryFrame::setPyramidImage", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
	void cv_rgbd_OdometryFrame_propPyramidImage_const_vectorLMatG(cv::rgbd::OdometryFrame* instance, const std::vector<cv::Mat>* val) {
			instance->pyramidImage = *val;
	}

	// cv::rgbd::OdometryFrame::pyramidDepth() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:542
	// ("cv::rgbd::OdometryFrame::pyramidDepth", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Mat>* cv_rgbd_OdometryFrame_propPyramidDepth_const(const cv::rgbd::OdometryFrame* instance) {
			std::vector<cv::Mat> ret = instance->pyramidDepth;
			return new std::vector<cv::Mat>(ret);
	}

	// cv::rgbd::OdometryFrame::setPyramidDepth(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:542
	// ("cv::rgbd::OdometryFrame::setPyramidDepth", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
	void cv_rgbd_OdometryFrame_propPyramidDepth_const_vectorLMatG(cv::rgbd::OdometryFrame* instance, const std::vector<cv::Mat>* val) {
			instance->pyramidDepth = *val;
	}

	// cv::rgbd::OdometryFrame::pyramidMask() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:543
	// ("cv::rgbd::OdometryFrame::pyramidMask", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Mat>* cv_rgbd_OdometryFrame_propPyramidMask_const(const cv::rgbd::OdometryFrame* instance) {
			std::vector<cv::Mat> ret = instance->pyramidMask;
			return new std::vector<cv::Mat>(ret);
	}

	// cv::rgbd::OdometryFrame::setPyramidMask(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:543
	// ("cv::rgbd::OdometryFrame::setPyramidMask", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
	void cv_rgbd_OdometryFrame_propPyramidMask_const_vectorLMatG(cv::rgbd::OdometryFrame* instance, const std::vector<cv::Mat>* val) {
			instance->pyramidMask = *val;
	}

	// cv::rgbd::OdometryFrame::pyramidCloud() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:545
	// ("cv::rgbd::OdometryFrame::pyramidCloud", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Mat>* cv_rgbd_OdometryFrame_propPyramidCloud_const(const cv::rgbd::OdometryFrame* instance) {
			std::vector<cv::Mat> ret = instance->pyramidCloud;
			return new std::vector<cv::Mat>(ret);
	}

	// cv::rgbd::OdometryFrame::setPyramidCloud(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:545
	// ("cv::rgbd::OdometryFrame::setPyramidCloud", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
	void cv_rgbd_OdometryFrame_propPyramidCloud_const_vectorLMatG(cv::rgbd::OdometryFrame* instance, const std::vector<cv::Mat>* val) {
			instance->pyramidCloud = *val;
	}

	// cv::rgbd::OdometryFrame::pyramid_dI_dx() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:547
	// ("cv::rgbd::OdometryFrame::pyramid_dI_dx", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Mat>* cv_rgbd_OdometryFrame_propPyramid_dI_dx_const(const cv::rgbd::OdometryFrame* instance) {
			std::vector<cv::Mat> ret = instance->pyramid_dI_dx;
			return new std::vector<cv::Mat>(ret);
	}

	// cv::rgbd::OdometryFrame::setPyramid_dI_dx(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:547
	// ("cv::rgbd::OdometryFrame::setPyramid_dI_dx", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
	void cv_rgbd_OdometryFrame_propPyramid_dI_dx_const_vectorLMatG(cv::rgbd::OdometryFrame* instance, const std::vector<cv::Mat>* val) {
			instance->pyramid_dI_dx = *val;
	}

	// cv::rgbd::OdometryFrame::pyramid_dI_dy() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:548
	// ("cv::rgbd::OdometryFrame::pyramid_dI_dy", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Mat>* cv_rgbd_OdometryFrame_propPyramid_dI_dy_const(const cv::rgbd::OdometryFrame* instance) {
			std::vector<cv::Mat> ret = instance->pyramid_dI_dy;
			return new std::vector<cv::Mat>(ret);
	}

	// cv::rgbd::OdometryFrame::setPyramid_dI_dy(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:548
	// ("cv::rgbd::OdometryFrame::setPyramid_dI_dy", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
	void cv_rgbd_OdometryFrame_propPyramid_dI_dy_const_vectorLMatG(cv::rgbd::OdometryFrame* instance, const std::vector<cv::Mat>* val) {
			instance->pyramid_dI_dy = *val;
	}

	// cv::rgbd::OdometryFrame::pyramidTexturedMask() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:549
	// ("cv::rgbd::OdometryFrame::pyramidTexturedMask", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Mat>* cv_rgbd_OdometryFrame_propPyramidTexturedMask_const(const cv::rgbd::OdometryFrame* instance) {
			std::vector<cv::Mat> ret = instance->pyramidTexturedMask;
			return new std::vector<cv::Mat>(ret);
	}

	// cv::rgbd::OdometryFrame::setPyramidTexturedMask(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:549
	// ("cv::rgbd::OdometryFrame::setPyramidTexturedMask", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
	void cv_rgbd_OdometryFrame_propPyramidTexturedMask_const_vectorLMatG(cv::rgbd::OdometryFrame* instance, const std::vector<cv::Mat>* val) {
			instance->pyramidTexturedMask = *val;
	}

	// cv::rgbd::OdometryFrame::pyramidNormals() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:551
	// ("cv::rgbd::OdometryFrame::pyramidNormals", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Mat>* cv_rgbd_OdometryFrame_propPyramidNormals_const(const cv::rgbd::OdometryFrame* instance) {
			std::vector<cv::Mat> ret = instance->pyramidNormals;
			return new std::vector<cv::Mat>(ret);
	}

	// cv::rgbd::OdometryFrame::setPyramidNormals(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:551
	// ("cv::rgbd::OdometryFrame::setPyramidNormals", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
	void cv_rgbd_OdometryFrame_propPyramidNormals_const_vectorLMatG(cv::rgbd::OdometryFrame* instance, const std::vector<cv::Mat>* val) {
			instance->pyramidNormals = *val;
	}

	// cv::rgbd::OdometryFrame::pyramidNormalsMask() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:552
	// ("cv::rgbd::OdometryFrame::pyramidNormalsMask", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Mat>* cv_rgbd_OdometryFrame_propPyramidNormalsMask_const(const cv::rgbd::OdometryFrame* instance) {
			std::vector<cv::Mat> ret = instance->pyramidNormalsMask;
			return new std::vector<cv::Mat>(ret);
	}

	// cv::rgbd::OdometryFrame::setPyramidNormalsMask(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:552
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

	// RgbdFrame()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:496
	// ("cv::rgbd::RgbdFrame::RgbdFrame", vec![(pred!(mut, [], []), _)]),
	void cv_rgbd_RgbdFrame_RgbdFrame(Result<cv::rgbd::RgbdFrame*>* ocvrs_return) {
		try {
			cv::rgbd::RgbdFrame* ret = new cv::rgbd::RgbdFrame();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// RgbdFrame(const Mat &, const Mat &, const Mat &, const Mat &, int)(TraitClass, TraitClass, TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:497
	// ("cv::rgbd::RgbdFrame::RgbdFrame", vec![(pred!(mut, ["image", "depth", "mask", "normals", "ID"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "int"]), _)]),
	void cv_rgbd_RgbdFrame_RgbdFrame_const_MatR_const_MatR_const_MatR_const_MatR_int(const cv::Mat* image, const cv::Mat* depth, const cv::Mat* mask, const cv::Mat* normals, int ID, Result<cv::rgbd::RgbdFrame*>* ocvrs_return) {
		try {
			cv::rgbd::RgbdFrame* ret = new cv::rgbd::RgbdFrame(*image, *depth, *mask, *normals, ID);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rgbd::RgbdFrame::RgbdFrame(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:497
	// ("cv::rgbd::RgbdFrame::RgbdFrame", vec![(pred!(mut, ["image", "depth"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
	void cv_rgbd_RgbdFrame_RgbdFrame_const_MatR_const_MatR(const cv::Mat* image, const cv::Mat* depth, Result<cv::rgbd::RgbdFrame*>* ocvrs_return) {
		try {
			cv::rgbd::RgbdFrame* ret = new cv::rgbd::RgbdFrame(*image, *depth);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const Mat &, const Mat &, const Mat &, const Mat &, int)(TraitClass, TraitClass, TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:500
	// ("cv::rgbd::RgbdFrame::create", vec![(pred!(mut, ["image", "depth", "mask", "normals", "ID"], ["const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "const cv::Mat*", "int"]), _)]),
	void cv_rgbd_RgbdFrame_create_const_MatR_const_MatR_const_MatR_const_MatR_int(const cv::Mat* image, const cv::Mat* depth, const cv::Mat* mask, const cv::Mat* normals, int ID, Result<cv::Ptr<cv::rgbd::RgbdFrame>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::rgbd::RgbdFrame> ret = cv::rgbd::RgbdFrame::create(*image, *depth, *mask, *normals, ID);
			Ok(new cv::Ptr<cv::rgbd::RgbdFrame>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rgbd::RgbdFrame::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:500
	// ("cv::rgbd::RgbdFrame::create", vec![(pred!(mut, [], []), _)]),
	void cv_rgbd_RgbdFrame_create(Result<cv::Ptr<cv::rgbd::RgbdFrame>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::rgbd::RgbdFrame> ret = cv::rgbd::RgbdFrame::create();
			Ok(new cv::Ptr<cv::rgbd::RgbdFrame>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// release()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:503
	// ("cv::rgbd::RgbdFrame::release", vec![(pred!(mut, [], []), _)]),
	void cv_rgbd_RgbdFrame_release(cv::rgbd::RgbdFrame* instance, ResultVoid* ocvrs_return) {
		try {
			instance->release();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rgbd::RgbdFrame::ID() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:505
	// ("cv::rgbd::RgbdFrame::ID", vec![(pred!(const, [], []), _)]),
	int cv_rgbd_RgbdFrame_propID_const(const cv::rgbd::RgbdFrame* instance) {
			int ret = instance->ID;
			return ret;
	}

	// cv::rgbd::RgbdFrame::setID(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:505
	// ("cv::rgbd::RgbdFrame::setID", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_rgbd_RgbdFrame_propID_const_int(cv::rgbd::RgbdFrame* instance, const int val) {
			instance->ID = val;
	}

	// cv::rgbd::RgbdFrame::image() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:506
	// ("cv::rgbd::RgbdFrame::image", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_rgbd_RgbdFrame_propImage_const(const cv::rgbd::RgbdFrame* instance) {
			cv::Mat ret = instance->image;
			return new cv::Mat(ret);
	}

	// cv::rgbd::RgbdFrame::setImage(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:506
	// ("cv::rgbd::RgbdFrame::setImage", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_rgbd_RgbdFrame_propImage_const_Mat(cv::rgbd::RgbdFrame* instance, const cv::Mat* val) {
			instance->image = *val;
	}

	// cv::rgbd::RgbdFrame::depth() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:507
	// ("cv::rgbd::RgbdFrame::depth", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_rgbd_RgbdFrame_propDepth_const(const cv::rgbd::RgbdFrame* instance) {
			cv::Mat ret = instance->depth;
			return new cv::Mat(ret);
	}

	// cv::rgbd::RgbdFrame::setDepth(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:507
	// ("cv::rgbd::RgbdFrame::setDepth", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_rgbd_RgbdFrame_propDepth_const_Mat(cv::rgbd::RgbdFrame* instance, const cv::Mat* val) {
			instance->depth = *val;
	}

	// cv::rgbd::RgbdFrame::mask() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:508
	// ("cv::rgbd::RgbdFrame::mask", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_rgbd_RgbdFrame_propMask_const(const cv::rgbd::RgbdFrame* instance) {
			cv::Mat ret = instance->mask;
			return new cv::Mat(ret);
	}

	// cv::rgbd::RgbdFrame::setMask(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:508
	// ("cv::rgbd::RgbdFrame::setMask", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_rgbd_RgbdFrame_propMask_const_Mat(cv::rgbd::RgbdFrame* instance, const cv::Mat* val) {
			instance->mask = *val;
	}

	// cv::rgbd::RgbdFrame::normals() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:509
	// ("cv::rgbd::RgbdFrame::normals", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_rgbd_RgbdFrame_propNormals_const(const cv::rgbd::RgbdFrame* instance) {
			cv::Mat ret = instance->normals;
			return new cv::Mat(ret);
	}

	// cv::rgbd::RgbdFrame::setNormals(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:509
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

	// RgbdICPOdometry()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:923
	// ("cv::rgbd::RgbdICPOdometry::RgbdICPOdometry", vec![(pred!(mut, [], []), _)]),
	void cv_rgbd_RgbdICPOdometry_RgbdICPOdometry(Result<cv::rgbd::RgbdICPOdometry*>* ocvrs_return) {
		try {
			cv::rgbd::RgbdICPOdometry* ret = new cv::rgbd::RgbdICPOdometry();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// RgbdICPOdometry(const Mat &, float, float, float, float, const std::vector<int> &, const std::vector<float> &, int)(TraitClass, Primitive, Primitive, Primitive, Primitive, CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:936
	// ("cv::rgbd::RgbdICPOdometry::RgbdICPOdometry", vec![(pred!(mut, ["cameraMatrix", "minDepth", "maxDepth", "maxDepthDiff", "maxPointsPart", "iterCounts", "minGradientMagnitudes", "transformType"], ["const cv::Mat*", "float", "float", "float", "float", "const std::vector<int>*", "const std::vector<float>*", "int"]), _)]),
	void cv_rgbd_RgbdICPOdometry_RgbdICPOdometry_const_MatR_float_float_float_float_const_vectorLintGR_const_vectorLfloatGR_int(const cv::Mat* cameraMatrix, float minDepth, float maxDepth, float maxDepthDiff, float maxPointsPart, const std::vector<int>* iterCounts, const std::vector<float>* minGradientMagnitudes, int transformType, Result<cv::rgbd::RgbdICPOdometry*>* ocvrs_return) {
		try {
			cv::rgbd::RgbdICPOdometry* ret = new cv::rgbd::RgbdICPOdometry(*cameraMatrix, minDepth, maxDepth, maxDepthDiff, maxPointsPart, *iterCounts, *minGradientMagnitudes, transformType);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rgbd::RgbdICPOdometry::RgbdICPOdometry(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:936
	// ("cv::rgbd::RgbdICPOdometry::RgbdICPOdometry", vec![(pred!(mut, ["cameraMatrix"], ["const cv::Mat*"]), _)]),
	void cv_rgbd_RgbdICPOdometry_RgbdICPOdometry_const_MatR(const cv::Mat* cameraMatrix, Result<cv::rgbd::RgbdICPOdometry*>* ocvrs_return) {
		try {
			cv::rgbd::RgbdICPOdometry* ret = new cv::rgbd::RgbdICPOdometry(*cameraMatrix);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const Mat &, float, float, float, float, const std::vector<int> &, const std::vector<float> &, int)(TraitClass, Primitive, Primitive, Primitive, Primitive, CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:942
	// ("cv::rgbd::RgbdICPOdometry::create", vec![(pred!(mut, ["cameraMatrix", "minDepth", "maxDepth", "maxDepthDiff", "maxPointsPart", "iterCounts", "minGradientMagnitudes", "transformType"], ["const cv::Mat*", "float", "float", "float", "float", "const std::vector<int>*", "const std::vector<float>*", "int"]), _)]),
	void cv_rgbd_RgbdICPOdometry_create_const_MatR_float_float_float_float_const_vectorLintGR_const_vectorLfloatGR_int(const cv::Mat* cameraMatrix, float minDepth, float maxDepth, float maxDepthDiff, float maxPointsPart, const std::vector<int>* iterCounts, const std::vector<float>* minGradientMagnitudes, int transformType, Result<cv::Ptr<cv::rgbd::RgbdICPOdometry>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::rgbd::RgbdICPOdometry> ret = cv::rgbd::RgbdICPOdometry::create(*cameraMatrix, minDepth, maxDepth, maxDepthDiff, maxPointsPart, *iterCounts, *minGradientMagnitudes, transformType);
			Ok(new cv::Ptr<cv::rgbd::RgbdICPOdometry>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rgbd::RgbdICPOdometry::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:942
	// ("cv::rgbd::RgbdICPOdometry::create", vec![(pred!(mut, [], []), _)]),
	void cv_rgbd_RgbdICPOdometry_create(Result<cv::Ptr<cv::rgbd::RgbdICPOdometry>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::rgbd::RgbdICPOdometry> ret = cv::rgbd::RgbdICPOdometry::create();
			Ok(new cv::Ptr<cv::rgbd::RgbdICPOdometry>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// prepareFrameCache(Ptr<OdometryFrame> &, int)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:948
	// ("cv::rgbd::RgbdICPOdometry::prepareFrameCache", vec![(pred!(const, ["frame", "cacheType"], ["cv::Ptr<cv::rgbd::OdometryFrame>*", "int"]), _)]),
	void cv_rgbd_RgbdICPOdometry_prepareFrameCache_const_PtrLOdometryFrameGR_int(const cv::rgbd::RgbdICPOdometry* instance, cv::Ptr<cv::rgbd::OdometryFrame>* frame, int cacheType, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->prepareFrameCache(*frame, cacheType);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getCameraMatrix()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:950
	// ("cv::rgbd::RgbdICPOdometry::getCameraMatrix", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdICPOdometry_getCameraMatrix_const(const cv::rgbd::RgbdICPOdometry* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getCameraMatrix();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setCameraMatrix(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:954
	// ("cv::rgbd::RgbdICPOdometry::setCameraMatrix", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
	void cv_rgbd_RgbdICPOdometry_setCameraMatrix_const_MatR(cv::rgbd::RgbdICPOdometry* instance, const cv::Mat* val, ResultVoid* ocvrs_return) {
		try {
			instance->setCameraMatrix(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMinDepth()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:958
	// ("cv::rgbd::RgbdICPOdometry::getMinDepth", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdICPOdometry_getMinDepth_const(const cv::rgbd::RgbdICPOdometry* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMinDepth();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMinDepth(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:962
	// ("cv::rgbd::RgbdICPOdometry::setMinDepth", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_rgbd_RgbdICPOdometry_setMinDepth_double(cv::rgbd::RgbdICPOdometry* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setMinDepth(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxDepth()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:966
	// ("cv::rgbd::RgbdICPOdometry::getMaxDepth", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdICPOdometry_getMaxDepth_const(const cv::rgbd::RgbdICPOdometry* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxDepth();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxDepth(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:970
	// ("cv::rgbd::RgbdICPOdometry::setMaxDepth", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_rgbd_RgbdICPOdometry_setMaxDepth_double(cv::rgbd::RgbdICPOdometry* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxDepth(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxDepthDiff()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:974
	// ("cv::rgbd::RgbdICPOdometry::getMaxDepthDiff", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdICPOdometry_getMaxDepthDiff_const(const cv::rgbd::RgbdICPOdometry* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxDepthDiff();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxDepthDiff(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:978
	// ("cv::rgbd::RgbdICPOdometry::setMaxDepthDiff", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_rgbd_RgbdICPOdometry_setMaxDepthDiff_double(cv::rgbd::RgbdICPOdometry* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxDepthDiff(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxPointsPart()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:982
	// ("cv::rgbd::RgbdICPOdometry::getMaxPointsPart", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdICPOdometry_getMaxPointsPart_const(const cv::rgbd::RgbdICPOdometry* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxPointsPart();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxPointsPart(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:986
	// ("cv::rgbd::RgbdICPOdometry::setMaxPointsPart", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_rgbd_RgbdICPOdometry_setMaxPointsPart_double(cv::rgbd::RgbdICPOdometry* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxPointsPart(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getIterationCounts()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:990
	// ("cv::rgbd::RgbdICPOdometry::getIterationCounts", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdICPOdometry_getIterationCounts_const(const cv::rgbd::RgbdICPOdometry* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getIterationCounts();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setIterationCounts(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:994
	// ("cv::rgbd::RgbdICPOdometry::setIterationCounts", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
	void cv_rgbd_RgbdICPOdometry_setIterationCounts_const_MatR(cv::rgbd::RgbdICPOdometry* instance, const cv::Mat* val, ResultVoid* ocvrs_return) {
		try {
			instance->setIterationCounts(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMinGradientMagnitudes()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:998
	// ("cv::rgbd::RgbdICPOdometry::getMinGradientMagnitudes", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdICPOdometry_getMinGradientMagnitudes_const(const cv::rgbd::RgbdICPOdometry* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getMinGradientMagnitudes();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMinGradientMagnitudes(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:1002
	// ("cv::rgbd::RgbdICPOdometry::setMinGradientMagnitudes", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
	void cv_rgbd_RgbdICPOdometry_setMinGradientMagnitudes_const_MatR(cv::rgbd::RgbdICPOdometry* instance, const cv::Mat* val, ResultVoid* ocvrs_return) {
		try {
			instance->setMinGradientMagnitudes(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTransformType()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:1006
	// ("cv::rgbd::RgbdICPOdometry::getTransformType", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdICPOdometry_getTransformType_const(const cv::rgbd::RgbdICPOdometry* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getTransformType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTransformType(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:1010
	// ("cv::rgbd::RgbdICPOdometry::setTransformType", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_rgbd_RgbdICPOdometry_setTransformType_int(cv::rgbd::RgbdICPOdometry* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setTransformType(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxTranslation()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:1014
	// ("cv::rgbd::RgbdICPOdometry::getMaxTranslation", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdICPOdometry_getMaxTranslation_const(const cv::rgbd::RgbdICPOdometry* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxTranslation();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxTranslation(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:1018
	// ("cv::rgbd::RgbdICPOdometry::setMaxTranslation", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_rgbd_RgbdICPOdometry_setMaxTranslation_double(cv::rgbd::RgbdICPOdometry* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxTranslation(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxRotation()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:1022
	// ("cv::rgbd::RgbdICPOdometry::getMaxRotation", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdICPOdometry_getMaxRotation_const(const cv::rgbd::RgbdICPOdometry* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxRotation();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxRotation(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:1026
	// ("cv::rgbd::RgbdICPOdometry::setMaxRotation", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_rgbd_RgbdICPOdometry_setMaxRotation_double(cv::rgbd::RgbdICPOdometry* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxRotation(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNormalsComputer()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:1030
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

	// RgbdNormals()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:117
	// ("cv::rgbd::RgbdNormals::RgbdNormals", vec![(pred!(mut, [], []), _)]),
	void cv_rgbd_RgbdNormals_RgbdNormals(Result<cv::rgbd::RgbdNormals*>* ocvrs_return) {
		try {
			cv::rgbd::RgbdNormals* ret = new cv::rgbd::RgbdNormals();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// RgbdNormals(int, int, int, InputArray, int, int)(Primitive, Primitive, Primitive, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:137
	// ("cv::rgbd::RgbdNormals::RgbdNormals", vec![(pred!(mut, ["rows", "cols", "depth", "K", "window_size", "method"], ["int", "int", "int", "const cv::_InputArray*", "int", "int"]), _)]),
	void cv_rgbd_RgbdNormals_RgbdNormals_int_int_int_const__InputArrayR_int_int(int rows, int cols, int depth, const cv::_InputArray* K, int window_size, int method, Result<cv::rgbd::RgbdNormals*>* ocvrs_return) {
		try {
			cv::rgbd::RgbdNormals* ret = new cv::rgbd::RgbdNormals(rows, cols, depth, *K, window_size, method);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rgbd::RgbdNormals::RgbdNormals(Primitive, Primitive, Primitive, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:137
	// ("cv::rgbd::RgbdNormals::RgbdNormals", vec![(pred!(mut, ["rows", "cols", "depth", "K"], ["int", "int", "int", "const cv::_InputArray*"]), _)]),
	void cv_rgbd_RgbdNormals_RgbdNormals_int_int_int_const__InputArrayR(int rows, int cols, int depth, const cv::_InputArray* K, Result<cv::rgbd::RgbdNormals*>* ocvrs_return) {
		try {
			cv::rgbd::RgbdNormals* ret = new cv::rgbd::RgbdNormals(rows, cols, depth, *K);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(int, int, int, InputArray, int, int)(Primitive, Primitive, Primitive, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:142
	// ("cv::rgbd::RgbdNormals::create", vec![(pred!(mut, ["rows", "cols", "depth", "K", "window_size", "method"], ["int", "int", "int", "const cv::_InputArray*", "int", "int"]), _)]),
	void cv_rgbd_RgbdNormals_create_int_int_int_const__InputArrayR_int_int(int rows, int cols, int depth, const cv::_InputArray* K, int window_size, int method, Result<cv::Ptr<cv::rgbd::RgbdNormals>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::rgbd::RgbdNormals> ret = cv::rgbd::RgbdNormals::create(rows, cols, depth, *K, window_size, method);
			Ok(new cv::Ptr<cv::rgbd::RgbdNormals>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rgbd::RgbdNormals::create(Primitive, Primitive, Primitive, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:142
	// ("cv::rgbd::RgbdNormals::create", vec![(pred!(mut, ["rows", "cols", "depth", "K"], ["int", "int", "int", "const cv::_InputArray*"]), _)]),
	void cv_rgbd_RgbdNormals_create_int_int_int_const__InputArrayR(int rows, int cols, int depth, const cv::_InputArray* K, Result<cv::Ptr<cv::rgbd::RgbdNormals>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::rgbd::RgbdNormals> ret = cv::rgbd::RgbdNormals::create(rows, cols, depth, *K);
			Ok(new cv::Ptr<cv::rgbd::RgbdNormals>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator()(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:150
	// ("cv::rgbd::RgbdNormals::operator()", vec![(pred!(const, ["points", "normals"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_rgbd_RgbdNormals_operator___const_const__InputArrayR_const__OutputArrayR(const cv::rgbd::RgbdNormals* instance, const cv::_InputArray* points, const cv::_OutputArray* normals, ResultVoid* ocvrs_return) {
		try {
			instance->operator()(*points, *normals);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// initialize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:156
	// ("cv::rgbd::RgbdNormals::initialize", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdNormals_initialize_const(const cv::rgbd::RgbdNormals* instance, ResultVoid* ocvrs_return) {
		try {
			instance->initialize();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getRows()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:158
	// ("cv::rgbd::RgbdNormals::getRows", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdNormals_getRows_const(const cv::rgbd::RgbdNormals* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getRows();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setRows(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:162
	// ("cv::rgbd::RgbdNormals::setRows", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_rgbd_RgbdNormals_setRows_int(cv::rgbd::RgbdNormals* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setRows(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getCols()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:166
	// ("cv::rgbd::RgbdNormals::getCols", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdNormals_getCols_const(const cv::rgbd::RgbdNormals* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getCols();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setCols(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:170
	// ("cv::rgbd::RgbdNormals::setCols", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_rgbd_RgbdNormals_setCols_int(cv::rgbd::RgbdNormals* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setCols(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getWindowSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:174
	// ("cv::rgbd::RgbdNormals::getWindowSize", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdNormals_getWindowSize_const(const cv::rgbd::RgbdNormals* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getWindowSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWindowSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:178
	// ("cv::rgbd::RgbdNormals::setWindowSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_rgbd_RgbdNormals_setWindowSize_int(cv::rgbd::RgbdNormals* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setWindowSize(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDepth()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:182
	// ("cv::rgbd::RgbdNormals::getDepth", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdNormals_getDepth_const(const cv::rgbd::RgbdNormals* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getDepth();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDepth(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:186
	// ("cv::rgbd::RgbdNormals::setDepth", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_rgbd_RgbdNormals_setDepth_int(cv::rgbd::RgbdNormals* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setDepth(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getK()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:190
	// ("cv::rgbd::RgbdNormals::getK", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdNormals_getK_const(const cv::rgbd::RgbdNormals* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getK();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setK(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:194
	// ("cv::rgbd::RgbdNormals::setK", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
	void cv_rgbd_RgbdNormals_setK_const_MatR(cv::rgbd::RgbdNormals* instance, const cv::Mat* val, ResultVoid* ocvrs_return) {
		try {
			instance->setK(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMethod()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:198
	// ("cv::rgbd::RgbdNormals::getMethod", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdNormals_getMethod_const(const cv::rgbd::RgbdNormals* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMethod();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMethod(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:202
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

	// RgbdOdometry()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:660
	// ("cv::rgbd::RgbdOdometry::RgbdOdometry", vec![(pred!(mut, [], []), _)]),
	void cv_rgbd_RgbdOdometry_RgbdOdometry(Result<cv::rgbd::RgbdOdometry*>* ocvrs_return) {
		try {
			cv::rgbd::RgbdOdometry* ret = new cv::rgbd::RgbdOdometry();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// RgbdOdometry(const Mat &, float, float, float, const std::vector<int> &, const std::vector<float> &, float, int)(TraitClass, Primitive, Primitive, Primitive, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:673
	// ("cv::rgbd::RgbdOdometry::RgbdOdometry", vec![(pred!(mut, ["cameraMatrix", "minDepth", "maxDepth", "maxDepthDiff", "iterCounts", "minGradientMagnitudes", "maxPointsPart", "transformType"], ["const cv::Mat*", "float", "float", "float", "const std::vector<int>*", "const std::vector<float>*", "float", "int"]), _)]),
	void cv_rgbd_RgbdOdometry_RgbdOdometry_const_MatR_float_float_float_const_vectorLintGR_const_vectorLfloatGR_float_int(const cv::Mat* cameraMatrix, float minDepth, float maxDepth, float maxDepthDiff, const std::vector<int>* iterCounts, const std::vector<float>* minGradientMagnitudes, float maxPointsPart, int transformType, Result<cv::rgbd::RgbdOdometry*>* ocvrs_return) {
		try {
			cv::rgbd::RgbdOdometry* ret = new cv::rgbd::RgbdOdometry(*cameraMatrix, minDepth, maxDepth, maxDepthDiff, *iterCounts, *minGradientMagnitudes, maxPointsPart, transformType);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rgbd::RgbdOdometry::RgbdOdometry(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:673
	// ("cv::rgbd::RgbdOdometry::RgbdOdometry", vec![(pred!(mut, ["cameraMatrix"], ["const cv::Mat*"]), _)]),
	void cv_rgbd_RgbdOdometry_RgbdOdometry_const_MatR(const cv::Mat* cameraMatrix, Result<cv::rgbd::RgbdOdometry*>* ocvrs_return) {
		try {
			cv::rgbd::RgbdOdometry* ret = new cv::rgbd::RgbdOdometry(*cameraMatrix);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const Mat &, float, float, float, const std::vector<int> &, const std::vector<float> &, float, int)(TraitClass, Primitive, Primitive, Primitive, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:678
	// ("cv::rgbd::RgbdOdometry::create", vec![(pred!(mut, ["cameraMatrix", "minDepth", "maxDepth", "maxDepthDiff", "iterCounts", "minGradientMagnitudes", "maxPointsPart", "transformType"], ["const cv::Mat*", "float", "float", "float", "const std::vector<int>*", "const std::vector<float>*", "float", "int"]), _)]),
	void cv_rgbd_RgbdOdometry_create_const_MatR_float_float_float_const_vectorLintGR_const_vectorLfloatGR_float_int(const cv::Mat* cameraMatrix, float minDepth, float maxDepth, float maxDepthDiff, const std::vector<int>* iterCounts, const std::vector<float>* minGradientMagnitudes, float maxPointsPart, int transformType, Result<cv::Ptr<cv::rgbd::RgbdOdometry>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::rgbd::RgbdOdometry> ret = cv::rgbd::RgbdOdometry::create(*cameraMatrix, minDepth, maxDepth, maxDepthDiff, *iterCounts, *minGradientMagnitudes, maxPointsPart, transformType);
			Ok(new cv::Ptr<cv::rgbd::RgbdOdometry>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rgbd::RgbdOdometry::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:678
	// ("cv::rgbd::RgbdOdometry::create", vec![(pred!(mut, [], []), _)]),
	void cv_rgbd_RgbdOdometry_create(Result<cv::Ptr<cv::rgbd::RgbdOdometry>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::rgbd::RgbdOdometry> ret = cv::rgbd::RgbdOdometry::create();
			Ok(new cv::Ptr<cv::rgbd::RgbdOdometry>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// prepareFrameCache(Ptr<OdometryFrame> &, int)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:683
	// ("cv::rgbd::RgbdOdometry::prepareFrameCache", vec![(pred!(const, ["frame", "cacheType"], ["cv::Ptr<cv::rgbd::OdometryFrame>*", "int"]), _)]),
	void cv_rgbd_RgbdOdometry_prepareFrameCache_const_PtrLOdometryFrameGR_int(const cv::rgbd::RgbdOdometry* instance, cv::Ptr<cv::rgbd::OdometryFrame>* frame, int cacheType, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->prepareFrameCache(*frame, cacheType);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getCameraMatrix()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:685
	// ("cv::rgbd::RgbdOdometry::getCameraMatrix", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdOdometry_getCameraMatrix_const(const cv::rgbd::RgbdOdometry* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getCameraMatrix();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setCameraMatrix(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:689
	// ("cv::rgbd::RgbdOdometry::setCameraMatrix", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
	void cv_rgbd_RgbdOdometry_setCameraMatrix_const_MatR(cv::rgbd::RgbdOdometry* instance, const cv::Mat* val, ResultVoid* ocvrs_return) {
		try {
			instance->setCameraMatrix(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMinDepth()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:693
	// ("cv::rgbd::RgbdOdometry::getMinDepth", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdOdometry_getMinDepth_const(const cv::rgbd::RgbdOdometry* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMinDepth();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMinDepth(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:697
	// ("cv::rgbd::RgbdOdometry::setMinDepth", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_rgbd_RgbdOdometry_setMinDepth_double(cv::rgbd::RgbdOdometry* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setMinDepth(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxDepth()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:701
	// ("cv::rgbd::RgbdOdometry::getMaxDepth", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdOdometry_getMaxDepth_const(const cv::rgbd::RgbdOdometry* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxDepth();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxDepth(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:705
	// ("cv::rgbd::RgbdOdometry::setMaxDepth", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_rgbd_RgbdOdometry_setMaxDepth_double(cv::rgbd::RgbdOdometry* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxDepth(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxDepthDiff()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:709
	// ("cv::rgbd::RgbdOdometry::getMaxDepthDiff", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdOdometry_getMaxDepthDiff_const(const cv::rgbd::RgbdOdometry* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxDepthDiff();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxDepthDiff(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:713
	// ("cv::rgbd::RgbdOdometry::setMaxDepthDiff", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_rgbd_RgbdOdometry_setMaxDepthDiff_double(cv::rgbd::RgbdOdometry* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxDepthDiff(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getIterationCounts()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:717
	// ("cv::rgbd::RgbdOdometry::getIterationCounts", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdOdometry_getIterationCounts_const(const cv::rgbd::RgbdOdometry* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getIterationCounts();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setIterationCounts(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:721
	// ("cv::rgbd::RgbdOdometry::setIterationCounts", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
	void cv_rgbd_RgbdOdometry_setIterationCounts_const_MatR(cv::rgbd::RgbdOdometry* instance, const cv::Mat* val, ResultVoid* ocvrs_return) {
		try {
			instance->setIterationCounts(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMinGradientMagnitudes()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:725
	// ("cv::rgbd::RgbdOdometry::getMinGradientMagnitudes", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdOdometry_getMinGradientMagnitudes_const(const cv::rgbd::RgbdOdometry* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getMinGradientMagnitudes();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMinGradientMagnitudes(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:729
	// ("cv::rgbd::RgbdOdometry::setMinGradientMagnitudes", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
	void cv_rgbd_RgbdOdometry_setMinGradientMagnitudes_const_MatR(cv::rgbd::RgbdOdometry* instance, const cv::Mat* val, ResultVoid* ocvrs_return) {
		try {
			instance->setMinGradientMagnitudes(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxPointsPart()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:733
	// ("cv::rgbd::RgbdOdometry::getMaxPointsPart", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdOdometry_getMaxPointsPart_const(const cv::rgbd::RgbdOdometry* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxPointsPart();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxPointsPart(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:737
	// ("cv::rgbd::RgbdOdometry::setMaxPointsPart", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_rgbd_RgbdOdometry_setMaxPointsPart_double(cv::rgbd::RgbdOdometry* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxPointsPart(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTransformType()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:741
	// ("cv::rgbd::RgbdOdometry::getTransformType", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdOdometry_getTransformType_const(const cv::rgbd::RgbdOdometry* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getTransformType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTransformType(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:745
	// ("cv::rgbd::RgbdOdometry::setTransformType", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_rgbd_RgbdOdometry_setTransformType_int(cv::rgbd::RgbdOdometry* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setTransformType(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxTranslation()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:749
	// ("cv::rgbd::RgbdOdometry::getMaxTranslation", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdOdometry_getMaxTranslation_const(const cv::rgbd::RgbdOdometry* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxTranslation();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxTranslation(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:753
	// ("cv::rgbd::RgbdOdometry::setMaxTranslation", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_rgbd_RgbdOdometry_setMaxTranslation_double(cv::rgbd::RgbdOdometry* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxTranslation(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxRotation()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:757
	// ("cv::rgbd::RgbdOdometry::getMaxRotation", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdOdometry_getMaxRotation_const(const cv::rgbd::RgbdOdometry* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxRotation();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxRotation(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:761
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

	// RgbdPlane(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:370
	// ("cv::rgbd::RgbdPlane::RgbdPlane", vec![(pred!(mut, ["method"], ["int"]), _)]),
	void cv_rgbd_RgbdPlane_RgbdPlane_int(int method, Result<cv::rgbd::RgbdPlane*>* ocvrs_return) {
		try {
			cv::rgbd::RgbdPlane* ret = new cv::rgbd::RgbdPlane(method);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rgbd::RgbdPlane::RgbdPlane() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:370
	// ("cv::rgbd::RgbdPlane::RgbdPlane", vec![(pred!(mut, [], []), _)]),
	void cv_rgbd_RgbdPlane_RgbdPlane(Result<cv::rgbd::RgbdPlane*>* ocvrs_return) {
		try {
			cv::rgbd::RgbdPlane* ret = new cv::rgbd::RgbdPlane();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// RgbdPlane(int, int, int, double, double, double, double)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:391
	// ("cv::rgbd::RgbdPlane::RgbdPlane", vec![(pred!(mut, ["method", "block_size", "min_size", "threshold", "sensor_error_a", "sensor_error_b", "sensor_error_c"], ["int", "int", "int", "double", "double", "double", "double"]), _)]),
	void cv_rgbd_RgbdPlane_RgbdPlane_int_int_int_double_double_double_double(int method, int block_size, int min_size, double threshold, double sensor_error_a, double sensor_error_b, double sensor_error_c, Result<cv::rgbd::RgbdPlane*>* ocvrs_return) {
		try {
			cv::rgbd::RgbdPlane* ret = new cv::rgbd::RgbdPlane(method, block_size, min_size, threshold, sensor_error_a, sensor_error_b, sensor_error_c);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rgbd::RgbdPlane::RgbdPlane(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:391
	// ("cv::rgbd::RgbdPlane::RgbdPlane", vec![(pred!(mut, ["method", "block_size", "min_size", "threshold"], ["int", "int", "int", "double"]), _)]),
	void cv_rgbd_RgbdPlane_RgbdPlane_int_int_int_double(int method, int block_size, int min_size, double threshold, Result<cv::rgbd::RgbdPlane*>* ocvrs_return) {
		try {
			cv::rgbd::RgbdPlane* ret = new cv::rgbd::RgbdPlane(method, block_size, min_size, threshold);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(int, int, int, double, double, double, double)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:397
	// ("cv::rgbd::RgbdPlane::create", vec![(pred!(mut, ["method", "block_size", "min_size", "threshold", "sensor_error_a", "sensor_error_b", "sensor_error_c"], ["int", "int", "int", "double", "double", "double", "double"]), _)]),
	void cv_rgbd_RgbdPlane_create_int_int_int_double_double_double_double(int method, int block_size, int min_size, double threshold, double sensor_error_a, double sensor_error_b, double sensor_error_c, Result<cv::Ptr<cv::rgbd::RgbdPlane>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::rgbd::RgbdPlane> ret = cv::rgbd::RgbdPlane::create(method, block_size, min_size, threshold, sensor_error_a, sensor_error_b, sensor_error_c);
			Ok(new cv::Ptr<cv::rgbd::RgbdPlane>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::rgbd::RgbdPlane::create(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:397
	// ("cv::rgbd::RgbdPlane::create", vec![(pred!(mut, ["method", "block_size", "min_size", "threshold"], ["int", "int", "int", "double"]), _)]),
	void cv_rgbd_RgbdPlane_create_int_int_int_double(int method, int block_size, int min_size, double threshold, Result<cv::Ptr<cv::rgbd::RgbdPlane>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::rgbd::RgbdPlane> ret = cv::rgbd::RgbdPlane::create(method, block_size, min_size, threshold);
			Ok(new cv::Ptr<cv::rgbd::RgbdPlane>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator()(InputArray, InputArray, OutputArray, OutputArray)(InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:410
	// ("cv::rgbd::RgbdPlane::operator()", vec![(pred!(mut, ["points3d", "normals", "mask", "plane_coefficients"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_rgbd_RgbdPlane_operator___const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::rgbd::RgbdPlane* instance, const cv::_InputArray* points3d, const cv::_InputArray* normals, const cv::_OutputArray* mask, const cv::_OutputArray* plane_coefficients, ResultVoid* ocvrs_return) {
		try {
			instance->operator()(*points3d, *normals, *mask, *plane_coefficients);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator()(InputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:420
	// ("cv::rgbd::RgbdPlane::operator()", vec![(pred!(mut, ["points3d", "mask", "plane_coefficients"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_rgbd_RgbdPlane_operator___const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::rgbd::RgbdPlane* instance, const cv::_InputArray* points3d, const cv::_OutputArray* mask, const cv::_OutputArray* plane_coefficients, ResultVoid* ocvrs_return) {
		try {
			instance->operator()(*points3d, *mask, *plane_coefficients);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getBlockSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:422
	// ("cv::rgbd::RgbdPlane::getBlockSize", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdPlane_getBlockSize_const(const cv::rgbd::RgbdPlane* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getBlockSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setBlockSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:426
	// ("cv::rgbd::RgbdPlane::setBlockSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_rgbd_RgbdPlane_setBlockSize_int(cv::rgbd::RgbdPlane* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setBlockSize(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMinSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:430
	// ("cv::rgbd::RgbdPlane::getMinSize", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdPlane_getMinSize_const(const cv::rgbd::RgbdPlane* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMinSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMinSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:434
	// ("cv::rgbd::RgbdPlane::setMinSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_rgbd_RgbdPlane_setMinSize_int(cv::rgbd::RgbdPlane* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setMinSize(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMethod()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:438
	// ("cv::rgbd::RgbdPlane::getMethod", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdPlane_getMethod_const(const cv::rgbd::RgbdPlane* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMethod();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMethod(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:442
	// ("cv::rgbd::RgbdPlane::setMethod", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_rgbd_RgbdPlane_setMethod_int(cv::rgbd::RgbdPlane* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setMethod(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getThreshold()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:446
	// ("cv::rgbd::RgbdPlane::getThreshold", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdPlane_getThreshold_const(const cv::rgbd::RgbdPlane* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:450
	// ("cv::rgbd::RgbdPlane::setThreshold", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_rgbd_RgbdPlane_setThreshold_double(cv::rgbd::RgbdPlane* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setThreshold(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSensorErrorA()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:454
	// ("cv::rgbd::RgbdPlane::getSensorErrorA", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdPlane_getSensorErrorA_const(const cv::rgbd::RgbdPlane* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getSensorErrorA();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSensorErrorA(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:458
	// ("cv::rgbd::RgbdPlane::setSensorErrorA", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_rgbd_RgbdPlane_setSensorErrorA_double(cv::rgbd::RgbdPlane* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setSensorErrorA(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSensorErrorB()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:462
	// ("cv::rgbd::RgbdPlane::getSensorErrorB", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdPlane_getSensorErrorB_const(const cv::rgbd::RgbdPlane* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getSensorErrorB();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSensorErrorB(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:466
	// ("cv::rgbd::RgbdPlane::setSensorErrorB", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_rgbd_RgbdPlane_setSensorErrorB_double(cv::rgbd::RgbdPlane* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setSensorErrorB(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSensorErrorC()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:470
	// ("cv::rgbd::RgbdPlane::getSensorErrorC", vec![(pred!(const, [], []), _)]),
	void cv_rgbd_RgbdPlane_getSensorErrorC_const(const cv::rgbd::RgbdPlane* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getSensorErrorC();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSensorErrorC(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/rgbd.hpp:474
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
