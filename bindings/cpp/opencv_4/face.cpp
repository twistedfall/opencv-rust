#include "face.hpp"
#include "face_types.hpp"

extern "C" {
	Result<cv::Ptr<cv::face::Facemark>*> cv_face_createFacemarkAAM() {
		try {
			cv::Ptr<cv::face::Facemark> ret = cv::face::createFacemarkAAM();
			return Ok(new cv::Ptr<cv::face::Facemark>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::face::Facemark>*>))
	}
	
	Result<cv::Ptr<cv::face::Facemark>*> cv_face_createFacemarkKazemi() {
		try {
			cv::Ptr<cv::face::Facemark> ret = cv::face::createFacemarkKazemi();
			return Ok(new cv::Ptr<cv::face::Facemark>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::face::Facemark>*>))
	}
	
	Result<cv::Ptr<cv::face::Facemark>*> cv_face_createFacemarkLBF() {
		try {
			cv::Ptr<cv::face::Facemark> ret = cv::face::createFacemarkLBF();
			return Ok(new cv::Ptr<cv::face::Facemark>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::face::Facemark>*>))
	}
	
	Result_void cv_face_drawFacemarks_const__InputOutputArrayR_const__InputArrayR_Scalar(const cv::_InputOutputArray* image, const cv::_InputArray* points, cv::Scalar* color) {
		try {
			cv::face::drawFacemarks(*image, *points, *color);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_face_getFacesHAAR_const__InputArrayR_const__OutputArrayR_const_StringR(const cv::_InputArray* image, const cv::_OutputArray* faces, const char* face_cascade_name) {
		try {
			bool ret = cv::face::getFacesHAAR(*image, *faces, std::string(face_cascade_name));
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_face_getFaces_const__InputArrayR_const__OutputArrayR_CParamsX(const cv::_InputArray* image, const cv::_OutputArray* faces, cv::face::CParams* params) {
		try {
			bool ret = cv::face::getFaces(*image, *faces, params);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_face_loadDatasetList_String_String_vector_String_R_vector_String_R(char* imageList, char* annotationList, std::vector<cv::String>* images, std::vector<cv::String>* annotations) {
		try {
			bool ret = cv::face::loadDatasetList(std::string(imageList), std::string(annotationList), *images, *annotations);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_face_loadFacePoints_String_const__OutputArrayR_float(char* filename, const cv::_OutputArray* points, float offset) {
		try {
			bool ret = cv::face::loadFacePoints(std::string(filename), *points, offset);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_face_loadTrainingData_String_String_vector_String_R_const__OutputArrayR_float(char* imageList, char* groundTruth, std::vector<cv::String>* images, const cv::_OutputArray* facePoints, float offset) {
		try {
			bool ret = cv::face::loadTrainingData(std::string(imageList), std::string(groundTruth), *images, *facePoints, offset);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_face_loadTrainingData_String_vector_String_R_const__OutputArrayR_char_float(char* filename, std::vector<cv::String>* images, const cv::_OutputArray* facePoints, char delim, float offset) {
		try {
			bool ret = cv::face::loadTrainingData(std::string(filename), *images, *facePoints, delim, offset);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_face_loadTrainingData_vector_String__vector_vector_Point2f__R_vector_String_R(std::vector<cv::String>* filename, std::vector<std::vector<cv::Point2f>>* trainlandmarks, std::vector<cv::String>* trainimages) {
		try {
			bool ret = cv::face::loadTrainingData(*filename, *trainlandmarks, *trainimages);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<int> cv_face_BIF_getNumBands_const(const cv::face::BIF* instance) {
		try {
			int ret = instance->getNumBands();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_face_BIF_getNumRotations_const(const cv::face::BIF* instance) {
		try {
			int ret = instance->getNumRotations();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_face_BIF_compute_const_const__InputArrayR_const__OutputArrayR(const cv::face::BIF* instance, const cv::_InputArray* image, const cv::_OutputArray* features) {
		try {
			instance->compute(*image, *features);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::face::BIF>*> cv_face_BIF_create_int_int(int num_bands, int num_rotations) {
		try {
			cv::Ptr<cv::face::BIF> ret = cv::face::BIF::create(num_bands, num_rotations);
			return Ok(new cv::Ptr<cv::face::BIF>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::face::BIF>*>))
	}
	
	Result<int> cv_face_BasicFaceRecognizer_getNumComponents_const(const cv::face::BasicFaceRecognizer* instance) {
		try {
			int ret = instance->getNumComponents();
			return Ok<int>(ret);
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
			return Ok<double>(ret);
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
	
	Result_void cv_face_BasicFaceRecognizer_read_const_FileNodeR(cv::face::BasicFaceRecognizer* instance, const cv::FileNode* fn) {
		try {
			instance->read(*fn);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_face_BasicFaceRecognizer_write_const_FileStorageR(const cv::face::BasicFaceRecognizer* instance, cv::FileStorage* fs) {
		try {
			instance->write(*fs);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_face_BasicFaceRecognizer_empty_const(const cv::face::BasicFaceRecognizer* instance) {
		try {
			bool ret = instance->empty();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<void*> cv_face_CParams_getPropCascade_const(const cv::face::CParams* instance) {
		try {
			cv::String ret = instance->cascade;
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result_void cv_face_CParams_setPropCascade_String(cv::face::CParams* instance, char* val) {
		try {
			instance->cascade = std::string(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_face_CParams_getPropScaleFactor_const(const cv::face::CParams* instance) {
		try {
			double ret = instance->scaleFactor;
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_face_CParams_setPropScaleFactor_double(cv::face::CParams* instance, double val) {
		try {
			instance->scaleFactor = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_face_CParams_getPropMinNeighbors_const(const cv::face::CParams* instance) {
		try {
			int ret = instance->minNeighbors;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_face_CParams_setPropMinNeighbors_int(cv::face::CParams* instance, int val) {
		try {
			instance->minNeighbors = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Size> cv_face_CParams_getPropMinSize_const(const cv::face::CParams* instance) {
		try {
			cv::Size ret = instance->minSize;
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	Result_void cv_face_CParams_setPropMinSize_Size(cv::face::CParams* instance, cv::Size* val) {
		try {
			instance->minSize = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Size> cv_face_CParams_getPropMaxSize_const(const cv::face::CParams* instance) {
		try {
			cv::Size ret = instance->maxSize;
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	Result_void cv_face_CParams_setPropMaxSize_Size(cv::face::CParams* instance, cv::Size* val) {
		try {
			instance->maxSize = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::CascadeClassifier*> cv_face_CParams_getPropFace_cascade(cv::face::CParams* instance) {
		try {
			cv::CascadeClassifier ret = instance->face_cascade;
			return Ok(new cv::CascadeClassifier(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::CascadeClassifier*>))
	}
	
	Result_void cv_face_CParams_setPropFace_cascade_CascadeClassifier(cv::face::CParams* instance, cv::CascadeClassifier* val) {
		try {
			instance->face_cascade = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_CParams_delete(cv::face::CParams* instance) {
		delete instance;
	}
	Result<cv::face::CParams*> cv_face_CParams_CParams_String_double_int_Size_Size(char* cascade_model, double sf, int minN, cv::Size* minSz, cv::Size* maxSz) {
		try {
			cv::face::CParams* ret = new cv::face::CParams(std::string(cascade_model), sf, minN, *minSz, *maxSz);
			return Ok<cv::face::CParams*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::face::CParams*>))
	}
	
	Result<cv::Ptr<cv::face::EigenFaceRecognizer>*> cv_face_EigenFaceRecognizer_create_int_double(int num_components, double threshold) {
		try {
			cv::Ptr<cv::face::EigenFaceRecognizer> ret = cv::face::EigenFaceRecognizer::create(num_components, threshold);
			return Ok(new cv::Ptr<cv::face::EigenFaceRecognizer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::face::EigenFaceRecognizer>*>))
	}
	
	Result_void cv_face_FaceRecognizer_train_const__InputArrayR_const__InputArrayR(cv::face::FaceRecognizer* instance, const cv::_InputArray* src, const cv::_InputArray* labels) {
		try {
			instance->train(*src, *labels);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_face_FaceRecognizer_update_const__InputArrayR_const__InputArrayR(cv::face::FaceRecognizer* instance, const cv::_InputArray* src, const cv::_InputArray* labels) {
		try {
			instance->update(*src, *labels);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_face_FaceRecognizer_predict_const_const__InputArrayR(const cv::face::FaceRecognizer* instance, const cv::_InputArray* src) {
		try {
			int ret = instance->predict(*src);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_face_FaceRecognizer_predict_const_const__InputArrayR_intR_doubleR(const cv::face::FaceRecognizer* instance, const cv::_InputArray* src, int* label, double* confidence) {
		try {
			instance->predict(*src, *label, *confidence);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_face_FaceRecognizer_predict_const_const__InputArrayR_Ptr_PredictCollector_(const cv::face::FaceRecognizer* instance, const cv::_InputArray* src, cv::Ptr<cv::face::PredictCollector>* collector) {
		try {
			instance->predict(*src, *collector);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_face_FaceRecognizer_write_const_const_StringR(const cv::face::FaceRecognizer* instance, const char* filename) {
		try {
			instance->write(std::string(filename));
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_face_FaceRecognizer_read_const_StringR(cv::face::FaceRecognizer* instance, const char* filename) {
		try {
			instance->read(std::string(filename));
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_face_FaceRecognizer_write_const_FileStorageR(const cv::face::FaceRecognizer* instance, cv::FileStorage* fs) {
		try {
			instance->write(*fs);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_face_FaceRecognizer_read_const_FileNodeR(cv::face::FaceRecognizer* instance, const cv::FileNode* fn) {
		try {
			instance->read(*fn);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_face_FaceRecognizer_empty_const(const cv::face::FaceRecognizer* instance) {
		try {
			bool ret = instance->empty();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_face_FaceRecognizer_setLabelInfo_int_const_StringR(cv::face::FaceRecognizer* instance, int label, const char* strInfo) {
		try {
			instance->setLabelInfo(label, std::string(strInfo));
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<void*> cv_face_FaceRecognizer_getLabelInfo_const_int(const cv::face::FaceRecognizer* instance, int label) {
		try {
			cv::String ret = instance->getLabelInfo(label);
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result<std::vector<int>*> cv_face_FaceRecognizer_getLabelsByString_const_const_StringR(const cv::face::FaceRecognizer* instance, const char* str) {
		try {
			std::vector<int> ret = instance->getLabelsByString(std::string(str));
			return Ok(new std::vector<int>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<int>*>))
	}
	
	Result<double> cv_face_FaceRecognizer_getThreshold_const(const cv::face::FaceRecognizer* instance) {
		try {
			double ret = instance->getThreshold();
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_face_FaceRecognizer_setThreshold_double(cv::face::FaceRecognizer* instance, double val) {
		try {
			instance->setThreshold(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_face_Facemark_loadModel_String(cv::face::Facemark* instance, char* model) {
		try {
			instance->loadModel(std::string(model));
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_face_Facemark_fit_const__InputArrayR_const__InputArrayR_const__OutputArrayR(cv::face::Facemark* instance, const cv::_InputArray* image, const cv::_InputArray* faces, const cv::_OutputArray* landmarks) {
		try {
			bool ret = instance->fit(*image, *faces, *landmarks);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_face_FacemarkAAM_fitConfig_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const_vector_Config_R(cv::face::FacemarkAAM* instance, const cv::_InputArray* image, const cv::_InputArray* roi, const cv::_OutputArray* _landmarks, const std::vector<cv::face::FacemarkAAM::Config>* runtime_params) {
		try {
			bool ret = instance->fitConfig(*image, *roi, *_landmarks, *runtime_params);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<cv::Ptr<cv::face::FacemarkAAM>*> cv_face_FacemarkAAM_create_const_ParamsR(const cv::face::FacemarkAAM::Params* parameters) {
		try {
			cv::Ptr<cv::face::FacemarkAAM> ret = cv::face::FacemarkAAM::create(*parameters);
			return Ok(new cv::Ptr<cv::face::FacemarkAAM>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::face::FacemarkAAM>*>))
	}
	
	Result<cv::Mat*> cv_face_FacemarkAAM_Config_getPropR(cv::face::FacemarkAAM::Config* instance) {
		try {
			cv::Mat ret = instance->R;
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_face_FacemarkAAM_Config_setPropR_Mat(cv::face::FacemarkAAM::Config* instance, cv::Mat* val) {
		try {
			instance->R = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Point2f> cv_face_FacemarkAAM_Config_getPropT_const(const cv::face::FacemarkAAM::Config* instance) {
		try {
			cv::Point2f ret = instance->t;
			return Ok<cv::Point2f>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point2f>))
	}
	
	Result_void cv_face_FacemarkAAM_Config_setPropT_Point2f(cv::face::FacemarkAAM::Config* instance, cv::Point2f* val) {
		try {
			instance->t = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_face_FacemarkAAM_Config_getPropScale_const(const cv::face::FacemarkAAM::Config* instance) {
		try {
			float ret = instance->scale;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_face_FacemarkAAM_Config_setPropScale_float(cv::face::FacemarkAAM::Config* instance, float val) {
		try {
			instance->scale = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_face_FacemarkAAM_Config_getPropModel_scale_idx_const(const cv::face::FacemarkAAM::Config* instance) {
		try {
			int ret = instance->model_scale_idx;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_face_FacemarkAAM_Config_setPropModel_scale_idx_int(cv::face::FacemarkAAM::Config* instance, int val) {
		try {
			instance->model_scale_idx = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_FacemarkAAM_Config_delete(cv::face::FacemarkAAM::Config* instance) {
		delete instance;
	}
	Result<cv::face::FacemarkAAM::Config*> cv_face_FacemarkAAM_Config_Config_Mat_Point2f_float_int(cv::Mat* rot, cv::Point2f* trans, float scaling, int scale_id) {
		try {
			cv::face::FacemarkAAM::Config* ret = new cv::face::FacemarkAAM::Config(*rot, *trans, scaling, scale_id);
			return Ok<cv::face::FacemarkAAM::Config*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::face::FacemarkAAM::Config*>))
	}
	
	Result<std::vector<cv::Point2f>*> cv_face_FacemarkAAM_Data_getPropS0(cv::face::FacemarkAAM::Data* instance) {
		try {
			std::vector<cv::Point2f> ret = instance->s0;
			return Ok(new std::vector<cv::Point2f>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::Point2f>*>))
	}
	
	Result_void cv_face_FacemarkAAM_Data_setPropS0_vector_Point2f_(cv::face::FacemarkAAM::Data* instance, std::vector<cv::Point2f>* val) {
		try {
			instance->s0 = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_FacemarkAAM_Data_delete(cv::face::FacemarkAAM::Data* instance) {
		delete instance;
	}
	Result<std::vector<float>*> cv_face_FacemarkAAM_Model_getPropScales(cv::face::FacemarkAAM::Model* instance) {
		try {
			std::vector<float> ret = instance->scales;
			return Ok(new std::vector<float>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<float>*>))
	}
	
	Result_void cv_face_FacemarkAAM_Model_setPropScales_vector_float_(cv::face::FacemarkAAM::Model* instance, std::vector<float>* val) {
		try {
			instance->scales = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<cv::Vec3i>*> cv_face_FacemarkAAM_Model_getPropTriangles(cv::face::FacemarkAAM::Model* instance) {
		try {
			std::vector<cv::Vec3i> ret = instance->triangles;
			return Ok(new std::vector<cv::Vec3i>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::Vec3i>*>))
	}
	
	Result_void cv_face_FacemarkAAM_Model_setPropTriangles_vector_Vec3i_(cv::face::FacemarkAAM::Model* instance, std::vector<cv::Vec3i>* val) {
		try {
			instance->triangles = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<cv::face::FacemarkAAM::Model::Texture>*> cv_face_FacemarkAAM_Model_getPropTextures(cv::face::FacemarkAAM::Model* instance) {
		try {
			std::vector<cv::face::FacemarkAAM::Model::Texture> ret = instance->textures;
			return Ok(new std::vector<cv::face::FacemarkAAM::Model::Texture>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::face::FacemarkAAM::Model::Texture>*>))
	}
	
	Result_void cv_face_FacemarkAAM_Model_setPropTextures_vector_Texture_(cv::face::FacemarkAAM::Model* instance, std::vector<cv::face::FacemarkAAM::Model::Texture>* val) {
		try {
			instance->textures = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<cv::Point2f>*> cv_face_FacemarkAAM_Model_getPropS0(cv::face::FacemarkAAM::Model* instance) {
		try {
			std::vector<cv::Point2f> ret = instance->s0;
			return Ok(new std::vector<cv::Point2f>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::Point2f>*>))
	}
	
	Result_void cv_face_FacemarkAAM_Model_setPropS0_vector_Point2f_(cv::face::FacemarkAAM::Model* instance, std::vector<cv::Point2f>* val) {
		try {
			instance->s0 = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_face_FacemarkAAM_Model_getPropS(cv::face::FacemarkAAM::Model* instance) {
		try {
			cv::Mat ret = instance->S;
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_face_FacemarkAAM_Model_setPropS_Mat(cv::face::FacemarkAAM::Model* instance, cv::Mat* val) {
		try {
			instance->S = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_face_FacemarkAAM_Model_getPropQ(cv::face::FacemarkAAM::Model* instance) {
		try {
			cv::Mat ret = instance->Q;
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_face_FacemarkAAM_Model_setPropQ_Mat(cv::face::FacemarkAAM::Model* instance, cv::Mat* val) {
		try {
			instance->Q = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_FacemarkAAM_Model_delete(cv::face::FacemarkAAM::Model* instance) {
		delete instance;
	}
	Result<int> cv_face_FacemarkAAM_Model_Texture_getPropMax_m_const(const cv::face::FacemarkAAM::Model::Texture* instance) {
		try {
			int ret = instance->max_m;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_face_FacemarkAAM_Model_Texture_setPropMax_m_int(cv::face::FacemarkAAM::Model::Texture* instance, int val) {
		try {
			instance->max_m = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Rect> cv_face_FacemarkAAM_Model_Texture_getPropResolution_const(const cv::face::FacemarkAAM::Model::Texture* instance) {
		try {
			cv::Rect ret = instance->resolution;
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	Result_void cv_face_FacemarkAAM_Model_Texture_setPropResolution_Rect(cv::face::FacemarkAAM::Model::Texture* instance, cv::Rect* val) {
		try {
			instance->resolution = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_face_FacemarkAAM_Model_Texture_getPropA(cv::face::FacemarkAAM::Model::Texture* instance) {
		try {
			cv::Mat ret = instance->A;
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_face_FacemarkAAM_Model_Texture_setPropA_Mat(cv::face::FacemarkAAM::Model::Texture* instance, cv::Mat* val) {
		try {
			instance->A = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_face_FacemarkAAM_Model_Texture_getPropA0(cv::face::FacemarkAAM::Model::Texture* instance) {
		try {
			cv::Mat ret = instance->A0;
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_face_FacemarkAAM_Model_Texture_setPropA0_Mat(cv::face::FacemarkAAM::Model::Texture* instance, cv::Mat* val) {
		try {
			instance->A0 = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_face_FacemarkAAM_Model_Texture_getPropAA(cv::face::FacemarkAAM::Model::Texture* instance) {
		try {
			cv::Mat ret = instance->AA;
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_face_FacemarkAAM_Model_Texture_setPropAA_Mat(cv::face::FacemarkAAM::Model::Texture* instance, cv::Mat* val) {
		try {
			instance->AA = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Mat*> cv_face_FacemarkAAM_Model_Texture_getPropAA0(cv::face::FacemarkAAM::Model::Texture* instance) {
		try {
			cv::Mat ret = instance->AA0;
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	Result_void cv_face_FacemarkAAM_Model_Texture_setPropAA0_Mat(cv::face::FacemarkAAM::Model::Texture* instance, cv::Mat* val) {
		try {
			instance->AA0 = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<std::vector<cv::Point>>*> cv_face_FacemarkAAM_Model_Texture_getPropTextureIdx(cv::face::FacemarkAAM::Model::Texture* instance) {
		try {
			std::vector<std::vector<cv::Point>> ret = instance->textureIdx;
			return Ok(new std::vector<std::vector<cv::Point>>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<std::vector<cv::Point>>*>))
	}
	
	Result_void cv_face_FacemarkAAM_Model_Texture_setPropTextureIdx_vector_vector_Point__(cv::face::FacemarkAAM::Model::Texture* instance, std::vector<std::vector<cv::Point>>* val) {
		try {
			instance->textureIdx = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<cv::Point2f>*> cv_face_FacemarkAAM_Model_Texture_getPropBase_shape(cv::face::FacemarkAAM::Model::Texture* instance) {
		try {
			std::vector<cv::Point2f> ret = instance->base_shape;
			return Ok(new std::vector<cv::Point2f>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::Point2f>*>))
	}
	
	Result_void cv_face_FacemarkAAM_Model_Texture_setPropBase_shape_vector_Point2f_(cv::face::FacemarkAAM::Model::Texture* instance, std::vector<cv::Point2f>* val) {
		try {
			instance->base_shape = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<int>*> cv_face_FacemarkAAM_Model_Texture_getPropInd1(cv::face::FacemarkAAM::Model::Texture* instance) {
		try {
			std::vector<int> ret = instance->ind1;
			return Ok(new std::vector<int>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<int>*>))
	}
	
	Result_void cv_face_FacemarkAAM_Model_Texture_setPropInd1_vector_int_(cv::face::FacemarkAAM::Model::Texture* instance, std::vector<int>* val) {
		try {
			instance->ind1 = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<int>*> cv_face_FacemarkAAM_Model_Texture_getPropInd2(cv::face::FacemarkAAM::Model::Texture* instance) {
		try {
			std::vector<int> ret = instance->ind2;
			return Ok(new std::vector<int>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<int>*>))
	}
	
	Result_void cv_face_FacemarkAAM_Model_Texture_setPropInd2_vector_int_(cv::face::FacemarkAAM::Model::Texture* instance, std::vector<int>* val) {
		try {
			instance->ind2 = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_FacemarkAAM_Model_Texture_delete(cv::face::FacemarkAAM::Model::Texture* instance) {
		delete instance;
	}
	Result<void*> cv_face_FacemarkAAM_Params_getPropModel_filename_const(const cv::face::FacemarkAAM::Params* instance) {
		try {
			std::string ret = instance->model_filename;
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result_void cv_face_FacemarkAAM_Params_setPropModel_filename_string(cv::face::FacemarkAAM::Params* instance, char* val) {
		try {
			instance->model_filename = std::string(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_face_FacemarkAAM_Params_getPropM_const(const cv::face::FacemarkAAM::Params* instance) {
		try {
			int ret = instance->m;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_face_FacemarkAAM_Params_setPropM_int(cv::face::FacemarkAAM::Params* instance, int val) {
		try {
			instance->m = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_face_FacemarkAAM_Params_getPropN_const(const cv::face::FacemarkAAM::Params* instance) {
		try {
			int ret = instance->n;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_face_FacemarkAAM_Params_setPropN_int(cv::face::FacemarkAAM::Params* instance, int val) {
		try {
			instance->n = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_face_FacemarkAAM_Params_getPropN_iter_const(const cv::face::FacemarkAAM::Params* instance) {
		try {
			int ret = instance->n_iter;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_face_FacemarkAAM_Params_setPropN_iter_int(cv::face::FacemarkAAM::Params* instance, int val) {
		try {
			instance->n_iter = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_face_FacemarkAAM_Params_getPropVerbose_const(const cv::face::FacemarkAAM::Params* instance) {
		try {
			bool ret = instance->verbose;
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_face_FacemarkAAM_Params_setPropVerbose_bool(cv::face::FacemarkAAM::Params* instance, bool val) {
		try {
			instance->verbose = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_face_FacemarkAAM_Params_getPropSave_model_const(const cv::face::FacemarkAAM::Params* instance) {
		try {
			bool ret = instance->save_model;
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_face_FacemarkAAM_Params_setPropSave_model_bool(cv::face::FacemarkAAM::Params* instance, bool val) {
		try {
			instance->save_model = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_face_FacemarkAAM_Params_getPropMax_m_const(const cv::face::FacemarkAAM::Params* instance) {
		try {
			int ret = instance->max_m;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_face_FacemarkAAM_Params_setPropMax_m_int(cv::face::FacemarkAAM::Params* instance, int val) {
		try {
			instance->max_m = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_face_FacemarkAAM_Params_getPropMax_n_const(const cv::face::FacemarkAAM::Params* instance) {
		try {
			int ret = instance->max_n;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_face_FacemarkAAM_Params_setPropMax_n_int(cv::face::FacemarkAAM::Params* instance, int val) {
		try {
			instance->max_n = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_face_FacemarkAAM_Params_getPropTexture_max_m_const(const cv::face::FacemarkAAM::Params* instance) {
		try {
			int ret = instance->texture_max_m;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_face_FacemarkAAM_Params_setPropTexture_max_m_int(cv::face::FacemarkAAM::Params* instance, int val) {
		try {
			instance->texture_max_m = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<float>*> cv_face_FacemarkAAM_Params_getPropScales(cv::face::FacemarkAAM::Params* instance) {
		try {
			std::vector<float> ret = instance->scales;
			return Ok(new std::vector<float>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<float>*>))
	}
	
	Result_void cv_face_FacemarkAAM_Params_setPropScales_vector_float_(cv::face::FacemarkAAM::Params* instance, std::vector<float>* val) {
		try {
			instance->scales = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_FacemarkAAM_Params_delete(cv::face::FacemarkAAM::Params* instance) {
		delete instance;
	}
	Result<cv::face::FacemarkAAM::Params*> cv_face_FacemarkAAM_Params_Params() {
		try {
			cv::face::FacemarkAAM::Params* ret = new cv::face::FacemarkAAM::Params();
			return Ok<cv::face::FacemarkAAM::Params*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::face::FacemarkAAM::Params*>))
	}
	
	Result_void cv_face_FacemarkAAM_Params_read_const_FileNodeR(cv::face::FacemarkAAM::Params* instance, const cv::FileNode* unnamed) {
		try {
			instance->read(*unnamed);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_face_FacemarkAAM_Params_write_const_FileStorageR(const cv::face::FacemarkAAM::Params* instance, cv::FileStorage* unnamed) {
		try {
			instance->write(*unnamed);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::face::FacemarkKazemi>*> cv_face_FacemarkKazemi_create_const_ParamsR(const cv::face::FacemarkKazemi::Params* parameters) {
		try {
			cv::Ptr<cv::face::FacemarkKazemi> ret = cv::face::FacemarkKazemi::create(*parameters);
			return Ok(new cv::Ptr<cv::face::FacemarkKazemi>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::face::FacemarkKazemi>*>))
	}
	
	Result<bool> cv_face_FacemarkKazemi_training_vector_Mat_R_vector_vector_Point2f__R_string_Size_string(cv::face::FacemarkKazemi* instance, std::vector<cv::Mat>* images, std::vector<std::vector<cv::Point2f>>* landmarks, char* configfile, cv::Size* scale, char* modelFilename) {
		try {
			bool ret = instance->training(*images, *landmarks, std::string(configfile), *scale, std::string(modelFilename));
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_face_FacemarkKazemi_setFaceDetector_bool__X__const_cv__InputArrayR__const_cv__OutputArrayR__voidX__voidX(cv::face::FacemarkKazemi* instance, bool (*f)(const cv::_InputArray&, const cv::_OutputArray&, void*), void* userData) {
		try {
			bool ret = instance->setFaceDetector(f, userData);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_face_FacemarkKazemi_getFaces_const__InputArrayR_const__OutputArrayR(cv::face::FacemarkKazemi* instance, const cv::_InputArray* image, const cv::_OutputArray* faces) {
		try {
			bool ret = instance->getFaces(*image, *faces);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<unsigned long> cv_face_FacemarkKazemi_Params_getPropCascade_depth_const(const cv::face::FacemarkKazemi::Params* instance) {
		try {
			unsigned long ret = instance->cascade_depth;
			return Ok<unsigned long>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned long>))
	}
	
	Result_void cv_face_FacemarkKazemi_Params_setPropCascade_depth_unsigned_long(cv::face::FacemarkKazemi::Params* instance, unsigned long val) {
		try {
			instance->cascade_depth = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<unsigned long> cv_face_FacemarkKazemi_Params_getPropTree_depth_const(const cv::face::FacemarkKazemi::Params* instance) {
		try {
			unsigned long ret = instance->tree_depth;
			return Ok<unsigned long>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned long>))
	}
	
	Result_void cv_face_FacemarkKazemi_Params_setPropTree_depth_unsigned_long(cv::face::FacemarkKazemi::Params* instance, unsigned long val) {
		try {
			instance->tree_depth = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<unsigned long> cv_face_FacemarkKazemi_Params_getPropNum_trees_per_cascade_level_const(const cv::face::FacemarkKazemi::Params* instance) {
		try {
			unsigned long ret = instance->num_trees_per_cascade_level;
			return Ok<unsigned long>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned long>))
	}
	
	Result_void cv_face_FacemarkKazemi_Params_setPropNum_trees_per_cascade_level_unsigned_long(cv::face::FacemarkKazemi::Params* instance, unsigned long val) {
		try {
			instance->num_trees_per_cascade_level = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_face_FacemarkKazemi_Params_getPropLearning_rate_const(const cv::face::FacemarkKazemi::Params* instance) {
		try {
			float ret = instance->learning_rate;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_face_FacemarkKazemi_Params_setPropLearning_rate_float(cv::face::FacemarkKazemi::Params* instance, float val) {
		try {
			instance->learning_rate = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<unsigned long> cv_face_FacemarkKazemi_Params_getPropOversampling_amount_const(const cv::face::FacemarkKazemi::Params* instance) {
		try {
			unsigned long ret = instance->oversampling_amount;
			return Ok<unsigned long>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned long>))
	}
	
	Result_void cv_face_FacemarkKazemi_Params_setPropOversampling_amount_unsigned_long(cv::face::FacemarkKazemi::Params* instance, unsigned long val) {
		try {
			instance->oversampling_amount = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<unsigned long> cv_face_FacemarkKazemi_Params_getPropNum_test_coordinates_const(const cv::face::FacemarkKazemi::Params* instance) {
		try {
			unsigned long ret = instance->num_test_coordinates;
			return Ok<unsigned long>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned long>))
	}
	
	Result_void cv_face_FacemarkKazemi_Params_setPropNum_test_coordinates_unsigned_long(cv::face::FacemarkKazemi::Params* instance, unsigned long val) {
		try {
			instance->num_test_coordinates = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_face_FacemarkKazemi_Params_getPropLambda_const(const cv::face::FacemarkKazemi::Params* instance) {
		try {
			float ret = instance->lambda;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_face_FacemarkKazemi_Params_setPropLambda_float(cv::face::FacemarkKazemi::Params* instance, float val) {
		try {
			instance->lambda = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<unsigned long> cv_face_FacemarkKazemi_Params_getPropNum_test_splits_const(const cv::face::FacemarkKazemi::Params* instance) {
		try {
			unsigned long ret = instance->num_test_splits;
			return Ok<unsigned long>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned long>))
	}
	
	Result_void cv_face_FacemarkKazemi_Params_setPropNum_test_splits_unsigned_long(cv::face::FacemarkKazemi::Params* instance, unsigned long val) {
		try {
			instance->num_test_splits = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<void*> cv_face_FacemarkKazemi_Params_getPropConfigfile_const(const cv::face::FacemarkKazemi::Params* instance) {
		try {
			cv::String ret = instance->configfile;
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result_void cv_face_FacemarkKazemi_Params_setPropConfigfile_String(cv::face::FacemarkKazemi::Params* instance, char* val) {
		try {
			instance->configfile = std::string(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_FacemarkKazemi_Params_delete(cv::face::FacemarkKazemi::Params* instance) {
		delete instance;
	}
	Result<cv::face::FacemarkKazemi::Params*> cv_face_FacemarkKazemi_Params_Params() {
		try {
			cv::face::FacemarkKazemi::Params* ret = new cv::face::FacemarkKazemi::Params();
			return Ok<cv::face::FacemarkKazemi::Params*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::face::FacemarkKazemi::Params*>))
	}
	
	Result<cv::Ptr<cv::face::FacemarkLBF>*> cv_face_FacemarkLBF_create_const_ParamsR(const cv::face::FacemarkLBF::Params* parameters) {
		try {
			cv::Ptr<cv::face::FacemarkLBF> ret = cv::face::FacemarkLBF::create(*parameters);
			return Ok(new cv::Ptr<cv::face::FacemarkLBF>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::face::FacemarkLBF>*>))
	}
	
	Result<double> cv_face_FacemarkLBF_Params_getPropShape_offset_const(const cv::face::FacemarkLBF::Params* instance) {
		try {
			double ret = instance->shape_offset;
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_face_FacemarkLBF_Params_setPropShape_offset_double(cv::face::FacemarkLBF::Params* instance, double val) {
		try {
			instance->shape_offset = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<void*> cv_face_FacemarkLBF_Params_getPropCascade_face_const(const cv::face::FacemarkLBF::Params* instance) {
		try {
			cv::String ret = instance->cascade_face;
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result_void cv_face_FacemarkLBF_Params_setPropCascade_face_String(cv::face::FacemarkLBF::Params* instance, char* val) {
		try {
			instance->cascade_face = std::string(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_face_FacemarkLBF_Params_getPropVerbose_const(const cv::face::FacemarkLBF::Params* instance) {
		try {
			bool ret = instance->verbose;
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_face_FacemarkLBF_Params_setPropVerbose_bool(cv::face::FacemarkLBF::Params* instance, bool val) {
		try {
			instance->verbose = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_face_FacemarkLBF_Params_getPropN_landmarks_const(const cv::face::FacemarkLBF::Params* instance) {
		try {
			int ret = instance->n_landmarks;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_face_FacemarkLBF_Params_setPropN_landmarks_int(cv::face::FacemarkLBF::Params* instance, int val) {
		try {
			instance->n_landmarks = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_face_FacemarkLBF_Params_getPropInitShape_n_const(const cv::face::FacemarkLBF::Params* instance) {
		try {
			int ret = instance->initShape_n;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_face_FacemarkLBF_Params_setPropInitShape_n_int(cv::face::FacemarkLBF::Params* instance, int val) {
		try {
			instance->initShape_n = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_face_FacemarkLBF_Params_getPropStages_n_const(const cv::face::FacemarkLBF::Params* instance) {
		try {
			int ret = instance->stages_n;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_face_FacemarkLBF_Params_setPropStages_n_int(cv::face::FacemarkLBF::Params* instance, int val) {
		try {
			instance->stages_n = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_face_FacemarkLBF_Params_getPropTree_n_const(const cv::face::FacemarkLBF::Params* instance) {
		try {
			int ret = instance->tree_n;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_face_FacemarkLBF_Params_setPropTree_n_int(cv::face::FacemarkLBF::Params* instance, int val) {
		try {
			instance->tree_n = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_face_FacemarkLBF_Params_getPropTree_depth_const(const cv::face::FacemarkLBF::Params* instance) {
		try {
			int ret = instance->tree_depth;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_face_FacemarkLBF_Params_setPropTree_depth_int(cv::face::FacemarkLBF::Params* instance, int val) {
		try {
			instance->tree_depth = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<double> cv_face_FacemarkLBF_Params_getPropBagging_overlap_const(const cv::face::FacemarkLBF::Params* instance) {
		try {
			double ret = instance->bagging_overlap;
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_face_FacemarkLBF_Params_setPropBagging_overlap_double(cv::face::FacemarkLBF::Params* instance, double val) {
		try {
			instance->bagging_overlap = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<void*> cv_face_FacemarkLBF_Params_getPropModel_filename_const(const cv::face::FacemarkLBF::Params* instance) {
		try {
			std::string ret = instance->model_filename;
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result_void cv_face_FacemarkLBF_Params_setPropModel_filename_string(cv::face::FacemarkLBF::Params* instance, char* val) {
		try {
			instance->model_filename = std::string(val);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_face_FacemarkLBF_Params_getPropSave_model_const(const cv::face::FacemarkLBF::Params* instance) {
		try {
			bool ret = instance->save_model;
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_face_FacemarkLBF_Params_setPropSave_model_bool(cv::face::FacemarkLBF::Params* instance, bool val) {
		try {
			instance->save_model = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<unsigned int> cv_face_FacemarkLBF_Params_getPropSeed_const(const cv::face::FacemarkLBF::Params* instance) {
		try {
			unsigned int ret = instance->seed;
			return Ok<unsigned int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned int>))
	}
	
	Result_void cv_face_FacemarkLBF_Params_setPropSeed_unsigned_int(cv::face::FacemarkLBF::Params* instance, unsigned int val) {
		try {
			instance->seed = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<int>*> cv_face_FacemarkLBF_Params_getPropFeats_m(cv::face::FacemarkLBF::Params* instance) {
		try {
			std::vector<int> ret = instance->feats_m;
			return Ok(new std::vector<int>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<int>*>))
	}
	
	Result_void cv_face_FacemarkLBF_Params_setPropFeats_m_vector_int_(cv::face::FacemarkLBF::Params* instance, std::vector<int>* val) {
		try {
			instance->feats_m = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<std::vector<double>*> cv_face_FacemarkLBF_Params_getPropRadius_m(cv::face::FacemarkLBF::Params* instance) {
		try {
			std::vector<double> ret = instance->radius_m;
			return Ok(new std::vector<double>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<double>*>))
	}
	
	Result_void cv_face_FacemarkLBF_Params_setPropRadius_m_vector_double_(cv::face::FacemarkLBF::Params* instance, std::vector<double>* val) {
		try {
			instance->radius_m = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Rect> cv_face_FacemarkLBF_Params_getPropDetectROI_const(const cv::face::FacemarkLBF::Params* instance) {
		try {
			cv::Rect ret = instance->detectROI;
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	Result_void cv_face_FacemarkLBF_Params_setPropDetectROI_Rect(cv::face::FacemarkLBF::Params* instance, cv::Rect* val) {
		try {
			instance->detectROI = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_FacemarkLBF_Params_delete(cv::face::FacemarkLBF::Params* instance) {
		delete instance;
	}
	Result<cv::face::FacemarkLBF::Params*> cv_face_FacemarkLBF_Params_Params() {
		try {
			cv::face::FacemarkLBF::Params* ret = new cv::face::FacemarkLBF::Params();
			return Ok<cv::face::FacemarkLBF::Params*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::face::FacemarkLBF::Params*>))
	}
	
	Result_void cv_face_FacemarkLBF_Params_read_const_FileNodeR(cv::face::FacemarkLBF::Params* instance, const cv::FileNode* unnamed) {
		try {
			instance->read(*unnamed);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_face_FacemarkLBF_Params_write_const_FileStorageR(const cv::face::FacemarkLBF::Params* instance, cv::FileStorage* unnamed) {
		try {
			instance->write(*unnamed);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_face_FacemarkTrain_addTrainingSample_const__InputArrayR_const__InputArrayR(cv::face::FacemarkTrain* instance, const cv::_InputArray* image, const cv::_InputArray* landmarks) {
		try {
			bool ret = instance->addTrainingSample(*image, *landmarks);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_face_FacemarkTrain_training_voidX(cv::face::FacemarkTrain* instance, void* parameters) {
		try {
			instance->training(parameters);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_face_FacemarkTrain_setFaceDetector_FN_FaceDetector_voidX(cv::face::FacemarkTrain* instance, cv::face::FN_FaceDetector detector, void* userData) {
		try {
			bool ret = instance->setFaceDetector(detector, userData);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_face_FacemarkTrain_getFaces_const__InputArrayR_const__OutputArrayR(cv::face::FacemarkTrain* instance, const cv::_InputArray* image, const cv::_OutputArray* faces) {
		try {
			bool ret = instance->getFaces(*image, *faces);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_face_FacemarkTrain_getData_voidX(cv::face::FacemarkTrain* instance, void* items) {
		try {
			bool ret = instance->getData(items);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<cv::Ptr<cv::face::FisherFaceRecognizer>*> cv_face_FisherFaceRecognizer_create_int_double(int num_components, double threshold) {
		try {
			cv::Ptr<cv::face::FisherFaceRecognizer> ret = cv::face::FisherFaceRecognizer::create(num_components, threshold);
			return Ok(new cv::Ptr<cv::face::FisherFaceRecognizer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::face::FisherFaceRecognizer>*>))
	}
	
	Result<int> cv_face_LBPHFaceRecognizer_getGridX_const(const cv::face::LBPHFaceRecognizer* instance) {
		try {
			int ret = instance->getGridX();
			return Ok<int>(ret);
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
			return Ok<int>(ret);
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
			return Ok<int>(ret);
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
			return Ok<int>(ret);
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
			return Ok<double>(ret);
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
	
	Result<cv::Ptr<cv::face::LBPHFaceRecognizer>*> cv_face_LBPHFaceRecognizer_create_int_int_int_int_double(int radius, int neighbors, int grid_x, int grid_y, double threshold) {
		try {
			cv::Ptr<cv::face::LBPHFaceRecognizer> ret = cv::face::LBPHFaceRecognizer::create(radius, neighbors, grid_x, grid_y, threshold);
			return Ok(new cv::Ptr<cv::face::LBPHFaceRecognizer>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::face::LBPHFaceRecognizer>*>))
	}
	
	Result_void cv_face_MACE_salt_const_StringR(cv::face::MACE* instance, const char* passphrase) {
		try {
			instance->salt(std::string(passphrase));
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_face_MACE_train_const__InputArrayR(cv::face::MACE* instance, const cv::_InputArray* images) {
		try {
			instance->train(*images);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_face_MACE_same_const_const__InputArrayR(const cv::face::MACE* instance, const cv::_InputArray* query) {
		try {
			bool ret = instance->same(*query);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<cv::Ptr<cv::face::MACE>*> cv_face_MACE_load_const_StringR_const_StringR(const char* filename, const char* objname) {
		try {
			cv::Ptr<cv::face::MACE> ret = cv::face::MACE::load(std::string(filename), std::string(objname));
			return Ok(new cv::Ptr<cv::face::MACE>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::face::MACE>*>))
	}
	
	Result<cv::Ptr<cv::face::MACE>*> cv_face_MACE_create_int(int IMGSIZE) {
		try {
			cv::Ptr<cv::face::MACE> ret = cv::face::MACE::create(IMGSIZE);
			return Ok(new cv::Ptr<cv::face::MACE>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::face::MACE>*>))
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
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	void cv_StandardCollector_delete(cv::face::StandardCollector* instance) {
		delete instance;
	}
	Result<cv::face::StandardCollector*> cv_face_StandardCollector_StandardCollector_double(double threshold_) {
		try {
			cv::face::StandardCollector* ret = new cv::face::StandardCollector(threshold_);
			return Ok<cv::face::StandardCollector*>(ret);
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
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<int> cv_face_StandardCollector_getMinLabel_const(const cv::face::StandardCollector* instance) {
		try {
			int ret = instance->getMinLabel();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<double> cv_face_StandardCollector_getMinDist_const(const cv::face::StandardCollector* instance) {
		try {
			double ret = instance->getMinDist();
			return Ok<double>(ret);
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
			return Ok<cv::face::StandardCollector::PredictResult>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::face::StandardCollector::PredictResult>))
	}
	
}
