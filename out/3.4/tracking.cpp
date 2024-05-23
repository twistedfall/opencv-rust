#include "ocvrs_common.hpp"
#include <opencv2/tracking.hpp>
#include "tracking_types.hpp"

extern "C" {
	// cv::tld::tld_InitDataset(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tldDataset.hpp:51
	// ("cv::tld::tld_InitDataset", vec![(pred!(mut, ["videoInd"], ["int"]), _)]),
	void cv_tld_tld_InitDataset_int(int videoInd, Result<cv::Rect2d>* ocvrs_return) {
		try {
			cv::Rect2d ret = cv::tld::tld_InitDataset(videoInd);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// tld_InitDataset(int, const char *, int)(Primitive, InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tldDataset.hpp:51
	// ("cv::tld::tld_InitDataset", vec![(pred!(mut, ["videoInd", "rootPath", "datasetInd"], ["int", "const char*", "int"]), _)]),
	void cv_tld_tld_InitDataset_int_const_charX_int(int videoInd, const char* rootPath, int datasetInd, Result<cv::Rect2d>* ocvrs_return) {
		try {
			cv::Rect2d ret = cv::tld::tld_InitDataset(videoInd, rootPath, datasetInd);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// tld_getNextDatasetFrame()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tldDataset.hpp:52
	// ("cv::tld::tld_getNextDatasetFrame", vec![(pred!(mut, [], []), _)]),
	void cv_tld_tld_getNextDatasetFrame(Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::tld::tld_getNextDatasetFrame();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ClfMilBoost()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/onlineMIL.hpp:70
	// ("cv::ClfMilBoost::ClfMilBoost", vec![(pred!(mut, [], []), _)]),
	void cv_ClfMilBoost_ClfMilBoost(Result<cv::ClfMilBoost*>* ocvrs_return) {
		try {
			cv::ClfMilBoost* ret = new cv::ClfMilBoost();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// init(const ClfMilBoost::Params &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/onlineMIL.hpp:72
	// ("cv::ClfMilBoost::init", vec![(pred!(mut, ["parameters"], ["const cv::ClfMilBoost::Params*"]), _)]),
	void cv_ClfMilBoost_init_const_ParamsR(cv::ClfMilBoost* instance, const cv::ClfMilBoost::Params* parameters, ResultVoid* ocvrs_return) {
		try {
			instance->init(*parameters);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ClfMilBoost::init() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/onlineMIL.hpp:72
	// ("cv::ClfMilBoost::init", vec![(pred!(mut, [], []), _)]),
	void cv_ClfMilBoost_init(cv::ClfMilBoost* instance, ResultVoid* ocvrs_return) {
		try {
			instance->init();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// update(const Mat &, const Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/onlineMIL.hpp:73
	// ("cv::ClfMilBoost::update", vec![(pred!(mut, ["posx", "negx"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
	void cv_ClfMilBoost_update_const_MatR_const_MatR(cv::ClfMilBoost* instance, const cv::Mat* posx, const cv::Mat* negx, ResultVoid* ocvrs_return) {
		try {
			instance->update(*posx, *negx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// classify(const Mat &, bool)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/onlineMIL.hpp:74
	// ("cv::ClfMilBoost::classify", vec![(pred!(mut, ["x", "logR"], ["const cv::Mat*", "bool"]), _)]),
	void cv_ClfMilBoost_classify_const_MatR_bool(cv::ClfMilBoost* instance, const cv::Mat* x, bool logR, Result<std::vector<float>*>* ocvrs_return) {
		try {
			std::vector<float> ret = instance->classify(*x, logR);
			Ok(new std::vector<float>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ClfMilBoost::classify(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/onlineMIL.hpp:74
	// ("cv::ClfMilBoost::classify", vec![(pred!(mut, ["x"], ["const cv::Mat*"]), _)]),
	void cv_ClfMilBoost_classify_const_MatR(cv::ClfMilBoost* instance, const cv::Mat* x, Result<std::vector<float>*>* ocvrs_return) {
		try {
			std::vector<float> ret = instance->classify(*x);
			Ok(new std::vector<float>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// sigmoid(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/onlineMIL.hpp:76
	// ("cv::ClfMilBoost::sigmoid", vec![(pred!(mut, ["x"], ["float"]), _)]),
	void cv_ClfMilBoost_sigmoid_float(cv::ClfMilBoost* instance, float x, Result<float>* ocvrs_return) {
		try {
			float ret = instance->sigmoid(x);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ClfMilBoost::delete() generated
	// ("cv::ClfMilBoost::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ClfMilBoost_delete(cv::ClfMilBoost* instance) {
			delete instance;
	}

	// Params()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/onlineMIL.hpp:64
	// ("cv::ClfMilBoost::Params::Params", vec![(pred!(mut, [], []), _)]),
	void cv_ClfMilBoost_Params_Params(Result<cv::ClfMilBoost::Params*>* ocvrs_return) {
		try {
			cv::ClfMilBoost::Params* ret = new cv::ClfMilBoost::Params();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ClfMilBoost::Params::_numSel() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/onlineMIL.hpp:65
	// ("cv::ClfMilBoost::Params::_numSel", vec![(pred!(const, [], []), _)]),
	int cv_ClfMilBoost_Params_prop_numSel_const(const cv::ClfMilBoost::Params* instance) {
			int ret = instance->_numSel;
			return ret;
	}

	// cv::ClfMilBoost::Params::set_numSel(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/onlineMIL.hpp:65
	// ("cv::ClfMilBoost::Params::set_numSel", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_ClfMilBoost_Params_prop_numSel_const_int(cv::ClfMilBoost::Params* instance, const int val) {
			instance->_numSel = val;
	}

	// cv::ClfMilBoost::Params::_numFeat() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/onlineMIL.hpp:66
	// ("cv::ClfMilBoost::Params::_numFeat", vec![(pred!(const, [], []), _)]),
	int cv_ClfMilBoost_Params_prop_numFeat_const(const cv::ClfMilBoost::Params* instance) {
			int ret = instance->_numFeat;
			return ret;
	}

	// cv::ClfMilBoost::Params::set_numFeat(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/onlineMIL.hpp:66
	// ("cv::ClfMilBoost::Params::set_numFeat", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_ClfMilBoost_Params_prop_numFeat_const_int(cv::ClfMilBoost::Params* instance, const int val) {
			instance->_numFeat = val;
	}

	// cv::ClfMilBoost::Params::_lRate() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/onlineMIL.hpp:67
	// ("cv::ClfMilBoost::Params::_lRate", vec![(pred!(const, [], []), _)]),
	float cv_ClfMilBoost_Params_prop_lRate_const(const cv::ClfMilBoost::Params* instance) {
			float ret = instance->_lRate;
			return ret;
	}

	// cv::ClfMilBoost::Params::set_lRate(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/onlineMIL.hpp:67
	// ("cv::ClfMilBoost::Params::set_lRate", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_ClfMilBoost_Params_prop_lRate_const_float(cv::ClfMilBoost::Params* instance, const float val) {
			instance->_lRate = val;
	}

	// cv::ClfMilBoost::Params::delete() generated
	// ("cv::ClfMilBoost::Params::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ClfMilBoost_Params_delete(cv::ClfMilBoost::Params* instance) {
			delete instance;
	}

	// init(const CvFeatureParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:147
	// ("cv::CvFeatureParams::init", vec![(pred!(mut, ["fp"], ["const cv::CvFeatureParams*"]), _)]),
	void cv_CvFeatureParams_init_const_CvFeatureParamsR(cv::CvFeatureParams* instance, const cv::CvFeatureParams* fp, ResultVoid* ocvrs_return) {
		try {
			instance->init(*fp);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:148
	// ("cv::CvFeatureParams::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	void cv_CvFeatureParams_write_const_FileStorageR(const cv::CvFeatureParams* instance, cv::FileStorage* fs, ResultVoid* ocvrs_return) {
		try {
			instance->write(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:149
	// ("cv::CvFeatureParams::read", vec![(pred!(mut, ["node"], ["const cv::FileNode*"]), _)]),
	void cv_CvFeatureParams_read_const_FileNodeR(cv::CvFeatureParams* instance, const cv::FileNode* node, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->read(*node);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::CvFeatureParams::maxCatCount() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:151
	// ("cv::CvFeatureParams::maxCatCount", vec![(pred!(const, [], []), _)]),
	int cv_CvFeatureParams_propMaxCatCount_const(const cv::CvFeatureParams* instance) {
			int ret = instance->maxCatCount;
			return ret;
	}

	// cv::CvFeatureParams::setMaxCatCount(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:151
	// ("cv::CvFeatureParams::setMaxCatCount", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_CvFeatureParams_propMaxCatCount_const_int(cv::CvFeatureParams* instance, const int val) {
			instance->maxCatCount = val;
	}

	// cv::CvFeatureParams::featSize() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:152
	// ("cv::CvFeatureParams::featSize", vec![(pred!(const, [], []), _)]),
	int cv_CvFeatureParams_propFeatSize_const(const cv::CvFeatureParams* instance) {
			int ret = instance->featSize;
			return ret;
	}

	// cv::CvFeatureParams::setFeatSize(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:152
	// ("cv::CvFeatureParams::setFeatSize", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_CvFeatureParams_propFeatSize_const_int(cv::CvFeatureParams* instance, const int val) {
			instance->featSize = val;
	}

	// cv::CvFeatureParams::numFeatures() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:153
	// ("cv::CvFeatureParams::numFeatures", vec![(pred!(const, [], []), _)]),
	int cv_CvFeatureParams_propNumFeatures_const(const cv::CvFeatureParams* instance) {
			int ret = instance->numFeatures;
			return ret;
	}

	// cv::CvFeatureParams::setNumFeatures(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:153
	// ("cv::CvFeatureParams::setNumFeatures", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_CvFeatureParams_propNumFeatures_const_int(cv::CvFeatureParams* instance, const int val) {
			instance->numFeatures = val;
	}

	// cv::CvFeatureParams::delete() generated
	// ("cv::CvFeatureParams::delete", vec![(pred!(mut, [], []), _)]),
	void cv_CvFeatureParams_delete(cv::CvFeatureParams* instance) {
			delete instance;
	}

	// init(const CvFeatureParams *, int, Size)(TraitClass, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:254
	// ("cv::CvHaarEvaluator::init", vec![(pred!(mut, ["_featureParams", "_maxSampleCount", "_winSize"], ["const cv::CvFeatureParams*", "int", "cv::Size"]), _)]),
	void cv_CvHaarEvaluator_init_const_CvFeatureParamsX_int_Size(cv::CvHaarEvaluator* instance, const cv::CvFeatureParams* _featureParams, int _maxSampleCount, cv::Size* _winSize, ResultVoid* ocvrs_return) {
		try {
			instance->init(_featureParams, _maxSampleCount, *_winSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setImage(const Mat &, uchar, int)(TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:255
	// ("cv::CvHaarEvaluator::setImage", vec![(pred!(mut, ["img", "clsLabel", "idx"], ["const cv::Mat*", "unsigned char", "int"]), _)]),
	void cv_CvHaarEvaluator_setImage_const_MatR_unsigned_char_int(cv::CvHaarEvaluator* instance, const cv::Mat* img, unsigned char clsLabel, int idx, ResultVoid* ocvrs_return) {
		try {
			instance->setImage(*img, clsLabel, idx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::CvHaarEvaluator::setImage(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:255
	// ("cv::CvHaarEvaluator::setImage", vec![(pred!(mut, ["img"], ["const cv::Mat*"]), _)]),
	void cv_CvHaarEvaluator_setImage_const_MatR(cv::CvHaarEvaluator* instance, const cv::Mat* img, ResultVoid* ocvrs_return) {
		try {
			instance->setImage(*img);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator()(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:256
	// ("cv::CvHaarEvaluator::operator()", vec![(pred!(mut, ["featureIdx", "sampleIdx"], ["int", "int"]), _)]),
	void cv_CvHaarEvaluator_operator___int_int(cv::CvHaarEvaluator* instance, int featureIdx, int sampleIdx, Result<float>* ocvrs_return) {
		try {
			float ret = instance->operator()(featureIdx, sampleIdx);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// writeFeatures(FileStorage &, const Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:257
	// ("cv::CvHaarEvaluator::writeFeatures", vec![(pred!(const, ["fs", "featureMap"], ["cv::FileStorage*", "const cv::Mat*"]), _)]),
	void cv_CvHaarEvaluator_writeFeatures_const_FileStorageR_const_MatR(const cv::CvHaarEvaluator* instance, cv::FileStorage* fs, const cv::Mat* featureMap, ResultVoid* ocvrs_return) {
		try {
			instance->writeFeatures(*fs, *featureMap);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getFeatures(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:260
	// ("cv::CvHaarEvaluator::getFeatures", vec![(pred!(mut, ["idx"], ["int"]), _)]),
	void cv_CvHaarEvaluator_getFeatures_int(cv::CvHaarEvaluator* instance, int idx, Result<cv::CvHaarEvaluator::FeatureHaar*>* ocvrs_return) {
		try {
			cv::CvHaarEvaluator::FeatureHaar ret = instance->getFeatures(idx);
			Ok(new cv::CvHaarEvaluator::FeatureHaar(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// generateFeatures()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:266
	// ("cv::CvHaarEvaluator::generateFeatures", vec![(pred!(mut, [], []), _)]),
	void cv_CvHaarEvaluator_generateFeatures(cv::CvHaarEvaluator* instance, ResultVoid* ocvrs_return) {
		try {
			instance->generateFeatures();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// generateFeatures(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:274
	// ("cv::CvHaarEvaluator::generateFeatures", vec![(pred!(mut, ["numFeatures"], ["int"]), _)]),
	void cv_CvHaarEvaluator_generateFeatures_int(cv::CvHaarEvaluator* instance, int numFeatures, ResultVoid* ocvrs_return) {
		try {
			instance->generateFeatures(numFeatures);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::CvHaarEvaluator::defaultNew() generated
	// ("cv::CvHaarEvaluator::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::CvHaarEvaluator* cv_CvHaarEvaluator_defaultNew_const() {
			cv::CvHaarEvaluator* ret = new cv::CvHaarEvaluator();
			return ret;
	}

	// cv::CvHaarEvaluator::delete() generated
	// ("cv::CvHaarEvaluator::delete", vec![(pred!(mut, [], []), _)]),
	void cv_CvHaarEvaluator_delete(cv::CvHaarEvaluator* instance) {
			delete instance;
	}

	// write(FileStorage)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:229
	// ("cv::CvHaarEvaluator::FeatureHaar::write", vec![(pred!(const, ["unnamed"], ["cv::FileStorage"]), _)]),
	void cv_CvHaarEvaluator_FeatureHaar_write_const_FileStorage(const cv::CvHaarEvaluator::FeatureHaar* instance, cv::FileStorage* unnamed, ResultVoid* ocvrs_return) {
		try {
			instance->write(*unnamed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::CvHaarEvaluator::FeatureHaar::delete() generated
	// ("cv::CvHaarEvaluator::FeatureHaar::delete", vec![(pred!(mut, [], []), _)]),
	void cv_CvHaarEvaluator_FeatureHaar_delete(cv::CvHaarEvaluator::FeatureHaar* instance) {
			delete instance;
	}

	// MultiTracker()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1339
	// ("cv::MultiTracker::MultiTracker", vec![(pred!(mut, [], []), _)]),
	void cv_MultiTracker_MultiTracker(Result<cv::MultiTracker*>* ocvrs_return) {
		try {
			cv::MultiTracker* ret = new cv::MultiTracker();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// add(Ptr<Tracker>, InputArray, const Rect2d &)(CppPassByVoidPtr, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1353
	// ("cv::MultiTracker::add", vec![(pred!(mut, ["newTracker", "image", "boundingBox"], ["cv::Ptr<cv::Tracker>", "const cv::_InputArray*", "const cv::Rect2d*"]), _)]),
	void cv_MultiTracker_add_PtrLTrackerG_const__InputArrayR_const_Rect2dR(cv::MultiTracker* instance, cv::Ptr<cv::Tracker>* newTracker, const cv::_InputArray* image, const cv::Rect2d* boundingBox, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->add(*newTracker, *image, *boundingBox);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// add(std::vector<Ptr<Tracker>>, InputArray, std::vector<Rect2d>)(CppPassByVoidPtr, InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1361
	// ("cv::MultiTracker::add", vec![(pred!(mut, ["newTrackers", "image", "boundingBox"], ["std::vector<cv::Ptr<cv::Tracker>>", "const cv::_InputArray*", "std::vector<cv::Rect2d>"]), _)]),
	void cv_MultiTracker_add_vectorLPtrLTrackerGG_const__InputArrayR_vectorLRect2dG(cv::MultiTracker* instance, std::vector<cv::Ptr<cv::Tracker>>* newTrackers, const cv::_InputArray* image, std::vector<cv::Rect2d>* boundingBox, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->add(*newTrackers, *image, *boundingBox);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// update(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1368
	// ("cv::MultiTracker::update", vec![(pred!(mut, ["image"], ["const cv::_InputArray*"]), _)]),
	void cv_MultiTracker_update_const__InputArrayR(cv::MultiTracker* instance, const cv::_InputArray* image, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->update(*image);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// update(InputArray, std::vector<Rect2d> &)(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1375
	// ("cv::MultiTracker::update", vec![(pred!(mut, ["image", "boundingBox"], ["const cv::_InputArray*", "std::vector<cv::Rect2d>*"]), _)]),
	void cv_MultiTracker_update_const__InputArrayR_vectorLRect2dGR(cv::MultiTracker* instance, const cv::_InputArray* image, std::vector<cv::Rect2d>* boundingBox, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->update(*image, *boundingBox);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getObjects()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1380
	// ("cv::MultiTracker::getObjects", vec![(pred!(const, [], []), _)]),
	void cv_MultiTracker_getObjects_const(const cv::MultiTracker* instance, Result<std::vector<cv::Rect2d>*>* ocvrs_return) {
		try {
			const std::vector<cv::Rect2d> ret = instance->getObjects();
			Ok(new const std::vector<cv::Rect2d>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1385
	// ("cv::MultiTracker::create", vec![(pred!(mut, [], []), _)]),
	void cv_MultiTracker_create(Result<cv::Ptr<cv::MultiTracker>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::MultiTracker> ret = cv::MultiTracker::create();
			Ok(new cv::Ptr<cv::MultiTracker>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::MultiTracker::to_Algorithm() generated
	// ("cv::MultiTracker::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_MultiTracker_to_Algorithm(cv::MultiTracker* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::MultiTracker::delete() generated
	// ("cv::MultiTracker::delete", vec![(pred!(mut, [], []), _)]),
	void cv_MultiTracker_delete(cv::MultiTracker* instance) {
			delete instance;
	}

	// update_opt(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1473
	// ("cv::MultiTrackerTLD::update_opt", vec![(pred!(mut, ["image"], ["const cv::_InputArray*"]), _)]),
	void cv_MultiTrackerTLD_update_opt_const__InputArrayR(cv::MultiTrackerTLD* instance, const cv::_InputArray* image, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->update_opt(*image);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::MultiTrackerTLD::defaultNew() generated
	// ("cv::MultiTrackerTLD::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::MultiTrackerTLD* cv_MultiTrackerTLD_defaultNew_const() {
			cv::MultiTrackerTLD* ret = new cv::MultiTrackerTLD();
			return ret;
	}

	// cv::MultiTrackerTLD::to_MultiTracker_Alt() generated
	// ("cv::MultiTrackerTLD::to_MultiTracker_Alt", vec![(pred!(mut, [], []), _)]),
	cv::MultiTracker_Alt* cv_MultiTrackerTLD_to_MultiTracker_Alt(cv::MultiTrackerTLD* instance) {
			return dynamic_cast<cv::MultiTracker_Alt*>(instance);
	}

	// cv::MultiTrackerTLD::delete() generated
	// ("cv::MultiTrackerTLD::delete", vec![(pred!(mut, [], []), _)]),
	void cv_MultiTrackerTLD_delete(cv::MultiTrackerTLD* instance) {
			delete instance;
	}

	// MultiTracker_Alt()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1406
	// ("cv::MultiTracker_Alt::MultiTracker_Alt", vec![(pred!(mut, [], []), _)]),
	void cv_MultiTracker_Alt_MultiTracker_Alt(Result<cv::MultiTracker_Alt*>* ocvrs_return) {
		try {
			cv::MultiTracker_Alt* ret = new cv::MultiTracker_Alt();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// addTarget(InputArray, const Rect2d &, Ptr<Tracker>)(InputArray, SimpleClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1418
	// ("cv::MultiTracker_Alt::addTarget", vec![(pred!(mut, ["image", "boundingBox", "tracker_algorithm"], ["const cv::_InputArray*", "const cv::Rect2d*", "cv::Ptr<cv::Tracker>"]), _)]),
	void cv_MultiTracker_Alt_addTarget_const__InputArrayR_const_Rect2dR_PtrLTrackerG(cv::MultiTracker_Alt* instance, const cv::_InputArray* image, const cv::Rect2d* boundingBox, cv::Ptr<cv::Tracker>* tracker_algorithm, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->addTarget(*image, *boundingBox, *tracker_algorithm);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// update(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1427
	// ("cv::MultiTracker_Alt::update", vec![(pred!(mut, ["image"], ["const cv::_InputArray*"]), _)]),
	void cv_MultiTracker_Alt_update_const__InputArrayR(cv::MultiTracker_Alt* instance, const cv::_InputArray* image, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->update(*image);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::MultiTracker_Alt::targetNum() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1431
	// ("cv::MultiTracker_Alt::targetNum", vec![(pred!(const, [], []), _)]),
	int cv_MultiTracker_Alt_propTargetNum_const(const cv::MultiTracker_Alt* instance) {
			int ret = instance->targetNum;
			return ret;
	}

	// cv::MultiTracker_Alt::setTargetNum(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1431
	// ("cv::MultiTracker_Alt::setTargetNum", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_MultiTracker_Alt_propTargetNum_const_int(cv::MultiTracker_Alt* instance, const int val) {
			instance->targetNum = val;
	}

	// cv::MultiTracker_Alt::trackers() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1435
	// ("cv::MultiTracker_Alt::trackers", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Ptr<cv::Tracker>>* cv_MultiTracker_Alt_propTrackers_const(const cv::MultiTracker_Alt* instance) {
			std::vector<cv::Ptr<cv::Tracker>> ret = instance->trackers;
			return new std::vector<cv::Ptr<cv::Tracker>>(ret);
	}

	// cv::MultiTracker_Alt::setTrackers(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1435
	// ("cv::MultiTracker_Alt::setTrackers", vec![(pred!(mut, ["val"], ["const std::vector<cv::Ptr<cv::Tracker>>"]), _)]),
	void cv_MultiTracker_Alt_propTrackers_const_vectorLPtrLTrackerGG(cv::MultiTracker_Alt* instance, const std::vector<cv::Ptr<cv::Tracker>>* val) {
			instance->trackers = *val;
	}

	// cv::MultiTracker_Alt::boundingBoxes() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1439
	// ("cv::MultiTracker_Alt::boundingBoxes", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Rect2d>* cv_MultiTracker_Alt_propBoundingBoxes_const(const cv::MultiTracker_Alt* instance) {
			std::vector<cv::Rect2d> ret = instance->boundingBoxes;
			return new std::vector<cv::Rect2d>(ret);
	}

	// cv::MultiTracker_Alt::setBoundingBoxes(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1439
	// ("cv::MultiTracker_Alt::setBoundingBoxes", vec![(pred!(mut, ["val"], ["const std::vector<cv::Rect2d>"]), _)]),
	void cv_MultiTracker_Alt_propBoundingBoxes_const_vectorLRect2dG(cv::MultiTracker_Alt* instance, const std::vector<cv::Rect2d>* val) {
			instance->boundingBoxes = *val;
	}

	// cv::MultiTracker_Alt::colors() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1442
	// ("cv::MultiTracker_Alt::colors", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Scalar>* cv_MultiTracker_Alt_propColors_const(const cv::MultiTracker_Alt* instance) {
			std::vector<cv::Scalar> ret = instance->colors;
			return new std::vector<cv::Scalar>(ret);
	}

	// cv::MultiTracker_Alt::setColors(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1442
	// ("cv::MultiTracker_Alt::setColors", vec![(pred!(mut, ["val"], ["const std::vector<cv::Scalar>"]), _)]),
	void cv_MultiTracker_Alt_propColors_const_vectorLScalarG(cv::MultiTracker_Alt* instance, const std::vector<cv::Scalar>* val) {
			instance->colors = *val;
	}

	// cv::MultiTracker_Alt::delete() generated
	// ("cv::MultiTracker_Alt::delete", vec![(pred!(mut, [], []), _)]),
	void cv_MultiTracker_Alt_delete(cv::MultiTracker_Alt* instance) {
			delete instance;
	}

	// init(InputArray, const Rect2d &)(InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:536
	// ("cv::Tracker::init", vec![(pred!(mut, ["image", "boundingBox"], ["const cv::_InputArray*", "const cv::Rect2d*"]), _)]),
	void cv_Tracker_init_const__InputArrayR_const_Rect2dR(cv::Tracker* instance, const cv::_InputArray* image, const cv::Rect2d* boundingBox, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->init(*image, *boundingBox);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// update(InputArray, Rect2d &)(InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:547
	// ("cv::Tracker::update", vec![(pred!(mut, ["image", "boundingBox"], ["const cv::_InputArray*", "cv::Rect2d*"]), _)]),
	void cv_Tracker_update_const__InputArrayR_Rect2dR(cv::Tracker* instance, const cv::_InputArray* image, cv::Rect2d* boundingBox, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->update(*image, *boundingBox);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:549
	// ("cv::Tracker::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	void cv_Tracker_read_const_FileNodeR(cv::Tracker* instance, const cv::FileNode* fn, ResultVoid* ocvrs_return) {
		try {
			instance->read(*fn);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:550
	// ("cv::Tracker::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	void cv_Tracker_write_const_FileStorageR(const cv::Tracker* instance, cv::FileStorage* fs, ResultVoid* ocvrs_return) {
		try {
			instance->write(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::Tracker::to_TrackerBoosting() generated
	// ("cv::Tracker::to_TrackerBoosting", vec![(pred!(mut, [], []), _)]),
	cv::TrackerBoosting* cv_Tracker_to_TrackerBoosting(cv::Tracker* instance) {
			return dynamic_cast<cv::TrackerBoosting*>(instance);
	}

	// cv::Tracker::to_TrackerCSRT() generated
	// ("cv::Tracker::to_TrackerCSRT", vec![(pred!(mut, [], []), _)]),
	cv::TrackerCSRT* cv_Tracker_to_TrackerCSRT(cv::Tracker* instance) {
			return dynamic_cast<cv::TrackerCSRT*>(instance);
	}

	// cv::Tracker::to_TrackerGOTURN() generated
	// ("cv::Tracker::to_TrackerGOTURN", vec![(pred!(mut, [], []), _)]),
	cv::TrackerGOTURN* cv_Tracker_to_TrackerGOTURN(cv::Tracker* instance) {
			return dynamic_cast<cv::TrackerGOTURN*>(instance);
	}

	// cv::Tracker::to_TrackerKCF() generated
	// ("cv::Tracker::to_TrackerKCF", vec![(pred!(mut, [], []), _)]),
	cv::TrackerKCF* cv_Tracker_to_TrackerKCF(cv::Tracker* instance) {
			return dynamic_cast<cv::TrackerKCF*>(instance);
	}

	// cv::Tracker::to_TrackerMIL() generated
	// ("cv::Tracker::to_TrackerMIL", vec![(pred!(mut, [], []), _)]),
	cv::TrackerMIL* cv_Tracker_to_TrackerMIL(cv::Tracker* instance) {
			return dynamic_cast<cv::TrackerMIL*>(instance);
	}

	// cv::Tracker::to_TrackerMOSSE() generated
	// ("cv::Tracker::to_TrackerMOSSE", vec![(pred!(mut, [], []), _)]),
	cv::TrackerMOSSE* cv_Tracker_to_TrackerMOSSE(cv::Tracker* instance) {
			return dynamic_cast<cv::TrackerMOSSE*>(instance);
	}

	// cv::Tracker::to_TrackerMedianFlow() generated
	// ("cv::Tracker::to_TrackerMedianFlow", vec![(pred!(mut, [], []), _)]),
	cv::TrackerMedianFlow* cv_Tracker_to_TrackerMedianFlow(cv::Tracker* instance) {
			return dynamic_cast<cv::TrackerMedianFlow*>(instance);
	}

	// cv::Tracker::to_TrackerTLD() generated
	// ("cv::Tracker::to_TrackerTLD", vec![(pred!(mut, [], []), _)]),
	cv::TrackerTLD* cv_Tracker_to_TrackerTLD(cv::Tracker* instance) {
			return dynamic_cast<cv::TrackerTLD*>(instance);
	}

	// cv::Tracker::to_Algorithm() generated
	// ("cv::Tracker::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_Tracker_to_Algorithm(cv::Tracker* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::Tracker::delete() generated
	// ("cv::Tracker::delete", vec![(pred!(mut, [], []), _)]),
	void cv_Tracker_delete(cv::Tracker* instance) {
			delete instance;
	}

	// create(const TrackerBoosting::Params &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1125
	// ("cv::TrackerBoosting::create", vec![(pred!(mut, ["parameters"], ["const cv::TrackerBoosting::Params*"]), _)]),
	void cv_TrackerBoosting_create_const_ParamsR(const cv::TrackerBoosting::Params* parameters, Result<cv::Ptr<cv::TrackerBoosting>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::TrackerBoosting> ret = cv::TrackerBoosting::create(*parameters);
			Ok(new cv::Ptr<cv::TrackerBoosting>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1127
	// ("cv::TrackerBoosting::create", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerBoosting_create(Result<cv::Ptr<cv::TrackerBoosting>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::TrackerBoosting> ret = cv::TrackerBoosting::create();
			Ok(new cv::Ptr<cv::TrackerBoosting>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TrackerBoosting::to_Algorithm() generated
	// ("cv::TrackerBoosting::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_TrackerBoosting_to_Algorithm(cv::TrackerBoosting* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::TrackerBoosting::to_Tracker() generated
	// ("cv::TrackerBoosting::to_Tracker", vec![(pred!(mut, [], []), _)]),
	cv::Tracker* cv_TrackerBoosting_to_Tracker(cv::TrackerBoosting* instance) {
			return dynamic_cast<cv::Tracker*>(instance);
	}

	// cv::TrackerBoosting::delete() generated
	// ("cv::TrackerBoosting::delete", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerBoosting_delete(cv::TrackerBoosting* instance) {
			delete instance;
	}

	// Params()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1105
	// ("cv::TrackerBoosting::Params::Params", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerBoosting_Params_Params(Result<cv::TrackerBoosting::Params*>* ocvrs_return) {
		try {
			cv::TrackerBoosting::Params* ret = new cv::TrackerBoosting::Params();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1114
	// ("cv::TrackerBoosting::Params::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	void cv_TrackerBoosting_Params_read_const_FileNodeR(cv::TrackerBoosting::Params* instance, const cv::FileNode* fn, ResultVoid* ocvrs_return) {
		try {
			instance->read(*fn);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1119
	// ("cv::TrackerBoosting::Params::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	void cv_TrackerBoosting_Params_write_const_FileStorageR(const cv::TrackerBoosting::Params* instance, cv::FileStorage* fs, ResultVoid* ocvrs_return) {
		try {
			instance->write(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TrackerBoosting::Params::numClassifiers() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1106
	// ("cv::TrackerBoosting::Params::numClassifiers", vec![(pred!(const, [], []), _)]),
	int cv_TrackerBoosting_Params_propNumClassifiers_const(const cv::TrackerBoosting::Params* instance) {
			int ret = instance->numClassifiers;
			return ret;
	}

	// cv::TrackerBoosting::Params::setNumClassifiers(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1106
	// ("cv::TrackerBoosting::Params::setNumClassifiers", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_TrackerBoosting_Params_propNumClassifiers_const_int(cv::TrackerBoosting::Params* instance, const int val) {
			instance->numClassifiers = val;
	}

	// cv::TrackerBoosting::Params::samplerOverlap() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1107
	// ("cv::TrackerBoosting::Params::samplerOverlap", vec![(pred!(const, [], []), _)]),
	float cv_TrackerBoosting_Params_propSamplerOverlap_const(const cv::TrackerBoosting::Params* instance) {
			float ret = instance->samplerOverlap;
			return ret;
	}

	// cv::TrackerBoosting::Params::setSamplerOverlap(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1107
	// ("cv::TrackerBoosting::Params::setSamplerOverlap", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_TrackerBoosting_Params_propSamplerOverlap_const_float(cv::TrackerBoosting::Params* instance, const float val) {
			instance->samplerOverlap = val;
	}

	// cv::TrackerBoosting::Params::samplerSearchFactor() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1108
	// ("cv::TrackerBoosting::Params::samplerSearchFactor", vec![(pred!(const, [], []), _)]),
	float cv_TrackerBoosting_Params_propSamplerSearchFactor_const(const cv::TrackerBoosting::Params* instance) {
			float ret = instance->samplerSearchFactor;
			return ret;
	}

	// cv::TrackerBoosting::Params::setSamplerSearchFactor(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1108
	// ("cv::TrackerBoosting::Params::setSamplerSearchFactor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_TrackerBoosting_Params_propSamplerSearchFactor_const_float(cv::TrackerBoosting::Params* instance, const float val) {
			instance->samplerSearchFactor = val;
	}

	// cv::TrackerBoosting::Params::iterationInit() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1109
	// ("cv::TrackerBoosting::Params::iterationInit", vec![(pred!(const, [], []), _)]),
	int cv_TrackerBoosting_Params_propIterationInit_const(const cv::TrackerBoosting::Params* instance) {
			int ret = instance->iterationInit;
			return ret;
	}

	// cv::TrackerBoosting::Params::setIterationInit(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1109
	// ("cv::TrackerBoosting::Params::setIterationInit", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_TrackerBoosting_Params_propIterationInit_const_int(cv::TrackerBoosting::Params* instance, const int val) {
			instance->iterationInit = val;
	}

	// cv::TrackerBoosting::Params::featureSetNumFeatures() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1110
	// ("cv::TrackerBoosting::Params::featureSetNumFeatures", vec![(pred!(const, [], []), _)]),
	int cv_TrackerBoosting_Params_propFeatureSetNumFeatures_const(const cv::TrackerBoosting::Params* instance) {
			int ret = instance->featureSetNumFeatures;
			return ret;
	}

	// cv::TrackerBoosting::Params::setFeatureSetNumFeatures(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1110
	// ("cv::TrackerBoosting::Params::setFeatureSetNumFeatures", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_TrackerBoosting_Params_propFeatureSetNumFeatures_const_int(cv::TrackerBoosting::Params* instance, const int val) {
			instance->featureSetNumFeatures = val;
	}

	// cv::TrackerBoosting::Params::delete() generated
	// ("cv::TrackerBoosting::Params::delete", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerBoosting_Params_delete(cv::TrackerBoosting::Params* instance) {
			delete instance;
	}

	// create(const TrackerCSRT::Params &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1536
	// ("cv::TrackerCSRT::create", vec![(pred!(mut, ["parameters"], ["const cv::TrackerCSRT::Params*"]), _)]),
	void cv_TrackerCSRT_create_const_ParamsR(const cv::TrackerCSRT::Params* parameters, Result<cv::Ptr<cv::TrackerCSRT>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::TrackerCSRT> ret = cv::TrackerCSRT::create(*parameters);
			Ok(new cv::Ptr<cv::TrackerCSRT>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1538
	// ("cv::TrackerCSRT::create", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerCSRT_create(Result<cv::Ptr<cv::TrackerCSRT>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::TrackerCSRT> ret = cv::TrackerCSRT::create();
			Ok(new cv::Ptr<cv::TrackerCSRT>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setInitialMask(const Mat)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1540
	// ("cv::TrackerCSRT::setInitialMask", vec![(pred!(mut, ["mask"], ["const cv::Mat"]), _)]),
	void cv_TrackerCSRT_setInitialMask_const_Mat(cv::TrackerCSRT* instance, const cv::Mat* mask, ResultVoid* ocvrs_return) {
		try {
			instance->setInitialMask(*mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TrackerCSRT::to_Algorithm() generated
	// ("cv::TrackerCSRT::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_TrackerCSRT_to_Algorithm(cv::TrackerCSRT* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::TrackerCSRT::to_Tracker() generated
	// ("cv::TrackerCSRT::to_Tracker", vec![(pred!(mut, [], []), _)]),
	cv::Tracker* cv_TrackerCSRT_to_Tracker(cv::TrackerCSRT* instance) {
			return dynamic_cast<cv::Tracker*>(instance);
	}

	// cv::TrackerCSRT::delete() generated
	// ("cv::TrackerCSRT::delete", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerCSRT_delete(cv::TrackerCSRT* instance) {
			delete instance;
	}

	// Params()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1489
	// ("cv::TrackerCSRT::Params::Params", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerCSRT_Params_Params(Result<cv::TrackerCSRT::Params*>* ocvrs_return) {
		try {
			cv::TrackerCSRT::Params* ret = new cv::TrackerCSRT::Params();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1494
	// ("cv::TrackerCSRT::Params::read", vec![(pred!(mut, ["unnamed"], ["const cv::FileNode*"]), _)]),
	void cv_TrackerCSRT_Params_read_const_FileNodeR(cv::TrackerCSRT::Params* instance, const cv::FileNode* unnamed, ResultVoid* ocvrs_return) {
		try {
			instance->read(*unnamed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(cv::FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1499
	// ("cv::TrackerCSRT::Params::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	void cv_TrackerCSRT_Params_write_const_FileStorageR(const cv::TrackerCSRT::Params* instance, cv::FileStorage* fs, ResultVoid* ocvrs_return) {
		try {
			instance->write(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TrackerCSRT::Params::use_hog() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1501
	// ("cv::TrackerCSRT::Params::use_hog", vec![(pred!(const, [], []), _)]),
	bool cv_TrackerCSRT_Params_propUse_hog_const(const cv::TrackerCSRT::Params* instance) {
			bool ret = instance->use_hog;
			return ret;
	}

	// cv::TrackerCSRT::Params::setUse_hog(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1501
	// ("cv::TrackerCSRT::Params::setUse_hog", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_TrackerCSRT_Params_propUse_hog_const_bool(cv::TrackerCSRT::Params* instance, const bool val) {
			instance->use_hog = val;
	}

	// cv::TrackerCSRT::Params::use_color_names() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1502
	// ("cv::TrackerCSRT::Params::use_color_names", vec![(pred!(const, [], []), _)]),
	bool cv_TrackerCSRT_Params_propUse_color_names_const(const cv::TrackerCSRT::Params* instance) {
			bool ret = instance->use_color_names;
			return ret;
	}

	// cv::TrackerCSRT::Params::setUse_color_names(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1502
	// ("cv::TrackerCSRT::Params::setUse_color_names", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_TrackerCSRT_Params_propUse_color_names_const_bool(cv::TrackerCSRT::Params* instance, const bool val) {
			instance->use_color_names = val;
	}

	// cv::TrackerCSRT::Params::use_gray() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1503
	// ("cv::TrackerCSRT::Params::use_gray", vec![(pred!(const, [], []), _)]),
	bool cv_TrackerCSRT_Params_propUse_gray_const(const cv::TrackerCSRT::Params* instance) {
			bool ret = instance->use_gray;
			return ret;
	}

	// cv::TrackerCSRT::Params::setUse_gray(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1503
	// ("cv::TrackerCSRT::Params::setUse_gray", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_TrackerCSRT_Params_propUse_gray_const_bool(cv::TrackerCSRT::Params* instance, const bool val) {
			instance->use_gray = val;
	}

	// cv::TrackerCSRT::Params::use_rgb() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1504
	// ("cv::TrackerCSRT::Params::use_rgb", vec![(pred!(const, [], []), _)]),
	bool cv_TrackerCSRT_Params_propUse_rgb_const(const cv::TrackerCSRT::Params* instance) {
			bool ret = instance->use_rgb;
			return ret;
	}

	// cv::TrackerCSRT::Params::setUse_rgb(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1504
	// ("cv::TrackerCSRT::Params::setUse_rgb", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_TrackerCSRT_Params_propUse_rgb_const_bool(cv::TrackerCSRT::Params* instance, const bool val) {
			instance->use_rgb = val;
	}

	// cv::TrackerCSRT::Params::use_channel_weights() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1505
	// ("cv::TrackerCSRT::Params::use_channel_weights", vec![(pred!(const, [], []), _)]),
	bool cv_TrackerCSRT_Params_propUse_channel_weights_const(const cv::TrackerCSRT::Params* instance) {
			bool ret = instance->use_channel_weights;
			return ret;
	}

	// cv::TrackerCSRT::Params::setUse_channel_weights(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1505
	// ("cv::TrackerCSRT::Params::setUse_channel_weights", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_TrackerCSRT_Params_propUse_channel_weights_const_bool(cv::TrackerCSRT::Params* instance, const bool val) {
			instance->use_channel_weights = val;
	}

	// cv::TrackerCSRT::Params::use_segmentation() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1506
	// ("cv::TrackerCSRT::Params::use_segmentation", vec![(pred!(const, [], []), _)]),
	bool cv_TrackerCSRT_Params_propUse_segmentation_const(const cv::TrackerCSRT::Params* instance) {
			bool ret = instance->use_segmentation;
			return ret;
	}

	// cv::TrackerCSRT::Params::setUse_segmentation(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1506
	// ("cv::TrackerCSRT::Params::setUse_segmentation", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_TrackerCSRT_Params_propUse_segmentation_const_bool(cv::TrackerCSRT::Params* instance, const bool val) {
			instance->use_segmentation = val;
	}

	// cv::TrackerCSRT::Params::window_function() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1508
	// ("cv::TrackerCSRT::Params::window_function", vec![(pred!(const, [], []), _)]),
	void* cv_TrackerCSRT_Params_propWindow_function_const(const cv::TrackerCSRT::Params* instance) {
			std::string ret = instance->window_function;
			return ocvrs_create_string(ret.c_str());
	}

	// cv::TrackerCSRT::Params::setWindow_function(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1508
	// ("cv::TrackerCSRT::Params::setWindow_function", vec![(pred!(mut, ["val"], ["const std::string"]), _)]),
	void cv_TrackerCSRT_Params_propWindow_function_const_string(cv::TrackerCSRT::Params* instance, const char* val) {
			instance->window_function = std::string(val);
	}

	// cv::TrackerCSRT::Params::kaiser_alpha() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1509
	// ("cv::TrackerCSRT::Params::kaiser_alpha", vec![(pred!(const, [], []), _)]),
	float cv_TrackerCSRT_Params_propKaiser_alpha_const(const cv::TrackerCSRT::Params* instance) {
			float ret = instance->kaiser_alpha;
			return ret;
	}

	// cv::TrackerCSRT::Params::setKaiser_alpha(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1509
	// ("cv::TrackerCSRT::Params::setKaiser_alpha", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_TrackerCSRT_Params_propKaiser_alpha_const_float(cv::TrackerCSRT::Params* instance, const float val) {
			instance->kaiser_alpha = val;
	}

	// cv::TrackerCSRT::Params::cheb_attenuation() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1510
	// ("cv::TrackerCSRT::Params::cheb_attenuation", vec![(pred!(const, [], []), _)]),
	float cv_TrackerCSRT_Params_propCheb_attenuation_const(const cv::TrackerCSRT::Params* instance) {
			float ret = instance->cheb_attenuation;
			return ret;
	}

	// cv::TrackerCSRT::Params::setCheb_attenuation(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1510
	// ("cv::TrackerCSRT::Params::setCheb_attenuation", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_TrackerCSRT_Params_propCheb_attenuation_const_float(cv::TrackerCSRT::Params* instance, const float val) {
			instance->cheb_attenuation = val;
	}

	// cv::TrackerCSRT::Params::template_size() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1512
	// ("cv::TrackerCSRT::Params::template_size", vec![(pred!(const, [], []), _)]),
	float cv_TrackerCSRT_Params_propTemplate_size_const(const cv::TrackerCSRT::Params* instance) {
			float ret = instance->template_size;
			return ret;
	}

	// cv::TrackerCSRT::Params::setTemplate_size(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1512
	// ("cv::TrackerCSRT::Params::setTemplate_size", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_TrackerCSRT_Params_propTemplate_size_const_float(cv::TrackerCSRT::Params* instance, const float val) {
			instance->template_size = val;
	}

	// cv::TrackerCSRT::Params::gsl_sigma() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1513
	// ("cv::TrackerCSRT::Params::gsl_sigma", vec![(pred!(const, [], []), _)]),
	float cv_TrackerCSRT_Params_propGsl_sigma_const(const cv::TrackerCSRT::Params* instance) {
			float ret = instance->gsl_sigma;
			return ret;
	}

	// cv::TrackerCSRT::Params::setGsl_sigma(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1513
	// ("cv::TrackerCSRT::Params::setGsl_sigma", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_TrackerCSRT_Params_propGsl_sigma_const_float(cv::TrackerCSRT::Params* instance, const float val) {
			instance->gsl_sigma = val;
	}

	// cv::TrackerCSRT::Params::hog_orientations() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1514
	// ("cv::TrackerCSRT::Params::hog_orientations", vec![(pred!(const, [], []), _)]),
	float cv_TrackerCSRT_Params_propHog_orientations_const(const cv::TrackerCSRT::Params* instance) {
			float ret = instance->hog_orientations;
			return ret;
	}

	// cv::TrackerCSRT::Params::setHog_orientations(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1514
	// ("cv::TrackerCSRT::Params::setHog_orientations", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_TrackerCSRT_Params_propHog_orientations_const_float(cv::TrackerCSRT::Params* instance, const float val) {
			instance->hog_orientations = val;
	}

	// cv::TrackerCSRT::Params::hog_clip() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1515
	// ("cv::TrackerCSRT::Params::hog_clip", vec![(pred!(const, [], []), _)]),
	float cv_TrackerCSRT_Params_propHog_clip_const(const cv::TrackerCSRT::Params* instance) {
			float ret = instance->hog_clip;
			return ret;
	}

	// cv::TrackerCSRT::Params::setHog_clip(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1515
	// ("cv::TrackerCSRT::Params::setHog_clip", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_TrackerCSRT_Params_propHog_clip_const_float(cv::TrackerCSRT::Params* instance, const float val) {
			instance->hog_clip = val;
	}

	// cv::TrackerCSRT::Params::padding() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1516
	// ("cv::TrackerCSRT::Params::padding", vec![(pred!(const, [], []), _)]),
	float cv_TrackerCSRT_Params_propPadding_const(const cv::TrackerCSRT::Params* instance) {
			float ret = instance->padding;
			return ret;
	}

	// cv::TrackerCSRT::Params::setPadding(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1516
	// ("cv::TrackerCSRT::Params::setPadding", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_TrackerCSRT_Params_propPadding_const_float(cv::TrackerCSRT::Params* instance, const float val) {
			instance->padding = val;
	}

	// cv::TrackerCSRT::Params::filter_lr() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1517
	// ("cv::TrackerCSRT::Params::filter_lr", vec![(pred!(const, [], []), _)]),
	float cv_TrackerCSRT_Params_propFilter_lr_const(const cv::TrackerCSRT::Params* instance) {
			float ret = instance->filter_lr;
			return ret;
	}

	// cv::TrackerCSRT::Params::setFilter_lr(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1517
	// ("cv::TrackerCSRT::Params::setFilter_lr", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_TrackerCSRT_Params_propFilter_lr_const_float(cv::TrackerCSRT::Params* instance, const float val) {
			instance->filter_lr = val;
	}

	// cv::TrackerCSRT::Params::weights_lr() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1518
	// ("cv::TrackerCSRT::Params::weights_lr", vec![(pred!(const, [], []), _)]),
	float cv_TrackerCSRT_Params_propWeights_lr_const(const cv::TrackerCSRT::Params* instance) {
			float ret = instance->weights_lr;
			return ret;
	}

	// cv::TrackerCSRT::Params::setWeights_lr(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1518
	// ("cv::TrackerCSRT::Params::setWeights_lr", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_TrackerCSRT_Params_propWeights_lr_const_float(cv::TrackerCSRT::Params* instance, const float val) {
			instance->weights_lr = val;
	}

	// cv::TrackerCSRT::Params::num_hog_channels_used() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1519
	// ("cv::TrackerCSRT::Params::num_hog_channels_used", vec![(pred!(const, [], []), _)]),
	int cv_TrackerCSRT_Params_propNum_hog_channels_used_const(const cv::TrackerCSRT::Params* instance) {
			int ret = instance->num_hog_channels_used;
			return ret;
	}

	// cv::TrackerCSRT::Params::setNum_hog_channels_used(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1519
	// ("cv::TrackerCSRT::Params::setNum_hog_channels_used", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_TrackerCSRT_Params_propNum_hog_channels_used_const_int(cv::TrackerCSRT::Params* instance, const int val) {
			instance->num_hog_channels_used = val;
	}

	// cv::TrackerCSRT::Params::admm_iterations() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1520
	// ("cv::TrackerCSRT::Params::admm_iterations", vec![(pred!(const, [], []), _)]),
	int cv_TrackerCSRT_Params_propAdmm_iterations_const(const cv::TrackerCSRT::Params* instance) {
			int ret = instance->admm_iterations;
			return ret;
	}

	// cv::TrackerCSRT::Params::setAdmm_iterations(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1520
	// ("cv::TrackerCSRT::Params::setAdmm_iterations", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_TrackerCSRT_Params_propAdmm_iterations_const_int(cv::TrackerCSRT::Params* instance, const int val) {
			instance->admm_iterations = val;
	}

	// cv::TrackerCSRT::Params::histogram_bins() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1521
	// ("cv::TrackerCSRT::Params::histogram_bins", vec![(pred!(const, [], []), _)]),
	int cv_TrackerCSRT_Params_propHistogram_bins_const(const cv::TrackerCSRT::Params* instance) {
			int ret = instance->histogram_bins;
			return ret;
	}

	// cv::TrackerCSRT::Params::setHistogram_bins(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1521
	// ("cv::TrackerCSRT::Params::setHistogram_bins", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_TrackerCSRT_Params_propHistogram_bins_const_int(cv::TrackerCSRT::Params* instance, const int val) {
			instance->histogram_bins = val;
	}

	// cv::TrackerCSRT::Params::histogram_lr() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1522
	// ("cv::TrackerCSRT::Params::histogram_lr", vec![(pred!(const, [], []), _)]),
	float cv_TrackerCSRT_Params_propHistogram_lr_const(const cv::TrackerCSRT::Params* instance) {
			float ret = instance->histogram_lr;
			return ret;
	}

	// cv::TrackerCSRT::Params::setHistogram_lr(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1522
	// ("cv::TrackerCSRT::Params::setHistogram_lr", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_TrackerCSRT_Params_propHistogram_lr_const_float(cv::TrackerCSRT::Params* instance, const float val) {
			instance->histogram_lr = val;
	}

	// cv::TrackerCSRT::Params::background_ratio() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1523
	// ("cv::TrackerCSRT::Params::background_ratio", vec![(pred!(const, [], []), _)]),
	int cv_TrackerCSRT_Params_propBackground_ratio_const(const cv::TrackerCSRT::Params* instance) {
			int ret = instance->background_ratio;
			return ret;
	}

	// cv::TrackerCSRT::Params::setBackground_ratio(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1523
	// ("cv::TrackerCSRT::Params::setBackground_ratio", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_TrackerCSRT_Params_propBackground_ratio_const_int(cv::TrackerCSRT::Params* instance, const int val) {
			instance->background_ratio = val;
	}

	// cv::TrackerCSRT::Params::number_of_scales() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1524
	// ("cv::TrackerCSRT::Params::number_of_scales", vec![(pred!(const, [], []), _)]),
	int cv_TrackerCSRT_Params_propNumber_of_scales_const(const cv::TrackerCSRT::Params* instance) {
			int ret = instance->number_of_scales;
			return ret;
	}

	// cv::TrackerCSRT::Params::setNumber_of_scales(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1524
	// ("cv::TrackerCSRT::Params::setNumber_of_scales", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_TrackerCSRT_Params_propNumber_of_scales_const_int(cv::TrackerCSRT::Params* instance, const int val) {
			instance->number_of_scales = val;
	}

	// cv::TrackerCSRT::Params::scale_sigma_factor() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1525
	// ("cv::TrackerCSRT::Params::scale_sigma_factor", vec![(pred!(const, [], []), _)]),
	float cv_TrackerCSRT_Params_propScale_sigma_factor_const(const cv::TrackerCSRT::Params* instance) {
			float ret = instance->scale_sigma_factor;
			return ret;
	}

	// cv::TrackerCSRT::Params::setScale_sigma_factor(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1525
	// ("cv::TrackerCSRT::Params::setScale_sigma_factor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_TrackerCSRT_Params_propScale_sigma_factor_const_float(cv::TrackerCSRT::Params* instance, const float val) {
			instance->scale_sigma_factor = val;
	}

	// cv::TrackerCSRT::Params::scale_model_max_area() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1526
	// ("cv::TrackerCSRT::Params::scale_model_max_area", vec![(pred!(const, [], []), _)]),
	float cv_TrackerCSRT_Params_propScale_model_max_area_const(const cv::TrackerCSRT::Params* instance) {
			float ret = instance->scale_model_max_area;
			return ret;
	}

	// cv::TrackerCSRT::Params::setScale_model_max_area(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1526
	// ("cv::TrackerCSRT::Params::setScale_model_max_area", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_TrackerCSRT_Params_propScale_model_max_area_const_float(cv::TrackerCSRT::Params* instance, const float val) {
			instance->scale_model_max_area = val;
	}

	// cv::TrackerCSRT::Params::scale_lr() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1527
	// ("cv::TrackerCSRT::Params::scale_lr", vec![(pred!(const, [], []), _)]),
	float cv_TrackerCSRT_Params_propScale_lr_const(const cv::TrackerCSRT::Params* instance) {
			float ret = instance->scale_lr;
			return ret;
	}

	// cv::TrackerCSRT::Params::setScale_lr(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1527
	// ("cv::TrackerCSRT::Params::setScale_lr", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_TrackerCSRT_Params_propScale_lr_const_float(cv::TrackerCSRT::Params* instance, const float val) {
			instance->scale_lr = val;
	}

	// cv::TrackerCSRT::Params::scale_step() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1528
	// ("cv::TrackerCSRT::Params::scale_step", vec![(pred!(const, [], []), _)]),
	float cv_TrackerCSRT_Params_propScale_step_const(const cv::TrackerCSRT::Params* instance) {
			float ret = instance->scale_step;
			return ret;
	}

	// cv::TrackerCSRT::Params::setScale_step(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1528
	// ("cv::TrackerCSRT::Params::setScale_step", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_TrackerCSRT_Params_propScale_step_const_float(cv::TrackerCSRT::Params* instance, const float val) {
			instance->scale_step = val;
	}

	// cv::TrackerCSRT::Params::psr_threshold() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1530
	// ("cv::TrackerCSRT::Params::psr_threshold", vec![(pred!(const, [], []), _)]),
	float cv_TrackerCSRT_Params_propPsr_threshold_const(const cv::TrackerCSRT::Params* instance) {
			float ret = instance->psr_threshold;
			return ret;
	}

	// cv::TrackerCSRT::Params::setPsr_threshold(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1530
	// ("cv::TrackerCSRT::Params::setPsr_threshold", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_TrackerCSRT_Params_propPsr_threshold_const_float(cv::TrackerCSRT::Params* instance, const float val) {
			instance->psr_threshold = val;
	}

	// cv::TrackerCSRT::Params::delete() generated
	// ("cv::TrackerCSRT::Params::delete", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerCSRT_Params_delete(cv::TrackerCSRT::Params* instance) {
			delete instance;
	}

	// compute(const std::vector<Mat> &, Mat &)(CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:80
	// ("cv::TrackerFeature::compute", vec![(pred!(mut, ["images", "response"], ["const std::vector<cv::Mat>*", "cv::Mat*"]), _)]),
	void cv_TrackerFeature_compute_const_vectorLMatGR_MatR(cv::TrackerFeature* instance, const std::vector<cv::Mat>* images, cv::Mat* response, ResultVoid* ocvrs_return) {
		try {
			instance->compute(*images, *response);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:95
	// ("cv::TrackerFeature::create", vec![(pred!(mut, ["trackerFeatureType"], ["const cv::String*"]), _)]),
	void cv_TrackerFeature_create_const_StringR(const char* trackerFeatureType, Result<cv::Ptr<cv::TrackerFeature>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::TrackerFeature> ret = cv::TrackerFeature::create(cv::String(trackerFeatureType));
			Ok(new cv::Ptr<cv::TrackerFeature>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// selection(Mat &, int)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:103
	// ("cv::TrackerFeature::selection", vec![(pred!(mut, ["response", "npoints"], ["cv::Mat*", "int"]), _)]),
	void cv_TrackerFeature_selection_MatR_int(cv::TrackerFeature* instance, cv::Mat* response, int npoints, ResultVoid* ocvrs_return) {
		try {
			instance->selection(*response, npoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getClassName()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:107
	// ("cv::TrackerFeature::getClassName", vec![(pred!(const, [], []), _)]),
	void cv_TrackerFeature_getClassName_const(const cv::TrackerFeature* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getClassName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TrackerFeature::to_TrackerFeatureFeature2d() generated
	// ("cv::TrackerFeature::to_TrackerFeatureFeature2d", vec![(pred!(mut, [], []), _)]),
	cv::TrackerFeatureFeature2d* cv_TrackerFeature_to_TrackerFeatureFeature2d(cv::TrackerFeature* instance) {
			return dynamic_cast<cv::TrackerFeatureFeature2d*>(instance);
	}

	// cv::TrackerFeature::to_TrackerFeatureHAAR() generated
	// ("cv::TrackerFeature::to_TrackerFeatureHAAR", vec![(pred!(mut, [], []), _)]),
	cv::TrackerFeatureHAAR* cv_TrackerFeature_to_TrackerFeatureHAAR(cv::TrackerFeature* instance) {
			return dynamic_cast<cv::TrackerFeatureHAAR*>(instance);
	}

	// cv::TrackerFeature::to_TrackerFeatureHOG() generated
	// ("cv::TrackerFeature::to_TrackerFeatureHOG", vec![(pred!(mut, [], []), _)]),
	cv::TrackerFeatureHOG* cv_TrackerFeature_to_TrackerFeatureHOG(cv::TrackerFeature* instance) {
			return dynamic_cast<cv::TrackerFeatureHOG*>(instance);
	}

	// cv::TrackerFeature::to_TrackerFeatureLBP() generated
	// ("cv::TrackerFeature::to_TrackerFeatureLBP", vec![(pred!(mut, [], []), _)]),
	cv::TrackerFeatureLBP* cv_TrackerFeature_to_TrackerFeatureLBP(cv::TrackerFeature* instance) {
			return dynamic_cast<cv::TrackerFeatureLBP*>(instance);
	}

	// cv::TrackerFeature::delete() generated
	// ("cv::TrackerFeature::delete", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerFeature_delete(cv::TrackerFeature* instance) {
			delete instance;
	}

	// TrackerFeatureFeature2d(String, String)(InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:940
	// ("cv::TrackerFeatureFeature2d::TrackerFeatureFeature2d", vec![(pred!(mut, ["detectorType", "descriptorType"], ["cv::String", "cv::String"]), _)]),
	void cv_TrackerFeatureFeature2d_TrackerFeatureFeature2d_String_String(const char* detectorType, const char* descriptorType, Result<cv::TrackerFeatureFeature2d*>* ocvrs_return) {
		try {
			cv::TrackerFeatureFeature2d* ret = new cv::TrackerFeatureFeature2d(cv::String(detectorType), cv::String(descriptorType));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// selection(Mat &, int)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:944
	// ("cv::TrackerFeatureFeature2d::selection", vec![(pred!(mut, ["response", "npoints"], ["cv::Mat*", "int"]), _)]),
	void cv_TrackerFeatureFeature2d_selection_MatR_int(cv::TrackerFeatureFeature2d* instance, cv::Mat* response, int npoints, ResultVoid* ocvrs_return) {
		try {
			instance->selection(*response, npoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TrackerFeatureFeature2d::to_TrackerFeature() generated
	// ("cv::TrackerFeatureFeature2d::to_TrackerFeature", vec![(pred!(mut, [], []), _)]),
	cv::TrackerFeature* cv_TrackerFeatureFeature2d_to_TrackerFeature(cv::TrackerFeatureFeature2d* instance) {
			return dynamic_cast<cv::TrackerFeature*>(instance);
	}

	// cv::TrackerFeatureFeature2d::delete() generated
	// ("cv::TrackerFeatureFeature2d::delete", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerFeatureFeature2d_delete(cv::TrackerFeatureFeature2d* instance) {
			delete instance;
	}

	// TrackerFeatureHAAR(const TrackerFeatureHAAR::Params &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:991
	// ("cv::TrackerFeatureHAAR::TrackerFeatureHAAR", vec![(pred!(mut, ["parameters"], ["const cv::TrackerFeatureHAAR::Params*"]), _)]),
	void cv_TrackerFeatureHAAR_TrackerFeatureHAAR_const_ParamsR(const cv::TrackerFeatureHAAR::Params* parameters, Result<cv::TrackerFeatureHAAR*>* ocvrs_return) {
		try {
			cv::TrackerFeatureHAAR* ret = new cv::TrackerFeatureHAAR(*parameters);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TrackerFeatureHAAR::TrackerFeatureHAAR() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:991
	// ("cv::TrackerFeatureHAAR::TrackerFeatureHAAR", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerFeatureHAAR_TrackerFeatureHAAR(Result<cv::TrackerFeatureHAAR*>* ocvrs_return) {
		try {
			cv::TrackerFeatureHAAR* ret = new cv::TrackerFeatureHAAR();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// extractSelected(const std::vector<int>, const std::vector<Mat> &, Mat &)(CppPassByVoidPtr, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1000
	// ("cv::TrackerFeatureHAAR::extractSelected", vec![(pred!(mut, ["selFeatures", "images", "response"], ["const std::vector<int>", "const std::vector<cv::Mat>*", "cv::Mat*"]), _)]),
	void cv_TrackerFeatureHAAR_extractSelected_const_vectorLintG_const_vectorLMatGR_MatR(cv::TrackerFeatureHAAR* instance, const std::vector<int>* selFeatures, const std::vector<cv::Mat>* images, cv::Mat* response, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->extractSelected(*selFeatures, *images, *response);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// selection(Mat &, int)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1008
	// ("cv::TrackerFeatureHAAR::selection", vec![(pred!(mut, ["response", "npoints"], ["cv::Mat*", "int"]), _)]),
	void cv_TrackerFeatureHAAR_selection_MatR_int(cv::TrackerFeatureHAAR* instance, cv::Mat* response, int npoints, ResultVoid* ocvrs_return) {
		try {
			instance->selection(*response, npoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// swapFeature(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1014
	// ("cv::TrackerFeatureHAAR::swapFeature", vec![(pred!(mut, ["source", "target"], ["int", "int"]), _)]),
	void cv_TrackerFeatureHAAR_swapFeature_int_int(cv::TrackerFeatureHAAR* instance, int source, int target, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->swapFeature(source, target);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// swapFeature(int, CvHaarEvaluator::FeatureHaar &)(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1020
	// ("cv::TrackerFeatureHAAR::swapFeature", vec![(pred!(mut, ["id", "feature"], ["int", "cv::CvHaarEvaluator::FeatureHaar*"]), _)]),
	void cv_TrackerFeatureHAAR_swapFeature_int_FeatureHaarR(cv::TrackerFeatureHAAR* instance, int id, cv::CvHaarEvaluator::FeatureHaar* feature, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->swapFeature(id, *feature);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getFeatureAt(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1025
	// ("cv::TrackerFeatureHAAR::getFeatureAt", vec![(pred!(mut, ["id"], ["int"]), _)]),
	void cv_TrackerFeatureHAAR_getFeatureAt_int(cv::TrackerFeatureHAAR* instance, int id, Result<cv::CvHaarEvaluator::FeatureHaar*>* ocvrs_return) {
		try {
			cv::CvHaarEvaluator::FeatureHaar ret = instance->getFeatureAt(id);
			Ok(new cv::CvHaarEvaluator::FeatureHaar(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TrackerFeatureHAAR::to_TrackerFeature() generated
	// ("cv::TrackerFeatureHAAR::to_TrackerFeature", vec![(pred!(mut, [], []), _)]),
	cv::TrackerFeature* cv_TrackerFeatureHAAR_to_TrackerFeature(cv::TrackerFeatureHAAR* instance) {
			return dynamic_cast<cv::TrackerFeature*>(instance);
	}

	// cv::TrackerFeatureHAAR::delete() generated
	// ("cv::TrackerFeatureHAAR::delete", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerFeatureHAAR_delete(cv::TrackerFeatureHAAR* instance) {
			delete instance;
	}

	// Params()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:982
	// ("cv::TrackerFeatureHAAR::Params::Params", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerFeatureHAAR_Params_Params(Result<cv::TrackerFeatureHAAR::Params*>* ocvrs_return) {
		try {
			cv::TrackerFeatureHAAR::Params* ret = new cv::TrackerFeatureHAAR::Params();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TrackerFeatureHAAR::Params::numFeatures() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:983
	// ("cv::TrackerFeatureHAAR::Params::numFeatures", vec![(pred!(const, [], []), _)]),
	int cv_TrackerFeatureHAAR_Params_propNumFeatures_const(const cv::TrackerFeatureHAAR::Params* instance) {
			int ret = instance->numFeatures;
			return ret;
	}

	// cv::TrackerFeatureHAAR::Params::setNumFeatures(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:983
	// ("cv::TrackerFeatureHAAR::Params::setNumFeatures", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_TrackerFeatureHAAR_Params_propNumFeatures_const_int(cv::TrackerFeatureHAAR::Params* instance, const int val) {
			instance->numFeatures = val;
	}

	// cv::TrackerFeatureHAAR::Params::rectSize() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:984
	// ("cv::TrackerFeatureHAAR::Params::rectSize", vec![(pred!(const, [], []), _)]),
	void cv_TrackerFeatureHAAR_Params_propRectSize_const(const cv::TrackerFeatureHAAR::Params* instance, cv::Size* ocvrs_return) {
			cv::Size ret = instance->rectSize;
			*ocvrs_return = ret;
	}

	// cv::TrackerFeatureHAAR::Params::setRectSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:984
	// ("cv::TrackerFeatureHAAR::Params::setRectSize", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	void cv_TrackerFeatureHAAR_Params_propRectSize_const_Size(cv::TrackerFeatureHAAR::Params* instance, const cv::Size* val) {
			instance->rectSize = *val;
	}

	// cv::TrackerFeatureHAAR::Params::isIntegral() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:985
	// ("cv::TrackerFeatureHAAR::Params::isIntegral", vec![(pred!(const, [], []), _)]),
	bool cv_TrackerFeatureHAAR_Params_propIsIntegral_const(const cv::TrackerFeatureHAAR::Params* instance) {
			bool ret = instance->isIntegral;
			return ret;
	}

	// cv::TrackerFeatureHAAR::Params::setIsIntegral(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:985
	// ("cv::TrackerFeatureHAAR::Params::setIsIntegral", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_TrackerFeatureHAAR_Params_propIsIntegral_const_bool(cv::TrackerFeatureHAAR::Params* instance, const bool val) {
			instance->isIntegral = val;
	}

	// cv::TrackerFeatureHAAR::Params::delete() generated
	// ("cv::TrackerFeatureHAAR::Params::delete", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerFeatureHAAR_Params_delete(cv::TrackerFeatureHAAR::Params* instance) {
			delete instance;
	}

	// TrackerFeatureHOG()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:962
	// ("cv::TrackerFeatureHOG::TrackerFeatureHOG", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerFeatureHOG_TrackerFeatureHOG(Result<cv::TrackerFeatureHOG*>* ocvrs_return) {
		try {
			cv::TrackerFeatureHOG* ret = new cv::TrackerFeatureHOG();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// selection(Mat &, int)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:966
	// ("cv::TrackerFeatureHOG::selection", vec![(pred!(mut, ["response", "npoints"], ["cv::Mat*", "int"]), _)]),
	void cv_TrackerFeatureHOG_selection_MatR_int(cv::TrackerFeatureHOG* instance, cv::Mat* response, int npoints, ResultVoid* ocvrs_return) {
		try {
			instance->selection(*response, npoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TrackerFeatureHOG::to_TrackerFeature() generated
	// ("cv::TrackerFeatureHOG::to_TrackerFeature", vec![(pred!(mut, [], []), _)]),
	cv::TrackerFeature* cv_TrackerFeatureHOG_to_TrackerFeature(cv::TrackerFeatureHOG* instance) {
			return dynamic_cast<cv::TrackerFeature*>(instance);
	}

	// cv::TrackerFeatureHOG::delete() generated
	// ("cv::TrackerFeatureHOG::delete", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerFeatureHOG_delete(cv::TrackerFeatureHOG* instance) {
			delete instance;
	}

	// TrackerFeatureLBP()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1043
	// ("cv::TrackerFeatureLBP::TrackerFeatureLBP", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerFeatureLBP_TrackerFeatureLBP(Result<cv::TrackerFeatureLBP*>* ocvrs_return) {
		try {
			cv::TrackerFeatureLBP* ret = new cv::TrackerFeatureLBP();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// selection(Mat &, int)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1047
	// ("cv::TrackerFeatureLBP::selection", vec![(pred!(mut, ["response", "npoints"], ["cv::Mat*", "int"]), _)]),
	void cv_TrackerFeatureLBP_selection_MatR_int(cv::TrackerFeatureLBP* instance, cv::Mat* response, int npoints, ResultVoid* ocvrs_return) {
		try {
			instance->selection(*response, npoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TrackerFeatureLBP::to_TrackerFeature() generated
	// ("cv::TrackerFeatureLBP::to_TrackerFeature", vec![(pred!(mut, [], []), _)]),
	cv::TrackerFeature* cv_TrackerFeatureLBP_to_TrackerFeature(cv::TrackerFeatureLBP* instance) {
			return dynamic_cast<cv::TrackerFeature*>(instance);
	}

	// cv::TrackerFeatureLBP::delete() generated
	// ("cv::TrackerFeatureLBP::delete", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerFeatureLBP_delete(cv::TrackerFeatureLBP* instance) {
			delete instance;
	}

	// TrackerFeatureSet()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:132
	// ("cv::TrackerFeatureSet::TrackerFeatureSet", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerFeatureSet_TrackerFeatureSet(Result<cv::TrackerFeatureSet*>* ocvrs_return) {
		try {
			cv::TrackerFeatureSet* ret = new cv::TrackerFeatureSet();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// extraction(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:139
	// ("cv::TrackerFeatureSet::extraction", vec![(pred!(mut, ["images"], ["const std::vector<cv::Mat>*"]), _)]),
	void cv_TrackerFeatureSet_extraction_const_vectorLMatGR(cv::TrackerFeatureSet* instance, const std::vector<cv::Mat>* images, ResultVoid* ocvrs_return) {
		try {
			instance->extraction(*images);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// selection()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:143
	// ("cv::TrackerFeatureSet::selection", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerFeatureSet_selection(cv::TrackerFeatureSet* instance, ResultVoid* ocvrs_return) {
		try {
			instance->selection();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// removeOutliers()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:147
	// ("cv::TrackerFeatureSet::removeOutliers", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerFeatureSet_removeOutliers(cv::TrackerFeatureSet* instance, ResultVoid* ocvrs_return) {
		try {
			instance->removeOutliers();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// addTrackerFeature(String)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:174
	// ("cv::TrackerFeatureSet::addTrackerFeature", vec![(pred!(mut, ["trackerFeatureType"], ["cv::String"]), _)]),
	void cv_TrackerFeatureSet_addTrackerFeature_String(cv::TrackerFeatureSet* instance, const char* trackerFeatureType, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->addTrackerFeature(cv::String(trackerFeatureType));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// addTrackerFeature(Ptr<TrackerFeature> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:179
	// ("cv::TrackerFeatureSet::addTrackerFeature", vec![(pred!(mut, ["feature"], ["cv::Ptr<cv::TrackerFeature>*"]), _)]),
	void cv_TrackerFeatureSet_addTrackerFeature_PtrLTrackerFeatureGR(cv::TrackerFeatureSet* instance, cv::Ptr<cv::TrackerFeature>* feature, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->addTrackerFeature(*feature);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTrackerFeature()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:183
	// ("cv::TrackerFeatureSet::getTrackerFeature", vec![(pred!(const, [], []), _)]),
	void cv_TrackerFeatureSet_getTrackerFeature_const(const cv::TrackerFeatureSet* instance, Result<std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>>*>* ocvrs_return) {
		try {
			const std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>> ret = instance->getTrackerFeature();
			Ok(new const std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getResponses()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:189
	// ("cv::TrackerFeatureSet::getResponses", vec![(pred!(const, [], []), _)]),
	void cv_TrackerFeatureSet_getResponses_const(const cv::TrackerFeatureSet* instance, Result<std::vector<cv::Mat>*>* ocvrs_return) {
		try {
			const std::vector<cv::Mat> ret = instance->getResponses();
			Ok(new const std::vector<cv::Mat>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TrackerFeatureSet::delete() generated
	// ("cv::TrackerFeatureSet::delete", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerFeatureSet_delete(cv::TrackerFeatureSet* instance) {
			delete instance;
	}

	// create(const TrackerGOTURN::Params &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1302
	// ("cv::TrackerGOTURN::create", vec![(pred!(mut, ["parameters"], ["const cv::TrackerGOTURN::Params*"]), _)]),
	void cv_TrackerGOTURN_create_const_ParamsR(const cv::TrackerGOTURN::Params* parameters, Result<cv::Ptr<cv::TrackerGOTURN>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::TrackerGOTURN> ret = cv::TrackerGOTURN::create(*parameters);
			Ok(new cv::Ptr<cv::TrackerGOTURN>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1304
	// ("cv::TrackerGOTURN::create", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerGOTURN_create(Result<cv::Ptr<cv::TrackerGOTURN>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::TrackerGOTURN> ret = cv::TrackerGOTURN::create();
			Ok(new cv::Ptr<cv::TrackerGOTURN>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TrackerGOTURN::to_Algorithm() generated
	// ("cv::TrackerGOTURN::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_TrackerGOTURN_to_Algorithm(cv::TrackerGOTURN* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::TrackerGOTURN::to_Tracker() generated
	// ("cv::TrackerGOTURN::to_Tracker", vec![(pred!(mut, [], []), _)]),
	cv::Tracker* cv_TrackerGOTURN_to_Tracker(cv::TrackerGOTURN* instance) {
			return dynamic_cast<cv::Tracker*>(instance);
	}

	// cv::TrackerGOTURN::delete() generated
	// ("cv::TrackerGOTURN::delete", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerGOTURN_delete(cv::TrackerGOTURN* instance) {
			delete instance;
	}

	// Params()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1292
	// ("cv::TrackerGOTURN::Params::Params", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerGOTURN_Params_Params(Result<cv::TrackerGOTURN::Params*>* ocvrs_return) {
		try {
			cv::TrackerGOTURN::Params* ret = new cv::TrackerGOTURN::Params();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1293
	// ("cv::TrackerGOTURN::Params::read", vec![(pred!(mut, ["unnamed"], ["const cv::FileNode*"]), _)]),
	void cv_TrackerGOTURN_Params_read_const_FileNodeR(cv::TrackerGOTURN::Params* instance, const cv::FileNode* unnamed, ResultVoid* ocvrs_return) {
		try {
			instance->read(*unnamed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1294
	// ("cv::TrackerGOTURN::Params::write", vec![(pred!(const, ["unnamed"], ["cv::FileStorage*"]), _)]),
	void cv_TrackerGOTURN_Params_write_const_FileStorageR(const cv::TrackerGOTURN::Params* instance, cv::FileStorage* unnamed, ResultVoid* ocvrs_return) {
		try {
			instance->write(*unnamed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TrackerGOTURN::Params::modelTxt() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1295
	// ("cv::TrackerGOTURN::Params::modelTxt", vec![(pred!(const, [], []), _)]),
	void* cv_TrackerGOTURN_Params_propModelTxt_const(const cv::TrackerGOTURN::Params* instance) {
			cv::String ret = instance->modelTxt;
			return ocvrs_create_string(ret.c_str());
	}

	// cv::TrackerGOTURN::Params::setModelTxt(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1295
	// ("cv::TrackerGOTURN::Params::setModelTxt", vec![(pred!(mut, ["val"], ["const cv::String"]), _)]),
	void cv_TrackerGOTURN_Params_propModelTxt_const_String(cv::TrackerGOTURN::Params* instance, const char* val) {
			instance->modelTxt = cv::String(val);
	}

	// cv::TrackerGOTURN::Params::modelBin() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1296
	// ("cv::TrackerGOTURN::Params::modelBin", vec![(pred!(const, [], []), _)]),
	void* cv_TrackerGOTURN_Params_propModelBin_const(const cv::TrackerGOTURN::Params* instance) {
			cv::String ret = instance->modelBin;
			return ocvrs_create_string(ret.c_str());
	}

	// cv::TrackerGOTURN::Params::setModelBin(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1296
	// ("cv::TrackerGOTURN::Params::setModelBin", vec![(pred!(mut, ["val"], ["const cv::String"]), _)]),
	void cv_TrackerGOTURN_Params_propModelBin_const_String(cv::TrackerGOTURN::Params* instance, const char* val) {
			instance->modelBin = cv::String(val);
	}

	// cv::TrackerGOTURN::Params::delete() generated
	// ("cv::TrackerGOTURN::Params::delete", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerGOTURN_Params_delete(cv::TrackerGOTURN::Params* instance) {
			delete instance;
	}

	// setFeatureExtractor(void (*)(const Mat, const Rect, Mat &), bool)(Function, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1260
	// ("cv::TrackerKCF::setFeatureExtractor", vec![(pred!(mut, ["unnamed", "pca_func"], ["void (*)(const cv::Mat, const cv::Rect, cv::Mat&)", "bool"]), _)]),
	void cv_TrackerKCF_setFeatureExtractor_void__X__const_cv_Mat__const_cv_Rect__cv_MatR__bool(cv::TrackerKCF* instance, void (*unnamed)(const cv::Mat, const cv::Rect, cv::Mat&), bool pca_func, ResultVoid* ocvrs_return) {
		try {
			instance->setFeatureExtractor(unnamed, pca_func);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TrackerKCF::setFeatureExtractor(Function) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1260
	// ("cv::TrackerKCF::setFeatureExtractor", vec![(pred!(mut, ["unnamed"], ["void (*)(const cv::Mat, const cv::Rect, cv::Mat&)"]), _)]),
	void cv_TrackerKCF_setFeatureExtractor_void__X__const_cv_Mat__const_cv_Rect__cv_MatR_(cv::TrackerKCF* instance, void (*unnamed)(const cv::Mat, const cv::Rect, cv::Mat&), ResultVoid* ocvrs_return) {
		try {
			instance->setFeatureExtractor(unnamed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const TrackerKCF::Params &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1265
	// ("cv::TrackerKCF::create", vec![(pred!(mut, ["parameters"], ["const cv::TrackerKCF::Params*"]), _)]),
	void cv_TrackerKCF_create_const_ParamsR(const cv::TrackerKCF::Params* parameters, Result<cv::Ptr<cv::TrackerKCF>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::TrackerKCF> ret = cv::TrackerKCF::create(*parameters);
			Ok(new cv::Ptr<cv::TrackerKCF>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1267
	// ("cv::TrackerKCF::create", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerKCF_create(Result<cv::Ptr<cv::TrackerKCF>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::TrackerKCF> ret = cv::TrackerKCF::create();
			Ok(new cv::Ptr<cv::TrackerKCF>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TrackerKCF::to_Algorithm() generated
	// ("cv::TrackerKCF::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_TrackerKCF_to_Algorithm(cv::TrackerKCF* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::TrackerKCF::to_Tracker() generated
	// ("cv::TrackerKCF::to_Tracker", vec![(pred!(mut, [], []), _)]),
	cv::Tracker* cv_TrackerKCF_to_Tracker(cv::TrackerKCF* instance) {
			return dynamic_cast<cv::Tracker*>(instance);
	}

	// cv::TrackerKCF::delete() generated
	// ("cv::TrackerKCF::delete", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerKCF_delete(cv::TrackerKCF* instance) {
			delete instance;
	}

	// Params()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1232
	// ("cv::TrackerKCF::Params::Params", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerKCF_Params_Params(Result<cv::TrackerKCF::Params*>* ocvrs_return) {
		try {
			cv::TrackerKCF::Params* ret = new cv::TrackerKCF::Params();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1237
	// ("cv::TrackerKCF::Params::read", vec![(pred!(mut, ["unnamed"], ["const cv::FileNode*"]), _)]),
	void cv_TrackerKCF_Params_read_const_FileNodeR(cv::TrackerKCF::Params* instance, const cv::FileNode* unnamed, ResultVoid* ocvrs_return) {
		try {
			instance->read(*unnamed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1242
	// ("cv::TrackerKCF::Params::write", vec![(pred!(const, ["unnamed"], ["cv::FileStorage*"]), _)]),
	void cv_TrackerKCF_Params_write_const_FileStorageR(const cv::TrackerKCF::Params* instance, cv::FileStorage* unnamed, ResultVoid* ocvrs_return) {
		try {
			instance->write(*unnamed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TrackerKCF::Params::detect_thresh() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1244
	// ("cv::TrackerKCF::Params::detect_thresh", vec![(pred!(const, [], []), _)]),
	float cv_TrackerKCF_Params_propDetect_thresh_const(const cv::TrackerKCF::Params* instance) {
			float ret = instance->detect_thresh;
			return ret;
	}

	// cv::TrackerKCF::Params::setDetect_thresh(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1244
	// ("cv::TrackerKCF::Params::setDetect_thresh", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_TrackerKCF_Params_propDetect_thresh_const_float(cv::TrackerKCF::Params* instance, const float val) {
			instance->detect_thresh = val;
	}

	// cv::TrackerKCF::Params::sigma() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1245
	// ("cv::TrackerKCF::Params::sigma", vec![(pred!(const, [], []), _)]),
	float cv_TrackerKCF_Params_propSigma_const(const cv::TrackerKCF::Params* instance) {
			float ret = instance->sigma;
			return ret;
	}

	// cv::TrackerKCF::Params::setSigma(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1245
	// ("cv::TrackerKCF::Params::setSigma", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_TrackerKCF_Params_propSigma_const_float(cv::TrackerKCF::Params* instance, const float val) {
			instance->sigma = val;
	}

	// cv::TrackerKCF::Params::lambda() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1246
	// ("cv::TrackerKCF::Params::lambda", vec![(pred!(const, [], []), _)]),
	float cv_TrackerKCF_Params_propLambda_const(const cv::TrackerKCF::Params* instance) {
			float ret = instance->lambda;
			return ret;
	}

	// cv::TrackerKCF::Params::setLambda(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1246
	// ("cv::TrackerKCF::Params::setLambda", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_TrackerKCF_Params_propLambda_const_float(cv::TrackerKCF::Params* instance, const float val) {
			instance->lambda = val;
	}

	// cv::TrackerKCF::Params::interp_factor() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1247
	// ("cv::TrackerKCF::Params::interp_factor", vec![(pred!(const, [], []), _)]),
	float cv_TrackerKCF_Params_propInterp_factor_const(const cv::TrackerKCF::Params* instance) {
			float ret = instance->interp_factor;
			return ret;
	}

	// cv::TrackerKCF::Params::setInterp_factor(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1247
	// ("cv::TrackerKCF::Params::setInterp_factor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_TrackerKCF_Params_propInterp_factor_const_float(cv::TrackerKCF::Params* instance, const float val) {
			instance->interp_factor = val;
	}

	// cv::TrackerKCF::Params::output_sigma_factor() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1248
	// ("cv::TrackerKCF::Params::output_sigma_factor", vec![(pred!(const, [], []), _)]),
	float cv_TrackerKCF_Params_propOutput_sigma_factor_const(const cv::TrackerKCF::Params* instance) {
			float ret = instance->output_sigma_factor;
			return ret;
	}

	// cv::TrackerKCF::Params::setOutput_sigma_factor(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1248
	// ("cv::TrackerKCF::Params::setOutput_sigma_factor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_TrackerKCF_Params_propOutput_sigma_factor_const_float(cv::TrackerKCF::Params* instance, const float val) {
			instance->output_sigma_factor = val;
	}

	// cv::TrackerKCF::Params::pca_learning_rate() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1249
	// ("cv::TrackerKCF::Params::pca_learning_rate", vec![(pred!(const, [], []), _)]),
	float cv_TrackerKCF_Params_propPca_learning_rate_const(const cv::TrackerKCF::Params* instance) {
			float ret = instance->pca_learning_rate;
			return ret;
	}

	// cv::TrackerKCF::Params::setPca_learning_rate(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1249
	// ("cv::TrackerKCF::Params::setPca_learning_rate", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_TrackerKCF_Params_propPca_learning_rate_const_float(cv::TrackerKCF::Params* instance, const float val) {
			instance->pca_learning_rate = val;
	}

	// cv::TrackerKCF::Params::resize() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1250
	// ("cv::TrackerKCF::Params::resize", vec![(pred!(const, [], []), _)]),
	bool cv_TrackerKCF_Params_propResize_const(const cv::TrackerKCF::Params* instance) {
			bool ret = instance->resize;
			return ret;
	}

	// cv::TrackerKCF::Params::setResize(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1250
	// ("cv::TrackerKCF::Params::setResize", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_TrackerKCF_Params_propResize_const_bool(cv::TrackerKCF::Params* instance, const bool val) {
			instance->resize = val;
	}

	// cv::TrackerKCF::Params::split_coeff() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1251
	// ("cv::TrackerKCF::Params::split_coeff", vec![(pred!(const, [], []), _)]),
	bool cv_TrackerKCF_Params_propSplit_coeff_const(const cv::TrackerKCF::Params* instance) {
			bool ret = instance->split_coeff;
			return ret;
	}

	// cv::TrackerKCF::Params::setSplit_coeff(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1251
	// ("cv::TrackerKCF::Params::setSplit_coeff", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_TrackerKCF_Params_propSplit_coeff_const_bool(cv::TrackerKCF::Params* instance, const bool val) {
			instance->split_coeff = val;
	}

	// cv::TrackerKCF::Params::wrap_kernel() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1252
	// ("cv::TrackerKCF::Params::wrap_kernel", vec![(pred!(const, [], []), _)]),
	bool cv_TrackerKCF_Params_propWrap_kernel_const(const cv::TrackerKCF::Params* instance) {
			bool ret = instance->wrap_kernel;
			return ret;
	}

	// cv::TrackerKCF::Params::setWrap_kernel(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1252
	// ("cv::TrackerKCF::Params::setWrap_kernel", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_TrackerKCF_Params_propWrap_kernel_const_bool(cv::TrackerKCF::Params* instance, const bool val) {
			instance->wrap_kernel = val;
	}

	// cv::TrackerKCF::Params::compress_feature() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1253
	// ("cv::TrackerKCF::Params::compress_feature", vec![(pred!(const, [], []), _)]),
	bool cv_TrackerKCF_Params_propCompress_feature_const(const cv::TrackerKCF::Params* instance) {
			bool ret = instance->compress_feature;
			return ret;
	}

	// cv::TrackerKCF::Params::setCompress_feature(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1253
	// ("cv::TrackerKCF::Params::setCompress_feature", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_TrackerKCF_Params_propCompress_feature_const_bool(cv::TrackerKCF::Params* instance, const bool val) {
			instance->compress_feature = val;
	}

	// cv::TrackerKCF::Params::max_patch_size() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1254
	// ("cv::TrackerKCF::Params::max_patch_size", vec![(pred!(const, [], []), _)]),
	int cv_TrackerKCF_Params_propMax_patch_size_const(const cv::TrackerKCF::Params* instance) {
			int ret = instance->max_patch_size;
			return ret;
	}

	// cv::TrackerKCF::Params::setMax_patch_size(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1254
	// ("cv::TrackerKCF::Params::setMax_patch_size", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_TrackerKCF_Params_propMax_patch_size_const_int(cv::TrackerKCF::Params* instance, const int val) {
			instance->max_patch_size = val;
	}

	// cv::TrackerKCF::Params::compressed_size() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1255
	// ("cv::TrackerKCF::Params::compressed_size", vec![(pred!(const, [], []), _)]),
	int cv_TrackerKCF_Params_propCompressed_size_const(const cv::TrackerKCF::Params* instance) {
			int ret = instance->compressed_size;
			return ret;
	}

	// cv::TrackerKCF::Params::setCompressed_size(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1255
	// ("cv::TrackerKCF::Params::setCompressed_size", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_TrackerKCF_Params_propCompressed_size_const_int(cv::TrackerKCF::Params* instance, const int val) {
			instance->compressed_size = val;
	}

	// cv::TrackerKCF::Params::desc_pca() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1256
	// ("cv::TrackerKCF::Params::desc_pca", vec![(pred!(const, [], []), _)]),
	int cv_TrackerKCF_Params_propDesc_pca_const(const cv::TrackerKCF::Params* instance) {
			int ret = instance->desc_pca;
			return ret;
	}

	// cv::TrackerKCF::Params::setDesc_pca(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1256
	// ("cv::TrackerKCF::Params::setDesc_pca", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_TrackerKCF_Params_propDesc_pca_const_int(cv::TrackerKCF::Params* instance, const int val) {
			instance->desc_pca = val;
	}

	// cv::TrackerKCF::Params::desc_npca() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1257
	// ("cv::TrackerKCF::Params::desc_npca", vec![(pred!(const, [], []), _)]),
	int cv_TrackerKCF_Params_propDesc_npca_const(const cv::TrackerKCF::Params* instance) {
			int ret = instance->desc_npca;
			return ret;
	}

	// cv::TrackerKCF::Params::setDesc_npca(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1257
	// ("cv::TrackerKCF::Params::setDesc_npca", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_TrackerKCF_Params_propDesc_npca_const_int(cv::TrackerKCF::Params* instance, const int val) {
			instance->desc_npca = val;
	}

	// cv::TrackerKCF::Params::delete() generated
	// ("cv::TrackerKCF::Params::delete", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerKCF_Params_delete(cv::TrackerKCF::Params* instance) {
			delete instance;
	}

	// create(const TrackerMIL::Params &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1087
	// ("cv::TrackerMIL::create", vec![(pred!(mut, ["parameters"], ["const cv::TrackerMIL::Params*"]), _)]),
	void cv_TrackerMIL_create_const_ParamsR(const cv::TrackerMIL::Params* parameters, Result<cv::Ptr<cv::TrackerMIL>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::TrackerMIL> ret = cv::TrackerMIL::create(*parameters);
			Ok(new cv::Ptr<cv::TrackerMIL>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1089
	// ("cv::TrackerMIL::create", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerMIL_create(Result<cv::Ptr<cv::TrackerMIL>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::TrackerMIL> ret = cv::TrackerMIL::create();
			Ok(new cv::Ptr<cv::TrackerMIL>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TrackerMIL::to_Algorithm() generated
	// ("cv::TrackerMIL::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_TrackerMIL_to_Algorithm(cv::TrackerMIL* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::TrackerMIL::to_Tracker() generated
	// ("cv::TrackerMIL::to_Tracker", vec![(pred!(mut, [], []), _)]),
	cv::Tracker* cv_TrackerMIL_to_Tracker(cv::TrackerMIL* instance) {
			return dynamic_cast<cv::Tracker*>(instance);
	}

	// cv::TrackerMIL::delete() generated
	// ("cv::TrackerMIL::delete", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerMIL_delete(cv::TrackerMIL* instance) {
			delete instance;
	}

	// Params()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1070
	// ("cv::TrackerMIL::Params::Params", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerMIL_Params_Params(Result<cv::TrackerMIL::Params*>* ocvrs_return) {
		try {
			cv::TrackerMIL::Params* ret = new cv::TrackerMIL::Params();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1080
	// ("cv::TrackerMIL::Params::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	void cv_TrackerMIL_Params_read_const_FileNodeR(cv::TrackerMIL::Params* instance, const cv::FileNode* fn, ResultVoid* ocvrs_return) {
		try {
			instance->read(*fn);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1081
	// ("cv::TrackerMIL::Params::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	void cv_TrackerMIL_Params_write_const_FileStorageR(const cv::TrackerMIL::Params* instance, cv::FileStorage* fs, ResultVoid* ocvrs_return) {
		try {
			instance->write(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TrackerMIL::Params::samplerInitInRadius() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1072
	// ("cv::TrackerMIL::Params::samplerInitInRadius", vec![(pred!(const, [], []), _)]),
	float cv_TrackerMIL_Params_propSamplerInitInRadius_const(const cv::TrackerMIL::Params* instance) {
			float ret = instance->samplerInitInRadius;
			return ret;
	}

	// cv::TrackerMIL::Params::setSamplerInitInRadius(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1072
	// ("cv::TrackerMIL::Params::setSamplerInitInRadius", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_TrackerMIL_Params_propSamplerInitInRadius_const_float(cv::TrackerMIL::Params* instance, const float val) {
			instance->samplerInitInRadius = val;
	}

	// cv::TrackerMIL::Params::samplerInitMaxNegNum() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1073
	// ("cv::TrackerMIL::Params::samplerInitMaxNegNum", vec![(pred!(const, [], []), _)]),
	int cv_TrackerMIL_Params_propSamplerInitMaxNegNum_const(const cv::TrackerMIL::Params* instance) {
			int ret = instance->samplerInitMaxNegNum;
			return ret;
	}

	// cv::TrackerMIL::Params::setSamplerInitMaxNegNum(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1073
	// ("cv::TrackerMIL::Params::setSamplerInitMaxNegNum", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_TrackerMIL_Params_propSamplerInitMaxNegNum_const_int(cv::TrackerMIL::Params* instance, const int val) {
			instance->samplerInitMaxNegNum = val;
	}

	// cv::TrackerMIL::Params::samplerSearchWinSize() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1074
	// ("cv::TrackerMIL::Params::samplerSearchWinSize", vec![(pred!(const, [], []), _)]),
	float cv_TrackerMIL_Params_propSamplerSearchWinSize_const(const cv::TrackerMIL::Params* instance) {
			float ret = instance->samplerSearchWinSize;
			return ret;
	}

	// cv::TrackerMIL::Params::setSamplerSearchWinSize(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1074
	// ("cv::TrackerMIL::Params::setSamplerSearchWinSize", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_TrackerMIL_Params_propSamplerSearchWinSize_const_float(cv::TrackerMIL::Params* instance, const float val) {
			instance->samplerSearchWinSize = val;
	}

	// cv::TrackerMIL::Params::samplerTrackInRadius() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1075
	// ("cv::TrackerMIL::Params::samplerTrackInRadius", vec![(pred!(const, [], []), _)]),
	float cv_TrackerMIL_Params_propSamplerTrackInRadius_const(const cv::TrackerMIL::Params* instance) {
			float ret = instance->samplerTrackInRadius;
			return ret;
	}

	// cv::TrackerMIL::Params::setSamplerTrackInRadius(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1075
	// ("cv::TrackerMIL::Params::setSamplerTrackInRadius", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_TrackerMIL_Params_propSamplerTrackInRadius_const_float(cv::TrackerMIL::Params* instance, const float val) {
			instance->samplerTrackInRadius = val;
	}

	// cv::TrackerMIL::Params::samplerTrackMaxPosNum() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1076
	// ("cv::TrackerMIL::Params::samplerTrackMaxPosNum", vec![(pred!(const, [], []), _)]),
	int cv_TrackerMIL_Params_propSamplerTrackMaxPosNum_const(const cv::TrackerMIL::Params* instance) {
			int ret = instance->samplerTrackMaxPosNum;
			return ret;
	}

	// cv::TrackerMIL::Params::setSamplerTrackMaxPosNum(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1076
	// ("cv::TrackerMIL::Params::setSamplerTrackMaxPosNum", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_TrackerMIL_Params_propSamplerTrackMaxPosNum_const_int(cv::TrackerMIL::Params* instance, const int val) {
			instance->samplerTrackMaxPosNum = val;
	}

	// cv::TrackerMIL::Params::samplerTrackMaxNegNum() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1077
	// ("cv::TrackerMIL::Params::samplerTrackMaxNegNum", vec![(pred!(const, [], []), _)]),
	int cv_TrackerMIL_Params_propSamplerTrackMaxNegNum_const(const cv::TrackerMIL::Params* instance) {
			int ret = instance->samplerTrackMaxNegNum;
			return ret;
	}

	// cv::TrackerMIL::Params::setSamplerTrackMaxNegNum(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1077
	// ("cv::TrackerMIL::Params::setSamplerTrackMaxNegNum", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_TrackerMIL_Params_propSamplerTrackMaxNegNum_const_int(cv::TrackerMIL::Params* instance, const int val) {
			instance->samplerTrackMaxNegNum = val;
	}

	// cv::TrackerMIL::Params::featureSetNumFeatures() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1078
	// ("cv::TrackerMIL::Params::featureSetNumFeatures", vec![(pred!(const, [], []), _)]),
	int cv_TrackerMIL_Params_propFeatureSetNumFeatures_const(const cv::TrackerMIL::Params* instance) {
			int ret = instance->featureSetNumFeatures;
			return ret;
	}

	// cv::TrackerMIL::Params::setFeatureSetNumFeatures(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1078
	// ("cv::TrackerMIL::Params::setFeatureSetNumFeatures", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_TrackerMIL_Params_propFeatureSetNumFeatures_const_int(cv::TrackerMIL::Params* instance, const int val) {
			instance->featureSetNumFeatures = val;
	}

	// cv::TrackerMIL::Params::delete() generated
	// ("cv::TrackerMIL::Params::delete", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerMIL_Params_delete(cv::TrackerMIL::Params* instance) {
			delete instance;
	}

	// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1320
	// ("cv::TrackerMOSSE::create", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerMOSSE_create(Result<cv::Ptr<cv::TrackerMOSSE>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::TrackerMOSSE> ret = cv::TrackerMOSSE::create();
			Ok(new cv::Ptr<cv::TrackerMOSSE>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TrackerMOSSE::to_Algorithm() generated
	// ("cv::TrackerMOSSE::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_TrackerMOSSE_to_Algorithm(cv::TrackerMOSSE* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::TrackerMOSSE::to_Tracker() generated
	// ("cv::TrackerMOSSE::to_Tracker", vec![(pred!(mut, [], []), _)]),
	cv::Tracker* cv_TrackerMOSSE_to_Tracker(cv::TrackerMOSSE* instance) {
			return dynamic_cast<cv::Tracker*>(instance);
	}

	// cv::TrackerMOSSE::delete() generated
	// ("cv::TrackerMOSSE::delete", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerMOSSE_delete(cv::TrackerMOSSE* instance) {
			delete instance;
	}

	// create(const TrackerMedianFlow::Params &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1164
	// ("cv::TrackerMedianFlow::create", vec![(pred!(mut, ["parameters"], ["const cv::TrackerMedianFlow::Params*"]), _)]),
	void cv_TrackerMedianFlow_create_const_ParamsR(const cv::TrackerMedianFlow::Params* parameters, Result<cv::Ptr<cv::TrackerMedianFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::TrackerMedianFlow> ret = cv::TrackerMedianFlow::create(*parameters);
			Ok(new cv::Ptr<cv::TrackerMedianFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1166
	// ("cv::TrackerMedianFlow::create", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerMedianFlow_create(Result<cv::Ptr<cv::TrackerMedianFlow>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::TrackerMedianFlow> ret = cv::TrackerMedianFlow::create();
			Ok(new cv::Ptr<cv::TrackerMedianFlow>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TrackerMedianFlow::to_Algorithm() generated
	// ("cv::TrackerMedianFlow::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_TrackerMedianFlow_to_Algorithm(cv::TrackerMedianFlow* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::TrackerMedianFlow::to_Tracker() generated
	// ("cv::TrackerMedianFlow::to_Tracker", vec![(pred!(mut, [], []), _)]),
	cv::Tracker* cv_TrackerMedianFlow_to_Tracker(cv::TrackerMedianFlow* instance) {
			return dynamic_cast<cv::Tracker*>(instance);
	}

	// cv::TrackerMedianFlow::delete() generated
	// ("cv::TrackerMedianFlow::delete", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerMedianFlow_delete(cv::TrackerMedianFlow* instance) {
			delete instance;
	}

	// Params()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1147
	// ("cv::TrackerMedianFlow::Params::Params", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerMedianFlow_Params_Params(Result<cv::TrackerMedianFlow::Params*>* ocvrs_return) {
		try {
			cv::TrackerMedianFlow::Params* ret = new cv::TrackerMedianFlow::Params();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1157
	// ("cv::TrackerMedianFlow::Params::read", vec![(pred!(mut, ["unnamed"], ["const cv::FileNode*"]), _)]),
	void cv_TrackerMedianFlow_Params_read_const_FileNodeR(cv::TrackerMedianFlow::Params* instance, const cv::FileNode* unnamed, ResultVoid* ocvrs_return) {
		try {
			instance->read(*unnamed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1158
	// ("cv::TrackerMedianFlow::Params::write", vec![(pred!(const, ["unnamed"], ["cv::FileStorage*"]), _)]),
	void cv_TrackerMedianFlow_Params_write_const_FileStorageR(const cv::TrackerMedianFlow::Params* instance, cv::FileStorage* unnamed, ResultVoid* ocvrs_return) {
		try {
			instance->write(*unnamed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TrackerMedianFlow::Params::pointsInGrid() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1149
	// ("cv::TrackerMedianFlow::Params::pointsInGrid", vec![(pred!(const, [], []), _)]),
	int cv_TrackerMedianFlow_Params_propPointsInGrid_const(const cv::TrackerMedianFlow::Params* instance) {
			int ret = instance->pointsInGrid;
			return ret;
	}

	// cv::TrackerMedianFlow::Params::setPointsInGrid(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1149
	// ("cv::TrackerMedianFlow::Params::setPointsInGrid", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_TrackerMedianFlow_Params_propPointsInGrid_const_int(cv::TrackerMedianFlow::Params* instance, const int val) {
			instance->pointsInGrid = val;
	}

	// cv::TrackerMedianFlow::Params::winSize() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1151
	// ("cv::TrackerMedianFlow::Params::winSize", vec![(pred!(const, [], []), _)]),
	void cv_TrackerMedianFlow_Params_propWinSize_const(const cv::TrackerMedianFlow::Params* instance, cv::Size* ocvrs_return) {
			cv::Size ret = instance->winSize;
			*ocvrs_return = ret;
	}

	// cv::TrackerMedianFlow::Params::setWinSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1151
	// ("cv::TrackerMedianFlow::Params::setWinSize", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	void cv_TrackerMedianFlow_Params_propWinSize_const_Size(cv::TrackerMedianFlow::Params* instance, const cv::Size* val) {
			instance->winSize = *val;
	}

	// cv::TrackerMedianFlow::Params::maxLevel() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1152
	// ("cv::TrackerMedianFlow::Params::maxLevel", vec![(pred!(const, [], []), _)]),
	int cv_TrackerMedianFlow_Params_propMaxLevel_const(const cv::TrackerMedianFlow::Params* instance) {
			int ret = instance->maxLevel;
			return ret;
	}

	// cv::TrackerMedianFlow::Params::setMaxLevel(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1152
	// ("cv::TrackerMedianFlow::Params::setMaxLevel", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_TrackerMedianFlow_Params_propMaxLevel_const_int(cv::TrackerMedianFlow::Params* instance, const int val) {
			instance->maxLevel = val;
	}

	// cv::TrackerMedianFlow::Params::termCriteria() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1153
	// ("cv::TrackerMedianFlow::Params::termCriteria", vec![(pred!(const, [], []), _)]),
	void cv_TrackerMedianFlow_Params_propTermCriteria_const(const cv::TrackerMedianFlow::Params* instance, cv::TermCriteria* ocvrs_return) {
			cv::TermCriteria ret = instance->termCriteria;
			*ocvrs_return = ret;
	}

	// cv::TrackerMedianFlow::Params::setTermCriteria(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1153
	// ("cv::TrackerMedianFlow::Params::setTermCriteria", vec![(pred!(mut, ["val"], ["const cv::TermCriteria"]), _)]),
	void cv_TrackerMedianFlow_Params_propTermCriteria_const_TermCriteria(cv::TrackerMedianFlow::Params* instance, const cv::TermCriteria* val) {
			instance->termCriteria = *val;
	}

	// cv::TrackerMedianFlow::Params::winSizeNCC() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1154
	// ("cv::TrackerMedianFlow::Params::winSizeNCC", vec![(pred!(const, [], []), _)]),
	void cv_TrackerMedianFlow_Params_propWinSizeNCC_const(const cv::TrackerMedianFlow::Params* instance, cv::Size* ocvrs_return) {
			cv::Size ret = instance->winSizeNCC;
			*ocvrs_return = ret;
	}

	// cv::TrackerMedianFlow::Params::setWinSizeNCC(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1154
	// ("cv::TrackerMedianFlow::Params::setWinSizeNCC", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	void cv_TrackerMedianFlow_Params_propWinSizeNCC_const_Size(cv::TrackerMedianFlow::Params* instance, const cv::Size* val) {
			instance->winSizeNCC = *val;
	}

	// cv::TrackerMedianFlow::Params::maxMedianLengthOfDisplacementDifference() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1155
	// ("cv::TrackerMedianFlow::Params::maxMedianLengthOfDisplacementDifference", vec![(pred!(const, [], []), _)]),
	double cv_TrackerMedianFlow_Params_propMaxMedianLengthOfDisplacementDifference_const(const cv::TrackerMedianFlow::Params* instance) {
			double ret = instance->maxMedianLengthOfDisplacementDifference;
			return ret;
	}

	// cv::TrackerMedianFlow::Params::setMaxMedianLengthOfDisplacementDifference(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1155
	// ("cv::TrackerMedianFlow::Params::setMaxMedianLengthOfDisplacementDifference", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_TrackerMedianFlow_Params_propMaxMedianLengthOfDisplacementDifference_const_double(cv::TrackerMedianFlow::Params* instance, const double val) {
			instance->maxMedianLengthOfDisplacementDifference = val;
	}

	// cv::TrackerMedianFlow::Params::delete() generated
	// ("cv::TrackerMedianFlow::Params::delete", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerMedianFlow_Params_delete(cv::TrackerMedianFlow::Params* instance) {
			delete instance;
	}

	// setTrackerStateEstimator(Ptr<TrackerStateEstimator>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:464
	// ("cv::TrackerModel::setTrackerStateEstimator", vec![(pred!(mut, ["trackerStateEstimator"], ["cv::Ptr<cv::TrackerStateEstimator>"]), _)]),
	void cv_TrackerModel_setTrackerStateEstimator_PtrLTrackerStateEstimatorG(cv::TrackerModel* instance, cv::Ptr<cv::TrackerStateEstimator>* trackerStateEstimator, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->setTrackerStateEstimator(*trackerStateEstimator);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// modelEstimation(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:471
	// ("cv::TrackerModel::modelEstimation", vec![(pred!(mut, ["responses"], ["const std::vector<cv::Mat>*"]), _)]),
	void cv_TrackerModel_modelEstimation_const_vectorLMatGR(cv::TrackerModel* instance, const std::vector<cv::Mat>* responses, ResultVoid* ocvrs_return) {
		try {
			instance->modelEstimation(*responses);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// modelUpdate()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:477
	// ("cv::TrackerModel::modelUpdate", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerModel_modelUpdate(cv::TrackerModel* instance, ResultVoid* ocvrs_return) {
		try {
			instance->modelUpdate();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// runStateEstimator()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:481
	// ("cv::TrackerModel::runStateEstimator", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerModel_runStateEstimator(cv::TrackerModel* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->runStateEstimator();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setLastTargetState(const Ptr<TrackerTargetState> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:486
	// ("cv::TrackerModel::setLastTargetState", vec![(pred!(mut, ["lastTargetState"], ["const cv::Ptr<cv::TrackerTargetState>*"]), _)]),
	void cv_TrackerModel_setLastTargetState_const_PtrLTrackerTargetStateGR(cv::TrackerModel* instance, const cv::Ptr<cv::TrackerTargetState>* lastTargetState, ResultVoid* ocvrs_return) {
		try {
			instance->setLastTargetState(*lastTargetState);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLastTargetState()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:490
	// ("cv::TrackerModel::getLastTargetState", vec![(pred!(const, [], []), _)]),
	void cv_TrackerModel_getLastTargetState_const(const cv::TrackerModel* instance, Result<cv::Ptr<cv::TrackerTargetState>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::TrackerTargetState> ret = instance->getLastTargetState();
			Ok(new cv::Ptr<cv::TrackerTargetState>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getConfidenceMaps()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:494
	// ("cv::TrackerModel::getConfidenceMaps", vec![(pred!(const, [], []), _)]),
	void cv_TrackerModel_getConfidenceMaps_const(const cv::TrackerModel* instance, Result<std::vector<cv::ConfidenceMap>*>* ocvrs_return) {
		try {
			const std::vector<cv::ConfidenceMap> ret = instance->getConfidenceMaps();
			Ok(new const std::vector<cv::ConfidenceMap>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLastConfidenceMap()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:498
	// ("cv::TrackerModel::getLastConfidenceMap", vec![(pred!(const, [], []), _)]),
	void cv_TrackerModel_getLastConfidenceMap_const(const cv::TrackerModel* instance, Result<cv::ConfidenceMap*>* ocvrs_return) {
		try {
			const cv::ConfidenceMap ret = instance->getLastConfidenceMap();
			Ok(new const cv::ConfidenceMap(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTrackerStateEstimator()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:502
	// ("cv::TrackerModel::getTrackerStateEstimator", vec![(pred!(const, [], []), _)]),
	void cv_TrackerModel_getTrackerStateEstimator_const(const cv::TrackerModel* instance, Result<cv::Ptr<cv::TrackerStateEstimator>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::TrackerStateEstimator> ret = instance->getTrackerStateEstimator();
			Ok(new cv::Ptr<cv::TrackerStateEstimator>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TrackerModel::delete() generated
	// ("cv::TrackerModel::delete", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerModel_delete(cv::TrackerModel* instance) {
			delete instance;
	}

	// TrackerSampler()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:265
	// ("cv::TrackerSampler::TrackerSampler", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerSampler_TrackerSampler(Result<cv::TrackerSampler*>* ocvrs_return) {
		try {
			cv::TrackerSampler* ret = new cv::TrackerSampler();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// sampling(const Mat &, Rect)(TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:276
	// ("cv::TrackerSampler::sampling", vec![(pred!(mut, ["image", "boundingBox"], ["const cv::Mat*", "cv::Rect"]), _)]),
	void cv_TrackerSampler_sampling_const_MatR_Rect(cv::TrackerSampler* instance, const cv::Mat* image, cv::Rect* boundingBox, ResultVoid* ocvrs_return) {
		try {
			instance->sampling(*image, *boundingBox);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSamplers()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:280
	// ("cv::TrackerSampler::getSamplers", vec![(pred!(const, [], []), _)]),
	void cv_TrackerSampler_getSamplers_const(const cv::TrackerSampler* instance, Result<std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>>*>* ocvrs_return) {
		try {
			const std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>> ret = instance->getSamplers();
			Ok(new const std::vector<std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSamples()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:284
	// ("cv::TrackerSampler::getSamples", vec![(pred!(const, [], []), _)]),
	void cv_TrackerSampler_getSamples_const(const cv::TrackerSampler* instance, Result<std::vector<cv::Mat>*>* ocvrs_return) {
		try {
			const std::vector<cv::Mat> ret = instance->getSamples();
			Ok(new const std::vector<cv::Mat>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// addTrackerSamplerAlgorithm(String)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:307
	// ("cv::TrackerSampler::addTrackerSamplerAlgorithm", vec![(pred!(mut, ["trackerSamplerAlgorithmType"], ["cv::String"]), _)]),
	void cv_TrackerSampler_addTrackerSamplerAlgorithm_String(cv::TrackerSampler* instance, const char* trackerSamplerAlgorithmType, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->addTrackerSamplerAlgorithm(cv::String(trackerSamplerAlgorithmType));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// addTrackerSamplerAlgorithm(Ptr<TrackerSamplerAlgorithm> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:312
	// ("cv::TrackerSampler::addTrackerSamplerAlgorithm", vec![(pred!(mut, ["sampler"], ["cv::Ptr<cv::TrackerSamplerAlgorithm>*"]), _)]),
	void cv_TrackerSampler_addTrackerSamplerAlgorithm_PtrLTrackerSamplerAlgorithmGR(cv::TrackerSampler* instance, cv::Ptr<cv::TrackerSamplerAlgorithm>* sampler, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->addTrackerSamplerAlgorithm(*sampler);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TrackerSampler::delete() generated
	// ("cv::TrackerSampler::delete", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerSampler_delete(cv::TrackerSampler* instance) {
			delete instance;
	}

	// create(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:222
	// ("cv::TrackerSamplerAlgorithm::create", vec![(pred!(mut, ["trackerSamplerType"], ["const cv::String*"]), _)]),
	void cv_TrackerSamplerAlgorithm_create_const_StringR(const char* trackerSamplerType, Result<cv::Ptr<cv::TrackerSamplerAlgorithm>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::TrackerSamplerAlgorithm> ret = cv::TrackerSamplerAlgorithm::create(cv::String(trackerSamplerType));
			Ok(new cv::Ptr<cv::TrackerSamplerAlgorithm>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// sampling(const Mat &, Rect, std::vector<Mat> &)(TraitClass, SimpleClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:233
	// ("cv::TrackerSamplerAlgorithm::sampling", vec![(pred!(mut, ["image", "boundingBox", "sample"], ["const cv::Mat*", "cv::Rect", "std::vector<cv::Mat>*"]), _)]),
	void cv_TrackerSamplerAlgorithm_sampling_const_MatR_Rect_vectorLMatGR(cv::TrackerSamplerAlgorithm* instance, const cv::Mat* image, cv::Rect* boundingBox, std::vector<cv::Mat>* sample, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->sampling(*image, *boundingBox, *sample);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getClassName()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:237
	// ("cv::TrackerSamplerAlgorithm::getClassName", vec![(pred!(const, [], []), _)]),
	void cv_TrackerSamplerAlgorithm_getClassName_const(const cv::TrackerSamplerAlgorithm* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getClassName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TrackerSamplerAlgorithm::to_TrackerSamplerCS() generated
	// ("cv::TrackerSamplerAlgorithm::to_TrackerSamplerCS", vec![(pred!(mut, [], []), _)]),
	cv::TrackerSamplerCS* cv_TrackerSamplerAlgorithm_to_TrackerSamplerCS(cv::TrackerSamplerAlgorithm* instance) {
			return dynamic_cast<cv::TrackerSamplerCS*>(instance);
	}

	// cv::TrackerSamplerAlgorithm::to_TrackerSamplerCSC() generated
	// ("cv::TrackerSamplerAlgorithm::to_TrackerSamplerCSC", vec![(pred!(mut, [], []), _)]),
	cv::TrackerSamplerCSC* cv_TrackerSamplerAlgorithm_to_TrackerSamplerCSC(cv::TrackerSamplerAlgorithm* instance) {
			return dynamic_cast<cv::TrackerSamplerCSC*>(instance);
	}

	// cv::TrackerSamplerAlgorithm::to_TrackerSamplerPF() generated
	// ("cv::TrackerSamplerAlgorithm::to_TrackerSamplerPF", vec![(pred!(mut, [], []), _)]),
	cv::TrackerSamplerPF* cv_TrackerSamplerAlgorithm_to_TrackerSamplerPF(cv::TrackerSamplerAlgorithm* instance) {
			return dynamic_cast<cv::TrackerSamplerPF*>(instance);
	}

	// cv::TrackerSamplerAlgorithm::delete() generated
	// ("cv::TrackerSamplerAlgorithm::delete", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerSamplerAlgorithm_delete(cv::TrackerSamplerAlgorithm* instance) {
			delete instance;
	}

	// TrackerSamplerCS(const TrackerSamplerCS::Params &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:848
	// ("cv::TrackerSamplerCS::TrackerSamplerCS", vec![(pred!(mut, ["parameters"], ["const cv::TrackerSamplerCS::Params*"]), _)]),
	void cv_TrackerSamplerCS_TrackerSamplerCS_const_ParamsR(const cv::TrackerSamplerCS::Params* parameters, Result<cv::TrackerSamplerCS*>* ocvrs_return) {
		try {
			cv::TrackerSamplerCS* ret = new cv::TrackerSamplerCS(*parameters);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TrackerSamplerCS::TrackerSamplerCS() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:848
	// ("cv::TrackerSamplerCS::TrackerSamplerCS", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerSamplerCS_TrackerSamplerCS(Result<cv::TrackerSamplerCS*>* ocvrs_return) {
		try {
			cv::TrackerSamplerCS* ret = new cv::TrackerSamplerCS();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMode(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:859
	// ("cv::TrackerSamplerCS::setMode", vec![(pred!(mut, ["samplingMode"], ["int"]), _)]),
	void cv_TrackerSamplerCS_setMode_int(cv::TrackerSamplerCS* instance, int samplingMode, ResultVoid* ocvrs_return) {
		try {
			instance->setMode(samplingMode);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// samplingImpl(const Mat &, Rect, std::vector<Mat> &)(TraitClass, SimpleClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:863
	// ("cv::TrackerSamplerCS::samplingImpl", vec![(pred!(mut, ["image", "boundingBox", "sample"], ["const cv::Mat*", "cv::Rect", "std::vector<cv::Mat>*"]), _)]),
	void cv_TrackerSamplerCS_samplingImpl_const_MatR_Rect_vectorLMatGR(cv::TrackerSamplerCS* instance, const cv::Mat* image, cv::Rect* boundingBox, std::vector<cv::Mat>* sample, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->samplingImpl(*image, *boundingBox, *sample);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getROI()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:864
	// ("cv::TrackerSamplerCS::getROI", vec![(pred!(const, [], []), _)]),
	void cv_TrackerSamplerCS_getROI_const(const cv::TrackerSamplerCS* instance, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = instance->getROI();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TrackerSamplerCS::to_TrackerSamplerAlgorithm() generated
	// ("cv::TrackerSamplerCS::to_TrackerSamplerAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::TrackerSamplerAlgorithm* cv_TrackerSamplerCS_to_TrackerSamplerAlgorithm(cv::TrackerSamplerCS* instance) {
			return dynamic_cast<cv::TrackerSamplerAlgorithm*>(instance);
	}

	// cv::TrackerSamplerCS::delete() generated
	// ("cv::TrackerSamplerCS::delete", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerSamplerCS_delete(cv::TrackerSamplerCS* instance) {
			delete instance;
	}

	// Params()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:841
	// ("cv::TrackerSamplerCS::Params::Params", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerSamplerCS_Params_Params(Result<cv::TrackerSamplerCS::Params*>* ocvrs_return) {
		try {
			cv::TrackerSamplerCS::Params* ret = new cv::TrackerSamplerCS::Params();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TrackerSamplerCS::Params::overlap() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:842
	// ("cv::TrackerSamplerCS::Params::overlap", vec![(pred!(const, [], []), _)]),
	float cv_TrackerSamplerCS_Params_propOverlap_const(const cv::TrackerSamplerCS::Params* instance) {
			float ret = instance->overlap;
			return ret;
	}

	// cv::TrackerSamplerCS::Params::setOverlap(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:842
	// ("cv::TrackerSamplerCS::Params::setOverlap", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_TrackerSamplerCS_Params_propOverlap_const_float(cv::TrackerSamplerCS::Params* instance, const float val) {
			instance->overlap = val;
	}

	// cv::TrackerSamplerCS::Params::searchFactor() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:843
	// ("cv::TrackerSamplerCS::Params::searchFactor", vec![(pred!(const, [], []), _)]),
	float cv_TrackerSamplerCS_Params_propSearchFactor_const(const cv::TrackerSamplerCS::Params* instance) {
			float ret = instance->searchFactor;
			return ret;
	}

	// cv::TrackerSamplerCS::Params::setSearchFactor(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:843
	// ("cv::TrackerSamplerCS::Params::setSearchFactor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_TrackerSamplerCS_Params_propSearchFactor_const_float(cv::TrackerSamplerCS::Params* instance, const float val) {
			instance->searchFactor = val;
	}

	// cv::TrackerSamplerCS::Params::delete() generated
	// ("cv::TrackerSamplerCS::Params::delete", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerSamplerCS_Params_delete(cv::TrackerSamplerCS::Params* instance) {
			delete instance;
	}

	// TrackerSamplerCSC(const TrackerSamplerCSC::Params &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:797
	// ("cv::TrackerSamplerCSC::TrackerSamplerCSC", vec![(pred!(mut, ["parameters"], ["const cv::TrackerSamplerCSC::Params*"]), _)]),
	void cv_TrackerSamplerCSC_TrackerSamplerCSC_const_ParamsR(const cv::TrackerSamplerCSC::Params* parameters, Result<cv::TrackerSamplerCSC*>* ocvrs_return) {
		try {
			cv::TrackerSamplerCSC* ret = new cv::TrackerSamplerCSC(*parameters);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TrackerSamplerCSC::TrackerSamplerCSC() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:797
	// ("cv::TrackerSamplerCSC::TrackerSamplerCSC", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerSamplerCSC_TrackerSamplerCSC(Result<cv::TrackerSamplerCSC*>* ocvrs_return) {
		try {
			cv::TrackerSamplerCSC* ret = new cv::TrackerSamplerCSC();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMode(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:810
	// ("cv::TrackerSamplerCSC::setMode", vec![(pred!(mut, ["samplingMode"], ["int"]), _)]),
	void cv_TrackerSamplerCSC_setMode_int(cv::TrackerSamplerCSC* instance, int samplingMode, ResultVoid* ocvrs_return) {
		try {
			instance->setMode(samplingMode);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TrackerSamplerCSC::to_TrackerSamplerAlgorithm() generated
	// ("cv::TrackerSamplerCSC::to_TrackerSamplerAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::TrackerSamplerAlgorithm* cv_TrackerSamplerCSC_to_TrackerSamplerAlgorithm(cv::TrackerSamplerCSC* instance) {
			return dynamic_cast<cv::TrackerSamplerAlgorithm*>(instance);
	}

	// cv::TrackerSamplerCSC::delete() generated
	// ("cv::TrackerSamplerCSC::delete", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerSamplerCSC_delete(cv::TrackerSamplerCSC* instance) {
			delete instance;
	}

	// Params()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:785
	// ("cv::TrackerSamplerCSC::Params::Params", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerSamplerCSC_Params_Params(Result<cv::TrackerSamplerCSC::Params*>* ocvrs_return) {
		try {
			cv::TrackerSamplerCSC::Params* ret = new cv::TrackerSamplerCSC::Params();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TrackerSamplerCSC::Params::initInRad() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:786
	// ("cv::TrackerSamplerCSC::Params::initInRad", vec![(pred!(const, [], []), _)]),
	float cv_TrackerSamplerCSC_Params_propInitInRad_const(const cv::TrackerSamplerCSC::Params* instance) {
			float ret = instance->initInRad;
			return ret;
	}

	// cv::TrackerSamplerCSC::Params::setInitInRad(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:786
	// ("cv::TrackerSamplerCSC::Params::setInitInRad", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_TrackerSamplerCSC_Params_propInitInRad_const_float(cv::TrackerSamplerCSC::Params* instance, const float val) {
			instance->initInRad = val;
	}

	// cv::TrackerSamplerCSC::Params::trackInPosRad() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:787
	// ("cv::TrackerSamplerCSC::Params::trackInPosRad", vec![(pred!(const, [], []), _)]),
	float cv_TrackerSamplerCSC_Params_propTrackInPosRad_const(const cv::TrackerSamplerCSC::Params* instance) {
			float ret = instance->trackInPosRad;
			return ret;
	}

	// cv::TrackerSamplerCSC::Params::setTrackInPosRad(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:787
	// ("cv::TrackerSamplerCSC::Params::setTrackInPosRad", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_TrackerSamplerCSC_Params_propTrackInPosRad_const_float(cv::TrackerSamplerCSC::Params* instance, const float val) {
			instance->trackInPosRad = val;
	}

	// cv::TrackerSamplerCSC::Params::searchWinSize() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:788
	// ("cv::TrackerSamplerCSC::Params::searchWinSize", vec![(pred!(const, [], []), _)]),
	float cv_TrackerSamplerCSC_Params_propSearchWinSize_const(const cv::TrackerSamplerCSC::Params* instance) {
			float ret = instance->searchWinSize;
			return ret;
	}

	// cv::TrackerSamplerCSC::Params::setSearchWinSize(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:788
	// ("cv::TrackerSamplerCSC::Params::setSearchWinSize", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_TrackerSamplerCSC_Params_propSearchWinSize_const_float(cv::TrackerSamplerCSC::Params* instance, const float val) {
			instance->searchWinSize = val;
	}

	// cv::TrackerSamplerCSC::Params::initMaxNegNum() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:789
	// ("cv::TrackerSamplerCSC::Params::initMaxNegNum", vec![(pred!(const, [], []), _)]),
	int cv_TrackerSamplerCSC_Params_propInitMaxNegNum_const(const cv::TrackerSamplerCSC::Params* instance) {
			int ret = instance->initMaxNegNum;
			return ret;
	}

	// cv::TrackerSamplerCSC::Params::setInitMaxNegNum(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:789
	// ("cv::TrackerSamplerCSC::Params::setInitMaxNegNum", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_TrackerSamplerCSC_Params_propInitMaxNegNum_const_int(cv::TrackerSamplerCSC::Params* instance, const int val) {
			instance->initMaxNegNum = val;
	}

	// cv::TrackerSamplerCSC::Params::trackMaxPosNum() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:790
	// ("cv::TrackerSamplerCSC::Params::trackMaxPosNum", vec![(pred!(const, [], []), _)]),
	int cv_TrackerSamplerCSC_Params_propTrackMaxPosNum_const(const cv::TrackerSamplerCSC::Params* instance) {
			int ret = instance->trackMaxPosNum;
			return ret;
	}

	// cv::TrackerSamplerCSC::Params::setTrackMaxPosNum(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:790
	// ("cv::TrackerSamplerCSC::Params::setTrackMaxPosNum", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_TrackerSamplerCSC_Params_propTrackMaxPosNum_const_int(cv::TrackerSamplerCSC::Params* instance, const int val) {
			instance->trackMaxPosNum = val;
	}

	// cv::TrackerSamplerCSC::Params::trackMaxNegNum() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:791
	// ("cv::TrackerSamplerCSC::Params::trackMaxNegNum", vec![(pred!(const, [], []), _)]),
	int cv_TrackerSamplerCSC_Params_propTrackMaxNegNum_const(const cv::TrackerSamplerCSC::Params* instance) {
			int ret = instance->trackMaxNegNum;
			return ret;
	}

	// cv::TrackerSamplerCSC::Params::setTrackMaxNegNum(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:791
	// ("cv::TrackerSamplerCSC::Params::setTrackMaxNegNum", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_TrackerSamplerCSC_Params_propTrackMaxNegNum_const_int(cv::TrackerSamplerCSC::Params* instance, const int val) {
			instance->trackMaxNegNum = val;
	}

	// cv::TrackerSamplerCSC::Params::delete() generated
	// ("cv::TrackerSamplerCSC::Params::delete", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerSamplerCSC_Params_delete(cv::TrackerSamplerCSC::Params* instance) {
			delete instance;
	}

	// TrackerSamplerPF(const Mat &, const TrackerSamplerPF::Params &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:917
	// ("cv::TrackerSamplerPF::TrackerSamplerPF", vec![(pred!(mut, ["chosenRect", "parameters"], ["const cv::Mat*", "const cv::TrackerSamplerPF::Params*"]), _)]),
	void cv_TrackerSamplerPF_TrackerSamplerPF_const_MatR_const_ParamsR(const cv::Mat* chosenRect, const cv::TrackerSamplerPF::Params* parameters, Result<cv::TrackerSamplerPF*>* ocvrs_return) {
		try {
			cv::TrackerSamplerPF* ret = new cv::TrackerSamplerPF(*chosenRect, *parameters);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TrackerSamplerPF::TrackerSamplerPF(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:917
	// ("cv::TrackerSamplerPF::TrackerSamplerPF", vec![(pred!(mut, ["chosenRect"], ["const cv::Mat*"]), _)]),
	void cv_TrackerSamplerPF_TrackerSamplerPF_const_MatR(const cv::Mat* chosenRect, Result<cv::TrackerSamplerPF*>* ocvrs_return) {
		try {
			cv::TrackerSamplerPF* ret = new cv::TrackerSamplerPF(*chosenRect);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TrackerSamplerPF::to_TrackerSamplerAlgorithm() generated
	// ("cv::TrackerSamplerPF::to_TrackerSamplerAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::TrackerSamplerAlgorithm* cv_TrackerSamplerPF_to_TrackerSamplerAlgorithm(cv::TrackerSamplerPF* instance) {
			return dynamic_cast<cv::TrackerSamplerAlgorithm*>(instance);
	}

	// cv::TrackerSamplerPF::delete() generated
	// ("cv::TrackerSamplerPF::delete", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerSamplerPF_delete(cv::TrackerSamplerPF* instance) {
			delete instance;
	}

	// Params()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:905
	// ("cv::TrackerSamplerPF::Params::Params", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerSamplerPF_Params_Params(Result<cv::TrackerSamplerPF::Params*>* ocvrs_return) {
		try {
			cv::TrackerSamplerPF::Params* ret = new cv::TrackerSamplerPF::Params();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TrackerSamplerPF::Params::iterationNum() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:906
	// ("cv::TrackerSamplerPF::Params::iterationNum", vec![(pred!(const, [], []), _)]),
	int cv_TrackerSamplerPF_Params_propIterationNum_const(const cv::TrackerSamplerPF::Params* instance) {
			int ret = instance->iterationNum;
			return ret;
	}

	// cv::TrackerSamplerPF::Params::setIterationNum(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:906
	// ("cv::TrackerSamplerPF::Params::setIterationNum", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_TrackerSamplerPF_Params_propIterationNum_const_int(cv::TrackerSamplerPF::Params* instance, const int val) {
			instance->iterationNum = val;
	}

	// cv::TrackerSamplerPF::Params::particlesNum() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:907
	// ("cv::TrackerSamplerPF::Params::particlesNum", vec![(pred!(const, [], []), _)]),
	int cv_TrackerSamplerPF_Params_propParticlesNum_const(const cv::TrackerSamplerPF::Params* instance) {
			int ret = instance->particlesNum;
			return ret;
	}

	// cv::TrackerSamplerPF::Params::setParticlesNum(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:907
	// ("cv::TrackerSamplerPF::Params::setParticlesNum", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_TrackerSamplerPF_Params_propParticlesNum_const_int(cv::TrackerSamplerPF::Params* instance, const int val) {
			instance->particlesNum = val;
	}

	// cv::TrackerSamplerPF::Params::alpha() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:908
	// ("cv::TrackerSamplerPF::Params::alpha", vec![(pred!(const, [], []), _)]),
	double cv_TrackerSamplerPF_Params_propAlpha_const(const cv::TrackerSamplerPF::Params* instance) {
			double ret = instance->alpha;
			return ret;
	}

	// cv::TrackerSamplerPF::Params::setAlpha(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:908
	// ("cv::TrackerSamplerPF::Params::setAlpha", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_TrackerSamplerPF_Params_propAlpha_const_double(cv::TrackerSamplerPF::Params* instance, const double val) {
			instance->alpha = val;
	}

	// cv::TrackerSamplerPF::Params::std() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:910
	// ("cv::TrackerSamplerPF::Params::std", vec![(pred!(const, [], []), _)]),
	cv::Mat_<double>* cv_TrackerSamplerPF_Params_propStd_const(const cv::TrackerSamplerPF::Params* instance) {
			cv::Mat_<double> ret = instance->std;
			return new cv::Mat_<double>(ret);
	}

	// cv::TrackerSamplerPF::Params::setStd(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:910
	// ("cv::TrackerSamplerPF::Params::setStd", vec![(pred!(mut, ["val"], ["const cv::Mat_<double>"]), _)]),
	void cv_TrackerSamplerPF_Params_propStd_const_Mat_LdoubleG(cv::TrackerSamplerPF::Params* instance, const cv::Mat_<double>* val) {
			instance->std = *val;
	}

	// cv::TrackerSamplerPF::Params::delete() generated
	// ("cv::TrackerSamplerPF::Params::delete", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerSamplerPF_Params_delete(cv::TrackerSamplerPF::Params* instance) {
			delete instance;
	}

	// estimate(const std::vector<ConfidenceMap> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:408
	// ("cv::TrackerStateEstimator::estimate", vec![(pred!(mut, ["confidenceMaps"], ["const std::vector<cv::ConfidenceMap>*"]), _)]),
	void cv_TrackerStateEstimator_estimate_const_vectorLConfidenceMapGR(cv::TrackerStateEstimator* instance, const std::vector<cv::ConfidenceMap>* confidenceMaps, Result<cv::Ptr<cv::TrackerTargetState>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::TrackerTargetState> ret = instance->estimate(*confidenceMaps);
			Ok(new cv::Ptr<cv::TrackerTargetState>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// update(std::vector<ConfidenceMap> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:413
	// ("cv::TrackerStateEstimator::update", vec![(pred!(mut, ["confidenceMaps"], ["std::vector<cv::ConfidenceMap>*"]), _)]),
	void cv_TrackerStateEstimator_update_vectorLConfidenceMapGR(cv::TrackerStateEstimator* instance, std::vector<cv::ConfidenceMap>* confidenceMaps, ResultVoid* ocvrs_return) {
		try {
			instance->update(*confidenceMaps);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:426
	// ("cv::TrackerStateEstimator::create", vec![(pred!(mut, ["trackeStateEstimatorType"], ["const cv::String*"]), _)]),
	void cv_TrackerStateEstimator_create_const_StringR(const char* trackeStateEstimatorType, Result<cv::Ptr<cv::TrackerStateEstimator>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::TrackerStateEstimator> ret = cv::TrackerStateEstimator::create(cv::String(trackeStateEstimatorType));
			Ok(new cv::Ptr<cv::TrackerStateEstimator>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getClassName()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:430
	// ("cv::TrackerStateEstimator::getClassName", vec![(pred!(const, [], []), _)]),
	void cv_TrackerStateEstimator_getClassName_const(const cv::TrackerStateEstimator* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getClassName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TrackerStateEstimator::to_TrackerStateEstimatorAdaBoosting() generated
	// ("cv::TrackerStateEstimator::to_TrackerStateEstimatorAdaBoosting", vec![(pred!(mut, [], []), _)]),
	cv::TrackerStateEstimatorAdaBoosting* cv_TrackerStateEstimator_to_TrackerStateEstimatorAdaBoosting(cv::TrackerStateEstimator* instance) {
			return dynamic_cast<cv::TrackerStateEstimatorAdaBoosting*>(instance);
	}

	// cv::TrackerStateEstimator::to_TrackerStateEstimatorMILBoosting() generated
	// ("cv::TrackerStateEstimator::to_TrackerStateEstimatorMILBoosting", vec![(pred!(mut, [], []), _)]),
	cv::TrackerStateEstimatorMILBoosting* cv_TrackerStateEstimator_to_TrackerStateEstimatorMILBoosting(cv::TrackerStateEstimator* instance) {
			return dynamic_cast<cv::TrackerStateEstimatorMILBoosting*>(instance);
	}

	// cv::TrackerStateEstimator::to_TrackerStateEstimatorSVM() generated
	// ("cv::TrackerStateEstimator::to_TrackerStateEstimatorSVM", vec![(pred!(mut, [], []), _)]),
	cv::TrackerStateEstimatorSVM* cv_TrackerStateEstimator_to_TrackerStateEstimatorSVM(cv::TrackerStateEstimator* instance) {
			return dynamic_cast<cv::TrackerStateEstimatorSVM*>(instance);
	}

	// cv::TrackerStateEstimator::delete() generated
	// ("cv::TrackerStateEstimator::delete", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerStateEstimator_delete(cv::TrackerStateEstimator* instance) {
			delete instance;
	}

	// TrackerStateEstimatorAdaBoosting(int, int, int, Size, const Rect &)(Primitive, Primitive, Primitive, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:701
	// ("cv::TrackerStateEstimatorAdaBoosting::TrackerStateEstimatorAdaBoosting", vec![(pred!(mut, ["numClassifer", "initIterations", "nFeatures", "patchSize", "ROI"], ["int", "int", "int", "cv::Size", "const cv::Rect*"]), _)]),
	void cv_TrackerStateEstimatorAdaBoosting_TrackerStateEstimatorAdaBoosting_int_int_int_Size_const_RectR(int numClassifer, int initIterations, int nFeatures, cv::Size* patchSize, const cv::Rect* ROI, Result<cv::TrackerStateEstimatorAdaBoosting*>* ocvrs_return) {
		try {
			cv::TrackerStateEstimatorAdaBoosting* ret = new cv::TrackerStateEstimatorAdaBoosting(numClassifer, initIterations, nFeatures, *patchSize, *ROI);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSampleROI()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:710
	// ("cv::TrackerStateEstimatorAdaBoosting::getSampleROI", vec![(pred!(const, [], []), _)]),
	void cv_TrackerStateEstimatorAdaBoosting_getSampleROI_const(const cv::TrackerStateEstimatorAdaBoosting* instance, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = instance->getSampleROI();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSampleROI(const Rect &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:715
	// ("cv::TrackerStateEstimatorAdaBoosting::setSampleROI", vec![(pred!(mut, ["ROI"], ["const cv::Rect*"]), _)]),
	void cv_TrackerStateEstimatorAdaBoosting_setSampleROI_const_RectR(cv::TrackerStateEstimatorAdaBoosting* instance, const cv::Rect* ROI, ResultVoid* ocvrs_return) {
		try {
			instance->setSampleROI(*ROI);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setCurrentConfidenceMap(ConfidenceMap &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:720
	// ("cv::TrackerStateEstimatorAdaBoosting::setCurrentConfidenceMap", vec![(pred!(mut, ["confidenceMap"], ["cv::ConfidenceMap*"]), _)]),
	void cv_TrackerStateEstimatorAdaBoosting_setCurrentConfidenceMap_ConfidenceMapR(cv::TrackerStateEstimatorAdaBoosting* instance, cv::ConfidenceMap* confidenceMap, ResultVoid* ocvrs_return) {
		try {
			instance->setCurrentConfidenceMap(*confidenceMap);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// computeSelectedWeakClassifier()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:724
	// ("cv::TrackerStateEstimatorAdaBoosting::computeSelectedWeakClassifier", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerStateEstimatorAdaBoosting_computeSelectedWeakClassifier(cv::TrackerStateEstimatorAdaBoosting* instance, Result<std::vector<int>*>* ocvrs_return) {
		try {
			std::vector<int> ret = instance->computeSelectedWeakClassifier();
			Ok(new std::vector<int>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// computeReplacedClassifier()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:728
	// ("cv::TrackerStateEstimatorAdaBoosting::computeReplacedClassifier", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerStateEstimatorAdaBoosting_computeReplacedClassifier(cv::TrackerStateEstimatorAdaBoosting* instance, Result<std::vector<int>*>* ocvrs_return) {
		try {
			std::vector<int> ret = instance->computeReplacedClassifier();
			Ok(new std::vector<int>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// computeSwappedClassifier()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:732
	// ("cv::TrackerStateEstimatorAdaBoosting::computeSwappedClassifier", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerStateEstimatorAdaBoosting_computeSwappedClassifier(cv::TrackerStateEstimatorAdaBoosting* instance, Result<std::vector<int>*>* ocvrs_return) {
		try {
			std::vector<int> ret = instance->computeSwappedClassifier();
			Ok(new std::vector<int>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TrackerStateEstimatorAdaBoosting::to_TrackerStateEstimator() generated
	// ("cv::TrackerStateEstimatorAdaBoosting::to_TrackerStateEstimator", vec![(pred!(mut, [], []), _)]),
	cv::TrackerStateEstimator* cv_TrackerStateEstimatorAdaBoosting_to_TrackerStateEstimator(cv::TrackerStateEstimatorAdaBoosting* instance) {
			return dynamic_cast<cv::TrackerStateEstimator*>(instance);
	}

	// cv::TrackerStateEstimatorAdaBoosting::delete() generated
	// ("cv::TrackerStateEstimatorAdaBoosting::delete", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerStateEstimatorAdaBoosting_delete(cv::TrackerStateEstimatorAdaBoosting* instance) {
			delete instance;
	}

	#if !defined(OCVRS_TARGET_OS_WINDOWS)
	// TrackerAdaBoostingTargetState(const Point2f &, int, int, bool, const Mat &)(SimpleClass, Primitive, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:663
	// ("cv::TrackerStateEstimatorAdaBoosting::TrackerAdaBoostingTargetState::TrackerAdaBoostingTargetState", vec![(pred!(mut, ["position", "width", "height", "foreground", "responses"], ["const cv::Point2f*", "int", "int", "bool", "const cv::Mat*"]), _)]),
	void cv_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState_TrackerAdaBoostingTargetState_const_Point2fR_int_int_bool_const_MatR(const cv::Point2f* position, int width, int height, bool foreground, const cv::Mat* responses, Result<cv::TrackerStateEstimatorAdaBoosting::TrackerAdaBoostingTargetState*>* ocvrs_return) {
		try {
			cv::TrackerStateEstimatorAdaBoosting::TrackerAdaBoostingTargetState* ret = new cv::TrackerStateEstimatorAdaBoosting::TrackerAdaBoostingTargetState(*position, width, height, foreground, *responses);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	#endif

	#if !defined(OCVRS_TARGET_OS_WINDOWS)
	// setTargetResponses(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:676
	// ("cv::TrackerStateEstimatorAdaBoosting::TrackerAdaBoostingTargetState::setTargetResponses", vec![(pred!(mut, ["responses"], ["const cv::Mat*"]), _)]),
	void cv_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState_setTargetResponses_const_MatR(cv::TrackerStateEstimatorAdaBoosting::TrackerAdaBoostingTargetState* instance, const cv::Mat* responses, ResultVoid* ocvrs_return) {
		try {
			instance->setTargetResponses(*responses);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	#endif

	#if !defined(OCVRS_TARGET_OS_WINDOWS)
	// setTargetFg(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:680
	// ("cv::TrackerStateEstimatorAdaBoosting::TrackerAdaBoostingTargetState::setTargetFg", vec![(pred!(mut, ["foreground"], ["bool"]), _)]),
	void cv_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState_setTargetFg_bool(cv::TrackerStateEstimatorAdaBoosting::TrackerAdaBoostingTargetState* instance, bool foreground, ResultVoid* ocvrs_return) {
		try {
			instance->setTargetFg(foreground);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	#endif

	#if !defined(OCVRS_TARGET_OS_WINDOWS)
	// getTargetResponses()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:683
	// ("cv::TrackerStateEstimatorAdaBoosting::TrackerAdaBoostingTargetState::getTargetResponses", vec![(pred!(const, [], []), _)]),
	void cv_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState_getTargetResponses_const(const cv::TrackerStateEstimatorAdaBoosting::TrackerAdaBoostingTargetState* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getTargetResponses();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	#endif

	#if !defined(OCVRS_TARGET_OS_WINDOWS)
	// isTargetFg()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:686
	// ("cv::TrackerStateEstimatorAdaBoosting::TrackerAdaBoostingTargetState::isTargetFg", vec![(pred!(const, [], []), _)]),
	void cv_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState_isTargetFg_const(const cv::TrackerStateEstimatorAdaBoosting::TrackerAdaBoostingTargetState* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isTargetFg();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	#endif

	// cv::TrackerStateEstimatorAdaBoosting::TrackerAdaBoostingTargetState::to_TrackerTargetState() generated
	// ("cv::TrackerStateEstimatorAdaBoosting::TrackerAdaBoostingTargetState::to_TrackerTargetState", vec![(pred!(mut, [], []), _)]),
	cv::TrackerTargetState* cv_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState_to_TrackerTargetState(cv::TrackerStateEstimatorAdaBoosting::TrackerAdaBoostingTargetState* instance) {
			return dynamic_cast<cv::TrackerTargetState*>(instance);
	}

	// cv::TrackerStateEstimatorAdaBoosting::TrackerAdaBoostingTargetState::delete() generated
	// ("cv::TrackerStateEstimatorAdaBoosting::TrackerAdaBoostingTargetState::delete", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState_delete(cv::TrackerStateEstimatorAdaBoosting::TrackerAdaBoostingTargetState* instance) {
			delete instance;
	}

	// TrackerStateEstimatorMILBoosting(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:621
	// ("cv::TrackerStateEstimatorMILBoosting::TrackerStateEstimatorMILBoosting", vec![(pred!(mut, ["nFeatures"], ["int"]), _)]),
	void cv_TrackerStateEstimatorMILBoosting_TrackerStateEstimatorMILBoosting_int(int nFeatures, Result<cv::TrackerStateEstimatorMILBoosting*>* ocvrs_return) {
		try {
			cv::TrackerStateEstimatorMILBoosting* ret = new cv::TrackerStateEstimatorMILBoosting(nFeatures);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TrackerStateEstimatorMILBoosting::TrackerStateEstimatorMILBoosting() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:621
	// ("cv::TrackerStateEstimatorMILBoosting::TrackerStateEstimatorMILBoosting", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerStateEstimatorMILBoosting_TrackerStateEstimatorMILBoosting(Result<cv::TrackerStateEstimatorMILBoosting*>* ocvrs_return) {
		try {
			cv::TrackerStateEstimatorMILBoosting* ret = new cv::TrackerStateEstimatorMILBoosting();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setCurrentConfidenceMap(ConfidenceMap &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:627
	// ("cv::TrackerStateEstimatorMILBoosting::setCurrentConfidenceMap", vec![(pred!(mut, ["confidenceMap"], ["cv::ConfidenceMap*"]), _)]),
	void cv_TrackerStateEstimatorMILBoosting_setCurrentConfidenceMap_ConfidenceMapR(cv::TrackerStateEstimatorMILBoosting* instance, cv::ConfidenceMap* confidenceMap, ResultVoid* ocvrs_return) {
		try {
			instance->setCurrentConfidenceMap(*confidenceMap);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TrackerStateEstimatorMILBoosting::to_TrackerStateEstimator() generated
	// ("cv::TrackerStateEstimatorMILBoosting::to_TrackerStateEstimator", vec![(pred!(mut, [], []), _)]),
	cv::TrackerStateEstimator* cv_TrackerStateEstimatorMILBoosting_to_TrackerStateEstimator(cv::TrackerStateEstimatorMILBoosting* instance) {
			return dynamic_cast<cv::TrackerStateEstimator*>(instance);
	}

	// cv::TrackerStateEstimatorMILBoosting::delete() generated
	// ("cv::TrackerStateEstimatorMILBoosting::delete", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerStateEstimatorMILBoosting_delete(cv::TrackerStateEstimatorMILBoosting* instance) {
			delete instance;
	}

	#if !defined(OCVRS_TARGET_OS_WINDOWS)
	// TrackerMILTargetState(const Point2f &, int, int, bool, const Mat &)(SimpleClass, Primitive, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:588
	// ("cv::TrackerStateEstimatorMILBoosting::TrackerMILTargetState::TrackerMILTargetState", vec![(pred!(mut, ["position", "width", "height", "foreground", "features"], ["const cv::Point2f*", "int", "int", "bool", "const cv::Mat*"]), _)]),
	void cv_TrackerStateEstimatorMILBoosting_TrackerMILTargetState_TrackerMILTargetState_const_Point2fR_int_int_bool_const_MatR(const cv::Point2f* position, int width, int height, bool foreground, const cv::Mat* features, Result<cv::TrackerStateEstimatorMILBoosting::TrackerMILTargetState*>* ocvrs_return) {
		try {
			cv::TrackerStateEstimatorMILBoosting::TrackerMILTargetState* ret = new cv::TrackerStateEstimatorMILBoosting::TrackerMILTargetState(*position, width, height, foreground, *features);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	#endif

	#if !defined(OCVRS_TARGET_OS_WINDOWS)
	// setTargetFg(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:601
	// ("cv::TrackerStateEstimatorMILBoosting::TrackerMILTargetState::setTargetFg", vec![(pred!(mut, ["foreground"], ["bool"]), _)]),
	void cv_TrackerStateEstimatorMILBoosting_TrackerMILTargetState_setTargetFg_bool(cv::TrackerStateEstimatorMILBoosting::TrackerMILTargetState* instance, bool foreground, ResultVoid* ocvrs_return) {
		try {
			instance->setTargetFg(foreground);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	#endif

	#if !defined(OCVRS_TARGET_OS_WINDOWS)
	// setFeatures(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:605
	// ("cv::TrackerStateEstimatorMILBoosting::TrackerMILTargetState::setFeatures", vec![(pred!(mut, ["features"], ["const cv::Mat*"]), _)]),
	void cv_TrackerStateEstimatorMILBoosting_TrackerMILTargetState_setFeatures_const_MatR(cv::TrackerStateEstimatorMILBoosting::TrackerMILTargetState* instance, const cv::Mat* features, ResultVoid* ocvrs_return) {
		try {
			instance->setFeatures(*features);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	#endif

	#if !defined(OCVRS_TARGET_OS_WINDOWS)
	// isTargetFg()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:608
	// ("cv::TrackerStateEstimatorMILBoosting::TrackerMILTargetState::isTargetFg", vec![(pred!(const, [], []), _)]),
	void cv_TrackerStateEstimatorMILBoosting_TrackerMILTargetState_isTargetFg_const(const cv::TrackerStateEstimatorMILBoosting::TrackerMILTargetState* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isTargetFg();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	#endif

	#if !defined(OCVRS_TARGET_OS_WINDOWS)
	// getFeatures()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:611
	// ("cv::TrackerStateEstimatorMILBoosting::TrackerMILTargetState::getFeatures", vec![(pred!(const, [], []), _)]),
	void cv_TrackerStateEstimatorMILBoosting_TrackerMILTargetState_getFeatures_const(const cv::TrackerStateEstimatorMILBoosting::TrackerMILTargetState* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getFeatures();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	#endif

	// cv::TrackerStateEstimatorMILBoosting::TrackerMILTargetState::to_TrackerTargetState() generated
	// ("cv::TrackerStateEstimatorMILBoosting::TrackerMILTargetState::to_TrackerTargetState", vec![(pred!(mut, [], []), _)]),
	cv::TrackerTargetState* cv_TrackerStateEstimatorMILBoosting_TrackerMILTargetState_to_TrackerTargetState(cv::TrackerStateEstimatorMILBoosting::TrackerMILTargetState* instance) {
			return dynamic_cast<cv::TrackerTargetState*>(instance);
	}

	// cv::TrackerStateEstimatorMILBoosting::TrackerMILTargetState::delete() generated
	// ("cv::TrackerStateEstimatorMILBoosting::TrackerMILTargetState::delete", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerStateEstimatorMILBoosting_TrackerMILTargetState_delete(cv::TrackerStateEstimatorMILBoosting::TrackerMILTargetState* instance) {
			delete instance;
	}

	// TrackerStateEstimatorSVM()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:759
	// ("cv::TrackerStateEstimatorSVM::TrackerStateEstimatorSVM", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerStateEstimatorSVM_TrackerStateEstimatorSVM(Result<cv::TrackerStateEstimatorSVM*>* ocvrs_return) {
		try {
			cv::TrackerStateEstimatorSVM* ret = new cv::TrackerStateEstimatorSVM();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TrackerStateEstimatorSVM::to_TrackerStateEstimator() generated
	// ("cv::TrackerStateEstimatorSVM::to_TrackerStateEstimator", vec![(pred!(mut, [], []), _)]),
	cv::TrackerStateEstimator* cv_TrackerStateEstimatorSVM_to_TrackerStateEstimator(cv::TrackerStateEstimatorSVM* instance) {
			return dynamic_cast<cv::TrackerStateEstimator*>(instance);
	}

	// cv::TrackerStateEstimatorSVM::delete() generated
	// ("cv::TrackerStateEstimatorSVM::delete", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerStateEstimatorSVM_delete(cv::TrackerStateEstimatorSVM* instance) {
			delete instance;
	}

	// create(const TrackerTLD::Params &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1197
	// ("cv::TrackerTLD::create", vec![(pred!(mut, ["parameters"], ["const cv::TrackerTLD::Params*"]), _)]),
	void cv_TrackerTLD_create_const_ParamsR(const cv::TrackerTLD::Params* parameters, Result<cv::Ptr<cv::TrackerTLD>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::TrackerTLD> ret = cv::TrackerTLD::create(*parameters);
			Ok(new cv::Ptr<cv::TrackerTLD>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1199
	// ("cv::TrackerTLD::create", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerTLD_create(Result<cv::Ptr<cv::TrackerTLD>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::TrackerTLD> ret = cv::TrackerTLD::create();
			Ok(new cv::Ptr<cv::TrackerTLD>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TrackerTLD::to_Algorithm() generated
	// ("cv::TrackerTLD::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_TrackerTLD_to_Algorithm(cv::TrackerTLD* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::TrackerTLD::to_Tracker() generated
	// ("cv::TrackerTLD::to_Tracker", vec![(pred!(mut, [], []), _)]),
	cv::Tracker* cv_TrackerTLD_to_Tracker(cv::TrackerTLD* instance) {
			return dynamic_cast<cv::Tracker*>(instance);
	}

	// cv::TrackerTLD::delete() generated
	// ("cv::TrackerTLD::delete", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerTLD_delete(cv::TrackerTLD* instance) {
			delete instance;
	}

	// Params()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1189
	// ("cv::TrackerTLD::Params::Params", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerTLD_Params_Params(Result<cv::TrackerTLD::Params*>* ocvrs_return) {
		try {
			cv::TrackerTLD::Params* ret = new cv::TrackerTLD::Params();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1190
	// ("cv::TrackerTLD::Params::read", vec![(pred!(mut, ["unnamed"], ["const cv::FileNode*"]), _)]),
	void cv_TrackerTLD_Params_read_const_FileNodeR(cv::TrackerTLD::Params* instance, const cv::FileNode* unnamed, ResultVoid* ocvrs_return) {
		try {
			instance->read(*unnamed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1191
	// ("cv::TrackerTLD::Params::write", vec![(pred!(const, ["unnamed"], ["cv::FileStorage*"]), _)]),
	void cv_TrackerTLD_Params_write_const_FileStorageR(const cv::TrackerTLD::Params* instance, cv::FileStorage* unnamed, ResultVoid* ocvrs_return) {
		try {
			instance->write(*unnamed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TrackerTLD::Params::delete() generated
	// ("cv::TrackerTLD::Params::delete", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerTLD_Params_delete(cv::TrackerTLD::Params* instance) {
			delete instance;
	}

	// getTargetPosition()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:342
	// ("cv::TrackerTargetState::getTargetPosition", vec![(pred!(const, [], []), _)]),
	void cv_TrackerTargetState_getTargetPosition_const(const cv::TrackerTargetState* instance, Result<cv::Point2f>* ocvrs_return) {
		try {
			cv::Point2f ret = instance->getTargetPosition();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTargetPosition(const Point2f &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:348
	// ("cv::TrackerTargetState::setTargetPosition", vec![(pred!(mut, ["position"], ["const cv::Point2f*"]), _)]),
	void cv_TrackerTargetState_setTargetPosition_const_Point2fR(cv::TrackerTargetState* instance, const cv::Point2f* position, ResultVoid* ocvrs_return) {
		try {
			instance->setTargetPosition(*position);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTargetWidth()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:353
	// ("cv::TrackerTargetState::getTargetWidth", vec![(pred!(const, [], []), _)]),
	void cv_TrackerTargetState_getTargetWidth_const(const cv::TrackerTargetState* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getTargetWidth();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTargetWidth(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:359
	// ("cv::TrackerTargetState::setTargetWidth", vec![(pred!(mut, ["width"], ["int"]), _)]),
	void cv_TrackerTargetState_setTargetWidth_int(cv::TrackerTargetState* instance, int width, ResultVoid* ocvrs_return) {
		try {
			instance->setTargetWidth(width);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTargetHeight()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:364
	// ("cv::TrackerTargetState::getTargetHeight", vec![(pred!(const, [], []), _)]),
	void cv_TrackerTargetState_getTargetHeight_const(const cv::TrackerTargetState* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getTargetHeight();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTargetHeight(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:370
	// ("cv::TrackerTargetState::setTargetHeight", vec![(pred!(mut, ["height"], ["int"]), _)]),
	void cv_TrackerTargetState_setTargetHeight_int(cv::TrackerTargetState* instance, int height, ResultVoid* ocvrs_return) {
		try {
			instance->setTargetHeight(height);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::TrackerTargetState::defaultNew() generated
	// ("cv::TrackerTargetState::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::TrackerTargetState* cv_TrackerTargetState_defaultNew_const() {
			cv::TrackerTargetState* ret = new cv::TrackerTargetState();
			return ret;
	}

	// cv::TrackerTargetState::delete() generated
	// ("cv::TrackerTargetState::delete", vec![(pred!(mut, [], []), _)]),
	void cv_TrackerTargetState_delete(cv::TrackerTargetState* instance) {
			delete instance;
	}

}
