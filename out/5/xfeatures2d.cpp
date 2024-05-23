#include "xfeatures2d.hpp"
#include "xfeatures2d_types.hpp"

extern "C" {
	// cv::xfeatures2d::AGAST(InputArray, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1333
	// ("cv::xfeatures2d::AGAST", vec![(pred!(mut, ["image", "keypoints", "threshold"], ["const cv::_InputArray*", "std::vector<cv::KeyPoint>*", "int"]), _)]),
	void cv_xfeatures2d_AGAST_const__InputArrayR_vectorLKeyPointGR_int(const cv::_InputArray* image, std::vector<cv::KeyPoint>* keypoints, int threshold, ResultVoid* ocvrs_return) {
		try {
			cv::xfeatures2d::AGAST(*image, *keypoints, threshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// AGAST(InputArray, std::vector<KeyPoint> &, int, bool, AgastFeatureDetector::DetectorType)(InputArray, CppPassByVoidPtr, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1333
	// ("cv::xfeatures2d::AGAST", vec![(pred!(mut, ["image", "keypoints", "threshold", "nonmaxSuppression", "type"], ["const cv::_InputArray*", "std::vector<cv::KeyPoint>*", "int", "bool", "cv::xfeatures2d::AgastFeatureDetector::DetectorType"]), _)]),
	void cv_xfeatures2d_AGAST_const__InputArrayR_vectorLKeyPointGR_int_bool_DetectorType(const cv::_InputArray* image, std::vector<cv::KeyPoint>* keypoints, int threshold, bool nonmaxSuppression, cv::xfeatures2d::AgastFeatureDetector::DetectorType type, ResultVoid* ocvrs_return) {
		try {
			cv::xfeatures2d::AGAST(*image, *keypoints, threshold, nonmaxSuppression, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::FASTForPointSet(InputArray, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1480
	// ("cv::xfeatures2d::FASTForPointSet", vec![(pred!(mut, ["image", "keypoints", "threshold"], ["const cv::_InputArray*", "std::vector<cv::KeyPoint>*", "int"]), _)]),
	void cv_xfeatures2d_FASTForPointSet_const__InputArrayR_vectorLKeyPointGR_int(const cv::_InputArray* image, std::vector<cv::KeyPoint>* keypoints, int threshold, ResultVoid* ocvrs_return) {
		try {
			cv::xfeatures2d::FASTForPointSet(*image, *keypoints, threshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// FASTForPointSet(InputArray, std::vector<KeyPoint> &, int, bool, cv::FastFeatureDetector::DetectorType)(InputArray, CppPassByVoidPtr, Primitive, Primitive, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1480
	// ("cv::xfeatures2d::FASTForPointSet", vec![(pred!(mut, ["image", "keypoints", "threshold", "nonmaxSuppression", "type"], ["const cv::_InputArray*", "std::vector<cv::KeyPoint>*", "int", "bool", "cv::FastFeatureDetector::DetectorType"]), _)]),
	void cv_xfeatures2d_FASTForPointSet_const__InputArrayR_vectorLKeyPointGR_int_bool_DetectorType(const cv::_InputArray* image, std::vector<cv::KeyPoint>* keypoints, int threshold, bool nonmaxSuppression, cv::FastFeatureDetector::DetectorType type, ResultVoid* ocvrs_return) {
		try {
			cv::xfeatures2d::FASTForPointSet(*image, *keypoints, threshold, nonmaxSuppression, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::matchGMS(SimpleClass, SimpleClass, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1505
	// ("cv::xfeatures2d::matchGMS", vec![(pred!(mut, ["size1", "size2", "keypoints1", "keypoints2", "matches1to2", "matchesGMS"], ["const cv::Size*", "const cv::Size*", "const std::vector<cv::KeyPoint>*", "const std::vector<cv::KeyPoint>*", "const std::vector<cv::DMatch>*", "std::vector<cv::DMatch>*"]), _)]),
	void cv_xfeatures2d_matchGMS_const_SizeR_const_SizeR_const_vectorLKeyPointGR_const_vectorLKeyPointGR_const_vectorLDMatchGR_vectorLDMatchGR(const cv::Size* size1, const cv::Size* size2, const std::vector<cv::KeyPoint>* keypoints1, const std::vector<cv::KeyPoint>* keypoints2, const std::vector<cv::DMatch>* matches1to2, std::vector<cv::DMatch>* matchesGMS, ResultVoid* ocvrs_return) {
		try {
			cv::xfeatures2d::matchGMS(*size1, *size2, *keypoints1, *keypoints2, *matches1to2, *matchesGMS);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// matchGMS(const Size &, const Size &, const std::vector<KeyPoint> &, const std::vector<KeyPoint> &, const std::vector<DMatch> &, std::vector<DMatch> &, const bool, const bool, const double)(SimpleClass, SimpleClass, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1505
	// ("cv::xfeatures2d::matchGMS", vec![(pred!(mut, ["size1", "size2", "keypoints1", "keypoints2", "matches1to2", "matchesGMS", "withRotation", "withScale", "thresholdFactor"], ["const cv::Size*", "const cv::Size*", "const std::vector<cv::KeyPoint>*", "const std::vector<cv::KeyPoint>*", "const std::vector<cv::DMatch>*", "std::vector<cv::DMatch>*", "const bool", "const bool", "const double"]), _)]),
	void cv_xfeatures2d_matchGMS_const_SizeR_const_SizeR_const_vectorLKeyPointGR_const_vectorLKeyPointGR_const_vectorLDMatchGR_vectorLDMatchGR_const_bool_const_bool_const_double(const cv::Size* size1, const cv::Size* size2, const std::vector<cv::KeyPoint>* keypoints1, const std::vector<cv::KeyPoint>* keypoints2, const std::vector<cv::DMatch>* matches1to2, std::vector<cv::DMatch>* matchesGMS, const bool withRotation, const bool withScale, const double thresholdFactor, ResultVoid* ocvrs_return) {
		try {
			cv::xfeatures2d::matchGMS(*size1, *size2, *keypoints1, *keypoints2, *matches1to2, *matchesGMS, withRotation, withScale, thresholdFactor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// matchLOGOS(const std::vector<KeyPoint> &, const std::vector<KeyPoint> &, const std::vector<int> &, const std::vector<int> &, std::vector<DMatch> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1520
	// ("cv::xfeatures2d::matchLOGOS", vec![(pred!(mut, ["keypoints1", "keypoints2", "nn1", "nn2", "matches1to2"], ["const std::vector<cv::KeyPoint>*", "const std::vector<cv::KeyPoint>*", "const std::vector<int>*", "const std::vector<int>*", "std::vector<cv::DMatch>*"]), _)]),
	void cv_xfeatures2d_matchLOGOS_const_vectorLKeyPointGR_const_vectorLKeyPointGR_const_vectorLintGR_const_vectorLintGR_vectorLDMatchGR(const std::vector<cv::KeyPoint>* keypoints1, const std::vector<cv::KeyPoint>* keypoints2, const std::vector<int>* nn1, const std::vector<int>* nn2, std::vector<cv::DMatch>* matches1to2, ResultVoid* ocvrs_return) {
		try {
			cv::xfeatures2d::matchLOGOS(*keypoints1, *keypoints2, *nn1, *nn2, *matches1to2);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// SURF_CUDA()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:102
	// ("cv::cuda::SURF_CUDA::SURF_CUDA", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_SURF_CUDA_SURF_CUDA(Result<cv::cuda::SURF_CUDA*>* ocvrs_return) {
		try {
			cv::cuda::SURF_CUDA* ret = new cv::cuda::SURF_CUDA();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// SURF_CUDA(double, int, int, bool, float, bool)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:104
	// ("cv::cuda::SURF_CUDA::SURF_CUDA", vec![(pred!(mut, ["_hessianThreshold", "_nOctaves", "_nOctaveLayers", "_extended", "_keypointsRatio", "_upright"], ["double", "int", "int", "bool", "float", "bool"]), _)]),
	void cv_cuda_SURF_CUDA_SURF_CUDA_double_int_int_bool_float_bool(double _hessianThreshold, int _nOctaves, int _nOctaveLayers, bool _extended, float _keypointsRatio, bool _upright, Result<cv::cuda::SURF_CUDA*>* ocvrs_return) {
		try {
			cv::cuda::SURF_CUDA* ret = new cv::cuda::SURF_CUDA(_hessianThreshold, _nOctaves, _nOctaveLayers, _extended, _keypointsRatio, _upright);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::SURF_CUDA::SURF_CUDA(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:104
	// ("cv::cuda::SURF_CUDA::SURF_CUDA", vec![(pred!(mut, ["_hessianThreshold"], ["double"]), _)]),
	void cv_cuda_SURF_CUDA_SURF_CUDA_double(double _hessianThreshold, Result<cv::cuda::SURF_CUDA*>* ocvrs_return) {
		try {
			cv::cuda::SURF_CUDA* ret = new cv::cuda::SURF_CUDA(_hessianThreshold);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(double, int, int, bool, float, bool)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:117
	// ("cv::cuda::SURF_CUDA::create", vec![(pred!(mut, ["_hessianThreshold", "_nOctaves", "_nOctaveLayers", "_extended", "_keypointsRatio", "_upright"], ["double", "int", "int", "bool", "float", "bool"]), _)]),
	void cv_cuda_SURF_CUDA_create_double_int_int_bool_float_bool(double _hessianThreshold, int _nOctaves, int _nOctaveLayers, bool _extended, float _keypointsRatio, bool _upright, Result<cv::Ptr<cv::cuda::SURF_CUDA>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::SURF_CUDA> ret = cv::cuda::SURF_CUDA::create(_hessianThreshold, _nOctaves, _nOctaveLayers, _extended, _keypointsRatio, _upright);
			Ok(new cv::Ptr<cv::cuda::SURF_CUDA>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::SURF_CUDA::create(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:117
	// ("cv::cuda::SURF_CUDA::create", vec![(pred!(mut, ["_hessianThreshold"], ["double"]), _)]),
	void cv_cuda_SURF_CUDA_create_double(double _hessianThreshold, Result<cv::Ptr<cv::cuda::SURF_CUDA>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::SURF_CUDA> ret = cv::cuda::SURF_CUDA::create(_hessianThreshold);
			Ok(new cv::Ptr<cv::cuda::SURF_CUDA>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// descriptorSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:121
	// ("cv::cuda::SURF_CUDA::descriptorSize", vec![(pred!(const, [], []), _)]),
	void cv_cuda_SURF_CUDA_descriptorSize_const(const cv::cuda::SURF_CUDA* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->descriptorSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// defaultNorm()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:123
	// ("cv::cuda::SURF_CUDA::defaultNorm", vec![(pred!(const, [], []), _)]),
	void cv_cuda_SURF_CUDA_defaultNorm_const(const cv::cuda::SURF_CUDA* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->defaultNorm();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// uploadKeypoints(const std::vector<KeyPoint> &, GpuMat &)(CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:126
	// ("cv::cuda::SURF_CUDA::uploadKeypoints", vec![(pred!(mut, ["keypoints", "keypointsGPU"], ["const std::vector<cv::KeyPoint>*", "cv::cuda::GpuMat*"]), _)]),
	void cv_cuda_SURF_CUDA_uploadKeypoints_const_vectorLKeyPointGR_GpuMatR(cv::cuda::SURF_CUDA* instance, const std::vector<cv::KeyPoint>* keypoints, cv::cuda::GpuMat* keypointsGPU, ResultVoid* ocvrs_return) {
		try {
			instance->uploadKeypoints(*keypoints, *keypointsGPU);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// downloadKeypoints(const GpuMat &, std::vector<KeyPoint> &)(TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:128
	// ("cv::cuda::SURF_CUDA::downloadKeypoints", vec![(pred!(mut, ["keypointsGPU", "keypoints"], ["const cv::cuda::GpuMat*", "std::vector<cv::KeyPoint>*"]), _)]),
	void cv_cuda_SURF_CUDA_downloadKeypoints_const_GpuMatR_vectorLKeyPointGR(cv::cuda::SURF_CUDA* instance, const cv::cuda::GpuMat* keypointsGPU, std::vector<cv::KeyPoint>* keypoints, ResultVoid* ocvrs_return) {
		try {
			instance->downloadKeypoints(*keypointsGPU, *keypoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// downloadDescriptors(const GpuMat &, std::vector<float> &)(TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:131
	// ("cv::cuda::SURF_CUDA::downloadDescriptors", vec![(pred!(mut, ["descriptorsGPU", "descriptors"], ["const cv::cuda::GpuMat*", "std::vector<float>*"]), _)]),
	void cv_cuda_SURF_CUDA_downloadDescriptors_const_GpuMatR_vectorLfloatGR(cv::cuda::SURF_CUDA* instance, const cv::cuda::GpuMat* descriptorsGPU, std::vector<float>* descriptors, ResultVoid* ocvrs_return) {
		try {
			instance->downloadDescriptors(*descriptorsGPU, *descriptors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator()(const GpuMat &, const GpuMat &, GpuMat &)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:143
	// ("cv::cuda::SURF_CUDA::operator()", vec![(pred!(mut, ["img", "mask", "keypoints"], ["const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "cv::cuda::GpuMat*"]), _)]),
	void cv_cuda_SURF_CUDA_operator___const_GpuMatR_const_GpuMatR_GpuMatR(cv::cuda::SURF_CUDA* instance, const cv::cuda::GpuMat* img, const cv::cuda::GpuMat* mask, cv::cuda::GpuMat* keypoints, ResultVoid* ocvrs_return) {
		try {
			instance->operator()(*img, *mask, *keypoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator()(const GpuMat &, const GpuMat &, GpuMat &, GpuMat &, bool)(TraitClass, TraitClass, TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:146
	// ("cv::cuda::SURF_CUDA::operator()", vec![(pred!(mut, ["img", "mask", "keypoints", "descriptors", "useProvidedKeypoints"], ["const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "bool"]), _)]),
	void cv_cuda_SURF_CUDA_operator___const_GpuMatR_const_GpuMatR_GpuMatR_GpuMatR_bool(cv::cuda::SURF_CUDA* instance, const cv::cuda::GpuMat* img, const cv::cuda::GpuMat* mask, cv::cuda::GpuMat* keypoints, cv::cuda::GpuMat* descriptors, bool useProvidedKeypoints, ResultVoid* ocvrs_return) {
		try {
			instance->operator()(*img, *mask, *keypoints, *descriptors, useProvidedKeypoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::SURF_CUDA::operator()(TraitClass, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:146
	// ("cv::cuda::SURF_CUDA::operator()", vec![(pred!(mut, ["img", "mask", "keypoints", "descriptors"], ["const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*"]), _)]),
	void cv_cuda_SURF_CUDA_operator___const_GpuMatR_const_GpuMatR_GpuMatR_GpuMatR(cv::cuda::SURF_CUDA* instance, const cv::cuda::GpuMat* img, const cv::cuda::GpuMat* mask, cv::cuda::GpuMat* keypoints, cv::cuda::GpuMat* descriptors, ResultVoid* ocvrs_return) {
		try {
			instance->operator()(*img, *mask, *keypoints, *descriptors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detect(const GpuMat &, const GpuMat &, GpuMat &)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:155
	// ("cv::cuda::SURF_CUDA::detect", vec![(pred!(mut, ["img", "mask", "keypoints"], ["const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "cv::cuda::GpuMat*"]), _)]),
	void cv_cuda_SURF_CUDA_detect_const_GpuMatR_const_GpuMatR_GpuMatR(cv::cuda::SURF_CUDA* instance, const cv::cuda::GpuMat* img, const cv::cuda::GpuMat* mask, cv::cuda::GpuMat* keypoints, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*img, *mask, *keypoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator()(const GpuMat &, const GpuMat &, std::vector<KeyPoint> &)(TraitClass, TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:159
	// ("cv::cuda::SURF_CUDA::operator()", vec![(pred!(mut, ["img", "mask", "keypoints"], ["const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "std::vector<cv::KeyPoint>*"]), _)]),
	void cv_cuda_SURF_CUDA_operator___const_GpuMatR_const_GpuMatR_vectorLKeyPointGR(cv::cuda::SURF_CUDA* instance, const cv::cuda::GpuMat* img, const cv::cuda::GpuMat* mask, std::vector<cv::KeyPoint>* keypoints, ResultVoid* ocvrs_return) {
		try {
			instance->operator()(*img, *mask, *keypoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator()(const GpuMat &, const GpuMat &, std::vector<KeyPoint> &, GpuMat &, bool)(TraitClass, TraitClass, CppPassByVoidPtr, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:160
	// ("cv::cuda::SURF_CUDA::operator()", vec![(pred!(mut, ["img", "mask", "keypoints", "descriptors", "useProvidedKeypoints"], ["const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "std::vector<cv::KeyPoint>*", "cv::cuda::GpuMat*", "bool"]), _)]),
	void cv_cuda_SURF_CUDA_operator___const_GpuMatR_const_GpuMatR_vectorLKeyPointGR_GpuMatR_bool(cv::cuda::SURF_CUDA* instance, const cv::cuda::GpuMat* img, const cv::cuda::GpuMat* mask, std::vector<cv::KeyPoint>* keypoints, cv::cuda::GpuMat* descriptors, bool useProvidedKeypoints, ResultVoid* ocvrs_return) {
		try {
			instance->operator()(*img, *mask, *keypoints, *descriptors, useProvidedKeypoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::SURF_CUDA::operator()(TraitClass, TraitClass, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:160
	// ("cv::cuda::SURF_CUDA::operator()", vec![(pred!(mut, ["img", "mask", "keypoints", "descriptors"], ["const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "std::vector<cv::KeyPoint>*", "cv::cuda::GpuMat*"]), _)]),
	void cv_cuda_SURF_CUDA_operator___const_GpuMatR_const_GpuMatR_vectorLKeyPointGR_GpuMatR(cv::cuda::SURF_CUDA* instance, const cv::cuda::GpuMat* img, const cv::cuda::GpuMat* mask, std::vector<cv::KeyPoint>* keypoints, cv::cuda::GpuMat* descriptors, ResultVoid* ocvrs_return) {
		try {
			instance->operator()(*img, *mask, *keypoints, *descriptors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectWithDescriptors(const GpuMat &, const GpuMat &, GpuMat &, GpuMat &, bool)(TraitClass, TraitClass, TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:171
	// ("cv::cuda::SURF_CUDA::detectWithDescriptors", vec![(pred!(mut, ["img", "mask", "keypoints", "descriptors", "useProvidedKeypoints"], ["const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "bool"]), _)]),
	void cv_cuda_SURF_CUDA_detectWithDescriptors_const_GpuMatR_const_GpuMatR_GpuMatR_GpuMatR_bool(cv::cuda::SURF_CUDA* instance, const cv::cuda::GpuMat* img, const cv::cuda::GpuMat* mask, cv::cuda::GpuMat* keypoints, cv::cuda::GpuMat* descriptors, bool useProvidedKeypoints, ResultVoid* ocvrs_return) {
		try {
			instance->detectWithDescriptors(*img, *mask, *keypoints, *descriptors, useProvidedKeypoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::SURF_CUDA::detectWithDescriptors(TraitClass, TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:171
	// ("cv::cuda::SURF_CUDA::detectWithDescriptors", vec![(pred!(mut, ["img", "mask", "keypoints", "descriptors"], ["const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::GpuMat*"]), _)]),
	void cv_cuda_SURF_CUDA_detectWithDescriptors_const_GpuMatR_const_GpuMatR_GpuMatR_GpuMatR(cv::cuda::SURF_CUDA* instance, const cv::cuda::GpuMat* img, const cv::cuda::GpuMat* mask, cv::cuda::GpuMat* keypoints, cv::cuda::GpuMat* descriptors, ResultVoid* ocvrs_return) {
		try {
			instance->detectWithDescriptors(*img, *mask, *keypoints, *descriptors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator()(const GpuMat &, const GpuMat &, std::vector<KeyPoint> &, std::vector<float> &, bool)(TraitClass, TraitClass, CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:176
	// ("cv::cuda::SURF_CUDA::operator()", vec![(pred!(mut, ["img", "mask", "keypoints", "descriptors", "useProvidedKeypoints"], ["const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "std::vector<cv::KeyPoint>*", "std::vector<float>*", "bool"]), _)]),
	void cv_cuda_SURF_CUDA_operator___const_GpuMatR_const_GpuMatR_vectorLKeyPointGR_vectorLfloatGR_bool(cv::cuda::SURF_CUDA* instance, const cv::cuda::GpuMat* img, const cv::cuda::GpuMat* mask, std::vector<cv::KeyPoint>* keypoints, std::vector<float>* descriptors, bool useProvidedKeypoints, ResultVoid* ocvrs_return) {
		try {
			instance->operator()(*img, *mask, *keypoints, *descriptors, useProvidedKeypoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::SURF_CUDA::operator()(TraitClass, TraitClass, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:176
	// ("cv::cuda::SURF_CUDA::operator()", vec![(pred!(mut, ["img", "mask", "keypoints", "descriptors"], ["const cv::cuda::GpuMat*", "const cv::cuda::GpuMat*", "std::vector<cv::KeyPoint>*", "std::vector<float>*"]), _)]),
	void cv_cuda_SURF_CUDA_operator___const_GpuMatR_const_GpuMatR_vectorLKeyPointGR_vectorLfloatGR(cv::cuda::SURF_CUDA* instance, const cv::cuda::GpuMat* img, const cv::cuda::GpuMat* mask, std::vector<cv::KeyPoint>* keypoints, std::vector<float>* descriptors, ResultVoid* ocvrs_return) {
		try {
			instance->operator()(*img, *mask, *keypoints, *descriptors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// releaseMemory()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:179
	// ("cv::cuda::SURF_CUDA::releaseMemory", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_SURF_CUDA_releaseMemory(cv::cuda::SURF_CUDA* instance, ResultVoid* ocvrs_return) {
		try {
			instance->releaseMemory();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::SURF_CUDA::hessianThreshold() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:182
	// ("cv::cuda::SURF_CUDA::hessianThreshold", vec![(pred!(const, [], []), _)]),
	double cv_cuda_SURF_CUDA_propHessianThreshold_const(const cv::cuda::SURF_CUDA* instance) {
			double ret = instance->hessianThreshold;
			return ret;
	}

	// cv::cuda::SURF_CUDA::setHessianThreshold(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:182
	// ("cv::cuda::SURF_CUDA::setHessianThreshold", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_cuda_SURF_CUDA_propHessianThreshold_const_double(cv::cuda::SURF_CUDA* instance, const double val) {
			instance->hessianThreshold = val;
	}

	// cv::cuda::SURF_CUDA::nOctaves() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:183
	// ("cv::cuda::SURF_CUDA::nOctaves", vec![(pred!(const, [], []), _)]),
	int cv_cuda_SURF_CUDA_propNOctaves_const(const cv::cuda::SURF_CUDA* instance) {
			int ret = instance->nOctaves;
			return ret;
	}

	// cv::cuda::SURF_CUDA::setNOctaves(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:183
	// ("cv::cuda::SURF_CUDA::setNOctaves", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_cuda_SURF_CUDA_propNOctaves_const_int(cv::cuda::SURF_CUDA* instance, const int val) {
			instance->nOctaves = val;
	}

	// cv::cuda::SURF_CUDA::nOctaveLayers() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:184
	// ("cv::cuda::SURF_CUDA::nOctaveLayers", vec![(pred!(const, [], []), _)]),
	int cv_cuda_SURF_CUDA_propNOctaveLayers_const(const cv::cuda::SURF_CUDA* instance) {
			int ret = instance->nOctaveLayers;
			return ret;
	}

	// cv::cuda::SURF_CUDA::setNOctaveLayers(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:184
	// ("cv::cuda::SURF_CUDA::setNOctaveLayers", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_cuda_SURF_CUDA_propNOctaveLayers_const_int(cv::cuda::SURF_CUDA* instance, const int val) {
			instance->nOctaveLayers = val;
	}

	// cv::cuda::SURF_CUDA::extended() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:185
	// ("cv::cuda::SURF_CUDA::extended", vec![(pred!(const, [], []), _)]),
	bool cv_cuda_SURF_CUDA_propExtended_const(const cv::cuda::SURF_CUDA* instance) {
			bool ret = instance->extended;
			return ret;
	}

	// cv::cuda::SURF_CUDA::setExtended(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:185
	// ("cv::cuda::SURF_CUDA::setExtended", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_cuda_SURF_CUDA_propExtended_const_bool(cv::cuda::SURF_CUDA* instance, const bool val) {
			instance->extended = val;
	}

	// cv::cuda::SURF_CUDA::upright() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:186
	// ("cv::cuda::SURF_CUDA::upright", vec![(pred!(const, [], []), _)]),
	bool cv_cuda_SURF_CUDA_propUpright_const(const cv::cuda::SURF_CUDA* instance) {
			bool ret = instance->upright;
			return ret;
	}

	// cv::cuda::SURF_CUDA::setUpright(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:186
	// ("cv::cuda::SURF_CUDA::setUpright", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_cuda_SURF_CUDA_propUpright_const_bool(cv::cuda::SURF_CUDA* instance, const bool val) {
			instance->upright = val;
	}

	// cv::cuda::SURF_CUDA::keypointsRatio() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:189
	// ("cv::cuda::SURF_CUDA::keypointsRatio", vec![(pred!(const, [], []), _)]),
	float cv_cuda_SURF_CUDA_propKeypointsRatio_const(const cv::cuda::SURF_CUDA* instance) {
			float ret = instance->keypointsRatio;
			return ret;
	}

	// cv::cuda::SURF_CUDA::setKeypointsRatio(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:189
	// ("cv::cuda::SURF_CUDA::setKeypointsRatio", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_cuda_SURF_CUDA_propKeypointsRatio_const_float(cv::cuda::SURF_CUDA* instance, const float val) {
			instance->keypointsRatio = val;
	}

	// cv::cuda::SURF_CUDA::sum() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:191
	// ("cv::cuda::SURF_CUDA::sum", vec![(pred!(const, [], []), _)]),
	cv::cuda::GpuMat* cv_cuda_SURF_CUDA_propSum_const(const cv::cuda::SURF_CUDA* instance) {
			cv::cuda::GpuMat ret = instance->sum;
			return new cv::cuda::GpuMat(ret);
	}

	// cv::cuda::SURF_CUDA::setSum(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:191
	// ("cv::cuda::SURF_CUDA::setSum", vec![(pred!(mut, ["val"], ["const cv::cuda::GpuMat"]), _)]),
	void cv_cuda_SURF_CUDA_propSum_const_GpuMat(cv::cuda::SURF_CUDA* instance, const cv::cuda::GpuMat* val) {
			instance->sum = *val;
	}

	// cv::cuda::SURF_CUDA::mask1() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:191
	// ("cv::cuda::SURF_CUDA::mask1", vec![(pred!(const, [], []), _)]),
	cv::cuda::GpuMat* cv_cuda_SURF_CUDA_propMask1_const(const cv::cuda::SURF_CUDA* instance) {
			cv::cuda::GpuMat ret = instance->mask1;
			return new cv::cuda::GpuMat(ret);
	}

	// cv::cuda::SURF_CUDA::setMask1(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:191
	// ("cv::cuda::SURF_CUDA::setMask1", vec![(pred!(mut, ["val"], ["const cv::cuda::GpuMat"]), _)]),
	void cv_cuda_SURF_CUDA_propMask1_const_GpuMat(cv::cuda::SURF_CUDA* instance, const cv::cuda::GpuMat* val) {
			instance->mask1 = *val;
	}

	// cv::cuda::SURF_CUDA::maskSum() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:191
	// ("cv::cuda::SURF_CUDA::maskSum", vec![(pred!(const, [], []), _)]),
	cv::cuda::GpuMat* cv_cuda_SURF_CUDA_propMaskSum_const(const cv::cuda::SURF_CUDA* instance) {
			cv::cuda::GpuMat ret = instance->maskSum;
			return new cv::cuda::GpuMat(ret);
	}

	// cv::cuda::SURF_CUDA::setMaskSum(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:191
	// ("cv::cuda::SURF_CUDA::setMaskSum", vec![(pred!(mut, ["val"], ["const cv::cuda::GpuMat"]), _)]),
	void cv_cuda_SURF_CUDA_propMaskSum_const_GpuMat(cv::cuda::SURF_CUDA* instance, const cv::cuda::GpuMat* val) {
			instance->maskSum = *val;
	}

	// cv::cuda::SURF_CUDA::det() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:193
	// ("cv::cuda::SURF_CUDA::det", vec![(pred!(const, [], []), _)]),
	cv::cuda::GpuMat* cv_cuda_SURF_CUDA_propDet_const(const cv::cuda::SURF_CUDA* instance) {
			cv::cuda::GpuMat ret = instance->det;
			return new cv::cuda::GpuMat(ret);
	}

	// cv::cuda::SURF_CUDA::setDet(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:193
	// ("cv::cuda::SURF_CUDA::setDet", vec![(pred!(mut, ["val"], ["const cv::cuda::GpuMat"]), _)]),
	void cv_cuda_SURF_CUDA_propDet_const_GpuMat(cv::cuda::SURF_CUDA* instance, const cv::cuda::GpuMat* val) {
			instance->det = *val;
	}

	// cv::cuda::SURF_CUDA::trace() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:193
	// ("cv::cuda::SURF_CUDA::trace", vec![(pred!(const, [], []), _)]),
	cv::cuda::GpuMat* cv_cuda_SURF_CUDA_propTrace_const(const cv::cuda::SURF_CUDA* instance) {
			cv::cuda::GpuMat ret = instance->trace;
			return new cv::cuda::GpuMat(ret);
	}

	// cv::cuda::SURF_CUDA::setTrace(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:193
	// ("cv::cuda::SURF_CUDA::setTrace", vec![(pred!(mut, ["val"], ["const cv::cuda::GpuMat"]), _)]),
	void cv_cuda_SURF_CUDA_propTrace_const_GpuMat(cv::cuda::SURF_CUDA* instance, const cv::cuda::GpuMat* val) {
			instance->trace = *val;
	}

	// cv::cuda::SURF_CUDA::maxPosBuffer() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:195
	// ("cv::cuda::SURF_CUDA::maxPosBuffer", vec![(pred!(const, [], []), _)]),
	cv::cuda::GpuMat* cv_cuda_SURF_CUDA_propMaxPosBuffer_const(const cv::cuda::SURF_CUDA* instance) {
			cv::cuda::GpuMat ret = instance->maxPosBuffer;
			return new cv::cuda::GpuMat(ret);
	}

	// cv::cuda::SURF_CUDA::setMaxPosBuffer(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/cuda.hpp:195
	// ("cv::cuda::SURF_CUDA::setMaxPosBuffer", vec![(pred!(mut, ["val"], ["const cv::cuda::GpuMat"]), _)]),
	void cv_cuda_SURF_CUDA_propMaxPosBuffer_const_GpuMat(cv::cuda::SURF_CUDA* instance, const cv::cuda::GpuMat* val) {
			instance->maxPosBuffer = *val;
	}

	// cv::cuda::SURF_CUDA::delete() generated
	// ("cv::cuda::SURF_CUDA::delete", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_SURF_CUDA_delete(cv::cuda::SURF_CUDA* instance) {
			delete instance;
	}

	// create(int, int, int, float, int, int, int, int)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1433
	// ("cv::xfeatures2d::AKAZE::create", vec![(pred!(mut, ["descriptor_type", "descriptor_size", "descriptor_channels", "threshold", "nOctaves", "nOctaveLayers", "diffusivity", "max_points"], ["int", "int", "int", "float", "int", "int", "int", "int"]), _)]),
	void cv_xfeatures2d_AKAZE_create_int_int_int_float_int_int_int_int(int descriptor_type, int descriptor_size, int descriptor_channels, float threshold, int nOctaves, int nOctaveLayers, int diffusivity, int max_points, Result<cv::Ptr<cv::xfeatures2d::AKAZE>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::AKAZE> ret = cv::xfeatures2d::AKAZE::create(descriptor_type, descriptor_size, descriptor_channels, threshold, nOctaves, nOctaveLayers, diffusivity, max_points);
			Ok(new cv::Ptr<cv::xfeatures2d::AKAZE>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::AKAZE::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1433
	// ("cv::xfeatures2d::AKAZE::create", vec![(pred!(mut, [], []), _)]),
	void cv_xfeatures2d_AKAZE_create(Result<cv::Ptr<cv::xfeatures2d::AKAZE>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::AKAZE> ret = cv::xfeatures2d::AKAZE::create();
			Ok(new cv::Ptr<cv::xfeatures2d::AKAZE>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDescriptorType(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1439
	// ("cv::xfeatures2d::AKAZE::setDescriptorType", vec![(pred!(mut, ["dtype"], ["int"]), _)]),
	void cv_xfeatures2d_AKAZE_setDescriptorType_int(cv::xfeatures2d::AKAZE* instance, int dtype, ResultVoid* ocvrs_return) {
		try {
			instance->setDescriptorType(dtype);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDescriptorType()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1440
	// ("cv::xfeatures2d::AKAZE::getDescriptorType", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_AKAZE_getDescriptorType_const(const cv::xfeatures2d::AKAZE* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getDescriptorType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDescriptorSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1442
	// ("cv::xfeatures2d::AKAZE::setDescriptorSize", vec![(pred!(mut, ["dsize"], ["int"]), _)]),
	void cv_xfeatures2d_AKAZE_setDescriptorSize_int(cv::xfeatures2d::AKAZE* instance, int dsize, ResultVoid* ocvrs_return) {
		try {
			instance->setDescriptorSize(dsize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDescriptorSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1443
	// ("cv::xfeatures2d::AKAZE::getDescriptorSize", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_AKAZE_getDescriptorSize_const(const cv::xfeatures2d::AKAZE* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getDescriptorSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDescriptorChannels(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1445
	// ("cv::xfeatures2d::AKAZE::setDescriptorChannels", vec![(pred!(mut, ["dch"], ["int"]), _)]),
	void cv_xfeatures2d_AKAZE_setDescriptorChannels_int(cv::xfeatures2d::AKAZE* instance, int dch, ResultVoid* ocvrs_return) {
		try {
			instance->setDescriptorChannels(dch);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDescriptorChannels()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1446
	// ("cv::xfeatures2d::AKAZE::getDescriptorChannels", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_AKAZE_getDescriptorChannels_const(const cv::xfeatures2d::AKAZE* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getDescriptorChannels();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1448
	// ("cv::xfeatures2d::AKAZE::setThreshold", vec![(pred!(mut, ["threshold"], ["double"]), _)]),
	void cv_xfeatures2d_AKAZE_setThreshold_double(cv::xfeatures2d::AKAZE* instance, double threshold, ResultVoid* ocvrs_return) {
		try {
			instance->setThreshold(threshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1449
	// ("cv::xfeatures2d::AKAZE::getThreshold", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_AKAZE_getThreshold_const(const cv::xfeatures2d::AKAZE* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNOctaves(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1451
	// ("cv::xfeatures2d::AKAZE::setNOctaves", vec![(pred!(mut, ["octaves"], ["int"]), _)]),
	void cv_xfeatures2d_AKAZE_setNOctaves_int(cv::xfeatures2d::AKAZE* instance, int octaves, ResultVoid* ocvrs_return) {
		try {
			instance->setNOctaves(octaves);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNOctaves()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1452
	// ("cv::xfeatures2d::AKAZE::getNOctaves", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_AKAZE_getNOctaves_const(const cv::xfeatures2d::AKAZE* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNOctaves();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNOctaveLayers(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1454
	// ("cv::xfeatures2d::AKAZE::setNOctaveLayers", vec![(pred!(mut, ["octaveLayers"], ["int"]), _)]),
	void cv_xfeatures2d_AKAZE_setNOctaveLayers_int(cv::xfeatures2d::AKAZE* instance, int octaveLayers, ResultVoid* ocvrs_return) {
		try {
			instance->setNOctaveLayers(octaveLayers);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNOctaveLayers()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1455
	// ("cv::xfeatures2d::AKAZE::getNOctaveLayers", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_AKAZE_getNOctaveLayers_const(const cv::xfeatures2d::AKAZE* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNOctaveLayers();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDiffusivity(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1457
	// ("cv::xfeatures2d::AKAZE::setDiffusivity", vec![(pred!(mut, ["diff"], ["int"]), _)]),
	void cv_xfeatures2d_AKAZE_setDiffusivity_int(cv::xfeatures2d::AKAZE* instance, int diff, ResultVoid* ocvrs_return) {
		try {
			instance->setDiffusivity(diff);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDiffusivity()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1458
	// ("cv::xfeatures2d::AKAZE::getDiffusivity", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_AKAZE_getDiffusivity_const(const cv::xfeatures2d::AKAZE* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getDiffusivity();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1459
	// ("cv::xfeatures2d::AKAZE::getDefaultName", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_AKAZE_getDefaultName_const(const cv::xfeatures2d::AKAZE* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getDefaultName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxPoints(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1461
	// ("cv::xfeatures2d::AKAZE::setMaxPoints", vec![(pred!(mut, ["max_points"], ["int"]), _)]),
	void cv_xfeatures2d_AKAZE_setMaxPoints_int(cv::xfeatures2d::AKAZE* instance, int max_points, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxPoints(max_points);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxPoints()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1462
	// ("cv::xfeatures2d::AKAZE::getMaxPoints", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_AKAZE_getMaxPoints_const(const cv::xfeatures2d::AKAZE* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMaxPoints();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::AKAZE::to_Algorithm() generated
	// ("cv::xfeatures2d::AKAZE::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_xfeatures2d_AKAZE_to_Algorithm(cv::xfeatures2d::AKAZE* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::xfeatures2d::AKAZE::to_Feature2D() generated
	// ("cv::xfeatures2d::AKAZE::to_Feature2D", vec![(pred!(mut, [], []), _)]),
	cv::Feature2D* cv_xfeatures2d_AKAZE_to_Feature2D(cv::xfeatures2d::AKAZE* instance) {
			return dynamic_cast<cv::Feature2D*>(instance);
	}

	// cv::xfeatures2d::AKAZE::delete() generated
	// ("cv::xfeatures2d::AKAZE::delete", vec![(pred!(mut, [], []), _)]),
	void cv_xfeatures2d_AKAZE_delete(cv::xfeatures2d::AKAZE* instance) {
			delete instance;
	}

	// create(Ptr<FeatureDetector>, Ptr<DescriptorExtractor>)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1146
	// ("cv::xfeatures2d::AffineFeature2D::create", vec![(pred!(mut, ["keypoint_detector", "descriptor_extractor"], ["cv::Ptr<cv::Feature2D>", "cv::Ptr<cv::Feature2D>"]), _)]),
	void cv_xfeatures2d_AffineFeature2D_create_PtrLFeature2DG_PtrLFeature2DG(cv::Ptr<cv::Feature2D>* keypoint_detector, cv::Ptr<cv::Feature2D>* descriptor_extractor, Result<cv::Ptr<cv::xfeatures2d::AffineFeature2D>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::AffineFeature2D> ret = cv::xfeatures2d::AffineFeature2D::create(*keypoint_detector, *descriptor_extractor);
			Ok(new cv::Ptr<cv::xfeatures2d::AffineFeature2D>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(Ptr<FeatureDetector>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1154
	// ("cv::xfeatures2d::AffineFeature2D::create", vec![(pred!(mut, ["keypoint_detector"], ["cv::Ptr<cv::Feature2D>"]), _)]),
	void cv_xfeatures2d_AffineFeature2D_create_PtrLFeature2DG(cv::Ptr<cv::Feature2D>* keypoint_detector, Result<cv::Ptr<cv::xfeatures2d::AffineFeature2D>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::AffineFeature2D> ret = cv::xfeatures2d::AffineFeature2D::create(*keypoint_detector);
			Ok(new cv::Ptr<cv::xfeatures2d::AffineFeature2D>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detect(InputArray, std::vector<Elliptic_KeyPoint> &, InputArray)(InputArray, CppPassByVoidPtr, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1165
	// ("cv::xfeatures2d::AffineFeature2D::detect", vec![(pred!(mut, ["image", "keypoints", "mask"], ["const cv::_InputArray*", "std::vector<cv::xfeatures2d::Elliptic_KeyPoint>*", "const cv::_InputArray*"]), _)]),
	void cv_xfeatures2d_AffineFeature2D_detect_const__InputArrayR_vectorLElliptic_KeyPointGR_const__InputArrayR(cv::xfeatures2d::AffineFeature2D* instance, const cv::_InputArray* image, std::vector<cv::xfeatures2d::Elliptic_KeyPoint>* keypoints, const cv::_InputArray* mask, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*image, *keypoints, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::AffineFeature2D::detect(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1165
	// ("cv::xfeatures2d::AffineFeature2D::detect", vec![(pred!(mut, ["image", "keypoints"], ["const cv::_InputArray*", "std::vector<cv::xfeatures2d::Elliptic_KeyPoint>*"]), _)]),
	void cv_xfeatures2d_AffineFeature2D_detect_const__InputArrayR_vectorLElliptic_KeyPointGR(cv::xfeatures2d::AffineFeature2D* instance, const cv::_InputArray* image, std::vector<cv::xfeatures2d::Elliptic_KeyPoint>* keypoints, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*image, *keypoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectAndCompute(InputArray, InputArray, std::vector<Elliptic_KeyPoint> &, OutputArray, bool)(InputArray, InputArray, CppPassByVoidPtr, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1175
	// ("cv::xfeatures2d::AffineFeature2D::detectAndCompute", vec![(pred!(mut, ["image", "mask", "keypoints", "descriptors", "useProvidedKeypoints"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<cv::xfeatures2d::Elliptic_KeyPoint>*", "const cv::_OutputArray*", "bool"]), _)]),
	void cv_xfeatures2d_AffineFeature2D_detectAndCompute_const__InputArrayR_const__InputArrayR_vectorLElliptic_KeyPointGR_const__OutputArrayR_bool(cv::xfeatures2d::AffineFeature2D* instance, const cv::_InputArray* image, const cv::_InputArray* mask, std::vector<cv::xfeatures2d::Elliptic_KeyPoint>* keypoints, const cv::_OutputArray* descriptors, bool useProvidedKeypoints, ResultVoid* ocvrs_return) {
		try {
			instance->detectAndCompute(*image, *mask, *keypoints, *descriptors, useProvidedKeypoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::AffineFeature2D::detectAndCompute(InputArray, InputArray, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1175
	// ("cv::xfeatures2d::AffineFeature2D::detectAndCompute", vec![(pred!(mut, ["image", "mask", "keypoints", "descriptors"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<cv::xfeatures2d::Elliptic_KeyPoint>*", "const cv::_OutputArray*"]), _)]),
	void cv_xfeatures2d_AffineFeature2D_detectAndCompute_const__InputArrayR_const__InputArrayR_vectorLElliptic_KeyPointGR_const__OutputArrayR(cv::xfeatures2d::AffineFeature2D* instance, const cv::_InputArray* image, const cv::_InputArray* mask, std::vector<cv::xfeatures2d::Elliptic_KeyPoint>* keypoints, const cv::_OutputArray* descriptors, ResultVoid* ocvrs_return) {
		try {
			instance->detectAndCompute(*image, *mask, *keypoints, *descriptors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::AffineFeature2D::to_TBMR() generated
	// ("cv::xfeatures2d::AffineFeature2D::to_TBMR", vec![(pred!(mut, [], []), _)]),
	cv::xfeatures2d::TBMR* cv_xfeatures2d_AffineFeature2D_to_TBMR(cv::xfeatures2d::AffineFeature2D* instance) {
			return dynamic_cast<cv::xfeatures2d::TBMR*>(instance);
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

	// create(int, bool, int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1301
	// ("cv::xfeatures2d::AgastFeatureDetector::create", vec![(pred!(mut, ["threshold", "nonmaxSuppression", "type"], ["int", "bool", "int"]), _)]),
	void cv_xfeatures2d_AgastFeatureDetector_create_int_bool_int(int threshold, bool nonmaxSuppression, int type, Result<cv::Ptr<cv::xfeatures2d::AgastFeatureDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::AgastFeatureDetector> ret = cv::xfeatures2d::AgastFeatureDetector::create(threshold, nonmaxSuppression, type);
			Ok(new cv::Ptr<cv::xfeatures2d::AgastFeatureDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::AgastFeatureDetector::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1301
	// ("cv::xfeatures2d::AgastFeatureDetector::create", vec![(pred!(mut, [], []), _)]),
	void cv_xfeatures2d_AgastFeatureDetector_create(Result<cv::Ptr<cv::xfeatures2d::AgastFeatureDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::AgastFeatureDetector> ret = cv::xfeatures2d::AgastFeatureDetector::create();
			Ok(new cv::Ptr<cv::xfeatures2d::AgastFeatureDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setThreshold(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1305
	// ("cv::xfeatures2d::AgastFeatureDetector::setThreshold", vec![(pred!(mut, ["threshold"], ["int"]), _)]),
	void cv_xfeatures2d_AgastFeatureDetector_setThreshold_int(cv::xfeatures2d::AgastFeatureDetector* instance, int threshold, ResultVoid* ocvrs_return) {
		try {
			instance->setThreshold(threshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1306
	// ("cv::xfeatures2d::AgastFeatureDetector::getThreshold", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_AgastFeatureDetector_getThreshold_const(const cv::xfeatures2d::AgastFeatureDetector* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNonmaxSuppression(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1308
	// ("cv::xfeatures2d::AgastFeatureDetector::setNonmaxSuppression", vec![(pred!(mut, ["f"], ["bool"]), _)]),
	void cv_xfeatures2d_AgastFeatureDetector_setNonmaxSuppression_bool(cv::xfeatures2d::AgastFeatureDetector* instance, bool f, ResultVoid* ocvrs_return) {
		try {
			instance->setNonmaxSuppression(f);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNonmaxSuppression()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1309
	// ("cv::xfeatures2d::AgastFeatureDetector::getNonmaxSuppression", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_AgastFeatureDetector_getNonmaxSuppression_const(const cv::xfeatures2d::AgastFeatureDetector* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getNonmaxSuppression();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setType(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1311
	// ("cv::xfeatures2d::AgastFeatureDetector::setType", vec![(pred!(mut, ["type"], ["int"]), _)]),
	void cv_xfeatures2d_AgastFeatureDetector_setType_int(cv::xfeatures2d::AgastFeatureDetector* instance, int type, ResultVoid* ocvrs_return) {
		try {
			instance->setType(type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getType()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1312
	// ("cv::xfeatures2d::AgastFeatureDetector::getType", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_AgastFeatureDetector_getType_const(const cv::xfeatures2d::AgastFeatureDetector* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1313
	// ("cv::xfeatures2d::AgastFeatureDetector::getDefaultName", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_AgastFeatureDetector_getDefaultName_const(const cv::xfeatures2d::AgastFeatureDetector* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getDefaultName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::AgastFeatureDetector::to_Algorithm() generated
	// ("cv::xfeatures2d::AgastFeatureDetector::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_xfeatures2d_AgastFeatureDetector_to_Algorithm(cv::xfeatures2d::AgastFeatureDetector* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::xfeatures2d::AgastFeatureDetector::to_Feature2D() generated
	// ("cv::xfeatures2d::AgastFeatureDetector::to_Feature2D", vec![(pred!(mut, [], []), _)]),
	cv::Feature2D* cv_xfeatures2d_AgastFeatureDetector_to_Feature2D(cv::xfeatures2d::AgastFeatureDetector* instance) {
			return dynamic_cast<cv::Feature2D*>(instance);
	}

	// cv::xfeatures2d::AgastFeatureDetector::delete() generated
	// ("cv::xfeatures2d::AgastFeatureDetector::delete", vec![(pred!(mut, [], []), _)]),
	void cv_xfeatures2d_AgastFeatureDetector_delete(cv::xfeatures2d::AgastFeatureDetector* instance) {
			delete instance;
	}

	// create(float, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:288
	// ("cv::xfeatures2d::BEBLID::create", vec![(pred!(mut, ["scale_factor", "n_bits"], ["float", "int"]), _)]),
	void cv_xfeatures2d_BEBLID_create_float_int(float scale_factor, int n_bits, Result<cv::Ptr<cv::xfeatures2d::BEBLID>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::BEBLID> ret = cv::xfeatures2d::BEBLID::create(scale_factor, n_bits);
			Ok(new cv::Ptr<cv::xfeatures2d::BEBLID>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::BEBLID::create(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:288
	// ("cv::xfeatures2d::BEBLID::create", vec![(pred!(mut, ["scale_factor"], ["float"]), _)]),
	void cv_xfeatures2d_BEBLID_create_float(float scale_factor, Result<cv::Ptr<cv::xfeatures2d::BEBLID>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::BEBLID> ret = cv::xfeatures2d::BEBLID::create(scale_factor);
			Ok(new cv::Ptr<cv::xfeatures2d::BEBLID>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setScaleFactor(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:290
	// ("cv::xfeatures2d::BEBLID::setScaleFactor", vec![(pred!(mut, ["scale_factor"], ["float"]), _)]),
	void cv_xfeatures2d_BEBLID_setScaleFactor_float(cv::xfeatures2d::BEBLID* instance, float scale_factor, ResultVoid* ocvrs_return) {
		try {
			instance->setScaleFactor(scale_factor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getScaleFactor()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:291
	// ("cv::xfeatures2d::BEBLID::getScaleFactor", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_BEBLID_getScaleFactor_const(const cv::xfeatures2d::BEBLID* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getScaleFactor();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:293
	// ("cv::xfeatures2d::BEBLID::getDefaultName", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_BEBLID_getDefaultName_const(const cv::xfeatures2d::BEBLID* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getDefaultName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::BEBLID::to_Algorithm() generated
	// ("cv::xfeatures2d::BEBLID::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_xfeatures2d_BEBLID_to_Algorithm(cv::xfeatures2d::BEBLID* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::xfeatures2d::BEBLID::to_Feature2D() generated
	// ("cv::xfeatures2d::BEBLID::to_Feature2D", vec![(pred!(mut, [], []), _)]),
	cv::Feature2D* cv_xfeatures2d_BEBLID_to_Feature2D(cv::xfeatures2d::BEBLID* instance) {
			return dynamic_cast<cv::Feature2D*>(instance);
	}

	// cv::xfeatures2d::BEBLID::delete() generated
	// ("cv::xfeatures2d::BEBLID::delete", vec![(pred!(mut, [], []), _)]),
	void cv_xfeatures2d_BEBLID_delete(cv::xfeatures2d::BEBLID* instance) {
			delete instance;
	}

	// BOWImgDescriptorExtractor(const Ptr<Feature2D> &, const Ptr<DescriptorMatcher> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1627
	// ("cv::xfeatures2d::BOWImgDescriptorExtractor::BOWImgDescriptorExtractor", vec![(pred!(mut, ["dextractor", "dmatcher"], ["const cv::Ptr<cv::Feature2D>*", "const cv::Ptr<cv::DescriptorMatcher>*"]), _)]),
	void cv_xfeatures2d_BOWImgDescriptorExtractor_BOWImgDescriptorExtractor_const_PtrLFeature2DGR_const_PtrLDescriptorMatcherGR(const cv::Ptr<cv::Feature2D>* dextractor, const cv::Ptr<cv::DescriptorMatcher>* dmatcher, Result<cv::xfeatures2d::BOWImgDescriptorExtractor*>* ocvrs_return) {
		try {
			cv::xfeatures2d::BOWImgDescriptorExtractor* ret = new cv::xfeatures2d::BOWImgDescriptorExtractor(*dextractor, *dmatcher);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// BOWImgDescriptorExtractor(const Ptr<DescriptorMatcher> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1630
	// ("cv::xfeatures2d::BOWImgDescriptorExtractor::BOWImgDescriptorExtractor", vec![(pred!(mut, ["dmatcher"], ["const cv::Ptr<cv::DescriptorMatcher>*"]), _)]),
	void cv_xfeatures2d_BOWImgDescriptorExtractor_BOWImgDescriptorExtractor_const_PtrLDescriptorMatcherGR(const cv::Ptr<cv::DescriptorMatcher>* dmatcher, Result<cv::xfeatures2d::BOWImgDescriptorExtractor*>* ocvrs_return) {
		try {
			cv::xfeatures2d::BOWImgDescriptorExtractor* ret = new cv::xfeatures2d::BOWImgDescriptorExtractor(*dmatcher);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setVocabulary(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1638
	// ("cv::xfeatures2d::BOWImgDescriptorExtractor::setVocabulary", vec![(pred!(mut, ["vocabulary"], ["const cv::Mat*"]), _)]),
	void cv_xfeatures2d_BOWImgDescriptorExtractor_setVocabulary_const_MatR(cv::xfeatures2d::BOWImgDescriptorExtractor* instance, const cv::Mat* vocabulary, ResultVoid* ocvrs_return) {
		try {
			instance->setVocabulary(*vocabulary);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getVocabulary()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1642
	// ("cv::xfeatures2d::BOWImgDescriptorExtractor::getVocabulary", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_BOWImgDescriptorExtractor_getVocabulary_const(const cv::xfeatures2d::BOWImgDescriptorExtractor* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			const cv::Mat ret = instance->getVocabulary();
			Ok(new const cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// compute(InputArray, std::vector<KeyPoint> &, OutputArray, std::vector<std::vector<int>> *, Mat *)(InputArray, CppPassByVoidPtr, OutputArray, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1654
	// ("cv::xfeatures2d::BOWImgDescriptorExtractor::compute", vec![(pred!(mut, ["image", "keypoints", "imgDescriptor", "pointIdxsOfClusters", "descriptors"], ["const cv::_InputArray*", "std::vector<cv::KeyPoint>*", "const cv::_OutputArray*", "std::vector<std::vector<int>>*", "cv::Mat*"]), _)]),
	void cv_xfeatures2d_BOWImgDescriptorExtractor_compute_const__InputArrayR_vectorLKeyPointGR_const__OutputArrayR_vectorLvectorLintGGX_MatX(cv::xfeatures2d::BOWImgDescriptorExtractor* instance, const cv::_InputArray* image, std::vector<cv::KeyPoint>* keypoints, const cv::_OutputArray* imgDescriptor, std::vector<std::vector<int>>* pointIdxsOfClusters, cv::Mat* descriptors, ResultVoid* ocvrs_return) {
		try {
			instance->compute(*image, *keypoints, *imgDescriptor, pointIdxsOfClusters, descriptors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::BOWImgDescriptorExtractor::compute(InputArray, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1654
	// ("cv::xfeatures2d::BOWImgDescriptorExtractor::compute", vec![(pred!(mut, ["image", "keypoints", "imgDescriptor"], ["const cv::_InputArray*", "std::vector<cv::KeyPoint>*", "const cv::_OutputArray*"]), _)]),
	void cv_xfeatures2d_BOWImgDescriptorExtractor_compute_const__InputArrayR_vectorLKeyPointGR_const__OutputArrayR(cv::xfeatures2d::BOWImgDescriptorExtractor* instance, const cv::_InputArray* image, std::vector<cv::KeyPoint>* keypoints, const cv::_OutputArray* imgDescriptor, ResultVoid* ocvrs_return) {
		try {
			instance->compute(*image, *keypoints, *imgDescriptor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// compute(InputArray, OutputArray, std::vector<std::vector<int>> *)(InputArray, OutputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1663
	// ("cv::xfeatures2d::BOWImgDescriptorExtractor::compute", vec![(pred!(mut, ["keypointDescriptors", "imgDescriptor", "pointIdxsOfClusters"], ["const cv::_InputArray*", "const cv::_OutputArray*", "std::vector<std::vector<int>>*"]), _)]),
	void cv_xfeatures2d_BOWImgDescriptorExtractor_compute_const__InputArrayR_const__OutputArrayR_vectorLvectorLintGGX(cv::xfeatures2d::BOWImgDescriptorExtractor* instance, const cv::_InputArray* keypointDescriptors, const cv::_OutputArray* imgDescriptor, std::vector<std::vector<int>>* pointIdxsOfClusters, ResultVoid* ocvrs_return) {
		try {
			instance->compute(*keypointDescriptors, *imgDescriptor, pointIdxsOfClusters);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::BOWImgDescriptorExtractor::compute(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1663
	// ("cv::xfeatures2d::BOWImgDescriptorExtractor::compute", vec![(pred!(mut, ["keypointDescriptors", "imgDescriptor"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_xfeatures2d_BOWImgDescriptorExtractor_compute_const__InputArrayR_const__OutputArrayR(cv::xfeatures2d::BOWImgDescriptorExtractor* instance, const cv::_InputArray* keypointDescriptors, const cv::_OutputArray* imgDescriptor, ResultVoid* ocvrs_return) {
		try {
			instance->compute(*keypointDescriptors, *imgDescriptor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// compute2(const Mat &, std::vector<KeyPoint> &, Mat &)(TraitClass, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1667
	// ("cv::xfeatures2d::BOWImgDescriptorExtractor::compute2", vec![(pred!(mut, ["image", "keypoints", "imgDescriptor"], ["const cv::Mat*", "std::vector<cv::KeyPoint>*", "cv::Mat*"]), _)]),
	void cv_xfeatures2d_BOWImgDescriptorExtractor_compute2_const_MatR_vectorLKeyPointGR_MatR(cv::xfeatures2d::BOWImgDescriptorExtractor* instance, const cv::Mat* image, std::vector<cv::KeyPoint>* keypoints, cv::Mat* imgDescriptor, ResultVoid* ocvrs_return) {
		try {
			instance->compute2(*image, *keypoints, *imgDescriptor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// descriptorSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1672
	// ("cv::xfeatures2d::BOWImgDescriptorExtractor::descriptorSize", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_BOWImgDescriptorExtractor_descriptorSize_const(const cv::xfeatures2d::BOWImgDescriptorExtractor* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->descriptorSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// descriptorType()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1676
	// ("cv::xfeatures2d::BOWImgDescriptorExtractor::descriptorType", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_BOWImgDescriptorExtractor_descriptorType_const(const cv::xfeatures2d::BOWImgDescriptorExtractor* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->descriptorType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::BOWImgDescriptorExtractor::delete() generated
	// ("cv::xfeatures2d::BOWImgDescriptorExtractor::delete", vec![(pred!(mut, [], []), _)]),
	void cv_xfeatures2d_BOWImgDescriptorExtractor_delete(cv::xfeatures2d::BOWImgDescriptorExtractor* instance) {
			delete instance;
	}

	// BOWKMeansTrainer(int, const TermCriteria &, int, int)(Primitive, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1591
	// ("cv::xfeatures2d::BOWKMeansTrainer::BOWKMeansTrainer", vec![(pred!(mut, ["clusterCount", "termcrit", "attempts", "flags"], ["int", "const cv::TermCriteria*", "int", "int"]), _)]),
	void cv_xfeatures2d_BOWKMeansTrainer_BOWKMeansTrainer_int_const_TermCriteriaR_int_int(int clusterCount, const cv::TermCriteria* termcrit, int attempts, int flags, Result<cv::xfeatures2d::BOWKMeansTrainer*>* ocvrs_return) {
		try {
			cv::xfeatures2d::BOWKMeansTrainer* ret = new cv::xfeatures2d::BOWKMeansTrainer(clusterCount, *termcrit, attempts, flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::BOWKMeansTrainer::BOWKMeansTrainer(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1591
	// ("cv::xfeatures2d::BOWKMeansTrainer::BOWKMeansTrainer", vec![(pred!(mut, ["clusterCount"], ["int"]), _)]),
	void cv_xfeatures2d_BOWKMeansTrainer_BOWKMeansTrainer_int(int clusterCount, Result<cv::xfeatures2d::BOWKMeansTrainer*>* ocvrs_return) {
		try {
			cv::xfeatures2d::BOWKMeansTrainer* ret = new cv::xfeatures2d::BOWKMeansTrainer(clusterCount);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cluster()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1596
	// ("cv::xfeatures2d::BOWKMeansTrainer::cluster", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_BOWKMeansTrainer_cluster_const(const cv::xfeatures2d::BOWKMeansTrainer* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->cluster();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cluster(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1597
	// ("cv::xfeatures2d::BOWKMeansTrainer::cluster", vec![(pred!(const, ["descriptors"], ["const cv::Mat*"]), _)]),
	void cv_xfeatures2d_BOWKMeansTrainer_cluster_const_const_MatR(const cv::xfeatures2d::BOWKMeansTrainer* instance, const cv::Mat* descriptors, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->cluster(*descriptors);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::BOWKMeansTrainer::to_BOWTrainer() generated
	// ("cv::xfeatures2d::BOWKMeansTrainer::to_BOWTrainer", vec![(pred!(mut, [], []), _)]),
	cv::xfeatures2d::BOWTrainer* cv_xfeatures2d_BOWKMeansTrainer_to_BOWTrainer(cv::xfeatures2d::BOWKMeansTrainer* instance) {
			return dynamic_cast<cv::xfeatures2d::BOWTrainer*>(instance);
	}

	// cv::xfeatures2d::BOWKMeansTrainer::delete() generated
	// ("cv::xfeatures2d::BOWKMeansTrainer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_xfeatures2d_BOWKMeansTrainer_delete(cv::xfeatures2d::BOWKMeansTrainer* instance) {
			delete instance;
	}

	// add(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1551
	// ("cv::xfeatures2d::BOWTrainer::add", vec![(pred!(mut, ["descriptors"], ["const cv::Mat*"]), _)]),
	void cv_xfeatures2d_BOWTrainer_add_const_MatR(cv::xfeatures2d::BOWTrainer* instance, const cv::Mat* descriptors, ResultVoid* ocvrs_return) {
		try {
			instance->add(*descriptors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDescriptors()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1555
	// ("cv::xfeatures2d::BOWTrainer::getDescriptors", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_BOWTrainer_getDescriptors_const(const cv::xfeatures2d::BOWTrainer* instance, Result<std::vector<cv::Mat>*>* ocvrs_return) {
		try {
			const std::vector<cv::Mat> ret = instance->getDescriptors();
			Ok(new const std::vector<cv::Mat>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// descriptorsCount()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1559
	// ("cv::xfeatures2d::BOWTrainer::descriptorsCount", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_BOWTrainer_descriptorsCount_const(const cv::xfeatures2d::BOWTrainer* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->descriptorsCount();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// clear()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1561
	// ("cv::xfeatures2d::BOWTrainer::clear", vec![(pred!(mut, [], []), _)]),
	void cv_xfeatures2d_BOWTrainer_clear(cv::xfeatures2d::BOWTrainer* instance, ResultVoid* ocvrs_return) {
		try {
			instance->clear();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cluster()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1564
	// ("cv::xfeatures2d::BOWTrainer::cluster", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_BOWTrainer_cluster_const(const cv::xfeatures2d::BOWTrainer* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->cluster();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cluster(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1575
	// ("cv::xfeatures2d::BOWTrainer::cluster", vec![(pred!(const, ["descriptors"], ["const cv::Mat*"]), _)]),
	void cv_xfeatures2d_BOWTrainer_cluster_const_const_MatR(const cv::xfeatures2d::BOWTrainer* instance, const cv::Mat* descriptors, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->cluster(*descriptors);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::BOWTrainer::to_BOWKMeansTrainer() generated
	// ("cv::xfeatures2d::BOWTrainer::to_BOWKMeansTrainer", vec![(pred!(mut, [], []), _)]),
	cv::xfeatures2d::BOWKMeansTrainer* cv_xfeatures2d_BOWTrainer_to_BOWKMeansTrainer(cv::xfeatures2d::BOWTrainer* instance) {
			return dynamic_cast<cv::xfeatures2d::BOWKMeansTrainer*>(instance);
	}

	// cv::xfeatures2d::BOWTrainer::delete() generated
	// ("cv::xfeatures2d::BOWTrainer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_xfeatures2d_BOWTrainer_delete(cv::xfeatures2d::BOWTrainer* instance) {
			delete instance;
	}

	// create(int, int, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1232
	// ("cv::xfeatures2d::BRISK::create", vec![(pred!(mut, ["thresh", "octaves", "patternScale"], ["int", "int", "float"]), _)]),
	void cv_xfeatures2d_BRISK_create_int_int_float(int thresh, int octaves, float patternScale, Result<cv::Ptr<cv::xfeatures2d::BRISK>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::BRISK> ret = cv::xfeatures2d::BRISK::create(thresh, octaves, patternScale);
			Ok(new cv::Ptr<cv::xfeatures2d::BRISK>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::BRISK::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1232
	// ("cv::xfeatures2d::BRISK::create", vec![(pred!(mut, [], []), _)]),
	void cv_xfeatures2d_BRISK_create(Result<cv::Ptr<cv::xfeatures2d::BRISK>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::BRISK> ret = cv::xfeatures2d::BRISK::create();
			Ok(new cv::Ptr<cv::xfeatures2d::BRISK>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const std::vector<float> &, const std::vector<int> &, float, float, const std::vector<int> &)(CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1245
	// ("cv::xfeatures2d::BRISK::create", vec![(pred!(mut, ["radiusList", "numberList", "dMax", "dMin", "indexChange"], ["const std::vector<float>*", "const std::vector<int>*", "float", "float", "const std::vector<int>*"]), _)]),
	void cv_xfeatures2d_BRISK_create_const_vectorLfloatGR_const_vectorLintGR_float_float_const_vectorLintGR(const std::vector<float>* radiusList, const std::vector<int>* numberList, float dMax, float dMin, const std::vector<int>* indexChange, Result<cv::Ptr<cv::xfeatures2d::BRISK>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::BRISK> ret = cv::xfeatures2d::BRISK::create(*radiusList, *numberList, dMax, dMin, *indexChange);
			Ok(new cv::Ptr<cv::xfeatures2d::BRISK>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::BRISK::create(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1245
	// ("cv::xfeatures2d::BRISK::create", vec![(pred!(mut, ["radiusList", "numberList"], ["const std::vector<float>*", "const std::vector<int>*"]), _)]),
	void cv_xfeatures2d_BRISK_create_const_vectorLfloatGR_const_vectorLintGR(const std::vector<float>* radiusList, const std::vector<int>* numberList, Result<cv::Ptr<cv::xfeatures2d::BRISK>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::BRISK> ret = cv::xfeatures2d::BRISK::create(*radiusList, *numberList);
			Ok(new cv::Ptr<cv::xfeatures2d::BRISK>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(int, int, const std::vector<float> &, const std::vector<int> &, float, float, const std::vector<int> &)(Primitive, Primitive, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1261
	// ("cv::xfeatures2d::BRISK::create", vec![(pred!(mut, ["thresh", "octaves", "radiusList", "numberList", "dMax", "dMin", "indexChange"], ["int", "int", "const std::vector<float>*", "const std::vector<int>*", "float", "float", "const std::vector<int>*"]), _)]),
	void cv_xfeatures2d_BRISK_create_int_int_const_vectorLfloatGR_const_vectorLintGR_float_float_const_vectorLintGR(int thresh, int octaves, const std::vector<float>* radiusList, const std::vector<int>* numberList, float dMax, float dMin, const std::vector<int>* indexChange, Result<cv::Ptr<cv::xfeatures2d::BRISK>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::BRISK> ret = cv::xfeatures2d::BRISK::create(thresh, octaves, *radiusList, *numberList, dMax, dMin, *indexChange);
			Ok(new cv::Ptr<cv::xfeatures2d::BRISK>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::BRISK::create(Primitive, Primitive, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1261
	// ("cv::xfeatures2d::BRISK::create", vec![(pred!(mut, ["thresh", "octaves", "radiusList", "numberList"], ["int", "int", "const std::vector<float>*", "const std::vector<int>*"]), _)]),
	void cv_xfeatures2d_BRISK_create_int_int_const_vectorLfloatGR_const_vectorLintGR(int thresh, int octaves, const std::vector<float>* radiusList, const std::vector<int>* numberList, Result<cv::Ptr<cv::xfeatures2d::BRISK>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::BRISK> ret = cv::xfeatures2d::BRISK::create(thresh, octaves, *radiusList, *numberList);
			Ok(new cv::Ptr<cv::xfeatures2d::BRISK>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1264
	// ("cv::xfeatures2d::BRISK::getDefaultName", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_BRISK_getDefaultName_const(const cv::xfeatures2d::BRISK* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getDefaultName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setThreshold(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1269
	// ("cv::xfeatures2d::BRISK::setThreshold", vec![(pred!(mut, ["threshold"], ["int"]), _)]),
	void cv_xfeatures2d_BRISK_setThreshold_int(cv::xfeatures2d::BRISK* instance, int threshold, ResultVoid* ocvrs_return) {
		try {
			instance->setThreshold(threshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1270
	// ("cv::xfeatures2d::BRISK::getThreshold", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_BRISK_getThreshold_const(const cv::xfeatures2d::BRISK* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setOctaves(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1275
	// ("cv::xfeatures2d::BRISK::setOctaves", vec![(pred!(mut, ["octaves"], ["int"]), _)]),
	void cv_xfeatures2d_BRISK_setOctaves_int(cv::xfeatures2d::BRISK* instance, int octaves, ResultVoid* ocvrs_return) {
		try {
			instance->setOctaves(octaves);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getOctaves()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1276
	// ("cv::xfeatures2d::BRISK::getOctaves", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_BRISK_getOctaves_const(const cv::xfeatures2d::BRISK* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getOctaves();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setPatternScale(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1281
	// ("cv::xfeatures2d::BRISK::setPatternScale", vec![(pred!(mut, ["patternScale"], ["float"]), _)]),
	void cv_xfeatures2d_BRISK_setPatternScale_float(cv::xfeatures2d::BRISK* instance, float patternScale, ResultVoid* ocvrs_return) {
		try {
			instance->setPatternScale(patternScale);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPatternScale()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1282
	// ("cv::xfeatures2d::BRISK::getPatternScale", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_BRISK_getPatternScale_const(const cv::xfeatures2d::BRISK* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getPatternScale();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::BRISK::to_Algorithm() generated
	// ("cv::xfeatures2d::BRISK::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_xfeatures2d_BRISK_to_Algorithm(cv::xfeatures2d::BRISK* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::xfeatures2d::BRISK::to_Feature2D() generated
	// ("cv::xfeatures2d::BRISK::to_Feature2D", vec![(pred!(mut, [], []), _)]),
	cv::Feature2D* cv_xfeatures2d_BRISK_to_Feature2D(cv::xfeatures2d::BRISK* instance) {
			return dynamic_cast<cv::Feature2D*>(instance);
	}

	// cv::xfeatures2d::BRISK::delete() generated
	// ("cv::xfeatures2d::BRISK::delete", vec![(pred!(mut, [], []), _)]),
	void cv_xfeatures2d_BRISK_delete(cv::xfeatures2d::BRISK* instance) {
			delete instance;
	}

	// create(int, bool, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:590
	// ("cv::xfeatures2d::BoostDesc::create", vec![(pred!(mut, ["desc", "use_scale_orientation", "scale_factor"], ["int", "bool", "float"]), _)]),
	void cv_xfeatures2d_BoostDesc_create_int_bool_float(int desc, bool use_scale_orientation, float scale_factor, Result<cv::Ptr<cv::xfeatures2d::BoostDesc>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::BoostDesc> ret = cv::xfeatures2d::BoostDesc::create(desc, use_scale_orientation, scale_factor);
			Ok(new cv::Ptr<cv::xfeatures2d::BoostDesc>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::BoostDesc::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:590
	// ("cv::xfeatures2d::BoostDesc::create", vec![(pred!(mut, [], []), _)]),
	void cv_xfeatures2d_BoostDesc_create(Result<cv::Ptr<cv::xfeatures2d::BoostDesc>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::BoostDesc> ret = cv::xfeatures2d::BoostDesc::create();
			Ok(new cv::Ptr<cv::xfeatures2d::BoostDesc>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:593
	// ("cv::xfeatures2d::BoostDesc::getDefaultName", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_BoostDesc_getDefaultName_const(const cv::xfeatures2d::BoostDesc* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getDefaultName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setUseScaleOrientation(const bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:595
	// ("cv::xfeatures2d::BoostDesc::setUseScaleOrientation", vec![(pred!(mut, ["use_scale_orientation"], ["const bool"]), _)]),
	void cv_xfeatures2d_BoostDesc_setUseScaleOrientation_const_bool(cv::xfeatures2d::BoostDesc* instance, const bool use_scale_orientation, ResultVoid* ocvrs_return) {
		try {
			instance->setUseScaleOrientation(use_scale_orientation);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getUseScaleOrientation()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:596
	// ("cv::xfeatures2d::BoostDesc::getUseScaleOrientation", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_BoostDesc_getUseScaleOrientation_const(const cv::xfeatures2d::BoostDesc* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUseScaleOrientation();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setScaleFactor(const float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:598
	// ("cv::xfeatures2d::BoostDesc::setScaleFactor", vec![(pred!(mut, ["scale_factor"], ["const float"]), _)]),
	void cv_xfeatures2d_BoostDesc_setScaleFactor_const_float(cv::xfeatures2d::BoostDesc* instance, const float scale_factor, ResultVoid* ocvrs_return) {
		try {
			instance->setScaleFactor(scale_factor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getScaleFactor()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:599
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

	// create(int, bool)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:167
	// ("cv::xfeatures2d::BriefDescriptorExtractor::create", vec![(pred!(mut, ["bytes", "use_orientation"], ["int", "bool"]), _)]),
	void cv_xfeatures2d_BriefDescriptorExtractor_create_int_bool(int bytes, bool use_orientation, Result<cv::Ptr<cv::xfeatures2d::BriefDescriptorExtractor>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::BriefDescriptorExtractor> ret = cv::xfeatures2d::BriefDescriptorExtractor::create(bytes, use_orientation);
			Ok(new cv::Ptr<cv::xfeatures2d::BriefDescriptorExtractor>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::BriefDescriptorExtractor::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:167
	// ("cv::xfeatures2d::BriefDescriptorExtractor::create", vec![(pred!(mut, [], []), _)]),
	void cv_xfeatures2d_BriefDescriptorExtractor_create(Result<cv::Ptr<cv::xfeatures2d::BriefDescriptorExtractor>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::BriefDescriptorExtractor> ret = cv::xfeatures2d::BriefDescriptorExtractor::create();
			Ok(new cv::Ptr<cv::xfeatures2d::BriefDescriptorExtractor>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDescriptorSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:169
	// ("cv::xfeatures2d::BriefDescriptorExtractor::setDescriptorSize", vec![(pred!(mut, ["bytes"], ["int"]), _)]),
	void cv_xfeatures2d_BriefDescriptorExtractor_setDescriptorSize_int(cv::xfeatures2d::BriefDescriptorExtractor* instance, int bytes, ResultVoid* ocvrs_return) {
		try {
			instance->setDescriptorSize(bytes);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDescriptorSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:170
	// ("cv::xfeatures2d::BriefDescriptorExtractor::getDescriptorSize", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_BriefDescriptorExtractor_getDescriptorSize_const(const cv::xfeatures2d::BriefDescriptorExtractor* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getDescriptorSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setUseOrientation(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:172
	// ("cv::xfeatures2d::BriefDescriptorExtractor::setUseOrientation", vec![(pred!(mut, ["use_orientation"], ["bool"]), _)]),
	void cv_xfeatures2d_BriefDescriptorExtractor_setUseOrientation_bool(cv::xfeatures2d::BriefDescriptorExtractor* instance, bool use_orientation, ResultVoid* ocvrs_return) {
		try {
			instance->setUseOrientation(use_orientation);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getUseOrientation()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:173
	// ("cv::xfeatures2d::BriefDescriptorExtractor::getUseOrientation", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_BriefDescriptorExtractor_getUseOrientation_const(const cv::xfeatures2d::BriefDescriptorExtractor* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUseOrientation();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:175
	// ("cv::xfeatures2d::BriefDescriptorExtractor::getDefaultName", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_BriefDescriptorExtractor_getDefaultName_const(const cv::xfeatures2d::BriefDescriptorExtractor* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getDefaultName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
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

	// create(float, int, int, int, DAISY::NormalizationType, InputArray, bool, bool)(Primitive, Primitive, Primitive, Primitive, Enum, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:364
	// ("cv::xfeatures2d::DAISY::create", vec![(pred!(mut, ["radius", "q_radius", "q_theta", "q_hist", "norm", "H", "interpolation", "use_orientation"], ["float", "int", "int", "int", "cv::xfeatures2d::DAISY::NormalizationType", "const cv::_InputArray*", "bool", "bool"]), _)]),
	void cv_xfeatures2d_DAISY_create_float_int_int_int_NormalizationType_const__InputArrayR_bool_bool(float radius, int q_radius, int q_theta, int q_hist, cv::xfeatures2d::DAISY::NormalizationType norm, const cv::_InputArray* H, bool interpolation, bool use_orientation, Result<cv::Ptr<cv::xfeatures2d::DAISY>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::DAISY> ret = cv::xfeatures2d::DAISY::create(radius, q_radius, q_theta, q_hist, norm, *H, interpolation, use_orientation);
			Ok(new cv::Ptr<cv::xfeatures2d::DAISY>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::DAISY::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:364
	// ("cv::xfeatures2d::DAISY::create", vec![(pred!(mut, [], []), _)]),
	void cv_xfeatures2d_DAISY_create(Result<cv::Ptr<cv::xfeatures2d::DAISY>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::DAISY> ret = cv::xfeatures2d::DAISY::create();
			Ok(new cv::Ptr<cv::xfeatures2d::DAISY>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setRadius(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:368
	// ("cv::xfeatures2d::DAISY::setRadius", vec![(pred!(mut, ["radius"], ["float"]), _)]),
	void cv_xfeatures2d_DAISY_setRadius_float(cv::xfeatures2d::DAISY* instance, float radius, ResultVoid* ocvrs_return) {
		try {
			instance->setRadius(radius);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getRadius()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:369
	// ("cv::xfeatures2d::DAISY::getRadius", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_DAISY_getRadius_const(const cv::xfeatures2d::DAISY* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getRadius();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setQRadius(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:371
	// ("cv::xfeatures2d::DAISY::setQRadius", vec![(pred!(mut, ["q_radius"], ["int"]), _)]),
	void cv_xfeatures2d_DAISY_setQRadius_int(cv::xfeatures2d::DAISY* instance, int q_radius, ResultVoid* ocvrs_return) {
		try {
			instance->setQRadius(q_radius);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getQRadius()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:372
	// ("cv::xfeatures2d::DAISY::getQRadius", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_DAISY_getQRadius_const(const cv::xfeatures2d::DAISY* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getQRadius();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setQTheta(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:374
	// ("cv::xfeatures2d::DAISY::setQTheta", vec![(pred!(mut, ["q_theta"], ["int"]), _)]),
	void cv_xfeatures2d_DAISY_setQTheta_int(cv::xfeatures2d::DAISY* instance, int q_theta, ResultVoid* ocvrs_return) {
		try {
			instance->setQTheta(q_theta);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getQTheta()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:375
	// ("cv::xfeatures2d::DAISY::getQTheta", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_DAISY_getQTheta_const(const cv::xfeatures2d::DAISY* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getQTheta();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setQHist(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:377
	// ("cv::xfeatures2d::DAISY::setQHist", vec![(pred!(mut, ["q_hist"], ["int"]), _)]),
	void cv_xfeatures2d_DAISY_setQHist_int(cv::xfeatures2d::DAISY* instance, int q_hist, ResultVoid* ocvrs_return) {
		try {
			instance->setQHist(q_hist);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getQHist()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:378
	// ("cv::xfeatures2d::DAISY::getQHist", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_DAISY_getQHist_const(const cv::xfeatures2d::DAISY* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getQHist();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNorm(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:380
	// ("cv::xfeatures2d::DAISY::setNorm", vec![(pred!(mut, ["norm"], ["int"]), _)]),
	void cv_xfeatures2d_DAISY_setNorm_int(cv::xfeatures2d::DAISY* instance, int norm, ResultVoid* ocvrs_return) {
		try {
			instance->setNorm(norm);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNorm()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:381
	// ("cv::xfeatures2d::DAISY::getNorm", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_DAISY_getNorm_const(const cv::xfeatures2d::DAISY* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNorm();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setH(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:383
	// ("cv::xfeatures2d::DAISY::setH", vec![(pred!(mut, ["H"], ["const cv::_InputArray*"]), _)]),
	void cv_xfeatures2d_DAISY_setH_const__InputArrayR(cv::xfeatures2d::DAISY* instance, const cv::_InputArray* H, ResultVoid* ocvrs_return) {
		try {
			instance->setH(*H);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getH()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:384
	// ("cv::xfeatures2d::DAISY::getH", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_DAISY_getH_const(const cv::xfeatures2d::DAISY* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getH();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setInterpolation(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:386
	// ("cv::xfeatures2d::DAISY::setInterpolation", vec![(pred!(mut, ["interpolation"], ["bool"]), _)]),
	void cv_xfeatures2d_DAISY_setInterpolation_bool(cv::xfeatures2d::DAISY* instance, bool interpolation, ResultVoid* ocvrs_return) {
		try {
			instance->setInterpolation(interpolation);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getInterpolation()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:387
	// ("cv::xfeatures2d::DAISY::getInterpolation", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_DAISY_getInterpolation_const(const cv::xfeatures2d::DAISY* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getInterpolation();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setUseOrientation(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:389
	// ("cv::xfeatures2d::DAISY::setUseOrientation", vec![(pred!(mut, ["use_orientation"], ["bool"]), _)]),
	void cv_xfeatures2d_DAISY_setUseOrientation_bool(cv::xfeatures2d::DAISY* instance, bool use_orientation, ResultVoid* ocvrs_return) {
		try {
			instance->setUseOrientation(use_orientation);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getUseOrientation()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:390
	// ("cv::xfeatures2d::DAISY::getUseOrientation", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_DAISY_getUseOrientation_const(const cv::xfeatures2d::DAISY* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUseOrientation();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:392
	// ("cv::xfeatures2d::DAISY::getDefaultName", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_DAISY_getDefaultName_const(const cv::xfeatures2d::DAISY* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getDefaultName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// compute(InputArray, std::vector<KeyPoint> &, OutputArray)(InputArray, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:399
	// ("cv::xfeatures2d::DAISY::compute", vec![(pred!(mut, ["image", "keypoints", "descriptors"], ["const cv::_InputArray*", "std::vector<cv::KeyPoint>*", "const cv::_OutputArray*"]), _)]),
	void cv_xfeatures2d_DAISY_compute_const__InputArrayR_vectorLKeyPointGR_const__OutputArrayR(cv::xfeatures2d::DAISY* instance, const cv::_InputArray* image, std::vector<cv::KeyPoint>* keypoints, const cv::_OutputArray* descriptors, ResultVoid* ocvrs_return) {
		try {
			instance->compute(*image, *keypoints, *descriptors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// compute(InputArrayOfArrays, std::vector<std::vector<KeyPoint>> &, OutputArrayOfArrays)(InputArray, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:401
	// ("cv::xfeatures2d::DAISY::compute", vec![(pred!(mut, ["images", "keypoints", "descriptors"], ["const cv::_InputArray*", "std::vector<std::vector<cv::KeyPoint>>*", "const cv::_OutputArray*"]), _)]),
	void cv_xfeatures2d_DAISY_compute_const__InputArrayR_vectorLvectorLKeyPointGGR_const__OutputArrayR(cv::xfeatures2d::DAISY* instance, const cv::_InputArray* images, std::vector<std::vector<cv::KeyPoint>>* keypoints, const cv::_OutputArray* descriptors, ResultVoid* ocvrs_return) {
		try {
			instance->compute(*images, *keypoints, *descriptors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// compute(InputArray, Rect, OutputArray)(InputArray, SimpleClass, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:410
	// ("cv::xfeatures2d::DAISY::compute", vec![(pred!(mut, ["image", "roi", "descriptors"], ["const cv::_InputArray*", "cv::Rect", "const cv::_OutputArray*"]), _)]),
	void cv_xfeatures2d_DAISY_compute_const__InputArrayR_Rect_const__OutputArrayR(cv::xfeatures2d::DAISY* instance, const cv::_InputArray* image, cv::Rect* roi, const cv::_OutputArray* descriptors, ResultVoid* ocvrs_return) {
		try {
			instance->compute(*image, *roi, *descriptors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// compute(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:416
	// ("cv::xfeatures2d::DAISY::compute", vec![(pred!(mut, ["image", "descriptors"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_xfeatures2d_DAISY_compute_const__InputArrayR_const__OutputArrayR(cv::xfeatures2d::DAISY* instance, const cv::_InputArray* image, const cv::_OutputArray* descriptors, ResultVoid* ocvrs_return) {
		try {
			instance->compute(*image, *descriptors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// GetDescriptor(double, double, int, float *)(Primitive, Primitive, Primitive, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:424
	// ("cv::xfeatures2d::DAISY::GetDescriptor", vec![(pred!(const, ["y", "x", "orientation", "descriptor"], ["double", "double", "int", "float*"]), _)]),
	void cv_xfeatures2d_DAISY_GetDescriptor_const_double_double_int_floatX(const cv::xfeatures2d::DAISY* instance, double y, double x, int orientation, float* descriptor, ResultVoid* ocvrs_return) {
		try {
			instance->GetDescriptor(y, x, orientation, descriptor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// GetDescriptor(double, double, int, float *, double *)(Primitive, Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:433
	// ("cv::xfeatures2d::DAISY::GetDescriptor", vec![(pred!(const, ["y", "x", "orientation", "descriptor", "H"], ["double", "double", "int", "float*", "double*"]), _)]),
	void cv_xfeatures2d_DAISY_GetDescriptor_const_double_double_int_floatX_doubleX(const cv::xfeatures2d::DAISY* instance, double y, double x, int orientation, float* descriptor, double* H, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->GetDescriptor(y, x, orientation, descriptor, H);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// GetUnnormalizedDescriptor(double, double, int, float *)(Primitive, Primitive, Primitive, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:441
	// ("cv::xfeatures2d::DAISY::GetUnnormalizedDescriptor", vec![(pred!(const, ["y", "x", "orientation", "descriptor"], ["double", "double", "int", "float*"]), _)]),
	void cv_xfeatures2d_DAISY_GetUnnormalizedDescriptor_const_double_double_int_floatX(const cv::xfeatures2d::DAISY* instance, double y, double x, int orientation, float* descriptor, ResultVoid* ocvrs_return) {
		try {
			instance->GetUnnormalizedDescriptor(y, x, orientation, descriptor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// GetUnnormalizedDescriptor(double, double, int, float *, double *)(Primitive, Primitive, Primitive, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:450
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

	// Elliptic_KeyPoint()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1084
	// ("cv::xfeatures2d::Elliptic_KeyPoint::Elliptic_KeyPoint", vec![(pred!(mut, [], []), _)]),
	void cv_xfeatures2d_Elliptic_KeyPoint_Elliptic_KeyPoint(Result<cv::xfeatures2d::Elliptic_KeyPoint*>* ocvrs_return) {
		try {
			cv::xfeatures2d::Elliptic_KeyPoint* ret = new cv::xfeatures2d::Elliptic_KeyPoint();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// Elliptic_KeyPoint(Point2f, float, Size, float, float)(SimpleClass, Primitive, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1085
	// ("cv::xfeatures2d::Elliptic_KeyPoint::Elliptic_KeyPoint", vec![(pred!(mut, ["pt", "angle", "axes", "size", "si"], ["cv::Point2f", "float", "cv::Size", "float", "float"]), _)]),
	void cv_xfeatures2d_Elliptic_KeyPoint_Elliptic_KeyPoint_Point2f_float_Size_float_float(cv::Point2f* pt, float angle, cv::Size* axes, float size, float si, Result<cv::xfeatures2d::Elliptic_KeyPoint*>* ocvrs_return) {
		try {
			cv::xfeatures2d::Elliptic_KeyPoint* ret = new cv::xfeatures2d::Elliptic_KeyPoint(*pt, angle, *axes, size, si);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::Elliptic_KeyPoint::axes() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1081
	// ("cv::xfeatures2d::Elliptic_KeyPoint::axes", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_Elliptic_KeyPoint_propAxes_const(const cv::xfeatures2d::Elliptic_KeyPoint* instance, cv::Size_<float>* ocvrs_return) {
			cv::Size_<float> ret = instance->axes;
			*ocvrs_return = ret;
	}

	// cv::xfeatures2d::Elliptic_KeyPoint::setAxes(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1081
	// ("cv::xfeatures2d::Elliptic_KeyPoint::setAxes", vec![(pred!(mut, ["val"], ["const cv::Size_<float>"]), _)]),
	void cv_xfeatures2d_Elliptic_KeyPoint_propAxes_const_Size_LfloatG(cv::xfeatures2d::Elliptic_KeyPoint* instance, const cv::Size_<float>* val) {
			instance->axes = *val;
	}

	// cv::xfeatures2d::Elliptic_KeyPoint::si() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1082
	// ("cv::xfeatures2d::Elliptic_KeyPoint::si", vec![(pred!(const, [], []), _)]),
	float cv_xfeatures2d_Elliptic_KeyPoint_propSi_const(const cv::xfeatures2d::Elliptic_KeyPoint* instance) {
			float ret = instance->si;
			return ret;
	}

	// cv::xfeatures2d::Elliptic_KeyPoint::setSi(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1082
	// ("cv::xfeatures2d::Elliptic_KeyPoint::setSi", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_xfeatures2d_Elliptic_KeyPoint_propSi_const_float(cv::xfeatures2d::Elliptic_KeyPoint* instance, const float val) {
			instance->si = val;
	}

	// cv::xfeatures2d::Elliptic_KeyPoint::transf() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1083
	// ("cv::xfeatures2d::Elliptic_KeyPoint::transf", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_Elliptic_KeyPoint_propTransf_const(const cv::xfeatures2d::Elliptic_KeyPoint* instance, cv::Matx23f* ocvrs_return) {
			cv::Matx23f ret = instance->transf;
			*ocvrs_return = ret;
	}

	// cv::xfeatures2d::Elliptic_KeyPoint::setTransf(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1083
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

	// create(bool, bool, float, int, const std::vector<int> &)(Primitive, Primitive, Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:103
	// ("cv::xfeatures2d::FREAK::create", vec![(pred!(mut, ["orientationNormalized", "scaleNormalized", "patternScale", "nOctaves", "selectedPairs"], ["bool", "bool", "float", "int", "const std::vector<int>*"]), _)]),
	void cv_xfeatures2d_FREAK_create_bool_bool_float_int_const_vectorLintGR(bool orientationNormalized, bool scaleNormalized, float patternScale, int nOctaves, const std::vector<int>* selectedPairs, Result<cv::Ptr<cv::xfeatures2d::FREAK>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::FREAK> ret = cv::xfeatures2d::FREAK::create(orientationNormalized, scaleNormalized, patternScale, nOctaves, *selectedPairs);
			Ok(new cv::Ptr<cv::xfeatures2d::FREAK>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::FREAK::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:103
	// ("cv::xfeatures2d::FREAK::create", vec![(pred!(mut, [], []), _)]),
	void cv_xfeatures2d_FREAK_create(Result<cv::Ptr<cv::xfeatures2d::FREAK>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::FREAK> ret = cv::xfeatures2d::FREAK::create();
			Ok(new cv::Ptr<cv::xfeatures2d::FREAK>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setOrientationNormalized(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:109
	// ("cv::xfeatures2d::FREAK::setOrientationNormalized", vec![(pred!(mut, ["orientationNormalized"], ["bool"]), _)]),
	void cv_xfeatures2d_FREAK_setOrientationNormalized_bool(cv::xfeatures2d::FREAK* instance, bool orientationNormalized, ResultVoid* ocvrs_return) {
		try {
			instance->setOrientationNormalized(orientationNormalized);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getOrientationNormalized()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:110
	// ("cv::xfeatures2d::FREAK::getOrientationNormalized", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_FREAK_getOrientationNormalized_const(const cv::xfeatures2d::FREAK* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getOrientationNormalized();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setScaleNormalized(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:112
	// ("cv::xfeatures2d::FREAK::setScaleNormalized", vec![(pred!(mut, ["scaleNormalized"], ["bool"]), _)]),
	void cv_xfeatures2d_FREAK_setScaleNormalized_bool(cv::xfeatures2d::FREAK* instance, bool scaleNormalized, ResultVoid* ocvrs_return) {
		try {
			instance->setScaleNormalized(scaleNormalized);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getScaleNormalized()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:113
	// ("cv::xfeatures2d::FREAK::getScaleNormalized", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_FREAK_getScaleNormalized_const(const cv::xfeatures2d::FREAK* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getScaleNormalized();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setPatternScale(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:115
	// ("cv::xfeatures2d::FREAK::setPatternScale", vec![(pred!(mut, ["patternScale"], ["double"]), _)]),
	void cv_xfeatures2d_FREAK_setPatternScale_double(cv::xfeatures2d::FREAK* instance, double patternScale, ResultVoid* ocvrs_return) {
		try {
			instance->setPatternScale(patternScale);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPatternScale()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:116
	// ("cv::xfeatures2d::FREAK::getPatternScale", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_FREAK_getPatternScale_const(const cv::xfeatures2d::FREAK* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getPatternScale();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNOctaves(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:118
	// ("cv::xfeatures2d::FREAK::setNOctaves", vec![(pred!(mut, ["nOctaves"], ["int"]), _)]),
	void cv_xfeatures2d_FREAK_setNOctaves_int(cv::xfeatures2d::FREAK* instance, int nOctaves, ResultVoid* ocvrs_return) {
		try {
			instance->setNOctaves(nOctaves);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNOctaves()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:119
	// ("cv::xfeatures2d::FREAK::getNOctaves", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_FREAK_getNOctaves_const(const cv::xfeatures2d::FREAK* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNOctaves();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:121
	// ("cv::xfeatures2d::FREAK::getDefaultName", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_FREAK_getDefaultName_const(const cv::xfeatures2d::FREAK* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getDefaultName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
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

	// create(int, float, float, int, int)(Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1104
	// ("cv::xfeatures2d::HarrisLaplaceFeatureDetector::create", vec![(pred!(mut, ["numOctaves", "corn_thresh", "DOG_thresh", "maxCorners", "num_layers"], ["int", "float", "float", "int", "int"]), _)]),
	void cv_xfeatures2d_HarrisLaplaceFeatureDetector_create_int_float_float_int_int(int numOctaves, float corn_thresh, float DOG_thresh, int maxCorners, int num_layers, Result<cv::Ptr<cv::xfeatures2d::HarrisLaplaceFeatureDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::HarrisLaplaceFeatureDetector> ret = cv::xfeatures2d::HarrisLaplaceFeatureDetector::create(numOctaves, corn_thresh, DOG_thresh, maxCorners, num_layers);
			Ok(new cv::Ptr<cv::xfeatures2d::HarrisLaplaceFeatureDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::HarrisLaplaceFeatureDetector::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1104
	// ("cv::xfeatures2d::HarrisLaplaceFeatureDetector::create", vec![(pred!(mut, [], []), _)]),
	void cv_xfeatures2d_HarrisLaplaceFeatureDetector_create(Result<cv::Ptr<cv::xfeatures2d::HarrisLaplaceFeatureDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::HarrisLaplaceFeatureDetector> ret = cv::xfeatures2d::HarrisLaplaceFeatureDetector::create();
			Ok(new cv::Ptr<cv::xfeatures2d::HarrisLaplaceFeatureDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNumOctaves(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1111
	// ("cv::xfeatures2d::HarrisLaplaceFeatureDetector::setNumOctaves", vec![(pred!(mut, ["numOctaves_"], ["int"]), _)]),
	void cv_xfeatures2d_HarrisLaplaceFeatureDetector_setNumOctaves_int(cv::xfeatures2d::HarrisLaplaceFeatureDetector* instance, int numOctaves_, ResultVoid* ocvrs_return) {
		try {
			instance->setNumOctaves(numOctaves_);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNumOctaves()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1112
	// ("cv::xfeatures2d::HarrisLaplaceFeatureDetector::getNumOctaves", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_HarrisLaplaceFeatureDetector_getNumOctaves_const(const cv::xfeatures2d::HarrisLaplaceFeatureDetector* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNumOctaves();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setCornThresh(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1114
	// ("cv::xfeatures2d::HarrisLaplaceFeatureDetector::setCornThresh", vec![(pred!(mut, ["corn_thresh_"], ["float"]), _)]),
	void cv_xfeatures2d_HarrisLaplaceFeatureDetector_setCornThresh_float(cv::xfeatures2d::HarrisLaplaceFeatureDetector* instance, float corn_thresh_, ResultVoid* ocvrs_return) {
		try {
			instance->setCornThresh(corn_thresh_);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getCornThresh()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1115
	// ("cv::xfeatures2d::HarrisLaplaceFeatureDetector::getCornThresh", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_HarrisLaplaceFeatureDetector_getCornThresh_const(const cv::xfeatures2d::HarrisLaplaceFeatureDetector* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getCornThresh();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDOGThresh(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1117
	// ("cv::xfeatures2d::HarrisLaplaceFeatureDetector::setDOGThresh", vec![(pred!(mut, ["DOG_thresh_"], ["float"]), _)]),
	void cv_xfeatures2d_HarrisLaplaceFeatureDetector_setDOGThresh_float(cv::xfeatures2d::HarrisLaplaceFeatureDetector* instance, float DOG_thresh_, ResultVoid* ocvrs_return) {
		try {
			instance->setDOGThresh(DOG_thresh_);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDOGThresh()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1118
	// ("cv::xfeatures2d::HarrisLaplaceFeatureDetector::getDOGThresh", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_HarrisLaplaceFeatureDetector_getDOGThresh_const(const cv::xfeatures2d::HarrisLaplaceFeatureDetector* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getDOGThresh();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxCorners(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1120
	// ("cv::xfeatures2d::HarrisLaplaceFeatureDetector::setMaxCorners", vec![(pred!(mut, ["maxCorners_"], ["int"]), _)]),
	void cv_xfeatures2d_HarrisLaplaceFeatureDetector_setMaxCorners_int(cv::xfeatures2d::HarrisLaplaceFeatureDetector* instance, int maxCorners_, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxCorners(maxCorners_);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxCorners()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1121
	// ("cv::xfeatures2d::HarrisLaplaceFeatureDetector::getMaxCorners", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_HarrisLaplaceFeatureDetector_getMaxCorners_const(const cv::xfeatures2d::HarrisLaplaceFeatureDetector* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMaxCorners();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNumLayers(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1123
	// ("cv::xfeatures2d::HarrisLaplaceFeatureDetector::setNumLayers", vec![(pred!(mut, ["num_layers_"], ["int"]), _)]),
	void cv_xfeatures2d_HarrisLaplaceFeatureDetector_setNumLayers_int(cv::xfeatures2d::HarrisLaplaceFeatureDetector* instance, int num_layers_, ResultVoid* ocvrs_return) {
		try {
			instance->setNumLayers(num_layers_);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNumLayers()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1124
	// ("cv::xfeatures2d::HarrisLaplaceFeatureDetector::getNumLayers", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_HarrisLaplaceFeatureDetector_getNumLayers_const(const cv::xfeatures2d::HarrisLaplaceFeatureDetector* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNumLayers();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1126
	// ("cv::xfeatures2d::HarrisLaplaceFeatureDetector::getDefaultName", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_HarrisLaplaceFeatureDetector_getDefaultName_const(const cv::xfeatures2d::HarrisLaplaceFeatureDetector* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getDefaultName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
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

	// create(bool, bool, float, int, int, int)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1364
	// ("cv::xfeatures2d::KAZE::create", vec![(pred!(mut, ["extended", "upright", "threshold", "nOctaves", "nOctaveLayers", "diffusivity"], ["bool", "bool", "float", "int", "int", "int"]), _)]),
	void cv_xfeatures2d_KAZE_create_bool_bool_float_int_int_int(bool extended, bool upright, float threshold, int nOctaves, int nOctaveLayers, int diffusivity, Result<cv::Ptr<cv::xfeatures2d::KAZE>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::KAZE> ret = cv::xfeatures2d::KAZE::create(extended, upright, threshold, nOctaves, nOctaveLayers, diffusivity);
			Ok(new cv::Ptr<cv::xfeatures2d::KAZE>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::KAZE::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1364
	// ("cv::xfeatures2d::KAZE::create", vec![(pred!(mut, [], []), _)]),
	void cv_xfeatures2d_KAZE_create(Result<cv::Ptr<cv::xfeatures2d::KAZE>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::KAZE> ret = cv::xfeatures2d::KAZE::create();
			Ok(new cv::Ptr<cv::xfeatures2d::KAZE>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setExtended(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1369
	// ("cv::xfeatures2d::KAZE::setExtended", vec![(pred!(mut, ["extended"], ["bool"]), _)]),
	void cv_xfeatures2d_KAZE_setExtended_bool(cv::xfeatures2d::KAZE* instance, bool extended, ResultVoid* ocvrs_return) {
		try {
			instance->setExtended(extended);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getExtended()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1370
	// ("cv::xfeatures2d::KAZE::getExtended", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_KAZE_getExtended_const(const cv::xfeatures2d::KAZE* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getExtended();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setUpright(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1372
	// ("cv::xfeatures2d::KAZE::setUpright", vec![(pred!(mut, ["upright"], ["bool"]), _)]),
	void cv_xfeatures2d_KAZE_setUpright_bool(cv::xfeatures2d::KAZE* instance, bool upright, ResultVoid* ocvrs_return) {
		try {
			instance->setUpright(upright);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getUpright()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1373
	// ("cv::xfeatures2d::KAZE::getUpright", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_KAZE_getUpright_const(const cv::xfeatures2d::KAZE* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUpright();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1375
	// ("cv::xfeatures2d::KAZE::setThreshold", vec![(pred!(mut, ["threshold"], ["double"]), _)]),
	void cv_xfeatures2d_KAZE_setThreshold_double(cv::xfeatures2d::KAZE* instance, double threshold, ResultVoid* ocvrs_return) {
		try {
			instance->setThreshold(threshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1376
	// ("cv::xfeatures2d::KAZE::getThreshold", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_KAZE_getThreshold_const(const cv::xfeatures2d::KAZE* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNOctaves(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1378
	// ("cv::xfeatures2d::KAZE::setNOctaves", vec![(pred!(mut, ["octaves"], ["int"]), _)]),
	void cv_xfeatures2d_KAZE_setNOctaves_int(cv::xfeatures2d::KAZE* instance, int octaves, ResultVoid* ocvrs_return) {
		try {
			instance->setNOctaves(octaves);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNOctaves()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1379
	// ("cv::xfeatures2d::KAZE::getNOctaves", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_KAZE_getNOctaves_const(const cv::xfeatures2d::KAZE* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNOctaves();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNOctaveLayers(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1381
	// ("cv::xfeatures2d::KAZE::setNOctaveLayers", vec![(pred!(mut, ["octaveLayers"], ["int"]), _)]),
	void cv_xfeatures2d_KAZE_setNOctaveLayers_int(cv::xfeatures2d::KAZE* instance, int octaveLayers, ResultVoid* ocvrs_return) {
		try {
			instance->setNOctaveLayers(octaveLayers);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNOctaveLayers()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1382
	// ("cv::xfeatures2d::KAZE::getNOctaveLayers", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_KAZE_getNOctaveLayers_const(const cv::xfeatures2d::KAZE* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNOctaveLayers();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDiffusivity(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1384
	// ("cv::xfeatures2d::KAZE::setDiffusivity", vec![(pred!(mut, ["diff"], ["int"]), _)]),
	void cv_xfeatures2d_KAZE_setDiffusivity_int(cv::xfeatures2d::KAZE* instance, int diff, ResultVoid* ocvrs_return) {
		try {
			instance->setDiffusivity(diff);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDiffusivity()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1385
	// ("cv::xfeatures2d::KAZE::getDiffusivity", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_KAZE_getDiffusivity_const(const cv::xfeatures2d::KAZE* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getDiffusivity();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1386
	// ("cv::xfeatures2d::KAZE::getDefaultName", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_KAZE_getDefaultName_const(const cv::xfeatures2d::KAZE* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getDefaultName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::KAZE::to_Algorithm() generated
	// ("cv::xfeatures2d::KAZE::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_xfeatures2d_KAZE_to_Algorithm(cv::xfeatures2d::KAZE* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::xfeatures2d::KAZE::to_Feature2D() generated
	// ("cv::xfeatures2d::KAZE::to_Feature2D", vec![(pred!(mut, [], []), _)]),
	cv::Feature2D* cv_xfeatures2d_KAZE_to_Feature2D(cv::xfeatures2d::KAZE* instance) {
			return dynamic_cast<cv::Feature2D*>(instance);
	}

	// cv::xfeatures2d::KAZE::delete() generated
	// ("cv::xfeatures2d::KAZE::delete", vec![(pred!(mut, [], []), _)]),
	void cv_xfeatures2d_KAZE_delete(cv::xfeatures2d::KAZE* instance) {
			delete instance;
	}

	// create(int, bool, int, double)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:229
	// ("cv::xfeatures2d::LATCH::create", vec![(pred!(mut, ["bytes", "rotationInvariance", "half_ssd_size", "sigma"], ["int", "bool", "int", "double"]), _)]),
	void cv_xfeatures2d_LATCH_create_int_bool_int_double(int bytes, bool rotationInvariance, int half_ssd_size, double sigma, Result<cv::Ptr<cv::xfeatures2d::LATCH>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::LATCH> ret = cv::xfeatures2d::LATCH::create(bytes, rotationInvariance, half_ssd_size, sigma);
			Ok(new cv::Ptr<cv::xfeatures2d::LATCH>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::LATCH::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:229
	// ("cv::xfeatures2d::LATCH::create", vec![(pred!(mut, [], []), _)]),
	void cv_xfeatures2d_LATCH_create(Result<cv::Ptr<cv::xfeatures2d::LATCH>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::LATCH> ret = cv::xfeatures2d::LATCH::create();
			Ok(new cv::Ptr<cv::xfeatures2d::LATCH>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setBytes(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:231
	// ("cv::xfeatures2d::LATCH::setBytes", vec![(pred!(mut, ["bytes"], ["int"]), _)]),
	void cv_xfeatures2d_LATCH_setBytes_int(cv::xfeatures2d::LATCH* instance, int bytes, ResultVoid* ocvrs_return) {
		try {
			instance->setBytes(bytes);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getBytes()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:232
	// ("cv::xfeatures2d::LATCH::getBytes", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_LATCH_getBytes_const(const cv::xfeatures2d::LATCH* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getBytes();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setRotationInvariance(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:234
	// ("cv::xfeatures2d::LATCH::setRotationInvariance", vec![(pred!(mut, ["rotationInvariance"], ["bool"]), _)]),
	void cv_xfeatures2d_LATCH_setRotationInvariance_bool(cv::xfeatures2d::LATCH* instance, bool rotationInvariance, ResultVoid* ocvrs_return) {
		try {
			instance->setRotationInvariance(rotationInvariance);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getRotationInvariance()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:235
	// ("cv::xfeatures2d::LATCH::getRotationInvariance", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_LATCH_getRotationInvariance_const(const cv::xfeatures2d::LATCH* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getRotationInvariance();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setHalfSSDsize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:237
	// ("cv::xfeatures2d::LATCH::setHalfSSDsize", vec![(pred!(mut, ["half_ssd_size"], ["int"]), _)]),
	void cv_xfeatures2d_LATCH_setHalfSSDsize_int(cv::xfeatures2d::LATCH* instance, int half_ssd_size, ResultVoid* ocvrs_return) {
		try {
			instance->setHalfSSDsize(half_ssd_size);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getHalfSSDsize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:238
	// ("cv::xfeatures2d::LATCH::getHalfSSDsize", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_LATCH_getHalfSSDsize_const(const cv::xfeatures2d::LATCH* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getHalfSSDsize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSigma(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:240
	// ("cv::xfeatures2d::LATCH::setSigma", vec![(pred!(mut, ["sigma"], ["double"]), _)]),
	void cv_xfeatures2d_LATCH_setSigma_double(cv::xfeatures2d::LATCH* instance, double sigma, ResultVoid* ocvrs_return) {
		try {
			instance->setSigma(sigma);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSigma()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:241
	// ("cv::xfeatures2d::LATCH::getSigma", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_LATCH_getSigma_const(const cv::xfeatures2d::LATCH* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getSigma();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:243
	// ("cv::xfeatures2d::LATCH::getDefaultName", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_LATCH_getDefaultName_const(const cv::xfeatures2d::LATCH* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getDefaultName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
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

	// create(const int, const int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:192
	// ("cv::xfeatures2d::LUCID::create", vec![(pred!(mut, ["lucid_kernel", "blur_kernel"], ["const int", "const int"]), _)]),
	void cv_xfeatures2d_LUCID_create_const_int_const_int(const int lucid_kernel, const int blur_kernel, Result<cv::Ptr<cv::xfeatures2d::LUCID>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::LUCID> ret = cv::xfeatures2d::LUCID::create(lucid_kernel, blur_kernel);
			Ok(new cv::Ptr<cv::xfeatures2d::LUCID>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::LUCID::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:192
	// ("cv::xfeatures2d::LUCID::create", vec![(pred!(mut, [], []), _)]),
	void cv_xfeatures2d_LUCID_create(Result<cv::Ptr<cv::xfeatures2d::LUCID>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::LUCID> ret = cv::xfeatures2d::LUCID::create();
			Ok(new cv::Ptr<cv::xfeatures2d::LUCID>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setLucidKernel(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:194
	// ("cv::xfeatures2d::LUCID::setLucidKernel", vec![(pred!(mut, ["lucid_kernel"], ["int"]), _)]),
	void cv_xfeatures2d_LUCID_setLucidKernel_int(cv::xfeatures2d::LUCID* instance, int lucid_kernel, ResultVoid* ocvrs_return) {
		try {
			instance->setLucidKernel(lucid_kernel);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLucidKernel()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:195
	// ("cv::xfeatures2d::LUCID::getLucidKernel", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_LUCID_getLucidKernel_const(const cv::xfeatures2d::LUCID* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getLucidKernel();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setBlurKernel(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:197
	// ("cv::xfeatures2d::LUCID::setBlurKernel", vec![(pred!(mut, ["blur_kernel"], ["int"]), _)]),
	void cv_xfeatures2d_LUCID_setBlurKernel_int(cv::xfeatures2d::LUCID* instance, int blur_kernel, ResultVoid* ocvrs_return) {
		try {
			instance->setBlurKernel(blur_kernel);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getBlurKernel()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:198
	// ("cv::xfeatures2d::LUCID::getBlurKernel", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_LUCID_getBlurKernel_const(const cv::xfeatures2d::LUCID* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getBlurKernel();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:200
	// ("cv::xfeatures2d::LUCID::getDefaultName", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_LUCID_getDefaultName_const(const cv::xfeatures2d::LUCID* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getDefaultName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
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

	// create(int, int, int, int, float, int, float, int, bool)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:471
	// ("cv::xfeatures2d::MSDDetector::create", vec![(pred!(mut, ["m_patch_radius", "m_search_area_radius", "m_nms_radius", "m_nms_scale_radius", "m_th_saliency", "m_kNN", "m_scale_factor", "m_n_scales", "m_compute_orientation"], ["int", "int", "int", "int", "float", "int", "float", "int", "bool"]), _)]),
	void cv_xfeatures2d_MSDDetector_create_int_int_int_int_float_int_float_int_bool(int m_patch_radius, int m_search_area_radius, int m_nms_radius, int m_nms_scale_radius, float m_th_saliency, int m_kNN, float m_scale_factor, int m_n_scales, bool m_compute_orientation, Result<cv::Ptr<cv::xfeatures2d::MSDDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::MSDDetector> ret = cv::xfeatures2d::MSDDetector::create(m_patch_radius, m_search_area_radius, m_nms_radius, m_nms_scale_radius, m_th_saliency, m_kNN, m_scale_factor, m_n_scales, m_compute_orientation);
			Ok(new cv::Ptr<cv::xfeatures2d::MSDDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::MSDDetector::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:471
	// ("cv::xfeatures2d::MSDDetector::create", vec![(pred!(mut, [], []), _)]),
	void cv_xfeatures2d_MSDDetector_create(Result<cv::Ptr<cv::xfeatures2d::MSDDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::MSDDetector> ret = cv::xfeatures2d::MSDDetector::create();
			Ok(new cv::Ptr<cv::xfeatures2d::MSDDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setPatchRadius(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:475
	// ("cv::xfeatures2d::MSDDetector::setPatchRadius", vec![(pred!(mut, ["patch_radius"], ["int"]), _)]),
	void cv_xfeatures2d_MSDDetector_setPatchRadius_int(cv::xfeatures2d::MSDDetector* instance, int patch_radius, ResultVoid* ocvrs_return) {
		try {
			instance->setPatchRadius(patch_radius);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPatchRadius()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:476
	// ("cv::xfeatures2d::MSDDetector::getPatchRadius", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_MSDDetector_getPatchRadius_const(const cv::xfeatures2d::MSDDetector* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getPatchRadius();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSearchAreaRadius(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:478
	// ("cv::xfeatures2d::MSDDetector::setSearchAreaRadius", vec![(pred!(mut, ["use_orientation"], ["int"]), _)]),
	void cv_xfeatures2d_MSDDetector_setSearchAreaRadius_int(cv::xfeatures2d::MSDDetector* instance, int use_orientation, ResultVoid* ocvrs_return) {
		try {
			instance->setSearchAreaRadius(use_orientation);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSearchAreaRadius()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:479
	// ("cv::xfeatures2d::MSDDetector::getSearchAreaRadius", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_MSDDetector_getSearchAreaRadius_const(const cv::xfeatures2d::MSDDetector* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getSearchAreaRadius();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNmsRadius(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:481
	// ("cv::xfeatures2d::MSDDetector::setNmsRadius", vec![(pred!(mut, ["nms_radius"], ["int"]), _)]),
	void cv_xfeatures2d_MSDDetector_setNmsRadius_int(cv::xfeatures2d::MSDDetector* instance, int nms_radius, ResultVoid* ocvrs_return) {
		try {
			instance->setNmsRadius(nms_radius);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNmsRadius()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:482
	// ("cv::xfeatures2d::MSDDetector::getNmsRadius", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_MSDDetector_getNmsRadius_const(const cv::xfeatures2d::MSDDetector* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNmsRadius();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNmsScaleRadius(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:484
	// ("cv::xfeatures2d::MSDDetector::setNmsScaleRadius", vec![(pred!(mut, ["nms_scale_radius"], ["int"]), _)]),
	void cv_xfeatures2d_MSDDetector_setNmsScaleRadius_int(cv::xfeatures2d::MSDDetector* instance, int nms_scale_radius, ResultVoid* ocvrs_return) {
		try {
			instance->setNmsScaleRadius(nms_scale_radius);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNmsScaleRadius()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:485
	// ("cv::xfeatures2d::MSDDetector::getNmsScaleRadius", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_MSDDetector_getNmsScaleRadius_const(const cv::xfeatures2d::MSDDetector* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNmsScaleRadius();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setThSaliency(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:487
	// ("cv::xfeatures2d::MSDDetector::setThSaliency", vec![(pred!(mut, ["th_saliency"], ["float"]), _)]),
	void cv_xfeatures2d_MSDDetector_setThSaliency_float(cv::xfeatures2d::MSDDetector* instance, float th_saliency, ResultVoid* ocvrs_return) {
		try {
			instance->setThSaliency(th_saliency);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getThSaliency()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:488
	// ("cv::xfeatures2d::MSDDetector::getThSaliency", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_MSDDetector_getThSaliency_const(const cv::xfeatures2d::MSDDetector* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getThSaliency();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setKNN(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:490
	// ("cv::xfeatures2d::MSDDetector::setKNN", vec![(pred!(mut, ["kNN"], ["int"]), _)]),
	void cv_xfeatures2d_MSDDetector_setKNN_int(cv::xfeatures2d::MSDDetector* instance, int kNN, ResultVoid* ocvrs_return) {
		try {
			instance->setKNN(kNN);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getKNN()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:491
	// ("cv::xfeatures2d::MSDDetector::getKNN", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_MSDDetector_getKNN_const(const cv::xfeatures2d::MSDDetector* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getKNN();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setScaleFactor(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:493
	// ("cv::xfeatures2d::MSDDetector::setScaleFactor", vec![(pred!(mut, ["scale_factor"], ["float"]), _)]),
	void cv_xfeatures2d_MSDDetector_setScaleFactor_float(cv::xfeatures2d::MSDDetector* instance, float scale_factor, ResultVoid* ocvrs_return) {
		try {
			instance->setScaleFactor(scale_factor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getScaleFactor()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:494
	// ("cv::xfeatures2d::MSDDetector::getScaleFactor", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_MSDDetector_getScaleFactor_const(const cv::xfeatures2d::MSDDetector* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getScaleFactor();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNScales(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:496
	// ("cv::xfeatures2d::MSDDetector::setNScales", vec![(pred!(mut, ["use_orientation"], ["int"]), _)]),
	void cv_xfeatures2d_MSDDetector_setNScales_int(cv::xfeatures2d::MSDDetector* instance, int use_orientation, ResultVoid* ocvrs_return) {
		try {
			instance->setNScales(use_orientation);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNScales()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:497
	// ("cv::xfeatures2d::MSDDetector::getNScales", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_MSDDetector_getNScales_const(const cv::xfeatures2d::MSDDetector* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNScales();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setComputeOrientation(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:499
	// ("cv::xfeatures2d::MSDDetector::setComputeOrientation", vec![(pred!(mut, ["compute_orientation"], ["bool"]), _)]),
	void cv_xfeatures2d_MSDDetector_setComputeOrientation_bool(cv::xfeatures2d::MSDDetector* instance, bool compute_orientation, ResultVoid* ocvrs_return) {
		try {
			instance->setComputeOrientation(compute_orientation);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getComputeOrientation()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:500
	// ("cv::xfeatures2d::MSDDetector::getComputeOrientation", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_MSDDetector_getComputeOrientation_const(const cv::xfeatures2d::MSDDetector* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getComputeOrientation();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:502
	// ("cv::xfeatures2d::MSDDetector::getDefaultName", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_MSDDetector_getDefaultName_const(const cv::xfeatures2d::MSDDetector* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getDefaultName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
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

	// create(const int, const int, const int)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:670
	// ("cv::xfeatures2d::PCTSignatures::create", vec![(pred!(mut, ["initSampleCount", "initSeedCount", "pointDistribution"], ["const int", "const int", "const int"]), _)]),
	void cv_xfeatures2d_PCTSignatures_create_const_int_const_int_const_int(const int initSampleCount, const int initSeedCount, const int pointDistribution, Result<cv::Ptr<cv::xfeatures2d::PCTSignatures>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::PCTSignatures> ret = cv::xfeatures2d::PCTSignatures::create(initSampleCount, initSeedCount, pointDistribution);
			Ok(new cv::Ptr<cv::xfeatures2d::PCTSignatures>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::PCTSignatures::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:670
	// ("cv::xfeatures2d::PCTSignatures::create", vec![(pred!(mut, [], []), _)]),
	void cv_xfeatures2d_PCTSignatures_create(Result<cv::Ptr<cv::xfeatures2d::PCTSignatures>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::PCTSignatures> ret = cv::xfeatures2d::PCTSignatures::create();
			Ok(new cv::Ptr<cv::xfeatures2d::PCTSignatures>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const std::vector<Point2f> &, const int)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:684
	// ("cv::xfeatures2d::PCTSignatures::create", vec![(pred!(mut, ["initSamplingPoints", "initSeedCount"], ["const std::vector<cv::Point2f>*", "const int"]), _)]),
	void cv_xfeatures2d_PCTSignatures_create_const_vectorLPoint2fGR_const_int(const std::vector<cv::Point2f>* initSamplingPoints, const int initSeedCount, Result<cv::Ptr<cv::xfeatures2d::PCTSignatures>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::PCTSignatures> ret = cv::xfeatures2d::PCTSignatures::create(*initSamplingPoints, initSeedCount);
			Ok(new cv::Ptr<cv::xfeatures2d::PCTSignatures>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const std::vector<Point2f> &, const std::vector<int> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:696
	// ("cv::xfeatures2d::PCTSignatures::create", vec![(pred!(mut, ["initSamplingPoints", "initClusterSeedIndexes"], ["const std::vector<cv::Point2f>*", "const std::vector<int>*"]), _)]),
	void cv_xfeatures2d_PCTSignatures_create_const_vectorLPoint2fGR_const_vectorLintGR(const std::vector<cv::Point2f>* initSamplingPoints, const std::vector<int>* initClusterSeedIndexes, Result<cv::Ptr<cv::xfeatures2d::PCTSignatures>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::PCTSignatures> ret = cv::xfeatures2d::PCTSignatures::create(*initSamplingPoints, *initClusterSeedIndexes);
			Ok(new cv::Ptr<cv::xfeatures2d::PCTSignatures>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// computeSignature(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:707
	// ("cv::xfeatures2d::PCTSignatures::computeSignature", vec![(pred!(const, ["image", "signature"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_xfeatures2d_PCTSignatures_computeSignature_const_const__InputArrayR_const__OutputArrayR(const cv::xfeatures2d::PCTSignatures* instance, const cv::_InputArray* image, const cv::_OutputArray* signature, ResultVoid* ocvrs_return) {
		try {
			instance->computeSignature(*image, *signature);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// computeSignatures(const std::vector<Mat> &, std::vector<Mat> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:716
	// ("cv::xfeatures2d::PCTSignatures::computeSignatures", vec![(pred!(const, ["images", "signatures"], ["const std::vector<cv::Mat>*", "std::vector<cv::Mat>*"]), _)]),
	void cv_xfeatures2d_PCTSignatures_computeSignatures_const_const_vectorLMatGR_vectorLMatGR(const cv::xfeatures2d::PCTSignatures* instance, const std::vector<cv::Mat>* images, std::vector<cv::Mat>* signatures, ResultVoid* ocvrs_return) {
		try {
			instance->computeSignatures(*images, *signatures);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// drawSignature(InputArray, InputArray, OutputArray, float, int)(InputArray, InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:732
	// ("cv::xfeatures2d::PCTSignatures::drawSignature", vec![(pred!(mut, ["source", "signature", "result", "radiusToShorterSideRatio", "borderThickness"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "float", "int"]), _)]),
	void cv_xfeatures2d_PCTSignatures_drawSignature_const__InputArrayR_const__InputArrayR_const__OutputArrayR_float_int(const cv::_InputArray* source, const cv::_InputArray* signature, const cv::_OutputArray* result, float radiusToShorterSideRatio, int borderThickness, ResultVoid* ocvrs_return) {
		try {
			cv::xfeatures2d::PCTSignatures::drawSignature(*source, *signature, *result, radiusToShorterSideRatio, borderThickness);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::PCTSignatures::drawSignature(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:732
	// ("cv::xfeatures2d::PCTSignatures::drawSignature", vec![(pred!(mut, ["source", "signature", "result"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_xfeatures2d_PCTSignatures_drawSignature_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* source, const cv::_InputArray* signature, const cv::_OutputArray* result, ResultVoid* ocvrs_return) {
		try {
			cv::xfeatures2d::PCTSignatures::drawSignature(*source, *signature, *result);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// generateInitPoints(std::vector<Point2f> &, const int, int)(CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:747
	// ("cv::xfeatures2d::PCTSignatures::generateInitPoints", vec![(pred!(mut, ["initPoints", "count", "pointDistribution"], ["std::vector<cv::Point2f>*", "const int", "int"]), _)]),
	void cv_xfeatures2d_PCTSignatures_generateInitPoints_vectorLPoint2fGR_const_int_int(std::vector<cv::Point2f>* initPoints, const int count, int pointDistribution, ResultVoid* ocvrs_return) {
		try {
			cv::xfeatures2d::PCTSignatures::generateInitPoints(*initPoints, count, pointDistribution);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSampleCount()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:758
	// ("cv::xfeatures2d::PCTSignatures::getSampleCount", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_PCTSignatures_getSampleCount_const(const cv::xfeatures2d::PCTSignatures* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getSampleCount();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getGrayscaleBits()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:765
	// ("cv::xfeatures2d::PCTSignatures::getGrayscaleBits", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_PCTSignatures_getGrayscaleBits_const(const cv::xfeatures2d::PCTSignatures* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getGrayscaleBits();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setGrayscaleBits(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:771
	// ("cv::xfeatures2d::PCTSignatures::setGrayscaleBits", vec![(pred!(mut, ["grayscaleBits"], ["int"]), _)]),
	void cv_xfeatures2d_PCTSignatures_setGrayscaleBits_int(cv::xfeatures2d::PCTSignatures* instance, int grayscaleBits, ResultVoid* ocvrs_return) {
		try {
			instance->setGrayscaleBits(grayscaleBits);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getWindowRadius()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:778
	// ("cv::xfeatures2d::PCTSignatures::getWindowRadius", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_PCTSignatures_getWindowRadius_const(const cv::xfeatures2d::PCTSignatures* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getWindowRadius();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWindowRadius(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:784
	// ("cv::xfeatures2d::PCTSignatures::setWindowRadius", vec![(pred!(mut, ["radius"], ["int"]), _)]),
	void cv_xfeatures2d_PCTSignatures_setWindowRadius_int(cv::xfeatures2d::PCTSignatures* instance, int radius, ResultVoid* ocvrs_return) {
		try {
			instance->setWindowRadius(radius);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getWeightX()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:791
	// ("cv::xfeatures2d::PCTSignatures::getWeightX", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_PCTSignatures_getWeightX_const(const cv::xfeatures2d::PCTSignatures* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getWeightX();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWeightX(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:796
	// ("cv::xfeatures2d::PCTSignatures::setWeightX", vec![(pred!(mut, ["weight"], ["float"]), _)]),
	void cv_xfeatures2d_PCTSignatures_setWeightX_float(cv::xfeatures2d::PCTSignatures* instance, float weight, ResultVoid* ocvrs_return) {
		try {
			instance->setWeightX(weight);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getWeightY()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:802
	// ("cv::xfeatures2d::PCTSignatures::getWeightY", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_PCTSignatures_getWeightY_const(const cv::xfeatures2d::PCTSignatures* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getWeightY();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWeightY(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:807
	// ("cv::xfeatures2d::PCTSignatures::setWeightY", vec![(pred!(mut, ["weight"], ["float"]), _)]),
	void cv_xfeatures2d_PCTSignatures_setWeightY_float(cv::xfeatures2d::PCTSignatures* instance, float weight, ResultVoid* ocvrs_return) {
		try {
			instance->setWeightY(weight);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getWeightL()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:813
	// ("cv::xfeatures2d::PCTSignatures::getWeightL", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_PCTSignatures_getWeightL_const(const cv::xfeatures2d::PCTSignatures* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getWeightL();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWeightL(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:818
	// ("cv::xfeatures2d::PCTSignatures::setWeightL", vec![(pred!(mut, ["weight"], ["float"]), _)]),
	void cv_xfeatures2d_PCTSignatures_setWeightL_float(cv::xfeatures2d::PCTSignatures* instance, float weight, ResultVoid* ocvrs_return) {
		try {
			instance->setWeightL(weight);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getWeightA()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:824
	// ("cv::xfeatures2d::PCTSignatures::getWeightA", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_PCTSignatures_getWeightA_const(const cv::xfeatures2d::PCTSignatures* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getWeightA();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWeightA(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:829
	// ("cv::xfeatures2d::PCTSignatures::setWeightA", vec![(pred!(mut, ["weight"], ["float"]), _)]),
	void cv_xfeatures2d_PCTSignatures_setWeightA_float(cv::xfeatures2d::PCTSignatures* instance, float weight, ResultVoid* ocvrs_return) {
		try {
			instance->setWeightA(weight);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getWeightB()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:835
	// ("cv::xfeatures2d::PCTSignatures::getWeightB", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_PCTSignatures_getWeightB_const(const cv::xfeatures2d::PCTSignatures* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getWeightB();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWeightB(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:840
	// ("cv::xfeatures2d::PCTSignatures::setWeightB", vec![(pred!(mut, ["weight"], ["float"]), _)]),
	void cv_xfeatures2d_PCTSignatures_setWeightB_float(cv::xfeatures2d::PCTSignatures* instance, float weight, ResultVoid* ocvrs_return) {
		try {
			instance->setWeightB(weight);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getWeightContrast()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:846
	// ("cv::xfeatures2d::PCTSignatures::getWeightContrast", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_PCTSignatures_getWeightContrast_const(const cv::xfeatures2d::PCTSignatures* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getWeightContrast();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWeightContrast(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:851
	// ("cv::xfeatures2d::PCTSignatures::setWeightContrast", vec![(pred!(mut, ["weight"], ["float"]), _)]),
	void cv_xfeatures2d_PCTSignatures_setWeightContrast_float(cv::xfeatures2d::PCTSignatures* instance, float weight, ResultVoid* ocvrs_return) {
		try {
			instance->setWeightContrast(weight);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getWeightEntropy()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:857
	// ("cv::xfeatures2d::PCTSignatures::getWeightEntropy", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_PCTSignatures_getWeightEntropy_const(const cv::xfeatures2d::PCTSignatures* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getWeightEntropy();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWeightEntropy(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:862
	// ("cv::xfeatures2d::PCTSignatures::setWeightEntropy", vec![(pred!(mut, ["weight"], ["float"]), _)]),
	void cv_xfeatures2d_PCTSignatures_setWeightEntropy_float(cv::xfeatures2d::PCTSignatures* instance, float weight, ResultVoid* ocvrs_return) {
		try {
			instance->setWeightEntropy(weight);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSamplingPoints()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:868
	// ("cv::xfeatures2d::PCTSignatures::getSamplingPoints", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_PCTSignatures_getSamplingPoints_const(const cv::xfeatures2d::PCTSignatures* instance, Result<std::vector<cv::Point2f>*>* ocvrs_return) {
		try {
			std::vector<cv::Point2f> ret = instance->getSamplingPoints();
			Ok(new std::vector<cv::Point2f>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWeight(int, float)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:886
	// ("cv::xfeatures2d::PCTSignatures::setWeight", vec![(pred!(mut, ["idx", "value"], ["int", "float"]), _)]),
	void cv_xfeatures2d_PCTSignatures_setWeight_int_float(cv::xfeatures2d::PCTSignatures* instance, int idx, float value, ResultVoid* ocvrs_return) {
		try {
			instance->setWeight(idx, value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWeights(const std::vector<float> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:900
	// ("cv::xfeatures2d::PCTSignatures::setWeights", vec![(pred!(mut, ["weights"], ["const std::vector<float>*"]), _)]),
	void cv_xfeatures2d_PCTSignatures_setWeights_const_vectorLfloatGR(cv::xfeatures2d::PCTSignatures* instance, const std::vector<float>* weights, ResultVoid* ocvrs_return) {
		try {
			instance->setWeights(*weights);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTranslation(int, float)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:916
	// ("cv::xfeatures2d::PCTSignatures::setTranslation", vec![(pred!(mut, ["idx", "value"], ["int", "float"]), _)]),
	void cv_xfeatures2d_PCTSignatures_setTranslation_int_float(cv::xfeatures2d::PCTSignatures* instance, int idx, float value, ResultVoid* ocvrs_return) {
		try {
			instance->setTranslation(idx, value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTranslations(const std::vector<float> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:930
	// ("cv::xfeatures2d::PCTSignatures::setTranslations", vec![(pred!(mut, ["translations"], ["const std::vector<float>*"]), _)]),
	void cv_xfeatures2d_PCTSignatures_setTranslations_const_vectorLfloatGR(cv::xfeatures2d::PCTSignatures* instance, const std::vector<float>* translations, ResultVoid* ocvrs_return) {
		try {
			instance->setTranslations(*translations);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSamplingPoints(std::vector<Point2f>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:937
	// ("cv::xfeatures2d::PCTSignatures::setSamplingPoints", vec![(pred!(mut, ["samplingPoints"], ["std::vector<cv::Point2f>"]), _)]),
	void cv_xfeatures2d_PCTSignatures_setSamplingPoints_vectorLPoint2fG(cv::xfeatures2d::PCTSignatures* instance, std::vector<cv::Point2f>* samplingPoints, ResultVoid* ocvrs_return) {
		try {
			instance->setSamplingPoints(*samplingPoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getInitSeedIndexes()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:945
	// ("cv::xfeatures2d::PCTSignatures::getInitSeedIndexes", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_PCTSignatures_getInitSeedIndexes_const(const cv::xfeatures2d::PCTSignatures* instance, Result<std::vector<int>*>* ocvrs_return) {
		try {
			std::vector<int> ret = instance->getInitSeedIndexes();
			Ok(new std::vector<int>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setInitSeedIndexes(std::vector<int>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:949
	// ("cv::xfeatures2d::PCTSignatures::setInitSeedIndexes", vec![(pred!(mut, ["initSeedIndexes"], ["std::vector<int>"]), _)]),
	void cv_xfeatures2d_PCTSignatures_setInitSeedIndexes_vectorLintG(cv::xfeatures2d::PCTSignatures* instance, std::vector<int>* initSeedIndexes, ResultVoid* ocvrs_return) {
		try {
			instance->setInitSeedIndexes(*initSeedIndexes);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getInitSeedCount()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:953
	// ("cv::xfeatures2d::PCTSignatures::getInitSeedCount", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_PCTSignatures_getInitSeedCount_const(const cv::xfeatures2d::PCTSignatures* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getInitSeedCount();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getIterationCount()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:960
	// ("cv::xfeatures2d::PCTSignatures::getIterationCount", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_PCTSignatures_getIterationCount_const(const cv::xfeatures2d::PCTSignatures* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getIterationCount();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setIterationCount(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:966
	// ("cv::xfeatures2d::PCTSignatures::setIterationCount", vec![(pred!(mut, ["iterationCount"], ["int"]), _)]),
	void cv_xfeatures2d_PCTSignatures_setIterationCount_int(cv::xfeatures2d::PCTSignatures* instance, int iterationCount, ResultVoid* ocvrs_return) {
		try {
			instance->setIterationCount(iterationCount);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxClustersCount()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:972
	// ("cv::xfeatures2d::PCTSignatures::getMaxClustersCount", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_PCTSignatures_getMaxClustersCount_const(const cv::xfeatures2d::PCTSignatures* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMaxClustersCount();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxClustersCount(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:977
	// ("cv::xfeatures2d::PCTSignatures::setMaxClustersCount", vec![(pred!(mut, ["maxClustersCount"], ["int"]), _)]),
	void cv_xfeatures2d_PCTSignatures_setMaxClustersCount_int(cv::xfeatures2d::PCTSignatures* instance, int maxClustersCount, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxClustersCount(maxClustersCount);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getClusterMinSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:984
	// ("cv::xfeatures2d::PCTSignatures::getClusterMinSize", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_PCTSignatures_getClusterMinSize_const(const cv::xfeatures2d::PCTSignatures* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getClusterMinSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setClusterMinSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:990
	// ("cv::xfeatures2d::PCTSignatures::setClusterMinSize", vec![(pred!(mut, ["clusterMinSize"], ["int"]), _)]),
	void cv_xfeatures2d_PCTSignatures_setClusterMinSize_int(cv::xfeatures2d::PCTSignatures* instance, int clusterMinSize, ResultVoid* ocvrs_return) {
		try {
			instance->setClusterMinSize(clusterMinSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getJoiningDistance()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:997
	// ("cv::xfeatures2d::PCTSignatures::getJoiningDistance", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_PCTSignatures_getJoiningDistance_const(const cv::xfeatures2d::PCTSignatures* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getJoiningDistance();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setJoiningDistance(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1003
	// ("cv::xfeatures2d::PCTSignatures::setJoiningDistance", vec![(pred!(mut, ["joiningDistance"], ["float"]), _)]),
	void cv_xfeatures2d_PCTSignatures_setJoiningDistance_float(cv::xfeatures2d::PCTSignatures* instance, float joiningDistance, ResultVoid* ocvrs_return) {
		try {
			instance->setJoiningDistance(joiningDistance);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDropThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1008
	// ("cv::xfeatures2d::PCTSignatures::getDropThreshold", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_PCTSignatures_getDropThreshold_const(const cv::xfeatures2d::PCTSignatures* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getDropThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDropThreshold(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1012
	// ("cv::xfeatures2d::PCTSignatures::setDropThreshold", vec![(pred!(mut, ["dropThreshold"], ["float"]), _)]),
	void cv_xfeatures2d_PCTSignatures_setDropThreshold_float(cv::xfeatures2d::PCTSignatures* instance, float dropThreshold, ResultVoid* ocvrs_return) {
		try {
			instance->setDropThreshold(dropThreshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDistanceFunction()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1017
	// ("cv::xfeatures2d::PCTSignatures::getDistanceFunction", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_PCTSignatures_getDistanceFunction_const(const cv::xfeatures2d::PCTSignatures* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getDistanceFunction();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDistanceFunction(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1022
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

	// create(const int, const int, const float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1047
	// ("cv::xfeatures2d::PCTSignaturesSQFD::create", vec![(pred!(mut, ["distanceFunction", "similarityFunction", "similarityParameter"], ["const int", "const int", "const float"]), _)]),
	void cv_xfeatures2d_PCTSignaturesSQFD_create_const_int_const_int_const_float(const int distanceFunction, const int similarityFunction, const float similarityParameter, Result<cv::Ptr<cv::xfeatures2d::PCTSignaturesSQFD>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::PCTSignaturesSQFD> ret = cv::xfeatures2d::PCTSignaturesSQFD::create(distanceFunction, similarityFunction, similarityParameter);
			Ok(new cv::Ptr<cv::xfeatures2d::PCTSignaturesSQFD>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::PCTSignaturesSQFD::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1047
	// ("cv::xfeatures2d::PCTSignaturesSQFD::create", vec![(pred!(mut, [], []), _)]),
	void cv_xfeatures2d_PCTSignaturesSQFD_create(Result<cv::Ptr<cv::xfeatures2d::PCTSignaturesSQFD>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::PCTSignaturesSQFD> ret = cv::xfeatures2d::PCTSignaturesSQFD::create();
			Ok(new cv::Ptr<cv::xfeatures2d::PCTSignaturesSQFD>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// computeQuadraticFormDistance(InputArray, InputArray)(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1057
	// ("cv::xfeatures2d::PCTSignaturesSQFD::computeQuadraticFormDistance", vec![(pred!(const, ["_signature0", "_signature1"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_xfeatures2d_PCTSignaturesSQFD_computeQuadraticFormDistance_const_const__InputArrayR_const__InputArrayR(const cv::xfeatures2d::PCTSignaturesSQFD* instance, const cv::_InputArray* _signature0, const cv::_InputArray* _signature1, Result<float>* ocvrs_return) {
		try {
			float ret = instance->computeQuadraticFormDistance(*_signature0, *_signature1);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// computeQuadraticFormDistances(const Mat &, const std::vector<Mat> &, std::vector<float> &)(TraitClass, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1068
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

	// create(double, int, int, bool, bool)(Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/nonfree.hpp:97
	// ("cv::xfeatures2d::SURF::create", vec![(pred!(mut, ["hessianThreshold", "nOctaves", "nOctaveLayers", "extended", "upright"], ["double", "int", "int", "bool", "bool"]), _)]),
	void cv_xfeatures2d_SURF_create_double_int_int_bool_bool(double hessianThreshold, int nOctaves, int nOctaveLayers, bool extended, bool upright, Result<cv::Ptr<cv::xfeatures2d::SURF>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::SURF> ret = cv::xfeatures2d::SURF::create(hessianThreshold, nOctaves, nOctaveLayers, extended, upright);
			Ok(new cv::Ptr<cv::xfeatures2d::SURF>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::SURF::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/nonfree.hpp:97
	// ("cv::xfeatures2d::SURF::create", vec![(pred!(mut, [], []), _)]),
	void cv_xfeatures2d_SURF_create(Result<cv::Ptr<cv::xfeatures2d::SURF>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::SURF> ret = cv::xfeatures2d::SURF::create();
			Ok(new cv::Ptr<cv::xfeatures2d::SURF>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setHessianThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/nonfree.hpp:101
	// ("cv::xfeatures2d::SURF::setHessianThreshold", vec![(pred!(mut, ["hessianThreshold"], ["double"]), _)]),
	void cv_xfeatures2d_SURF_setHessianThreshold_double(cv::xfeatures2d::SURF* instance, double hessianThreshold, ResultVoid* ocvrs_return) {
		try {
			instance->setHessianThreshold(hessianThreshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getHessianThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/nonfree.hpp:102
	// ("cv::xfeatures2d::SURF::getHessianThreshold", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_SURF_getHessianThreshold_const(const cv::xfeatures2d::SURF* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getHessianThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNOctaves(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/nonfree.hpp:104
	// ("cv::xfeatures2d::SURF::setNOctaves", vec![(pred!(mut, ["nOctaves"], ["int"]), _)]),
	void cv_xfeatures2d_SURF_setNOctaves_int(cv::xfeatures2d::SURF* instance, int nOctaves, ResultVoid* ocvrs_return) {
		try {
			instance->setNOctaves(nOctaves);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNOctaves()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/nonfree.hpp:105
	// ("cv::xfeatures2d::SURF::getNOctaves", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_SURF_getNOctaves_const(const cv::xfeatures2d::SURF* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNOctaves();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNOctaveLayers(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/nonfree.hpp:107
	// ("cv::xfeatures2d::SURF::setNOctaveLayers", vec![(pred!(mut, ["nOctaveLayers"], ["int"]), _)]),
	void cv_xfeatures2d_SURF_setNOctaveLayers_int(cv::xfeatures2d::SURF* instance, int nOctaveLayers, ResultVoid* ocvrs_return) {
		try {
			instance->setNOctaveLayers(nOctaveLayers);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNOctaveLayers()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/nonfree.hpp:108
	// ("cv::xfeatures2d::SURF::getNOctaveLayers", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_SURF_getNOctaveLayers_const(const cv::xfeatures2d::SURF* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNOctaveLayers();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setExtended(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/nonfree.hpp:110
	// ("cv::xfeatures2d::SURF::setExtended", vec![(pred!(mut, ["extended"], ["bool"]), _)]),
	void cv_xfeatures2d_SURF_setExtended_bool(cv::xfeatures2d::SURF* instance, bool extended, ResultVoid* ocvrs_return) {
		try {
			instance->setExtended(extended);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getExtended()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/nonfree.hpp:111
	// ("cv::xfeatures2d::SURF::getExtended", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_SURF_getExtended_const(const cv::xfeatures2d::SURF* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getExtended();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setUpright(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/nonfree.hpp:113
	// ("cv::xfeatures2d::SURF::setUpright", vec![(pred!(mut, ["upright"], ["bool"]), _)]),
	void cv_xfeatures2d_SURF_setUpright_bool(cv::xfeatures2d::SURF* instance, bool upright, ResultVoid* ocvrs_return) {
		try {
			instance->setUpright(upright);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getUpright()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/nonfree.hpp:114
	// ("cv::xfeatures2d::SURF::getUpright", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_SURF_getUpright_const(const cv::xfeatures2d::SURF* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUpright();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d/nonfree.hpp:116
	// ("cv::xfeatures2d::SURF::getDefaultName", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_SURF_getDefaultName_const(const cv::xfeatures2d::SURF* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getDefaultName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
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

	// create(int, int, int, int, int)(Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:131
	// ("cv::xfeatures2d::StarDetector::create", vec![(pred!(mut, ["maxSize", "responseThreshold", "lineThresholdProjected", "lineThresholdBinarized", "suppressNonmaxSize"], ["int", "int", "int", "int", "int"]), _)]),
	void cv_xfeatures2d_StarDetector_create_int_int_int_int_int(int maxSize, int responseThreshold, int lineThresholdProjected, int lineThresholdBinarized, int suppressNonmaxSize, Result<cv::Ptr<cv::xfeatures2d::StarDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::StarDetector> ret = cv::xfeatures2d::StarDetector::create(maxSize, responseThreshold, lineThresholdProjected, lineThresholdBinarized, suppressNonmaxSize);
			Ok(new cv::Ptr<cv::xfeatures2d::StarDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::StarDetector::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:131
	// ("cv::xfeatures2d::StarDetector::create", vec![(pred!(mut, [], []), _)]),
	void cv_xfeatures2d_StarDetector_create(Result<cv::Ptr<cv::xfeatures2d::StarDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::StarDetector> ret = cv::xfeatures2d::StarDetector::create();
			Ok(new cv::Ptr<cv::xfeatures2d::StarDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:136
	// ("cv::xfeatures2d::StarDetector::setMaxSize", vec![(pred!(mut, ["_maxSize"], ["int"]), _)]),
	void cv_xfeatures2d_StarDetector_setMaxSize_int(cv::xfeatures2d::StarDetector* instance, int _maxSize, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxSize(_maxSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:137
	// ("cv::xfeatures2d::StarDetector::getMaxSize", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_StarDetector_getMaxSize_const(const cv::xfeatures2d::StarDetector* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMaxSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setResponseThreshold(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:139
	// ("cv::xfeatures2d::StarDetector::setResponseThreshold", vec![(pred!(mut, ["_responseThreshold"], ["int"]), _)]),
	void cv_xfeatures2d_StarDetector_setResponseThreshold_int(cv::xfeatures2d::StarDetector* instance, int _responseThreshold, ResultVoid* ocvrs_return) {
		try {
			instance->setResponseThreshold(_responseThreshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getResponseThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:140
	// ("cv::xfeatures2d::StarDetector::getResponseThreshold", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_StarDetector_getResponseThreshold_const(const cv::xfeatures2d::StarDetector* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getResponseThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setLineThresholdProjected(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:142
	// ("cv::xfeatures2d::StarDetector::setLineThresholdProjected", vec![(pred!(mut, ["_lineThresholdProjected"], ["int"]), _)]),
	void cv_xfeatures2d_StarDetector_setLineThresholdProjected_int(cv::xfeatures2d::StarDetector* instance, int _lineThresholdProjected, ResultVoid* ocvrs_return) {
		try {
			instance->setLineThresholdProjected(_lineThresholdProjected);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLineThresholdProjected()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:143
	// ("cv::xfeatures2d::StarDetector::getLineThresholdProjected", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_StarDetector_getLineThresholdProjected_const(const cv::xfeatures2d::StarDetector* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getLineThresholdProjected();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setLineThresholdBinarized(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:145
	// ("cv::xfeatures2d::StarDetector::setLineThresholdBinarized", vec![(pred!(mut, ["_lineThresholdBinarized"], ["int"]), _)]),
	void cv_xfeatures2d_StarDetector_setLineThresholdBinarized_int(cv::xfeatures2d::StarDetector* instance, int _lineThresholdBinarized, ResultVoid* ocvrs_return) {
		try {
			instance->setLineThresholdBinarized(_lineThresholdBinarized);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLineThresholdBinarized()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:146
	// ("cv::xfeatures2d::StarDetector::getLineThresholdBinarized", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_StarDetector_getLineThresholdBinarized_const(const cv::xfeatures2d::StarDetector* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getLineThresholdBinarized();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSuppressNonmaxSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:148
	// ("cv::xfeatures2d::StarDetector::setSuppressNonmaxSize", vec![(pred!(mut, ["_suppressNonmaxSize"], ["int"]), _)]),
	void cv_xfeatures2d_StarDetector_setSuppressNonmaxSize_int(cv::xfeatures2d::StarDetector* instance, int _suppressNonmaxSize, ResultVoid* ocvrs_return) {
		try {
			instance->setSuppressNonmaxSize(_suppressNonmaxSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSuppressNonmaxSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:149
	// ("cv::xfeatures2d::StarDetector::getSuppressNonmaxSize", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_StarDetector_getSuppressNonmaxSize_const(const cv::xfeatures2d::StarDetector* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getSuppressNonmaxSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:151
	// ("cv::xfeatures2d::StarDetector::getDefaultName", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_StarDetector_getDefaultName_const(const cv::xfeatures2d::StarDetector* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getDefaultName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
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

	// create(int, float, float, int)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1203
	// ("cv::xfeatures2d::TBMR::create", vec![(pred!(mut, ["min_area", "max_area_relative", "scale_factor", "n_scales"], ["int", "float", "float", "int"]), _)]),
	void cv_xfeatures2d_TBMR_create_int_float_float_int(int min_area, float max_area_relative, float scale_factor, int n_scales, Result<cv::Ptr<cv::xfeatures2d::TBMR>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::TBMR> ret = cv::xfeatures2d::TBMR::create(min_area, max_area_relative, scale_factor, n_scales);
			Ok(new cv::Ptr<cv::xfeatures2d::TBMR>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::TBMR::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1203
	// ("cv::xfeatures2d::TBMR::create", vec![(pred!(mut, [], []), _)]),
	void cv_xfeatures2d_TBMR_create(Result<cv::Ptr<cv::xfeatures2d::TBMR>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::TBMR> ret = cv::xfeatures2d::TBMR::create();
			Ok(new cv::Ptr<cv::xfeatures2d::TBMR>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMinArea(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1208
	// ("cv::xfeatures2d::TBMR::setMinArea", vec![(pred!(mut, ["minArea"], ["int"]), _)]),
	void cv_xfeatures2d_TBMR_setMinArea_int(cv::xfeatures2d::TBMR* instance, int minArea, ResultVoid* ocvrs_return) {
		try {
			instance->setMinArea(minArea);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMinArea()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1209
	// ("cv::xfeatures2d::TBMR::getMinArea", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_TBMR_getMinArea_const(const cv::xfeatures2d::TBMR* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMinArea();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxAreaRelative(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1210
	// ("cv::xfeatures2d::TBMR::setMaxAreaRelative", vec![(pred!(mut, ["maxArea"], ["float"]), _)]),
	void cv_xfeatures2d_TBMR_setMaxAreaRelative_float(cv::xfeatures2d::TBMR* instance, float maxArea, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxAreaRelative(maxArea);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxAreaRelative()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1211
	// ("cv::xfeatures2d::TBMR::getMaxAreaRelative", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_TBMR_getMaxAreaRelative_const(const cv::xfeatures2d::TBMR* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getMaxAreaRelative();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setScaleFactor(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1212
	// ("cv::xfeatures2d::TBMR::setScaleFactor", vec![(pred!(mut, ["scale_factor"], ["float"]), _)]),
	void cv_xfeatures2d_TBMR_setScaleFactor_float(cv::xfeatures2d::TBMR* instance, float scale_factor, ResultVoid* ocvrs_return) {
		try {
			instance->setScaleFactor(scale_factor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getScaleFactor()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1213
	// ("cv::xfeatures2d::TBMR::getScaleFactor", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_TBMR_getScaleFactor_const(const cv::xfeatures2d::TBMR* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getScaleFactor();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNScales(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1214
	// ("cv::xfeatures2d::TBMR::setNScales", vec![(pred!(mut, ["n_scales"], ["int"]), _)]),
	void cv_xfeatures2d_TBMR_setNScales_int(cv::xfeatures2d::TBMR* instance, int n_scales, ResultVoid* ocvrs_return) {
		try {
			instance->setNScales(n_scales);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNScales()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1215
	// ("cv::xfeatures2d::TBMR::getNScales", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_TBMR_getNScales_const(const cv::xfeatures2d::TBMR* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNScales();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:1217
	// ("cv::xfeatures2d::TBMR::getDefaultName", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_TBMR_getDefaultName_const(const cv::xfeatures2d::TBMR* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getDefaultName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::TBMR::to_AffineFeature2D() generated
	// ("cv::xfeatures2d::TBMR::to_AffineFeature2D", vec![(pred!(mut, [], []), _)]),
	cv::xfeatures2d::AffineFeature2D* cv_xfeatures2d_TBMR_to_AffineFeature2D(cv::xfeatures2d::TBMR* instance) {
			return dynamic_cast<cv::xfeatures2d::AffineFeature2D*>(instance);
	}

	// cv::xfeatures2d::TBMR::to_Algorithm() generated
	// ("cv::xfeatures2d::TBMR::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_xfeatures2d_TBMR_to_Algorithm(cv::xfeatures2d::TBMR* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::xfeatures2d::TBMR::to_Feature2D() generated
	// ("cv::xfeatures2d::TBMR::to_Feature2D", vec![(pred!(mut, [], []), _)]),
	cv::Feature2D* cv_xfeatures2d_TBMR_to_Feature2D(cv::xfeatures2d::TBMR* instance) {
			return dynamic_cast<cv::Feature2D*>(instance);
	}

	// cv::xfeatures2d::TBMR::delete() generated
	// ("cv::xfeatures2d::TBMR::delete", vec![(pred!(mut, [], []), _)]),
	void cv_xfeatures2d_TBMR_delete(cv::xfeatures2d::TBMR* instance) {
			delete instance;
	}

	// create(float, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:336
	// ("cv::xfeatures2d::TEBLID::create", vec![(pred!(mut, ["scale_factor", "n_bits"], ["float", "int"]), _)]),
	void cv_xfeatures2d_TEBLID_create_float_int(float scale_factor, int n_bits, Result<cv::Ptr<cv::xfeatures2d::TEBLID>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::TEBLID> ret = cv::xfeatures2d::TEBLID::create(scale_factor, n_bits);
			Ok(new cv::Ptr<cv::xfeatures2d::TEBLID>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::TEBLID::create(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:336
	// ("cv::xfeatures2d::TEBLID::create", vec![(pred!(mut, ["scale_factor"], ["float"]), _)]),
	void cv_xfeatures2d_TEBLID_create_float(float scale_factor, Result<cv::Ptr<cv::xfeatures2d::TEBLID>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::TEBLID> ret = cv::xfeatures2d::TEBLID::create(scale_factor);
			Ok(new cv::Ptr<cv::xfeatures2d::TEBLID>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:338
	// ("cv::xfeatures2d::TEBLID::getDefaultName", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_TEBLID_getDefaultName_const(const cv::xfeatures2d::TEBLID* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getDefaultName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::TEBLID::defaultNew() generated
	// ("cv::xfeatures2d::TEBLID::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::xfeatures2d::TEBLID* cv_xfeatures2d_TEBLID_defaultNew_const() {
			cv::xfeatures2d::TEBLID* ret = new cv::xfeatures2d::TEBLID();
			return ret;
	}

	// cv::xfeatures2d::TEBLID::to_Algorithm() generated
	// ("cv::xfeatures2d::TEBLID::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_xfeatures2d_TEBLID_to_Algorithm(cv::xfeatures2d::TEBLID* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::xfeatures2d::TEBLID::to_Feature2D() generated
	// ("cv::xfeatures2d::TEBLID::to_Feature2D", vec![(pred!(mut, [], []), _)]),
	cv::Feature2D* cv_xfeatures2d_TEBLID_to_Feature2D(cv::xfeatures2d::TEBLID* instance) {
			return dynamic_cast<cv::Feature2D*>(instance);
	}

	// cv::xfeatures2d::TEBLID::delete() generated
	// ("cv::xfeatures2d::TEBLID::delete", vec![(pred!(mut, [], []), _)]),
	void cv_xfeatures2d_TEBLID_delete(cv::xfeatures2d::TEBLID* instance) {
			delete instance;
	}

	// create(int, float, bool, bool, float, bool)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:531
	// ("cv::xfeatures2d::VGG::create", vec![(pred!(mut, ["desc", "isigma", "img_normalize", "use_scale_orientation", "scale_factor", "dsc_normalize"], ["int", "float", "bool", "bool", "float", "bool"]), _)]),
	void cv_xfeatures2d_VGG_create_int_float_bool_bool_float_bool(int desc, float isigma, bool img_normalize, bool use_scale_orientation, float scale_factor, bool dsc_normalize, Result<cv::Ptr<cv::xfeatures2d::VGG>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::VGG> ret = cv::xfeatures2d::VGG::create(desc, isigma, img_normalize, use_scale_orientation, scale_factor, dsc_normalize);
			Ok(new cv::Ptr<cv::xfeatures2d::VGG>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xfeatures2d::VGG::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:531
	// ("cv::xfeatures2d::VGG::create", vec![(pred!(mut, [], []), _)]),
	void cv_xfeatures2d_VGG_create(Result<cv::Ptr<cv::xfeatures2d::VGG>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xfeatures2d::VGG> ret = cv::xfeatures2d::VGG::create();
			Ok(new cv::Ptr<cv::xfeatures2d::VGG>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDefaultName()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:535
	// ("cv::xfeatures2d::VGG::getDefaultName", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_VGG_getDefaultName_const(const cv::xfeatures2d::VGG* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getDefaultName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSigma(const float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:537
	// ("cv::xfeatures2d::VGG::setSigma", vec![(pred!(mut, ["isigma"], ["const float"]), _)]),
	void cv_xfeatures2d_VGG_setSigma_const_float(cv::xfeatures2d::VGG* instance, const float isigma, ResultVoid* ocvrs_return) {
		try {
			instance->setSigma(isigma);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSigma()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:538
	// ("cv::xfeatures2d::VGG::getSigma", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_VGG_getSigma_const(const cv::xfeatures2d::VGG* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getSigma();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setUseNormalizeImage(const bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:540
	// ("cv::xfeatures2d::VGG::setUseNormalizeImage", vec![(pred!(mut, ["img_normalize"], ["const bool"]), _)]),
	void cv_xfeatures2d_VGG_setUseNormalizeImage_const_bool(cv::xfeatures2d::VGG* instance, const bool img_normalize, ResultVoid* ocvrs_return) {
		try {
			instance->setUseNormalizeImage(img_normalize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getUseNormalizeImage()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:541
	// ("cv::xfeatures2d::VGG::getUseNormalizeImage", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_VGG_getUseNormalizeImage_const(const cv::xfeatures2d::VGG* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUseNormalizeImage();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setUseScaleOrientation(const bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:543
	// ("cv::xfeatures2d::VGG::setUseScaleOrientation", vec![(pred!(mut, ["use_scale_orientation"], ["const bool"]), _)]),
	void cv_xfeatures2d_VGG_setUseScaleOrientation_const_bool(cv::xfeatures2d::VGG* instance, const bool use_scale_orientation, ResultVoid* ocvrs_return) {
		try {
			instance->setUseScaleOrientation(use_scale_orientation);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getUseScaleOrientation()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:544
	// ("cv::xfeatures2d::VGG::getUseScaleOrientation", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_VGG_getUseScaleOrientation_const(const cv::xfeatures2d::VGG* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUseScaleOrientation();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setScaleFactor(const float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:546
	// ("cv::xfeatures2d::VGG::setScaleFactor", vec![(pred!(mut, ["scale_factor"], ["const float"]), _)]),
	void cv_xfeatures2d_VGG_setScaleFactor_const_float(cv::xfeatures2d::VGG* instance, const float scale_factor, ResultVoid* ocvrs_return) {
		try {
			instance->setScaleFactor(scale_factor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getScaleFactor()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:547
	// ("cv::xfeatures2d::VGG::getScaleFactor", vec![(pred!(const, [], []), _)]),
	void cv_xfeatures2d_VGG_getScaleFactor_const(const cv::xfeatures2d::VGG* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getScaleFactor();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setUseNormalizeDescriptor(const bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:549
	// ("cv::xfeatures2d::VGG::setUseNormalizeDescriptor", vec![(pred!(mut, ["dsc_normalize"], ["const bool"]), _)]),
	void cv_xfeatures2d_VGG_setUseNormalizeDescriptor_const_bool(cv::xfeatures2d::VGG* instance, const bool dsc_normalize, ResultVoid* ocvrs_return) {
		try {
			instance->setUseNormalizeDescriptor(dsc_normalize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getUseNormalizeDescriptor()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xfeatures2d.hpp:550
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
