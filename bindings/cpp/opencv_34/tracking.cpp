#include "ocvrs_common.hpp"
#include <opencv2/tracking.hpp>
#include "tracking_types.hpp"

extern "C" {
	Result<cv::Rect2d> cv_tld_tld_InitDataset_int_const_charX_int(int videoInd, const char* rootPath, int datasetInd) {
		try {
			cv::Rect2d ret = cv::tld::tld_InitDataset(videoInd, rootPath, datasetInd);
			return Ok<cv::Rect2d>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect2d>))
	}
	
	Result<void*> cv_tld_tld_getNextDatasetFrame() {
		try {
			cv::String ret = cv::tld::tld_getNextDatasetFrame();
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
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
	
	void cv_MultiTracker_delete(cv::MultiTracker* instance) {
		delete instance;
	}
	Result<cv::MultiTracker*> cv_MultiTracker_MultiTracker() {
		try {
			cv::MultiTracker* ret = new cv::MultiTracker();
			return Ok<cv::MultiTracker*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MultiTracker*>))
	}
	
	Result<bool> cv_MultiTracker_add_Ptr_Tracker__const__InputArrayR_const_Rect2dR(cv::MultiTracker* instance, cv::Ptr<cv::Tracker>* newTracker, const cv::_InputArray* image, const cv::Rect2d* boundingBox) {
		try {
			bool ret = instance->add(*newTracker, *image, *boundingBox);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_MultiTracker_add_vector_Ptr_Tracker___const__InputArrayR_vector_Rect2d_(cv::MultiTracker* instance, std::vector<cv::Ptr<cv::Tracker>>* newTrackers, const cv::_InputArray* image, std::vector<cv::Rect2d>* boundingBox) {
		try {
			bool ret = instance->add(*newTrackers, *image, *boundingBox);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_MultiTracker_update_const__InputArrayR(cv::MultiTracker* instance, const cv::_InputArray* image) {
		try {
			bool ret = instance->update(*image);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_MultiTracker_update_const__InputArrayR_vector_Rect2d_R(cv::MultiTracker* instance, const cv::_InputArray* image, std::vector<cv::Rect2d>* boundingBox) {
		try {
			bool ret = instance->update(*image, *boundingBox);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<const std::vector<cv::Rect2d>*> cv_MultiTracker_getObjects_const(const cv::MultiTracker* instance) {
		try {
			const std::vector<cv::Rect2d> ret = instance->getObjects();
			return Ok(new const std::vector<cv::Rect2d>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<const std::vector<cv::Rect2d>*>))
	}
	
	Result<cv::Ptr<cv::MultiTracker>*> cv_MultiTracker_create() {
		try {
			cv::Ptr<cv::MultiTracker> ret = cv::MultiTracker::create();
			return Ok(new cv::Ptr<cv::MultiTracker>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::MultiTracker>*>))
	}
	
	void cv_MultiTrackerTLD_delete(cv::MultiTrackerTLD* instance) {
		delete instance;
	}
	Result<bool> cv_MultiTrackerTLD_update_opt_const__InputArrayR(cv::MultiTrackerTLD* instance, const cv::_InputArray* image) {
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
	
	Result<bool> cv_MultiTracker_Alt_addTarget_const__InputArrayR_const_Rect2dR_Ptr_Tracker_(cv::MultiTracker_Alt* instance, const cv::_InputArray* image, const cv::Rect2d* boundingBox, cv::Ptr<cv::Tracker>* tracker_algorithm) {
		try {
			bool ret = instance->addTarget(*image, *boundingBox, *tracker_algorithm);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_MultiTracker_Alt_update_const__InputArrayR(cv::MultiTracker_Alt* instance, const cv::_InputArray* image) {
		try {
			bool ret = instance->update(*image);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_Tracker_init_const__InputArrayR_const_Rect2dR(cv::Tracker* instance, const cv::_InputArray* image, const cv::Rect2d* boundingBox) {
		try {
			bool ret = instance->init(*image, *boundingBox);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_Tracker_update_const__InputArrayR_Rect2dR(cv::Tracker* instance, const cv::_InputArray* image, cv::Rect2d* boundingBox) {
		try {
			bool ret = instance->update(*image, *boundingBox);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
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
	
	Result<cv::Ptr<cv::TrackerBoosting>*> cv_TrackerBoosting_create_const_ParamsR(const cv::TrackerBoosting::Params* parameters) {
		try {
			cv::Ptr<cv::TrackerBoosting> ret = cv::TrackerBoosting::create(*parameters);
			return Ok(new cv::Ptr<cv::TrackerBoosting>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::TrackerBoosting>*>))
	}
	
	Result<cv::Ptr<cv::TrackerBoosting>*> cv_TrackerBoosting_create() {
		try {
			cv::Ptr<cv::TrackerBoosting> ret = cv::TrackerBoosting::create();
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
	
	Result<cv::Ptr<cv::TrackerCSRT>*> cv_TrackerCSRT_create_const_ParamsR(const cv::TrackerCSRT::Params* parameters) {
		try {
			cv::Ptr<cv::TrackerCSRT> ret = cv::TrackerCSRT::create(*parameters);
			return Ok(new cv::Ptr<cv::TrackerCSRT>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::TrackerCSRT>*>))
	}
	
	Result<cv::Ptr<cv::TrackerCSRT>*> cv_TrackerCSRT_create() {
		try {
			cv::Ptr<cv::TrackerCSRT> ret = cv::TrackerCSRT::create();
			return Ok(new cv::Ptr<cv::TrackerCSRT>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::TrackerCSRT>*>))
	}
	
	Result_void cv_TrackerCSRT_setInitialMask_const_Mat(cv::TrackerCSRT* instance, const cv::Mat* mask) {
		try {
			instance->setInitialMask(*mask);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_TrackerCSRT_Params_getPropUse_hog_const(const cv::TrackerCSRT::Params* instance) {
		try {
			bool ret = instance->use_hog;
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_TrackerCSRT_Params_setPropUse_hog_bool(cv::TrackerCSRT::Params* instance, bool val) {
		try {
			instance->use_hog = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_TrackerCSRT_Params_getPropUse_color_names_const(const cv::TrackerCSRT::Params* instance) {
		try {
			bool ret = instance->use_color_names;
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_TrackerCSRT_Params_setPropUse_color_names_bool(cv::TrackerCSRT::Params* instance, bool val) {
		try {
			instance->use_color_names = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_TrackerCSRT_Params_getPropUse_gray_const(const cv::TrackerCSRT::Params* instance) {
		try {
			bool ret = instance->use_gray;
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_TrackerCSRT_Params_setPropUse_gray_bool(cv::TrackerCSRT::Params* instance, bool val) {
		try {
			instance->use_gray = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_TrackerCSRT_Params_getPropUse_rgb_const(const cv::TrackerCSRT::Params* instance) {
		try {
			bool ret = instance->use_rgb;
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_TrackerCSRT_Params_setPropUse_rgb_bool(cv::TrackerCSRT::Params* instance, bool val) {
		try {
			instance->use_rgb = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_TrackerCSRT_Params_getPropUse_channel_weights_const(const cv::TrackerCSRT::Params* instance) {
		try {
			bool ret = instance->use_channel_weights;
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_TrackerCSRT_Params_setPropUse_channel_weights_bool(cv::TrackerCSRT::Params* instance, bool val) {
		try {
			instance->use_channel_weights = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_TrackerCSRT_Params_getPropUse_segmentation_const(const cv::TrackerCSRT::Params* instance) {
		try {
			bool ret = instance->use_segmentation;
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_TrackerCSRT_Params_setPropUse_segmentation_bool(cv::TrackerCSRT::Params* instance, bool val) {
		try {
			instance->use_segmentation = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<void*> cv_TrackerCSRT_Params_getPropWindow_function_const(const cv::TrackerCSRT::Params* instance) {
		try {
			std::string ret = instance->window_function;
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result_void cv_TrackerCSRT_Params_setPropWindow_function_string(cv::TrackerCSRT::Params* instance, char* val) {
		try {
			instance->window_function = std::string(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_TrackerCSRT_Params_getPropKaiser_alpha_const(const cv::TrackerCSRT::Params* instance) {
		try {
			float ret = instance->kaiser_alpha;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_TrackerCSRT_Params_setPropKaiser_alpha_float(cv::TrackerCSRT::Params* instance, float val) {
		try {
			instance->kaiser_alpha = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_TrackerCSRT_Params_getPropCheb_attenuation_const(const cv::TrackerCSRT::Params* instance) {
		try {
			float ret = instance->cheb_attenuation;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_TrackerCSRT_Params_setPropCheb_attenuation_float(cv::TrackerCSRT::Params* instance, float val) {
		try {
			instance->cheb_attenuation = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_TrackerCSRT_Params_getPropTemplate_size_const(const cv::TrackerCSRT::Params* instance) {
		try {
			float ret = instance->template_size;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_TrackerCSRT_Params_setPropTemplate_size_float(cv::TrackerCSRT::Params* instance, float val) {
		try {
			instance->template_size = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_TrackerCSRT_Params_getPropGsl_sigma_const(const cv::TrackerCSRT::Params* instance) {
		try {
			float ret = instance->gsl_sigma;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_TrackerCSRT_Params_setPropGsl_sigma_float(cv::TrackerCSRT::Params* instance, float val) {
		try {
			instance->gsl_sigma = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_TrackerCSRT_Params_getPropHog_orientations_const(const cv::TrackerCSRT::Params* instance) {
		try {
			float ret = instance->hog_orientations;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_TrackerCSRT_Params_setPropHog_orientations_float(cv::TrackerCSRT::Params* instance, float val) {
		try {
			instance->hog_orientations = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_TrackerCSRT_Params_getPropHog_clip_const(const cv::TrackerCSRT::Params* instance) {
		try {
			float ret = instance->hog_clip;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_TrackerCSRT_Params_setPropHog_clip_float(cv::TrackerCSRT::Params* instance, float val) {
		try {
			instance->hog_clip = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_TrackerCSRT_Params_getPropPadding_const(const cv::TrackerCSRT::Params* instance) {
		try {
			float ret = instance->padding;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_TrackerCSRT_Params_setPropPadding_float(cv::TrackerCSRT::Params* instance, float val) {
		try {
			instance->padding = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_TrackerCSRT_Params_getPropFilter_lr_const(const cv::TrackerCSRT::Params* instance) {
		try {
			float ret = instance->filter_lr;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_TrackerCSRT_Params_setPropFilter_lr_float(cv::TrackerCSRT::Params* instance, float val) {
		try {
			instance->filter_lr = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_TrackerCSRT_Params_getPropWeights_lr_const(const cv::TrackerCSRT::Params* instance) {
		try {
			float ret = instance->weights_lr;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_TrackerCSRT_Params_setPropWeights_lr_float(cv::TrackerCSRT::Params* instance, float val) {
		try {
			instance->weights_lr = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_TrackerCSRT_Params_getPropNum_hog_channels_used_const(const cv::TrackerCSRT::Params* instance) {
		try {
			int ret = instance->num_hog_channels_used;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_TrackerCSRT_Params_setPropNum_hog_channels_used_int(cv::TrackerCSRT::Params* instance, int val) {
		try {
			instance->num_hog_channels_used = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_TrackerCSRT_Params_getPropAdmm_iterations_const(const cv::TrackerCSRT::Params* instance) {
		try {
			int ret = instance->admm_iterations;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_TrackerCSRT_Params_setPropAdmm_iterations_int(cv::TrackerCSRT::Params* instance, int val) {
		try {
			instance->admm_iterations = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_TrackerCSRT_Params_getPropHistogram_bins_const(const cv::TrackerCSRT::Params* instance) {
		try {
			int ret = instance->histogram_bins;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_TrackerCSRT_Params_setPropHistogram_bins_int(cv::TrackerCSRT::Params* instance, int val) {
		try {
			instance->histogram_bins = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_TrackerCSRT_Params_getPropHistogram_lr_const(const cv::TrackerCSRT::Params* instance) {
		try {
			float ret = instance->histogram_lr;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_TrackerCSRT_Params_setPropHistogram_lr_float(cv::TrackerCSRT::Params* instance, float val) {
		try {
			instance->histogram_lr = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_TrackerCSRT_Params_getPropBackground_ratio_const(const cv::TrackerCSRT::Params* instance) {
		try {
			int ret = instance->background_ratio;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_TrackerCSRT_Params_setPropBackground_ratio_int(cv::TrackerCSRT::Params* instance, int val) {
		try {
			instance->background_ratio = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_TrackerCSRT_Params_getPropNumber_of_scales_const(const cv::TrackerCSRT::Params* instance) {
		try {
			int ret = instance->number_of_scales;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_TrackerCSRT_Params_setPropNumber_of_scales_int(cv::TrackerCSRT::Params* instance, int val) {
		try {
			instance->number_of_scales = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_TrackerCSRT_Params_getPropScale_sigma_factor_const(const cv::TrackerCSRT::Params* instance) {
		try {
			float ret = instance->scale_sigma_factor;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_TrackerCSRT_Params_setPropScale_sigma_factor_float(cv::TrackerCSRT::Params* instance, float val) {
		try {
			instance->scale_sigma_factor = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_TrackerCSRT_Params_getPropScale_model_max_area_const(const cv::TrackerCSRT::Params* instance) {
		try {
			float ret = instance->scale_model_max_area;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_TrackerCSRT_Params_setPropScale_model_max_area_float(cv::TrackerCSRT::Params* instance, float val) {
		try {
			instance->scale_model_max_area = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_TrackerCSRT_Params_getPropScale_lr_const(const cv::TrackerCSRT::Params* instance) {
		try {
			float ret = instance->scale_lr;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_TrackerCSRT_Params_setPropScale_lr_float(cv::TrackerCSRT::Params* instance, float val) {
		try {
			instance->scale_lr = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_TrackerCSRT_Params_getPropScale_step_const(const cv::TrackerCSRT::Params* instance) {
		try {
			float ret = instance->scale_step;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_TrackerCSRT_Params_setPropScale_step_float(cv::TrackerCSRT::Params* instance, float val) {
		try {
			instance->scale_step = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_TrackerCSRT_Params_getPropPsr_threshold_const(const cv::TrackerCSRT::Params* instance) {
		try {
			float ret = instance->psr_threshold;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_TrackerCSRT_Params_setPropPsr_threshold_float(cv::TrackerCSRT::Params* instance, float val) {
		try {
			instance->psr_threshold = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_TrackerCSRT_Params_delete(cv::TrackerCSRT::Params* instance) {
		delete instance;
	}
	Result<cv::TrackerCSRT::Params*> cv_TrackerCSRT_Params_Params() {
		try {
			cv::TrackerCSRT::Params* ret = new cv::TrackerCSRT::Params();
			return Ok<cv::TrackerCSRT::Params*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::TrackerCSRT::Params*>))
	}
	
	Result_void cv_TrackerCSRT_Params_read_const_FileNodeR(cv::TrackerCSRT::Params* instance, const cv::FileNode* unnamed) {
		try {
			instance->read(*unnamed);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_TrackerCSRT_Params_write_const_FileStorageR(const cv::TrackerCSRT::Params* instance, cv::FileStorage* fs) {
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
	
	Result<cv::Ptr<cv::TrackerGOTURN>*> cv_TrackerGOTURN_create_const_ParamsR(const cv::TrackerGOTURN::Params* parameters) {
		try {
			cv::Ptr<cv::TrackerGOTURN> ret = cv::TrackerGOTURN::create(*parameters);
			return Ok(new cv::Ptr<cv::TrackerGOTURN>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::TrackerGOTURN>*>))
	}
	
	Result<cv::Ptr<cv::TrackerGOTURN>*> cv_TrackerGOTURN_create() {
		try {
			cv::Ptr<cv::TrackerGOTURN> ret = cv::TrackerGOTURN::create();
			return Ok(new cv::Ptr<cv::TrackerGOTURN>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::TrackerGOTURN>*>))
	}
	
	Result<void*> cv_TrackerGOTURN_Params_getPropModelTxt_const(const cv::TrackerGOTURN::Params* instance) {
		try {
			cv::String ret = instance->modelTxt;
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result_void cv_TrackerGOTURN_Params_setPropModelTxt_String(cv::TrackerGOTURN::Params* instance, char* val) {
		try {
			instance->modelTxt = cv::String(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<void*> cv_TrackerGOTURN_Params_getPropModelBin_const(const cv::TrackerGOTURN::Params* instance) {
		try {
			cv::String ret = instance->modelBin;
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result_void cv_TrackerGOTURN_Params_setPropModelBin_String(cv::TrackerGOTURN::Params* instance, char* val) {
		try {
			instance->modelBin = cv::String(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
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
	
	Result<cv::Ptr<cv::TrackerKCF>*> cv_TrackerKCF_create_const_ParamsR(const cv::TrackerKCF::Params* parameters) {
		try {
			cv::Ptr<cv::TrackerKCF> ret = cv::TrackerKCF::create(*parameters);
			return Ok(new cv::Ptr<cv::TrackerKCF>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::TrackerKCF>*>))
	}
	
	Result<cv::Ptr<cv::TrackerKCF>*> cv_TrackerKCF_create() {
		try {
			cv::Ptr<cv::TrackerKCF> ret = cv::TrackerKCF::create();
			return Ok(new cv::Ptr<cv::TrackerKCF>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::TrackerKCF>*>))
	}
	
	Result<float> cv_TrackerKCF_Params_getPropDetect_thresh_const(const cv::TrackerKCF::Params* instance) {
		try {
			float ret = instance->detect_thresh;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_TrackerKCF_Params_setPropDetect_thresh_float(cv::TrackerKCF::Params* instance, float val) {
		try {
			instance->detect_thresh = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_TrackerKCF_Params_getPropSigma_const(const cv::TrackerKCF::Params* instance) {
		try {
			float ret = instance->sigma;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_TrackerKCF_Params_setPropSigma_float(cv::TrackerKCF::Params* instance, float val) {
		try {
			instance->sigma = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_TrackerKCF_Params_getPropLambda_const(const cv::TrackerKCF::Params* instance) {
		try {
			float ret = instance->lambda;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_TrackerKCF_Params_setPropLambda_float(cv::TrackerKCF::Params* instance, float val) {
		try {
			instance->lambda = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_TrackerKCF_Params_getPropInterp_factor_const(const cv::TrackerKCF::Params* instance) {
		try {
			float ret = instance->interp_factor;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_TrackerKCF_Params_setPropInterp_factor_float(cv::TrackerKCF::Params* instance, float val) {
		try {
			instance->interp_factor = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_TrackerKCF_Params_getPropOutput_sigma_factor_const(const cv::TrackerKCF::Params* instance) {
		try {
			float ret = instance->output_sigma_factor;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_TrackerKCF_Params_setPropOutput_sigma_factor_float(cv::TrackerKCF::Params* instance, float val) {
		try {
			instance->output_sigma_factor = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_TrackerKCF_Params_getPropPca_learning_rate_const(const cv::TrackerKCF::Params* instance) {
		try {
			float ret = instance->pca_learning_rate;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_TrackerKCF_Params_setPropPca_learning_rate_float(cv::TrackerKCF::Params* instance, float val) {
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
	
	Result<int> cv_TrackerKCF_Params_getPropDesc_pca_const(const cv::TrackerKCF::Params* instance) {
		try {
			int ret = instance->desc_pca;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_TrackerKCF_Params_setPropDesc_pca_int(cv::TrackerKCF::Params* instance, int val) {
		try {
			instance->desc_pca = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_TrackerKCF_Params_getPropDesc_npca_const(const cv::TrackerKCF::Params* instance) {
		try {
			int ret = instance->desc_npca;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_TrackerKCF_Params_setPropDesc_npca_int(cv::TrackerKCF::Params* instance, int val) {
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
	
	Result<cv::Ptr<cv::TrackerMIL>*> cv_TrackerMIL_create_const_ParamsR(const cv::TrackerMIL::Params* parameters) {
		try {
			cv::Ptr<cv::TrackerMIL> ret = cv::TrackerMIL::create(*parameters);
			return Ok(new cv::Ptr<cv::TrackerMIL>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::TrackerMIL>*>))
	}
	
	Result<cv::Ptr<cv::TrackerMIL>*> cv_TrackerMIL_create() {
		try {
			cv::Ptr<cv::TrackerMIL> ret = cv::TrackerMIL::create();
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
	
	Result<cv::Ptr<cv::TrackerMOSSE>*> cv_TrackerMOSSE_create() {
		try {
			cv::Ptr<cv::TrackerMOSSE> ret = cv::TrackerMOSSE::create();
			return Ok(new cv::Ptr<cv::TrackerMOSSE>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::TrackerMOSSE>*>))
	}
	
	Result<cv::Ptr<cv::TrackerMedianFlow>*> cv_TrackerMedianFlow_create_const_ParamsR(const cv::TrackerMedianFlow::Params* parameters) {
		try {
			cv::Ptr<cv::TrackerMedianFlow> ret = cv::TrackerMedianFlow::create(*parameters);
			return Ok(new cv::Ptr<cv::TrackerMedianFlow>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::TrackerMedianFlow>*>))
	}
	
	Result<cv::Ptr<cv::TrackerMedianFlow>*> cv_TrackerMedianFlow_create() {
		try {
			cv::Ptr<cv::TrackerMedianFlow> ret = cv::TrackerMedianFlow::create();
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
	
	Result<cv::Size> cv_TrackerMedianFlow_Params_getPropWinSize_const(const cv::TrackerMedianFlow::Params* instance) {
		try {
			cv::Size ret = instance->winSize;
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	Result_void cv_TrackerMedianFlow_Params_setPropWinSize_Size(cv::TrackerMedianFlow::Params* instance, cv::Size* val) {
		try {
			instance->winSize = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_TrackerMedianFlow_Params_getPropMaxLevel_const(const cv::TrackerMedianFlow::Params* instance) {
		try {
			int ret = instance->maxLevel;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_TrackerMedianFlow_Params_setPropMaxLevel_int(cv::TrackerMedianFlow::Params* instance, int val) {
		try {
			instance->maxLevel = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::TermCriteria> cv_TrackerMedianFlow_Params_getPropTermCriteria_const(const cv::TrackerMedianFlow::Params* instance) {
		try {
			cv::TermCriteria ret = instance->termCriteria;
			return Ok<cv::TermCriteria>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::TermCriteria>))
	}
	
	Result_void cv_TrackerMedianFlow_Params_setPropTermCriteria_TermCriteria(cv::TrackerMedianFlow::Params* instance, cv::TermCriteria* val) {
		try {
			instance->termCriteria = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Size> cv_TrackerMedianFlow_Params_getPropWinSizeNCC_const(const cv::TrackerMedianFlow::Params* instance) {
		try {
			cv::Size ret = instance->winSizeNCC;
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	Result_void cv_TrackerMedianFlow_Params_setPropWinSizeNCC_Size(cv::TrackerMedianFlow::Params* instance, cv::Size* val) {
		try {
			instance->winSizeNCC = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_TrackerMedianFlow_Params_getPropMaxMedianLengthOfDisplacementDifference_const(const cv::TrackerMedianFlow::Params* instance) {
		try {
			double ret = instance->maxMedianLengthOfDisplacementDifference;
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_TrackerMedianFlow_Params_setPropMaxMedianLengthOfDisplacementDifference_double(cv::TrackerMedianFlow::Params* instance, double val) {
		try {
			instance->maxMedianLengthOfDisplacementDifference = val;
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
	
	Result<cv::Ptr<cv::TrackerTLD>*> cv_TrackerTLD_create_const_ParamsR(const cv::TrackerTLD::Params* parameters) {
		try {
			cv::Ptr<cv::TrackerTLD> ret = cv::TrackerTLD::create(*parameters);
			return Ok(new cv::Ptr<cv::TrackerTLD>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::TrackerTLD>*>))
	}
	
	Result<cv::Ptr<cv::TrackerTLD>*> cv_TrackerTLD_create() {
		try {
			cv::Ptr<cv::TrackerTLD> ret = cv::TrackerTLD::create();
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
