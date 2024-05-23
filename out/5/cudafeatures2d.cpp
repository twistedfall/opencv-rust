#include "ocvrs_common.hpp"
#include <opencv2/cudafeatures2d.hpp>
#include "cudafeatures2d_types.hpp"

extern "C" {
	// createBFMatcher(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:92
	// ("cv::cuda::DescriptorMatcher::createBFMatcher", vec![(pred!(mut, ["normType"], ["int"]), _)]),
	void cv_cuda_DescriptorMatcher_createBFMatcher_int(int normType, Result<cv::Ptr<cv::cuda::DescriptorMatcher>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::DescriptorMatcher> ret = cv::cuda::DescriptorMatcher::createBFMatcher(normType);
			Ok(new cv::Ptr<cv::cuda::DescriptorMatcher>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::DescriptorMatcher::createBFMatcher() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:92
	// ("cv::cuda::DescriptorMatcher::createBFMatcher", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_DescriptorMatcher_createBFMatcher(Result<cv::Ptr<cv::cuda::DescriptorMatcher>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::DescriptorMatcher> ret = cv::cuda::DescriptorMatcher::createBFMatcher();
			Ok(new cv::Ptr<cv::cuda::DescriptorMatcher>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isMaskSupported()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:100
	// ("cv::cuda::DescriptorMatcher::isMaskSupported", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DescriptorMatcher_isMaskSupported_const(const cv::cuda::DescriptorMatcher* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isMaskSupported();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// add(const std::vector<GpuMat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:113
	// ("cv::cuda::DescriptorMatcher::add", vec![(pred!(mut, ["descriptors"], ["const std::vector<cv::cuda::GpuMat>*"]), _)]),
	void cv_cuda_DescriptorMatcher_add_const_vectorLGpuMatGR(cv::cuda::DescriptorMatcher* instance, const std::vector<cv::cuda::GpuMat>* descriptors, ResultVoid* ocvrs_return) {
		try {
			instance->add(*descriptors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTrainDescriptors()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:117
	// ("cv::cuda::DescriptorMatcher::getTrainDescriptors", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DescriptorMatcher_getTrainDescriptors_const(const cv::cuda::DescriptorMatcher* instance, Result<std::vector<cv::cuda::GpuMat>*>* ocvrs_return) {
		try {
			const std::vector<cv::cuda::GpuMat> ret = instance->getTrainDescriptors();
			Ok(new const std::vector<cv::cuda::GpuMat>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// clear()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:121
	// ("cv::cuda::DescriptorMatcher::clear", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_DescriptorMatcher_clear(cv::cuda::DescriptorMatcher* instance, ResultVoid* ocvrs_return) {
		try {
			instance->clear();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// empty()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:125
	// ("cv::cuda::DescriptorMatcher::empty", vec![(pred!(const, [], []), _)]),
	void cv_cuda_DescriptorMatcher_empty_const(const cv::cuda::DescriptorMatcher* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// train()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:132
	// ("cv::cuda::DescriptorMatcher::train", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_DescriptorMatcher_train(cv::cuda::DescriptorMatcher* instance, ResultVoid* ocvrs_return) {
		try {
			instance->train();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// match(InputArray, InputArray, std::vector<DMatch> &, InputArray)(InputArray, InputArray, CppPassByVoidPtr, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:154
	// ("cv::cuda::DescriptorMatcher::match", vec![(pred!(mut, ["queryDescriptors", "trainDescriptors", "matches", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<cv::DMatch>*", "const cv::_InputArray*"]), _)]),
	void cv_cuda_DescriptorMatcher_match_const__InputArrayR_const__InputArrayR_vectorLDMatchGR_const__InputArrayR(cv::cuda::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, const cv::_InputArray* trainDescriptors, std::vector<cv::DMatch>* matches, const cv::_InputArray* mask, ResultVoid* ocvrs_return) {
		try {
			instance->match(*queryDescriptors, *trainDescriptors, *matches, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::DescriptorMatcher::match(InputArray, InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:154
	// ("cv::cuda::DescriptorMatcher::match", vec![(pred!(mut, ["queryDescriptors", "trainDescriptors", "matches"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<cv::DMatch>*"]), _)]),
	void cv_cuda_DescriptorMatcher_match_const__InputArrayR_const__InputArrayR_vectorLDMatchGR(cv::cuda::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, const cv::_InputArray* trainDescriptors, std::vector<cv::DMatch>* matches, ResultVoid* ocvrs_return) {
		try {
			instance->match(*queryDescriptors, *trainDescriptors, *matches);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// match(InputArray, std::vector<DMatch> &, const std::vector<GpuMat> &)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:160
	// ("cv::cuda::DescriptorMatcher::match", vec![(pred!(mut, ["queryDescriptors", "matches", "masks"], ["const cv::_InputArray*", "std::vector<cv::DMatch>*", "const std::vector<cv::cuda::GpuMat>*"]), _)]),
	void cv_cuda_DescriptorMatcher_match_const__InputArrayR_vectorLDMatchGR_const_vectorLGpuMatGR(cv::cuda::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, std::vector<cv::DMatch>* matches, const std::vector<cv::cuda::GpuMat>* masks, ResultVoid* ocvrs_return) {
		try {
			instance->match(*queryDescriptors, *matches, *masks);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::DescriptorMatcher::match(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:160
	// ("cv::cuda::DescriptorMatcher::match", vec![(pred!(mut, ["queryDescriptors", "matches"], ["const cv::_InputArray*", "std::vector<cv::DMatch>*"]), _)]),
	void cv_cuda_DescriptorMatcher_match_const__InputArrayR_vectorLDMatchGR(cv::cuda::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, std::vector<cv::DMatch>* matches, ResultVoid* ocvrs_return) {
		try {
			instance->match(*queryDescriptors, *matches);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// matchAsync(InputArray, InputArray, OutputArray, InputArray, Stream &)(InputArray, InputArray, OutputArray, InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:181
	// ("cv::cuda::DescriptorMatcher::matchAsync", vec![(pred!(mut, ["queryDescriptors", "trainDescriptors", "matches", "mask", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_DescriptorMatcher_matchAsync_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR_StreamR(cv::cuda::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, const cv::_InputArray* trainDescriptors, const cv::_OutputArray* matches, const cv::_InputArray* mask, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			instance->matchAsync(*queryDescriptors, *trainDescriptors, *matches, *mask, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::DescriptorMatcher::matchAsync(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:181
	// ("cv::cuda::DescriptorMatcher::matchAsync", vec![(pred!(mut, ["queryDescriptors", "trainDescriptors", "matches"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_DescriptorMatcher_matchAsync_const__InputArrayR_const__InputArrayR_const__OutputArrayR(cv::cuda::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, const cv::_InputArray* trainDescriptors, const cv::_OutputArray* matches, ResultVoid* ocvrs_return) {
		try {
			instance->matchAsync(*queryDescriptors, *trainDescriptors, *matches);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// matchAsync(InputArray, OutputArray, const std::vector<GpuMat> &, Stream &)(InputArray, OutputArray, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:188
	// ("cv::cuda::DescriptorMatcher::matchAsync", vec![(pred!(mut, ["queryDescriptors", "matches", "masks", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const std::vector<cv::cuda::GpuMat>*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_DescriptorMatcher_matchAsync_const__InputArrayR_const__OutputArrayR_const_vectorLGpuMatGR_StreamR(cv::cuda::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, const cv::_OutputArray* matches, const std::vector<cv::cuda::GpuMat>* masks, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			instance->matchAsync(*queryDescriptors, *matches, *masks, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::DescriptorMatcher::matchAsync(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:188
	// ("cv::cuda::DescriptorMatcher::matchAsync", vec![(pred!(mut, ["queryDescriptors", "matches"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_DescriptorMatcher_matchAsync_const__InputArrayR_const__OutputArrayR(cv::cuda::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, const cv::_OutputArray* matches, ResultVoid* ocvrs_return) {
		try {
			instance->matchAsync(*queryDescriptors, *matches);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// matchConvert(InputArray, std::vector<DMatch> &)(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:201
	// ("cv::cuda::DescriptorMatcher::matchConvert", vec![(pred!(mut, ["gpu_matches", "matches"], ["const cv::_InputArray*", "std::vector<cv::DMatch>*"]), _)]),
	void cv_cuda_DescriptorMatcher_matchConvert_const__InputArrayR_vectorLDMatchGR(cv::cuda::DescriptorMatcher* instance, const cv::_InputArray* gpu_matches, std::vector<cv::DMatch>* matches, ResultVoid* ocvrs_return) {
		try {
			instance->matchConvert(*gpu_matches, *matches);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// knnMatch(InputArray, InputArray, std::vector<std::vector<DMatch>> &, int, InputArray, bool)(InputArray, InputArray, CppPassByVoidPtr, Primitive, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:226
	// ("cv::cuda::DescriptorMatcher::knnMatch", vec![(pred!(mut, ["queryDescriptors", "trainDescriptors", "matches", "k", "mask", "compactResult"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<std::vector<cv::DMatch>>*", "int", "const cv::_InputArray*", "bool"]), _)]),
	void cv_cuda_DescriptorMatcher_knnMatch_const__InputArrayR_const__InputArrayR_vectorLvectorLDMatchGGR_int_const__InputArrayR_bool(cv::cuda::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, const cv::_InputArray* trainDescriptors, std::vector<std::vector<cv::DMatch>>* matches, int k, const cv::_InputArray* mask, bool compactResult, ResultVoid* ocvrs_return) {
		try {
			instance->knnMatch(*queryDescriptors, *trainDescriptors, *matches, k, *mask, compactResult);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::DescriptorMatcher::knnMatch(InputArray, InputArray, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:226
	// ("cv::cuda::DescriptorMatcher::knnMatch", vec![(pred!(mut, ["queryDescriptors", "trainDescriptors", "matches", "k"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<std::vector<cv::DMatch>>*", "int"]), _)]),
	void cv_cuda_DescriptorMatcher_knnMatch_const__InputArrayR_const__InputArrayR_vectorLvectorLDMatchGGR_int(cv::cuda::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, const cv::_InputArray* trainDescriptors, std::vector<std::vector<cv::DMatch>>* matches, int k, ResultVoid* ocvrs_return) {
		try {
			instance->knnMatch(*queryDescriptors, *trainDescriptors, *matches, k);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// knnMatch(InputArray, std::vector<std::vector<DMatch>> &, int, const std::vector<GpuMat> &, bool)(InputArray, CppPassByVoidPtr, Primitive, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:234
	// ("cv::cuda::DescriptorMatcher::knnMatch", vec![(pred!(mut, ["queryDescriptors", "matches", "k", "masks", "compactResult"], ["const cv::_InputArray*", "std::vector<std::vector<cv::DMatch>>*", "int", "const std::vector<cv::cuda::GpuMat>*", "bool"]), _)]),
	void cv_cuda_DescriptorMatcher_knnMatch_const__InputArrayR_vectorLvectorLDMatchGGR_int_const_vectorLGpuMatGR_bool(cv::cuda::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, std::vector<std::vector<cv::DMatch>>* matches, int k, const std::vector<cv::cuda::GpuMat>* masks, bool compactResult, ResultVoid* ocvrs_return) {
		try {
			instance->knnMatch(*queryDescriptors, *matches, k, *masks, compactResult);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::DescriptorMatcher::knnMatch(InputArray, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:234
	// ("cv::cuda::DescriptorMatcher::knnMatch", vec![(pred!(mut, ["queryDescriptors", "matches", "k"], ["const cv::_InputArray*", "std::vector<std::vector<cv::DMatch>>*", "int"]), _)]),
	void cv_cuda_DescriptorMatcher_knnMatch_const__InputArrayR_vectorLvectorLDMatchGGR_int(cv::cuda::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, std::vector<std::vector<cv::DMatch>>* matches, int k, ResultVoid* ocvrs_return) {
		try {
			instance->knnMatch(*queryDescriptors, *matches, k);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// knnMatchAsync(InputArray, InputArray, OutputArray, int, InputArray, Stream &)(InputArray, InputArray, OutputArray, Primitive, InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:257
	// ("cv::cuda::DescriptorMatcher::knnMatchAsync", vec![(pred!(mut, ["queryDescriptors", "trainDescriptors", "matches", "k", "mask", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "const cv::_InputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_DescriptorMatcher_knnMatchAsync_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_const__InputArrayR_StreamR(cv::cuda::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, const cv::_InputArray* trainDescriptors, const cv::_OutputArray* matches, int k, const cv::_InputArray* mask, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			instance->knnMatchAsync(*queryDescriptors, *trainDescriptors, *matches, k, *mask, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::DescriptorMatcher::knnMatchAsync(InputArray, InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:257
	// ("cv::cuda::DescriptorMatcher::knnMatchAsync", vec![(pred!(mut, ["queryDescriptors", "trainDescriptors", "matches", "k"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_cuda_DescriptorMatcher_knnMatchAsync_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int(cv::cuda::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, const cv::_InputArray* trainDescriptors, const cv::_OutputArray* matches, int k, ResultVoid* ocvrs_return) {
		try {
			instance->knnMatchAsync(*queryDescriptors, *trainDescriptors, *matches, k);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// knnMatchAsync(InputArray, OutputArray, int, const std::vector<GpuMat> &, Stream &)(InputArray, OutputArray, Primitive, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:265
	// ("cv::cuda::DescriptorMatcher::knnMatchAsync", vec![(pred!(mut, ["queryDescriptors", "matches", "k", "masks", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "const std::vector<cv::cuda::GpuMat>*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_DescriptorMatcher_knnMatchAsync_const__InputArrayR_const__OutputArrayR_int_const_vectorLGpuMatGR_StreamR(cv::cuda::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, const cv::_OutputArray* matches, int k, const std::vector<cv::cuda::GpuMat>* masks, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			instance->knnMatchAsync(*queryDescriptors, *matches, k, *masks, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::DescriptorMatcher::knnMatchAsync(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:265
	// ("cv::cuda::DescriptorMatcher::knnMatchAsync", vec![(pred!(mut, ["queryDescriptors", "matches", "k"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_cuda_DescriptorMatcher_knnMatchAsync_const__InputArrayR_const__OutputArrayR_int(cv::cuda::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, const cv::_OutputArray* matches, int k, ResultVoid* ocvrs_return) {
		try {
			instance->knnMatchAsync(*queryDescriptors, *matches, k);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// knnMatchConvert(InputArray, std::vector<std::vector<DMatch>> &, bool)(InputArray, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:282
	// ("cv::cuda::DescriptorMatcher::knnMatchConvert", vec![(pred!(mut, ["gpu_matches", "matches", "compactResult"], ["const cv::_InputArray*", "std::vector<std::vector<cv::DMatch>>*", "bool"]), _)]),
	void cv_cuda_DescriptorMatcher_knnMatchConvert_const__InputArrayR_vectorLvectorLDMatchGGR_bool(cv::cuda::DescriptorMatcher* instance, const cv::_InputArray* gpu_matches, std::vector<std::vector<cv::DMatch>>* matches, bool compactResult, ResultVoid* ocvrs_return) {
		try {
			instance->knnMatchConvert(*gpu_matches, *matches, compactResult);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::DescriptorMatcher::knnMatchConvert(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:282
	// ("cv::cuda::DescriptorMatcher::knnMatchConvert", vec![(pred!(mut, ["gpu_matches", "matches"], ["const cv::_InputArray*", "std::vector<std::vector<cv::DMatch>>*"]), _)]),
	void cv_cuda_DescriptorMatcher_knnMatchConvert_const__InputArrayR_vectorLvectorLDMatchGGR(cv::cuda::DescriptorMatcher* instance, const cv::_InputArray* gpu_matches, std::vector<std::vector<cv::DMatch>>* matches, ResultVoid* ocvrs_return) {
		try {
			instance->knnMatchConvert(*gpu_matches, *matches);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// radiusMatch(InputArray, InputArray, std::vector<std::vector<DMatch>> &, float, InputArray, bool)(InputArray, InputArray, CppPassByVoidPtr, Primitive, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:309
	// ("cv::cuda::DescriptorMatcher::radiusMatch", vec![(pred!(mut, ["queryDescriptors", "trainDescriptors", "matches", "maxDistance", "mask", "compactResult"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<std::vector<cv::DMatch>>*", "float", "const cv::_InputArray*", "bool"]), _)]),
	void cv_cuda_DescriptorMatcher_radiusMatch_const__InputArrayR_const__InputArrayR_vectorLvectorLDMatchGGR_float_const__InputArrayR_bool(cv::cuda::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, const cv::_InputArray* trainDescriptors, std::vector<std::vector<cv::DMatch>>* matches, float maxDistance, const cv::_InputArray* mask, bool compactResult, ResultVoid* ocvrs_return) {
		try {
			instance->radiusMatch(*queryDescriptors, *trainDescriptors, *matches, maxDistance, *mask, compactResult);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::DescriptorMatcher::radiusMatch(InputArray, InputArray, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:309
	// ("cv::cuda::DescriptorMatcher::radiusMatch", vec![(pred!(mut, ["queryDescriptors", "trainDescriptors", "matches", "maxDistance"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<std::vector<cv::DMatch>>*", "float"]), _)]),
	void cv_cuda_DescriptorMatcher_radiusMatch_const__InputArrayR_const__InputArrayR_vectorLvectorLDMatchGGR_float(cv::cuda::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, const cv::_InputArray* trainDescriptors, std::vector<std::vector<cv::DMatch>>* matches, float maxDistance, ResultVoid* ocvrs_return) {
		try {
			instance->radiusMatch(*queryDescriptors, *trainDescriptors, *matches, maxDistance);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// radiusMatch(InputArray, std::vector<std::vector<DMatch>> &, float, const std::vector<GpuMat> &, bool)(InputArray, CppPassByVoidPtr, Primitive, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:317
	// ("cv::cuda::DescriptorMatcher::radiusMatch", vec![(pred!(mut, ["queryDescriptors", "matches", "maxDistance", "masks", "compactResult"], ["const cv::_InputArray*", "std::vector<std::vector<cv::DMatch>>*", "float", "const std::vector<cv::cuda::GpuMat>*", "bool"]), _)]),
	void cv_cuda_DescriptorMatcher_radiusMatch_const__InputArrayR_vectorLvectorLDMatchGGR_float_const_vectorLGpuMatGR_bool(cv::cuda::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, std::vector<std::vector<cv::DMatch>>* matches, float maxDistance, const std::vector<cv::cuda::GpuMat>* masks, bool compactResult, ResultVoid* ocvrs_return) {
		try {
			instance->radiusMatch(*queryDescriptors, *matches, maxDistance, *masks, compactResult);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::DescriptorMatcher::radiusMatch(InputArray, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:317
	// ("cv::cuda::DescriptorMatcher::radiusMatch", vec![(pred!(mut, ["queryDescriptors", "matches", "maxDistance"], ["const cv::_InputArray*", "std::vector<std::vector<cv::DMatch>>*", "float"]), _)]),
	void cv_cuda_DescriptorMatcher_radiusMatch_const__InputArrayR_vectorLvectorLDMatchGGR_float(cv::cuda::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, std::vector<std::vector<cv::DMatch>>* matches, float maxDistance, ResultVoid* ocvrs_return) {
		try {
			instance->radiusMatch(*queryDescriptors, *matches, maxDistance);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// radiusMatchAsync(InputArray, InputArray, OutputArray, float, InputArray, Stream &)(InputArray, InputArray, OutputArray, Primitive, InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:341
	// ("cv::cuda::DescriptorMatcher::radiusMatchAsync", vec![(pred!(mut, ["queryDescriptors", "trainDescriptors", "matches", "maxDistance", "mask", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "float", "const cv::_InputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_DescriptorMatcher_radiusMatchAsync_const__InputArrayR_const__InputArrayR_const__OutputArrayR_float_const__InputArrayR_StreamR(cv::cuda::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, const cv::_InputArray* trainDescriptors, const cv::_OutputArray* matches, float maxDistance, const cv::_InputArray* mask, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			instance->radiusMatchAsync(*queryDescriptors, *trainDescriptors, *matches, maxDistance, *mask, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::DescriptorMatcher::radiusMatchAsync(InputArray, InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:341
	// ("cv::cuda::DescriptorMatcher::radiusMatchAsync", vec![(pred!(mut, ["queryDescriptors", "trainDescriptors", "matches", "maxDistance"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "float"]), _)]),
	void cv_cuda_DescriptorMatcher_radiusMatchAsync_const__InputArrayR_const__InputArrayR_const__OutputArrayR_float(cv::cuda::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, const cv::_InputArray* trainDescriptors, const cv::_OutputArray* matches, float maxDistance, ResultVoid* ocvrs_return) {
		try {
			instance->radiusMatchAsync(*queryDescriptors, *trainDescriptors, *matches, maxDistance);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// radiusMatchAsync(InputArray, OutputArray, float, const std::vector<GpuMat> &, Stream &)(InputArray, OutputArray, Primitive, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:349
	// ("cv::cuda::DescriptorMatcher::radiusMatchAsync", vec![(pred!(mut, ["queryDescriptors", "matches", "maxDistance", "masks", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "float", "const std::vector<cv::cuda::GpuMat>*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_DescriptorMatcher_radiusMatchAsync_const__InputArrayR_const__OutputArrayR_float_const_vectorLGpuMatGR_StreamR(cv::cuda::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, const cv::_OutputArray* matches, float maxDistance, const std::vector<cv::cuda::GpuMat>* masks, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			instance->radiusMatchAsync(*queryDescriptors, *matches, maxDistance, *masks, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::DescriptorMatcher::radiusMatchAsync(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:349
	// ("cv::cuda::DescriptorMatcher::radiusMatchAsync", vec![(pred!(mut, ["queryDescriptors", "matches", "maxDistance"], ["const cv::_InputArray*", "const cv::_OutputArray*", "float"]), _)]),
	void cv_cuda_DescriptorMatcher_radiusMatchAsync_const__InputArrayR_const__OutputArrayR_float(cv::cuda::DescriptorMatcher* instance, const cv::_InputArray* queryDescriptors, const cv::_OutputArray* matches, float maxDistance, ResultVoid* ocvrs_return) {
		try {
			instance->radiusMatchAsync(*queryDescriptors, *matches, maxDistance);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// radiusMatchConvert(InputArray, std::vector<std::vector<DMatch>> &, bool)(InputArray, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:366
	// ("cv::cuda::DescriptorMatcher::radiusMatchConvert", vec![(pred!(mut, ["gpu_matches", "matches", "compactResult"], ["const cv::_InputArray*", "std::vector<std::vector<cv::DMatch>>*", "bool"]), _)]),
	void cv_cuda_DescriptorMatcher_radiusMatchConvert_const__InputArrayR_vectorLvectorLDMatchGGR_bool(cv::cuda::DescriptorMatcher* instance, const cv::_InputArray* gpu_matches, std::vector<std::vector<cv::DMatch>>* matches, bool compactResult, ResultVoid* ocvrs_return) {
		try {
			instance->radiusMatchConvert(*gpu_matches, *matches, compactResult);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::DescriptorMatcher::radiusMatchConvert(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:366
	// ("cv::cuda::DescriptorMatcher::radiusMatchConvert", vec![(pred!(mut, ["gpu_matches", "matches"], ["const cv::_InputArray*", "std::vector<std::vector<cv::DMatch>>*"]), _)]),
	void cv_cuda_DescriptorMatcher_radiusMatchConvert_const__InputArrayR_vectorLvectorLDMatchGGR(cv::cuda::DescriptorMatcher* instance, const cv::_InputArray* gpu_matches, std::vector<std::vector<cv::DMatch>>* matches, ResultVoid* ocvrs_return) {
		try {
			instance->radiusMatchConvert(*gpu_matches, *matches);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::DescriptorMatcher::to_Algorithm() generated
	// ("cv::cuda::DescriptorMatcher::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_cuda_DescriptorMatcher_to_Algorithm(cv::cuda::DescriptorMatcher* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::cuda::DescriptorMatcher::delete() generated
	// ("cv::cuda::DescriptorMatcher::delete", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_DescriptorMatcher_delete(cv::cuda::DescriptorMatcher* instance) {
			delete instance;
	}

	// create(int, bool, int, int)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:434
	// ("cv::cuda::FastFeatureDetector::create", vec![(pred!(mut, ["threshold", "nonmaxSuppression", "type", "max_npoints"], ["int", "bool", "int", "int"]), _)]),
	void cv_cuda_FastFeatureDetector_create_int_bool_int_int(int threshold, bool nonmaxSuppression, int type, int max_npoints, Result<cv::Ptr<cv::cuda::FastFeatureDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::FastFeatureDetector> ret = cv::cuda::FastFeatureDetector::create(threshold, nonmaxSuppression, type, max_npoints);
			Ok(new cv::Ptr<cv::cuda::FastFeatureDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::FastFeatureDetector::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:434
	// ("cv::cuda::FastFeatureDetector::create", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_FastFeatureDetector_create(Result<cv::Ptr<cv::cuda::FastFeatureDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::FastFeatureDetector> ret = cv::cuda::FastFeatureDetector::create();
			Ok(new cv::Ptr<cv::cuda::FastFeatureDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setThreshold(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:438
	// ("cv::cuda::FastFeatureDetector::setThreshold", vec![(pred!(mut, ["threshold"], ["int"]), _)]),
	void cv_cuda_FastFeatureDetector_setThreshold_int(cv::cuda::FastFeatureDetector* instance, int threshold, ResultVoid* ocvrs_return) {
		try {
			instance->setThreshold(threshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxNumPoints(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:440
	// ("cv::cuda::FastFeatureDetector::setMaxNumPoints", vec![(pred!(mut, ["max_npoints"], ["int"]), _)]),
	void cv_cuda_FastFeatureDetector_setMaxNumPoints_int(cv::cuda::FastFeatureDetector* instance, int max_npoints, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxNumPoints(max_npoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxNumPoints()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:441
	// ("cv::cuda::FastFeatureDetector::getMaxNumPoints", vec![(pred!(const, [], []), _)]),
	void cv_cuda_FastFeatureDetector_getMaxNumPoints_const(const cv::cuda::FastFeatureDetector* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMaxNumPoints();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::FastFeatureDetector::to_Algorithm() generated
	// ("cv::cuda::FastFeatureDetector::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_cuda_FastFeatureDetector_to_Algorithm(cv::cuda::FastFeatureDetector* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::cuda::FastFeatureDetector::to_Feature2D() generated
	// ("cv::cuda::FastFeatureDetector::to_Feature2D", vec![(pred!(mut, [], []), _)]),
	cv::Feature2D* cv_cuda_FastFeatureDetector_to_Feature2D(cv::cuda::FastFeatureDetector* instance) {
			return dynamic_cast<cv::Feature2D*>(instance);
	}

	// cv::cuda::FastFeatureDetector::to_CUDA_Feature2DAsync() generated
	// ("cv::cuda::FastFeatureDetector::to_CUDA_Feature2DAsync", vec![(pred!(mut, [], []), _)]),
	cv::cuda::Feature2DAsync* cv_cuda_FastFeatureDetector_to_CUDA_Feature2DAsync(cv::cuda::FastFeatureDetector* instance) {
			return dynamic_cast<cv::cuda::Feature2DAsync*>(instance);
	}

	// cv::cuda::FastFeatureDetector::delete() generated
	// ("cv::cuda::FastFeatureDetector::delete", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_FastFeatureDetector_delete(cv::cuda::FastFeatureDetector* instance) {
			delete instance;
	}

	// detectAsync(InputArray, OutputArray, InputArray, Stream &)(InputArray, OutputArray, InputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:390
	// ("cv::cuda::Feature2DAsync::detectAsync", vec![(pred!(mut, ["image", "keypoints", "mask", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_Feature2DAsync_detectAsync_const__InputArrayR_const__OutputArrayR_const__InputArrayR_StreamR(cv::cuda::Feature2DAsync* instance, const cv::_InputArray* image, const cv::_OutputArray* keypoints, const cv::_InputArray* mask, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			instance->detectAsync(*image, *keypoints, *mask, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::Feature2DAsync::detectAsync(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:390
	// ("cv::cuda::Feature2DAsync::detectAsync", vec![(pred!(mut, ["image", "keypoints"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_Feature2DAsync_detectAsync_const__InputArrayR_const__OutputArrayR(cv::cuda::Feature2DAsync* instance, const cv::_InputArray* image, const cv::_OutputArray* keypoints, ResultVoid* ocvrs_return) {
		try {
			instance->detectAsync(*image, *keypoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// computeAsync(InputArray, OutputArray, OutputArray, Stream &)(InputArray, OutputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:402
	// ("cv::cuda::Feature2DAsync::computeAsync", vec![(pred!(mut, ["image", "keypoints", "descriptors", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_Feature2DAsync_computeAsync_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_StreamR(cv::cuda::Feature2DAsync* instance, const cv::_InputArray* image, const cv::_OutputArray* keypoints, const cv::_OutputArray* descriptors, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			instance->computeAsync(*image, *keypoints, *descriptors, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::Feature2DAsync::computeAsync(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:402
	// ("cv::cuda::Feature2DAsync::computeAsync", vec![(pred!(mut, ["image", "keypoints", "descriptors"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_Feature2DAsync_computeAsync_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::cuda::Feature2DAsync* instance, const cv::_InputArray* image, const cv::_OutputArray* keypoints, const cv::_OutputArray* descriptors, ResultVoid* ocvrs_return) {
		try {
			instance->computeAsync(*image, *keypoints, *descriptors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectAndComputeAsync(InputArray, InputArray, OutputArray, OutputArray, bool, Stream &)(InputArray, InputArray, OutputArray, OutputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:408
	// ("cv::cuda::Feature2DAsync::detectAndComputeAsync", vec![(pred!(mut, ["image", "mask", "keypoints", "descriptors", "useProvidedKeypoints", "stream"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "bool", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_Feature2DAsync_detectAndComputeAsync_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool_StreamR(cv::cuda::Feature2DAsync* instance, const cv::_InputArray* image, const cv::_InputArray* mask, const cv::_OutputArray* keypoints, const cv::_OutputArray* descriptors, bool useProvidedKeypoints, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			instance->detectAndComputeAsync(*image, *mask, *keypoints, *descriptors, useProvidedKeypoints, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::Feature2DAsync::detectAndComputeAsync(InputArray, InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:408
	// ("cv::cuda::Feature2DAsync::detectAndComputeAsync", vec![(pred!(mut, ["image", "mask", "keypoints", "descriptors"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_Feature2DAsync_detectAndComputeAsync_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::cuda::Feature2DAsync* instance, const cv::_InputArray* image, const cv::_InputArray* mask, const cv::_OutputArray* keypoints, const cv::_OutputArray* descriptors, ResultVoid* ocvrs_return) {
		try {
			instance->detectAndComputeAsync(*image, *mask, *keypoints, *descriptors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// convert(InputArray, std::vector<KeyPoint> &)(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:416
	// ("cv::cuda::Feature2DAsync::convert", vec![(pred!(mut, ["gpu_keypoints", "keypoints"], ["const cv::_InputArray*", "std::vector<cv::KeyPoint>*"]), _)]),
	void cv_cuda_Feature2DAsync_convert_const__InputArrayR_vectorLKeyPointGR(cv::cuda::Feature2DAsync* instance, const cv::_InputArray* gpu_keypoints, std::vector<cv::KeyPoint>* keypoints, ResultVoid* ocvrs_return) {
		try {
			instance->convert(*gpu_keypoints, *keypoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::Feature2DAsync::to_CUDA_FastFeatureDetector() generated
	// ("cv::cuda::Feature2DAsync::to_CUDA_FastFeatureDetector", vec![(pred!(mut, [], []), _)]),
	cv::cuda::FastFeatureDetector* cv_cuda_Feature2DAsync_to_CUDA_FastFeatureDetector(cv::cuda::Feature2DAsync* instance) {
			return dynamic_cast<cv::cuda::FastFeatureDetector*>(instance);
	}

	// cv::cuda::Feature2DAsync::to_CUDA_ORB() generated
	// ("cv::cuda::Feature2DAsync::to_CUDA_ORB", vec![(pred!(mut, [], []), _)]),
	cv::cuda::ORB* cv_cuda_Feature2DAsync_to_CUDA_ORB(cv::cuda::Feature2DAsync* instance) {
			return dynamic_cast<cv::cuda::ORB*>(instance);
	}

	// cv::cuda::Feature2DAsync::to_Algorithm() generated
	// ("cv::cuda::Feature2DAsync::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_cuda_Feature2DAsync_to_Algorithm(cv::cuda::Feature2DAsync* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::cuda::Feature2DAsync::to_Feature2D() generated
	// ("cv::cuda::Feature2DAsync::to_Feature2D", vec![(pred!(mut, [], []), _)]),
	cv::Feature2D* cv_cuda_Feature2DAsync_to_Feature2D(cv::cuda::Feature2DAsync* instance) {
			return dynamic_cast<cv::Feature2D*>(instance);
	}

	// cv::cuda::Feature2DAsync::delete() generated
	// ("cv::cuda::Feature2DAsync::delete", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_Feature2DAsync_delete(cv::cuda::Feature2DAsync* instance) {
			delete instance;
	}

	// create(int, float, int, int, int, int, int, int, int, bool)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:463
	// ("cv::cuda::ORB::create", vec![(pred!(mut, ["nfeatures", "scaleFactor", "nlevels", "edgeThreshold", "firstLevel", "WTA_K", "scoreType", "patchSize", "fastThreshold", "blurForDescriptor"], ["int", "float", "int", "int", "int", "int", "int", "int", "int", "bool"]), _)]),
	void cv_cuda_ORB_create_int_float_int_int_int_int_int_int_int_bool(int nfeatures, float scaleFactor, int nlevels, int edgeThreshold, int firstLevel, int WTA_K, int scoreType, int patchSize, int fastThreshold, bool blurForDescriptor, Result<cv::Ptr<cv::cuda::ORB>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::ORB> ret = cv::cuda::ORB::create(nfeatures, scaleFactor, nlevels, edgeThreshold, firstLevel, WTA_K, scoreType, patchSize, fastThreshold, blurForDescriptor);
			Ok(new cv::Ptr<cv::cuda::ORB>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::ORB::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:463
	// ("cv::cuda::ORB::create", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_ORB_create(Result<cv::Ptr<cv::cuda::ORB>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::ORB> ret = cv::cuda::ORB::create();
			Ok(new cv::Ptr<cv::cuda::ORB>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxFeatures(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:474
	// ("cv::cuda::ORB::setMaxFeatures", vec![(pred!(mut, ["maxFeatures"], ["int"]), _)]),
	void cv_cuda_ORB_setMaxFeatures_int(cv::cuda::ORB* instance, int maxFeatures, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxFeatures(maxFeatures);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxFeatures()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:475
	// ("cv::cuda::ORB::getMaxFeatures", vec![(pred!(const, [], []), _)]),
	void cv_cuda_ORB_getMaxFeatures_const(const cv::cuda::ORB* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMaxFeatures();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setScaleFactor(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:477
	// ("cv::cuda::ORB::setScaleFactor", vec![(pred!(mut, ["scaleFactor"], ["double"]), _)]),
	void cv_cuda_ORB_setScaleFactor_double(cv::cuda::ORB* instance, double scaleFactor, ResultVoid* ocvrs_return) {
		try {
			instance->setScaleFactor(scaleFactor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getScaleFactor()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:478
	// ("cv::cuda::ORB::getScaleFactor", vec![(pred!(const, [], []), _)]),
	void cv_cuda_ORB_getScaleFactor_const(const cv::cuda::ORB* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getScaleFactor();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNLevels(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:480
	// ("cv::cuda::ORB::setNLevels", vec![(pred!(mut, ["nlevels"], ["int"]), _)]),
	void cv_cuda_ORB_setNLevels_int(cv::cuda::ORB* instance, int nlevels, ResultVoid* ocvrs_return) {
		try {
			instance->setNLevels(nlevels);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNLevels()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:481
	// ("cv::cuda::ORB::getNLevels", vec![(pred!(const, [], []), _)]),
	void cv_cuda_ORB_getNLevels_const(const cv::cuda::ORB* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNLevels();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setEdgeThreshold(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:483
	// ("cv::cuda::ORB::setEdgeThreshold", vec![(pred!(mut, ["edgeThreshold"], ["int"]), _)]),
	void cv_cuda_ORB_setEdgeThreshold_int(cv::cuda::ORB* instance, int edgeThreshold, ResultVoid* ocvrs_return) {
		try {
			instance->setEdgeThreshold(edgeThreshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getEdgeThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:484
	// ("cv::cuda::ORB::getEdgeThreshold", vec![(pred!(const, [], []), _)]),
	void cv_cuda_ORB_getEdgeThreshold_const(const cv::cuda::ORB* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getEdgeThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setFirstLevel(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:486
	// ("cv::cuda::ORB::setFirstLevel", vec![(pred!(mut, ["firstLevel"], ["int"]), _)]),
	void cv_cuda_ORB_setFirstLevel_int(cv::cuda::ORB* instance, int firstLevel, ResultVoid* ocvrs_return) {
		try {
			instance->setFirstLevel(firstLevel);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getFirstLevel()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:487
	// ("cv::cuda::ORB::getFirstLevel", vec![(pred!(const, [], []), _)]),
	void cv_cuda_ORB_getFirstLevel_const(const cv::cuda::ORB* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getFirstLevel();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWTA_K(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:489
	// ("cv::cuda::ORB::setWTA_K", vec![(pred!(mut, ["wta_k"], ["int"]), _)]),
	void cv_cuda_ORB_setWTA_K_int(cv::cuda::ORB* instance, int wta_k, ResultVoid* ocvrs_return) {
		try {
			instance->setWTA_K(wta_k);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getWTA_K()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:490
	// ("cv::cuda::ORB::getWTA_K", vec![(pred!(const, [], []), _)]),
	void cv_cuda_ORB_getWTA_K_const(const cv::cuda::ORB* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getWTA_K();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setScoreType(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:492
	// ("cv::cuda::ORB::setScoreType", vec![(pred!(mut, ["scoreType"], ["int"]), _)]),
	void cv_cuda_ORB_setScoreType_int(cv::cuda::ORB* instance, int scoreType, ResultVoid* ocvrs_return) {
		try {
			instance->setScoreType(scoreType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getScoreType()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:493
	// ("cv::cuda::ORB::getScoreType", vec![(pred!(const, [], []), _)]),
	void cv_cuda_ORB_getScoreType_const(const cv::cuda::ORB* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getScoreType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setPatchSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:495
	// ("cv::cuda::ORB::setPatchSize", vec![(pred!(mut, ["patchSize"], ["int"]), _)]),
	void cv_cuda_ORB_setPatchSize_int(cv::cuda::ORB* instance, int patchSize, ResultVoid* ocvrs_return) {
		try {
			instance->setPatchSize(patchSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPatchSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:496
	// ("cv::cuda::ORB::getPatchSize", vec![(pred!(const, [], []), _)]),
	void cv_cuda_ORB_getPatchSize_const(const cv::cuda::ORB* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getPatchSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setFastThreshold(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:498
	// ("cv::cuda::ORB::setFastThreshold", vec![(pred!(mut, ["fastThreshold"], ["int"]), _)]),
	void cv_cuda_ORB_setFastThreshold_int(cv::cuda::ORB* instance, int fastThreshold, ResultVoid* ocvrs_return) {
		try {
			instance->setFastThreshold(fastThreshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getFastThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:499
	// ("cv::cuda::ORB::getFastThreshold", vec![(pred!(const, [], []), _)]),
	void cv_cuda_ORB_getFastThreshold_const(const cv::cuda::ORB* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getFastThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setBlurForDescriptor(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:502
	// ("cv::cuda::ORB::setBlurForDescriptor", vec![(pred!(mut, ["blurForDescriptor"], ["bool"]), _)]),
	void cv_cuda_ORB_setBlurForDescriptor_bool(cv::cuda::ORB* instance, bool blurForDescriptor, ResultVoid* ocvrs_return) {
		try {
			instance->setBlurForDescriptor(blurForDescriptor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getBlurForDescriptor()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudafeatures2d.hpp:503
	// ("cv::cuda::ORB::getBlurForDescriptor", vec![(pred!(const, [], []), _)]),
	void cv_cuda_ORB_getBlurForDescriptor_const(const cv::cuda::ORB* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getBlurForDescriptor();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::ORB::to_Algorithm() generated
	// ("cv::cuda::ORB::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_cuda_ORB_to_Algorithm(cv::cuda::ORB* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::cuda::ORB::to_Feature2D() generated
	// ("cv::cuda::ORB::to_Feature2D", vec![(pred!(mut, [], []), _)]),
	cv::Feature2D* cv_cuda_ORB_to_Feature2D(cv::cuda::ORB* instance) {
			return dynamic_cast<cv::Feature2D*>(instance);
	}

	// cv::cuda::ORB::to_CUDA_Feature2DAsync() generated
	// ("cv::cuda::ORB::to_CUDA_Feature2DAsync", vec![(pred!(mut, [], []), _)]),
	cv::cuda::Feature2DAsync* cv_cuda_ORB_to_CUDA_Feature2DAsync(cv::cuda::ORB* instance) {
			return dynamic_cast<cv::cuda::Feature2DAsync*>(instance);
	}

	// cv::cuda::ORB::delete() generated
	// ("cv::cuda::ORB::delete", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_ORB_delete(cv::cuda::ORB* instance) {
			delete instance;
	}

}
