#include "ocvrs_common.hpp"
#include <opencv2/tracking.hpp>
#include "tracking_types.hpp"

extern "C" {
	Result<cv::Rect2d> cv_selectROI_Mat_bool(cv::Mat* img, bool fromCenter) {
		try {
			cv::Rect2d ret = cv::selectROI(*img, fromCenter);
			return Ok<cv::Rect2d>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect2d>))
	}
	
	Result<cv::Rect2d> cv_selectROI_const_StringR_Mat_bool_bool(const char* windowName, cv::Mat* img, bool showCrossair, bool fromCenter) {
		try {
			cv::Rect2d ret = cv::selectROI(cv::String(windowName), *img, showCrossair, fromCenter);
			return Ok<cv::Rect2d>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect2d>))
	}
	
	Result_void cv_selectROI_const_StringR_Mat_vector_Rect2d_R_bool(const char* windowName, cv::Mat* img, std::vector<cv::Rect2d>* boundingBox, bool fromCenter) {
		try {
			cv::selectROI(cv::String(windowName), *img, *boundingBox, fromCenter);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Rect2d> cv_tld_tld_InitDataset_int_const_charX_int(int videoInd, const char* rootPath, int datasetInd) {
		try {
			cv::Rect2d ret = cv::tld::tld_InitDataset(videoInd, rootPath, datasetInd);
			return Ok<cv::Rect2d>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect2d>))
	}
	
	Result<cv::Mat*> cv_tld_tld_getNextDatasetFrame() {
		try {
			cv::Mat ret = cv::tld::tld_getNextDatasetFrame();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	void cv_ClfMilBoost_delete(cv::ClfMilBoost* instance) {
		delete instance;
	}
	Result<cv::ClfMilBoost*> cv_ClfMilBoost_ClfMilBoost() {
		try {
			cv::ClfMilBoost* ret = new cv::ClfMilBoost();
			return Ok<cv::ClfMilBoost*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ClfMilBoost*>))
	}
	
	Result_void cv_ClfMilBoost_init_const_ParamsR(cv::ClfMilBoost* instance, const cv::ClfMilBoost::Params* parameters) {
		try {
			instance->init(*parameters);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_ClfMilBoost_update_const_MatR_const_MatR(cv::ClfMilBoost* instance, const cv::Mat* posx, const cv::Mat* negx) {
		try {
			instance->update(*posx, *negx);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<float>*> cv_ClfMilBoost_classify_const_MatR_bool(cv::ClfMilBoost* instance, const cv::Mat* x, bool logR) {
		try {
			std::vector<float> ret = instance->classify(*x, logR);
			return Ok(new std::vector<float>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<float>*>))
	}
	
	Result<float> cv_ClfMilBoost_sigmoid_float(cv::ClfMilBoost* instance, float x) {
		try {
			float ret = instance->sigmoid(x);
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result<int> cv_ClfMilBoost_Params_getProp_numSel_const(const cv::ClfMilBoost::Params* instance) {
		try {
			int ret = instance->_numSel;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_ClfMilBoost_Params_setProp_numSel_int(cv::ClfMilBoost::Params* instance, int val) {
		try {
			instance->_numSel = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_ClfMilBoost_Params_getProp_numFeat_const(const cv::ClfMilBoost::Params* instance) {
		try {
			int ret = instance->_numFeat;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_ClfMilBoost_Params_setProp_numFeat_int(cv::ClfMilBoost::Params* instance, int val) {
		try {
			instance->_numFeat = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_ClfMilBoost_Params_getProp_lRate_const(const cv::ClfMilBoost::Params* instance) {
		try {
			float ret = instance->_lRate;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_ClfMilBoost_Params_setProp_lRate_float(cv::ClfMilBoost::Params* instance, float val) {
		try {
			instance->_lRate = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_ClfMilBoost_Params_delete(cv::ClfMilBoost::Params* instance) {
		delete instance;
	}
	Result<cv::ClfMilBoost::Params*> cv_ClfMilBoost_Params_Params() {
		try {
			cv::ClfMilBoost::Params* ret = new cv::ClfMilBoost::Params();
			return Ok<cv::ClfMilBoost::Params*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ClfMilBoost::Params*>))
	}
	
	Result<int> cv_CvFeatureParams_getPropMaxCatCount_const(const cv::CvFeatureParams* instance) {
		try {
			int ret = instance->maxCatCount;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_CvFeatureParams_setPropMaxCatCount_int(cv::CvFeatureParams* instance, int val) {
		try {
			instance->maxCatCount = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_CvFeatureParams_getPropFeatSize_const(const cv::CvFeatureParams* instance) {
		try {
			int ret = instance->featSize;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_CvFeatureParams_setPropFeatSize_int(cv::CvFeatureParams* instance, int val) {
		try {
			instance->featSize = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_CvFeatureParams_getPropNumFeatures_const(const cv::CvFeatureParams* instance) {
		try {
			int ret = instance->numFeatures;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_CvFeatureParams_setPropNumFeatures_int(cv::CvFeatureParams* instance, int val) {
		try {
			instance->numFeatures = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_CvFeatureParams_delete(cv::CvFeatureParams* instance) {
		delete instance;
	}
	Result_void cv_CvFeatureParams_init_const_CvFeatureParamsR(cv::CvFeatureParams* instance, const cv::CvFeatureParams* fp) {
		try {
			instance->init(*fp);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_CvFeatureParams_write_const_FileStorageR(const cv::CvFeatureParams* instance, cv::FileStorage* fs) {
		try {
			instance->write(*fs);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_CvFeatureParams_read_const_FileNodeR(cv::CvFeatureParams* instance, const cv::FileNode* node) {
		try {
			bool ret = instance->read(*node);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	void cv_CvHaarEvaluator_delete(cv::CvHaarEvaluator* instance) {
		delete instance;
	}
	Result_void cv_CvHaarEvaluator_init_const_CvFeatureParamsX_int_Size(cv::CvHaarEvaluator* instance, const cv::CvFeatureParams* _featureParams, int _maxSampleCount, cv::Size* _winSize) {
		try {
			instance->init(_featureParams, _maxSampleCount, *_winSize);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_CvHaarEvaluator_setImage_const_MatR_unsigned_char_int(cv::CvHaarEvaluator* instance, const cv::Mat* img, unsigned char clsLabel, int idx) {
		try {
			instance->setImage(*img, clsLabel, idx);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_CvHaarEvaluator_writeFeatures_const_FileStorageR_const_MatR(const cv::CvHaarEvaluator* instance, cv::FileStorage* fs, const cv::Mat* featureMap) {
		try {
			instance->writeFeatures(*fs, *featureMap);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::CvHaarEvaluator::FeatureHaar*> cv_CvHaarEvaluator_getFeatures_int(cv::CvHaarEvaluator* instance, int idx) {
		try {
			cv::CvHaarEvaluator::FeatureHaar ret = instance->getFeatures(idx);
			return Ok(new cv::CvHaarEvaluator::FeatureHaar(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::CvHaarEvaluator::FeatureHaar*>))
	}
	
	Result_void cv_CvHaarEvaluator_generateFeatures(cv::CvHaarEvaluator* instance) {
		try {
			instance->generateFeatures();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_CvHaarEvaluator_generateFeatures_int(cv::CvHaarEvaluator* instance, int numFeatures) {
		try {
			instance->generateFeatures(numFeatures);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_CvHaarEvaluator_FeatureHaar_delete(cv::CvHaarEvaluator::FeatureHaar* instance) {
		delete instance;
	}
	Result_void cv_CvHaarEvaluator_FeatureHaar_write_const_FileStorage(const cv::CvHaarEvaluator::FeatureHaar* instance, cv::FileStorage* unnamed) {
		try {
			instance->write(*unnamed);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<cv::Rect2d>*> cv_MultiTracker_getPropObjects(cv::MultiTracker* instance) {
		try {
			std::vector<cv::Rect2d> ret = instance->objects;
			return Ok(new std::vector<cv::Rect2d>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::Rect2d>*>))
	}
	
	Result_void cv_MultiTracker_setPropObjects_vector_Rect2d_(cv::MultiTracker* instance, std::vector<cv::Rect2d>* val) {
		try {
			instance->objects = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_MultiTracker_delete(cv::MultiTracker* instance) {
		delete instance;
	}
	Result<cv::MultiTracker*> cv_MultiTracker_MultiTracker_const_StringR(const char* trackerType) {
		try {
			cv::MultiTracker* ret = new cv::MultiTracker(cv::String(trackerType));
			return Ok<cv::MultiTracker*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MultiTracker*>))
	}
	
	Result<bool> cv_MultiTracker_add_const_MatR_const_Rect2dR(cv::MultiTracker* instance, const cv::Mat* image, const cv::Rect2d* boundingBox) {
		try {
			bool ret = instance->add(*image, *boundingBox);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_MultiTracker_add_const_StringR_const_MatR_const_Rect2dR(cv::MultiTracker* instance, const char* trackerType, const cv::Mat* image, const cv::Rect2d* boundingBox) {
		try {
			bool ret = instance->add(cv::String(trackerType), *image, *boundingBox);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_MultiTracker_add_const_StringR_const_MatR_vector_Rect2d_(cv::MultiTracker* instance, const char* trackerType, const cv::Mat* image, std::vector<cv::Rect2d>* boundingBox) {
		try {
			bool ret = instance->add(cv::String(trackerType), *image, *boundingBox);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_MultiTracker_add_const_MatR_vector_Rect2d_(cv::MultiTracker* instance, const cv::Mat* image, std::vector<cv::Rect2d>* boundingBox) {
		try {
			bool ret = instance->add(*image, *boundingBox);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_MultiTracker_update_const_MatR(cv::MultiTracker* instance, const cv::Mat* image) {
		try {
			bool ret = instance->update(*image);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_MultiTracker_update_const_MatR_vector_Rect2d_R(cv::MultiTracker* instance, const cv::Mat* image, std::vector<cv::Rect2d>* boundingBox) {
		try {
			bool ret = instance->update(*image, *boundingBox);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	void cv_MultiTrackerTLD_delete(cv::MultiTrackerTLD* instance) {
		delete instance;
	}
	Result<bool> cv_MultiTrackerTLD_update_opt_const_MatR(cv::MultiTrackerTLD* instance, const cv::Mat* image) {
		try {
			bool ret = instance->update_opt(*image);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<int> cv_MultiTracker_Alt_getPropTargetNum_const(const cv::MultiTracker_Alt* instance) {
		try {
			int ret = instance->targetNum;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_MultiTracker_Alt_setPropTargetNum_int(cv::MultiTracker_Alt* instance, int val) {
		try {
			instance->targetNum = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<cv::Ptr<cv::Tracker>>*> cv_MultiTracker_Alt_getPropTrackers(cv::MultiTracker_Alt* instance) {
		try {
			std::vector<cv::Ptr<cv::Tracker>> ret = instance->trackers;
			return Ok(new std::vector<cv::Ptr<cv::Tracker>>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::Ptr<cv::Tracker>>*>))
	}
	
	Result_void cv_MultiTracker_Alt_setPropTrackers_vector_Ptr_Tracker__(cv::MultiTracker_Alt* instance, std::vector<cv::Ptr<cv::Tracker>>* val) {
		try {
			instance->trackers = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<cv::Rect2d>*> cv_MultiTracker_Alt_getPropBoundingBoxes(cv::MultiTracker_Alt* instance) {
		try {
			std::vector<cv::Rect2d> ret = instance->boundingBoxes;
			return Ok(new std::vector<cv::Rect2d>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::Rect2d>*>))
	}
	
	Result_void cv_MultiTracker_Alt_setPropBoundingBoxes_vector_Rect2d_(cv::MultiTracker_Alt* instance, std::vector<cv::Rect2d>* val) {
		try {
			instance->boundingBoxes = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<cv::Scalar>*> cv_MultiTracker_Alt_getPropColors(cv::MultiTracker_Alt* instance) {
		try {
			std::vector<cv::Scalar> ret = instance->colors;
			return Ok(new std::vector<cv::Scalar>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::Scalar>*>))
	}
	
	Result_void cv_MultiTracker_Alt_setPropColors_vector_Scalar_(cv::MultiTracker_Alt* instance, std::vector<cv::Scalar>* val) {
		try {
			instance->colors = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_MultiTracker_Alt_delete(cv::MultiTracker_Alt* instance) {
		delete instance;
	}
	Result<cv::MultiTracker_Alt*> cv_MultiTracker_Alt_MultiTracker_Alt() {
		try {
			cv::MultiTracker_Alt* ret = new cv::MultiTracker_Alt();
			return Ok<cv::MultiTracker_Alt*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MultiTracker_Alt*>))
	}
	
	Result<bool> cv_MultiTracker_Alt_addTarget_const_MatR_const_Rect2dR_String(cv::MultiTracker_Alt* instance, const cv::Mat* image, const cv::Rect2d* boundingBox, char* tracker_algorithm_name) {
		try {
			bool ret = instance->addTarget(*image, *boundingBox, cv::String(tracker_algorithm_name));
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_MultiTracker_Alt_update_const_MatR(cv::MultiTracker_Alt* instance, const cv::Mat* image) {
		try {
			bool ret = instance->update(*image);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_Tracker_init_const_MatR_const_Rect2dR(cv::Tracker* instance, const cv::Mat* image, const cv::Rect2d* boundingBox) {
		try {
			bool ret = instance->init(*image, *boundingBox);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_Tracker_update_const_MatR_Rect2dR(cv::Tracker* instance, const cv::Mat* image, cv::Rect2d* boundingBox) {
		try {
			bool ret = instance->update(*image, *boundingBox);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<cv::Ptr<cv::Tracker>*> cv_Tracker_create_const_StringR(const char* trackerType) {
		try {
			cv::Ptr<cv::Tracker> ret = cv::Tracker::create(cv::String(trackerType));
			return Ok(new cv::Ptr<cv::Tracker>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::Tracker>*>))
	}
	
	Result_void cv_Tracker_read_const_FileNodeR(cv::Tracker* instance, const cv::FileNode* fn) {
		try {
			instance->read(*fn);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_Tracker_write_const_FileStorageR(const cv::Tracker* instance, cv::FileStorage* fs) {
		try {
			instance->write(*fs);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::TrackerModel>*> cv_Tracker_getModel(cv::Tracker* instance) {
		try {
			cv::Ptr<cv::TrackerModel> ret = instance->getModel();
			return Ok(new cv::Ptr<cv::TrackerModel>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::TrackerModel>*>))
	}
	
	Result<cv::Ptr<cv::TrackerBoosting>*> cv_TrackerBoosting_createTracker_const_ParamsR(const cv::TrackerBoosting::Params* parameters) {
		try {
			cv::Ptr<cv::TrackerBoosting> ret = cv::TrackerBoosting::createTracker(*parameters);
			return Ok(new cv::Ptr<cv::TrackerBoosting>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::TrackerBoosting>*>))
	}
	
	Result<int> cv_TrackerBoosting_Params_getPropNumClassifiers_const(const cv::TrackerBoosting::Params* instance) {
		try {
			int ret = instance->numClassifiers;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_TrackerBoosting_Params_setPropNumClassifiers_int(cv::TrackerBoosting::Params* instance, int val) {
		try {
			instance->numClassifiers = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_TrackerBoosting_Params_getPropSamplerOverlap_const(const cv::TrackerBoosting::Params* instance) {
		try {
			float ret = instance->samplerOverlap;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_TrackerBoosting_Params_setPropSamplerOverlap_float(cv::TrackerBoosting::Params* instance, float val) {
		try {
			instance->samplerOverlap = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_TrackerBoosting_Params_getPropSamplerSearchFactor_const(const cv::TrackerBoosting::Params* instance) {
		try {
			float ret = instance->samplerSearchFactor;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_TrackerBoosting_Params_setPropSamplerSearchFactor_float(cv::TrackerBoosting::Params* instance, float val) {
		try {
			instance->samplerSearchFactor = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_TrackerBoosting_Params_getPropIterationInit_const(const cv::TrackerBoosting::Params* instance) {
		try {
			int ret = instance->iterationInit;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_TrackerBoosting_Params_setPropIterationInit_int(cv::TrackerBoosting::Params* instance, int val) {
		try {
			instance->iterationInit = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_TrackerBoosting_Params_getPropFeatureSetNumFeatures_const(const cv::TrackerBoosting::Params* instance) {
		try {
			int ret = instance->featureSetNumFeatures;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_TrackerBoosting_Params_setPropFeatureSetNumFeatures_int(cv::TrackerBoosting::Params* instance, int val) {
		try {
			instance->featureSetNumFeatures = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_TrackerBoosting_Params_delete(cv::TrackerBoosting::Params* instance) {
		delete instance;
	}
	Result<cv::TrackerBoosting::Params*> cv_TrackerBoosting_Params_Params() {
		try {
			cv::TrackerBoosting::Params* ret = new cv::TrackerBoosting::Params();
			return Ok<cv::TrackerBoosting::Params*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::TrackerBoosting::Params*>))
	}
	
	Result_void cv_TrackerBoosting_Params_read_const_FileNodeR(cv::TrackerBoosting::Params* instance, const cv::FileNode* fn) {
		try {
			instance->read(*fn);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_TrackerBoosting_Params_write_const_FileStorageR(const cv::TrackerBoosting::Params* instance, cv::FileStorage* fs) {
		try {
			instance->write(*fs);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_TrackerFeature_compute_const_vector_Mat_R_MatR(cv::TrackerFeature* instance, const std::vector<cv::Mat>* images, cv::Mat* response) {
		try {
			instance->compute(*images, *response);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::TrackerFeature>*> cv_TrackerFeature_create_const_StringR(const char* trackerFeatureType) {
		try {
			cv::Ptr<cv::TrackerFeature> ret = cv::TrackerFeature::create(cv::String(trackerFeatureType));
			return Ok(new cv::Ptr<cv::TrackerFeature>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::TrackerFeature>*>))
	}
	
	Result_void cv_TrackerFeature_selection_MatR_int(cv::TrackerFeature* instance, cv::Mat* response, int npoints) {
		try {
			instance->selection(*response, npoints);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<void*> cv_TrackerFeature_getClassName_const(const cv::TrackerFeature* instance) {
		try {
			cv::String ret = instance->getClassName();
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	void cv_TrackerFeatureFeature2d_delete(cv::TrackerFeatureFeature2d* instance) {
		delete instance;
	}
	Result<cv::TrackerFeatureFeature2d*> cv_TrackerFeatureFeature2d_TrackerFeatureFeature2d_String_String(char* detectorType, char* descriptorType) {
		try {
			cv::TrackerFeatureFeature2d* ret = new cv::TrackerFeatureFeature2d(cv::String(detectorType), cv::String(descriptorType));
			return Ok<cv::TrackerFeatureFeature2d*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::TrackerFeatureFeature2d*>))
	}
	
	Result_void cv_TrackerFeatureFeature2d_selection_MatR_int(cv::TrackerFeatureFeature2d* instance, cv::Mat* response, int npoints) {
		try {
			instance->selection(*response, npoints);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_TrackerFeatureHAAR_delete(cv::TrackerFeatureHAAR* instance) {
		delete instance;
	}
	Result<cv::TrackerFeatureHAAR*> cv_TrackerFeatureHAAR_TrackerFeatureHAAR_const_ParamsR(const cv::TrackerFeatureHAAR::Params* parameters) {
		try {
			cv::TrackerFeatureHAAR* ret = new cv::TrackerFeatureHAAR(*parameters);
			return Ok<cv::TrackerFeatureHAAR*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::TrackerFeatureHAAR*>))
	}
	
	Result<bool> cv_TrackerFeatureHAAR_extractSelected_const_vector_int__const_vector_Mat_R_MatR(cv::TrackerFeatureHAAR* instance, const std::vector<int>* selFeatures, const std::vector<cv::Mat>* images, cv::Mat* response) {
		try {
			bool ret = instance->extractSelected(*selFeatures, *images, *response);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_TrackerFeatureHAAR_selection_MatR_int(cv::TrackerFeatureHAAR* instance, cv::Mat* response, int npoints) {
		try {
			instance->selection(*response, npoints);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_TrackerFeatureHAAR_swapFeature_int_int(cv::TrackerFeatureHAAR* instance, int source, int target) {
		try {
			bool ret = instance->swapFeature(source, target);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_TrackerFeatureHAAR_swapFeature_int_FeatureHaarR(cv::TrackerFeatureHAAR* instance, int id, cv::CvHaarEvaluator::FeatureHaar* feature) {
		try {
			bool ret = instance->swapFeature(id, *feature);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<cv::CvHaarEvaluator::FeatureHaar*> cv_TrackerFeatureHAAR_getFeatureAt_int(cv::TrackerFeatureHAAR* instance, int id) {
		try {
			cv::CvHaarEvaluator::FeatureHaar ret = instance->getFeatureAt(id);
			return Ok(new cv::CvHaarEvaluator::FeatureHaar(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::CvHaarEvaluator::FeatureHaar*>))
	}
	
	Result<int> cv_TrackerFeatureHAAR_Params_getPropNumFeatures_const(const cv::TrackerFeatureHAAR::Params* instance) {
		try {
			int ret = instance->numFeatures;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_TrackerFeatureHAAR_Params_setPropNumFeatures_int(cv::TrackerFeatureHAAR::Params* instance, int val) {
		try {
			instance->numFeatures = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Size> cv_TrackerFeatureHAAR_Params_getPropRectSize_const(const cv::TrackerFeatureHAAR::Params* instance) {
		try {
			cv::Size ret = instance->rectSize;
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	Result_void cv_TrackerFeatureHAAR_Params_setPropRectSize_Size(cv::TrackerFeatureHAAR::Params* instance, cv::Size* val) {
		try {
			instance->rectSize = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_TrackerFeatureHAAR_Params_getPropIsIntegral_const(const cv::TrackerFeatureHAAR::Params* instance) {
		try {
			bool ret = instance->isIntegral;
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_TrackerFeatureHAAR_Params_setPropIsIntegral_bool(cv::TrackerFeatureHAAR::Params* instance, bool val) {
		try {
			instance->isIntegral = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_TrackerFeatureHAAR_Params_delete(cv::TrackerFeatureHAAR::Params* instance) {
		delete instance;
	}
	Result<cv::TrackerFeatureHAAR::Params*> cv_TrackerFeatureHAAR_Params_Params() {
		try {
			cv::TrackerFeatureHAAR::Params* ret = new cv::TrackerFeatureHAAR::Params();
			return Ok<cv::TrackerFeatureHAAR::Params*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::TrackerFeatureHAAR::Params*>))
	}
	
	void cv_TrackerFeatureHOG_delete(cv::TrackerFeatureHOG* instance) {
		delete instance;
	}
	Result<cv::TrackerFeatureHOG*> cv_TrackerFeatureHOG_TrackerFeatureHOG() {
		try {
			cv::TrackerFeatureHOG* ret = new cv::TrackerFeatureHOG();
			return Ok<cv::TrackerFeatureHOG*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::TrackerFeatureHOG*>))
	}
	
	Result_void cv_TrackerFeatureHOG_selection_MatR_int(cv::TrackerFeatureHOG* instance, cv::Mat* response, int npoints) {
		try {
			instance->selection(*response, npoints);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_TrackerFeatureLBP_delete(cv::TrackerFeatureLBP* instance) {
		delete instance;
	}
	Result<cv::TrackerFeatureLBP*> cv_TrackerFeatureLBP_TrackerFeatureLBP() {
		try {
			cv::TrackerFeatureLBP* ret = new cv::TrackerFeatureLBP();
			return Ok<cv::TrackerFeatureLBP*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::TrackerFeatureLBP*>))
	}
	
	Result_void cv_TrackerFeatureLBP_selection_MatR_int(cv::TrackerFeatureLBP* instance, cv::Mat* response, int npoints) {
		try {
			instance->selection(*response, npoints);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_TrackerFeatureSet_delete(cv::TrackerFeatureSet* instance) {
		delete instance;
	}
	Result<cv::TrackerFeatureSet*> cv_TrackerFeatureSet_TrackerFeatureSet() {
		try {
			cv::TrackerFeatureSet* ret = new cv::TrackerFeatureSet();
			return Ok<cv::TrackerFeatureSet*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::TrackerFeatureSet*>))
	}
	
	Result_void cv_TrackerFeatureSet_extraction_const_vector_Mat_R(cv::TrackerFeatureSet* instance, const std::vector<cv::Mat>* images) {
		try {
			instance->extraction(*images);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_TrackerFeatureSet_selection(cv::TrackerFeatureSet* instance) {
		try {
			instance->selection();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_TrackerFeatureSet_removeOutliers(cv::TrackerFeatureSet* instance) {
		try {
			instance->removeOutliers();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_TrackerFeatureSet_addTrackerFeature_String(cv::TrackerFeatureSet* instance, char* trackerFeatureType) {
		try {
			bool ret = instance->addTrackerFeature(cv::String(trackerFeatureType));
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_TrackerFeatureSet_addTrackerFeature_Ptr_TrackerFeature_R(cv::TrackerFeatureSet* instance, cv::Ptr<cv::TrackerFeature>* feature) {
		try {
			bool ret = instance->addTrackerFeature(*feature);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<const std::vector<cv::Mat>*> cv_TrackerFeatureSet_getResponses_const(const cv::TrackerFeatureSet* instance) {
		try {
			const std::vector<cv::Mat> ret = instance->getResponses();
			return Ok(new const std::vector<cv::Mat>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<const std::vector<cv::Mat>*>))
	}
	
	Result<cv::Ptr<cv::TrackerGOTURN>*> cv_TrackerGOTURN_createTracker_const_ParamsR(const cv::TrackerGOTURN::Params* parameters) {
		try {
			cv::Ptr<cv::TrackerGOTURN> ret = cv::TrackerGOTURN::createTracker(*parameters);
			return Ok(new cv::Ptr<cv::TrackerGOTURN>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::TrackerGOTURN>*>))
	}
	
	void cv_TrackerGOTURN_Params_delete(cv::TrackerGOTURN::Params* instance) {
		delete instance;
	}
	Result<cv::TrackerGOTURN::Params*> cv_TrackerGOTURN_Params_Params() {
		try {
			cv::TrackerGOTURN::Params* ret = new cv::TrackerGOTURN::Params();
			return Ok<cv::TrackerGOTURN::Params*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::TrackerGOTURN::Params*>))
	}
	
	Result_void cv_TrackerGOTURN_Params_read_const_FileNodeR(cv::TrackerGOTURN::Params* instance, const cv::FileNode* unnamed) {
		try {
			instance->read(*unnamed);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_TrackerGOTURN_Params_write_const_FileStorageR(const cv::TrackerGOTURN::Params* instance, cv::FileStorage* unnamed) {
		try {
			instance->write(*unnamed);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_TrackerKCF_setFeatureExtractor_void__X__const_cv_Mat__const_cv_Rect__cv_MatR__bool(cv::TrackerKCF* instance, void (*unnamed)(const cv::Mat, const cv::Rect, cv::Mat&), bool pca_func) {
		try {
			instance->setFeatureExtractor(unnamed, pca_func);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::TrackerKCF>*> cv_TrackerKCF_createTracker_const_ParamsR(const cv::TrackerKCF::Params* parameters) {
		try {
			cv::Ptr<cv::TrackerKCF> ret = cv::TrackerKCF::createTracker(*parameters);
			return Ok(new cv::Ptr<cv::TrackerKCF>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::TrackerKCF>*>))
	}
	
	Result<double> cv_TrackerKCF_Params_getPropSigma_const(const cv::TrackerKCF::Params* instance) {
		try {
			double ret = instance->sigma;
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_TrackerKCF_Params_setPropSigma_double(cv::TrackerKCF::Params* instance, double val) {
		try {
			instance->sigma = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_TrackerKCF_Params_getPropLambda_const(const cv::TrackerKCF::Params* instance) {
		try {
			double ret = instance->lambda;
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_TrackerKCF_Params_setPropLambda_double(cv::TrackerKCF::Params* instance, double val) {
		try {
			instance->lambda = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_TrackerKCF_Params_getPropInterp_factor_const(const cv::TrackerKCF::Params* instance) {
		try {
			double ret = instance->interp_factor;
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_TrackerKCF_Params_setPropInterp_factor_double(cv::TrackerKCF::Params* instance, double val) {
		try {
			instance->interp_factor = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_TrackerKCF_Params_getPropOutput_sigma_factor_const(const cv::TrackerKCF::Params* instance) {
		try {
			double ret = instance->output_sigma_factor;
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_TrackerKCF_Params_setPropOutput_sigma_factor_double(cv::TrackerKCF::Params* instance, double val) {
		try {
			instance->output_sigma_factor = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_TrackerKCF_Params_getPropPca_learning_rate_const(const cv::TrackerKCF::Params* instance) {
		try {
			double ret = instance->pca_learning_rate;
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_TrackerKCF_Params_setPropPca_learning_rate_double(cv::TrackerKCF::Params* instance, double val) {
		try {
			instance->pca_learning_rate = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_TrackerKCF_Params_getPropResize_const(const cv::TrackerKCF::Params* instance) {
		try {
			bool ret = instance->resize;
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_TrackerKCF_Params_setPropResize_bool(cv::TrackerKCF::Params* instance, bool val) {
		try {
			instance->resize = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_TrackerKCF_Params_getPropSplit_coeff_const(const cv::TrackerKCF::Params* instance) {
		try {
			bool ret = instance->split_coeff;
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_TrackerKCF_Params_setPropSplit_coeff_bool(cv::TrackerKCF::Params* instance, bool val) {
		try {
			instance->split_coeff = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_TrackerKCF_Params_getPropWrap_kernel_const(const cv::TrackerKCF::Params* instance) {
		try {
			bool ret = instance->wrap_kernel;
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_TrackerKCF_Params_setPropWrap_kernel_bool(cv::TrackerKCF::Params* instance, bool val) {
		try {
			instance->wrap_kernel = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_TrackerKCF_Params_getPropCompress_feature_const(const cv::TrackerKCF::Params* instance) {
		try {
			bool ret = instance->compress_feature;
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_TrackerKCF_Params_setPropCompress_feature_bool(cv::TrackerKCF::Params* instance, bool val) {
		try {
			instance->compress_feature = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_TrackerKCF_Params_getPropMax_patch_size_const(const cv::TrackerKCF::Params* instance) {
		try {
			int ret = instance->max_patch_size;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_TrackerKCF_Params_setPropMax_patch_size_int(cv::TrackerKCF::Params* instance, int val) {
		try {
			instance->max_patch_size = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_TrackerKCF_Params_getPropCompressed_size_const(const cv::TrackerKCF::Params* instance) {
		try {
			int ret = instance->compressed_size;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_TrackerKCF_Params_setPropCompressed_size_int(cv::TrackerKCF::Params* instance, int val) {
		try {
			instance->compressed_size = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<unsigned int> cv_TrackerKCF_Params_getPropDesc_pca_const(const cv::TrackerKCF::Params* instance) {
		try {
			unsigned int ret = instance->desc_pca;
			return Ok<unsigned int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned int>))
	}
	
	Result_void cv_TrackerKCF_Params_setPropDesc_pca_unsigned_int(cv::TrackerKCF::Params* instance, unsigned int val) {
		try {
			instance->desc_pca = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<unsigned int> cv_TrackerKCF_Params_getPropDesc_npca_const(const cv::TrackerKCF::Params* instance) {
		try {
			unsigned int ret = instance->desc_npca;
			return Ok<unsigned int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned int>))
	}
	
	Result_void cv_TrackerKCF_Params_setPropDesc_npca_unsigned_int(cv::TrackerKCF::Params* instance, unsigned int val) {
		try {
			instance->desc_npca = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_TrackerKCF_Params_delete(cv::TrackerKCF::Params* instance) {
		delete instance;
	}
	Result<cv::TrackerKCF::Params*> cv_TrackerKCF_Params_Params() {
		try {
			cv::TrackerKCF::Params* ret = new cv::TrackerKCF::Params();
			return Ok<cv::TrackerKCF::Params*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::TrackerKCF::Params*>))
	}
	
	Result_void cv_TrackerKCF_Params_read_const_FileNodeR(cv::TrackerKCF::Params* instance, const cv::FileNode* unnamed) {
		try {
			instance->read(*unnamed);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_TrackerKCF_Params_write_const_FileStorageR(const cv::TrackerKCF::Params* instance, cv::FileStorage* unnamed) {
		try {
			instance->write(*unnamed);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::TrackerMIL>*> cv_TrackerMIL_createTracker_const_ParamsR(const cv::TrackerMIL::Params* parameters) {
		try {
			cv::Ptr<cv::TrackerMIL> ret = cv::TrackerMIL::createTracker(*parameters);
			return Ok(new cv::Ptr<cv::TrackerMIL>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::TrackerMIL>*>))
	}
	
	Result<float> cv_TrackerMIL_Params_getPropSamplerInitInRadius_const(const cv::TrackerMIL::Params* instance) {
		try {
			float ret = instance->samplerInitInRadius;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_TrackerMIL_Params_setPropSamplerInitInRadius_float(cv::TrackerMIL::Params* instance, float val) {
		try {
			instance->samplerInitInRadius = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_TrackerMIL_Params_getPropSamplerInitMaxNegNum_const(const cv::TrackerMIL::Params* instance) {
		try {
			int ret = instance->samplerInitMaxNegNum;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_TrackerMIL_Params_setPropSamplerInitMaxNegNum_int(cv::TrackerMIL::Params* instance, int val) {
		try {
			instance->samplerInitMaxNegNum = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_TrackerMIL_Params_getPropSamplerSearchWinSize_const(const cv::TrackerMIL::Params* instance) {
		try {
			float ret = instance->samplerSearchWinSize;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_TrackerMIL_Params_setPropSamplerSearchWinSize_float(cv::TrackerMIL::Params* instance, float val) {
		try {
			instance->samplerSearchWinSize = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_TrackerMIL_Params_getPropSamplerTrackInRadius_const(const cv::TrackerMIL::Params* instance) {
		try {
			float ret = instance->samplerTrackInRadius;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_TrackerMIL_Params_setPropSamplerTrackInRadius_float(cv::TrackerMIL::Params* instance, float val) {
		try {
			instance->samplerTrackInRadius = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_TrackerMIL_Params_getPropSamplerTrackMaxPosNum_const(const cv::TrackerMIL::Params* instance) {
		try {
			int ret = instance->samplerTrackMaxPosNum;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_TrackerMIL_Params_setPropSamplerTrackMaxPosNum_int(cv::TrackerMIL::Params* instance, int val) {
		try {
			instance->samplerTrackMaxPosNum = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_TrackerMIL_Params_getPropSamplerTrackMaxNegNum_const(const cv::TrackerMIL::Params* instance) {
		try {
			int ret = instance->samplerTrackMaxNegNum;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_TrackerMIL_Params_setPropSamplerTrackMaxNegNum_int(cv::TrackerMIL::Params* instance, int val) {
		try {
			instance->samplerTrackMaxNegNum = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_TrackerMIL_Params_getPropFeatureSetNumFeatures_const(const cv::TrackerMIL::Params* instance) {
		try {
			int ret = instance->featureSetNumFeatures;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_TrackerMIL_Params_setPropFeatureSetNumFeatures_int(cv::TrackerMIL::Params* instance, int val) {
		try {
			instance->featureSetNumFeatures = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_TrackerMIL_Params_delete(cv::TrackerMIL::Params* instance) {
		delete instance;
	}
	Result<cv::TrackerMIL::Params*> cv_TrackerMIL_Params_Params() {
		try {
			cv::TrackerMIL::Params* ret = new cv::TrackerMIL::Params();
			return Ok<cv::TrackerMIL::Params*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::TrackerMIL::Params*>))
	}
	
	Result_void cv_TrackerMIL_Params_read_const_FileNodeR(cv::TrackerMIL::Params* instance, const cv::FileNode* fn) {
		try {
			instance->read(*fn);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_TrackerMIL_Params_write_const_FileStorageR(const cv::TrackerMIL::Params* instance, cv::FileStorage* fs) {
		try {
			instance->write(*fs);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::TrackerMedianFlow>*> cv_TrackerMedianFlow_createTracker_const_ParamsR(const cv::TrackerMedianFlow::Params* parameters) {
		try {
			cv::Ptr<cv::TrackerMedianFlow> ret = cv::TrackerMedianFlow::createTracker(*parameters);
			return Ok(new cv::Ptr<cv::TrackerMedianFlow>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::TrackerMedianFlow>*>))
	}
	
	Result<int> cv_TrackerMedianFlow_Params_getPropPointsInGrid_const(const cv::TrackerMedianFlow::Params* instance) {
		try {
			int ret = instance->pointsInGrid;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_TrackerMedianFlow_Params_setPropPointsInGrid_int(cv::TrackerMedianFlow::Params* instance, int val) {
		try {
			instance->pointsInGrid = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_TrackerMedianFlow_Params_delete(cv::TrackerMedianFlow::Params* instance) {
		delete instance;
	}
	Result<cv::TrackerMedianFlow::Params*> cv_TrackerMedianFlow_Params_Params() {
		try {
			cv::TrackerMedianFlow::Params* ret = new cv::TrackerMedianFlow::Params();
			return Ok<cv::TrackerMedianFlow::Params*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::TrackerMedianFlow::Params*>))
	}
	
	Result_void cv_TrackerMedianFlow_Params_read_const_FileNodeR(cv::TrackerMedianFlow::Params* instance, const cv::FileNode* unnamed) {
		try {
			instance->read(*unnamed);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_TrackerMedianFlow_Params_write_const_FileStorageR(const cv::TrackerMedianFlow::Params* instance, cv::FileStorage* unnamed) {
		try {
			instance->write(*unnamed);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_TrackerModel_setTrackerStateEstimator_Ptr_TrackerStateEstimator_(cv::TrackerModel* instance, cv::Ptr<cv::TrackerStateEstimator>* trackerStateEstimator) {
		try {
			bool ret = instance->setTrackerStateEstimator(*trackerStateEstimator);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_TrackerModel_modelEstimation_const_vector_Mat_R(cv::TrackerModel* instance, const std::vector<cv::Mat>* responses) {
		try {
			instance->modelEstimation(*responses);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_TrackerModel_modelUpdate(cv::TrackerModel* instance) {
		try {
			instance->modelUpdate();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_TrackerModel_runStateEstimator(cv::TrackerModel* instance) {
		try {
			bool ret = instance->runStateEstimator();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_TrackerModel_setLastTargetState_const_Ptr_TrackerTargetState_R(cv::TrackerModel* instance, const cv::Ptr<cv::TrackerTargetState>* lastTargetState) {
		try {
			instance->setLastTargetState(*lastTargetState);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::TrackerTargetState>*> cv_TrackerModel_getLastTargetState_const(const cv::TrackerModel* instance) {
		try {
			cv::Ptr<cv::TrackerTargetState> ret = instance->getLastTargetState();
			return Ok(new cv::Ptr<cv::TrackerTargetState>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::TrackerTargetState>*>))
	}
	
	Result<cv::Ptr<cv::TrackerStateEstimator>*> cv_TrackerModel_getTrackerStateEstimator_const(const cv::TrackerModel* instance) {
		try {
			cv::Ptr<cv::TrackerStateEstimator> ret = instance->getTrackerStateEstimator();
			return Ok(new cv::Ptr<cv::TrackerStateEstimator>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::TrackerStateEstimator>*>))
	}
	
	void cv_TrackerSampler_delete(cv::TrackerSampler* instance) {
		delete instance;
	}
	Result<cv::TrackerSampler*> cv_TrackerSampler_TrackerSampler() {
		try {
			cv::TrackerSampler* ret = new cv::TrackerSampler();
			return Ok<cv::TrackerSampler*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::TrackerSampler*>))
	}
	
	Result_void cv_TrackerSampler_sampling_const_MatR_Rect(cv::TrackerSampler* instance, const cv::Mat* image, cv::Rect* boundingBox) {
		try {
			instance->sampling(*image, *boundingBox);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<const std::vector<cv::Mat>*> cv_TrackerSampler_getSamples_const(const cv::TrackerSampler* instance) {
		try {
			const std::vector<cv::Mat> ret = instance->getSamples();
			return Ok(new const std::vector<cv::Mat>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<const std::vector<cv::Mat>*>))
	}
	
	Result<bool> cv_TrackerSampler_addTrackerSamplerAlgorithm_String(cv::TrackerSampler* instance, char* trackerSamplerAlgorithmType) {
		try {
			bool ret = instance->addTrackerSamplerAlgorithm(cv::String(trackerSamplerAlgorithmType));
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_TrackerSampler_addTrackerSamplerAlgorithm_Ptr_TrackerSamplerAlgorithm_R(cv::TrackerSampler* instance, cv::Ptr<cv::TrackerSamplerAlgorithm>* sampler) {
		try {
			bool ret = instance->addTrackerSamplerAlgorithm(*sampler);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<cv::Ptr<cv::TrackerSamplerAlgorithm>*> cv_TrackerSamplerAlgorithm_create_const_StringR(const char* trackerSamplerType) {
		try {
			cv::Ptr<cv::TrackerSamplerAlgorithm> ret = cv::TrackerSamplerAlgorithm::create(cv::String(trackerSamplerType));
			return Ok(new cv::Ptr<cv::TrackerSamplerAlgorithm>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::TrackerSamplerAlgorithm>*>))
	}
	
	Result<bool> cv_TrackerSamplerAlgorithm_sampling_const_MatR_Rect_vector_Mat_R(cv::TrackerSamplerAlgorithm* instance, const cv::Mat* image, cv::Rect* boundingBox, std::vector<cv::Mat>* sample) {
		try {
			bool ret = instance->sampling(*image, *boundingBox, *sample);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<void*> cv_TrackerSamplerAlgorithm_getClassName_const(const cv::TrackerSamplerAlgorithm* instance) {
		try {
			cv::String ret = instance->getClassName();
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	void cv_TrackerSamplerCS_delete(cv::TrackerSamplerCS* instance) {
		delete instance;
	}
	Result<cv::TrackerSamplerCS*> cv_TrackerSamplerCS_TrackerSamplerCS_const_ParamsR(const cv::TrackerSamplerCS::Params* parameters) {
		try {
			cv::TrackerSamplerCS* ret = new cv::TrackerSamplerCS(*parameters);
			return Ok<cv::TrackerSamplerCS*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::TrackerSamplerCS*>))
	}
	
	Result_void cv_TrackerSamplerCS_setMode_int(cv::TrackerSamplerCS* instance, int samplingMode) {
		try {
			instance->setMode(samplingMode);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_TrackerSamplerCS_samplingImpl_const_MatR_Rect_vector_Mat_R(cv::TrackerSamplerCS* instance, const cv::Mat* image, cv::Rect* boundingBox, std::vector<cv::Mat>* sample) {
		try {
			bool ret = instance->samplingImpl(*image, *boundingBox, *sample);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<cv::Rect> cv_TrackerSamplerCS_getROI_const(const cv::TrackerSamplerCS* instance) {
		try {
			cv::Rect ret = instance->getROI();
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	Result<float> cv_TrackerSamplerCS_Params_getPropOverlap_const(const cv::TrackerSamplerCS::Params* instance) {
		try {
			float ret = instance->overlap;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_TrackerSamplerCS_Params_setPropOverlap_float(cv::TrackerSamplerCS::Params* instance, float val) {
		try {
			instance->overlap = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_TrackerSamplerCS_Params_getPropSearchFactor_const(const cv::TrackerSamplerCS::Params* instance) {
		try {
			float ret = instance->searchFactor;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_TrackerSamplerCS_Params_setPropSearchFactor_float(cv::TrackerSamplerCS::Params* instance, float val) {
		try {
			instance->searchFactor = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_TrackerSamplerCS_Params_delete(cv::TrackerSamplerCS::Params* instance) {
		delete instance;
	}
	Result<cv::TrackerSamplerCS::Params*> cv_TrackerSamplerCS_Params_Params() {
		try {
			cv::TrackerSamplerCS::Params* ret = new cv::TrackerSamplerCS::Params();
			return Ok<cv::TrackerSamplerCS::Params*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::TrackerSamplerCS::Params*>))
	}
	
	void cv_TrackerSamplerCSC_delete(cv::TrackerSamplerCSC* instance) {
		delete instance;
	}
	Result<cv::TrackerSamplerCSC*> cv_TrackerSamplerCSC_TrackerSamplerCSC_const_ParamsR(const cv::TrackerSamplerCSC::Params* parameters) {
		try {
			cv::TrackerSamplerCSC* ret = new cv::TrackerSamplerCSC(*parameters);
			return Ok<cv::TrackerSamplerCSC*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::TrackerSamplerCSC*>))
	}
	
	Result_void cv_TrackerSamplerCSC_setMode_int(cv::TrackerSamplerCSC* instance, int samplingMode) {
		try {
			instance->setMode(samplingMode);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_TrackerSamplerCSC_Params_getPropInitInRad_const(const cv::TrackerSamplerCSC::Params* instance) {
		try {
			float ret = instance->initInRad;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_TrackerSamplerCSC_Params_setPropInitInRad_float(cv::TrackerSamplerCSC::Params* instance, float val) {
		try {
			instance->initInRad = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_TrackerSamplerCSC_Params_getPropTrackInPosRad_const(const cv::TrackerSamplerCSC::Params* instance) {
		try {
			float ret = instance->trackInPosRad;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_TrackerSamplerCSC_Params_setPropTrackInPosRad_float(cv::TrackerSamplerCSC::Params* instance, float val) {
		try {
			instance->trackInPosRad = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_TrackerSamplerCSC_Params_getPropSearchWinSize_const(const cv::TrackerSamplerCSC::Params* instance) {
		try {
			float ret = instance->searchWinSize;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_TrackerSamplerCSC_Params_setPropSearchWinSize_float(cv::TrackerSamplerCSC::Params* instance, float val) {
		try {
			instance->searchWinSize = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_TrackerSamplerCSC_Params_getPropInitMaxNegNum_const(const cv::TrackerSamplerCSC::Params* instance) {
		try {
			int ret = instance->initMaxNegNum;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_TrackerSamplerCSC_Params_setPropInitMaxNegNum_int(cv::TrackerSamplerCSC::Params* instance, int val) {
		try {
			instance->initMaxNegNum = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_TrackerSamplerCSC_Params_getPropTrackMaxPosNum_const(const cv::TrackerSamplerCSC::Params* instance) {
		try {
			int ret = instance->trackMaxPosNum;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_TrackerSamplerCSC_Params_setPropTrackMaxPosNum_int(cv::TrackerSamplerCSC::Params* instance, int val) {
		try {
			instance->trackMaxPosNum = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_TrackerSamplerCSC_Params_getPropTrackMaxNegNum_const(const cv::TrackerSamplerCSC::Params* instance) {
		try {
			int ret = instance->trackMaxNegNum;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_TrackerSamplerCSC_Params_setPropTrackMaxNegNum_int(cv::TrackerSamplerCSC::Params* instance, int val) {
		try {
			instance->trackMaxNegNum = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_TrackerSamplerCSC_Params_delete(cv::TrackerSamplerCSC::Params* instance) {
		delete instance;
	}
	Result<cv::TrackerSamplerCSC::Params*> cv_TrackerSamplerCSC_Params_Params() {
		try {
			cv::TrackerSamplerCSC::Params* ret = new cv::TrackerSamplerCSC::Params();
			return Ok<cv::TrackerSamplerCSC::Params*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::TrackerSamplerCSC::Params*>))
	}
	
	void cv_TrackerSamplerPF_delete(cv::TrackerSamplerPF* instance) {
		delete instance;
	}
	Result<cv::TrackerSamplerPF*> cv_TrackerSamplerPF_TrackerSamplerPF_const_MatR_const_ParamsR(const cv::Mat* chosenRect, const cv::TrackerSamplerPF::Params* parameters) {
		try {
			cv::TrackerSamplerPF* ret = new cv::TrackerSamplerPF(*chosenRect, *parameters);
			return Ok<cv::TrackerSamplerPF*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::TrackerSamplerPF*>))
	}
	
	Result<int> cv_TrackerSamplerPF_Params_getPropIterationNum_const(const cv::TrackerSamplerPF::Params* instance) {
		try {
			int ret = instance->iterationNum;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_TrackerSamplerPF_Params_setPropIterationNum_int(cv::TrackerSamplerPF::Params* instance, int val) {
		try {
			instance->iterationNum = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_TrackerSamplerPF_Params_getPropParticlesNum_const(const cv::TrackerSamplerPF::Params* instance) {
		try {
			int ret = instance->particlesNum;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_TrackerSamplerPF_Params_setPropParticlesNum_int(cv::TrackerSamplerPF::Params* instance, int val) {
		try {
			instance->particlesNum = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_TrackerSamplerPF_Params_getPropAlpha_const(const cv::TrackerSamplerPF::Params* instance) {
		try {
			double ret = instance->alpha;
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_TrackerSamplerPF_Params_setPropAlpha_double(cv::TrackerSamplerPF::Params* instance, double val) {
		try {
			instance->alpha = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat_<double>*> cv_TrackerSamplerPF_Params_getPropStd(cv::TrackerSamplerPF::Params* instance) {
		try {
			cv::Mat_<double> ret = instance->std;
			return Ok(new cv::Mat_<double>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat_<double>*>))
	}
	
	Result_void cv_TrackerSamplerPF_Params_setPropStd_Mat__double_(cv::TrackerSamplerPF::Params* instance, cv::Mat_<double>* val) {
		try {
			instance->std = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_TrackerSamplerPF_Params_delete(cv::TrackerSamplerPF::Params* instance) {
		delete instance;
	}
	Result<cv::TrackerSamplerPF::Params*> cv_TrackerSamplerPF_Params_Params() {
		try {
			cv::TrackerSamplerPF::Params* ret = new cv::TrackerSamplerPF::Params();
			return Ok<cv::TrackerSamplerPF::Params*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::TrackerSamplerPF::Params*>))
	}
	
	Result<cv::Ptr<cv::TrackerStateEstimator>*> cv_TrackerStateEstimator_create_const_StringR(const char* trackeStateEstimatorType) {
		try {
			cv::Ptr<cv::TrackerStateEstimator> ret = cv::TrackerStateEstimator::create(cv::String(trackeStateEstimatorType));
			return Ok(new cv::Ptr<cv::TrackerStateEstimator>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::TrackerStateEstimator>*>))
	}
	
	Result<void*> cv_TrackerStateEstimator_getClassName_const(const cv::TrackerStateEstimator* instance) {
		try {
			cv::String ret = instance->getClassName();
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	void cv_TrackerStateEstimatorAdaBoosting_delete(cv::TrackerStateEstimatorAdaBoosting* instance) {
		delete instance;
	}
	Result<cv::TrackerStateEstimatorAdaBoosting*> cv_TrackerStateEstimatorAdaBoosting_TrackerStateEstimatorAdaBoosting_int_int_int_Size_const_RectR(int numClassifer, int initIterations, int nFeatures, cv::Size* patchSize, const cv::Rect* ROI) {
		try {
			cv::TrackerStateEstimatorAdaBoosting* ret = new cv::TrackerStateEstimatorAdaBoosting(numClassifer, initIterations, nFeatures, *patchSize, *ROI);
			return Ok<cv::TrackerStateEstimatorAdaBoosting*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::TrackerStateEstimatorAdaBoosting*>))
	}
	
	Result<cv::Rect> cv_TrackerStateEstimatorAdaBoosting_getSampleROI_const(const cv::TrackerStateEstimatorAdaBoosting* instance) {
		try {
			cv::Rect ret = instance->getSampleROI();
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	Result_void cv_TrackerStateEstimatorAdaBoosting_setSampleROI_const_RectR(cv::TrackerStateEstimatorAdaBoosting* instance, const cv::Rect* ROI) {
		try {
			instance->setSampleROI(*ROI);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<int>*> cv_TrackerStateEstimatorAdaBoosting_computeSelectedWeakClassifier(cv::TrackerStateEstimatorAdaBoosting* instance) {
		try {
			std::vector<int> ret = instance->computeSelectedWeakClassifier();
			return Ok(new std::vector<int>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<int>*>))
	}
	
	Result<std::vector<int>*> cv_TrackerStateEstimatorAdaBoosting_computeReplacedClassifier(cv::TrackerStateEstimatorAdaBoosting* instance) {
		try {
			std::vector<int> ret = instance->computeReplacedClassifier();
			return Ok(new std::vector<int>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<int>*>))
	}
	
	Result<std::vector<int>*> cv_TrackerStateEstimatorAdaBoosting_computeSwappedClassifier(cv::TrackerStateEstimatorAdaBoosting* instance) {
		try {
			std::vector<int> ret = instance->computeSwappedClassifier();
			return Ok(new std::vector<int>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<int>*>))
	}
	
	void cv_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState_delete(cv::TrackerStateEstimatorAdaBoosting::TrackerAdaBoostingTargetState* instance) {
		delete instance;
	}
	#if !defined(OCVRS_TARGET_OS_WINDOWS)
	Result<cv::TrackerStateEstimatorAdaBoosting::TrackerAdaBoostingTargetState*> cv_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState_TrackerAdaBoostingTargetState_const_Point2fR_int_int_bool_const_MatR(const cv::Point2f* position, int width, int height, bool foreground, const cv::Mat* responses) {
		try {
			cv::TrackerStateEstimatorAdaBoosting::TrackerAdaBoostingTargetState* ret = new cv::TrackerStateEstimatorAdaBoosting::TrackerAdaBoostingTargetState(*position, width, height, foreground, *responses);
			return Ok<cv::TrackerStateEstimatorAdaBoosting::TrackerAdaBoostingTargetState*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::TrackerStateEstimatorAdaBoosting::TrackerAdaBoostingTargetState*>))
	}
	#endif
	
	#if !defined(OCVRS_TARGET_OS_WINDOWS)
	Result_void cv_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState_setTargetResponses_const_MatR(cv::TrackerStateEstimatorAdaBoosting::TrackerAdaBoostingTargetState* instance, const cv::Mat* responses) {
		try {
			instance->setTargetResponses(*responses);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	#endif
	
	#if !defined(OCVRS_TARGET_OS_WINDOWS)
	Result_void cv_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState_setTargetFg_bool(cv::TrackerStateEstimatorAdaBoosting::TrackerAdaBoostingTargetState* instance, bool foreground) {
		try {
			instance->setTargetFg(foreground);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	#endif
	
	#if !defined(OCVRS_TARGET_OS_WINDOWS)
	Result<cv::Mat*> cv_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState_getTargetResponses_const(const cv::TrackerStateEstimatorAdaBoosting::TrackerAdaBoostingTargetState* instance) {
		try {
			cv::Mat ret = instance->getTargetResponses();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	#endif
	
	#if !defined(OCVRS_TARGET_OS_WINDOWS)
	Result<bool> cv_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState_isTargetFg_const(const cv::TrackerStateEstimatorAdaBoosting::TrackerAdaBoostingTargetState* instance) {
		try {
			bool ret = instance->isTargetFg();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	#endif
	
	void cv_TrackerStateEstimatorMILBoosting_delete(cv::TrackerStateEstimatorMILBoosting* instance) {
		delete instance;
	}
	Result<cv::TrackerStateEstimatorMILBoosting*> cv_TrackerStateEstimatorMILBoosting_TrackerStateEstimatorMILBoosting_int(int nFeatures) {
		try {
			cv::TrackerStateEstimatorMILBoosting* ret = new cv::TrackerStateEstimatorMILBoosting(nFeatures);
			return Ok<cv::TrackerStateEstimatorMILBoosting*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::TrackerStateEstimatorMILBoosting*>))
	}
	
	void cv_TrackerStateEstimatorMILBoosting_TrackerMILTargetState_delete(cv::TrackerStateEstimatorMILBoosting::TrackerMILTargetState* instance) {
		delete instance;
	}
	#if !defined(OCVRS_TARGET_OS_WINDOWS)
	Result<cv::TrackerStateEstimatorMILBoosting::TrackerMILTargetState*> cv_TrackerStateEstimatorMILBoosting_TrackerMILTargetState_TrackerMILTargetState_const_Point2fR_int_int_bool_const_MatR(const cv::Point2f* position, int width, int height, bool foreground, const cv::Mat* features) {
		try {
			cv::TrackerStateEstimatorMILBoosting::TrackerMILTargetState* ret = new cv::TrackerStateEstimatorMILBoosting::TrackerMILTargetState(*position, width, height, foreground, *features);
			return Ok<cv::TrackerStateEstimatorMILBoosting::TrackerMILTargetState*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::TrackerStateEstimatorMILBoosting::TrackerMILTargetState*>))
	}
	#endif
	
	#if !defined(OCVRS_TARGET_OS_WINDOWS)
	Result_void cv_TrackerStateEstimatorMILBoosting_TrackerMILTargetState_setTargetFg_bool(cv::TrackerStateEstimatorMILBoosting::TrackerMILTargetState* instance, bool foreground) {
		try {
			instance->setTargetFg(foreground);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	#endif
	
	#if !defined(OCVRS_TARGET_OS_WINDOWS)
	Result_void cv_TrackerStateEstimatorMILBoosting_TrackerMILTargetState_setFeatures_const_MatR(cv::TrackerStateEstimatorMILBoosting::TrackerMILTargetState* instance, const cv::Mat* features) {
		try {
			instance->setFeatures(*features);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	#endif
	
	#if !defined(OCVRS_TARGET_OS_WINDOWS)
	Result<bool> cv_TrackerStateEstimatorMILBoosting_TrackerMILTargetState_isTargetFg_const(const cv::TrackerStateEstimatorMILBoosting::TrackerMILTargetState* instance) {
		try {
			bool ret = instance->isTargetFg();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	#endif
	
	#if !defined(OCVRS_TARGET_OS_WINDOWS)
	Result<cv::Mat*> cv_TrackerStateEstimatorMILBoosting_TrackerMILTargetState_getFeatures_const(const cv::TrackerStateEstimatorMILBoosting::TrackerMILTargetState* instance) {
		try {
			cv::Mat ret = instance->getFeatures();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	#endif
	
	void cv_TrackerStateEstimatorSVM_delete(cv::TrackerStateEstimatorSVM* instance) {
		delete instance;
	}
	Result<cv::TrackerStateEstimatorSVM*> cv_TrackerStateEstimatorSVM_TrackerStateEstimatorSVM() {
		try {
			cv::TrackerStateEstimatorSVM* ret = new cv::TrackerStateEstimatorSVM();
			return Ok<cv::TrackerStateEstimatorSVM*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::TrackerStateEstimatorSVM*>))
	}
	
	Result<cv::Ptr<cv::TrackerTLD>*> cv_TrackerTLD_createTracker_const_ParamsR(const cv::TrackerTLD::Params* parameters) {
		try {
			cv::Ptr<cv::TrackerTLD> ret = cv::TrackerTLD::createTracker(*parameters);
			return Ok(new cv::Ptr<cv::TrackerTLD>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::TrackerTLD>*>))
	}
	
	void cv_TrackerTLD_Params_delete(cv::TrackerTLD::Params* instance) {
		delete instance;
	}
	Result<cv::TrackerTLD::Params*> cv_TrackerTLD_Params_Params() {
		try {
			cv::TrackerTLD::Params* ret = new cv::TrackerTLD::Params();
			return Ok<cv::TrackerTLD::Params*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::TrackerTLD::Params*>))
	}
	
	Result_void cv_TrackerTLD_Params_read_const_FileNodeR(cv::TrackerTLD::Params* instance, const cv::FileNode* unnamed) {
		try {
			instance->read(*unnamed);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_TrackerTLD_Params_write_const_FileStorageR(const cv::TrackerTLD::Params* instance, cv::FileStorage* unnamed) {
		try {
			instance->write(*unnamed);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_TrackerTargetState_delete(cv::TrackerTargetState* instance) {
		delete instance;
	}
	Result<cv::Point2f> cv_TrackerTargetState_getTargetPosition_const(const cv::TrackerTargetState* instance) {
		try {
			cv::Point2f ret = instance->getTargetPosition();
			return Ok<cv::Point2f>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point2f>))
	}
	
	Result_void cv_TrackerTargetState_setTargetPosition_const_Point2fR(cv::TrackerTargetState* instance, const cv::Point2f* position) {
		try {
			instance->setTargetPosition(*position);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_TrackerTargetState_getTargetWidth_const(const cv::TrackerTargetState* instance) {
		try {
			int ret = instance->getTargetWidth();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_TrackerTargetState_setTargetWidth_int(cv::TrackerTargetState* instance, int width) {
		try {
			instance->setTargetWidth(width);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_TrackerTargetState_getTargetHeight_const(const cv::TrackerTargetState* instance) {
		try {
			int ret = instance->getTargetHeight();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_TrackerTargetState_setTargetHeight_int(cv::TrackerTargetState* instance, int height) {
		try {
			instance->setTargetHeight(height);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
}
