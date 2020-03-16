#include "common.hpp"
#include <opencv2/objdetect.hpp>
#include "objdetect_types.hpp"

extern "C" {
	Result<cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>*> cv_createFaceDetectionMaskGenerator() {
		try {
			cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator> ret = cv::createFaceDetectionMaskGenerator();
			return Ok(new cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>*>)
	}
	
	Result_void cv_groupRectangles_meanshift_vector_Rect_X_vector_double_X_vector_double_X_double_Size(std::vector<cv::Rect>* rectList, std::vector<double>* foundWeights, std::vector<double>* foundScales, double detectThreshold, const cv::Size* winDetSize) {
		try {
			cv::groupRectangles_meanshift(*rectList, *foundWeights, *foundScales, detectThreshold, *winDetSize);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_groupRectangles_vector_Rect_X_int_double(std::vector<cv::Rect>* rectList, int groupThreshold, double eps) {
		try {
			cv::groupRectangles(*rectList, groupThreshold, eps);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_groupRectangles_vector_Rect_X_int_double_vector_int_X_vector_double_X(std::vector<cv::Rect>* rectList, int groupThreshold, double eps, std::vector<int>* weights, std::vector<double>* levelWeights) {
		try {
			cv::groupRectangles(*rectList, groupThreshold, eps, weights, levelWeights);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_groupRectangles_vector_Rect_X_vector_int_X_int_double(std::vector<cv::Rect>* rectList, std::vector<int>* weights, int groupThreshold, double eps) {
		try {
			cv::groupRectangles(*rectList, *weights, groupThreshold, eps);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_groupRectangles_vector_Rect_X_vector_int_X_vector_double_X_int_double(std::vector<cv::Rect>* rectList, std::vector<int>* rejectLevels, std::vector<double>* levelWeights, int groupThreshold, double eps) {
		try {
			cv::groupRectangles(*rectList, *rejectLevels, *levelWeights, groupThreshold, eps);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_BaseCascadeClassifier_empty_const(const cv::BaseCascadeClassifier* instance) {
		try {
			bool ret = instance->empty();
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_BaseCascadeClassifier_load_const_StringX(cv::BaseCascadeClassifier* instance, const char* filename) {
		try {
			bool ret = instance->load(std::string(filename));
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_BaseCascadeClassifier_detectMultiScale_const__InputArrayX_vector_Rect_X_double_int_int_Size_Size(cv::BaseCascadeClassifier* instance, const cv::_InputArray* image, std::vector<cv::Rect>* objects, double scaleFactor, int minNeighbors, int flags, const cv::Size* minSize, const cv::Size* maxSize) {
		try {
			instance->detectMultiScale(*image, *objects, scaleFactor, minNeighbors, flags, *minSize, *maxSize);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_BaseCascadeClassifier_detectMultiScale_const__InputArrayX_vector_Rect_X_vector_int_X_double_int_int_Size_Size(cv::BaseCascadeClassifier* instance, const cv::_InputArray* image, std::vector<cv::Rect>* objects, std::vector<int>* numDetections, double scaleFactor, int minNeighbors, int flags, const cv::Size* minSize, const cv::Size* maxSize) {
		try {
			instance->detectMultiScale(*image, *objects, *numDetections, scaleFactor, minNeighbors, flags, *minSize, *maxSize);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_BaseCascadeClassifier_detectMultiScale_const__InputArrayX_vector_Rect_X_vector_int_X_vector_double_X_double_int_int_Size_Size_bool(cv::BaseCascadeClassifier* instance, const cv::_InputArray* image, std::vector<cv::Rect>* objects, std::vector<int>* rejectLevels, std::vector<double>* levelWeights, double scaleFactor, int minNeighbors, int flags, const cv::Size* minSize, const cv::Size* maxSize, bool outputRejectLevels) {
		try {
			instance->detectMultiScale(*image, *objects, *rejectLevels, *levelWeights, scaleFactor, minNeighbors, flags, *minSize, *maxSize, outputRejectLevels);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_BaseCascadeClassifier_isOldFormatCascade_const(const cv::BaseCascadeClassifier* instance) {
		try {
			bool ret = instance->isOldFormatCascade();
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<cv::Size> cv_BaseCascadeClassifier_getOriginalWindowSize_const(const cv::BaseCascadeClassifier* instance) {
		try {
			cv::Size ret = instance->getOriginalWindowSize();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result<int> cv_BaseCascadeClassifier_getFeatureType_const(const cv::BaseCascadeClassifier* instance) {
		try {
			int ret = instance->getFeatureType();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<void*> cv_BaseCascadeClassifier_getOldCascade(cv::BaseCascadeClassifier* instance) {
		try {
			void* ret = instance->getOldCascade();
			return Ok(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_BaseCascadeClassifier_setMaskGenerator_const_Ptr_MaskGenerator_X(cv::BaseCascadeClassifier* instance, const cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>* maskGenerator) {
		try {
			instance->setMaskGenerator(*maskGenerator);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>*> cv_BaseCascadeClassifier_getMaskGenerator(cv::BaseCascadeClassifier* instance) {
		try {
			cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator> ret = instance->getMaskGenerator();
			return Ok(new cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>*>)
	}
	
	Result<cv::Mat*> cv_BaseCascadeClassifier_MaskGenerator_generateMask_const_MatX(cv::BaseCascadeClassifier::MaskGenerator* instance, const cv::Mat* src) {
		try {
			cv::Mat ret = instance->generateMask(*src);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result_void cv_BaseCascadeClassifier_MaskGenerator_initializeMask_const_MatX(cv::BaseCascadeClassifier::MaskGenerator* instance, const cv::Mat* unnamed) {
		try {
			instance->initializeMask(*unnamed);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Ptr<cv::BaseCascadeClassifier>*> cv_CascadeClassifier_cc(cv::CascadeClassifier* instance) {
		try {
			cv::Ptr<cv::BaseCascadeClassifier> ret = instance->cc;
			return Ok(new cv::Ptr<cv::BaseCascadeClassifier>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::BaseCascadeClassifier>*>)
	}
	
	Result_void cv_CascadeClassifier_setCc_Ptr_BaseCascadeClassifier_(cv::CascadeClassifier* instance, cv::Ptr<cv::BaseCascadeClassifier>* val) {
		try {
			instance->cc = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_CascadeClassifier_delete(cv::CascadeClassifier* instance) {
		delete instance;
	}
	Result<cv::CascadeClassifier*> cv_CascadeClassifier_CascadeClassifier() {
		try {
			cv::CascadeClassifier* ret = new cv::CascadeClassifier();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::CascadeClassifier*>)
	}
	
	Result<cv::CascadeClassifier*> cv_CascadeClassifier_CascadeClassifier_const_StringX(const char* filename) {
		try {
			cv::CascadeClassifier* ret = new cv::CascadeClassifier(std::string(filename));
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::CascadeClassifier*>)
	}
	
	Result<bool> cv_CascadeClassifier_empty_const(const cv::CascadeClassifier* instance) {
		try {
			bool ret = instance->empty();
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_CascadeClassifier_load_const_StringX(cv::CascadeClassifier* instance, const char* filename) {
		try {
			bool ret = instance->load(std::string(filename));
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_CascadeClassifier_read_const_FileNodeX(cv::CascadeClassifier* instance, const cv::FileNode* node) {
		try {
			bool ret = instance->read(*node);
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_CascadeClassifier_detectMultiScale_const__InputArrayX_vector_Rect_X_double_int_int_Size_Size(cv::CascadeClassifier* instance, const cv::_InputArray* image, std::vector<cv::Rect>* objects, double scaleFactor, int minNeighbors, int flags, const cv::Size* minSize, const cv::Size* maxSize) {
		try {
			instance->detectMultiScale(*image, *objects, scaleFactor, minNeighbors, flags, *minSize, *maxSize);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_CascadeClassifier_detectMultiScale_const__InputArrayX_vector_Rect_X_vector_int_X_double_int_int_Size_Size(cv::CascadeClassifier* instance, const cv::_InputArray* image, std::vector<cv::Rect>* objects, std::vector<int>* numDetections, double scaleFactor, int minNeighbors, int flags, const cv::Size* minSize, const cv::Size* maxSize) {
		try {
			instance->detectMultiScale(*image, *objects, *numDetections, scaleFactor, minNeighbors, flags, *minSize, *maxSize);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_CascadeClassifier_detectMultiScale_const__InputArrayX_vector_Rect_X_vector_int_X_vector_double_X_double_int_int_Size_Size_bool(cv::CascadeClassifier* instance, const cv::_InputArray* image, std::vector<cv::Rect>* objects, std::vector<int>* rejectLevels, std::vector<double>* levelWeights, double scaleFactor, int minNeighbors, int flags, const cv::Size* minSize, const cv::Size* maxSize, bool outputRejectLevels) {
		try {
			instance->detectMultiScale(*image, *objects, *rejectLevels, *levelWeights, scaleFactor, minNeighbors, flags, *minSize, *maxSize, outputRejectLevels);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_CascadeClassifier_isOldFormatCascade_const(const cv::CascadeClassifier* instance) {
		try {
			bool ret = instance->isOldFormatCascade();
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<cv::Size> cv_CascadeClassifier_getOriginalWindowSize_const(const cv::CascadeClassifier* instance) {
		try {
			cv::Size ret = instance->getOriginalWindowSize();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result<int> cv_CascadeClassifier_getFeatureType_const(const cv::CascadeClassifier* instance) {
		try {
			int ret = instance->getFeatureType();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<void*> cv_CascadeClassifier_getOldCascade(cv::CascadeClassifier* instance) {
		try {
			void* ret = instance->getOldCascade();
			return Ok(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<bool> cv_CascadeClassifier_convert_const_StringX_const_StringX(const char* oldcascade, const char* newcascade) {
		try {
			bool ret = cv::CascadeClassifier::convert(std::string(oldcascade), std::string(newcascade));
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_CascadeClassifier_setMaskGenerator_const_Ptr_MaskGenerator_X(cv::CascadeClassifier* instance, const cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>* maskGenerator) {
		try {
			instance->setMaskGenerator(*maskGenerator);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>*> cv_CascadeClassifier_getMaskGenerator(cv::CascadeClassifier* instance) {
		try {
			cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator> ret = instance->getMaskGenerator();
			return Ok(new cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>*>)
	}
	
	void cv_DetectionBasedTracker_delete(cv::DetectionBasedTracker* instance) {
		delete instance;
	}
	Result<cv::DetectionBasedTracker*> cv_DetectionBasedTracker_DetectionBasedTracker_Ptr_IDetector__Ptr_IDetector__const_ParametersX(cv::Ptr<cv::DetectionBasedTracker::IDetector>* mainDetector, cv::Ptr<cv::DetectionBasedTracker::IDetector>* trackingDetector, const cv::DetectionBasedTracker::Parameters* params) {
		try {
			cv::DetectionBasedTracker* ret = new cv::DetectionBasedTracker(*mainDetector, *trackingDetector, *params);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::DetectionBasedTracker*>)
	}
	
	Result<bool> cv_DetectionBasedTracker_run(cv::DetectionBasedTracker* instance) {
		try {
			bool ret = instance->run();
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_DetectionBasedTracker_stop(cv::DetectionBasedTracker* instance) {
		try {
			instance->stop();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_DetectionBasedTracker_resetTracking(cv::DetectionBasedTracker* instance) {
		try {
			instance->resetTracking();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_DetectionBasedTracker_process_const_MatX(cv::DetectionBasedTracker* instance, const cv::Mat* imageGray) {
		try {
			instance->process(*imageGray);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_DetectionBasedTracker_setParameters_const_ParametersX(cv::DetectionBasedTracker* instance, const cv::DetectionBasedTracker::Parameters* params) {
		try {
			bool ret = instance->setParameters(*params);
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<cv::DetectionBasedTracker::Parameters*> cv_DetectionBasedTracker_getParameters_const(const cv::DetectionBasedTracker* instance) {
		try {
			cv::DetectionBasedTracker::Parameters ret = instance->getParameters();
			return Ok(new cv::DetectionBasedTracker::Parameters(ret));
		} OCVRS_CATCH(Result<cv::DetectionBasedTracker::Parameters*>)
	}
	
	Result_void cv_DetectionBasedTracker_getObjects_const_vector_Rect_X(const cv::DetectionBasedTracker* instance, std::vector<cv::Rect>* result) {
		try {
			instance->getObjects(*result);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_DetectionBasedTracker_getObjects_const_vector_ExtObject_X(const cv::DetectionBasedTracker* instance, std::vector<cv::DetectionBasedTracker::ExtObject>* result) {
		try {
			instance->getObjects(*result);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_DetectionBasedTracker_addObject_const_RectX(cv::DetectionBasedTracker* instance, const cv::Rect* location) {
		try {
			int ret = instance->addObject(*location);
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_DetectionBasedTracker_ExtObject_id_const(const cv::DetectionBasedTracker::ExtObject* instance) {
		try {
			int ret = instance->id;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_DetectionBasedTracker_ExtObject_setId_int(cv::DetectionBasedTracker::ExtObject* instance, int val) {
		try {
			instance->id = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Rect> cv_DetectionBasedTracker_ExtObject_location_const(const cv::DetectionBasedTracker::ExtObject* instance) {
		try {
			cv::Rect ret = instance->location;
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Rect>)
	}
	
	Result_void cv_DetectionBasedTracker_ExtObject_setLocation_Rect(cv::DetectionBasedTracker::ExtObject* instance, const cv::Rect* val) {
		try {
			instance->location = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::DetectionBasedTracker::ObjectStatus> cv_DetectionBasedTracker_ExtObject_status_const(const cv::DetectionBasedTracker::ExtObject* instance) {
		try {
			cv::DetectionBasedTracker::ObjectStatus ret = instance->status;
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::DetectionBasedTracker::ObjectStatus>)
	}
	
	Result_void cv_DetectionBasedTracker_ExtObject_setStatus_ObjectStatus(cv::DetectionBasedTracker::ExtObject* instance, cv::DetectionBasedTracker::ObjectStatus val) {
		try {
			instance->status = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_DetectionBasedTracker_ExtObject_delete(cv::DetectionBasedTracker::ExtObject* instance) {
		delete instance;
	}
	Result<cv::DetectionBasedTracker::ExtObject*> cv_DetectionBasedTracker_ExtObject_ExtObject_int_Rect_ObjectStatus(int _id, const cv::Rect* _location, cv::DetectionBasedTracker::ObjectStatus _status) {
		try {
			cv::DetectionBasedTracker::ExtObject* ret = new cv::DetectionBasedTracker::ExtObject(_id, *_location, _status);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::DetectionBasedTracker::ExtObject*>)
	}
	
	Result_void cv_DetectionBasedTracker_IDetector_detect_const_MatX_vector_Rect_X(cv::DetectionBasedTracker::IDetector* instance, const cv::Mat* image, std::vector<cv::Rect>* objects) {
		try {
			instance->detect(*image, *objects);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_DetectionBasedTracker_IDetector_setMinObjectSize_const_SizeX(cv::DetectionBasedTracker::IDetector* instance, const cv::Size* min) {
		try {
			instance->setMinObjectSize(*min);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_DetectionBasedTracker_IDetector_setMaxObjectSize_const_SizeX(cv::DetectionBasedTracker::IDetector* instance, const cv::Size* max) {
		try {
			instance->setMaxObjectSize(*max);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Size> cv_DetectionBasedTracker_IDetector_getMinObjectSize_const(const cv::DetectionBasedTracker::IDetector* instance) {
		try {
			cv::Size ret = instance->getMinObjectSize();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result<cv::Size> cv_DetectionBasedTracker_IDetector_getMaxObjectSize_const(const cv::DetectionBasedTracker::IDetector* instance) {
		try {
			cv::Size ret = instance->getMaxObjectSize();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result<float> cv_DetectionBasedTracker_IDetector_getScaleFactor(cv::DetectionBasedTracker::IDetector* instance) {
		try {
			float ret = instance->getScaleFactor();
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_DetectionBasedTracker_IDetector_setScaleFactor_float(cv::DetectionBasedTracker::IDetector* instance, float value) {
		try {
			instance->setScaleFactor(value);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_DetectionBasedTracker_IDetector_getMinNeighbours(cv::DetectionBasedTracker::IDetector* instance) {
		try {
			int ret = instance->getMinNeighbours();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_DetectionBasedTracker_IDetector_setMinNeighbours_int(cv::DetectionBasedTracker::IDetector* instance, int value) {
		try {
			instance->setMinNeighbours(value);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_DetectionBasedTracker_Parameters_maxTrackLifetime_const(const cv::DetectionBasedTracker::Parameters* instance) {
		try {
			int ret = instance->maxTrackLifetime;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_DetectionBasedTracker_Parameters_setMaxTrackLifetime_int(cv::DetectionBasedTracker::Parameters* instance, int val) {
		try {
			instance->maxTrackLifetime = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_DetectionBasedTracker_Parameters_minDetectionPeriod_const(const cv::DetectionBasedTracker::Parameters* instance) {
		try {
			int ret = instance->minDetectionPeriod;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_DetectionBasedTracker_Parameters_setMinDetectionPeriod_int(cv::DetectionBasedTracker::Parameters* instance, int val) {
		try {
			instance->minDetectionPeriod = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_DetectionBasedTracker_Parameters_delete(cv::DetectionBasedTracker::Parameters* instance) {
		delete instance;
	}
	Result<cv::DetectionBasedTracker::Parameters*> cv_DetectionBasedTracker_Parameters_Parameters() {
		try {
			cv::DetectionBasedTracker::Parameters* ret = new cv::DetectionBasedTracker::Parameters();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::DetectionBasedTracker::Parameters*>)
	}
	
	Result<double> cv_DetectionROI_scale_const(const cv::DetectionROI* instance) {
		try {
			double ret = instance->scale;
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_DetectionROI_setScale_double(cv::DetectionROI* instance, double val) {
		try {
			instance->scale = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<std::vector<cv::Point>*> cv_DetectionROI_locations(cv::DetectionROI* instance) {
		try {
			std::vector<cv::Point> ret = instance->locations;
			return Ok(new std::vector<cv::Point>(ret));
		} OCVRS_CATCH(Result<std::vector<cv::Point>*>)
	}
	
	Result_void cv_DetectionROI_setLocations_vector_Point_(cv::DetectionROI* instance, std::vector<cv::Point>* val) {
		try {
			instance->locations = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<std::vector<double>*> cv_DetectionROI_confidences(cv::DetectionROI* instance) {
		try {
			std::vector<double> ret = instance->confidences;
			return Ok(new std::vector<double>(ret));
		} OCVRS_CATCH(Result<std::vector<double>*>)
	}
	
	Result_void cv_DetectionROI_setConfidences_vector_double_(cv::DetectionROI* instance, std::vector<double>* val) {
		try {
			instance->confidences = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_DetectionROI_delete(cv::DetectionROI* instance) {
		delete instance;
	}
	Result<cv::Size> cv_HOGDescriptor_winSize_const(const cv::HOGDescriptor* instance) {
		try {
			cv::Size ret = instance->winSize;
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_HOGDescriptor_setWinSize_Size(cv::HOGDescriptor* instance, const cv::Size* val) {
		try {
			instance->winSize = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Size> cv_HOGDescriptor_blockSize_const(const cv::HOGDescriptor* instance) {
		try {
			cv::Size ret = instance->blockSize;
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_HOGDescriptor_setBlockSize_Size(cv::HOGDescriptor* instance, const cv::Size* val) {
		try {
			instance->blockSize = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Size> cv_HOGDescriptor_blockStride_const(const cv::HOGDescriptor* instance) {
		try {
			cv::Size ret = instance->blockStride;
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_HOGDescriptor_setBlockStride_Size(cv::HOGDescriptor* instance, const cv::Size* val) {
		try {
			instance->blockStride = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Size> cv_HOGDescriptor_cellSize_const(const cv::HOGDescriptor* instance) {
		try {
			cv::Size ret = instance->cellSize;
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_HOGDescriptor_setCellSize_Size(cv::HOGDescriptor* instance, const cv::Size* val) {
		try {
			instance->cellSize = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_HOGDescriptor_nbins_const(const cv::HOGDescriptor* instance) {
		try {
			int ret = instance->nbins;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_HOGDescriptor_setNbins_int(cv::HOGDescriptor* instance, int val) {
		try {
			instance->nbins = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_HOGDescriptor_derivAperture_const(const cv::HOGDescriptor* instance) {
		try {
			int ret = instance->derivAperture;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_HOGDescriptor_setDerivAperture_int(cv::HOGDescriptor* instance, int val) {
		try {
			instance->derivAperture = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_HOGDescriptor_winSigma_const(const cv::HOGDescriptor* instance) {
		try {
			double ret = instance->winSigma;
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_HOGDescriptor_setWinSigma_double(cv::HOGDescriptor* instance, double val) {
		try {
			instance->winSigma = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::HOGDescriptor::HistogramNormType> cv_HOGDescriptor_histogramNormType_const(const cv::HOGDescriptor* instance) {
		try {
			cv::HOGDescriptor::HistogramNormType ret = instance->histogramNormType;
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::HOGDescriptor::HistogramNormType>)
	}
	
	Result_void cv_HOGDescriptor_setHistogramNormType_HistogramNormType(cv::HOGDescriptor* instance, cv::HOGDescriptor::HistogramNormType val) {
		try {
			instance->histogramNormType = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_HOGDescriptor_L2HysThreshold_const(const cv::HOGDescriptor* instance) {
		try {
			double ret = instance->L2HysThreshold;
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_HOGDescriptor_setL2HysThreshold_double(cv::HOGDescriptor* instance, double val) {
		try {
			instance->L2HysThreshold = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_HOGDescriptor_gammaCorrection_const(const cv::HOGDescriptor* instance) {
		try {
			bool ret = instance->gammaCorrection;
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_HOGDescriptor_setGammaCorrection_bool(cv::HOGDescriptor* instance, bool val) {
		try {
			instance->gammaCorrection = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<std::vector<float>*> cv_HOGDescriptor_svmDetector(cv::HOGDescriptor* instance) {
		try {
			std::vector<float> ret = instance->svmDetector;
			return Ok(new std::vector<float>(ret));
		} OCVRS_CATCH(Result<std::vector<float>*>)
	}
	
	Result_void cv_HOGDescriptor_setSvmDetector_vector_float_(cv::HOGDescriptor* instance, std::vector<float>* val) {
		try {
			instance->svmDetector = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::UMat*> cv_HOGDescriptor_oclSvmDetector(cv::HOGDescriptor* instance) {
		try {
			cv::UMat ret = instance->oclSvmDetector;
			return Ok(new cv::UMat(ret));
		} OCVRS_CATCH(Result<cv::UMat*>)
	}
	
	Result_void cv_HOGDescriptor_setOclSvmDetector_UMat(cv::HOGDescriptor* instance, cv::UMat* val) {
		try {
			instance->oclSvmDetector = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_HOGDescriptor_free_coef_const(const cv::HOGDescriptor* instance) {
		try {
			float ret = instance->free_coef;
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_HOGDescriptor_setFree_coef_float(cv::HOGDescriptor* instance, float val) {
		try {
			instance->free_coef = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_HOGDescriptor_nlevels_const(const cv::HOGDescriptor* instance) {
		try {
			int ret = instance->nlevels;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_HOGDescriptor_setNlevels_int(cv::HOGDescriptor* instance, int val) {
		try {
			instance->nlevels = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_HOGDescriptor_signedGradient_const(const cv::HOGDescriptor* instance) {
		try {
			bool ret = instance->signedGradient;
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_HOGDescriptor_setSignedGradient_bool(cv::HOGDescriptor* instance, bool val) {
		try {
			instance->signedGradient = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_HOGDescriptor_delete(cv::HOGDescriptor* instance) {
		delete instance;
	}
	Result<cv::HOGDescriptor*> cv_HOGDescriptor_HOGDescriptor() {
		try {
			cv::HOGDescriptor* ret = new cv::HOGDescriptor();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::HOGDescriptor*>)
	}
	
	Result<cv::HOGDescriptor*> cv_HOGDescriptor_HOGDescriptor_Size_Size_Size_Size_int_int_double_HistogramNormType_double_bool_int_bool(const cv::Size* _winSize, const cv::Size* _blockSize, const cv::Size* _blockStride, const cv::Size* _cellSize, int _nbins, int _derivAperture, double _winSigma, cv::HOGDescriptor::HistogramNormType _histogramNormType, double _L2HysThreshold, bool _gammaCorrection, int _nlevels, bool _signedGradient) {
		try {
			cv::HOGDescriptor* ret = new cv::HOGDescriptor(*_winSize, *_blockSize, *_blockStride, *_cellSize, _nbins, _derivAperture, _winSigma, _histogramNormType, _L2HysThreshold, _gammaCorrection, _nlevels, _signedGradient);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::HOGDescriptor*>)
	}
	
	Result<cv::HOGDescriptor*> cv_HOGDescriptor_HOGDescriptor_const_StringX(const char* filename) {
		try {
			cv::HOGDescriptor* ret = new cv::HOGDescriptor(std::string(filename));
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::HOGDescriptor*>)
	}
	
	Result<cv::HOGDescriptor*> cv_HOGDescriptor_HOGDescriptor_const_HOGDescriptorX(const cv::HOGDescriptor* d) {
		try {
			cv::HOGDescriptor* ret = new cv::HOGDescriptor(*d);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::HOGDescriptor*>)
	}
	
	Result<size_t> cv_HOGDescriptor_getDescriptorSize_const(const cv::HOGDescriptor* instance) {
		try {
			size_t ret = instance->getDescriptorSize();
			return Ok(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result<bool> cv_HOGDescriptor_checkDetectorSize_const(const cv::HOGDescriptor* instance) {
		try {
			bool ret = instance->checkDetectorSize();
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<double> cv_HOGDescriptor_getWinSigma_const(const cv::HOGDescriptor* instance) {
		try {
			double ret = instance->getWinSigma();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_HOGDescriptor_setSVMDetector_const__InputArrayX(cv::HOGDescriptor* instance, const cv::_InputArray* svmdetector) {
		try {
			instance->setSVMDetector(*svmdetector);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_HOGDescriptor_read_FileNodeX(cv::HOGDescriptor* instance, cv::FileNode* fn) {
		try {
			bool ret = instance->read(*fn);
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_HOGDescriptor_write_const_FileStorageX_const_StringX(const cv::HOGDescriptor* instance, cv::FileStorage* fs, const char* objname) {
		try {
			instance->write(*fs, std::string(objname));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_HOGDescriptor_load_const_StringX_const_StringX(cv::HOGDescriptor* instance, const char* filename, const char* objname) {
		try {
			bool ret = instance->load(std::string(filename), std::string(objname));
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_HOGDescriptor_save_const_const_StringX_const_StringX(const cv::HOGDescriptor* instance, const char* filename, const char* objname) {
		try {
			instance->save(std::string(filename), std::string(objname));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_HOGDescriptor_copyTo_const_HOGDescriptorX(const cv::HOGDescriptor* instance, cv::HOGDescriptor* c) {
		try {
			instance->copyTo(*c);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_HOGDescriptor_compute_const_const__InputArrayX_vector_float_X_Size_Size_const_vector_Point_X(const cv::HOGDescriptor* instance, const cv::_InputArray* img, std::vector<float>* descriptors, const cv::Size* winStride, const cv::Size* padding, const std::vector<cv::Point>* locations) {
		try {
			instance->compute(*img, *descriptors, *winStride, *padding, *locations);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_HOGDescriptor_detect_const_const__InputArrayX_vector_Point_X_vector_double_X_double_Size_Size_const_vector_Point_X(const cv::HOGDescriptor* instance, const cv::_InputArray* img, std::vector<cv::Point>* foundLocations, std::vector<double>* weights, double hitThreshold, const cv::Size* winStride, const cv::Size* padding, const std::vector<cv::Point>* searchLocations) {
		try {
			instance->detect(*img, *foundLocations, *weights, hitThreshold, *winStride, *padding, *searchLocations);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_HOGDescriptor_detect_const_const__InputArrayX_vector_Point_X_double_Size_Size_const_vector_Point_X(const cv::HOGDescriptor* instance, const cv::_InputArray* img, std::vector<cv::Point>* foundLocations, double hitThreshold, const cv::Size* winStride, const cv::Size* padding, const std::vector<cv::Point>* searchLocations) {
		try {
			instance->detect(*img, *foundLocations, hitThreshold, *winStride, *padding, *searchLocations);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_HOGDescriptor_detectMultiScale_const_const__InputArrayX_vector_Rect_X_vector_double_X_double_Size_Size_double_double_bool(const cv::HOGDescriptor* instance, const cv::_InputArray* img, std::vector<cv::Rect>* foundLocations, std::vector<double>* foundWeights, double hitThreshold, const cv::Size* winStride, const cv::Size* padding, double scale, double finalThreshold, bool useMeanshiftGrouping) {
		try {
			instance->detectMultiScale(*img, *foundLocations, *foundWeights, hitThreshold, *winStride, *padding, scale, finalThreshold, useMeanshiftGrouping);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_HOGDescriptor_detectMultiScale_const_const__InputArrayX_vector_Rect_X_double_Size_Size_double_double_bool(const cv::HOGDescriptor* instance, const cv::_InputArray* img, std::vector<cv::Rect>* foundLocations, double hitThreshold, const cv::Size* winStride, const cv::Size* padding, double scale, double finalThreshold, bool useMeanshiftGrouping) {
		try {
			instance->detectMultiScale(*img, *foundLocations, hitThreshold, *winStride, *padding, scale, finalThreshold, useMeanshiftGrouping);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_HOGDescriptor_computeGradient_const_const__InputArrayX_const__InputOutputArrayX_const__InputOutputArrayX_Size_Size(const cv::HOGDescriptor* instance, const cv::_InputArray* img, const cv::_InputOutputArray* grad, const cv::_InputOutputArray* angleOfs, const cv::Size* paddingTL, const cv::Size* paddingBR) {
		try {
			instance->computeGradient(*img, *grad, *angleOfs, *paddingTL, *paddingBR);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<std::vector<float>*> cv_HOGDescriptor_getDefaultPeopleDetector() {
		try {
			std::vector<float> ret = cv::HOGDescriptor::getDefaultPeopleDetector();
			return Ok(new std::vector<float>(ret));
		} OCVRS_CATCH(Result<std::vector<float>*>)
	}
	
	Result<std::vector<float>*> cv_HOGDescriptor_getDaimlerPeopleDetector() {
		try {
			std::vector<float> ret = cv::HOGDescriptor::getDaimlerPeopleDetector();
			return Ok(new std::vector<float>(ret));
		} OCVRS_CATCH(Result<std::vector<float>*>)
	}
	
	Result_void cv_HOGDescriptor_detectROI_const_const__InputArrayX_const_vector_Point_X_vector_Point_X_vector_double_X_double_Size_Size(const cv::HOGDescriptor* instance, const cv::_InputArray* img, const std::vector<cv::Point>* locations, std::vector<cv::Point>* foundLocations, std::vector<double>* confidences, double hitThreshold, const cv::Size* winStride, const cv::Size* padding) {
		try {
			instance->detectROI(*img, *locations, *foundLocations, *confidences, hitThreshold, *winStride, *padding);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_HOGDescriptor_detectMultiScaleROI_const_const__InputArrayX_vector_Rect_X_vector_DetectionROI_X_double_int(const cv::HOGDescriptor* instance, const cv::_InputArray* img, std::vector<cv::Rect>* foundLocations, std::vector<cv::DetectionROI>* locations, double hitThreshold, int groupThreshold) {
		try {
			instance->detectMultiScaleROI(*img, *foundLocations, *locations, hitThreshold, groupThreshold);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_HOGDescriptor_groupRectangles_const_vector_Rect_X_vector_double_X_int_double(const cv::HOGDescriptor* instance, std::vector<cv::Rect>* rectList, std::vector<double>* weights, int groupThreshold, double eps) {
		try {
			instance->groupRectangles(*rectList, *weights, groupThreshold, eps);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_QRCodeDetector_delete(cv::QRCodeDetector* instance) {
		delete instance;
	}
	Result<cv::QRCodeDetector*> cv_QRCodeDetector_QRCodeDetector() {
		try {
			cv::QRCodeDetector* ret = new cv::QRCodeDetector();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::QRCodeDetector*>)
	}
	
	Result_void cv_QRCodeDetector_setEpsX_double(cv::QRCodeDetector* instance, double epsX) {
		try {
			instance->setEpsX(epsX);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_QRCodeDetector_setEpsY_double(cv::QRCodeDetector* instance, double epsY) {
		try {
			instance->setEpsY(epsY);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_QRCodeDetector_detect_const_const__InputArrayX_const__OutputArrayX(const cv::QRCodeDetector* instance, const cv::_InputArray* img, const cv::_OutputArray* points) {
		try {
			bool ret = instance->detect(*img, *points);
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<void*> cv_QRCodeDetector_decode_const__InputArrayX_const__InputArrayX_const__OutputArrayX(cv::QRCodeDetector* instance, const cv::_InputArray* img, const cv::_InputArray* points, const cv::_OutputArray* straight_qrcode) {
		try {
			std::string ret = instance->decode(*img, *points, *straight_qrcode);
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_QRCodeDetector_detectAndDecode_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(cv::QRCodeDetector* instance, const cv::_InputArray* img, const cv::_OutputArray* points, const cv::_OutputArray* straight_qrcode) {
		try {
			std::string ret = instance->detectAndDecode(*img, *points, *straight_qrcode);
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<double> cv_SimilarRects_eps_const(const cv::SimilarRects* instance) {
		try {
			double ret = instance->eps;
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_SimilarRects_setEps_double(cv::SimilarRects* instance, double val) {
		try {
			instance->eps = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_SimilarRects_delete(cv::SimilarRects* instance) {
		delete instance;
	}
	Result<cv::SimilarRects*> cv_SimilarRects_SimilarRects_double(double _eps) {
		try {
			cv::SimilarRects* ret = new cv::SimilarRects(_eps);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::SimilarRects*>)
	}
	
}
