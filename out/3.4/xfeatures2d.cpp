#include "xfeatures2d.hpp"
#include "xfeatures2d_types.hpp"

extern "C" {
	// cv::xfeatures2d::FASTForPointSet(InputArray, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:963
	// ("cv::xfeatures2d::FASTForPointSet", vec![(pred!(mut, ["image", "keypoints", "threshold"], ["const cv::_InputArray*", "std::vector<cv::KeyPoint>*", "int"]), _)]),
	void cv_xfeatures2d_FASTForPointSet_const__InputArrayR_vectorLKeyPointGR_int(const cv::_InputArray* image, std::vector<cv::KeyPoint>* keypoints, int threshold, ResultVoid* ocvrs_return) {
		try {
			cv::xfeatures2d::FASTForPointSet(*image, *keypoints, threshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// FASTForPointSet(InputArray, std::vector<KeyPoint> &, int, bool, int)(InputArray, CppPassByVoidPtr, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:963
	// ("cv::xfeatures2d::FASTForPointSet", vec![(pred!(mut, ["image", "keypoints", "threshold", "nonmaxSuppression", "type"], ["const cv::_InputArray*", "std::vector<cv::KeyPoint>*", "int", "bool", "int"]), _)]),
	void cv_xfeatures2d_FASTForPointSet_const__InputArrayR_vectorLKeyPointGR_int_bool_int(const cv::_InputArray* image, std::vector<cv::KeyPoint>* keypoints, int threshold, bool nonmaxSuppression, int type, ResultVoid* ocvrs_return) {
		try {
			cv::xfeatures2d::FASTForPointSet(*image, *keypoints, threshold, nonmaxSuppression, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::matchGMS(SimpleClass, SimpleClass, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:989
	// ("cv::xfeatures2d::matchGMS", vec![(pred!(mut, ["size1", "size2", "keypoints1", "keypoints2", "matches1to2", "matchesGMS"], ["const cv::Size*", "const cv::Size*", "const std::vector<cv::KeyPoint>*", "const std::vector<cv::KeyPoint>*", "const std::vector<cv::DMatch>*", "std::vector<cv::DMatch>*"]), _)]),
	void cv_xfeatures2d_matchGMS_const_SizeR_const_SizeR_const_vectorLKeyPointGR_const_vectorLKeyPointGR_const_vectorLDMatchGR_vectorLDMatchGR(const cv::Size* size1, const cv::Size* size2, const std::vector<cv::KeyPoint>* keypoints1, const std::vector<cv::KeyPoint>* keypoints2, const std::vector<cv::DMatch>* matches1to2, std::vector<cv::DMatch>* matchesGMS, ResultVoid* ocvrs_return) {
		try {
			cv::xfeatures2d::matchGMS(*size1, *size2, *keypoints1, *keypoints2, *matches1to2, *matchesGMS);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// matchGMS(const Size &, const Size &, const std::vector<KeyPoint> &, const std::vector<KeyPoint> &, const std::vector<DMatch> &, std::vector<DMatch> &, const bool, const bool, const double)(SimpleClass, SimpleClass, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:989
	// ("cv::xfeatures2d::matchGMS", vec![(pred!(mut, ["size1", "size2", "keypoints1", "keypoints2", "matches1to2", "matchesGMS", "withRotation", "withScale", "thresholdFactor"], ["const cv::Size*", "const cv::Size*", "const std::vector<cv::KeyPoint>*", "const std::vector<cv::KeyPoint>*", "const std::vector<cv::DMatch>*", "std::vector<cv::DMatch>*", "const bool", "const bool", "const double"]), _)]),
	void cv_xfeatures2d_matchGMS_const_SizeR_const_SizeR_const_vectorLKeyPointGR_const_vectorLKeyPointGR_const_vectorLDMatchGR_vectorLDMatchGR_const_bool_const_bool_const_double(const cv::Size* size1, const cv::Size* size2, const std::vector<cv::KeyPoint>* keypoints1, const std::vector<cv::KeyPoint>* keypoints2, const std::vector<cv::DMatch>* matches1to2, std::vector<cv::DMatch>* matchesGMS, const bool withRotation, const bool withScale, const double thresholdFactor, ResultVoid* ocvrs_return) {
		try {
			cv::xfeatures2d::matchGMS(*size1, *size2, *keypoints1, *keypoints2, *matches1to2, *matchesGMS, withRotation, withScale, thresholdFactor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// SURF_CUDA()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:102
	// ("cv::cuda::SURF_CUDA::SURF_CUDA", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_SURF_CUDA_SURF_CUDA(Result<cv::cuda::SURF_CUDA*>* ocvrs_return) {
		try {
			cv::cuda::SURF_CUDA* ret = new cv::cuda::SURF_CUDA();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// SURF_CUDA(double, int, int, bool, float, bool)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:104
	// ("cv::cuda::SURF_CUDA::SURF_CUDA", vec![(pred!(mut, ["_hessianThreshold", "_nOctaves", "_nOctaveLayers", "_extended", "_keypointsRatio", "_upright"], ["double", "int", "int", "bool", "float", "bool"]), _)]),
	void cv_cuda_SURF_CUDA_SURF_CUDA_double_int_int_bool_float_bool(double _hessianThreshold, int _nOctaves, int _nOctaveLayers, bool _extended, float _keypointsRatio, bool _upright, Result<cv::cuda::SURF_CUDA*>* ocvrs_return) {
		try {
			cv::cuda::SURF_CUDA* ret = new cv::cuda::SURF_CUDA(_hessianThreshold, _nOctaves, _nOctaveLayers, _extended, _keypointsRatio, _upright);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::SURF_CUDA::SURF_CUDA(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:104
	// ("cv::cuda::SURF_CUDA::SURF_CUDA", vec![(pred!(mut, ["_hessianThreshold"], ["double"]), _)]),
	void cv_cuda_SURF_CUDA_SURF_CUDA_double(double _hessianThreshold, Result<cv::cuda::SURF_CUDA*>* ocvrs_return) {
		try {
			cv::cuda::SURF_CUDA* ret = new cv::cuda::SURF_CUDA(_hessianThreshold);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// descriptorSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:108
	// ("cv::cuda::SURF_CUDA::descriptorSize", vec![(pred!(const, [], []), _)]),
	void cv_cuda_SURF_CUDA_descriptorSize_const(const cv::cuda::SURF_CUDA* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->descriptorSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// defaultNorm()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:110
	// ("cv::cuda::SURF_CUDA::defaultNorm", vec![(pred!(const, [], []), _)]),
	void cv_cuda_SURF_CUDA_defaultNorm_const(const cv::cuda::SURF_CUDA* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->defaultNorm();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// uploadKeypoints(const std::vector<KeyPoint> &, GpuMat &)(CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:113
	// ("cv::cuda::SURF_CUDA::uploadKeypoints", vec![(pred!(mut, ["keypoints", "keypointsGPU"], ["const std::vector<cv::KeyPoint>*", "cv::cuda::GpuMat*"]), _)]),
	void cv_cuda_SURF_CUDA_uploadKeypoints_const_vectorLKeyPointGR_GpuMatR(cv::cuda::SURF_CUDA* instance, const std::vector<cv::KeyPoint>* keypoints, cv::cuda::GpuMat* keypointsGPU, ResultVoid* ocvrs_return) {
		try {
			instance->uploadKeypoints(*keypoints, *keypointsGPU);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// downloadKeypoints(const GpuMat &, std::vector<KeyPoint> &)(TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:115
	// ("cv::cuda::SURF_CUDA::downloadKeypoints", vec![(pred!(mut, ["keypointsGPU", "keypoints"], ["const cv::cuda::GpuMat*", "std::vector<cv::KeyPoint>*"]), _)]),
	void cv_cuda_SURF_CUDA_downloadKeypoints_const_GpuMatR_vectorLKeyPointGR(cv::cuda::SURF_CUDA* instance, const cv::cuda::GpuMat* keypointsGPU, std::vector<cv::KeyPoint>* keypoints, ResultVoid* ocvrs_return) {
		try {
			instance->downloadKeypoints(*keypointsGPU, *keypoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// downloadDescriptors(const GpuMat &, std::vector<float> &)(TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:118
	// ("cv::cuda::SURF_CUDA::downloadDescriptors", vec![(pred!(mut, ["descriptorsGPU", "descriptors"], ["const cv::cuda::GpuMat*", "std::vector<float>*"]), _)]),
	void cv_cuda_SURF_CUDA_downloadDescriptors_const_GpuMatR_vectorLfloatGR(cv::cuda::SURF_CUDA* instance, const cv::cuda::GpuMat* descriptorsGPU, std::vector<float>* descriptors, ResultVoid* ocvrs_return) {
		try {
			instance->downloadDescriptors(*descriptorsGPU, *descriptors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator()(const GpuMat &, const GpuMat &, GpuMat &)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:130
	// ("cv::cuda::SURF_CUDA::operator()", vec![(pred!(mut, ["img", "mask", "keypoints"], ["const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "cv::cuda::GpuMat*"]), _)]),
	void cv_cuda_SURF_CUDA_operator___const_GpuMatR_const_GpuMatR_GpuMatR(cv::cuda::SURF_CUDA* instance, const cv::cuda::GpuMat* img, const cv::cuda::GpuMat* mask, cv::cuda::GpuMat* keypoints, ResultVoid* ocvrs_return) {
		try {
			instance->operator()(*img, *mask, *keypoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator()(const GpuMat &, const GpuMat &, GpuMat &, GpuMat &, bool)(TraitClass, TraitClass, TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:133
	// ("cv::cuda::SURF_CUDA::operator()", vec![(pred!(mut, ["img", "mask", "keypoints", "descriptors", "useProvidedKeypoints"], ["const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "bool"]), _)]),
	void cv_cuda_SURF_CUDA_operator___const_GpuMatR_const_GpuMatR_GpuMatR_GpuMatR_bool(cv::cuda::SURF_CUDA* instance, const cv::cuda::GpuMat* img, const cv::cuda::GpuMat* mask, cv::cuda::GpuMat* keypoints, cv::cuda::GpuMat* descriptors, bool useProvidedKeypoints, ResultVoid* ocvrs_return) {
		try {
			instance->operator()(*img, *mask, *keypoints, *descriptors, useProvidedKeypoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::SURF_CUDA::operator()(TraitClass, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:133
	// ("cv::cuda::SURF_CUDA::operator()", vec![(pred!(mut, ["img", "mask", "keypoints", "descriptors"], ["const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*"]), _)]),
	void cv_cuda_SURF_CUDA_operator___const_GpuMatR_const_GpuMatR_GpuMatR_GpuMatR(cv::cuda::SURF_CUDA* instance, const cv::cuda::GpuMat* img, const cv::cuda::GpuMat* mask, cv::cuda::GpuMat* keypoints, cv::cuda::GpuMat* descriptors, ResultVoid* ocvrs_return) {
		try {
			instance->operator()(*img, *mask, *keypoints, *descriptors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator()(const GpuMat &, const GpuMat &, std::vector<KeyPoint> &)(TraitClass, TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:136
	// ("cv::cuda::SURF_CUDA::operator()", vec![(pred!(mut, ["img", "mask", "keypoints"], ["const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "std::vector<cv::KeyPoint>*"]), _)]),
	void cv_cuda_SURF_CUDA_operator___const_GpuMatR_const_GpuMatR_vectorLKeyPointGR(cv::cuda::SURF_CUDA* instance, const cv::cuda::GpuMat* img, const cv::cuda::GpuMat* mask, std::vector<cv::KeyPoint>* keypoints, ResultVoid* ocvrs_return) {
		try {
			instance->operator()(*img, *mask, *keypoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator()(const GpuMat &, const GpuMat &, std::vector<KeyPoint> &, GpuMat &, bool)(TraitClass, TraitClass, CppPassByVoidPtr, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:137
	// ("cv::cuda::SURF_CUDA::operator()", vec![(pred!(mut, ["img", "mask", "keypoints", "descriptors", "useProvidedKeypoints"], ["const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "std::vector<cv::KeyPoint>*", "cv::cuda::GpuMat*", "bool"]), _)]),
	void cv_cuda_SURF_CUDA_operator___const_GpuMatR_const_GpuMatR_vectorLKeyPointGR_GpuMatR_bool(cv::cuda::SURF_CUDA* instance, const cv::cuda::GpuMat* img, const cv::cuda::GpuMat* mask, std::vector<cv::KeyPoint>* keypoints, cv::cuda::GpuMat* descriptors, bool useProvidedKeypoints, ResultVoid* ocvrs_return) {
		try {
			instance->operator()(*img, *mask, *keypoints, *descriptors, useProvidedKeypoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::SURF_CUDA::operator()(TraitClass, TraitClass, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:137
	// ("cv::cuda::SURF_CUDA::operator()", vec![(pred!(mut, ["img", "mask", "keypoints", "descriptors"], ["const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "std::vector<cv::KeyPoint>*", "cv::cuda::GpuMat*"]), _)]),
	void cv_cuda_SURF_CUDA_operator___const_GpuMatR_const_GpuMatR_vectorLKeyPointGR_GpuMatR(cv::cuda::SURF_CUDA* instance, const cv::cuda::GpuMat* img, const cv::cuda::GpuMat* mask, std::vector<cv::KeyPoint>* keypoints, cv::cuda::GpuMat* descriptors, ResultVoid* ocvrs_return) {
		try {
			instance->operator()(*img, *mask, *keypoints, *descriptors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator()(const GpuMat &, const GpuMat &, std::vector<KeyPoint> &, std::vector<float> &, bool)(TraitClass, TraitClass, CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:140
	// ("cv::cuda::SURF_CUDA::operator()", vec![(pred!(mut, ["img", "mask", "keypoints", "descriptors", "useProvidedKeypoints"], ["const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "std::vector<cv::KeyPoint>*", "std::vector<float>*", "bool"]), _)]),
	void cv_cuda_SURF_CUDA_operator___const_GpuMatR_const_GpuMatR_vectorLKeyPointGR_vectorLfloatGR_bool(cv::cuda::SURF_CUDA* instance, const cv::cuda::GpuMat* img, const cv::cuda::GpuMat* mask, std::vector<cv::KeyPoint>* keypoints, std::vector<float>* descriptors, bool useProvidedKeypoints, ResultVoid* ocvrs_return) {
		try {
			instance->operator()(*img, *mask, *keypoints, *descriptors, useProvidedKeypoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::SURF_CUDA::operator()(TraitClass, TraitClass, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:140
	// ("cv::cuda::SURF_CUDA::operator()", vec![(pred!(mut, ["img", "mask", "keypoints", "descriptors"], ["const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "std::vector<cv::KeyPoint>*", "std::vector<float>*"]), _)]),
	void cv_cuda_SURF_CUDA_operator___const_GpuMatR_const_GpuMatR_vectorLKeyPointGR_vectorLfloatGR(cv::cuda::SURF_CUDA* instance, const cv::cuda::GpuMat* img, const cv::cuda::GpuMat* mask, std::vector<cv::KeyPoint>* keypoints, std::vector<float>* descriptors, ResultVoid* ocvrs_return) {
		try {
			instance->operator()(*img, *mask, *keypoints, *descriptors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// releaseMemory()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:143
	// ("cv::cuda::SURF_CUDA::releaseMemory", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_SURF_CUDA_releaseMemory(cv::cuda::SURF_CUDA* instance, ResultVoid* ocvrs_return) {
		try {
			instance->releaseMemory();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::SURF_CUDA::hessianThreshold() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:146
	// ("cv::cuda::SURF_CUDA::hessianThreshold", vec![(pred!(const, [], []), _)]),
	double cv_cuda_SURF_CUDA_propHessianThreshold_const(const cv::cuda::SURF_CUDA* instance) {
			double ret = instance->hessianThreshold;
			return ret;
	}

	// cv::cuda::SURF_CUDA::setHessianThreshold(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:146
	// ("cv::cuda::SURF_CUDA::setHessianThreshold", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_cuda_SURF_CUDA_propHessianThreshold_const_double(cv::cuda::SURF_CUDA* instance, const double val) {
			instance->hessianThreshold = val;
	}

	// cv::cuda::SURF_CUDA::nOctaves() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:147
	// ("cv::cuda::SURF_CUDA::nOctaves", vec![(pred!(const, [], []), _)]),
	int cv_cuda_SURF_CUDA_propNOctaves_const(const cv::cuda::SURF_CUDA* instance) {
			int ret = instance->nOctaves;
			return ret;
	}

	// cv::cuda::SURF_CUDA::setNOctaves(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:147
	// ("cv::cuda::SURF_CUDA::setNOctaves", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_cuda_SURF_CUDA_propNOctaves_const_int(cv::cuda::SURF_CUDA* instance, const int val) {
			instance->nOctaves = val;
	}

	// cv::cuda::SURF_CUDA::nOctaveLayers() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:148
	// ("cv::cuda::SURF_CUDA::nOctaveLayers", vec![(pred!(const, [], []), _)]),
	int cv_cuda_SURF_CUDA_propNOctaveLayers_const(const cv::cuda::SURF_CUDA* instance) {
			int ret = instance->nOctaveLayers;
			return ret;
	}

	// cv::cuda::SURF_CUDA::setNOctaveLayers(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:148
	// ("cv::cuda::SURF_CUDA::setNOctaveLayers", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_cuda_SURF_CUDA_propNOctaveLayers_const_int(cv::cuda::SURF_CUDA* instance, const int val) {
			instance->nOctaveLayers = val;
	}

	// cv::cuda::SURF_CUDA::extended() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:149
	// ("cv::cuda::SURF_CUDA::extended", vec![(pred!(const, [], []), _)]),
	bool cv_cuda_SURF_CUDA_propExtended_const(const cv::cuda::SURF_CUDA* instance) {
			bool ret = instance->extended;
			return ret;
	}

	// cv::cuda::SURF_CUDA::setExtended(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:149
	// ("cv::cuda::SURF_CUDA::setExtended", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_cuda_SURF_CUDA_propExtended_const_bool(cv::cuda::SURF_CUDA* instance, const bool val) {
			instance->extended = val;
	}

	// cv::cuda::SURF_CUDA::upright() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:150
	// ("cv::cuda::SURF_CUDA::upright", vec![(pred!(const, [], []), _)]),
	bool cv_cuda_SURF_CUDA_propUpright_const(const cv::cuda::SURF_CUDA* instance) {
			bool ret = instance->upright;
			return ret;
	}

	// cv::cuda::SURF_CUDA::setUpright(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:150
	// ("cv::cuda::SURF_CUDA::setUpright", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_cuda_SURF_CUDA_propUpright_const_bool(cv::cuda::SURF_CUDA* instance, const bool val) {
			instance->upright = val;
	}

	// cv::cuda::SURF_CUDA::keypointsRatio() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:153
	// ("cv::cuda::SURF_CUDA::keypointsRatio", vec![(pred!(const, [], []), _)]),
	float cv_cuda_SURF_CUDA_propKeypointsRatio_const(const cv::cuda::SURF_CUDA* instance) {
			float ret = instance->keypointsRatio;
			return ret;
	}

	// cv::cuda::SURF_CUDA::setKeypointsRatio(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:153
	// ("cv::cuda::SURF_CUDA::setKeypointsRatio", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_cuda_SURF_CUDA_propKeypointsRatio_const_float(cv::cuda::SURF_CUDA* instance, const float val) {
			instance->keypointsRatio = val;
	}

	// cv::cuda::SURF_CUDA::sum() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:155
	// ("cv::cuda::SURF_CUDA::sum", vec![(pred!(const, [], []), _)]),
	cv::cuda::GpuMat* cv_cuda_SURF_CUDA_propSum_const(const cv::cuda::SURF_CUDA* instance) {
			cv::cuda::GpuMat ret = instance->sum;
			return new cv::cuda::GpuMat(ret);
	}

	// cv::cuda::SURF_CUDA::setSum(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:155
	// ("cv::cuda::SURF_CUDA::setSum", vec![(pred!(mut, ["val"], ["const cv::cuda::GpuMat"]), _)]),
	void cv_cuda_SURF_CUDA_propSum_const_GpuMat(cv::cuda::SURF_CUDA* instance, const cv::cuda::GpuMat* val) {
			instance->sum = *val;
	}

	// cv::cuda::SURF_CUDA::mask1() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:155
	// ("cv::cuda::SURF_CUDA::mask1", vec![(pred!(const, [], []), _)]),
	cv::cuda::GpuMat* cv_cuda_SURF_CUDA_propMask1_const(const cv::cuda::SURF_CUDA* instance) {
			cv::cuda::GpuMat ret = instance->mask1;
			return new cv::cuda::GpuMat(ret);
	}

	// cv::cuda::SURF_CUDA::setMask1(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:155
	// ("cv::cuda::SURF_CUDA::setMask1", vec![(pred!(mut, ["val"], ["const cv::cuda::GpuMat"]), _)]),
	void cv_cuda_SURF_CUDA_propMask1_const_GpuMat(cv::cuda::SURF_CUDA* instance, const cv::cuda::GpuMat* val) {
			instance->mask1 = *val;
	}

	// cv::cuda::SURF_CUDA::maskSum() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:155
	// ("cv::cuda::SURF_CUDA::maskSum", vec![(pred!(const, [], []), _)]),
	cv::cuda::GpuMat* cv_cuda_SURF_CUDA_propMaskSum_const(const cv::cuda::SURF_CUDA* instance) {
			cv::cuda::GpuMat ret = instance->maskSum;
			return new cv::cuda::GpuMat(ret);
	}

	// cv::cuda::SURF_CUDA::setMaskSum(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:155
	// ("cv::cuda::SURF_CUDA::setMaskSum", vec![(pred!(mut, ["val"], ["const cv::cuda::GpuMat"]), _)]),
	void cv_cuda_SURF_CUDA_propMaskSum_const_GpuMat(cv::cuda::SURF_CUDA* instance, const cv::cuda::GpuMat* val) {
			instance->maskSum = *val;
	}

	// cv::cuda::SURF_CUDA::det() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:157
	// ("cv::cuda::SURF_CUDA::det", vec![(pred!(const, [], []), _)]),
	cv::cuda::GpuMat* cv_cuda_SURF_CUDA_propDet_const(const cv::cuda::SURF_CUDA* instance) {
			cv::cuda::GpuMat ret = instance->det;
			return new cv::cuda::GpuMat(ret);
	}

	// cv::cuda::SURF_CUDA::setDet(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:157
	// ("cv::cuda::SURF_CUDA::setDet", vec![(pred!(mut, ["val"], ["const cv::cuda::GpuMat"]), _)]),
	void cv_cuda_SURF_CUDA_propDet_const_GpuMat(cv::cuda::SURF_CUDA* instance, const cv::cuda::GpuMat* val) {
			instance->det = *val;
	}

	// cv::cuda::SURF_CUDA::trace() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:157
	// ("cv::cuda::SURF_CUDA::trace", vec![(pred!(const, [], []), _)]),
	cv::cuda::GpuMat* cv_cuda_SURF_CUDA_propTrace_const(const cv::cuda::SURF_CUDA* instance) {
			cv::cuda::GpuMat ret = instance->trace;
			return new cv::cuda::GpuMat(ret);
	}

	// cv::cuda::SURF_CUDA::setTrace(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:157
	// ("cv::cuda::SURF_CUDA::setTrace", vec![(pred!(mut, ["val"], ["const cv::cuda::GpuMat"]), _)]),
	void cv_cuda_SURF_CUDA_propTrace_const_GpuMat(cv::cuda::SURF_CUDA* instance, const cv::cuda::GpuMat* val) {
			instance->trace = *val;
	}

	// cv::cuda::SURF_CUDA::maxPosBuffer() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:159
	// ("cv::cuda::SURF_CUDA::maxPosBuffer", vec![(pred!(const, [], []), _)]),
	cv::cuda::GpuMat* cv_cuda_SURF_CUDA_propMaxPosBuffer_const(const cv::cuda::SURF_CUDA* instance) {
			cv::cuda::GpuMat ret = instance->maxPosBuffer;
			return new cv::cuda::GpuMat(ret);
	}

	// cv::cuda::SURF_CUDA::setMaxPosBuffer(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/cuda.hpp:159
	// ("cv::cuda::SURF_CUDA::setMaxPosBuffer", vec![(pred!(mut, ["val"], ["const cv::cuda::GpuMat"]), _)]),
	void cv_cuda_SURF_CUDA_propMaxPosBuffer_const_GpuMat(cv::cuda::SURF_CUDA* instance, const cv::cuda::GpuMat* val) {
			instance->maxPosBuffer = *val;
	}

	// cv::cuda::SURF_CUDA::delete() generated
	// ("cv::cuda::SURF_CUDA::delete", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_SURF_CUDA_delete(cv::cuda::SURF_CUDA* instance) {
			delete instance;
	}

	// create(Ptr<FeatureDetector>, Ptr<DescriptorExtractor>)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:910
	// ("cv::xfeatures2d::AffineFeature2D::create", vec![(pred!(mut, ["keypoint_detector", "descriptor_extractor"], ["cv::Ptr<cv::Feature2D>", "cv::Ptr<cv::Feature2D>"]), _)]),
	void cv_xfeatures2d_AffineFeature2D_create_PtrLFeature2DG_PtrLFeature2DG(cv::Ptr<cv::Feature2D>* keypoint_detector, cv::Ptr<cv::Feature2D>* descriptor_extractor, Result<cv::Ptr<cv::xfeatures2d::AffineFeature2D>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::AffineFeature2D> ret = cv::xfeatures2d::AffineFeature2D::create(*keypoint_detector, *descriptor_extractor);
			Ok(new cv::Ptr<cv::xfeatures2d::AffineFeature2D>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(Ptr<FeatureDetector>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:918
	// ("cv::xfeatures2d::AffineFeature2D::create", vec![(pred!(mut, ["keypoint_detector"], ["cv::Ptr<cv::Feature2D>"]), _)]),
	void cv_xfeatures2d_AffineFeature2D_create_PtrLFeature2DG(cv::Ptr<cv::Feature2D>* keypoint_detector, Result<cv::Ptr<cv::xfeatures2d::AffineFeature2D>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::AffineFeature2D> ret = cv::xfeatures2d::AffineFeature2D::create(*keypoint_detector);
			Ok(new cv::Ptr<cv::xfeatures2d::AffineFeature2D>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detect(InputArray, std::vector<Elliptic_KeyPoint> &, InputArray)(InputArray, CppPassByVoidPtr, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:929
	// ("cv::xfeatures2d::AffineFeature2D::detect", vec![(pred!(mut, ["image", "keypoints", "mask"], ["const cv::_InputArray*", "std::vector<cv::xfeatures2d::Elliptic_KeyPoint>*", "const cv::_InputArray*"]), _)]),
	void cv_xfeatures2d_AffineFeature2D_detect_const__InputArrayR_vectorLElliptic_KeyPointGR_const__InputArrayR(cv::xfeatures2d::AffineFeature2D* instance, const cv::_InputArray* image, std::vector<cv::xfeatures2d::Elliptic_KeyPoint>* keypoints, const cv::_InputArray* mask, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*image, *keypoints, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::AffineFeature2D::detect(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:929
	// ("cv::xfeatures2d::AffineFeature2D::detect", vec![(pred!(mut, ["image", "keypoints"], ["const cv::_InputArray*", "std::vector<cv::xfeatures2d::Elliptic_KeyPoint>*"]), _)]),
	void cv_xfeatures2d_AffineFeature2D_detect_const__InputArrayR_vectorLElliptic_KeyPointGR(cv::xfeatures2d::AffineFeature2D* instance, const cv::_InputArray* image, std::vector<cv::xfeatures2d::Elliptic_KeyPoint>* keypoints, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*image, *keypoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectAndCompute(InputArray, InputArray, std::vector<Elliptic_KeyPoint> &, OutputArray, bool)(InputArray, InputArray, CppPassByVoidPtr, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:939
	// ("cv::xfeatures2d::AffineFeature2D::detectAndCompute", vec![(pred!(mut, ["image", "mask", "keypoints", "descriptors", "useProvidedKeypoints"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<cv::xfeatures2d::Elliptic_KeyPoint>*", "const cv::_OutputArray*", "bool"]), _)]),
	void cv_xfeatures2d_AffineFeature2D_detectAndCompute_const__InputArrayR_const__InputArrayR_vectorLElliptic_KeyPointGR_const__OutputArrayR_bool(cv::xfeatures2d::AffineFeature2D* instance, const cv::_InputArray* image, const cv::_InputArray* mask, std::vector<cv::xfeatures2d::Elliptic_KeyPoint>* keypoints, const cv::_OutputArray* descriptors, bool useProvidedKeypoints, ResultVoid* ocvrs_return) {
		try {
			instance->detectAndCompute(*image, *mask, *keypoints, *descriptors, useProvidedKeypoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::AffineFeature2D::detectAndCompute(InputArray, InputArray, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:939
	// ("cv::xfeatures2d::AffineFeature2D::detectAndCompute", vec![(pred!(mut, ["image", "mask", "keypoints", "descriptors"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<cv::xfeatures2d::Elliptic_KeyPoint>*", "const cv::_OutputArray*"]), _)]),
	void cv_xfeatures2d_AffineFeature2D_detectAndCompute_const__InputArrayR_const__InputArrayR_vectorLElliptic_KeyPointGR_const__OutputArrayR(cv::xfeatures2d::AffineFeature2D* instance, const cv::_InputArray* image, const cv::_InputArray* mask, std::vector<cv::xfeatures2d::Elliptic_KeyPoint>* keypoints, const cv::_OutputArray* descriptors, ResultVoid* ocvrs_return) {
		try {
			instance->detectAndCompute(*image, *mask, *keypoints, *descriptors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::AffineFeature2D::to_Algorithm() generated
	// ("cv::xfeatures2d::AffineFeature2D::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_xfeatures2d_AffineFeature2D_to_Algorithm(cv::xfeatures2d::AffineFeature2D* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::xfeatures2d::AffineFeature2D::to_Feature2D() generated
	// ("cv::xfeatures2d::AffineFeature2D::to_Feature2D", vec![(pred!(mut, [], []), _)]),
	cv::Feature2D* cv_xfeatures2d_AffineFeature2D_to_Feature2D(cv::xfeatures2d::AffineFeature2D* instance) {
			return dynamic_cast<cv::Feature2D*>(instance);
	}

	// cv::xfeatures2d::AffineFeature2D::delete() generated
	// ("cv::xfeatures2d::AffineFeature2D::delete", vec![(pred!(mut, [], []), _)]),
	void cv_xfeatures2d_AffineFeature2D_delete(cv::xfeatures2d::AffineFeature2D* instance) {
			delete instance;
	}

	// create(int, bool, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:373
	// ("cv::xfeatures2d::BoostDesc::create", vec![(pred!(mut, ["desc", "use_scale_orientation", "scale_factor"], ["int", "bool", "float"]), _)]),
	void cv_xfeatures2d_BoostDesc_create_int_bool_float(int desc, bool use_scale_orientation, float scale_factor, Result<cv::Ptr<cv::xfeatures2d::BoostDesc>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::BoostDesc> ret = cv::xfeatures2d::BoostDesc::create(desc, use_scale_orientation, scale_factor);
			Ok(new cv::Ptr<cv::xfeatures2d::BoostDesc>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::BoostDesc::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:373
	// ("cv::xfeatures2d::BoostDesc::create", vec![(pred!(mut, [], []), _)]),
	void cv_xfeatures2d_BoostDesc_create(Result<cv::Ptr<cv::xfeatures2d::BoostDesc>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::BoostDesc> ret = cv::xfeatures2d::BoostDesc::create();
			Ok(new cv::Ptr<cv::xfeatures2d::BoostDesc>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setUseScaleOrientation(const bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:376
	// ("cv::xfeatures2d::BoostDesc::setUseScaleOrientation", vec![(pred!(mut, ["use_scale_orientation"], ["const bool"]), _)]),
	void cv_xfeatures2d_BoostDesc_setUseScaleOrientation_const_bool(cv::xfeatures2d::BoostDesc* instance, const bool use_scale_orientation, ResultVoid* ocvrs_return) {
		try {
			instance->setUseScaleOrientation(use_scale_orientation);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getUseScaleOrientation()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:377
	// ("cv::xfeatures2d::BoostDesc::getUseScaleOrientation", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_BoostDesc_getUseScaleOrientation_const(const cv::xfeatures2d::BoostDesc* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUseScaleOrientation();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setScaleFactor(const float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:379
	// ("cv::xfeatures2d::BoostDesc::setScaleFactor", vec![(pred!(mut, ["scale_factor"], ["const float"]), _)]),
	void cv_xfeatures2d_BoostDesc_setScaleFactor_const_float(cv::xfeatures2d::BoostDesc* instance, const float scale_factor, ResultVoid* ocvrs_return) {
		try {
			instance->setScaleFactor(scale_factor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getScaleFactor()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:380
	// ("cv::xfeatures2d::BoostDesc::getScaleFactor", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_BoostDesc_getScaleFactor_const(const cv::xfeatures2d::BoostDesc* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getScaleFactor();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::BoostDesc::to_Algorithm() generated
	// ("cv::xfeatures2d::BoostDesc::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_xfeatures2d_BoostDesc_to_Algorithm(cv::xfeatures2d::BoostDesc* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::xfeatures2d::BoostDesc::to_Feature2D() generated
	// ("cv::xfeatures2d::BoostDesc::to_Feature2D", vec![(pred!(mut, [], []), _)]),
	cv::Feature2D* cv_xfeatures2d_BoostDesc_to_Feature2D(cv::xfeatures2d::BoostDesc* instance) {
			return dynamic_cast<cv::Feature2D*>(instance);
	}

	// cv::xfeatures2d::BoostDesc::delete() generated
	// ("cv::xfeatures2d::BoostDesc::delete", vec![(pred!(mut, [], []), _)]),
	void cv_xfeatures2d_BoostDesc_delete(cv::xfeatures2d::BoostDesc* instance) {
			delete instance;
	}

	// create(int, bool)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:132
	// ("cv::xfeatures2d::BriefDescriptorExtractor::create", vec![(pred!(mut, ["bytes", "use_orientation"], ["int", "bool"]), _)]),
	void cv_xfeatures2d_BriefDescriptorExtractor_create_int_bool(int bytes, bool use_orientation, Result<cv::Ptr<cv::xfeatures2d::BriefDescriptorExtractor>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::BriefDescriptorExtractor> ret = cv::xfeatures2d::BriefDescriptorExtractor::create(bytes, use_orientation);
			Ok(new cv::Ptr<cv::xfeatures2d::BriefDescriptorExtractor>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::BriefDescriptorExtractor::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:132
	// ("cv::xfeatures2d::BriefDescriptorExtractor::create", vec![(pred!(mut, [], []), _)]),
	void cv_xfeatures2d_BriefDescriptorExtractor_create(Result<cv::Ptr<cv::xfeatures2d::BriefDescriptorExtractor>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::BriefDescriptorExtractor> ret = cv::xfeatures2d::BriefDescriptorExtractor::create();
			Ok(new cv::Ptr<cv::xfeatures2d::BriefDescriptorExtractor>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::BriefDescriptorExtractor::defaultNew() generated
	// ("cv::xfeatures2d::BriefDescriptorExtractor::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::xfeatures2d::BriefDescriptorExtractor* cv_xfeatures2d_BriefDescriptorExtractor_defaultNew_const() {
			cv::xfeatures2d::BriefDescriptorExtractor* ret = new cv::xfeatures2d::BriefDescriptorExtractor();
			return ret;
	}

	// cv::xfeatures2d::BriefDescriptorExtractor::to_Algorithm() generated
	// ("cv::xfeatures2d::BriefDescriptorExtractor::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_xfeatures2d_BriefDescriptorExtractor_to_Algorithm(cv::xfeatures2d::BriefDescriptorExtractor* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::xfeatures2d::BriefDescriptorExtractor::to_Feature2D() generated
	// ("cv::xfeatures2d::BriefDescriptorExtractor::to_Feature2D", vec![(pred!(mut, [], []), _)]),
	cv::Feature2D* cv_xfeatures2d_BriefDescriptorExtractor_to_Feature2D(cv::xfeatures2d::BriefDescriptorExtractor* instance) {
			return dynamic_cast<cv::Feature2D*>(instance);
	}

	// cv::xfeatures2d::BriefDescriptorExtractor::delete() generated
	// ("cv::xfeatures2d::BriefDescriptorExtractor::delete", vec![(pred!(mut, [], []), _)]),
	void cv_xfeatures2d_BriefDescriptorExtractor_delete(cv::xfeatures2d::BriefDescriptorExtractor* instance) {
			delete instance;
	}

	// create(float, int, int, int, int, InputArray, bool, bool)(Primitive, Primitive, Primitive, Primitive, Primitive, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:204
	// ("cv::xfeatures2d::DAISY::create", vec![(pred!(mut, ["radius", "q_radius", "q_theta", "q_hist", "norm", "H", "interpolation", "use_orientation"], ["float", "int", "int", "int", "int", "const cv::_InputArray*", "bool", "bool"]), _)]),
	void cv_xfeatures2d_DAISY_create_float_int_int_int_int_const__InputArrayR_bool_bool(float radius, int q_radius, int q_theta, int q_hist, int norm, const cv::_InputArray* H, bool interpolation, bool use_orientation, Result<cv::Ptr<cv::xfeatures2d::DAISY>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::DAISY> ret = cv::xfeatures2d::DAISY::create(radius, q_radius, q_theta, q_hist, norm, *H, interpolation, use_orientation);
			Ok(new cv::Ptr<cv::xfeatures2d::DAISY>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::DAISY::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:204
	// ("cv::xfeatures2d::DAISY::create", vec![(pred!(mut, [], []), _)]),
	void cv_xfeatures2d_DAISY_create(Result<cv::Ptr<cv::xfeatures2d::DAISY>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::DAISY> ret = cv::xfeatures2d::DAISY::create();
			Ok(new cv::Ptr<cv::xfeatures2d::DAISY>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// compute(InputArray, std::vector<KeyPoint> &, OutputArray)(InputArray, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:213
	// ("cv::xfeatures2d::DAISY::compute", vec![(pred!(mut, ["image", "keypoints", "descriptors"], ["const cv::_InputArray*", "std::vector<cv::KeyPoint>*", "const cv::_OutputArray*"]), _)]),
	void cv_xfeatures2d_DAISY_compute_const__InputArrayR_vectorLKeyPointGR_const__OutputArrayR(cv::xfeatures2d::DAISY* instance, const cv::_InputArray* image, std::vector<cv::KeyPoint>* keypoints, const cv::_OutputArray* descriptors, ResultVoid* ocvrs_return) {
		try {
			instance->compute(*image, *keypoints, *descriptors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// compute(InputArrayOfArrays, std::vector<std::vector<KeyPoint>> &, OutputArrayOfArrays)(InputArray, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:215
	// ("cv::xfeatures2d::DAISY::compute", vec![(pred!(mut, ["images", "keypoints", "descriptors"], ["const cv::_InputArray*", "std::vector<std::vector<cv::KeyPoint>>*", "const cv::_OutputArray*"]), _)]),
	void cv_xfeatures2d_DAISY_compute_const__InputArrayR_vectorLvectorLKeyPointGGR_const__OutputArrayR(cv::xfeatures2d::DAISY* instance, const cv::_InputArray* images, std::vector<std::vector<cv::KeyPoint>>* keypoints, const cv::_OutputArray* descriptors, ResultVoid* ocvrs_return) {
		try {
			instance->compute(*images, *keypoints, *descriptors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// compute(InputArray, Rect, OutputArray)(InputArray, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:224
	// ("cv::xfeatures2d::DAISY::compute", vec![(pred!(mut, ["image", "roi", "descriptors"], ["const cv::_InputArray*", "cv::Rect", "const cv::_OutputArray*"]), _)]),
	void cv_xfeatures2d_DAISY_compute_const__InputArrayR_Rect_const__OutputArrayR(cv::xfeatures2d::DAISY* instance, const cv::_InputArray* image, cv::Rect* roi, const cv::_OutputArray* descriptors, ResultVoid* ocvrs_return) {
		try {
			instance->compute(*image, *roi, *descriptors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// compute(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:230
	// ("cv::xfeatures2d::DAISY::compute", vec![(pred!(mut, ["image", "descriptors"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_xfeatures2d_DAISY_compute_const__InputArrayR_const__OutputArrayR(cv::xfeatures2d::DAISY* instance, const cv::_InputArray* image, const cv::_OutputArray* descriptors, ResultVoid* ocvrs_return) {
		try {
			instance->compute(*image, *descriptors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// GetDescriptor(double, double, int, float *)(Primitive, Primitive, Primitive, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:238
	// ("cv::xfeatures2d::DAISY::GetDescriptor", vec![(pred!(const, ["y", "x", "orientation", "descriptor"], ["double", "double", "int", "float*"]), _)]),
	void cv_xfeatures2d_DAISY_GetDescriptor_const_double_double_int_floatX(const cv::xfeatures2d::DAISY* instance, double y, double x, int orientation, float* descriptor, ResultVoid* ocvrs_return) {
		try {
			instance->GetDescriptor(y, x, orientation, descriptor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// GetDescriptor(double, double, int, float *, double *)(Primitive, Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:247
	// ("cv::xfeatures2d::DAISY::GetDescriptor", vec![(pred!(const, ["y", "x", "orientation", "descriptor", "H"], ["double", "double", "int", "float*", "double*"]), _)]),
	void cv_xfeatures2d_DAISY_GetDescriptor_const_double_double_int_floatX_doubleX(const cv::xfeatures2d::DAISY* instance, double y, double x, int orientation, float* descriptor, double* H, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->GetDescriptor(y, x, orientation, descriptor, H);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// GetUnnormalizedDescriptor(double, double, int, float *)(Primitive, Primitive, Primitive, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:255
	// ("cv::xfeatures2d::DAISY::GetUnnormalizedDescriptor", vec![(pred!(const, ["y", "x", "orientation", "descriptor"], ["double", "double", "int", "float*"]), _)]),
	void cv_xfeatures2d_DAISY_GetUnnormalizedDescriptor_const_double_double_int_floatX(const cv::xfeatures2d::DAISY* instance, double y, double x, int orientation, float* descriptor, ResultVoid* ocvrs_return) {
		try {
			instance->GetUnnormalizedDescriptor(y, x, orientation, descriptor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// GetUnnormalizedDescriptor(double, double, int, float *, double *)(Primitive, Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:264
	// ("cv::xfeatures2d::DAISY::GetUnnormalizedDescriptor", vec![(pred!(const, ["y", "x", "orientation", "descriptor", "H"], ["double", "double", "int", "float*", "double*"]), _)]),
	void cv_xfeatures2d_DAISY_GetUnnormalizedDescriptor_const_double_double_int_floatX_doubleX(const cv::xfeatures2d::DAISY* instance, double y, double x, int orientation, float* descriptor, double* H, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->GetUnnormalizedDescriptor(y, x, orientation, descriptor, H);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::DAISY::to_Algorithm() generated
	// ("cv::xfeatures2d::DAISY::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_xfeatures2d_DAISY_to_Algorithm(cv::xfeatures2d::DAISY* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::xfeatures2d::DAISY::to_Feature2D() generated
	// ("cv::xfeatures2d::DAISY::to_Feature2D", vec![(pred!(mut, [], []), _)]),
	cv::Feature2D* cv_xfeatures2d_DAISY_to_Feature2D(cv::xfeatures2d::DAISY* instance) {
			return dynamic_cast<cv::Feature2D*>(instance);
	}

	// cv::xfeatures2d::DAISY::delete() generated
	// ("cv::xfeatures2d::DAISY::delete", vec![(pred!(mut, [], []), _)]),
	void cv_xfeatures2d_DAISY_delete(cv::xfeatures2d::DAISY* instance) {
			delete instance;
	}

	// Elliptic_KeyPoint()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:865
	// ("cv::xfeatures2d::Elliptic_KeyPoint::Elliptic_KeyPoint", vec![(pred!(mut, [], []), _)]),
	void cv_xfeatures2d_Elliptic_KeyPoint_Elliptic_KeyPoint(Result<cv::xfeatures2d::Elliptic_KeyPoint*>* ocvrs_return) {
		try {
			cv::xfeatures2d::Elliptic_KeyPoint* ret = new cv::xfeatures2d::Elliptic_KeyPoint();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Elliptic_KeyPoint(Point2f, float, Size, float, float)(SimpleClass, Primitive, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:866
	// ("cv::xfeatures2d::Elliptic_KeyPoint::Elliptic_KeyPoint", vec![(pred!(mut, ["pt", "angle", "axes", "size", "si"], ["cv::Point2f", "float", "cv::Size", "float", "float"]), _)]),
	void cv_xfeatures2d_Elliptic_KeyPoint_Elliptic_KeyPoint_Point2f_float_Size_float_float(cv::Point2f* pt, float angle, cv::Size* axes, float size, float si, Result<cv::xfeatures2d::Elliptic_KeyPoint*>* ocvrs_return) {
		try {
			cv::xfeatures2d::Elliptic_KeyPoint* ret = new cv::xfeatures2d::Elliptic_KeyPoint(*pt, angle, *axes, size, si);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::Elliptic_KeyPoint::axes() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:862
	// ("cv::xfeatures2d::Elliptic_KeyPoint::axes", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_Elliptic_KeyPoint_propAxes_const(const cv::xfeatures2d::Elliptic_KeyPoint* instance, cv::Size_<float>* ocvrs_return) {
			cv::Size_<float> ret = instance->axes;
			*ocvrs_return = ret;
	}

	// cv::xfeatures2d::Elliptic_KeyPoint::setAxes(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:862
	// ("cv::xfeatures2d::Elliptic_KeyPoint::setAxes", vec![(pred!(mut, ["val"], ["const cv::Size_<float>"]), _)]),
	void cv_xfeatures2d_Elliptic_KeyPoint_propAxes_const_Size_LfloatG(cv::xfeatures2d::Elliptic_KeyPoint* instance, const cv::Size_<float>* val) {
			instance->axes = *val;
	}

	// cv::xfeatures2d::Elliptic_KeyPoint::si() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:863
	// ("cv::xfeatures2d::Elliptic_KeyPoint::si", vec![(pred!(const, [], []), _)]),
	float cv_xfeatures2d_Elliptic_KeyPoint_propSi_const(const cv::xfeatures2d::Elliptic_KeyPoint* instance) {
			float ret = instance->si;
			return ret;
	}

	// cv::xfeatures2d::Elliptic_KeyPoint::setSi(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:863
	// ("cv::xfeatures2d::Elliptic_KeyPoint::setSi", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_xfeatures2d_Elliptic_KeyPoint_propSi_const_float(cv::xfeatures2d::Elliptic_KeyPoint* instance, const float val) {
			instance->si = val;
	}

	// cv::xfeatures2d::Elliptic_KeyPoint::transf() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:864
	// ("cv::xfeatures2d::Elliptic_KeyPoint::transf", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_Elliptic_KeyPoint_propTransf_const(const cv::xfeatures2d::Elliptic_KeyPoint* instance, cv::Matx23f* ocvrs_return) {
			cv::Matx23f ret = instance->transf;
			*ocvrs_return = ret;
	}

	// cv::xfeatures2d::Elliptic_KeyPoint::setTransf(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:864
	// ("cv::xfeatures2d::Elliptic_KeyPoint::setTransf", vec![(pred!(mut, ["val"], ["const cv::Matx23f"]), _)]),
	void cv_xfeatures2d_Elliptic_KeyPoint_propTransf_const_Matx23f(cv::xfeatures2d::Elliptic_KeyPoint* instance, const cv::Matx23f* val) {
			instance->transf = *val;
	}

	// cv::xfeatures2d::Elliptic_KeyPoint::to_KeyPoint() generated
	// ("cv::xfeatures2d::Elliptic_KeyPoint::to_KeyPoint", vec![(pred!(mut, [], []), _)]),
	cv::KeyPoint* cv_xfeatures2d_Elliptic_KeyPoint_to_KeyPoint(cv::xfeatures2d::Elliptic_KeyPoint* instance) {
			return dynamic_cast<cv::KeyPoint*>(instance);
	}

	// cv::xfeatures2d::Elliptic_KeyPoint::delete() generated
	// ("cv::xfeatures2d::Elliptic_KeyPoint::delete", vec![(pred!(mut, [], []), _)]),
	void cv_xfeatures2d_Elliptic_KeyPoint_delete(cv::xfeatures2d::Elliptic_KeyPoint* instance) {
			delete instance;
	}

	// create(bool, bool, float, int, const std::vector<int> &)(Primitive, Primitive, Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:99
	// ("cv::xfeatures2d::FREAK::create", vec![(pred!(mut, ["orientationNormalized", "scaleNormalized", "patternScale", "nOctaves", "selectedPairs"], ["bool", "bool", "float", "int", "const std::vector<int>*"]), _)]),
	void cv_xfeatures2d_FREAK_create_bool_bool_float_int_const_vectorLintGR(bool orientationNormalized, bool scaleNormalized, float patternScale, int nOctaves, const std::vector<int>* selectedPairs, Result<cv::Ptr<cv::xfeatures2d::FREAK>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::FREAK> ret = cv::xfeatures2d::FREAK::create(orientationNormalized, scaleNormalized, patternScale, nOctaves, *selectedPairs);
			Ok(new cv::Ptr<cv::xfeatures2d::FREAK>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::FREAK::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:99
	// ("cv::xfeatures2d::FREAK::create", vec![(pred!(mut, [], []), _)]),
	void cv_xfeatures2d_FREAK_create(Result<cv::Ptr<cv::xfeatures2d::FREAK>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::FREAK> ret = cv::xfeatures2d::FREAK::create();
			Ok(new cv::Ptr<cv::xfeatures2d::FREAK>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::FREAK::defaultNew() generated
	// ("cv::xfeatures2d::FREAK::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::xfeatures2d::FREAK* cv_xfeatures2d_FREAK_defaultNew_const() {
			cv::xfeatures2d::FREAK* ret = new cv::xfeatures2d::FREAK();
			return ret;
	}

	// cv::xfeatures2d::FREAK::to_Algorithm() generated
	// ("cv::xfeatures2d::FREAK::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_xfeatures2d_FREAK_to_Algorithm(cv::xfeatures2d::FREAK* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::xfeatures2d::FREAK::to_Feature2D() generated
	// ("cv::xfeatures2d::FREAK::to_Feature2D", vec![(pred!(mut, [], []), _)]),
	cv::Feature2D* cv_xfeatures2d_FREAK_to_Feature2D(cv::xfeatures2d::FREAK* instance) {
			return dynamic_cast<cv::Feature2D*>(instance);
	}

	// cv::xfeatures2d::FREAK::delete() generated
	// ("cv::xfeatures2d::FREAK::delete", vec![(pred!(mut, [], []), _)]),
	void cv_xfeatures2d_FREAK_delete(cv::xfeatures2d::FREAK* instance) {
			delete instance;
	}

	// create(int, float, float, int, int)(Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:885
	// ("cv::xfeatures2d::HarrisLaplaceFeatureDetector::create", vec![(pred!(mut, ["numOctaves", "corn_thresh", "DOG_thresh", "maxCorners", "num_layers"], ["int", "float", "float", "int", "int"]), _)]),
	void cv_xfeatures2d_HarrisLaplaceFeatureDetector_create_int_float_float_int_int(int numOctaves, float corn_thresh, float DOG_thresh, int maxCorners, int num_layers, Result<cv::Ptr<cv::xfeatures2d::HarrisLaplaceFeatureDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::HarrisLaplaceFeatureDetector> ret = cv::xfeatures2d::HarrisLaplaceFeatureDetector::create(numOctaves, corn_thresh, DOG_thresh, maxCorners, num_layers);
			Ok(new cv::Ptr<cv::xfeatures2d::HarrisLaplaceFeatureDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::HarrisLaplaceFeatureDetector::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:885
	// ("cv::xfeatures2d::HarrisLaplaceFeatureDetector::create", vec![(pred!(mut, [], []), _)]),
	void cv_xfeatures2d_HarrisLaplaceFeatureDetector_create(Result<cv::Ptr<cv::xfeatures2d::HarrisLaplaceFeatureDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::HarrisLaplaceFeatureDetector> ret = cv::xfeatures2d::HarrisLaplaceFeatureDetector::create();
			Ok(new cv::Ptr<cv::xfeatures2d::HarrisLaplaceFeatureDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::HarrisLaplaceFeatureDetector::defaultNew() generated
	// ("cv::xfeatures2d::HarrisLaplaceFeatureDetector::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::xfeatures2d::HarrisLaplaceFeatureDetector* cv_xfeatures2d_HarrisLaplaceFeatureDetector_defaultNew_const() {
			cv::xfeatures2d::HarrisLaplaceFeatureDetector* ret = new cv::xfeatures2d::HarrisLaplaceFeatureDetector();
			return ret;
	}

	// cv::xfeatures2d::HarrisLaplaceFeatureDetector::to_Algorithm() generated
	// ("cv::xfeatures2d::HarrisLaplaceFeatureDetector::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_xfeatures2d_HarrisLaplaceFeatureDetector_to_Algorithm(cv::xfeatures2d::HarrisLaplaceFeatureDetector* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::xfeatures2d::HarrisLaplaceFeatureDetector::to_Feature2D() generated
	// ("cv::xfeatures2d::HarrisLaplaceFeatureDetector::to_Feature2D", vec![(pred!(mut, [], []), _)]),
	cv::Feature2D* cv_xfeatures2d_HarrisLaplaceFeatureDetector_to_Feature2D(cv::xfeatures2d::HarrisLaplaceFeatureDetector* instance) {
			return dynamic_cast<cv::Feature2D*>(instance);
	}

	// cv::xfeatures2d::HarrisLaplaceFeatureDetector::delete() generated
	// ("cv::xfeatures2d::HarrisLaplaceFeatureDetector::delete", vec![(pred!(mut, [], []), _)]),
	void cv_xfeatures2d_HarrisLaplaceFeatureDetector_delete(cv::xfeatures2d::HarrisLaplaceFeatureDetector* instance) {
			delete instance;
	}

	// create(int, bool, int, double)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:178
	// ("cv::xfeatures2d::LATCH::create", vec![(pred!(mut, ["bytes", "rotationInvariance", "half_ssd_size", "sigma"], ["int", "bool", "int", "double"]), _)]),
	void cv_xfeatures2d_LATCH_create_int_bool_int_double(int bytes, bool rotationInvariance, int half_ssd_size, double sigma, Result<cv::Ptr<cv::xfeatures2d::LATCH>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::LATCH> ret = cv::xfeatures2d::LATCH::create(bytes, rotationInvariance, half_ssd_size, sigma);
			Ok(new cv::Ptr<cv::xfeatures2d::LATCH>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::LATCH::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:178
	// ("cv::xfeatures2d::LATCH::create", vec![(pred!(mut, [], []), _)]),
	void cv_xfeatures2d_LATCH_create(Result<cv::Ptr<cv::xfeatures2d::LATCH>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::LATCH> ret = cv::xfeatures2d::LATCH::create();
			Ok(new cv::Ptr<cv::xfeatures2d::LATCH>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::LATCH::defaultNew() generated
	// ("cv::xfeatures2d::LATCH::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::xfeatures2d::LATCH* cv_xfeatures2d_LATCH_defaultNew_const() {
			cv::xfeatures2d::LATCH* ret = new cv::xfeatures2d::LATCH();
			return ret;
	}

	// cv::xfeatures2d::LATCH::to_Algorithm() generated
	// ("cv::xfeatures2d::LATCH::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_xfeatures2d_LATCH_to_Algorithm(cv::xfeatures2d::LATCH* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::xfeatures2d::LATCH::to_Feature2D() generated
	// ("cv::xfeatures2d::LATCH::to_Feature2D", vec![(pred!(mut, [], []), _)]),
	cv::Feature2D* cv_xfeatures2d_LATCH_to_Feature2D(cv::xfeatures2d::LATCH* instance) {
			return dynamic_cast<cv::Feature2D*>(instance);
	}

	// cv::xfeatures2d::LATCH::delete() generated
	// ("cv::xfeatures2d::LATCH::delete", vec![(pred!(mut, [], []), _)]),
	void cv_xfeatures2d_LATCH_delete(cv::xfeatures2d::LATCH* instance) {
			delete instance;
	}

	// create(const int, const int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:149
	// ("cv::xfeatures2d::LUCID::create", vec![(pred!(mut, ["lucid_kernel", "blur_kernel"], ["const int", "const int"]), _)]),
	void cv_xfeatures2d_LUCID_create_const_int_const_int(const int lucid_kernel, const int blur_kernel, Result<cv::Ptr<cv::xfeatures2d::LUCID>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::LUCID> ret = cv::xfeatures2d::LUCID::create(lucid_kernel, blur_kernel);
			Ok(new cv::Ptr<cv::xfeatures2d::LUCID>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::LUCID::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:149
	// ("cv::xfeatures2d::LUCID::create", vec![(pred!(mut, [], []), _)]),
	void cv_xfeatures2d_LUCID_create(Result<cv::Ptr<cv::xfeatures2d::LUCID>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::LUCID> ret = cv::xfeatures2d::LUCID::create();
			Ok(new cv::Ptr<cv::xfeatures2d::LUCID>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::LUCID::defaultNew() generated
	// ("cv::xfeatures2d::LUCID::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::xfeatures2d::LUCID* cv_xfeatures2d_LUCID_defaultNew_const() {
			cv::xfeatures2d::LUCID* ret = new cv::xfeatures2d::LUCID();
			return ret;
	}

	// cv::xfeatures2d::LUCID::to_Algorithm() generated
	// ("cv::xfeatures2d::LUCID::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_xfeatures2d_LUCID_to_Algorithm(cv::xfeatures2d::LUCID* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::xfeatures2d::LUCID::to_Feature2D() generated
	// ("cv::xfeatures2d::LUCID::to_Feature2D", vec![(pred!(mut, [], []), _)]),
	cv::Feature2D* cv_xfeatures2d_LUCID_to_Feature2D(cv::xfeatures2d::LUCID* instance) {
			return dynamic_cast<cv::Feature2D*>(instance);
	}

	// cv::xfeatures2d::LUCID::delete() generated
	// ("cv::xfeatures2d::LUCID::delete", vec![(pred!(mut, [], []), _)]),
	void cv_xfeatures2d_LUCID_delete(cv::xfeatures2d::LUCID* instance) {
			delete instance;
	}

	// create(int, int, int, int, float, int, float, int, bool)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:285
	// ("cv::xfeatures2d::MSDDetector::create", vec![(pred!(mut, ["m_patch_radius", "m_search_area_radius", "m_nms_radius", "m_nms_scale_radius", "m_th_saliency", "m_kNN", "m_scale_factor", "m_n_scales", "m_compute_orientation"], ["int", "int", "int", "int", "float", "int", "float", "int", "bool"]), _)]),
	void cv_xfeatures2d_MSDDetector_create_int_int_int_int_float_int_float_int_bool(int m_patch_radius, int m_search_area_radius, int m_nms_radius, int m_nms_scale_radius, float m_th_saliency, int m_kNN, float m_scale_factor, int m_n_scales, bool m_compute_orientation, Result<cv::Ptr<cv::xfeatures2d::MSDDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::MSDDetector> ret = cv::xfeatures2d::MSDDetector::create(m_patch_radius, m_search_area_radius, m_nms_radius, m_nms_scale_radius, m_th_saliency, m_kNN, m_scale_factor, m_n_scales, m_compute_orientation);
			Ok(new cv::Ptr<cv::xfeatures2d::MSDDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::MSDDetector::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:285
	// ("cv::xfeatures2d::MSDDetector::create", vec![(pred!(mut, [], []), _)]),
	void cv_xfeatures2d_MSDDetector_create(Result<cv::Ptr<cv::xfeatures2d::MSDDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::MSDDetector> ret = cv::xfeatures2d::MSDDetector::create();
			Ok(new cv::Ptr<cv::xfeatures2d::MSDDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::MSDDetector::defaultNew() generated
	// ("cv::xfeatures2d::MSDDetector::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::xfeatures2d::MSDDetector* cv_xfeatures2d_MSDDetector_defaultNew_const() {
			cv::xfeatures2d::MSDDetector* ret = new cv::xfeatures2d::MSDDetector();
			return ret;
	}

	// cv::xfeatures2d::MSDDetector::to_Algorithm() generated
	// ("cv::xfeatures2d::MSDDetector::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_xfeatures2d_MSDDetector_to_Algorithm(cv::xfeatures2d::MSDDetector* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::xfeatures2d::MSDDetector::to_Feature2D() generated
	// ("cv::xfeatures2d::MSDDetector::to_Feature2D", vec![(pred!(mut, [], []), _)]),
	cv::Feature2D* cv_xfeatures2d_MSDDetector_to_Feature2D(cv::xfeatures2d::MSDDetector* instance) {
			return dynamic_cast<cv::Feature2D*>(instance);
	}

	// cv::xfeatures2d::MSDDetector::delete() generated
	// ("cv::xfeatures2d::MSDDetector::delete", vec![(pred!(mut, [], []), _)]),
	void cv_xfeatures2d_MSDDetector_delete(cv::xfeatures2d::MSDDetector* instance) {
			delete instance;
	}

	// create(const int, const int, const int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:451
	// ("cv::xfeatures2d::PCTSignatures::create", vec![(pred!(mut, ["initSampleCount", "initSeedCount", "pointDistribution"], ["const int", "const int", "const int"]), _)]),
	void cv_xfeatures2d_PCTSignatures_create_const_int_const_int_const_int(const int initSampleCount, const int initSeedCount, const int pointDistribution, Result<cv::Ptr<cv::xfeatures2d::PCTSignatures>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::PCTSignatures> ret = cv::xfeatures2d::PCTSignatures::create(initSampleCount, initSeedCount, pointDistribution);
			Ok(new cv::Ptr<cv::xfeatures2d::PCTSignatures>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::PCTSignatures::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:451
	// ("cv::xfeatures2d::PCTSignatures::create", vec![(pred!(mut, [], []), _)]),
	void cv_xfeatures2d_PCTSignatures_create(Result<cv::Ptr<cv::xfeatures2d::PCTSignatures>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::PCTSignatures> ret = cv::xfeatures2d::PCTSignatures::create();
			Ok(new cv::Ptr<cv::xfeatures2d::PCTSignatures>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const std::vector<Point2f> &, const int)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:465
	// ("cv::xfeatures2d::PCTSignatures::create", vec![(pred!(mut, ["initSamplingPoints", "initSeedCount"], ["const std::vector<cv::Point2f>*", "const int"]), _)]),
	void cv_xfeatures2d_PCTSignatures_create_const_vectorLPoint2fGR_const_int(const std::vector<cv::Point2f>* initSamplingPoints, const int initSeedCount, Result<cv::Ptr<cv::xfeatures2d::PCTSignatures>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::PCTSignatures> ret = cv::xfeatures2d::PCTSignatures::create(*initSamplingPoints, initSeedCount);
			Ok(new cv::Ptr<cv::xfeatures2d::PCTSignatures>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const std::vector<Point2f> &, const std::vector<int> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:477
	// ("cv::xfeatures2d::PCTSignatures::create", vec![(pred!(mut, ["initSamplingPoints", "initClusterSeedIndexes"], ["const std::vector<cv::Point2f>*", "const std::vector<int>*"]), _)]),
	void cv_xfeatures2d_PCTSignatures_create_const_vectorLPoint2fGR_const_vectorLintGR(const std::vector<cv::Point2f>* initSamplingPoints, const std::vector<int>* initClusterSeedIndexes, Result<cv::Ptr<cv::xfeatures2d::PCTSignatures>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::PCTSignatures> ret = cv::xfeatures2d::PCTSignatures::create(*initSamplingPoints, *initClusterSeedIndexes);
			Ok(new cv::Ptr<cv::xfeatures2d::PCTSignatures>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// computeSignature(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:488
	// ("cv::xfeatures2d::PCTSignatures::computeSignature", vec![(pred!(const, ["image", "signature"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_xfeatures2d_PCTSignatures_computeSignature_const_const__InputArrayR_const__OutputArrayR(const cv::xfeatures2d::PCTSignatures* instance, const cv::_InputArray* image, const cv::_OutputArray* signature, ResultVoid* ocvrs_return) {
		try {
			instance->computeSignature(*image, *signature);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// computeSignatures(const std::vector<Mat> &, std::vector<Mat> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:497
	// ("cv::xfeatures2d::PCTSignatures::computeSignatures", vec![(pred!(const, ["images", "signatures"], ["const std::vector<cv::Mat>*", "std::vector<cv::Mat>*"]), _)]),
	void cv_xfeatures2d_PCTSignatures_computeSignatures_const_const_vectorLMatGR_vectorLMatGR(const cv::xfeatures2d::PCTSignatures* instance, const std::vector<cv::Mat>* images, std::vector<cv::Mat>* signatures, ResultVoid* ocvrs_return) {
		try {
			instance->computeSignatures(*images, *signatures);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// drawSignature(InputArray, InputArray, OutputArray, float, int)(InputArray, InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:513
	// ("cv::xfeatures2d::PCTSignatures::drawSignature", vec![(pred!(mut, ["source", "signature", "result", "radiusToShorterSideRatio", "borderThickness"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "float", "int"]), _)]),
	void cv_xfeatures2d_PCTSignatures_drawSignature_const__InputArrayR_const__InputArrayR_const__OutputArrayR_float_int(const cv::_InputArray* source, const cv::_InputArray* signature, const cv::_OutputArray* result, float radiusToShorterSideRatio, int borderThickness, ResultVoid* ocvrs_return) {
		try {
			cv::xfeatures2d::PCTSignatures::drawSignature(*source, *signature, *result, radiusToShorterSideRatio, borderThickness);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::PCTSignatures::drawSignature(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:513
	// ("cv::xfeatures2d::PCTSignatures::drawSignature", vec![(pred!(mut, ["source", "signature", "result"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_xfeatures2d_PCTSignatures_drawSignature_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* source, const cv::_InputArray* signature, const cv::_OutputArray* result, ResultVoid* ocvrs_return) {
		try {
			cv::xfeatures2d::PCTSignatures::drawSignature(*source, *signature, *result);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// generateInitPoints(std::vector<Point2f> &, const int, int)(CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:528
	// ("cv::xfeatures2d::PCTSignatures::generateInitPoints", vec![(pred!(mut, ["initPoints", "count", "pointDistribution"], ["std::vector<cv::Point2f>*", "const int", "int"]), _)]),
	void cv_xfeatures2d_PCTSignatures_generateInitPoints_vectorLPoint2fGR_const_int_int(std::vector<cv::Point2f>* initPoints, const int count, int pointDistribution, ResultVoid* ocvrs_return) {
		try {
			cv::xfeatures2d::PCTSignatures::generateInitPoints(*initPoints, count, pointDistribution);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSampleCount()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:539
	// ("cv::xfeatures2d::PCTSignatures::getSampleCount", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_PCTSignatures_getSampleCount_const(const cv::xfeatures2d::PCTSignatures* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getSampleCount();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getGrayscaleBits()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:546
	// ("cv::xfeatures2d::PCTSignatures::getGrayscaleBits", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_PCTSignatures_getGrayscaleBits_const(const cv::xfeatures2d::PCTSignatures* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getGrayscaleBits();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setGrayscaleBits(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:552
	// ("cv::xfeatures2d::PCTSignatures::setGrayscaleBits", vec![(pred!(mut, ["grayscaleBits"], ["int"]), _)]),
	void cv_xfeatures2d_PCTSignatures_setGrayscaleBits_int(cv::xfeatures2d::PCTSignatures* instance, int grayscaleBits, ResultVoid* ocvrs_return) {
		try {
			instance->setGrayscaleBits(grayscaleBits);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getWindowRadius()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:559
	// ("cv::xfeatures2d::PCTSignatures::getWindowRadius", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_PCTSignatures_getWindowRadius_const(const cv::xfeatures2d::PCTSignatures* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getWindowRadius();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWindowRadius(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:565
	// ("cv::xfeatures2d::PCTSignatures::setWindowRadius", vec![(pred!(mut, ["radius"], ["int"]), _)]),
	void cv_xfeatures2d_PCTSignatures_setWindowRadius_int(cv::xfeatures2d::PCTSignatures* instance, int radius, ResultVoid* ocvrs_return) {
		try {
			instance->setWindowRadius(radius);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getWeightX()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:572
	// ("cv::xfeatures2d::PCTSignatures::getWeightX", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_PCTSignatures_getWeightX_const(const cv::xfeatures2d::PCTSignatures* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getWeightX();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWeightX(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:577
	// ("cv::xfeatures2d::PCTSignatures::setWeightX", vec![(pred!(mut, ["weight"], ["float"]), _)]),
	void cv_xfeatures2d_PCTSignatures_setWeightX_float(cv::xfeatures2d::PCTSignatures* instance, float weight, ResultVoid* ocvrs_return) {
		try {
			instance->setWeightX(weight);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getWeightY()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:583
	// ("cv::xfeatures2d::PCTSignatures::getWeightY", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_PCTSignatures_getWeightY_const(const cv::xfeatures2d::PCTSignatures* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getWeightY();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWeightY(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:588
	// ("cv::xfeatures2d::PCTSignatures::setWeightY", vec![(pred!(mut, ["weight"], ["float"]), _)]),
	void cv_xfeatures2d_PCTSignatures_setWeightY_float(cv::xfeatures2d::PCTSignatures* instance, float weight, ResultVoid* ocvrs_return) {
		try {
			instance->setWeightY(weight);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getWeightL()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:594
	// ("cv::xfeatures2d::PCTSignatures::getWeightL", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_PCTSignatures_getWeightL_const(const cv::xfeatures2d::PCTSignatures* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getWeightL();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWeightL(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:599
	// ("cv::xfeatures2d::PCTSignatures::setWeightL", vec![(pred!(mut, ["weight"], ["float"]), _)]),
	void cv_xfeatures2d_PCTSignatures_setWeightL_float(cv::xfeatures2d::PCTSignatures* instance, float weight, ResultVoid* ocvrs_return) {
		try {
			instance->setWeightL(weight);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getWeightA()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:605
	// ("cv::xfeatures2d::PCTSignatures::getWeightA", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_PCTSignatures_getWeightA_const(const cv::xfeatures2d::PCTSignatures* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getWeightA();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWeightA(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:610
	// ("cv::xfeatures2d::PCTSignatures::setWeightA", vec![(pred!(mut, ["weight"], ["float"]), _)]),
	void cv_xfeatures2d_PCTSignatures_setWeightA_float(cv::xfeatures2d::PCTSignatures* instance, float weight, ResultVoid* ocvrs_return) {
		try {
			instance->setWeightA(weight);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getWeightB()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:616
	// ("cv::xfeatures2d::PCTSignatures::getWeightB", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_PCTSignatures_getWeightB_const(const cv::xfeatures2d::PCTSignatures* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getWeightB();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWeightB(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:621
	// ("cv::xfeatures2d::PCTSignatures::setWeightB", vec![(pred!(mut, ["weight"], ["float"]), _)]),
	void cv_xfeatures2d_PCTSignatures_setWeightB_float(cv::xfeatures2d::PCTSignatures* instance, float weight, ResultVoid* ocvrs_return) {
		try {
			instance->setWeightB(weight);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getWeightContrast()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:627
	// ("cv::xfeatures2d::PCTSignatures::getWeightContrast", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_PCTSignatures_getWeightContrast_const(const cv::xfeatures2d::PCTSignatures* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getWeightContrast();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWeightContrast(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:632
	// ("cv::xfeatures2d::PCTSignatures::setWeightContrast", vec![(pred!(mut, ["weight"], ["float"]), _)]),
	void cv_xfeatures2d_PCTSignatures_setWeightContrast_float(cv::xfeatures2d::PCTSignatures* instance, float weight, ResultVoid* ocvrs_return) {
		try {
			instance->setWeightContrast(weight);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getWeightEntropy()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:638
	// ("cv::xfeatures2d::PCTSignatures::getWeightEntropy", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_PCTSignatures_getWeightEntropy_const(const cv::xfeatures2d::PCTSignatures* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getWeightEntropy();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWeightEntropy(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:643
	// ("cv::xfeatures2d::PCTSignatures::setWeightEntropy", vec![(pred!(mut, ["weight"], ["float"]), _)]),
	void cv_xfeatures2d_PCTSignatures_setWeightEntropy_float(cv::xfeatures2d::PCTSignatures* instance, float weight, ResultVoid* ocvrs_return) {
		try {
			instance->setWeightEntropy(weight);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSamplingPoints()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:649
	// ("cv::xfeatures2d::PCTSignatures::getSamplingPoints", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_PCTSignatures_getSamplingPoints_const(const cv::xfeatures2d::PCTSignatures* instance, Result<std::vector<cv::Point2f>*>* ocvrs_return) {
		try {
			std::vector<cv::Point2f> ret = instance->getSamplingPoints();
			Ok(new std::vector<cv::Point2f>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWeight(int, float)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:667
	// ("cv::xfeatures2d::PCTSignatures::setWeight", vec![(pred!(mut, ["idx", "value"], ["int", "float"]), _)]),
	void cv_xfeatures2d_PCTSignatures_setWeight_int_float(cv::xfeatures2d::PCTSignatures* instance, int idx, float value, ResultVoid* ocvrs_return) {
		try {
			instance->setWeight(idx, value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWeights(const std::vector<float> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:681
	// ("cv::xfeatures2d::PCTSignatures::setWeights", vec![(pred!(mut, ["weights"], ["const std::vector<float>*"]), _)]),
	void cv_xfeatures2d_PCTSignatures_setWeights_const_vectorLfloatGR(cv::xfeatures2d::PCTSignatures* instance, const std::vector<float>* weights, ResultVoid* ocvrs_return) {
		try {
			instance->setWeights(*weights);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTranslation(int, float)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:697
	// ("cv::xfeatures2d::PCTSignatures::setTranslation", vec![(pred!(mut, ["idx", "value"], ["int", "float"]), _)]),
	void cv_xfeatures2d_PCTSignatures_setTranslation_int_float(cv::xfeatures2d::PCTSignatures* instance, int idx, float value, ResultVoid* ocvrs_return) {
		try {
			instance->setTranslation(idx, value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTranslations(const std::vector<float> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:711
	// ("cv::xfeatures2d::PCTSignatures::setTranslations", vec![(pred!(mut, ["translations"], ["const std::vector<float>*"]), _)]),
	void cv_xfeatures2d_PCTSignatures_setTranslations_const_vectorLfloatGR(cv::xfeatures2d::PCTSignatures* instance, const std::vector<float>* translations, ResultVoid* ocvrs_return) {
		try {
			instance->setTranslations(*translations);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSamplingPoints(std::vector<Point2f>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:718
	// ("cv::xfeatures2d::PCTSignatures::setSamplingPoints", vec![(pred!(mut, ["samplingPoints"], ["std::vector<cv::Point2f>"]), _)]),
	void cv_xfeatures2d_PCTSignatures_setSamplingPoints_vectorLPoint2fG(cv::xfeatures2d::PCTSignatures* instance, std::vector<cv::Point2f>* samplingPoints, ResultVoid* ocvrs_return) {
		try {
			instance->setSamplingPoints(*samplingPoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getInitSeedIndexes()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:726
	// ("cv::xfeatures2d::PCTSignatures::getInitSeedIndexes", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_PCTSignatures_getInitSeedIndexes_const(const cv::xfeatures2d::PCTSignatures* instance, Result<std::vector<int>*>* ocvrs_return) {
		try {
			std::vector<int> ret = instance->getInitSeedIndexes();
			Ok(new std::vector<int>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setInitSeedIndexes(std::vector<int>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:730
	// ("cv::xfeatures2d::PCTSignatures::setInitSeedIndexes", vec![(pred!(mut, ["initSeedIndexes"], ["std::vector<int>"]), _)]),
	void cv_xfeatures2d_PCTSignatures_setInitSeedIndexes_vectorLintG(cv::xfeatures2d::PCTSignatures* instance, std::vector<int>* initSeedIndexes, ResultVoid* ocvrs_return) {
		try {
			instance->setInitSeedIndexes(*initSeedIndexes);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getInitSeedCount()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:734
	// ("cv::xfeatures2d::PCTSignatures::getInitSeedCount", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_PCTSignatures_getInitSeedCount_const(const cv::xfeatures2d::PCTSignatures* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getInitSeedCount();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getIterationCount()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:741
	// ("cv::xfeatures2d::PCTSignatures::getIterationCount", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_PCTSignatures_getIterationCount_const(const cv::xfeatures2d::PCTSignatures* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getIterationCount();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setIterationCount(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:747
	// ("cv::xfeatures2d::PCTSignatures::setIterationCount", vec![(pred!(mut, ["iterationCount"], ["int"]), _)]),
	void cv_xfeatures2d_PCTSignatures_setIterationCount_int(cv::xfeatures2d::PCTSignatures* instance, int iterationCount, ResultVoid* ocvrs_return) {
		try {
			instance->setIterationCount(iterationCount);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxClustersCount()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:753
	// ("cv::xfeatures2d::PCTSignatures::getMaxClustersCount", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_PCTSignatures_getMaxClustersCount_const(const cv::xfeatures2d::PCTSignatures* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMaxClustersCount();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxClustersCount(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:758
	// ("cv::xfeatures2d::PCTSignatures::setMaxClustersCount", vec![(pred!(mut, ["maxClustersCount"], ["int"]), _)]),
	void cv_xfeatures2d_PCTSignatures_setMaxClustersCount_int(cv::xfeatures2d::PCTSignatures* instance, int maxClustersCount, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxClustersCount(maxClustersCount);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getClusterMinSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:765
	// ("cv::xfeatures2d::PCTSignatures::getClusterMinSize", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_PCTSignatures_getClusterMinSize_const(const cv::xfeatures2d::PCTSignatures* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getClusterMinSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setClusterMinSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:771
	// ("cv::xfeatures2d::PCTSignatures::setClusterMinSize", vec![(pred!(mut, ["clusterMinSize"], ["int"]), _)]),
	void cv_xfeatures2d_PCTSignatures_setClusterMinSize_int(cv::xfeatures2d::PCTSignatures* instance, int clusterMinSize, ResultVoid* ocvrs_return) {
		try {
			instance->setClusterMinSize(clusterMinSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getJoiningDistance()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:778
	// ("cv::xfeatures2d::PCTSignatures::getJoiningDistance", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_PCTSignatures_getJoiningDistance_const(const cv::xfeatures2d::PCTSignatures* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getJoiningDistance();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setJoiningDistance(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:784
	// ("cv::xfeatures2d::PCTSignatures::setJoiningDistance", vec![(pred!(mut, ["joiningDistance"], ["float"]), _)]),
	void cv_xfeatures2d_PCTSignatures_setJoiningDistance_float(cv::xfeatures2d::PCTSignatures* instance, float joiningDistance, ResultVoid* ocvrs_return) {
		try {
			instance->setJoiningDistance(joiningDistance);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDropThreshold()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:789
	// ("cv::xfeatures2d::PCTSignatures::getDropThreshold", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_PCTSignatures_getDropThreshold_const(const cv::xfeatures2d::PCTSignatures* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getDropThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDropThreshold(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:793
	// ("cv::xfeatures2d::PCTSignatures::setDropThreshold", vec![(pred!(mut, ["dropThreshold"], ["float"]), _)]),
	void cv_xfeatures2d_PCTSignatures_setDropThreshold_float(cv::xfeatures2d::PCTSignatures* instance, float dropThreshold, ResultVoid* ocvrs_return) {
		try {
			instance->setDropThreshold(dropThreshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDistanceFunction()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:798
	// ("cv::xfeatures2d::PCTSignatures::getDistanceFunction", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_PCTSignatures_getDistanceFunction_const(const cv::xfeatures2d::PCTSignatures* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getDistanceFunction();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDistanceFunction(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:803
	// ("cv::xfeatures2d::PCTSignatures::setDistanceFunction", vec![(pred!(mut, ["distanceFunction"], ["int"]), _)]),
	void cv_xfeatures2d_PCTSignatures_setDistanceFunction_int(cv::xfeatures2d::PCTSignatures* instance, int distanceFunction, ResultVoid* ocvrs_return) {
		try {
			instance->setDistanceFunction(distanceFunction);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::PCTSignatures::to_Algorithm() generated
	// ("cv::xfeatures2d::PCTSignatures::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_xfeatures2d_PCTSignatures_to_Algorithm(cv::xfeatures2d::PCTSignatures* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::xfeatures2d::PCTSignatures::delete() generated
	// ("cv::xfeatures2d::PCTSignatures::delete", vec![(pred!(mut, [], []), _)]),
	void cv_xfeatures2d_PCTSignatures_delete(cv::xfeatures2d::PCTSignatures* instance) {
			delete instance;
	}

	// create(const int, const int, const float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:828
	// ("cv::xfeatures2d::PCTSignaturesSQFD::create", vec![(pred!(mut, ["distanceFunction", "similarityFunction", "similarityParameter"], ["const int", "const int", "const float"]), _)]),
	void cv_xfeatures2d_PCTSignaturesSQFD_create_const_int_const_int_const_float(const int distanceFunction, const int similarityFunction, const float similarityParameter, Result<cv::Ptr<cv::xfeatures2d::PCTSignaturesSQFD>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::PCTSignaturesSQFD> ret = cv::xfeatures2d::PCTSignaturesSQFD::create(distanceFunction, similarityFunction, similarityParameter);
			Ok(new cv::Ptr<cv::xfeatures2d::PCTSignaturesSQFD>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::PCTSignaturesSQFD::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:828
	// ("cv::xfeatures2d::PCTSignaturesSQFD::create", vec![(pred!(mut, [], []), _)]),
	void cv_xfeatures2d_PCTSignaturesSQFD_create(Result<cv::Ptr<cv::xfeatures2d::PCTSignaturesSQFD>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::PCTSignaturesSQFD> ret = cv::xfeatures2d::PCTSignaturesSQFD::create();
			Ok(new cv::Ptr<cv::xfeatures2d::PCTSignaturesSQFD>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// computeQuadraticFormDistance(InputArray, InputArray)(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:838
	// ("cv::xfeatures2d::PCTSignaturesSQFD::computeQuadraticFormDistance", vec![(pred!(const, ["_signature0", "_signature1"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_xfeatures2d_PCTSignaturesSQFD_computeQuadraticFormDistance_const_const__InputArrayR_const__InputArrayR(const cv::xfeatures2d::PCTSignaturesSQFD* instance, const cv::_InputArray* _signature0, const cv::_InputArray* _signature1, Result<float>* ocvrs_return) {
		try {
			float ret = instance->computeQuadraticFormDistance(*_signature0, *_signature1);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// computeQuadraticFormDistances(const Mat &, const std::vector<Mat> &, std::vector<float> &)(TraitClass, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:849
	// ("cv::xfeatures2d::PCTSignaturesSQFD::computeQuadraticFormDistances", vec![(pred!(const, ["sourceSignature", "imageSignatures", "distances"], ["const cv::Mat*", "const std::vector<cv::Mat>*", "std::vector<float>*"]), _)]),
	void cv_xfeatures2d_PCTSignaturesSQFD_computeQuadraticFormDistances_const_const_MatR_const_vectorLMatGR_vectorLfloatGR(const cv::xfeatures2d::PCTSignaturesSQFD* instance, const cv::Mat* sourceSignature, const std::vector<cv::Mat>* imageSignatures, std::vector<float>* distances, ResultVoid* ocvrs_return) {
		try {
			instance->computeQuadraticFormDistances(*sourceSignature, *imageSignatures, *distances);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::PCTSignaturesSQFD::to_Algorithm() generated
	// ("cv::xfeatures2d::PCTSignaturesSQFD::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_xfeatures2d_PCTSignaturesSQFD_to_Algorithm(cv::xfeatures2d::PCTSignaturesSQFD* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::xfeatures2d::PCTSignaturesSQFD::delete() generated
	// ("cv::xfeatures2d::PCTSignaturesSQFD::delete", vec![(pred!(mut, [], []), _)]),
	void cv_xfeatures2d_PCTSignaturesSQFD_delete(cv::xfeatures2d::PCTSignaturesSQFD* instance) {
			delete instance;
	}

	// create(double, int, int, bool, bool)(Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/nonfree.hpp:94
	// ("cv::xfeatures2d::SURF::create", vec![(pred!(mut, ["hessianThreshold", "nOctaves", "nOctaveLayers", "extended", "upright"], ["double", "int", "int", "bool", "bool"]), _)]),
	void cv_xfeatures2d_SURF_create_double_int_int_bool_bool(double hessianThreshold, int nOctaves, int nOctaveLayers, bool extended, bool upright, Result<cv::Ptr<cv::xfeatures2d::SURF>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::SURF> ret = cv::xfeatures2d::SURF::create(hessianThreshold, nOctaves, nOctaveLayers, extended, upright);
			Ok(new cv::Ptr<cv::xfeatures2d::SURF>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::SURF::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/nonfree.hpp:94
	// ("cv::xfeatures2d::SURF::create", vec![(pred!(mut, [], []), _)]),
	void cv_xfeatures2d_SURF_create(Result<cv::Ptr<cv::xfeatures2d::SURF>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::SURF> ret = cv::xfeatures2d::SURF::create();
			Ok(new cv::Ptr<cv::xfeatures2d::SURF>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setHessianThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/nonfree.hpp:98
	// ("cv::xfeatures2d::SURF::setHessianThreshold", vec![(pred!(mut, ["hessianThreshold"], ["double"]), _)]),
	void cv_xfeatures2d_SURF_setHessianThreshold_double(cv::xfeatures2d::SURF* instance, double hessianThreshold, ResultVoid* ocvrs_return) {
		try {
			instance->setHessianThreshold(hessianThreshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getHessianThreshold()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/nonfree.hpp:99
	// ("cv::xfeatures2d::SURF::getHessianThreshold", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_SURF_getHessianThreshold_const(const cv::xfeatures2d::SURF* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getHessianThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNOctaves(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/nonfree.hpp:101
	// ("cv::xfeatures2d::SURF::setNOctaves", vec![(pred!(mut, ["nOctaves"], ["int"]), _)]),
	void cv_xfeatures2d_SURF_setNOctaves_int(cv::xfeatures2d::SURF* instance, int nOctaves, ResultVoid* ocvrs_return) {
		try {
			instance->setNOctaves(nOctaves);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNOctaves()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/nonfree.hpp:102
	// ("cv::xfeatures2d::SURF::getNOctaves", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_SURF_getNOctaves_const(const cv::xfeatures2d::SURF* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNOctaves();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNOctaveLayers(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/nonfree.hpp:104
	// ("cv::xfeatures2d::SURF::setNOctaveLayers", vec![(pred!(mut, ["nOctaveLayers"], ["int"]), _)]),
	void cv_xfeatures2d_SURF_setNOctaveLayers_int(cv::xfeatures2d::SURF* instance, int nOctaveLayers, ResultVoid* ocvrs_return) {
		try {
			instance->setNOctaveLayers(nOctaveLayers);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNOctaveLayers()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/nonfree.hpp:105
	// ("cv::xfeatures2d::SURF::getNOctaveLayers", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_SURF_getNOctaveLayers_const(const cv::xfeatures2d::SURF* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNOctaveLayers();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setExtended(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/nonfree.hpp:107
	// ("cv::xfeatures2d::SURF::setExtended", vec![(pred!(mut, ["extended"], ["bool"]), _)]),
	void cv_xfeatures2d_SURF_setExtended_bool(cv::xfeatures2d::SURF* instance, bool extended, ResultVoid* ocvrs_return) {
		try {
			instance->setExtended(extended);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getExtended()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/nonfree.hpp:108
	// ("cv::xfeatures2d::SURF::getExtended", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_SURF_getExtended_const(const cv::xfeatures2d::SURF* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getExtended();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setUpright(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/nonfree.hpp:110
	// ("cv::xfeatures2d::SURF::setUpright", vec![(pred!(mut, ["upright"], ["bool"]), _)]),
	void cv_xfeatures2d_SURF_setUpright_bool(cv::xfeatures2d::SURF* instance, bool upright, ResultVoid* ocvrs_return) {
		try {
			instance->setUpright(upright);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getUpright()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d/nonfree.hpp:111
	// ("cv::xfeatures2d::SURF::getUpright", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_SURF_getUpright_const(const cv::xfeatures2d::SURF* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUpright();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::SURF::to_Algorithm() generated
	// ("cv::xfeatures2d::SURF::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_xfeatures2d_SURF_to_Algorithm(cv::xfeatures2d::SURF* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::xfeatures2d::SURF::to_Feature2D() generated
	// ("cv::xfeatures2d::SURF::to_Feature2D", vec![(pred!(mut, [], []), _)]),
	cv::Feature2D* cv_xfeatures2d_SURF_to_Feature2D(cv::xfeatures2d::SURF* instance) {
			return dynamic_cast<cv::Feature2D*>(instance);
	}

	// cv::xfeatures2d::SURF::delete() generated
	// ("cv::xfeatures2d::SURF::delete", vec![(pred!(mut, [], []), _)]),
	void cv_xfeatures2d_SURF_delete(cv::xfeatures2d::SURF* instance) {
			delete instance;
	}

	// create(int, int, int, int, int)(Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:113
	// ("cv::xfeatures2d::StarDetector::create", vec![(pred!(mut, ["maxSize", "responseThreshold", "lineThresholdProjected", "lineThresholdBinarized", "suppressNonmaxSize"], ["int", "int", "int", "int", "int"]), _)]),
	void cv_xfeatures2d_StarDetector_create_int_int_int_int_int(int maxSize, int responseThreshold, int lineThresholdProjected, int lineThresholdBinarized, int suppressNonmaxSize, Result<cv::Ptr<cv::xfeatures2d::StarDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::StarDetector> ret = cv::xfeatures2d::StarDetector::create(maxSize, responseThreshold, lineThresholdProjected, lineThresholdBinarized, suppressNonmaxSize);
			Ok(new cv::Ptr<cv::xfeatures2d::StarDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::StarDetector::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:113
	// ("cv::xfeatures2d::StarDetector::create", vec![(pred!(mut, [], []), _)]),
	void cv_xfeatures2d_StarDetector_create(Result<cv::Ptr<cv::xfeatures2d::StarDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::StarDetector> ret = cv::xfeatures2d::StarDetector::create();
			Ok(new cv::Ptr<cv::xfeatures2d::StarDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::StarDetector::defaultNew() generated
	// ("cv::xfeatures2d::StarDetector::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::xfeatures2d::StarDetector* cv_xfeatures2d_StarDetector_defaultNew_const() {
			cv::xfeatures2d::StarDetector* ret = new cv::xfeatures2d::StarDetector();
			return ret;
	}

	// cv::xfeatures2d::StarDetector::to_Algorithm() generated
	// ("cv::xfeatures2d::StarDetector::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_xfeatures2d_StarDetector_to_Algorithm(cv::xfeatures2d::StarDetector* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::xfeatures2d::StarDetector::to_Feature2D() generated
	// ("cv::xfeatures2d::StarDetector::to_Feature2D", vec![(pred!(mut, [], []), _)]),
	cv::Feature2D* cv_xfeatures2d_StarDetector_to_Feature2D(cv::xfeatures2d::StarDetector* instance) {
			return dynamic_cast<cv::Feature2D*>(instance);
	}

	// cv::xfeatures2d::StarDetector::delete() generated
	// ("cv::xfeatures2d::StarDetector::delete", vec![(pred!(mut, [], []), _)]),
	void cv_xfeatures2d_StarDetector_delete(cv::xfeatures2d::StarDetector* instance) {
			delete instance;
	}

	// create(int, float, bool, bool, float, bool)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:316
	// ("cv::xfeatures2d::VGG::create", vec![(pred!(mut, ["desc", "isigma", "img_normalize", "use_scale_orientation", "scale_factor", "dsc_normalize"], ["int", "float", "bool", "bool", "float", "bool"]), _)]),
	void cv_xfeatures2d_VGG_create_int_float_bool_bool_float_bool(int desc, float isigma, bool img_normalize, bool use_scale_orientation, float scale_factor, bool dsc_normalize, Result<cv::Ptr<cv::xfeatures2d::VGG>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::VGG> ret = cv::xfeatures2d::VGG::create(desc, isigma, img_normalize, use_scale_orientation, scale_factor, dsc_normalize);
			Ok(new cv::Ptr<cv::xfeatures2d::VGG>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::VGG::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:316
	// ("cv::xfeatures2d::VGG::create", vec![(pred!(mut, [], []), _)]),
	void cv_xfeatures2d_VGG_create(Result<cv::Ptr<cv::xfeatures2d::VGG>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::VGG> ret = cv::xfeatures2d::VGG::create();
			Ok(new cv::Ptr<cv::xfeatures2d::VGG>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSigma(const float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:320
	// ("cv::xfeatures2d::VGG::setSigma", vec![(pred!(mut, ["isigma"], ["const float"]), _)]),
	void cv_xfeatures2d_VGG_setSigma_const_float(cv::xfeatures2d::VGG* instance, const float isigma, ResultVoid* ocvrs_return) {
		try {
			instance->setSigma(isigma);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSigma()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:321
	// ("cv::xfeatures2d::VGG::getSigma", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_VGG_getSigma_const(const cv::xfeatures2d::VGG* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getSigma();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setUseNormalizeImage(const bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:323
	// ("cv::xfeatures2d::VGG::setUseNormalizeImage", vec![(pred!(mut, ["img_normalize"], ["const bool"]), _)]),
	void cv_xfeatures2d_VGG_setUseNormalizeImage_const_bool(cv::xfeatures2d::VGG* instance, const bool img_normalize, ResultVoid* ocvrs_return) {
		try {
			instance->setUseNormalizeImage(img_normalize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getUseNormalizeImage()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:324
	// ("cv::xfeatures2d::VGG::getUseNormalizeImage", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_VGG_getUseNormalizeImage_const(const cv::xfeatures2d::VGG* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUseNormalizeImage();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setUseScaleOrientation(const bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:326
	// ("cv::xfeatures2d::VGG::setUseScaleOrientation", vec![(pred!(mut, ["use_scale_orientation"], ["const bool"]), _)]),
	void cv_xfeatures2d_VGG_setUseScaleOrientation_const_bool(cv::xfeatures2d::VGG* instance, const bool use_scale_orientation, ResultVoid* ocvrs_return) {
		try {
			instance->setUseScaleOrientation(use_scale_orientation);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getUseScaleOrientation()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:327
	// ("cv::xfeatures2d::VGG::getUseScaleOrientation", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_VGG_getUseScaleOrientation_const(const cv::xfeatures2d::VGG* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUseScaleOrientation();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setScaleFactor(const float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:329
	// ("cv::xfeatures2d::VGG::setScaleFactor", vec![(pred!(mut, ["scale_factor"], ["const float"]), _)]),
	void cv_xfeatures2d_VGG_setScaleFactor_const_float(cv::xfeatures2d::VGG* instance, const float scale_factor, ResultVoid* ocvrs_return) {
		try {
			instance->setScaleFactor(scale_factor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getScaleFactor()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:330
	// ("cv::xfeatures2d::VGG::getScaleFactor", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_VGG_getScaleFactor_const(const cv::xfeatures2d::VGG* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getScaleFactor();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setUseNormalizeDescriptor(const bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:332
	// ("cv::xfeatures2d::VGG::setUseNormalizeDescriptor", vec![(pred!(mut, ["dsc_normalize"], ["const bool"]), _)]),
	void cv_xfeatures2d_VGG_setUseNormalizeDescriptor_const_bool(cv::xfeatures2d::VGG* instance, const bool dsc_normalize, ResultVoid* ocvrs_return) {
		try {
			instance->setUseNormalizeDescriptor(dsc_normalize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getUseNormalizeDescriptor()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/xfeatures2d.hpp:333
	// ("cv::xfeatures2d::VGG::getUseNormalizeDescriptor", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_VGG_getUseNormalizeDescriptor_const(const cv::xfeatures2d::VGG* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUseNormalizeDescriptor();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::VGG::to_Algorithm() generated
	// ("cv::xfeatures2d::VGG::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_xfeatures2d_VGG_to_Algorithm(cv::xfeatures2d::VGG* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::xfeatures2d::VGG::to_Feature2D() generated
	// ("cv::xfeatures2d::VGG::to_Feature2D", vec![(pred!(mut, [], []), _)]),
	cv::Feature2D* cv_xfeatures2d_VGG_to_Feature2D(cv::xfeatures2d::VGG* instance) {
			return dynamic_cast<cv::Feature2D*>(instance);
	}

	// cv::xfeatures2d::VGG::delete() generated
	// ("cv::xfeatures2d::VGG::delete", vec![(pred!(mut, [], []), _)]),
	void cv_xfeatures2d_VGG_delete(cv::xfeatures2d::VGG* instance) {
			delete instance;
	}

}
