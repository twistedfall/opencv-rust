#include "ocvrs_common.hpp"
#include <opencv2/dpm.hpp>
#include "dpm_types.hpp"

extern "C" {
	// isEmpty()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dpm.hpp:117
	// ("cv::dpm::DPMDetector::isEmpty", vec![(pred!(const, [], []), _)]),
	void cv_dpm_DPMDetector_isEmpty_const(const cv::dpm::DPMDetector* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isEmpty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// detect(cv::Mat &, std::vector<ObjectDetection> &)(TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dpm.hpp:124
	// ("cv::dpm::DPMDetector::detect", vec![(pred!(mut, ["image", "objects"], ["cv::Mat*", "std::vector<cv::dpm::DPMDetector::ObjectDetection>*"]), _)]),
	void cv_dpm_DPMDetector_detect_MatR_vectorLObjectDetectionGR(cv::dpm::DPMDetector* instance, cv::Mat* image, std::vector<cv::dpm::DPMDetector::ObjectDetection>* objects, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*image, *objects);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getClassNames()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dpm.hpp:129
	// ("cv::dpm::DPMDetector::getClassNames", vec![(pred!(const, [], []), _)]),
	void cv_dpm_DPMDetector_getClassNames_const(const cv::dpm::DPMDetector* instance, Result<std::vector<std::string>*>* ocvrs_return) {
		try {
			const std::vector<std::string> ret = instance->getClassNames();
			Ok(new const std::vector<std::string>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getClassCount()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dpm.hpp:133
	// ("cv::dpm::DPMDetector::getClassCount", vec![(pred!(const, [], []), _)]),
	void cv_dpm_DPMDetector_getClassCount_const(const cv::dpm::DPMDetector* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->getClassCount();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const std::vector<std::string> &, const std::vector<std::string> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dpm.hpp:142
	// ("cv::dpm::DPMDetector::create", vec![(pred!(mut, ["filenames", "classNames"], ["const std::vector<std::string>*", "const std::vector<std::string>*"]), _)]),
	void cv_dpm_DPMDetector_create_const_vectorLstringGR_const_vectorLstringGR(const std::vector<std::string>* filenames, const std::vector<std::string>* classNames, Result<cv::Ptr<cv::dpm::DPMDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dpm::DPMDetector> ret = cv::dpm::DPMDetector::create(*filenames, *classNames);
			Ok(new cv::Ptr<cv::dpm::DPMDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dpm::DPMDetector::create(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dpm.hpp:142
	// ("cv::dpm::DPMDetector::create", vec![(pred!(mut, ["filenames"], ["const std::vector<std::string>*"]), _)]),
	void cv_dpm_DPMDetector_create_const_vectorLstringGR(const std::vector<std::string>* filenames, Result<cv::Ptr<cv::dpm::DPMDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dpm::DPMDetector> ret = cv::dpm::DPMDetector::create(*filenames);
			Ok(new cv::Ptr<cv::dpm::DPMDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dpm::DPMDetector::delete() generated
	// ("cv::dpm::DPMDetector::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dpm_DPMDetector_delete(cv::dpm::DPMDetector* instance) {
			delete instance;
	}

	// ObjectDetection()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dpm.hpp:110
	// ("cv::dpm::DPMDetector::ObjectDetection::ObjectDetection", vec![(pred!(mut, [], []), _)]),
	void cv_dpm_DPMDetector_ObjectDetection_ObjectDetection(Result<cv::dpm::DPMDetector::ObjectDetection*>* ocvrs_return) {
		try {
			cv::dpm::DPMDetector::ObjectDetection* ret = new cv::dpm::DPMDetector::ObjectDetection();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ObjectDetection(const Rect &, float, int)(SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dpm.hpp:111
	// ("cv::dpm::DPMDetector::ObjectDetection::ObjectDetection", vec![(pred!(mut, ["rect", "score", "classID"], ["const cv::Rect*", "float", "int"]), _)]),
	void cv_dpm_DPMDetector_ObjectDetection_ObjectDetection_const_RectR_float_int(const cv::Rect* rect, float score, int classID, Result<cv::dpm::DPMDetector::ObjectDetection*>* ocvrs_return) {
		try {
			cv::dpm::DPMDetector::ObjectDetection* ret = new cv::dpm::DPMDetector::ObjectDetection(*rect, score, classID);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dpm::DPMDetector::ObjectDetection::ObjectDetection(SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dpm.hpp:111
	// ("cv::dpm::DPMDetector::ObjectDetection::ObjectDetection", vec![(pred!(mut, ["rect", "score"], ["const cv::Rect*", "float"]), _)]),
	void cv_dpm_DPMDetector_ObjectDetection_ObjectDetection_const_RectR_float(const cv::Rect* rect, float score, Result<cv::dpm::DPMDetector::ObjectDetection*>* ocvrs_return) {
		try {
			cv::dpm::DPMDetector::ObjectDetection* ret = new cv::dpm::DPMDetector::ObjectDetection(*rect, score);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::dpm::DPMDetector::ObjectDetection::rect() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dpm.hpp:112
	// ("cv::dpm::DPMDetector::ObjectDetection::rect", vec![(pred!(const, [], []), _)]),
	void cv_dpm_DPMDetector_ObjectDetection_propRect_const(const cv::dpm::DPMDetector::ObjectDetection* instance, cv::Rect* ocvrs_return) {
			cv::Rect ret = instance->rect;
			*ocvrs_return = ret;
	}

	// cv::dpm::DPMDetector::ObjectDetection::setRect(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dpm.hpp:112
	// ("cv::dpm::DPMDetector::ObjectDetection::setRect", vec![(pred!(mut, ["val"], ["const cv::Rect"]), _)]),
	void cv_dpm_DPMDetector_ObjectDetection_propRect_const_Rect(cv::dpm::DPMDetector::ObjectDetection* instance, const cv::Rect* val) {
			instance->rect = *val;
	}

	// cv::dpm::DPMDetector::ObjectDetection::score() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dpm.hpp:113
	// ("cv::dpm::DPMDetector::ObjectDetection::score", vec![(pred!(const, [], []), _)]),
	float cv_dpm_DPMDetector_ObjectDetection_propScore_const(const cv::dpm::DPMDetector::ObjectDetection* instance) {
			float ret = instance->score;
			return ret;
	}

	// cv::dpm::DPMDetector::ObjectDetection::setScore(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dpm.hpp:113
	// ("cv::dpm::DPMDetector::ObjectDetection::setScore", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_dpm_DPMDetector_ObjectDetection_propScore_const_float(cv::dpm::DPMDetector::ObjectDetection* instance, const float val) {
			instance->score = val;
	}

	// cv::dpm::DPMDetector::ObjectDetection::classID() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dpm.hpp:114
	// ("cv::dpm::DPMDetector::ObjectDetection::classID", vec![(pred!(const, [], []), _)]),
	int cv_dpm_DPMDetector_ObjectDetection_propClassID_const(const cv::dpm::DPMDetector::ObjectDetection* instance) {
			int ret = instance->classID;
			return ret;
	}

	// cv::dpm::DPMDetector::ObjectDetection::setClassID(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/dpm.hpp:114
	// ("cv::dpm::DPMDetector::ObjectDetection::setClassID", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_dpm_DPMDetector_ObjectDetection_propClassID_const_int(cv::dpm::DPMDetector::ObjectDetection* instance, const int val) {
			instance->classID = val;
	}

	// cv::dpm::DPMDetector::ObjectDetection::delete() generated
	// ("cv::dpm::DPMDetector::ObjectDetection::delete", vec![(pred!(mut, [], []), _)]),
	void cv_dpm_DPMDetector_ObjectDetection_delete(cv::dpm::DPMDetector::ObjectDetection* instance) {
			delete instance;
	}

}
