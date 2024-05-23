#include "ocvrs_common.hpp"
#include <opencv2/cudaobjdetect.hpp>
#include "cudaobjdetect_types.hpp"

extern "C" {
	// create(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:239
	// ("cv::cuda::CascadeClassifier::create", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
	void cv_cuda_CascadeClassifier_create_const_StringR(const char* filename, Result<cv::Ptr<cv::cuda::CascadeClassifier>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::CascadeClassifier> ret = cv::cuda::CascadeClassifier::create(std::string(filename));
			Ok(new cv::Ptr<cv::cuda::CascadeClassifier>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:242
	// ("cv::cuda::CascadeClassifier::create", vec![(pred!(mut, ["file"], ["const cv::FileStorage*"]), _)]),
	void cv_cuda_CascadeClassifier_create_const_FileStorageR(const cv::FileStorage* file, Result<cv::Ptr<cv::cuda::CascadeClassifier>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::CascadeClassifier> ret = cv::cuda::CascadeClassifier::create(*file);
			Ok(new cv::Ptr<cv::cuda::CascadeClassifier>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxObjectSize(Size)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:246
	// ("cv::cuda::CascadeClassifier::setMaxObjectSize", vec![(pred!(mut, ["maxObjectSize"], ["cv::Size"]), _)]),
	void cv_cuda_CascadeClassifier_setMaxObjectSize_Size(cv::cuda::CascadeClassifier* instance, cv::Size* maxObjectSize, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxObjectSize(*maxObjectSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxObjectSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:247
	// ("cv::cuda::CascadeClassifier::getMaxObjectSize", vec![(pred!(const, [], []), _)]),
	void cv_cuda_CascadeClassifier_getMaxObjectSize_const(const cv::cuda::CascadeClassifier* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->getMaxObjectSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMinObjectSize(Size)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:250
	// ("cv::cuda::CascadeClassifier::setMinObjectSize", vec![(pred!(mut, ["minSize"], ["cv::Size"]), _)]),
	void cv_cuda_CascadeClassifier_setMinObjectSize_Size(cv::cuda::CascadeClassifier* instance, cv::Size* minSize, ResultVoid* ocvrs_return) {
		try {
			instance->setMinObjectSize(*minSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMinObjectSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:251
	// ("cv::cuda::CascadeClassifier::getMinObjectSize", vec![(pred!(const, [], []), _)]),
	void cv_cuda_CascadeClassifier_getMinObjectSize_const(const cv::cuda::CascadeClassifier* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->getMinObjectSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setScaleFactor(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:254
	// ("cv::cuda::CascadeClassifier::setScaleFactor", vec![(pred!(mut, ["scaleFactor"], ["double"]), _)]),
	void cv_cuda_CascadeClassifier_setScaleFactor_double(cv::cuda::CascadeClassifier* instance, double scaleFactor, ResultVoid* ocvrs_return) {
		try {
			instance->setScaleFactor(scaleFactor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getScaleFactor()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:255
	// ("cv::cuda::CascadeClassifier::getScaleFactor", vec![(pred!(const, [], []), _)]),
	void cv_cuda_CascadeClassifier_getScaleFactor_const(const cv::cuda::CascadeClassifier* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getScaleFactor();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMinNeighbors(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:259
	// ("cv::cuda::CascadeClassifier::setMinNeighbors", vec![(pred!(mut, ["minNeighbors"], ["int"]), _)]),
	void cv_cuda_CascadeClassifier_setMinNeighbors_int(cv::cuda::CascadeClassifier* instance, int minNeighbors, ResultVoid* ocvrs_return) {
		try {
			instance->setMinNeighbors(minNeighbors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMinNeighbors()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:260
	// ("cv::cuda::CascadeClassifier::getMinNeighbors", vec![(pred!(const, [], []), _)]),
	void cv_cuda_CascadeClassifier_getMinNeighbors_const(const cv::cuda::CascadeClassifier* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMinNeighbors();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setFindLargestObject(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:262
	// ("cv::cuda::CascadeClassifier::setFindLargestObject", vec![(pred!(mut, ["findLargestObject"], ["bool"]), _)]),
	void cv_cuda_CascadeClassifier_setFindLargestObject_bool(cv::cuda::CascadeClassifier* instance, bool findLargestObject, ResultVoid* ocvrs_return) {
		try {
			instance->setFindLargestObject(findLargestObject);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getFindLargestObject()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:263
	// ("cv::cuda::CascadeClassifier::getFindLargestObject", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_CascadeClassifier_getFindLargestObject(cv::cuda::CascadeClassifier* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getFindLargestObject();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxNumObjects(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:265
	// ("cv::cuda::CascadeClassifier::setMaxNumObjects", vec![(pred!(mut, ["maxNumObjects"], ["int"]), _)]),
	void cv_cuda_CascadeClassifier_setMaxNumObjects_int(cv::cuda::CascadeClassifier* instance, int maxNumObjects, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxNumObjects(maxNumObjects);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxNumObjects()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:266
	// ("cv::cuda::CascadeClassifier::getMaxNumObjects", vec![(pred!(const, [], []), _)]),
	void cv_cuda_CascadeClassifier_getMaxNumObjects_const(const cv::cuda::CascadeClassifier* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMaxNumObjects();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getClassifierSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:268
	// ("cv::cuda::CascadeClassifier::getClassifierSize", vec![(pred!(const, [], []), _)]),
	void cv_cuda_CascadeClassifier_getClassifierSize_const(const cv::cuda::CascadeClassifier* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->getClassifierSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectMultiScale(InputArray, OutputArray, Stream &)(InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:298
	// ("cv::cuda::CascadeClassifier::detectMultiScale", vec![(pred!(mut, ["image", "objects", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_CascadeClassifier_detectMultiScale_const__InputArrayR_const__OutputArrayR_StreamR(cv::cuda::CascadeClassifier* instance, const cv::_InputArray* image, const cv::_OutputArray* objects, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			instance->detectMultiScale(*image, *objects, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::CascadeClassifier::detectMultiScale(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:298
	// ("cv::cuda::CascadeClassifier::detectMultiScale", vec![(pred!(mut, ["image", "objects"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_CascadeClassifier_detectMultiScale_const__InputArrayR_const__OutputArrayR(cv::cuda::CascadeClassifier* instance, const cv::_InputArray* image, const cv::_OutputArray* objects, ResultVoid* ocvrs_return) {
		try {
			instance->detectMultiScale(*image, *objects);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// convert(OutputArray, std::vector<Rect> &)(OutputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:307
	// ("cv::cuda::CascadeClassifier::convert", vec![(pred!(mut, ["gpu_objects", "objects"], ["const cv::_OutputArray*", "std::vector<cv::Rect>*"]), _)]),
	void cv_cuda_CascadeClassifier_convert_const__OutputArrayR_vectorLRectGR(cv::cuda::CascadeClassifier* instance, const cv::_OutputArray* gpu_objects, std::vector<cv::Rect>* objects, ResultVoid* ocvrs_return) {
		try {
			instance->convert(*gpu_objects, *objects);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::CascadeClassifier::to_Algorithm() generated
	// ("cv::cuda::CascadeClassifier::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_cuda_CascadeClassifier_to_Algorithm(cv::cuda::CascadeClassifier* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::cuda::CascadeClassifier::delete() generated
	// ("cv::cuda::CascadeClassifier::delete", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_CascadeClassifier_delete(cv::cuda::CascadeClassifier* instance) {
			delete instance;
	}

	// create(Size, Size, Size, Size, int)(SimpleClass, SimpleClass, SimpleClass, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:90
	// ("cv::cuda::HOG::create", vec![(pred!(mut, ["win_size", "block_size", "block_stride", "cell_size", "nbins"], ["cv::Size", "cv::Size", "cv::Size", "cv::Size", "int"]), _)]),
	void cv_cuda_HOG_create_Size_Size_Size_Size_int(cv::Size* win_size, cv::Size* block_size, cv::Size* block_stride, cv::Size* cell_size, int nbins, Result<cv::Ptr<cv::cuda::HOG>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::HOG> ret = cv::cuda::HOG::create(*win_size, *block_size, *block_stride, *cell_size, nbins);
			Ok(new cv::Ptr<cv::cuda::HOG>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::HOG::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:90
	// ("cv::cuda::HOG::create", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_HOG_create(Result<cv::Ptr<cv::cuda::HOG>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::HOG> ret = cv::cuda::HOG::create();
			Ok(new cv::Ptr<cv::cuda::HOG>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWinSigma(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:97
	// ("cv::cuda::HOG::setWinSigma", vec![(pred!(mut, ["win_sigma"], ["double"]), _)]),
	void cv_cuda_HOG_setWinSigma_double(cv::cuda::HOG* instance, double win_sigma, ResultVoid* ocvrs_return) {
		try {
			instance->setWinSigma(win_sigma);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getWinSigma()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:98
	// ("cv::cuda::HOG::getWinSigma", vec![(pred!(const, [], []), _)]),
	void cv_cuda_HOG_getWinSigma_const(const cv::cuda::HOG* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getWinSigma();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setL2HysThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:101
	// ("cv::cuda::HOG::setL2HysThreshold", vec![(pred!(mut, ["threshold_L2hys"], ["double"]), _)]),
	void cv_cuda_HOG_setL2HysThreshold_double(cv::cuda::HOG* instance, double threshold_L2hys, ResultVoid* ocvrs_return) {
		try {
			instance->setL2HysThreshold(threshold_L2hys);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getL2HysThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:102
	// ("cv::cuda::HOG::getL2HysThreshold", vec![(pred!(const, [], []), _)]),
	void cv_cuda_HOG_getL2HysThreshold_const(const cv::cuda::HOG* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getL2HysThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setGammaCorrection(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:105
	// ("cv::cuda::HOG::setGammaCorrection", vec![(pred!(mut, ["gamma_correction"], ["bool"]), _)]),
	void cv_cuda_HOG_setGammaCorrection_bool(cv::cuda::HOG* instance, bool gamma_correction, ResultVoid* ocvrs_return) {
		try {
			instance->setGammaCorrection(gamma_correction);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getGammaCorrection()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:106
	// ("cv::cuda::HOG::getGammaCorrection", vec![(pred!(const, [], []), _)]),
	void cv_cuda_HOG_getGammaCorrection_const(const cv::cuda::HOG* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getGammaCorrection();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNumLevels(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:109
	// ("cv::cuda::HOG::setNumLevels", vec![(pred!(mut, ["nlevels"], ["int"]), _)]),
	void cv_cuda_HOG_setNumLevels_int(cv::cuda::HOG* instance, int nlevels, ResultVoid* ocvrs_return) {
		try {
			instance->setNumLevels(nlevels);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNumLevels()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:110
	// ("cv::cuda::HOG::getNumLevels", vec![(pred!(const, [], []), _)]),
	void cv_cuda_HOG_getNumLevels_const(const cv::cuda::HOG* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNumLevels();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setHitThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:116
	// ("cv::cuda::HOG::setHitThreshold", vec![(pred!(mut, ["hit_threshold"], ["double"]), _)]),
	void cv_cuda_HOG_setHitThreshold_double(cv::cuda::HOG* instance, double hit_threshold, ResultVoid* ocvrs_return) {
		try {
			instance->setHitThreshold(hit_threshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getHitThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:117
	// ("cv::cuda::HOG::getHitThreshold", vec![(pred!(const, [], []), _)]),
	void cv_cuda_HOG_getHitThreshold_const(const cv::cuda::HOG* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getHitThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWinStride(Size)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:120
	// ("cv::cuda::HOG::setWinStride", vec![(pred!(mut, ["win_stride"], ["cv::Size"]), _)]),
	void cv_cuda_HOG_setWinStride_Size(cv::cuda::HOG* instance, cv::Size* win_stride, ResultVoid* ocvrs_return) {
		try {
			instance->setWinStride(*win_stride);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getWinStride()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:121
	// ("cv::cuda::HOG::getWinStride", vec![(pred!(const, [], []), _)]),
	void cv_cuda_HOG_getWinStride_const(const cv::cuda::HOG* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->getWinStride();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setScaleFactor(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:124
	// ("cv::cuda::HOG::setScaleFactor", vec![(pred!(mut, ["scale0"], ["double"]), _)]),
	void cv_cuda_HOG_setScaleFactor_double(cv::cuda::HOG* instance, double scale0, ResultVoid* ocvrs_return) {
		try {
			instance->setScaleFactor(scale0);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getScaleFactor()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:125
	// ("cv::cuda::HOG::getScaleFactor", vec![(pred!(const, [], []), _)]),
	void cv_cuda_HOG_getScaleFactor_const(const cv::cuda::HOG* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getScaleFactor();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setGroupThreshold(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:130
	// ("cv::cuda::HOG::setGroupThreshold", vec![(pred!(mut, ["group_threshold"], ["int"]), _)]),
	void cv_cuda_HOG_setGroupThreshold_int(cv::cuda::HOG* instance, int group_threshold, ResultVoid* ocvrs_return) {
		try {
			instance->setGroupThreshold(group_threshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getGroupThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:131
	// ("cv::cuda::HOG::getGroupThreshold", vec![(pred!(const, [], []), _)]),
	void cv_cuda_HOG_getGroupThreshold_const(const cv::cuda::HOG* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getGroupThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDescriptorFormat(HOGDescriptor::DescriptorStorageFormat)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:136
	// ("cv::cuda::HOG::setDescriptorFormat", vec![(pred!(mut, ["descr_format"], ["cv::HOGDescriptor::DescriptorStorageFormat"]), _)]),
	void cv_cuda_HOG_setDescriptorFormat_DescriptorStorageFormat(cv::cuda::HOG* instance, cv::HOGDescriptor::DescriptorStorageFormat descr_format, ResultVoid* ocvrs_return) {
		try {
			instance->setDescriptorFormat(descr_format);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDescriptorFormat()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:137
	// ("cv::cuda::HOG::getDescriptorFormat", vec![(pred!(const, [], []), _)]),
	void cv_cuda_HOG_getDescriptorFormat_const(const cv::cuda::HOG* instance, Result<cv::HOGDescriptor::DescriptorStorageFormat>* ocvrs_return) {
		try {
			cv::HOGDescriptor::DescriptorStorageFormat ret = instance->getDescriptorFormat();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDescriptorSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:141
	// ("cv::cuda::HOG::getDescriptorSize", vec![(pred!(const, [], []), _)]),
	void cv_cuda_HOG_getDescriptorSize_const(const cv::cuda::HOG* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->getDescriptorSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getBlockHistogramSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:145
	// ("cv::cuda::HOG::getBlockHistogramSize", vec![(pred!(const, [], []), _)]),
	void cv_cuda_HOG_getBlockHistogramSize_const(const cv::cuda::HOG* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->getBlockHistogramSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSVMDetector(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:149
	// ("cv::cuda::HOG::setSVMDetector", vec![(pred!(mut, ["detector"], ["const cv::_InputArray*"]), _)]),
	void cv_cuda_HOG_setSVMDetector_const__InputArrayR(cv::cuda::HOG* instance, const cv::_InputArray* detector, ResultVoid* ocvrs_return) {
		try {
			instance->setSVMDetector(*detector);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDefaultPeopleDetector()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:153
	// ("cv::cuda::HOG::getDefaultPeopleDetector", vec![(pred!(const, [], []), _)]),
	void cv_cuda_HOG_getDefaultPeopleDetector_const(const cv::cuda::HOG* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getDefaultPeopleDetector();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detect(InputArray, std::vector<Point> &, std::vector<double> *)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:161
	// ("cv::cuda::HOG::detect", vec![(pred!(mut, ["img", "found_locations", "confidences"], ["const cv::_InputArray*", "std::vector<cv::Point>*", "std::vector<double>*"]), _)]),
	void cv_cuda_HOG_detect_const__InputArrayR_vectorLPointGR_vectorLdoubleGX(cv::cuda::HOG* instance, const cv::_InputArray* img, std::vector<cv::Point>* found_locations, std::vector<double>* confidences, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*img, *found_locations, confidences);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::HOG::detect(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:161
	// ("cv::cuda::HOG::detect", vec![(pred!(mut, ["img", "found_locations"], ["const cv::_InputArray*", "std::vector<cv::Point>*"]), _)]),
	void cv_cuda_HOG_detect_const__InputArrayR_vectorLPointGR(cv::cuda::HOG* instance, const cv::_InputArray* img, std::vector<cv::Point>* found_locations, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*img, *found_locations);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detect(InputArray, std::vector<Point> &, std::vector<double> &)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:165
	// ("cv::cuda::HOG::detect", vec![(pred!(mut, ["img", "found_locations", "confidences"], ["const cv::_InputArray*", "std::vector<cv::Point>*", "std::vector<double>*"]), _)]),
	void cv_cuda_HOG_detect_const__InputArrayR_vectorLPointGR_vectorLdoubleGR(cv::cuda::HOG* instance, const cv::_InputArray* img, std::vector<cv::Point>* found_locations, std::vector<double>* confidences, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*img, *found_locations, *confidences);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectWithoutConf(InputArray, std::vector<Point> &)(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:176
	// ("cv::cuda::HOG::detectWithoutConf", vec![(pred!(mut, ["img", "found_locations"], ["const cv::_InputArray*", "std::vector<cv::Point>*"]), _)]),
	void cv_cuda_HOG_detectWithoutConf_const__InputArrayR_vectorLPointGR(cv::cuda::HOG* instance, const cv::_InputArray* img, std::vector<cv::Point>* found_locations, ResultVoid* ocvrs_return) {
		try {
			instance->detectWithoutConf(*img, *found_locations);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectMultiScale(InputArray, std::vector<Rect> &, std::vector<double> *)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:187
	// ("cv::cuda::HOG::detectMultiScale", vec![(pred!(mut, ["img", "found_locations", "confidences"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<double>*"]), _)]),
	void cv_cuda_HOG_detectMultiScale_const__InputArrayR_vectorLRectGR_vectorLdoubleGX(cv::cuda::HOG* instance, const cv::_InputArray* img, std::vector<cv::Rect>* found_locations, std::vector<double>* confidences, ResultVoid* ocvrs_return) {
		try {
			instance->detectMultiScale(*img, *found_locations, confidences);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::HOG::detectMultiScale(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:187
	// ("cv::cuda::HOG::detectMultiScale", vec![(pred!(mut, ["img", "found_locations"], ["const cv::_InputArray*", "std::vector<cv::Rect>*"]), _)]),
	void cv_cuda_HOG_detectMultiScale_const__InputArrayR_vectorLRectGR(cv::cuda::HOG* instance, const cv::_InputArray* img, std::vector<cv::Rect>* found_locations, ResultVoid* ocvrs_return) {
		try {
			instance->detectMultiScale(*img, *found_locations);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectMultiScale(InputArray, std::vector<Rect> &, std::vector<double> &)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:191
	// ("cv::cuda::HOG::detectMultiScale", vec![(pred!(mut, ["img", "found_locations", "confidences"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<double>*"]), _)]),
	void cv_cuda_HOG_detectMultiScale_const__InputArrayR_vectorLRectGR_vectorLdoubleGR(cv::cuda::HOG* instance, const cv::_InputArray* img, std::vector<cv::Rect>* found_locations, std::vector<double>* confidences, ResultVoid* ocvrs_return) {
		try {
			instance->detectMultiScale(*img, *found_locations, *confidences);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detectMultiScaleWithoutConf(InputArray, std::vector<Rect> &)(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:202
	// ("cv::cuda::HOG::detectMultiScaleWithoutConf", vec![(pred!(mut, ["img", "found_locations"], ["const cv::_InputArray*", "std::vector<cv::Rect>*"]), _)]),
	void cv_cuda_HOG_detectMultiScaleWithoutConf_const__InputArrayR_vectorLRectGR(cv::cuda::HOG* instance, const cv::_InputArray* img, std::vector<cv::Rect>* found_locations, ResultVoid* ocvrs_return) {
		try {
			instance->detectMultiScaleWithoutConf(*img, *found_locations);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// compute(InputArray, OutputArray, Stream &)(InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:213
	// ("cv::cuda::HOG::compute", vec![(pred!(mut, ["img", "descriptors", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::cuda::Stream*"]), _)]),
	void cv_cuda_HOG_compute_const__InputArrayR_const__OutputArrayR_StreamR(cv::cuda::HOG* instance, const cv::_InputArray* img, const cv::_OutputArray* descriptors, cv::cuda::Stream* stream, ResultVoid* ocvrs_return) {
		try {
			instance->compute(*img, *descriptors, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::HOG::compute(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudaobjdetect.hpp:213
	// ("cv::cuda::HOG::compute", vec![(pred!(mut, ["img", "descriptors"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_cuda_HOG_compute_const__InputArrayR_const__OutputArrayR(cv::cuda::HOG* instance, const cv::_InputArray* img, const cv::_OutputArray* descriptors, ResultVoid* ocvrs_return) {
		try {
			instance->compute(*img, *descriptors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cuda::HOG::to_Algorithm() generated
	// ("cv::cuda::HOG::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_cuda_HOG_to_Algorithm(cv::cuda::HOG* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::cuda::HOG::delete() generated
	// ("cv::cuda::HOG::delete", vec![(pred!(mut, [], []), _)]),
	void cv_cuda_HOG_delete(cv::cuda::HOG* instance) {
			delete instance;
	}

}
