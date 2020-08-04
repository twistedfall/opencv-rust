#include "ocvrs_common.hpp"
#include <opencv2/objdetect.hpp>
#include "objdetect_types.hpp"

extern "C" {
	Result<cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>*> cv_createFaceDetectionMaskGenerator() {
		try {
			cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator> ret = cv::createFaceDetectionMaskGenerator();
			return Ok(new cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>*>))
	}
	
	Result<bool> cv_decodeQRCode_const__InputArrayR_const__InputArrayR_stringR_const__OutputArrayR(const cv::_InputArray* in, const cv::_InputArray* points, void** decoded_info, const cv::_OutputArray* straight_qrcode) {
		try {
			std::string decoded_info_out;
			bool ret = cv::decodeQRCode(*in, *points, decoded_info_out, *straight_qrcode);
			*decoded_info = ocvrs_create_string(decoded_info_out.c_str());
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_detectQRCode_const__InputArrayR_vector_Point_R_double_double(const cv::_InputArray* in, std::vector<cv::Point>* points, double eps_x, double eps_y) {
		try {
			bool ret = cv::detectQRCode(*in, *points, eps_x, eps_y);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_groupRectangles_meanshift_vector_Rect_R_vector_double_R_vector_double_R_double_Size(std::vector<cv::Rect>* rectList, std::vector<double>* foundWeights, std::vector<double>* foundScales, double detectThreshold, cv::Size* winDetSize) {
		try {
			cv::groupRectangles_meanshift(*rectList, *foundWeights, *foundScales, detectThreshold, *winDetSize);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_groupRectangles_vector_Rect_R_int_double(std::vector<cv::Rect>* rectList, int groupThreshold, double eps) {
		try {
			cv::groupRectangles(*rectList, groupThreshold, eps);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_groupRectangles_vector_Rect_R_int_double_vector_int_X_vector_double_X(std::vector<cv::Rect>* rectList, int groupThreshold, double eps, std::vector<int>* weights, std::vector<double>* levelWeights) {
		try {
			cv::groupRectangles(*rectList, groupThreshold, eps, weights, levelWeights);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_groupRectangles_vector_Rect_R_vector_int_R_int_double(std::vector<cv::Rect>* rectList, std::vector<int>* weights, int groupThreshold, double eps) {
		try {
			cv::groupRectangles(*rectList, *weights, groupThreshold, eps);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_groupRectangles_vector_Rect_R_vector_int_R_vector_double_R_int_double(std::vector<cv::Rect>* rectList, std::vector<int>* rejectLevels, std::vector<double>* levelWeights, int groupThreshold, double eps) {
		try {
			cv::groupRectangles(*rectList, *rejectLevels, *levelWeights, groupThreshold, eps);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_BaseCascadeClassifier_empty_const(const cv::BaseCascadeClassifier* instance) {
		try {
			bool ret = instance->empty();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_BaseCascadeClassifier_load_const_StringR(cv::BaseCascadeClassifier* instance, const char* filename) {
		try {
			bool ret = instance->load(cv::String(filename));
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_BaseCascadeClassifier_detectMultiScale_const__InputArrayR_vector_Rect_R_double_int_int_Size_Size(cv::BaseCascadeClassifier* instance, const cv::_InputArray* image, std::vector<cv::Rect>* objects, double scaleFactor, int minNeighbors, int flags, cv::Size* minSize, cv::Size* maxSize) {
		try {
			instance->detectMultiScale(*image, *objects, scaleFactor, minNeighbors, flags, *minSize, *maxSize);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_BaseCascadeClassifier_detectMultiScale_const__InputArrayR_vector_Rect_R_vector_int_R_double_int_int_Size_Size(cv::BaseCascadeClassifier* instance, const cv::_InputArray* image, std::vector<cv::Rect>* objects, std::vector<int>* numDetections, double scaleFactor, int minNeighbors, int flags, cv::Size* minSize, cv::Size* maxSize) {
		try {
			instance->detectMultiScale(*image, *objects, *numDetections, scaleFactor, minNeighbors, flags, *minSize, *maxSize);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_BaseCascadeClassifier_detectMultiScale_const__InputArrayR_vector_Rect_R_vector_int_R_vector_double_R_double_int_int_Size_Size_bool(cv::BaseCascadeClassifier* instance, const cv::_InputArray* image, std::vector<cv::Rect>* objects, std::vector<int>* rejectLevels, std::vector<double>* levelWeights, double scaleFactor, int minNeighbors, int flags, cv::Size* minSize, cv::Size* maxSize, bool outputRejectLevels) {
		try {
			instance->detectMultiScale(*image, *objects, *rejectLevels, *levelWeights, scaleFactor, minNeighbors, flags, *minSize, *maxSize, outputRejectLevels);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_BaseCascadeClassifier_isOldFormatCascade_const(const cv::BaseCascadeClassifier* instance) {
		try {
			bool ret = instance->isOldFormatCascade();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<cv::Size> cv_BaseCascadeClassifier_getOriginalWindowSize_const(const cv::BaseCascadeClassifier* instance) {
		try {
			cv::Size ret = instance->getOriginalWindowSize();
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	Result<int> cv_BaseCascadeClassifier_getFeatureType_const(const cv::BaseCascadeClassifier* instance) {
		try {
			int ret = instance->getFeatureType();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<void*> cv_BaseCascadeClassifier_getOldCascade(cv::BaseCascadeClassifier* instance) {
		try {
			void* ret = instance->getOldCascade();
			return Ok<void*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result_void cv_BaseCascadeClassifier_setMaskGenerator_const_Ptr_MaskGenerator_R(cv::BaseCascadeClassifier* instance, const cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>* maskGenerator) {
		try {
			instance->setMaskGenerator(*maskGenerator);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>*> cv_BaseCascadeClassifier_getMaskGenerator(cv::BaseCascadeClassifier* instance) {
		try {
			cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator> ret = instance->getMaskGenerator();
			return Ok(new cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>*>))
	}
	
	Result<cv::Mat*> cv_BaseCascadeClassifier_MaskGenerator_generateMask_const_MatR(cv::BaseCascadeClassifier::MaskGenerator* instance, const cv::Mat* src) {
		try {
			cv::Mat ret = instance->generateMask(*src);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_BaseCascadeClassifier_MaskGenerator_initializeMask_const_MatR(cv::BaseCascadeClassifier::MaskGenerator* instance, const cv::Mat* unnamed) {
		try {
			instance->initializeMask(*unnamed);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::BaseCascadeClassifier>*> cv_CascadeClassifier_getPropCc(cv::CascadeClassifier* instance) {
		try {
			cv::Ptr<cv::BaseCascadeClassifier> ret = instance->cc;
			return Ok(new cv::Ptr<cv::BaseCascadeClassifier>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::BaseCascadeClassifier>*>))
	}
	
	Result_void cv_CascadeClassifier_setPropCc_Ptr_BaseCascadeClassifier_(cv::CascadeClassifier* instance, cv::Ptr<cv::BaseCascadeClassifier>* val) {
		try {
			instance->cc = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_CascadeClassifier_delete(cv::CascadeClassifier* instance) {
		delete instance;
	}
	Result<cv::CascadeClassifier*> cv_CascadeClassifier_CascadeClassifier() {
		try {
			cv::CascadeClassifier* ret = new cv::CascadeClassifier();
			return Ok<cv::CascadeClassifier*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::CascadeClassifier*>))
	}
	
	Result<cv::CascadeClassifier*> cv_CascadeClassifier_CascadeClassifier_const_StringR(const char* filename) {
		try {
			cv::CascadeClassifier* ret = new cv::CascadeClassifier(cv::String(filename));
			return Ok<cv::CascadeClassifier*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::CascadeClassifier*>))
	}
	
	Result<bool> cv_CascadeClassifier_empty_const(const cv::CascadeClassifier* instance) {
		try {
			bool ret = instance->empty();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_CascadeClassifier_load_const_StringR(cv::CascadeClassifier* instance, const char* filename) {
		try {
			bool ret = instance->load(cv::String(filename));
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_CascadeClassifier_read_const_FileNodeR(cv::CascadeClassifier* instance, const cv::FileNode* node) {
		try {
			bool ret = instance->read(*node);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_CascadeClassifier_detectMultiScale_const__InputArrayR_vector_Rect_R_double_int_int_Size_Size(cv::CascadeClassifier* instance, const cv::_InputArray* image, std::vector<cv::Rect>* objects, double scaleFactor, int minNeighbors, int flags, cv::Size* minSize, cv::Size* maxSize) {
		try {
			instance->detectMultiScale(*image, *objects, scaleFactor, minNeighbors, flags, *minSize, *maxSize);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_CascadeClassifier_detectMultiScale_const__InputArrayR_vector_Rect_R_vector_int_R_double_int_int_Size_Size(cv::CascadeClassifier* instance, const cv::_InputArray* image, std::vector<cv::Rect>* objects, std::vector<int>* numDetections, double scaleFactor, int minNeighbors, int flags, cv::Size* minSize, cv::Size* maxSize) {
		try {
			instance->detectMultiScale(*image, *objects, *numDetections, scaleFactor, minNeighbors, flags, *minSize, *maxSize);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_CascadeClassifier_detectMultiScale_const__InputArrayR_vector_Rect_R_vector_int_R_vector_double_R_double_int_int_Size_Size_bool(cv::CascadeClassifier* instance, const cv::_InputArray* image, std::vector<cv::Rect>* objects, std::vector<int>* rejectLevels, std::vector<double>* levelWeights, double scaleFactor, int minNeighbors, int flags, cv::Size* minSize, cv::Size* maxSize, bool outputRejectLevels) {
		try {
			instance->detectMultiScale(*image, *objects, *rejectLevels, *levelWeights, scaleFactor, minNeighbors, flags, *minSize, *maxSize, outputRejectLevels);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_CascadeClassifier_isOldFormatCascade_const(const cv::CascadeClassifier* instance) {
		try {
			bool ret = instance->isOldFormatCascade();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<cv::Size> cv_CascadeClassifier_getOriginalWindowSize_const(const cv::CascadeClassifier* instance) {
		try {
			cv::Size ret = instance->getOriginalWindowSize();
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	Result<int> cv_CascadeClassifier_getFeatureType_const(const cv::CascadeClassifier* instance) {
		try {
			int ret = instance->getFeatureType();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<void*> cv_CascadeClassifier_getOldCascade(cv::CascadeClassifier* instance) {
		try {
			void* ret = instance->getOldCascade();
			return Ok<void*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result<bool> cv_CascadeClassifier_convert_const_StringR_const_StringR(const char* oldcascade, const char* newcascade) {
		try {
			bool ret = cv::CascadeClassifier::convert(cv::String(oldcascade), cv::String(newcascade));
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_CascadeClassifier_setMaskGenerator_const_Ptr_MaskGenerator_R(cv::CascadeClassifier* instance, const cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>* maskGenerator) {
		try {
			instance->setMaskGenerator(*maskGenerator);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>*> cv_CascadeClassifier_getMaskGenerator(cv::CascadeClassifier* instance) {
		try {
			cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator> ret = instance->getMaskGenerator();
			return Ok(new cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>*>))
	}
	
	void cv_DetectionBasedTracker_delete(cv::DetectionBasedTracker* instance) {
		delete instance;
	}
	Result<cv::DetectionBasedTracker*> cv_DetectionBasedTracker_DetectionBasedTracker_Ptr_IDetector__Ptr_IDetector__const_ParametersR(cv::Ptr<cv::DetectionBasedTracker::IDetector>* mainDetector, cv::Ptr<cv::DetectionBasedTracker::IDetector>* trackingDetector, const cv::DetectionBasedTracker::Parameters* params) {
		try {
			cv::DetectionBasedTracker* ret = new cv::DetectionBasedTracker(*mainDetector, *trackingDetector, *params);
			return Ok<cv::DetectionBasedTracker*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::DetectionBasedTracker*>))
	}
	
	Result<bool> cv_DetectionBasedTracker_run(cv::DetectionBasedTracker* instance) {
		try {
			bool ret = instance->run();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_DetectionBasedTracker_stop(cv::DetectionBasedTracker* instance) {
		try {
			instance->stop();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_DetectionBasedTracker_resetTracking(cv::DetectionBasedTracker* instance) {
		try {
			instance->resetTracking();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_DetectionBasedTracker_process_const_MatR(cv::DetectionBasedTracker* instance, const cv::Mat* imageGray) {
		try {
			instance->process(*imageGray);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_DetectionBasedTracker_setParameters_const_ParametersR(cv::DetectionBasedTracker* instance, const cv::DetectionBasedTracker::Parameters* params) {
		try {
			bool ret = instance->setParameters(*params);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<const cv::DetectionBasedTracker::Parameters*> cv_DetectionBasedTracker_getParameters_const(const cv::DetectionBasedTracker* instance) {
		try {
			const cv::DetectionBasedTracker::Parameters ret = instance->getParameters();
			return Ok(new const cv::DetectionBasedTracker::Parameters(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<const cv::DetectionBasedTracker::Parameters*>))
	}
	
	Result_void cv_DetectionBasedTracker_getObjects_const_vector_Rect_R(const cv::DetectionBasedTracker* instance, std::vector<cv::Rect>* result) {
		try {
			instance->getObjects(*result);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_DetectionBasedTracker_getObjects_const_vector_ExtObject_R(const cv::DetectionBasedTracker* instance, std::vector<cv::DetectionBasedTracker::ExtObject>* result) {
		try {
			instance->getObjects(*result);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_DetectionBasedTracker_addObject_const_RectR(cv::DetectionBasedTracker* instance, const cv::Rect* location) {
		try {
			int ret = instance->addObject(*location);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_DetectionBasedTracker_ExtObject_getPropId_const(const cv::DetectionBasedTracker::ExtObject* instance) {
		try {
			int ret = instance->id;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_DetectionBasedTracker_ExtObject_setPropId_int(cv::DetectionBasedTracker::ExtObject* instance, int val) {
		try {
			instance->id = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Rect> cv_DetectionBasedTracker_ExtObject_getPropLocation_const(const cv::DetectionBasedTracker::ExtObject* instance) {
		try {
			cv::Rect ret = instance->location;
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	Result_void cv_DetectionBasedTracker_ExtObject_setPropLocation_Rect(cv::DetectionBasedTracker::ExtObject* instance, cv::Rect* val) {
		try {
			instance->location = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::DetectionBasedTracker::ObjectStatus> cv_DetectionBasedTracker_ExtObject_getPropStatus_const(const cv::DetectionBasedTracker::ExtObject* instance) {
		try {
			cv::DetectionBasedTracker::ObjectStatus ret = instance->status;
			return Ok<cv::DetectionBasedTracker::ObjectStatus>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::DetectionBasedTracker::ObjectStatus>))
	}
	
	Result_void cv_DetectionBasedTracker_ExtObject_setPropStatus_ObjectStatus(cv::DetectionBasedTracker::ExtObject* instance, cv::DetectionBasedTracker::ObjectStatus val) {
		try {
			instance->status = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_DetectionBasedTracker_ExtObject_delete(cv::DetectionBasedTracker::ExtObject* instance) {
		delete instance;
	}
	Result<cv::DetectionBasedTracker::ExtObject*> cv_DetectionBasedTracker_ExtObject_ExtObject_int_Rect_ObjectStatus(int _id, cv::Rect* _location, cv::DetectionBasedTracker::ObjectStatus _status) {
		try {
			cv::DetectionBasedTracker::ExtObject* ret = new cv::DetectionBasedTracker::ExtObject(_id, *_location, _status);
			return Ok<cv::DetectionBasedTracker::ExtObject*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::DetectionBasedTracker::ExtObject*>))
	}
	
	Result_void cv_DetectionBasedTracker_IDetector_detect_const_MatR_vector_Rect_R(cv::DetectionBasedTracker::IDetector* instance, const cv::Mat* image, std::vector<cv::Rect>* objects) {
		try {
			instance->detect(*image, *objects);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_DetectionBasedTracker_IDetector_setMinObjectSize_const_SizeR(cv::DetectionBasedTracker::IDetector* instance, const cv::Size* min) {
		try {
			instance->setMinObjectSize(*min);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_DetectionBasedTracker_IDetector_setMaxObjectSize_const_SizeR(cv::DetectionBasedTracker::IDetector* instance, const cv::Size* max) {
		try {
			instance->setMaxObjectSize(*max);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Size> cv_DetectionBasedTracker_IDetector_getMinObjectSize_const(const cv::DetectionBasedTracker::IDetector* instance) {
		try {
			cv::Size ret = instance->getMinObjectSize();
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	Result<cv::Size> cv_DetectionBasedTracker_IDetector_getMaxObjectSize_const(const cv::DetectionBasedTracker::IDetector* instance) {
		try {
			cv::Size ret = instance->getMaxObjectSize();
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	Result<float> cv_DetectionBasedTracker_IDetector_getScaleFactor(cv::DetectionBasedTracker::IDetector* instance) {
		try {
			float ret = instance->getScaleFactor();
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_DetectionBasedTracker_IDetector_setScaleFactor_float(cv::DetectionBasedTracker::IDetector* instance, float value) {
		try {
			instance->setScaleFactor(value);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_DetectionBasedTracker_IDetector_getMinNeighbours(cv::DetectionBasedTracker::IDetector* instance) {
		try {
			int ret = instance->getMinNeighbours();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_DetectionBasedTracker_IDetector_setMinNeighbours_int(cv::DetectionBasedTracker::IDetector* instance, int value) {
		try {
			instance->setMinNeighbours(value);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_DetectionBasedTracker_Parameters_getPropMaxTrackLifetime_const(const cv::DetectionBasedTracker::Parameters* instance) {
		try {
			int ret = instance->maxTrackLifetime;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_DetectionBasedTracker_Parameters_setPropMaxTrackLifetime_int(cv::DetectionBasedTracker::Parameters* instance, int val) {
		try {
			instance->maxTrackLifetime = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_DetectionBasedTracker_Parameters_getPropMinDetectionPeriod_const(const cv::DetectionBasedTracker::Parameters* instance) {
		try {
			int ret = instance->minDetectionPeriod;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_DetectionBasedTracker_Parameters_setPropMinDetectionPeriod_int(cv::DetectionBasedTracker::Parameters* instance, int val) {
		try {
			instance->minDetectionPeriod = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_DetectionBasedTracker_Parameters_delete(cv::DetectionBasedTracker::Parameters* instance) {
		delete instance;
	}
	Result<cv::DetectionBasedTracker::Parameters*> cv_DetectionBasedTracker_Parameters_Parameters() {
		try {
			cv::DetectionBasedTracker::Parameters* ret = new cv::DetectionBasedTracker::Parameters();
			return Ok<cv::DetectionBasedTracker::Parameters*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::DetectionBasedTracker::Parameters*>))
	}
	
	Result<double> cv_DetectionROI_getPropScale_const(const cv::DetectionROI* instance) {
		try {
			double ret = instance->scale;
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_DetectionROI_setPropScale_double(cv::DetectionROI* instance, double val) {
		try {
			instance->scale = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<cv::Point>*> cv_DetectionROI_getPropLocations(cv::DetectionROI* instance) {
		try {
			std::vector<cv::Point> ret = instance->locations;
			return Ok(new std::vector<cv::Point>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::Point>*>))
	}
	
	Result_void cv_DetectionROI_setPropLocations_vector_Point_(cv::DetectionROI* instance, std::vector<cv::Point>* val) {
		try {
			instance->locations = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<double>*> cv_DetectionROI_getPropConfidences(cv::DetectionROI* instance) {
		try {
			std::vector<double> ret = instance->confidences;
			return Ok(new std::vector<double>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<double>*>))
	}
	
	Result_void cv_DetectionROI_setPropConfidences_vector_double_(cv::DetectionROI* instance, std::vector<double>* val) {
		try {
			instance->confidences = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_DetectionROI_delete(cv::DetectionROI* instance) {
		delete instance;
	}
	Result<cv::Size> cv_HOGDescriptor_getPropWinSize_const(const cv::HOGDescriptor* instance) {
		try {
			cv::Size ret = instance->winSize;
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	Result_void cv_HOGDescriptor_setPropWinSize_Size(cv::HOGDescriptor* instance, cv::Size* val) {
		try {
			instance->winSize = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Size> cv_HOGDescriptor_getPropBlockSize_const(const cv::HOGDescriptor* instance) {
		try {
			cv::Size ret = instance->blockSize;
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	Result_void cv_HOGDescriptor_setPropBlockSize_Size(cv::HOGDescriptor* instance, cv::Size* val) {
		try {
			instance->blockSize = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Size> cv_HOGDescriptor_getPropBlockStride_const(const cv::HOGDescriptor* instance) {
		try {
			cv::Size ret = instance->blockStride;
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	Result_void cv_HOGDescriptor_setPropBlockStride_Size(cv::HOGDescriptor* instance, cv::Size* val) {
		try {
			instance->blockStride = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Size> cv_HOGDescriptor_getPropCellSize_const(const cv::HOGDescriptor* instance) {
		try {
			cv::Size ret = instance->cellSize;
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	Result_void cv_HOGDescriptor_setPropCellSize_Size(cv::HOGDescriptor* instance, cv::Size* val) {
		try {
			instance->cellSize = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_HOGDescriptor_getPropNbins_const(const cv::HOGDescriptor* instance) {
		try {
			int ret = instance->nbins;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_HOGDescriptor_setPropNbins_int(cv::HOGDescriptor* instance, int val) {
		try {
			instance->nbins = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_HOGDescriptor_getPropDerivAperture_const(const cv::HOGDescriptor* instance) {
		try {
			int ret = instance->derivAperture;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_HOGDescriptor_setPropDerivAperture_int(cv::HOGDescriptor* instance, int val) {
		try {
			instance->derivAperture = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_HOGDescriptor_getPropWinSigma_const(const cv::HOGDescriptor* instance) {
		try {
			double ret = instance->winSigma;
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_HOGDescriptor_setPropWinSigma_double(cv::HOGDescriptor* instance, double val) {
		try {
			instance->winSigma = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_HOGDescriptor_getPropHistogramNormType_const(const cv::HOGDescriptor* instance) {
		try {
			int ret = instance->histogramNormType;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_HOGDescriptor_setPropHistogramNormType_int(cv::HOGDescriptor* instance, int val) {
		try {
			instance->histogramNormType = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_HOGDescriptor_getPropL2HysThreshold_const(const cv::HOGDescriptor* instance) {
		try {
			double ret = instance->L2HysThreshold;
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_HOGDescriptor_setPropL2HysThreshold_double(cv::HOGDescriptor* instance, double val) {
		try {
			instance->L2HysThreshold = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_HOGDescriptor_getPropGammaCorrection_const(const cv::HOGDescriptor* instance) {
		try {
			bool ret = instance->gammaCorrection;
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_HOGDescriptor_setPropGammaCorrection_bool(cv::HOGDescriptor* instance, bool val) {
		try {
			instance->gammaCorrection = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<float>*> cv_HOGDescriptor_getPropSvmDetector(cv::HOGDescriptor* instance) {
		try {
			std::vector<float> ret = instance->svmDetector;
			return Ok(new std::vector<float>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<float>*>))
	}
	
	Result_void cv_HOGDescriptor_setPropSvmDetector_vector_float_(cv::HOGDescriptor* instance, std::vector<float>* val) {
		try {
			instance->svmDetector = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::UMat*> cv_HOGDescriptor_getPropOclSvmDetector(cv::HOGDescriptor* instance) {
		try {
			cv::UMat ret = instance->oclSvmDetector;
			return Ok(new cv::UMat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	Result_void cv_HOGDescriptor_setPropOclSvmDetector_UMat(cv::HOGDescriptor* instance, cv::UMat* val) {
		try {
			instance->oclSvmDetector = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_HOGDescriptor_getPropFree_coef_const(const cv::HOGDescriptor* instance) {
		try {
			float ret = instance->free_coef;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_HOGDescriptor_setPropFree_coef_float(cv::HOGDescriptor* instance, float val) {
		try {
			instance->free_coef = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_HOGDescriptor_getPropNlevels_const(const cv::HOGDescriptor* instance) {
		try {
			int ret = instance->nlevels;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_HOGDescriptor_setPropNlevels_int(cv::HOGDescriptor* instance, int val) {
		try {
			instance->nlevels = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_HOGDescriptor_getPropSignedGradient_const(const cv::HOGDescriptor* instance) {
		try {
			bool ret = instance->signedGradient;
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_HOGDescriptor_setPropSignedGradient_bool(cv::HOGDescriptor* instance, bool val) {
		try {
			instance->signedGradient = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_HOGDescriptor_delete(cv::HOGDescriptor* instance) {
		delete instance;
	}
	Result<cv::HOGDescriptor*> cv_HOGDescriptor_HOGDescriptor() {
		try {
			cv::HOGDescriptor* ret = new cv::HOGDescriptor();
			return Ok<cv::HOGDescriptor*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::HOGDescriptor*>))
	}
	
	Result<cv::HOGDescriptor*> cv_HOGDescriptor_HOGDescriptor_Size_Size_Size_Size_int_int_double_int_double_bool_int_bool(cv::Size* _winSize, cv::Size* _blockSize, cv::Size* _blockStride, cv::Size* _cellSize, int _nbins, int _derivAperture, double _winSigma, int _histogramNormType, double _L2HysThreshold, bool _gammaCorrection, int _nlevels, bool _signedGradient) {
		try {
			cv::HOGDescriptor* ret = new cv::HOGDescriptor(*_winSize, *_blockSize, *_blockStride, *_cellSize, _nbins, _derivAperture, _winSigma, _histogramNormType, _L2HysThreshold, _gammaCorrection, _nlevels, _signedGradient);
			return Ok<cv::HOGDescriptor*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::HOGDescriptor*>))
	}
	
	Result<cv::HOGDescriptor*> cv_HOGDescriptor_HOGDescriptor_const_StringR(const char* filename) {
		try {
			cv::HOGDescriptor* ret = new cv::HOGDescriptor(cv::String(filename));
			return Ok<cv::HOGDescriptor*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::HOGDescriptor*>))
	}
	
	Result<cv::HOGDescriptor*> cv_HOGDescriptor_HOGDescriptor_const_HOGDescriptorR(const cv::HOGDescriptor* d) {
		try {
			cv::HOGDescriptor* ret = new cv::HOGDescriptor(*d);
			return Ok<cv::HOGDescriptor*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::HOGDescriptor*>))
	}
	
	Result<size_t> cv_HOGDescriptor_getDescriptorSize_const(const cv::HOGDescriptor* instance) {
		try {
			size_t ret = instance->getDescriptorSize();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	Result<bool> cv_HOGDescriptor_checkDetectorSize_const(const cv::HOGDescriptor* instance) {
		try {
			bool ret = instance->checkDetectorSize();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<double> cv_HOGDescriptor_getWinSigma_const(const cv::HOGDescriptor* instance) {
		try {
			double ret = instance->getWinSigma();
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_HOGDescriptor_setSVMDetector_const__InputArrayR(cv::HOGDescriptor* instance, const cv::_InputArray* _svmdetector) {
		try {
			instance->setSVMDetector(*_svmdetector);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_HOGDescriptor_read_FileNodeR(cv::HOGDescriptor* instance, cv::FileNode* fn) {
		try {
			bool ret = instance->read(*fn);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_HOGDescriptor_write_const_FileStorageR_const_StringR(const cv::HOGDescriptor* instance, cv::FileStorage* fs, const char* objname) {
		try {
			instance->write(*fs, cv::String(objname));
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_HOGDescriptor_load_const_StringR_const_StringR(cv::HOGDescriptor* instance, const char* filename, const char* objname) {
		try {
			bool ret = instance->load(cv::String(filename), cv::String(objname));
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_HOGDescriptor_save_const_const_StringR_const_StringR(const cv::HOGDescriptor* instance, const char* filename, const char* objname) {
		try {
			instance->save(cv::String(filename), cv::String(objname));
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_HOGDescriptor_copyTo_const_HOGDescriptorR(const cv::HOGDescriptor* instance, cv::HOGDescriptor* c) {
		try {
			instance->copyTo(*c);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_HOGDescriptor_compute_const_const__InputArrayR_vector_float_R_Size_Size_const_vector_Point_R(const cv::HOGDescriptor* instance, const cv::_InputArray* img, std::vector<float>* descriptors, cv::Size* winStride, cv::Size* padding, const std::vector<cv::Point>* locations) {
		try {
			instance->compute(*img, *descriptors, *winStride, *padding, *locations);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_HOGDescriptor_detect_const_const_MatR_vector_Point_R_vector_double_R_double_Size_Size_const_vector_Point_R(const cv::HOGDescriptor* instance, const cv::Mat* img, std::vector<cv::Point>* foundLocations, std::vector<double>* weights, double hitThreshold, cv::Size* winStride, cv::Size* padding, const std::vector<cv::Point>* searchLocations) {
		try {
			instance->detect(*img, *foundLocations, *weights, hitThreshold, *winStride, *padding, *searchLocations);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_HOGDescriptor_detect_const_const_MatR_vector_Point_R_double_Size_Size_const_vector_Point_R(const cv::HOGDescriptor* instance, const cv::Mat* img, std::vector<cv::Point>* foundLocations, double hitThreshold, cv::Size* winStride, cv::Size* padding, const std::vector<cv::Point>* searchLocations) {
		try {
			instance->detect(*img, *foundLocations, hitThreshold, *winStride, *padding, *searchLocations);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_HOGDescriptor_detectMultiScale_const_const__InputArrayR_vector_Rect_R_vector_double_R_double_Size_Size_double_double_bool(const cv::HOGDescriptor* instance, const cv::_InputArray* img, std::vector<cv::Rect>* foundLocations, std::vector<double>* foundWeights, double hitThreshold, cv::Size* winStride, cv::Size* padding, double scale, double finalThreshold, bool useMeanshiftGrouping) {
		try {
			instance->detectMultiScale(*img, *foundLocations, *foundWeights, hitThreshold, *winStride, *padding, scale, finalThreshold, useMeanshiftGrouping);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_HOGDescriptor_detectMultiScale_const_const__InputArrayR_vector_Rect_R_double_Size_Size_double_double_bool(const cv::HOGDescriptor* instance, const cv::_InputArray* img, std::vector<cv::Rect>* foundLocations, double hitThreshold, cv::Size* winStride, cv::Size* padding, double scale, double finalThreshold, bool useMeanshiftGrouping) {
		try {
			instance->detectMultiScale(*img, *foundLocations, hitThreshold, *winStride, *padding, scale, finalThreshold, useMeanshiftGrouping);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_HOGDescriptor_computeGradient_const_const_MatR_MatR_MatR_Size_Size(const cv::HOGDescriptor* instance, const cv::Mat* img, cv::Mat* grad, cv::Mat* angleOfs, cv::Size* paddingTL, cv::Size* paddingBR) {
		try {
			instance->computeGradient(*img, *grad, *angleOfs, *paddingTL, *paddingBR);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<float>*> cv_HOGDescriptor_getDefaultPeopleDetector() {
		try {
			std::vector<float> ret = cv::HOGDescriptor::getDefaultPeopleDetector();
			return Ok(new std::vector<float>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<float>*>))
	}
	
	Result<std::vector<float>*> cv_HOGDescriptor_getDaimlerPeopleDetector() {
		try {
			std::vector<float> ret = cv::HOGDescriptor::getDaimlerPeopleDetector();
			return Ok(new std::vector<float>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<float>*>))
	}
	
	Result_void cv_HOGDescriptor_detectROI_const_const_MatR_const_vector_Point_R_vector_Point_R_vector_double_R_double_Size_Size(const cv::HOGDescriptor* instance, const cv::Mat* img, const std::vector<cv::Point>* locations, std::vector<cv::Point>* foundLocations, std::vector<double>* confidences, double hitThreshold, cv::Size* winStride, cv::Size* padding) {
		try {
			instance->detectROI(*img, *locations, *foundLocations, *confidences, hitThreshold, *winStride, *padding);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_HOGDescriptor_detectMultiScaleROI_const_const_MatR_vector_Rect_R_vector_DetectionROI_R_double_int(const cv::HOGDescriptor* instance, const cv::Mat* img, std::vector<cv::Rect>* foundLocations, std::vector<cv::DetectionROI>* locations, double hitThreshold, int groupThreshold) {
		try {
			instance->detectMultiScaleROI(*img, *foundLocations, *locations, hitThreshold, groupThreshold);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_HOGDescriptor_readALTModel_String(cv::HOGDescriptor* instance, char* modelfile) {
		try {
			instance->readALTModel(cv::String(modelfile));
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_HOGDescriptor_groupRectangles_const_vector_Rect_R_vector_double_R_int_double(const cv::HOGDescriptor* instance, std::vector<cv::Rect>* rectList, std::vector<double>* weights, int groupThreshold, double eps) {
		try {
			instance->groupRectangles(*rectList, *weights, groupThreshold, eps);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_QRCodeDetector_delete(cv::QRCodeDetector* instance) {
		delete instance;
	}
	Result<cv::QRCodeDetector*> cv_QRCodeDetector_QRCodeDetector() {
		try {
			cv::QRCodeDetector* ret = new cv::QRCodeDetector();
			return Ok<cv::QRCodeDetector*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::QRCodeDetector*>))
	}
	
	Result_void cv_QRCodeDetector_setEpsX_double(cv::QRCodeDetector* instance, double epsX) {
		try {
			instance->setEpsX(epsX);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_QRCodeDetector_setEpsY_double(cv::QRCodeDetector* instance, double epsY) {
		try {
			instance->setEpsY(epsY);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_QRCodeDetector_detect_const_const__InputArrayR_const__OutputArrayR(const cv::QRCodeDetector* instance, const cv::_InputArray* img, const cv::_OutputArray* points) {
		try {
			bool ret = instance->detect(*img, *points);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<void*> cv_QRCodeDetector_decode_const__InputArrayR_const__InputArrayR_const__OutputArrayR(cv::QRCodeDetector* instance, const cv::_InputArray* img, const cv::_InputArray* points, const cv::_OutputArray* straight_qrcode) {
		try {
			cv::String ret = instance->decode(*img, *points, *straight_qrcode);
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result<void*> cv_QRCodeDetector_detectAndDecode_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::QRCodeDetector* instance, const cv::_InputArray* img, const cv::_OutputArray* points, const cv::_OutputArray* straight_qrcode) {
		try {
			cv::String ret = instance->detectAndDecode(*img, *points, *straight_qrcode);
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result<bool> cv_QRCodeDetector_detectMulti_const_const__InputArrayR_const__OutputArrayR(const cv::QRCodeDetector* instance, const cv::_InputArray* img, const cv::_OutputArray* points) {
		try {
			bool ret = instance->detectMulti(*img, *points);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_QRCodeDetector_decodeMulti_const_const__InputArrayR_const__InputArrayR_vector_String_R_const__OutputArrayR(const cv::QRCodeDetector* instance, const cv::_InputArray* img, const cv::_InputArray* points, std::vector<cv::String>* decoded_info, const cv::_OutputArray* straight_qrcode) {
		try {
			bool ret = instance->decodeMulti(*img, *points, *decoded_info, *straight_qrcode);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_QRCodeDetector_detectAndDecodeMulti_const_const__InputArrayR_vector_String_R_const__OutputArrayR_const__OutputArrayR(const cv::QRCodeDetector* instance, const cv::_InputArray* img, std::vector<cv::String>* decoded_info, const cv::_OutputArray* points, const cv::_OutputArray* straight_qrcode) {
		try {
			bool ret = instance->detectAndDecodeMulti(*img, *decoded_info, *points, *straight_qrcode);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_QRCodeDetector_decodeMulti_const_const__InputArrayR_const__InputArrayR_vector_string_R_const__OutputArrayR(const cv::QRCodeDetector* instance, const cv::_InputArray* img, const cv::_InputArray* points, std::vector<std::string>* decoded_info, const cv::_OutputArray* straight_qrcode) {
		try {
			bool ret = instance->decodeMulti(*img, *points, *decoded_info, *straight_qrcode);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_QRCodeDetector_detectAndDecodeMulti_const_const__InputArrayR_vector_string_R_const__OutputArrayR_const__OutputArrayR(const cv::QRCodeDetector* instance, const cv::_InputArray* img, std::vector<std::string>* decoded_info, const cv::_OutputArray* points, const cv::_OutputArray* straight_qrcode) {
		try {
			bool ret = instance->detectAndDecodeMulti(*img, *decoded_info, *points, *straight_qrcode);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<double> cv_SimilarRects_getPropEps_const(const cv::SimilarRects* instance) {
		try {
			double ret = instance->eps;
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_SimilarRects_setPropEps_double(cv::SimilarRects* instance, double val) {
		try {
			instance->eps = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_SimilarRects_delete(cv::SimilarRects* instance) {
		delete instance;
	}
	Result<cv::SimilarRects*> cv_SimilarRects_SimilarRects_double(double _eps) {
		try {
			cv::SimilarRects* ret = new cv::SimilarRects(_eps);
			return Ok<cv::SimilarRects*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::SimilarRects*>))
	}
	
}
