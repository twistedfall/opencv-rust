#include "common.hpp"
#include <opencv2/objdetect.hpp>
#include "objdetect_types.hpp"

extern "C" {
	Result<void*> cv_createFaceDetectionMaskGenerator() {
		try {
			cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator> ret = cv::createFaceDetectionMaskGenerator();
			return Ok<void*>(new cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_groupRectangles_meanshift_vector_Rect_X_vector_double_X_vector_double_X_double_Size(void* rectList, void* foundWeights, void* foundScales, double detectThreshold, const cv::Size* winDetSize) {
		try {
			cv::groupRectangles_meanshift(*reinterpret_cast<std::vector<cv::Rect>*>(rectList), *reinterpret_cast<std::vector<double>*>(foundWeights), *reinterpret_cast<std::vector<double>*>(foundScales), detectThreshold, *winDetSize);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_groupRectangles_vector_Rect_X_int_double(void* rectList, int groupThreshold, double eps) {
		try {
			cv::groupRectangles(*reinterpret_cast<std::vector<cv::Rect>*>(rectList), groupThreshold, eps);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_groupRectangles_vector_Rect_X_int_double_vector_int_X_vector_double_X(void* rectList, int groupThreshold, double eps, void* weights, void* levelWeights) {
		try {
			cv::groupRectangles(*reinterpret_cast<std::vector<cv::Rect>*>(rectList), groupThreshold, eps, reinterpret_cast<std::vector<int>*>(weights), reinterpret_cast<std::vector<double>*>(levelWeights));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_groupRectangles_vector_Rect_X_vector_int_X_int_double(void* rectList, void* weights, int groupThreshold, double eps) {
		try {
			cv::groupRectangles(*reinterpret_cast<std::vector<cv::Rect>*>(rectList), *reinterpret_cast<std::vector<int>*>(weights), groupThreshold, eps);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_groupRectangles_vector_Rect_X_vector_int_X_vector_double_X_int_double(void* rectList, void* rejectLevels, void* levelWeights, int groupThreshold, double eps) {
		try {
			cv::groupRectangles(*reinterpret_cast<std::vector<cv::Rect>*>(rectList), *reinterpret_cast<std::vector<int>*>(rejectLevels), *reinterpret_cast<std::vector<double>*>(levelWeights), groupThreshold, eps);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_BaseCascadeClassifier_empty_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::BaseCascadeClassifier*>(instance)->empty();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_BaseCascadeClassifier_load_const_StringX(void* instance, const char* filename) {
		try {
			bool ret = reinterpret_cast<cv::BaseCascadeClassifier*>(instance)->load(cv::String(filename));
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_BaseCascadeClassifier_detectMultiScale_const__InputArrayX_vector_Rect_X_double_int_int_Size_Size(void* instance, void* image, void* objects, double scaleFactor, int minNeighbors, int flags, const cv::Size* minSize, const cv::Size* maxSize) {
		try {
			reinterpret_cast<cv::BaseCascadeClassifier*>(instance)->detectMultiScale(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<std::vector<cv::Rect>*>(objects), scaleFactor, minNeighbors, flags, *minSize, *maxSize);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_BaseCascadeClassifier_detectMultiScale_const__InputArrayX_vector_Rect_X_vector_int_X_double_int_int_Size_Size(void* instance, void* image, void* objects, void* numDetections, double scaleFactor, int minNeighbors, int flags, const cv::Size* minSize, const cv::Size* maxSize) {
		try {
			reinterpret_cast<cv::BaseCascadeClassifier*>(instance)->detectMultiScale(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<std::vector<cv::Rect>*>(objects), *reinterpret_cast<std::vector<int>*>(numDetections), scaleFactor, minNeighbors, flags, *minSize, *maxSize);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_BaseCascadeClassifier_detectMultiScale_const__InputArrayX_vector_Rect_X_vector_int_X_vector_double_X_double_int_int_Size_Size_bool(void* instance, void* image, void* objects, void* rejectLevels, void* levelWeights, double scaleFactor, int minNeighbors, int flags, const cv::Size* minSize, const cv::Size* maxSize, bool outputRejectLevels) {
		try {
			reinterpret_cast<cv::BaseCascadeClassifier*>(instance)->detectMultiScale(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<std::vector<cv::Rect>*>(objects), *reinterpret_cast<std::vector<int>*>(rejectLevels), *reinterpret_cast<std::vector<double>*>(levelWeights), scaleFactor, minNeighbors, flags, *minSize, *maxSize, outputRejectLevels);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_BaseCascadeClassifier_isOldFormatCascade_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::BaseCascadeClassifier*>(instance)->isOldFormatCascade();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<cv::Size> cv_BaseCascadeClassifier_getOriginalWindowSize_const(void* instance) {
		try {
			cv::Size ret = reinterpret_cast<cv::BaseCascadeClassifier*>(instance)->getOriginalWindowSize();
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result<int> cv_BaseCascadeClassifier_getFeatureType_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::BaseCascadeClassifier*>(instance)->getFeatureType();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<void*> cv_BaseCascadeClassifier_getOldCascade(void* instance) {
		try {
			void* ret = reinterpret_cast<cv::BaseCascadeClassifier*>(instance)->getOldCascade();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_BaseCascadeClassifier_setMaskGenerator_const_Ptr_MaskGenerator_X(void* instance, void* maskGenerator) {
		try {
			reinterpret_cast<cv::BaseCascadeClassifier*>(instance)->setMaskGenerator(*reinterpret_cast<const cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>*>(maskGenerator));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_BaseCascadeClassifier_getMaskGenerator(void* instance) {
		try {
			cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator> ret = reinterpret_cast<cv::BaseCascadeClassifier*>(instance)->getMaskGenerator();
			return Ok<void*>(new cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_BaseCascadeClassifier_MaskGenerator_generateMask_const_MatX(void* instance, void* src) {
		try {
			cv::Mat ret = reinterpret_cast<cv::BaseCascadeClassifier::MaskGenerator*>(instance)->generateMask(*reinterpret_cast<const cv::Mat*>(src));
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_BaseCascadeClassifier_MaskGenerator_initializeMask_const_MatX(void* instance, void* unnamed) {
		try {
			reinterpret_cast<cv::BaseCascadeClassifier::MaskGenerator*>(instance)->initializeMask(*reinterpret_cast<const cv::Mat*>(unnamed));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_CascadeClassifier_cc(void* instance) {
		try {
			cv::Ptr<cv::BaseCascadeClassifier> ret = reinterpret_cast<cv::CascadeClassifier*>(instance)->cc;
			return Ok<void*>(new cv::Ptr<cv::BaseCascadeClassifier>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_CascadeClassifier_setCc_Ptr_BaseCascadeClassifier_(void* instance, void* val) {
		try {
			reinterpret_cast<cv::CascadeClassifier*>(instance)->cc = *reinterpret_cast<cv::Ptr<cv::BaseCascadeClassifier>*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_CascadeClassifier_delete(cv::CascadeClassifier* instance) {
		delete instance;
	}
	Result<void*> cv_CascadeClassifier_CascadeClassifier() {
		try {
			cv::CascadeClassifier* ret = new cv::CascadeClassifier();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_CascadeClassifier_CascadeClassifier_const_StringX(const char* filename) {
		try {
			cv::CascadeClassifier* ret = new cv::CascadeClassifier(cv::String(filename));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<bool> cv_CascadeClassifier_empty_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::CascadeClassifier*>(instance)->empty();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_CascadeClassifier_load_const_StringX(void* instance, const char* filename) {
		try {
			bool ret = reinterpret_cast<cv::CascadeClassifier*>(instance)->load(cv::String(filename));
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_CascadeClassifier_read_const_FileNodeX(void* instance, void* node) {
		try {
			bool ret = reinterpret_cast<cv::CascadeClassifier*>(instance)->read(*reinterpret_cast<const cv::FileNode*>(node));
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_CascadeClassifier_detectMultiScale_const__InputArrayX_vector_Rect_X_double_int_int_Size_Size(void* instance, void* image, void* objects, double scaleFactor, int minNeighbors, int flags, const cv::Size* minSize, const cv::Size* maxSize) {
		try {
			reinterpret_cast<cv::CascadeClassifier*>(instance)->detectMultiScale(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<std::vector<cv::Rect>*>(objects), scaleFactor, minNeighbors, flags, *minSize, *maxSize);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_CascadeClassifier_detectMultiScale_const__InputArrayX_vector_Rect_X_vector_int_X_double_int_int_Size_Size(void* instance, void* image, void* objects, void* numDetections, double scaleFactor, int minNeighbors, int flags, const cv::Size* minSize, const cv::Size* maxSize) {
		try {
			reinterpret_cast<cv::CascadeClassifier*>(instance)->detectMultiScale(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<std::vector<cv::Rect>*>(objects), *reinterpret_cast<std::vector<int>*>(numDetections), scaleFactor, minNeighbors, flags, *minSize, *maxSize);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_CascadeClassifier_detectMultiScale_const__InputArrayX_vector_Rect_X_vector_int_X_vector_double_X_double_int_int_Size_Size_bool(void* instance, void* image, void* objects, void* rejectLevels, void* levelWeights, double scaleFactor, int minNeighbors, int flags, const cv::Size* minSize, const cv::Size* maxSize, bool outputRejectLevels) {
		try {
			reinterpret_cast<cv::CascadeClassifier*>(instance)->detectMultiScale(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<std::vector<cv::Rect>*>(objects), *reinterpret_cast<std::vector<int>*>(rejectLevels), *reinterpret_cast<std::vector<double>*>(levelWeights), scaleFactor, minNeighbors, flags, *minSize, *maxSize, outputRejectLevels);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_CascadeClassifier_isOldFormatCascade_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::CascadeClassifier*>(instance)->isOldFormatCascade();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<cv::Size> cv_CascadeClassifier_getOriginalWindowSize_const(void* instance) {
		try {
			cv::Size ret = reinterpret_cast<cv::CascadeClassifier*>(instance)->getOriginalWindowSize();
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result<int> cv_CascadeClassifier_getFeatureType_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::CascadeClassifier*>(instance)->getFeatureType();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<void*> cv_CascadeClassifier_getOldCascade(void* instance) {
		try {
			void* ret = reinterpret_cast<cv::CascadeClassifier*>(instance)->getOldCascade();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<bool> cv_CascadeClassifier_convert_const_StringX_const_StringX(const char* oldcascade, const char* newcascade) {
		try {
			bool ret = cv::CascadeClassifier::convert(cv::String(oldcascade), cv::String(newcascade));
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_CascadeClassifier_setMaskGenerator_const_Ptr_MaskGenerator_X(void* instance, void* maskGenerator) {
		try {
			reinterpret_cast<cv::CascadeClassifier*>(instance)->setMaskGenerator(*reinterpret_cast<const cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>*>(maskGenerator));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_CascadeClassifier_getMaskGenerator(void* instance) {
		try {
			cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator> ret = reinterpret_cast<cv::CascadeClassifier*>(instance)->getMaskGenerator();
			return Ok<void*>(new cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_DetectionBasedTracker_delete(cv::DetectionBasedTracker* instance) {
		delete instance;
	}
	Result<void*> cv_DetectionBasedTracker_DetectionBasedTracker_Ptr_IDetector__Ptr_IDetector__const_ParametersX(void* mainDetector, void* trackingDetector, void* params) {
		try {
			cv::DetectionBasedTracker* ret = new cv::DetectionBasedTracker(*reinterpret_cast<cv::Ptr<cv::DetectionBasedTracker::IDetector>*>(mainDetector), *reinterpret_cast<cv::Ptr<cv::DetectionBasedTracker::IDetector>*>(trackingDetector), *reinterpret_cast<const cv::DetectionBasedTracker::Parameters*>(params));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<bool> cv_DetectionBasedTracker_run(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::DetectionBasedTracker*>(instance)->run();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_DetectionBasedTracker_stop(void* instance) {
		try {
			reinterpret_cast<cv::DetectionBasedTracker*>(instance)->stop();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_DetectionBasedTracker_resetTracking(void* instance) {
		try {
			reinterpret_cast<cv::DetectionBasedTracker*>(instance)->resetTracking();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_DetectionBasedTracker_process_const_MatX(void* instance, void* imageGray) {
		try {
			reinterpret_cast<cv::DetectionBasedTracker*>(instance)->process(*reinterpret_cast<const cv::Mat*>(imageGray));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_DetectionBasedTracker_setParameters_const_ParametersX(void* instance, void* params) {
		try {
			bool ret = reinterpret_cast<cv::DetectionBasedTracker*>(instance)->setParameters(*reinterpret_cast<const cv::DetectionBasedTracker::Parameters*>(params));
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<void*> cv_DetectionBasedTracker_getParameters_const(void* instance) {
		try {
			cv::DetectionBasedTracker::Parameters ret = reinterpret_cast<cv::DetectionBasedTracker*>(instance)->getParameters();
			return Ok<void*>(new cv::DetectionBasedTracker::Parameters(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_DetectionBasedTracker_getObjects_const_vector_Rect_X(void* instance, void* result) {
		try {
			reinterpret_cast<cv::DetectionBasedTracker*>(instance)->getObjects(*reinterpret_cast<std::vector<cv::Rect>*>(result));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_DetectionBasedTracker_getObjects_const_vector_ExtObject_X(void* instance, void* result) {
		try {
			reinterpret_cast<cv::DetectionBasedTracker*>(instance)->getObjects(*reinterpret_cast<std::vector<cv::DetectionBasedTracker::ExtObject>*>(result));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_DetectionBasedTracker_addObject_const_RectX(void* instance, const cv::Rect* location) {
		try {
			int ret = reinterpret_cast<cv::DetectionBasedTracker*>(instance)->addObject(*location);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_DetectionBasedTracker_ExtObject_id_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::DetectionBasedTracker::ExtObject*>(instance)->id;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_DetectionBasedTracker_ExtObject_setId_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::DetectionBasedTracker::ExtObject*>(instance)->id = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Rect> cv_DetectionBasedTracker_ExtObject_location_const(void* instance) {
		try {
			cv::Rect ret = reinterpret_cast<cv::DetectionBasedTracker::ExtObject*>(instance)->location;
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(Result<cv::Rect>)
	}
	
	Result_void cv_DetectionBasedTracker_ExtObject_setLocation_Rect(void* instance, const cv::Rect* val) {
		try {
			reinterpret_cast<cv::DetectionBasedTracker::ExtObject*>(instance)->location = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::DetectionBasedTracker::ObjectStatus> cv_DetectionBasedTracker_ExtObject_status_const(void* instance) {
		try {
			cv::DetectionBasedTracker::ObjectStatus ret = reinterpret_cast<cv::DetectionBasedTracker::ExtObject*>(instance)->status;
			return Ok<cv::DetectionBasedTracker::ObjectStatus>(ret);
		} OCVRS_CATCH(Result<cv::DetectionBasedTracker::ObjectStatus>)
	}
	
	Result_void cv_DetectionBasedTracker_ExtObject_setStatus_ObjectStatus(void* instance, cv::DetectionBasedTracker::ObjectStatus val) {
		try {
			reinterpret_cast<cv::DetectionBasedTracker::ExtObject*>(instance)->status = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_DetectionBasedTracker_ExtObject_delete(cv::DetectionBasedTracker::ExtObject* instance) {
		delete instance;
	}
	Result<void*> cv_DetectionBasedTracker_ExtObject_ExtObject_int_Rect_ObjectStatus(int _id, const cv::Rect* _location, cv::DetectionBasedTracker::ObjectStatus _status) {
		try {
			cv::DetectionBasedTracker::ExtObject* ret = new cv::DetectionBasedTracker::ExtObject(_id, *_location, _status);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_DetectionBasedTracker_IDetector_detect_const_MatX_vector_Rect_X(void* instance, void* image, void* objects) {
		try {
			reinterpret_cast<cv::DetectionBasedTracker::IDetector*>(instance)->detect(*reinterpret_cast<const cv::Mat*>(image), *reinterpret_cast<std::vector<cv::Rect>*>(objects));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_DetectionBasedTracker_IDetector_setMinObjectSize_const_SizeX(void* instance, const cv::Size* min) {
		try {
			reinterpret_cast<cv::DetectionBasedTracker::IDetector*>(instance)->setMinObjectSize(*min);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_DetectionBasedTracker_IDetector_setMaxObjectSize_const_SizeX(void* instance, const cv::Size* max) {
		try {
			reinterpret_cast<cv::DetectionBasedTracker::IDetector*>(instance)->setMaxObjectSize(*max);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Size> cv_DetectionBasedTracker_IDetector_getMinObjectSize_const(void* instance) {
		try {
			cv::Size ret = reinterpret_cast<cv::DetectionBasedTracker::IDetector*>(instance)->getMinObjectSize();
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result<cv::Size> cv_DetectionBasedTracker_IDetector_getMaxObjectSize_const(void* instance) {
		try {
			cv::Size ret = reinterpret_cast<cv::DetectionBasedTracker::IDetector*>(instance)->getMaxObjectSize();
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result<float> cv_DetectionBasedTracker_IDetector_getScaleFactor(void* instance) {
		try {
			float ret = reinterpret_cast<cv::DetectionBasedTracker::IDetector*>(instance)->getScaleFactor();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_DetectionBasedTracker_IDetector_setScaleFactor_float(void* instance, float value) {
		try {
			reinterpret_cast<cv::DetectionBasedTracker::IDetector*>(instance)->setScaleFactor(value);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_DetectionBasedTracker_IDetector_getMinNeighbours(void* instance) {
		try {
			int ret = reinterpret_cast<cv::DetectionBasedTracker::IDetector*>(instance)->getMinNeighbours();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_DetectionBasedTracker_IDetector_setMinNeighbours_int(void* instance, int value) {
		try {
			reinterpret_cast<cv::DetectionBasedTracker::IDetector*>(instance)->setMinNeighbours(value);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_DetectionBasedTracker_Parameters_maxTrackLifetime_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::DetectionBasedTracker::Parameters*>(instance)->maxTrackLifetime;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_DetectionBasedTracker_Parameters_setMaxTrackLifetime_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::DetectionBasedTracker::Parameters*>(instance)->maxTrackLifetime = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_DetectionBasedTracker_Parameters_minDetectionPeriod_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::DetectionBasedTracker::Parameters*>(instance)->minDetectionPeriod;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_DetectionBasedTracker_Parameters_setMinDetectionPeriod_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::DetectionBasedTracker::Parameters*>(instance)->minDetectionPeriod = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_DetectionBasedTracker_Parameters_delete(cv::DetectionBasedTracker::Parameters* instance) {
		delete instance;
	}
	Result<void*> cv_DetectionBasedTracker_Parameters_Parameters() {
		try {
			cv::DetectionBasedTracker::Parameters* ret = new cv::DetectionBasedTracker::Parameters();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<double> cv_DetectionROI_scale_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::DetectionROI*>(instance)->scale;
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_DetectionROI_setScale_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::DetectionROI*>(instance)->scale = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_DetectionROI_locations(void* instance) {
		try {
			std::vector<cv::Point> ret = reinterpret_cast<cv::DetectionROI*>(instance)->locations;
			return Ok<void*>(new std::vector<cv::Point>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_DetectionROI_setLocations_vector_Point_(void* instance, void* val) {
		try {
			reinterpret_cast<cv::DetectionROI*>(instance)->locations = *reinterpret_cast<std::vector<cv::Point>*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_DetectionROI_confidences(void* instance) {
		try {
			std::vector<double> ret = reinterpret_cast<cv::DetectionROI*>(instance)->confidences;
			return Ok<void*>(new std::vector<double>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_DetectionROI_setConfidences_vector_double_(void* instance, void* val) {
		try {
			reinterpret_cast<cv::DetectionROI*>(instance)->confidences = *reinterpret_cast<std::vector<double>*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_DetectionROI_delete(cv::DetectionROI* instance) {
		delete instance;
	}
	Result<cv::Size> cv_HOGDescriptor_winSize_const(void* instance) {
		try {
			cv::Size ret = reinterpret_cast<cv::HOGDescriptor*>(instance)->winSize;
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_HOGDescriptor_setWinSize_Size(void* instance, const cv::Size* val) {
		try {
			reinterpret_cast<cv::HOGDescriptor*>(instance)->winSize = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Size> cv_HOGDescriptor_blockSize_const(void* instance) {
		try {
			cv::Size ret = reinterpret_cast<cv::HOGDescriptor*>(instance)->blockSize;
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_HOGDescriptor_setBlockSize_Size(void* instance, const cv::Size* val) {
		try {
			reinterpret_cast<cv::HOGDescriptor*>(instance)->blockSize = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Size> cv_HOGDescriptor_blockStride_const(void* instance) {
		try {
			cv::Size ret = reinterpret_cast<cv::HOGDescriptor*>(instance)->blockStride;
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_HOGDescriptor_setBlockStride_Size(void* instance, const cv::Size* val) {
		try {
			reinterpret_cast<cv::HOGDescriptor*>(instance)->blockStride = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Size> cv_HOGDescriptor_cellSize_const(void* instance) {
		try {
			cv::Size ret = reinterpret_cast<cv::HOGDescriptor*>(instance)->cellSize;
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_HOGDescriptor_setCellSize_Size(void* instance, const cv::Size* val) {
		try {
			reinterpret_cast<cv::HOGDescriptor*>(instance)->cellSize = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_HOGDescriptor_nbins_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::HOGDescriptor*>(instance)->nbins;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_HOGDescriptor_setNbins_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::HOGDescriptor*>(instance)->nbins = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_HOGDescriptor_derivAperture_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::HOGDescriptor*>(instance)->derivAperture;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_HOGDescriptor_setDerivAperture_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::HOGDescriptor*>(instance)->derivAperture = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_HOGDescriptor_winSigma_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::HOGDescriptor*>(instance)->winSigma;
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_HOGDescriptor_setWinSigma_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::HOGDescriptor*>(instance)->winSigma = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_HOGDescriptor_histogramNormType_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::HOGDescriptor*>(instance)->histogramNormType;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_HOGDescriptor_setHistogramNormType_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::HOGDescriptor*>(instance)->histogramNormType = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_HOGDescriptor_L2HysThreshold_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::HOGDescriptor*>(instance)->L2HysThreshold;
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_HOGDescriptor_setL2HysThreshold_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::HOGDescriptor*>(instance)->L2HysThreshold = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_HOGDescriptor_gammaCorrection_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::HOGDescriptor*>(instance)->gammaCorrection;
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_HOGDescriptor_setGammaCorrection_bool(void* instance, bool val) {
		try {
			reinterpret_cast<cv::HOGDescriptor*>(instance)->gammaCorrection = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_HOGDescriptor_svmDetector(void* instance) {
		try {
			std::vector<float> ret = reinterpret_cast<cv::HOGDescriptor*>(instance)->svmDetector;
			return Ok<void*>(new std::vector<float>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_HOGDescriptor_setSvmDetector_vector_float_(void* instance, void* val) {
		try {
			reinterpret_cast<cv::HOGDescriptor*>(instance)->svmDetector = *reinterpret_cast<std::vector<float>*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_HOGDescriptor_oclSvmDetector(void* instance) {
		try {
			cv::UMat ret = reinterpret_cast<cv::HOGDescriptor*>(instance)->oclSvmDetector;
			return Ok<void*>(new cv::UMat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_HOGDescriptor_setOclSvmDetector_UMat(void* instance, void* val) {
		try {
			reinterpret_cast<cv::HOGDescriptor*>(instance)->oclSvmDetector = *reinterpret_cast<cv::UMat*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_HOGDescriptor_free_coef_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::HOGDescriptor*>(instance)->free_coef;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_HOGDescriptor_setFree_coef_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::HOGDescriptor*>(instance)->free_coef = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_HOGDescriptor_nlevels_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::HOGDescriptor*>(instance)->nlevels;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_HOGDescriptor_setNlevels_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::HOGDescriptor*>(instance)->nlevels = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_HOGDescriptor_signedGradient_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::HOGDescriptor*>(instance)->signedGradient;
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_HOGDescriptor_setSignedGradient_bool(void* instance, bool val) {
		try {
			reinterpret_cast<cv::HOGDescriptor*>(instance)->signedGradient = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_HOGDescriptor_delete(cv::HOGDescriptor* instance) {
		delete instance;
	}
	Result<void*> cv_HOGDescriptor_HOGDescriptor() {
		try {
			cv::HOGDescriptor* ret = new cv::HOGDescriptor();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_HOGDescriptor_HOGDescriptor_Size_Size_Size_Size_int_int_double_int_double_bool_int_bool(const cv::Size* _winSize, const cv::Size* _blockSize, const cv::Size* _blockStride, const cv::Size* _cellSize, int _nbins, int _derivAperture, double _winSigma, int _histogramNormType, double _L2HysThreshold, bool _gammaCorrection, int _nlevels, bool _signedGradient) {
		try {
			cv::HOGDescriptor* ret = new cv::HOGDescriptor(*_winSize, *_blockSize, *_blockStride, *_cellSize, _nbins, _derivAperture, _winSigma, _histogramNormType, _L2HysThreshold, _gammaCorrection, _nlevels, _signedGradient);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_HOGDescriptor_HOGDescriptor_const_StringX(const char* filename) {
		try {
			cv::HOGDescriptor* ret = new cv::HOGDescriptor(cv::String(filename));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_HOGDescriptor_HOGDescriptor_const_HOGDescriptorX(void* d) {
		try {
			cv::HOGDescriptor* ret = new cv::HOGDescriptor(*reinterpret_cast<const cv::HOGDescriptor*>(d));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<size_t> cv_HOGDescriptor_getDescriptorSize_const(void* instance) {
		try {
			size_t ret = reinterpret_cast<cv::HOGDescriptor*>(instance)->getDescriptorSize();
			return Ok<size_t>(ret);
		} OCVRS_CATCH(Result<size_t>)
	}
	
	Result<bool> cv_HOGDescriptor_checkDetectorSize_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::HOGDescriptor*>(instance)->checkDetectorSize();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<double> cv_HOGDescriptor_getWinSigma_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::HOGDescriptor*>(instance)->getWinSigma();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_HOGDescriptor_setSVMDetector_const__InputArrayX(void* instance, void* _svmdetector) {
		try {
			reinterpret_cast<cv::HOGDescriptor*>(instance)->setSVMDetector(*reinterpret_cast<const cv::_InputArray*>(_svmdetector));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_HOGDescriptor_read_FileNodeX(void* instance, void* fn) {
		try {
			bool ret = reinterpret_cast<cv::HOGDescriptor*>(instance)->read(*reinterpret_cast<cv::FileNode*>(fn));
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_HOGDescriptor_write_const_FileStorageX_const_StringX(void* instance, void* fs, const char* objname) {
		try {
			reinterpret_cast<cv::HOGDescriptor*>(instance)->write(*reinterpret_cast<cv::FileStorage*>(fs), cv::String(objname));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_HOGDescriptor_load_const_StringX_const_StringX(void* instance, const char* filename, const char* objname) {
		try {
			bool ret = reinterpret_cast<cv::HOGDescriptor*>(instance)->load(cv::String(filename), cv::String(objname));
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_HOGDescriptor_save_const_const_StringX_const_StringX(void* instance, const char* filename, const char* objname) {
		try {
			reinterpret_cast<cv::HOGDescriptor*>(instance)->save(cv::String(filename), cv::String(objname));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_HOGDescriptor_copyTo_const_HOGDescriptorX(void* instance, void* c) {
		try {
			reinterpret_cast<cv::HOGDescriptor*>(instance)->copyTo(*reinterpret_cast<cv::HOGDescriptor*>(c));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_HOGDescriptor_compute_const_const__InputArrayX_vector_float_X_Size_Size_const_vector_Point_X(void* instance, void* img, void* descriptors, const cv::Size* winStride, const cv::Size* padding, void* locations) {
		try {
			reinterpret_cast<cv::HOGDescriptor*>(instance)->compute(*reinterpret_cast<const cv::_InputArray*>(img), *reinterpret_cast<std::vector<float>*>(descriptors), *winStride, *padding, *reinterpret_cast<const std::vector<cv::Point>*>(locations));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_HOGDescriptor_detect_const_const_MatX_vector_Point_X_vector_double_X_double_Size_Size_const_vector_Point_X(void* instance, void* img, void* foundLocations, void* weights, double hitThreshold, const cv::Size* winStride, const cv::Size* padding, void* searchLocations) {
		try {
			reinterpret_cast<cv::HOGDescriptor*>(instance)->detect(*reinterpret_cast<const cv::Mat*>(img), *reinterpret_cast<std::vector<cv::Point>*>(foundLocations), *reinterpret_cast<std::vector<double>*>(weights), hitThreshold, *winStride, *padding, *reinterpret_cast<const std::vector<cv::Point>*>(searchLocations));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_HOGDescriptor_detect_const_const_MatX_vector_Point_X_double_Size_Size_const_vector_Point_X(void* instance, void* img, void* foundLocations, double hitThreshold, const cv::Size* winStride, const cv::Size* padding, void* searchLocations) {
		try {
			reinterpret_cast<cv::HOGDescriptor*>(instance)->detect(*reinterpret_cast<const cv::Mat*>(img), *reinterpret_cast<std::vector<cv::Point>*>(foundLocations), hitThreshold, *winStride, *padding, *reinterpret_cast<const std::vector<cv::Point>*>(searchLocations));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_HOGDescriptor_detectMultiScale_const_const__InputArrayX_vector_Rect_X_vector_double_X_double_Size_Size_double_double_bool(void* instance, void* img, void* foundLocations, void* foundWeights, double hitThreshold, const cv::Size* winStride, const cv::Size* padding, double scale, double finalThreshold, bool useMeanshiftGrouping) {
		try {
			reinterpret_cast<cv::HOGDescriptor*>(instance)->detectMultiScale(*reinterpret_cast<const cv::_InputArray*>(img), *reinterpret_cast<std::vector<cv::Rect>*>(foundLocations), *reinterpret_cast<std::vector<double>*>(foundWeights), hitThreshold, *winStride, *padding, scale, finalThreshold, useMeanshiftGrouping);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_HOGDescriptor_detectMultiScale_const_const__InputArrayX_vector_Rect_X_double_Size_Size_double_double_bool(void* instance, void* img, void* foundLocations, double hitThreshold, const cv::Size* winStride, const cv::Size* padding, double scale, double finalThreshold, bool useMeanshiftGrouping) {
		try {
			reinterpret_cast<cv::HOGDescriptor*>(instance)->detectMultiScale(*reinterpret_cast<const cv::_InputArray*>(img), *reinterpret_cast<std::vector<cv::Rect>*>(foundLocations), hitThreshold, *winStride, *padding, scale, finalThreshold, useMeanshiftGrouping);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_HOGDescriptor_computeGradient_const_const_MatX_MatX_MatX_Size_Size(void* instance, void* img, void* grad, void* angleOfs, const cv::Size* paddingTL, const cv::Size* paddingBR) {
		try {
			reinterpret_cast<cv::HOGDescriptor*>(instance)->computeGradient(*reinterpret_cast<const cv::Mat*>(img), *reinterpret_cast<cv::Mat*>(grad), *reinterpret_cast<cv::Mat*>(angleOfs), *paddingTL, *paddingBR);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_HOGDescriptor_getDefaultPeopleDetector() {
		try {
			std::vector<float> ret = cv::HOGDescriptor::getDefaultPeopleDetector();
			return Ok<void*>(new std::vector<float>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_HOGDescriptor_getDaimlerPeopleDetector() {
		try {
			std::vector<float> ret = cv::HOGDescriptor::getDaimlerPeopleDetector();
			return Ok<void*>(new std::vector<float>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_HOGDescriptor_detectROI_const_const_MatX_const_vector_Point_X_vector_Point_X_vector_double_X_double_Size_Size(void* instance, void* img, void* locations, void* foundLocations, void* confidences, double hitThreshold, const cv::Size* winStride, const cv::Size* padding) {
		try {
			reinterpret_cast<cv::HOGDescriptor*>(instance)->detectROI(*reinterpret_cast<const cv::Mat*>(img), *reinterpret_cast<const std::vector<cv::Point>*>(locations), *reinterpret_cast<std::vector<cv::Point>*>(foundLocations), *reinterpret_cast<std::vector<double>*>(confidences), hitThreshold, *winStride, *padding);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_HOGDescriptor_detectMultiScaleROI_const_const_MatX_vector_Rect_X_vector_DetectionROI_X_double_int(void* instance, void* img, void* foundLocations, void* locations, double hitThreshold, int groupThreshold) {
		try {
			reinterpret_cast<cv::HOGDescriptor*>(instance)->detectMultiScaleROI(*reinterpret_cast<const cv::Mat*>(img), *reinterpret_cast<std::vector<cv::Rect>*>(foundLocations), *reinterpret_cast<std::vector<cv::DetectionROI>*>(locations), hitThreshold, groupThreshold);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_HOGDescriptor_readALTModel_String(void* instance, char* modelfile) {
		try {
			reinterpret_cast<cv::HOGDescriptor*>(instance)->readALTModel(cv::String(modelfile));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_HOGDescriptor_groupRectangles_const_vector_Rect_X_vector_double_X_int_double(void* instance, void* rectList, void* weights, int groupThreshold, double eps) {
		try {
			reinterpret_cast<cv::HOGDescriptor*>(instance)->groupRectangles(*reinterpret_cast<std::vector<cv::Rect>*>(rectList), *reinterpret_cast<std::vector<double>*>(weights), groupThreshold, eps);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_SimilarRects_eps_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::SimilarRects*>(instance)->eps;
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_SimilarRects_setEps_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::SimilarRects*>(instance)->eps = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_SimilarRects_delete(cv::SimilarRects* instance) {
		delete instance;
	}
	Result<void*> cv_SimilarRects_SimilarRects_double(double _eps) {
		try {
			cv::SimilarRects* ret = new cv::SimilarRects(_eps);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
}
