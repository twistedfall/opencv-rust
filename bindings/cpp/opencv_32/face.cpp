#include "face.hpp"
#include "face_types.hpp"

extern "C" {
	Result<cv::Ptr<cv::face::BIF>*> cv_face_createBIF_int_int(int num_bands, int num_rotations) {
		try {
			cv::Ptr<cv::face::BIF> ret = cv::face::createBIF(num_bands, num_rotations);
			return Ok(new cv::Ptr<cv::face::BIF>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::face::BIF>*>))
	}
	
	Result<cv::Ptr<cv::face::BasicFaceRecognizer>*> cv_face_createEigenFaceRecognizer_int_double(int num_components, double threshold) {
		try {
			cv::Ptr<cv::face::BasicFaceRecognizer> ret = cv::face::createEigenFaceRecognizer(num_components, threshold);
			return Ok(new cv::Ptr<cv::face::BasicFaceRecognizer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::face::BasicFaceRecognizer>*>))
	}
	
	Result<cv::Ptr<cv::face::BasicFaceRecognizer>*> cv_face_createFisherFaceRecognizer_int_double(int num_components, double threshold) {
		try {
			cv::Ptr<cv::face::BasicFaceRecognizer> ret = cv::face::createFisherFaceRecognizer(num_components, threshold);
			return Ok(new cv::Ptr<cv::face::BasicFaceRecognizer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::face::BasicFaceRecognizer>*>))
	}
	
	Result<cv::Ptr<cv::face::LBPHFaceRecognizer>*> cv_face_createLBPHFaceRecognizer_int_int_int_int_double(int radius, int neighbors, int grid_x, int grid_y, double threshold) {
		try {
			cv::Ptr<cv::face::LBPHFaceRecognizer> ret = cv::face::createLBPHFaceRecognizer(radius, neighbors, grid_x, grid_y, threshold);
			return Ok(new cv::Ptr<cv::face::LBPHFaceRecognizer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::face::LBPHFaceRecognizer>*>))
	}
	
	Result<int> cv_face_BIF_getNumBands_const(const cv::face::BIF* instance) {
		try {
			int ret = instance->getNumBands();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_face_BIF_getNumRotations_const(const cv::face::BIF* instance) {
		try {
			int ret = instance->getNumRotations();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_face_BIF_compute_const_const__InputArrayX_const__OutputArrayX(const cv::face::BIF* instance, const cv::_InputArray* image, const cv::_OutputArray* features) {
		try {
			instance->compute(*image, *features);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_face_BasicFaceRecognizer_getNumComponents_const(const cv::face::BasicFaceRecognizer* instance) {
		try {
			int ret = instance->getNumComponents();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_face_BasicFaceRecognizer_setNumComponents_int(cv::face::BasicFaceRecognizer* instance, int val) {
		try {
			instance->setNumComponents(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_face_BasicFaceRecognizer_getThreshold_const(const cv::face::BasicFaceRecognizer* instance) {
		try {
			double ret = instance->getThreshold();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_face_BasicFaceRecognizer_setThreshold_double(cv::face::BasicFaceRecognizer* instance, double val) {
		try {
			instance->setThreshold(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<cv::Mat>*> cv_face_BasicFaceRecognizer_getProjections_const(const cv::face::BasicFaceRecognizer* instance) {
		try {
			std::vector<cv::Mat> ret = instance->getProjections();
			return Ok(new std::vector<cv::Mat>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::Mat>*>))
	}
	
	Result<cv::Mat*> cv_face_BasicFaceRecognizer_getLabels_const(const cv::face::BasicFaceRecognizer* instance) {
		try {
			cv::Mat ret = instance->getLabels();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv_face_BasicFaceRecognizer_getEigenValues_const(const cv::face::BasicFaceRecognizer* instance) {
		try {
			cv::Mat ret = instance->getEigenValues();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv_face_BasicFaceRecognizer_getEigenVectors_const(const cv::face::BasicFaceRecognizer* instance) {
		try {
			cv::Mat ret = instance->getEigenVectors();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result<cv::Mat*> cv_face_BasicFaceRecognizer_getMean_const(const cv::face::BasicFaceRecognizer* instance) {
		try {
			cv::Mat ret = instance->getMean();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_face_FaceRecognizer_train_const__InputArrayX_const__InputArrayX(cv::face::FaceRecognizer* instance, const cv::_InputArray* src, const cv::_InputArray* labels) {
		try {
			instance->train(*src, *labels);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_face_FaceRecognizer_update_const__InputArrayX_const__InputArrayX(cv::face::FaceRecognizer* instance, const cv::_InputArray* src, const cv::_InputArray* labels) {
		try {
			instance->update(*src, *labels);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_face_FaceRecognizer_predict_const_const__InputArrayX(const cv::face::FaceRecognizer* instance, const cv::_InputArray* src) {
		try {
			int ret = instance->predict(*src);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_face_FaceRecognizer_predict_const_const__InputArrayX_intR_doubleR(const cv::face::FaceRecognizer* instance, const cv::_InputArray* src, int* label, double* confidence) {
		try {
			instance->predict(*src, *label, *confidence);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_face_FaceRecognizer_predict_const_const__InputArrayX_Ptr_PredictCollector_(const cv::face::FaceRecognizer* instance, const cv::_InputArray* src, cv::Ptr<cv::face::PredictCollector>* collector) {
		try {
			instance->predict(*src, *collector);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_face_FaceRecognizer_save_const_const_StringX(const cv::face::FaceRecognizer* instance, const char* filename) {
		try {
			instance->save(cv::String(filename));
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_face_FaceRecognizer_load_const_StringX(cv::face::FaceRecognizer* instance, const char* filename) {
		try {
			instance->load(cv::String(filename));
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_face_FaceRecognizer_save_const_FileStorageX(const cv::face::FaceRecognizer* instance, cv::FileStorage* fs) {
		try {
			instance->save(*fs);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_face_FaceRecognizer_load_const_FileStorageX(cv::face::FaceRecognizer* instance, const cv::FileStorage* fs) {
		try {
			instance->load(*fs);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_face_FaceRecognizer_setLabelInfo_int_const_StringX(cv::face::FaceRecognizer* instance, int label, const char* strInfo) {
		try {
			instance->setLabelInfo(label, cv::String(strInfo));
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<void*> cv_face_FaceRecognizer_getLabelInfo_const_int(const cv::face::FaceRecognizer* instance, int label) {
		try {
			cv::String ret = instance->getLabelInfo(label);
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result<std::vector<int>*> cv_face_FaceRecognizer_getLabelsByString_const_const_StringX(const cv::face::FaceRecognizer* instance, const char* str) {
		try {
			std::vector<int> ret = instance->getLabelsByString(cv::String(str));
			return Ok(new std::vector<int>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<int>*>))
	}
	
	Result<double> cv_face_FaceRecognizer_getThreshold_const(const cv::face::FaceRecognizer* instance) {
		try {
			double ret = instance->getThreshold();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_face_FaceRecognizer_setThreshold_double(cv::face::FaceRecognizer* instance, double val) {
		try {
			instance->setThreshold(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_face_LBPHFaceRecognizer_getGridX_const(const cv::face::LBPHFaceRecognizer* instance) {
		try {
			int ret = instance->getGridX();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_face_LBPHFaceRecognizer_setGridX_int(cv::face::LBPHFaceRecognizer* instance, int val) {
		try {
			instance->setGridX(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_face_LBPHFaceRecognizer_getGridY_const(const cv::face::LBPHFaceRecognizer* instance) {
		try {
			int ret = instance->getGridY();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_face_LBPHFaceRecognizer_setGridY_int(cv::face::LBPHFaceRecognizer* instance, int val) {
		try {
			instance->setGridY(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_face_LBPHFaceRecognizer_getRadius_const(const cv::face::LBPHFaceRecognizer* instance) {
		try {
			int ret = instance->getRadius();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_face_LBPHFaceRecognizer_setRadius_int(cv::face::LBPHFaceRecognizer* instance, int val) {
		try {
			instance->setRadius(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_face_LBPHFaceRecognizer_getNeighbors_const(const cv::face::LBPHFaceRecognizer* instance) {
		try {
			int ret = instance->getNeighbors();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_face_LBPHFaceRecognizer_setNeighbors_int(cv::face::LBPHFaceRecognizer* instance, int val) {
		try {
			instance->setNeighbors(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_face_LBPHFaceRecognizer_getThreshold_const(const cv::face::LBPHFaceRecognizer* instance) {
		try {
			double ret = instance->getThreshold();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_face_LBPHFaceRecognizer_setThreshold_double(cv::face::LBPHFaceRecognizer* instance, double val) {
		try {
			instance->setThreshold(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<cv::Mat>*> cv_face_LBPHFaceRecognizer_getHistograms_const(const cv::face::LBPHFaceRecognizer* instance) {
		try {
			std::vector<cv::Mat> ret = instance->getHistograms();
			return Ok(new std::vector<cv::Mat>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::Mat>*>))
	}
	
	Result<cv::Mat*> cv_face_LBPHFaceRecognizer_getLabels_const(const cv::face::LBPHFaceRecognizer* instance) {
		try {
			cv::Mat ret = instance->getLabels();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_face_PredictCollector_init_size_t(cv::face::PredictCollector* instance, size_t size) {
		try {
			instance->init(size);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_face_PredictCollector_collect_int_double(cv::face::PredictCollector* instance, int label, double dist) {
		try {
			bool ret = instance->collect(label, dist);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	void cv_StandardCollector_delete(cv::face::StandardCollector* instance) {
		delete instance;
	}
	Result<cv::face::StandardCollector*> cv_face_StandardCollector_StandardCollector_double(double threshold_) {
		try {
			cv::face::StandardCollector* ret = new cv::face::StandardCollector(threshold_);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::face::StandardCollector*>))
	}
	
	Result_void cv_face_StandardCollector_init_size_t(cv::face::StandardCollector* instance, size_t size) {
		try {
			instance->init(size);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_face_StandardCollector_collect_int_double(cv::face::StandardCollector* instance, int label, double dist) {
		try {
			bool ret = instance->collect(label, dist);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<int> cv_face_StandardCollector_getMinLabel_const(const cv::face::StandardCollector* instance) {
		try {
			int ret = instance->getMinLabel();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<double> cv_face_StandardCollector_getMinDist_const(const cv::face::StandardCollector* instance) {
		try {
			double ret = instance->getMinDist();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result<cv::Ptr<cv::face::StandardCollector>*> cv_face_StandardCollector_create_double(double threshold) {
		try {
			cv::Ptr<cv::face::StandardCollector> ret = cv::face::StandardCollector::create(threshold);
			return Ok(new cv::Ptr<cv::face::StandardCollector>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::face::StandardCollector>*>))
	}
	
	Result<cv::face::StandardCollector::PredictResult> cv_face_StandardCollector_PredictResult_PredictResult_int_double(int label_, double distance_) {
		try {
			cv::face::StandardCollector::PredictResult ret(label_, distance_);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::face::StandardCollector::PredictResult>))
	}
	
}
