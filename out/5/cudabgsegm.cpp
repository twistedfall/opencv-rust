#include "ocvrs_common.hpp"
#include <opencv2/cudabgsegm.hpp>
#include "cudabgsegm_types.hpp"

extern "C" {
	// cv::cuda::createBackgroundSubtractorMOG() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudabgsegm.hpp:116
	// ("cv::cuda::createBackgroundSubtractorMOG", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_createBackgroundSubtractorMOG(Result<cv::Ptr<cv::cuda::BackgroundSubtractorMOG>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::BackgroundSubtractorMOG> ret = cv::cuda::createBackgroundSubtractorMOG();
			Ok(new cv::Ptr<cv::cuda::BackgroundSubtractorMOG>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::createBackgroundSubtractorMOG2() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudabgsegm.hpp:155
	// ("cv::cuda::createBackgroundSubtractorMOG2", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_createBackgroundSubtractorMOG2(Result<cv::Ptr<cv::cuda::BackgroundSubtractorMOG2>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::BackgroundSubtractorMOG2> ret = cv::cuda::createBackgroundSubtractorMOG2();
			Ok(new cv::Ptr<cv::cuda::BackgroundSubtractorMOG2>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createBackgroundSubtractorMOG2(int, double, bool)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudabgsegm.hpp:155
	// ("cv::cuda::createBackgroundSubtractorMOG2", vec![(pred!(mut, ["history", "varThreshold", "detectShadows"], ["int", "double", "bool"]), _)]),
	void cv_cuda_createBackgroundSubtractorMOG2_int_double_bool(int history, double varThreshold, bool detectShadows, Result<cv::Ptr<cv::cuda::BackgroundSubtractorMOG2>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::BackgroundSubtractorMOG2> ret = cv::cuda::createBackgroundSubtractorMOG2(history, varThreshold, detectShadows);
			Ok(new cv::Ptr<cv::cuda::BackgroundSubtractorMOG2>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createBackgroundSubtractorMOG(int, int, double, double)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudabgsegm.hpp:116
	// ("cv::cuda::createBackgroundSubtractorMOG", vec![(pred!(mut, ["history", "nmixtures", "backgroundRatio", "noiseSigma"], ["int", "int", "double", "double"]), _)]),
	void cv_cuda_createBackgroundSubtractorMOG_int_int_double_double(int history, int nmixtures, double backgroundRatio, double noiseSigma, Result<cv::Ptr<cv::cuda::BackgroundSubtractorMOG>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::BackgroundSubtractorMOG> ret = cv::cuda::createBackgroundSubtractorMOG(history, nmixtures, backgroundRatio, noiseSigma);
			Ok(new cv::Ptr<cv::cuda::BackgroundSubtractorMOG>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// apply(InputArray, OutputArray, double, Stream &)(InputArray, OutputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudabgsegm.hpp:85
	// ("cv::cuda::BackgroundSubtractorMOG::apply", vec![(pred!(mut, ["image", "fgmask", "learningRate", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_BackgroundSubtractorMOG_apply_const__InputArrayR_const__OutputArrayR_double_StreamR(cv::cuda::BackgroundSubtractorMOG* instance, const cv::_InputArray* image, const cv::_OutputArray* fgmask, double learningRate, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			instance->apply(*image, *fgmask, learningRate, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getBackgroundImage(OutputArray, Stream &)(OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudabgsegm.hpp:88
	// ("cv::cuda::BackgroundSubtractorMOG::getBackgroundImage", vec![(pred!(const, ["backgroundImage", "stream"], ["const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_BackgroundSubtractorMOG_getBackgroundImage_const_const__OutputArrayR_StreamR(const cv::cuda::BackgroundSubtractorMOG* instance, const cv::_OutputArray* backgroundImage, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			instance->getBackgroundImage(*backgroundImage, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getBackgroundImage(GpuMat &, Stream &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudabgsegm.hpp:90
	// ("cv::cuda::BackgroundSubtractorMOG::getBackgroundImage", vec![(pred!(mut, ["backgroundImage", "stream"], ["cv::cuda::GpuMat*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_BackgroundSubtractorMOG_getBackgroundImage_GpuMatR_StreamR(cv::cuda::BackgroundSubtractorMOG* instance, cv::cuda::GpuMat* backgroundImage, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			instance->getBackgroundImage(*backgroundImage, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getHistory()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudabgsegm.hpp:94
	// ("cv::cuda::BackgroundSubtractorMOG::getHistory", vec![(pred!(const, [], []), _)]),
	void cv_cuda_BackgroundSubtractorMOG_getHistory_const(const cv::cuda::BackgroundSubtractorMOG* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getHistory();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setHistory(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudabgsegm.hpp:95
	// ("cv::cuda::BackgroundSubtractorMOG::setHistory", vec![(pred!(mut, ["nframes"], ["int"]), _)]),
	void cv_cuda_BackgroundSubtractorMOG_setHistory_int(cv::cuda::BackgroundSubtractorMOG* instance, int nframes, ResultVoid* ocvrs_return) {
		try {
			instance->setHistory(nframes);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNMixtures()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudabgsegm.hpp:97
	// ("cv::cuda::BackgroundSubtractorMOG::getNMixtures", vec![(pred!(const, [], []), _)]),
	void cv_cuda_BackgroundSubtractorMOG_getNMixtures_const(const cv::cuda::BackgroundSubtractorMOG* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNMixtures();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNMixtures(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudabgsegm.hpp:98
	// ("cv::cuda::BackgroundSubtractorMOG::setNMixtures", vec![(pred!(mut, ["nmix"], ["int"]), _)]),
	void cv_cuda_BackgroundSubtractorMOG_setNMixtures_int(cv::cuda::BackgroundSubtractorMOG* instance, int nmix, ResultVoid* ocvrs_return) {
		try {
			instance->setNMixtures(nmix);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getBackgroundRatio()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudabgsegm.hpp:100
	// ("cv::cuda::BackgroundSubtractorMOG::getBackgroundRatio", vec![(pred!(const, [], []), _)]),
	void cv_cuda_BackgroundSubtractorMOG_getBackgroundRatio_const(const cv::cuda::BackgroundSubtractorMOG* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getBackgroundRatio();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setBackgroundRatio(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudabgsegm.hpp:101
	// ("cv::cuda::BackgroundSubtractorMOG::setBackgroundRatio", vec![(pred!(mut, ["backgroundRatio"], ["double"]), _)]),
	void cv_cuda_BackgroundSubtractorMOG_setBackgroundRatio_double(cv::cuda::BackgroundSubtractorMOG* instance, double backgroundRatio, ResultVoid* ocvrs_return) {
		try {
			instance->setBackgroundRatio(backgroundRatio);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNoiseSigma()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudabgsegm.hpp:103
	// ("cv::cuda::BackgroundSubtractorMOG::getNoiseSigma", vec![(pred!(const, [], []), _)]),
	void cv_cuda_BackgroundSubtractorMOG_getNoiseSigma_const(const cv::cuda::BackgroundSubtractorMOG* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getNoiseSigma();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNoiseSigma(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudabgsegm.hpp:104
	// ("cv::cuda::BackgroundSubtractorMOG::setNoiseSigma", vec![(pred!(mut, ["noiseSigma"], ["double"]), _)]),
	void cv_cuda_BackgroundSubtractorMOG_setNoiseSigma_double(cv::cuda::BackgroundSubtractorMOG* instance, double noiseSigma, ResultVoid* ocvrs_return) {
		try {
			instance->setNoiseSigma(noiseSigma);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::BackgroundSubtractorMOG::to_Algorithm() generated
	// ("cv::cuda::BackgroundSubtractorMOG::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_cuda_BackgroundSubtractorMOG_to_Algorithm(cv::cuda::BackgroundSubtractorMOG* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::cuda::BackgroundSubtractorMOG::to_BackgroundSubtractor() generated
	// ("cv::cuda::BackgroundSubtractorMOG::to_BackgroundSubtractor", vec![(pred!(mut, [], []), _)]),
	cv::BackgroundSubtractor* cv_cuda_BackgroundSubtractorMOG_to_BackgroundSubtractor(cv::cuda::BackgroundSubtractorMOG* instance) {
			return dynamic_cast<cv::BackgroundSubtractor*>(instance);
	}

	// cv::cuda::BackgroundSubtractorMOG::delete() generated
	// ("cv::cuda::BackgroundSubtractorMOG::delete", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_BackgroundSubtractorMOG_delete(cv::cuda::BackgroundSubtractorMOG* instance) {
			delete instance;
	}

	// apply(InputArray, OutputArray, double, Stream &)(InputArray, OutputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudabgsegm.hpp:136
	// ("cv::cuda::BackgroundSubtractorMOG2::apply", vec![(pred!(mut, ["image", "fgmask", "learningRate", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_BackgroundSubtractorMOG2_apply_const__InputArrayR_const__OutputArrayR_double_StreamR(cv::cuda::BackgroundSubtractorMOG2* instance, const cv::_InputArray* image, const cv::_OutputArray* fgmask, double learningRate, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			instance->apply(*image, *fgmask, learningRate, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getBackgroundImage(OutputArray, Stream &)(OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudabgsegm.hpp:138
	// ("cv::cuda::BackgroundSubtractorMOG2::getBackgroundImage", vec![(pred!(const, ["backgroundImage", "stream"], ["const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_BackgroundSubtractorMOG2_getBackgroundImage_const_const__OutputArrayR_StreamR(const cv::cuda::BackgroundSubtractorMOG2* instance, const cv::_OutputArray* backgroundImage, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			instance->getBackgroundImage(*backgroundImage, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getBackgroundImage(GpuMat &, Stream &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudabgsegm.hpp:140
	// ("cv::cuda::BackgroundSubtractorMOG2::getBackgroundImage", vec![(pred!(mut, ["backgroundImage", "stream"], ["cv::cuda::GpuMat*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_BackgroundSubtractorMOG2_getBackgroundImage_GpuMatR_StreamR(cv::cuda::BackgroundSubtractorMOG2* instance, cv::cuda::GpuMat* backgroundImage, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			instance->getBackgroundImage(*backgroundImage, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::BackgroundSubtractorMOG2::to_Algorithm() generated
	// ("cv::cuda::BackgroundSubtractorMOG2::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_cuda_BackgroundSubtractorMOG2_to_Algorithm(cv::cuda::BackgroundSubtractorMOG2* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::cuda::BackgroundSubtractorMOG2::to_BackgroundSubtractor() generated
	// ("cv::cuda::BackgroundSubtractorMOG2::to_BackgroundSubtractor", vec![(pred!(mut, [], []), _)]),
	cv::BackgroundSubtractor* cv_cuda_BackgroundSubtractorMOG2_to_BackgroundSubtractor(cv::cuda::BackgroundSubtractorMOG2* instance) {
			return dynamic_cast<cv::BackgroundSubtractor*>(instance);
	}

	// cv::cuda::BackgroundSubtractorMOG2::to_BackgroundSubtractorMOG2() generated
	// ("cv::cuda::BackgroundSubtractorMOG2::to_BackgroundSubtractorMOG2", vec![(pred!(mut, [], []), _)]),
	cv::BackgroundSubtractorMOG2* cv_cuda_BackgroundSubtractorMOG2_to_BackgroundSubtractorMOG2(cv::cuda::BackgroundSubtractorMOG2* instance) {
			return dynamic_cast<cv::BackgroundSubtractorMOG2*>(instance);
	}

	// cv::cuda::BackgroundSubtractorMOG2::delete() generated
	// ("cv::cuda::BackgroundSubtractorMOG2::delete", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_BackgroundSubtractorMOG2_delete(cv::cuda::BackgroundSubtractorMOG2* instance) {
			delete instance;
	}

}
