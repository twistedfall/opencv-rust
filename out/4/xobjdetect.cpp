#include "ocvrs_common.hpp"
#include <opencv2/xobjdetect.hpp>
#include "xobjdetect_types.hpp"

extern "C" {
	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/xobjdetect.hpp:66
	// ("cv::xobjdetect::WBDetector::read", vec![(pred!(mut, ["node"], ["const cv::FileNode*"]), _)]),
	void cv_xobjdetect_WBDetector_read_const_FileNodeR(cv::xobjdetect::WBDetector* instance, const cv::FileNode* node, ResultVoid* ocvrs_return) {
		try {
			instance->read(*node);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/xobjdetect.hpp:71
	// ("cv::xobjdetect::WBDetector::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	void cv_xobjdetect_WBDetector_write_const_FileStorageR(const cv::xobjdetect::WBDetector* instance, cv::FileStorage* fs, ResultVoid* ocvrs_return) {
		try {
			instance->write(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// train(const std::string &, const std::string &)(InString, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/xobjdetect.hpp:77
	// ("cv::xobjdetect::WBDetector::train", vec![(pred!(mut, ["pos_samples", "neg_imgs"], ["const std::string*", "const std::string*"]), _)]),
	void cv_xobjdetect_WBDetector_train_const_stringR_const_stringR(cv::xobjdetect::WBDetector* instance, const char* pos_samples, const char* neg_imgs, ResultVoid* ocvrs_return) {
		try {
			instance->train(std::string(pos_samples), std::string(neg_imgs));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detect(const Mat &, std::vector<Rect> &, std::vector<double> &)(TraitClass, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/xobjdetect.hpp:86
	// ("cv::xobjdetect::WBDetector::detect", vec![(pred!(mut, ["img", "bboxes", "confidences"], ["const cv::Mat*", "std::vector<cv::Rect>*", "std::vector<double>*"]), _)]),
	void cv_xobjdetect_WBDetector_detect_const_MatR_vectorLRectGR_vectorLdoubleGR(cv::xobjdetect::WBDetector* instance, const cv::Mat* img, std::vector<cv::Rect>* bboxes, std::vector<double>* confidences, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*img, *bboxes, *confidences);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/xobjdetect.hpp:93
	// ("cv::xobjdetect::WBDetector::create", vec![(pred!(mut, [], []), _)]),
	void cv_xobjdetect_WBDetector_create(Result<cv::Ptr<cv::xobjdetect::WBDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xobjdetect::WBDetector> ret = cv::xobjdetect::WBDetector::create();
			Ok(new cv::Ptr<cv::xobjdetect::WBDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xobjdetect::WBDetector::delete() generated
	// ("cv::xobjdetect::WBDetector::delete", vec![(pred!(mut, [], []), _)]),
	void cv_xobjdetect_WBDetector_delete(cv::xobjdetect::WBDetector* instance) {
			delete instance;
	}

}
