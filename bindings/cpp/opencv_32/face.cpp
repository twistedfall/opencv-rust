#include "face.hpp"
#include "face_types.hpp"

extern "C" {
	Result<void*> cv_face_createBIF_int_int(int num_bands, int num_rotations) {
		try {
			cv::Ptr<cv::face::BIF> ret = cv::face::createBIF(num_bands, num_rotations);
			return Ok<void*>(new cv::Ptr<cv::face::BIF>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_face_createEigenFaceRecognizer_int_double(int num_components, double threshold) {
		try {
			cv::Ptr<cv::face::BasicFaceRecognizer> ret = cv::face::createEigenFaceRecognizer(num_components, threshold);
			return Ok<void*>(new cv::Ptr<cv::face::BasicFaceRecognizer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_face_createFisherFaceRecognizer_int_double(int num_components, double threshold) {
		try {
			cv::Ptr<cv::face::BasicFaceRecognizer> ret = cv::face::createFisherFaceRecognizer(num_components, threshold);
			return Ok<void*>(new cv::Ptr<cv::face::BasicFaceRecognizer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_face_createLBPHFaceRecognizer_int_int_int_int_double(int radius, int neighbors, int grid_x, int grid_y, double threshold) {
		try {
			cv::Ptr<cv::face::LBPHFaceRecognizer> ret = cv::face::createLBPHFaceRecognizer(radius, neighbors, grid_x, grid_y, threshold);
			return Ok<void*>(new cv::Ptr<cv::face::LBPHFaceRecognizer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<int> cv_face_BIF_getNumBands_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::face::BIF*>(instance)->getNumBands();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_face_BIF_getNumRotations_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::face::BIF*>(instance)->getNumRotations();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_face_BIF_compute_const_const__InputArrayX_const__OutputArrayX(void* instance, void* image, void* features) {
		try {
			reinterpret_cast<cv::face::BIF*>(instance)->compute(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<const cv::_OutputArray*>(features));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_face_BasicFaceRecognizer_getNumComponents_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::face::BasicFaceRecognizer*>(instance)->getNumComponents();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_face_BasicFaceRecognizer_setNumComponents_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::face::BasicFaceRecognizer*>(instance)->setNumComponents(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_face_BasicFaceRecognizer_getThreshold_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::face::BasicFaceRecognizer*>(instance)->getThreshold();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_face_BasicFaceRecognizer_setThreshold_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::face::BasicFaceRecognizer*>(instance)->setThreshold(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_face_BasicFaceRecognizer_getProjections_const(void* instance) {
		try {
			std::vector<cv::Mat> ret = reinterpret_cast<cv::face::BasicFaceRecognizer*>(instance)->getProjections();
			return Ok<void*>(new std::vector<cv::Mat>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_face_BasicFaceRecognizer_getLabels_const(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::face::BasicFaceRecognizer*>(instance)->getLabels();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_face_BasicFaceRecognizer_getEigenValues_const(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::face::BasicFaceRecognizer*>(instance)->getEigenValues();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_face_BasicFaceRecognizer_getEigenVectors_const(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::face::BasicFaceRecognizer*>(instance)->getEigenVectors();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_face_BasicFaceRecognizer_getMean_const(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::face::BasicFaceRecognizer*>(instance)->getMean();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_face_FaceRecognizer_train_const__InputArrayX_const__InputArrayX(void* instance, void* src, void* labels) {
		try {
			reinterpret_cast<cv::face::FaceRecognizer*>(instance)->train(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_InputArray*>(labels));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_face_FaceRecognizer_update_const__InputArrayX_const__InputArrayX(void* instance, void* src, void* labels) {
		try {
			reinterpret_cast<cv::face::FaceRecognizer*>(instance)->update(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<const cv::_InputArray*>(labels));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_face_FaceRecognizer_predict_const_const__InputArrayX(void* instance, void* src) {
		try {
			int ret = reinterpret_cast<cv::face::FaceRecognizer*>(instance)->predict(*reinterpret_cast<const cv::_InputArray*>(src));
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_face_FaceRecognizer_predict_const_const__InputArrayX_intX_doubleX(void* instance, void* src, int* label, double* confidence) {
		try {
			reinterpret_cast<cv::face::FaceRecognizer*>(instance)->predict(*reinterpret_cast<const cv::_InputArray*>(src), *label, *confidence);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_face_FaceRecognizer_predict_const_const__InputArrayX_Ptr_PredictCollector_(void* instance, void* src, void* collector) {
		try {
			reinterpret_cast<cv::face::FaceRecognizer*>(instance)->predict(*reinterpret_cast<const cv::_InputArray*>(src), *reinterpret_cast<cv::Ptr<cv::face::PredictCollector>*>(collector));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_face_FaceRecognizer_save_const_const_StringX(void* instance, const char* filename) {
		try {
			reinterpret_cast<cv::face::FaceRecognizer*>(instance)->save(cv::String(filename));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_face_FaceRecognizer_load_const_StringX(void* instance, const char* filename) {
		try {
			reinterpret_cast<cv::face::FaceRecognizer*>(instance)->load(cv::String(filename));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_face_FaceRecognizer_save_const_FileStorageX(void* instance, void* fs) {
		try {
			reinterpret_cast<cv::face::FaceRecognizer*>(instance)->save(*reinterpret_cast<cv::FileStorage*>(fs));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_face_FaceRecognizer_load_const_FileStorageX(void* instance, void* fs) {
		try {
			reinterpret_cast<cv::face::FaceRecognizer*>(instance)->load(*reinterpret_cast<const cv::FileStorage*>(fs));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_face_FaceRecognizer_setLabelInfo_int_const_StringX(void* instance, int label, const char* strInfo) {
		try {
			reinterpret_cast<cv::face::FaceRecognizer*>(instance)->setLabelInfo(label, cv::String(strInfo));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_face_FaceRecognizer_getLabelInfo_const_int(void* instance, int label) {
		try {
			cv::String ret = reinterpret_cast<cv::face::FaceRecognizer*>(instance)->getLabelInfo(label);
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_face_FaceRecognizer_getLabelsByString_const_const_StringX(void* instance, const char* str) {
		try {
			std::vector<int> ret = reinterpret_cast<cv::face::FaceRecognizer*>(instance)->getLabelsByString(cv::String(str));
			return Ok<void*>(new std::vector<int>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<double> cv_face_FaceRecognizer_getThreshold_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::face::FaceRecognizer*>(instance)->getThreshold();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_face_FaceRecognizer_setThreshold_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::face::FaceRecognizer*>(instance)->setThreshold(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_face_LBPHFaceRecognizer_getGridX_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::face::LBPHFaceRecognizer*>(instance)->getGridX();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_face_LBPHFaceRecognizer_setGridX_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::face::LBPHFaceRecognizer*>(instance)->setGridX(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_face_LBPHFaceRecognizer_getGridY_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::face::LBPHFaceRecognizer*>(instance)->getGridY();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_face_LBPHFaceRecognizer_setGridY_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::face::LBPHFaceRecognizer*>(instance)->setGridY(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_face_LBPHFaceRecognizer_getRadius_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::face::LBPHFaceRecognizer*>(instance)->getRadius();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_face_LBPHFaceRecognizer_setRadius_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::face::LBPHFaceRecognizer*>(instance)->setRadius(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_face_LBPHFaceRecognizer_getNeighbors_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::face::LBPHFaceRecognizer*>(instance)->getNeighbors();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_face_LBPHFaceRecognizer_setNeighbors_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::face::LBPHFaceRecognizer*>(instance)->setNeighbors(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_face_LBPHFaceRecognizer_getThreshold_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::face::LBPHFaceRecognizer*>(instance)->getThreshold();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_face_LBPHFaceRecognizer_setThreshold_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::face::LBPHFaceRecognizer*>(instance)->setThreshold(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_face_LBPHFaceRecognizer_getHistograms_const(void* instance) {
		try {
			std::vector<cv::Mat> ret = reinterpret_cast<cv::face::LBPHFaceRecognizer*>(instance)->getHistograms();
			return Ok<void*>(new std::vector<cv::Mat>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_face_LBPHFaceRecognizer_getLabels_const(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::face::LBPHFaceRecognizer*>(instance)->getLabels();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_face_PredictCollector_init_size_t(void* instance, size_t size) {
		try {
			reinterpret_cast<cv::face::PredictCollector*>(instance)->init(size);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_face_PredictCollector_collect_int_double(void* instance, int label, double dist) {
		try {
			bool ret = reinterpret_cast<cv::face::PredictCollector*>(instance)->collect(label, dist);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	void cv_StandardCollector_delete(cv::face::StandardCollector* instance) {
		delete instance;
	}
	Result<void*> cv_face_StandardCollector_StandardCollector_double(double threshold_) {
		try {
			cv::face::StandardCollector* ret = new cv::face::StandardCollector(threshold_);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_face_StandardCollector_init_size_t(void* instance, size_t size) {
		try {
			reinterpret_cast<cv::face::StandardCollector*>(instance)->init(size);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_face_StandardCollector_collect_int_double(void* instance, int label, double dist) {
		try {
			bool ret = reinterpret_cast<cv::face::StandardCollector*>(instance)->collect(label, dist);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<int> cv_face_StandardCollector_getMinLabel_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::face::StandardCollector*>(instance)->getMinLabel();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<double> cv_face_StandardCollector_getMinDist_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::face::StandardCollector*>(instance)->getMinDist();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result<void*> cv_face_StandardCollector_create_double(double threshold) {
		try {
			cv::Ptr<cv::face::StandardCollector> ret = cv::face::StandardCollector::create(threshold);
			return Ok<void*>(new cv::Ptr<cv::face::StandardCollector>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<cv::face::StandardCollector::PredictResult> cv_face_StandardCollector_PredictResult_PredictResult_int_double(int label_, double distance_) {
		try {
			cv::face::StandardCollector::PredictResult ret(label_, distance_);
			return Ok<cv::face::StandardCollector::PredictResult>(ret);
		} OCVRS_CATCH(Result<cv::face::StandardCollector::PredictResult>)
	}
	
}
