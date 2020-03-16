#include "face.hpp"
#include "face_types.hpp"

extern "C" {
	Result<cv::Ptr<cv::face::Facemark>*> cv_face_createFacemarkAAM() {
		try {
			cv::Ptr<cv::face::Facemark> ret = cv::face::createFacemarkAAM();
			return Ok(new cv::Ptr<cv::face::Facemark>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::face::Facemark>*>)
	}
	
	Result<cv::Ptr<cv::face::Facemark>*> cv_face_createFacemarkKazemi() {
		try {
			cv::Ptr<cv::face::Facemark> ret = cv::face::createFacemarkKazemi();
			return Ok(new cv::Ptr<cv::face::Facemark>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::face::Facemark>*>)
	}
	
	Result<cv::Ptr<cv::face::Facemark>*> cv_face_createFacemarkLBF() {
		try {
			cv::Ptr<cv::face::Facemark> ret = cv::face::createFacemarkLBF();
			return Ok(new cv::Ptr<cv::face::Facemark>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::face::Facemark>*>)
	}
	
	Result_void cv_face_drawFacemarks_const__InputOutputArrayX_const__InputArrayX_Scalar(const cv::_InputOutputArray* image, const cv::_InputArray* points, const cv::Scalar* color) {
		try {
			cv::face::drawFacemarks(*image, *points, *color);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_face_getFacesHAAR_const__InputArrayX_const__OutputArrayX_const_StringX(const cv::_InputArray* image, const cv::_OutputArray* faces, const char* face_cascade_name) {
		try {
			bool ret = cv::face::getFacesHAAR(*image, *faces, std::string(face_cascade_name));
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_face_getFaces_const__InputArrayX_const__OutputArrayX_CParamsX(const cv::_InputArray* image, const cv::_OutputArray* faces, cv::face::CParams* params) {
		try {
			bool ret = cv::face::getFaces(*image, *faces, params);
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_face_loadDatasetList_String_String_vector_String_X_vector_String_X(char* imageList, char* annotationList, std::vector<cv::String>* images, std::vector<cv::String>* annotations) {
		try {
			bool ret = cv::face::loadDatasetList(std::string(imageList), std::string(annotationList), *images, *annotations);
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_face_loadFacePoints_String_const__OutputArrayX_float(char* filename, const cv::_OutputArray* points, float offset) {
		try {
			bool ret = cv::face::loadFacePoints(std::string(filename), *points, offset);
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_face_loadTrainingData_String_String_vector_String_X_const__OutputArrayX_float(char* imageList, char* groundTruth, std::vector<cv::String>* images, const cv::_OutputArray* facePoints, float offset) {
		try {
			bool ret = cv::face::loadTrainingData(std::string(imageList), std::string(groundTruth), *images, *facePoints, offset);
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_face_loadTrainingData_String_vector_String_X_const__OutputArrayX_char_float(char* filename, std::vector<cv::String>* images, const cv::_OutputArray* facePoints, char delim, float offset) {
		try {
			bool ret = cv::face::loadTrainingData(std::string(filename), *images, *facePoints, delim, offset);
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_face_loadTrainingData_vector_String__vector_vector_Point2f__X_vector_String_X(std::vector<cv::String>* filename, std::vector<std::vector<cv::Point2f>>* trainlandmarks, std::vector<cv::String>* trainimages) {
		try {
			bool ret = cv::face::loadTrainingData(*filename, *trainlandmarks, *trainimages);
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<int> cv_face_BIF_getNumBands_const(const cv::face::BIF* instance) {
		try {
			int ret = instance->getNumBands();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_face_BIF_getNumRotations_const(const cv::face::BIF* instance) {
		try {
			int ret = instance->getNumRotations();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_face_BIF_compute_const_const__InputArrayX_const__OutputArrayX(const cv::face::BIF* instance, const cv::_InputArray* image, const cv::_OutputArray* features) {
		try {
			instance->compute(*image, *features);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Ptr<cv::face::BIF>*> cv_face_BIF_create_int_int(int num_bands, int num_rotations) {
		try {
			cv::Ptr<cv::face::BIF> ret = cv::face::BIF::create(num_bands, num_rotations);
			return Ok(new cv::Ptr<cv::face::BIF>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::face::BIF>*>)
	}
	
	Result<int> cv_face_BasicFaceRecognizer_getNumComponents_const(const cv::face::BasicFaceRecognizer* instance) {
		try {
			int ret = instance->getNumComponents();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_face_BasicFaceRecognizer_setNumComponents_int(cv::face::BasicFaceRecognizer* instance, int val) {
		try {
			instance->setNumComponents(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_face_BasicFaceRecognizer_getThreshold_const(const cv::face::BasicFaceRecognizer* instance) {
		try {
			double ret = instance->getThreshold();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_face_BasicFaceRecognizer_setThreshold_double(cv::face::BasicFaceRecognizer* instance, double val) {
		try {
			instance->setThreshold(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<std::vector<cv::Mat>*> cv_face_BasicFaceRecognizer_getProjections_const(const cv::face::BasicFaceRecognizer* instance) {
		try {
			std::vector<cv::Mat> ret = instance->getProjections();
			return Ok(new std::vector<cv::Mat>(ret));
		} OCVRS_CATCH(Result<std::vector<cv::Mat>*>)
	}
	
	Result<cv::Mat*> cv_face_BasicFaceRecognizer_getLabels_const(const cv::face::BasicFaceRecognizer* instance) {
		try {
			cv::Mat ret = instance->getLabels();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result<cv::Mat*> cv_face_BasicFaceRecognizer_getEigenValues_const(const cv::face::BasicFaceRecognizer* instance) {
		try {
			cv::Mat ret = instance->getEigenValues();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result<cv::Mat*> cv_face_BasicFaceRecognizer_getEigenVectors_const(const cv::face::BasicFaceRecognizer* instance) {
		try {
			cv::Mat ret = instance->getEigenVectors();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result<cv::Mat*> cv_face_BasicFaceRecognizer_getMean_const(const cv::face::BasicFaceRecognizer* instance) {
		try {
			cv::Mat ret = instance->getMean();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result_void cv_face_BasicFaceRecognizer_read_const_FileNodeX(cv::face::BasicFaceRecognizer* instance, const cv::FileNode* fn) {
		try {
			instance->read(*fn);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_face_BasicFaceRecognizer_write_const_FileStorageX(const cv::face::BasicFaceRecognizer* instance, cv::FileStorage* fs) {
		try {
			instance->write(*fs);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_face_BasicFaceRecognizer_empty_const(const cv::face::BasicFaceRecognizer* instance) {
		try {
			bool ret = instance->empty();
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<void*> cv_face_CParams_cascade_const(const cv::face::CParams* instance) {
		try {
			cv::String ret = instance->cascade;
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_face_CParams_setCascade_String(cv::face::CParams* instance, char* val) {
		try {
			instance->cascade = std::string(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_face_CParams_scaleFactor_const(const cv::face::CParams* instance) {
		try {
			double ret = instance->scaleFactor;
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_face_CParams_setScaleFactor_double(cv::face::CParams* instance, double val) {
		try {
			instance->scaleFactor = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_face_CParams_minNeighbors_const(const cv::face::CParams* instance) {
		try {
			int ret = instance->minNeighbors;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_face_CParams_setMinNeighbors_int(cv::face::CParams* instance, int val) {
		try {
			instance->minNeighbors = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Size> cv_face_CParams_minSize_const(const cv::face::CParams* instance) {
		try {
			cv::Size ret = instance->minSize;
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_face_CParams_setMinSize_Size(cv::face::CParams* instance, const cv::Size* val) {
		try {
			instance->minSize = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Size> cv_face_CParams_maxSize_const(const cv::face::CParams* instance) {
		try {
			cv::Size ret = instance->maxSize;
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_face_CParams_setMaxSize_Size(cv::face::CParams* instance, const cv::Size* val) {
		try {
			instance->maxSize = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::CascadeClassifier*> cv_face_CParams_face_cascade(cv::face::CParams* instance) {
		try {
			cv::CascadeClassifier ret = instance->face_cascade;
			return Ok(new cv::CascadeClassifier(ret));
		} OCVRS_CATCH(Result<cv::CascadeClassifier*>)
	}
	
	Result_void cv_face_CParams_setFace_cascade_CascadeClassifier(cv::face::CParams* instance, cv::CascadeClassifier* val) {
		try {
			instance->face_cascade = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_CParams_delete(cv::face::CParams* instance) {
		delete instance;
	}
	Result<cv::face::CParams*> cv_face_CParams_CParams_String_double_int_Size_Size(char* cascade_model, double sf, int minN, const cv::Size* minSz, const cv::Size* maxSz) {
		try {
			cv::face::CParams* ret = new cv::face::CParams(std::string(cascade_model), sf, minN, *minSz, *maxSz);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::face::CParams*>)
	}
	
	Result<cv::Ptr<cv::face::EigenFaceRecognizer>*> cv_face_EigenFaceRecognizer_create_int_double(int num_components, double threshold) {
		try {
			cv::Ptr<cv::face::EigenFaceRecognizer> ret = cv::face::EigenFaceRecognizer::create(num_components, threshold);
			return Ok(new cv::Ptr<cv::face::EigenFaceRecognizer>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::face::EigenFaceRecognizer>*>)
	}
	
	Result_void cv_face_FaceRecognizer_train_const__InputArrayX_const__InputArrayX(cv::face::FaceRecognizer* instance, const cv::_InputArray* src, const cv::_InputArray* labels) {
		try {
			instance->train(*src, *labels);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_face_FaceRecognizer_update_const__InputArrayX_const__InputArrayX(cv::face::FaceRecognizer* instance, const cv::_InputArray* src, const cv::_InputArray* labels) {
		try {
			instance->update(*src, *labels);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_face_FaceRecognizer_predict_const_const__InputArrayX(const cv::face::FaceRecognizer* instance, const cv::_InputArray* src) {
		try {
			int ret = instance->predict(*src);
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_face_FaceRecognizer_predict_const_const__InputArrayX_intX_doubleX(const cv::face::FaceRecognizer* instance, const cv::_InputArray* src, int* label, double* confidence) {
		try {
			instance->predict(*src, *label, *confidence);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_face_FaceRecognizer_predict_const_const__InputArrayX_Ptr_PredictCollector_(const cv::face::FaceRecognizer* instance, const cv::_InputArray* src, cv::Ptr<cv::face::PredictCollector>* collector) {
		try {
			instance->predict(*src, *collector);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_face_FaceRecognizer_write_const_const_StringX(const cv::face::FaceRecognizer* instance, const char* filename) {
		try {
			instance->write(std::string(filename));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_face_FaceRecognizer_read_const_StringX(cv::face::FaceRecognizer* instance, const char* filename) {
		try {
			instance->read(std::string(filename));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_face_FaceRecognizer_write_const_FileStorageX(const cv::face::FaceRecognizer* instance, cv::FileStorage* fs) {
		try {
			instance->write(*fs);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_face_FaceRecognizer_read_const_FileNodeX(cv::face::FaceRecognizer* instance, const cv::FileNode* fn) {
		try {
			instance->read(*fn);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_face_FaceRecognizer_empty_const(const cv::face::FaceRecognizer* instance) {
		try {
			bool ret = instance->empty();
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_face_FaceRecognizer_setLabelInfo_int_const_StringX(cv::face::FaceRecognizer* instance, int label, const char* strInfo) {
		try {
			instance->setLabelInfo(label, std::string(strInfo));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_face_FaceRecognizer_getLabelInfo_const_int(const cv::face::FaceRecognizer* instance, int label) {
		try {
			cv::String ret = instance->getLabelInfo(label);
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<std::vector<int>*> cv_face_FaceRecognizer_getLabelsByString_const_const_StringX(const cv::face::FaceRecognizer* instance, const char* str) {
		try {
			std::vector<int> ret = instance->getLabelsByString(std::string(str));
			return Ok(new std::vector<int>(ret));
		} OCVRS_CATCH(Result<std::vector<int>*>)
	}
	
	Result<double> cv_face_FaceRecognizer_getThreshold_const(const cv::face::FaceRecognizer* instance) {
		try {
			double ret = instance->getThreshold();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_face_FaceRecognizer_setThreshold_double(cv::face::FaceRecognizer* instance, double val) {
		try {
			instance->setThreshold(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_face_Facemark_loadModel_String(cv::face::Facemark* instance, char* model) {
		try {
			instance->loadModel(std::string(model));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_face_Facemark_fit_const__InputArrayX_const__InputArrayX_const__OutputArrayX(cv::face::Facemark* instance, const cv::_InputArray* image, const cv::_InputArray* faces, const cv::_OutputArray* landmarks) {
		try {
			bool ret = instance->fit(*image, *faces, *landmarks);
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_face_FacemarkAAM_fitConfig_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const_vector_Config_X(cv::face::FacemarkAAM* instance, const cv::_InputArray* image, const cv::_InputArray* roi, const cv::_OutputArray* _landmarks, const std::vector<cv::face::FacemarkAAM::Config>* runtime_params) {
		try {
			bool ret = instance->fitConfig(*image, *roi, *_landmarks, *runtime_params);
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<cv::Ptr<cv::face::FacemarkAAM>*> cv_face_FacemarkAAM_create_const_ParamsX(const cv::face::FacemarkAAM::Params* parameters) {
		try {
			cv::Ptr<cv::face::FacemarkAAM> ret = cv::face::FacemarkAAM::create(*parameters);
			return Ok(new cv::Ptr<cv::face::FacemarkAAM>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::face::FacemarkAAM>*>)
	}
	
	Result<cv::Mat*> cv_face_FacemarkAAM_Config_R(cv::face::FacemarkAAM::Config* instance) {
		try {
			cv::Mat ret = instance->R;
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result_void cv_face_FacemarkAAM_Config_setR_Mat(cv::face::FacemarkAAM::Config* instance, cv::Mat* val) {
		try {
			instance->R = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Point2f> cv_face_FacemarkAAM_Config_t_const(const cv::face::FacemarkAAM::Config* instance) {
		try {
			cv::Point2f ret = instance->t;
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Point2f>)
	}
	
	Result_void cv_face_FacemarkAAM_Config_setT_Point2f(cv::face::FacemarkAAM::Config* instance, const cv::Point2f* val) {
		try {
			instance->t = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_face_FacemarkAAM_Config_scale_const(const cv::face::FacemarkAAM::Config* instance) {
		try {
			float ret = instance->scale;
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_face_FacemarkAAM_Config_setScale_float(cv::face::FacemarkAAM::Config* instance, float val) {
		try {
			instance->scale = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_face_FacemarkAAM_Config_model_scale_idx_const(const cv::face::FacemarkAAM::Config* instance) {
		try {
			int ret = instance->model_scale_idx;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_face_FacemarkAAM_Config_setModel_scale_idx_int(cv::face::FacemarkAAM::Config* instance, int val) {
		try {
			instance->model_scale_idx = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_FacemarkAAM_Config_delete(cv::face::FacemarkAAM::Config* instance) {
		delete instance;
	}
	Result<cv::face::FacemarkAAM::Config*> cv_face_FacemarkAAM_Config_Config_Mat_Point2f_float_int(cv::Mat* rot, const cv::Point2f* trans, float scaling, int scale_id) {
		try {
			cv::face::FacemarkAAM::Config* ret = new cv::face::FacemarkAAM::Config(*rot, *trans, scaling, scale_id);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::face::FacemarkAAM::Config*>)
	}
	
	Result<std::vector<cv::Point2f>*> cv_face_FacemarkAAM_Data_s0(cv::face::FacemarkAAM::Data* instance) {
		try {
			std::vector<cv::Point2f> ret = instance->s0;
			return Ok(new std::vector<cv::Point2f>(ret));
		} OCVRS_CATCH(Result<std::vector<cv::Point2f>*>)
	}
	
	Result_void cv_face_FacemarkAAM_Data_setS0_vector_Point2f_(cv::face::FacemarkAAM::Data* instance, std::vector<cv::Point2f>* val) {
		try {
			instance->s0 = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_FacemarkAAM_Data_delete(cv::face::FacemarkAAM::Data* instance) {
		delete instance;
	}
	Result<std::vector<float>*> cv_face_FacemarkAAM_Model_scales(cv::face::FacemarkAAM::Model* instance) {
		try {
			std::vector<float> ret = instance->scales;
			return Ok(new std::vector<float>(ret));
		} OCVRS_CATCH(Result<std::vector<float>*>)
	}
	
	Result_void cv_face_FacemarkAAM_Model_setScales_vector_float_(cv::face::FacemarkAAM::Model* instance, std::vector<float>* val) {
		try {
			instance->scales = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<std::vector<cv::Vec3i>*> cv_face_FacemarkAAM_Model_triangles(cv::face::FacemarkAAM::Model* instance) {
		try {
			std::vector<cv::Vec3i> ret = instance->triangles;
			return Ok(new std::vector<cv::Vec3i>(ret));
		} OCVRS_CATCH(Result<std::vector<cv::Vec3i>*>)
	}
	
	Result_void cv_face_FacemarkAAM_Model_setTriangles_vector_Vec3i_(cv::face::FacemarkAAM::Model* instance, std::vector<cv::Vec3i>* val) {
		try {
			instance->triangles = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<std::vector<cv::face::FacemarkAAM::Model::Texture>*> cv_face_FacemarkAAM_Model_textures(cv::face::FacemarkAAM::Model* instance) {
		try {
			std::vector<cv::face::FacemarkAAM::Model::Texture> ret = instance->textures;
			return Ok(new std::vector<cv::face::FacemarkAAM::Model::Texture>(ret));
		} OCVRS_CATCH(Result<std::vector<cv::face::FacemarkAAM::Model::Texture>*>)
	}
	
	Result_void cv_face_FacemarkAAM_Model_setTextures_vector_Texture_(cv::face::FacemarkAAM::Model* instance, std::vector<cv::face::FacemarkAAM::Model::Texture>* val) {
		try {
			instance->textures = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<std::vector<cv::Point2f>*> cv_face_FacemarkAAM_Model_s0(cv::face::FacemarkAAM::Model* instance) {
		try {
			std::vector<cv::Point2f> ret = instance->s0;
			return Ok(new std::vector<cv::Point2f>(ret));
		} OCVRS_CATCH(Result<std::vector<cv::Point2f>*>)
	}
	
	Result_void cv_face_FacemarkAAM_Model_setS0_vector_Point2f_(cv::face::FacemarkAAM::Model* instance, std::vector<cv::Point2f>* val) {
		try {
			instance->s0 = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Mat*> cv_face_FacemarkAAM_Model_S(cv::face::FacemarkAAM::Model* instance) {
		try {
			cv::Mat ret = instance->S;
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result_void cv_face_FacemarkAAM_Model_setS_Mat(cv::face::FacemarkAAM::Model* instance, cv::Mat* val) {
		try {
			instance->S = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Mat*> cv_face_FacemarkAAM_Model_Q(cv::face::FacemarkAAM::Model* instance) {
		try {
			cv::Mat ret = instance->Q;
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result_void cv_face_FacemarkAAM_Model_setQ_Mat(cv::face::FacemarkAAM::Model* instance, cv::Mat* val) {
		try {
			instance->Q = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_FacemarkAAM_Model_delete(cv::face::FacemarkAAM::Model* instance) {
		delete instance;
	}
	Result<int> cv_face_FacemarkAAM_Model_Texture_max_m_const(const cv::face::FacemarkAAM::Model::Texture* instance) {
		try {
			int ret = instance->max_m;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_face_FacemarkAAM_Model_Texture_setMax_m_int(cv::face::FacemarkAAM::Model::Texture* instance, int val) {
		try {
			instance->max_m = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Rect> cv_face_FacemarkAAM_Model_Texture_resolution_const(const cv::face::FacemarkAAM::Model::Texture* instance) {
		try {
			cv::Rect ret = instance->resolution;
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Rect>)
	}
	
	Result_void cv_face_FacemarkAAM_Model_Texture_setResolution_Rect(cv::face::FacemarkAAM::Model::Texture* instance, const cv::Rect* val) {
		try {
			instance->resolution = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Mat*> cv_face_FacemarkAAM_Model_Texture_A(cv::face::FacemarkAAM::Model::Texture* instance) {
		try {
			cv::Mat ret = instance->A;
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result_void cv_face_FacemarkAAM_Model_Texture_setA_Mat(cv::face::FacemarkAAM::Model::Texture* instance, cv::Mat* val) {
		try {
			instance->A = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Mat*> cv_face_FacemarkAAM_Model_Texture_A0(cv::face::FacemarkAAM::Model::Texture* instance) {
		try {
			cv::Mat ret = instance->A0;
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result_void cv_face_FacemarkAAM_Model_Texture_setA0_Mat(cv::face::FacemarkAAM::Model::Texture* instance, cv::Mat* val) {
		try {
			instance->A0 = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Mat*> cv_face_FacemarkAAM_Model_Texture_AA(cv::face::FacemarkAAM::Model::Texture* instance) {
		try {
			cv::Mat ret = instance->AA;
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result_void cv_face_FacemarkAAM_Model_Texture_setAA_Mat(cv::face::FacemarkAAM::Model::Texture* instance, cv::Mat* val) {
		try {
			instance->AA = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Mat*> cv_face_FacemarkAAM_Model_Texture_AA0(cv::face::FacemarkAAM::Model::Texture* instance) {
		try {
			cv::Mat ret = instance->AA0;
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result_void cv_face_FacemarkAAM_Model_Texture_setAA0_Mat(cv::face::FacemarkAAM::Model::Texture* instance, cv::Mat* val) {
		try {
			instance->AA0 = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<std::vector<std::vector<cv::Point>>*> cv_face_FacemarkAAM_Model_Texture_textureIdx(cv::face::FacemarkAAM::Model::Texture* instance) {
		try {
			std::vector<std::vector<cv::Point>> ret = instance->textureIdx;
			return Ok(new std::vector<std::vector<cv::Point>>(ret));
		} OCVRS_CATCH(Result<std::vector<std::vector<cv::Point>>*>)
	}
	
	Result_void cv_face_FacemarkAAM_Model_Texture_setTextureIdx_vector_vector_Point__(cv::face::FacemarkAAM::Model::Texture* instance, std::vector<std::vector<cv::Point>>* val) {
		try {
			instance->textureIdx = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<std::vector<cv::Point2f>*> cv_face_FacemarkAAM_Model_Texture_base_shape(cv::face::FacemarkAAM::Model::Texture* instance) {
		try {
			std::vector<cv::Point2f> ret = instance->base_shape;
			return Ok(new std::vector<cv::Point2f>(ret));
		} OCVRS_CATCH(Result<std::vector<cv::Point2f>*>)
	}
	
	Result_void cv_face_FacemarkAAM_Model_Texture_setBase_shape_vector_Point2f_(cv::face::FacemarkAAM::Model::Texture* instance, std::vector<cv::Point2f>* val) {
		try {
			instance->base_shape = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<std::vector<int>*> cv_face_FacemarkAAM_Model_Texture_ind1(cv::face::FacemarkAAM::Model::Texture* instance) {
		try {
			std::vector<int> ret = instance->ind1;
			return Ok(new std::vector<int>(ret));
		} OCVRS_CATCH(Result<std::vector<int>*>)
	}
	
	Result_void cv_face_FacemarkAAM_Model_Texture_setInd1_vector_int_(cv::face::FacemarkAAM::Model::Texture* instance, std::vector<int>* val) {
		try {
			instance->ind1 = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<std::vector<int>*> cv_face_FacemarkAAM_Model_Texture_ind2(cv::face::FacemarkAAM::Model::Texture* instance) {
		try {
			std::vector<int> ret = instance->ind2;
			return Ok(new std::vector<int>(ret));
		} OCVRS_CATCH(Result<std::vector<int>*>)
	}
	
	Result_void cv_face_FacemarkAAM_Model_Texture_setInd2_vector_int_(cv::face::FacemarkAAM::Model::Texture* instance, std::vector<int>* val) {
		try {
			instance->ind2 = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_FacemarkAAM_Model_Texture_delete(cv::face::FacemarkAAM::Model::Texture* instance) {
		delete instance;
	}
	Result<void*> cv_face_FacemarkAAM_Params_model_filename_const(const cv::face::FacemarkAAM::Params* instance) {
		try {
			std::string ret = instance->model_filename;
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_face_FacemarkAAM_Params_setModel_filename_string(cv::face::FacemarkAAM::Params* instance, char* val) {
		try {
			instance->model_filename = std::string(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_face_FacemarkAAM_Params_m_const(const cv::face::FacemarkAAM::Params* instance) {
		try {
			int ret = instance->m;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_face_FacemarkAAM_Params_setM_int(cv::face::FacemarkAAM::Params* instance, int val) {
		try {
			instance->m = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_face_FacemarkAAM_Params_n_const(const cv::face::FacemarkAAM::Params* instance) {
		try {
			int ret = instance->n;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_face_FacemarkAAM_Params_setN_int(cv::face::FacemarkAAM::Params* instance, int val) {
		try {
			instance->n = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_face_FacemarkAAM_Params_n_iter_const(const cv::face::FacemarkAAM::Params* instance) {
		try {
			int ret = instance->n_iter;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_face_FacemarkAAM_Params_setN_iter_int(cv::face::FacemarkAAM::Params* instance, int val) {
		try {
			instance->n_iter = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_face_FacemarkAAM_Params_verbose_const(const cv::face::FacemarkAAM::Params* instance) {
		try {
			bool ret = instance->verbose;
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_face_FacemarkAAM_Params_setVerbose_bool(cv::face::FacemarkAAM::Params* instance, bool val) {
		try {
			instance->verbose = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_face_FacemarkAAM_Params_save_model_const(const cv::face::FacemarkAAM::Params* instance) {
		try {
			bool ret = instance->save_model;
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_face_FacemarkAAM_Params_setSave_model_bool(cv::face::FacemarkAAM::Params* instance, bool val) {
		try {
			instance->save_model = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_face_FacemarkAAM_Params_max_m_const(const cv::face::FacemarkAAM::Params* instance) {
		try {
			int ret = instance->max_m;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_face_FacemarkAAM_Params_setMax_m_int(cv::face::FacemarkAAM::Params* instance, int val) {
		try {
			instance->max_m = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_face_FacemarkAAM_Params_max_n_const(const cv::face::FacemarkAAM::Params* instance) {
		try {
			int ret = instance->max_n;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_face_FacemarkAAM_Params_setMax_n_int(cv::face::FacemarkAAM::Params* instance, int val) {
		try {
			instance->max_n = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_face_FacemarkAAM_Params_texture_max_m_const(const cv::face::FacemarkAAM::Params* instance) {
		try {
			int ret = instance->texture_max_m;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_face_FacemarkAAM_Params_setTexture_max_m_int(cv::face::FacemarkAAM::Params* instance, int val) {
		try {
			instance->texture_max_m = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<std::vector<float>*> cv_face_FacemarkAAM_Params_scales(cv::face::FacemarkAAM::Params* instance) {
		try {
			std::vector<float> ret = instance->scales;
			return Ok(new std::vector<float>(ret));
		} OCVRS_CATCH(Result<std::vector<float>*>)
	}
	
	Result_void cv_face_FacemarkAAM_Params_setScales_vector_float_(cv::face::FacemarkAAM::Params* instance, std::vector<float>* val) {
		try {
			instance->scales = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_FacemarkAAM_Params_delete(cv::face::FacemarkAAM::Params* instance) {
		delete instance;
	}
	Result<cv::face::FacemarkAAM::Params*> cv_face_FacemarkAAM_Params_Params() {
		try {
			cv::face::FacemarkAAM::Params* ret = new cv::face::FacemarkAAM::Params();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::face::FacemarkAAM::Params*>)
	}
	
	Result_void cv_face_FacemarkAAM_Params_read_const_FileNodeX(cv::face::FacemarkAAM::Params* instance, const cv::FileNode* unnamed) {
		try {
			instance->read(*unnamed);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_face_FacemarkAAM_Params_write_const_FileStorageX(const cv::face::FacemarkAAM::Params* instance, cv::FileStorage* unnamed) {
		try {
			instance->write(*unnamed);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Ptr<cv::face::FacemarkKazemi>*> cv_face_FacemarkKazemi_create_const_ParamsX(const cv::face::FacemarkKazemi::Params* parameters) {
		try {
			cv::Ptr<cv::face::FacemarkKazemi> ret = cv::face::FacemarkKazemi::create(*parameters);
			return Ok(new cv::Ptr<cv::face::FacemarkKazemi>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::face::FacemarkKazemi>*>)
	}
	
	Result<bool> cv_face_FacemarkKazemi_training_vector_Mat_X_vector_vector_Point2f__X_string_Size_string(cv::face::FacemarkKazemi* instance, std::vector<cv::Mat>* images, std::vector<std::vector<cv::Point2f>>* landmarks, char* configfile, const cv::Size* scale, char* modelFilename) {
		try {
			bool ret = instance->training(*images, *landmarks, std::string(configfile), *scale, std::string(modelFilename));
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_face_FacemarkKazemi_getFaces_const__InputArrayX_const__OutputArrayX(cv::face::FacemarkKazemi* instance, const cv::_InputArray* image, const cv::_OutputArray* faces) {
		try {
			bool ret = instance->getFaces(*image, *faces);
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<unsigned long> cv_face_FacemarkKazemi_Params_cascade_depth_const(const cv::face::FacemarkKazemi::Params* instance) {
		try {
			unsigned long ret = instance->cascade_depth;
			return Ok(ret);
		} OCVRS_CATCH(Result<unsigned long>)
	}
	
	Result_void cv_face_FacemarkKazemi_Params_setCascade_depth_unsigned_long(cv::face::FacemarkKazemi::Params* instance, unsigned long val) {
		try {
			instance->cascade_depth = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<unsigned long> cv_face_FacemarkKazemi_Params_tree_depth_const(const cv::face::FacemarkKazemi::Params* instance) {
		try {
			unsigned long ret = instance->tree_depth;
			return Ok(ret);
		} OCVRS_CATCH(Result<unsigned long>)
	}
	
	Result_void cv_face_FacemarkKazemi_Params_setTree_depth_unsigned_long(cv::face::FacemarkKazemi::Params* instance, unsigned long val) {
		try {
			instance->tree_depth = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<unsigned long> cv_face_FacemarkKazemi_Params_num_trees_per_cascade_level_const(const cv::face::FacemarkKazemi::Params* instance) {
		try {
			unsigned long ret = instance->num_trees_per_cascade_level;
			return Ok(ret);
		} OCVRS_CATCH(Result<unsigned long>)
	}
	
	Result_void cv_face_FacemarkKazemi_Params_setNum_trees_per_cascade_level_unsigned_long(cv::face::FacemarkKazemi::Params* instance, unsigned long val) {
		try {
			instance->num_trees_per_cascade_level = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_face_FacemarkKazemi_Params_learning_rate_const(const cv::face::FacemarkKazemi::Params* instance) {
		try {
			float ret = instance->learning_rate;
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_face_FacemarkKazemi_Params_setLearning_rate_float(cv::face::FacemarkKazemi::Params* instance, float val) {
		try {
			instance->learning_rate = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<unsigned long> cv_face_FacemarkKazemi_Params_oversampling_amount_const(const cv::face::FacemarkKazemi::Params* instance) {
		try {
			unsigned long ret = instance->oversampling_amount;
			return Ok(ret);
		} OCVRS_CATCH(Result<unsigned long>)
	}
	
	Result_void cv_face_FacemarkKazemi_Params_setOversampling_amount_unsigned_long(cv::face::FacemarkKazemi::Params* instance, unsigned long val) {
		try {
			instance->oversampling_amount = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<unsigned long> cv_face_FacemarkKazemi_Params_num_test_coordinates_const(const cv::face::FacemarkKazemi::Params* instance) {
		try {
			unsigned long ret = instance->num_test_coordinates;
			return Ok(ret);
		} OCVRS_CATCH(Result<unsigned long>)
	}
	
	Result_void cv_face_FacemarkKazemi_Params_setNum_test_coordinates_unsigned_long(cv::face::FacemarkKazemi::Params* instance, unsigned long val) {
		try {
			instance->num_test_coordinates = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_face_FacemarkKazemi_Params_lambda_const(const cv::face::FacemarkKazemi::Params* instance) {
		try {
			float ret = instance->lambda;
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_face_FacemarkKazemi_Params_setLambda_float(cv::face::FacemarkKazemi::Params* instance, float val) {
		try {
			instance->lambda = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<unsigned long> cv_face_FacemarkKazemi_Params_num_test_splits_const(const cv::face::FacemarkKazemi::Params* instance) {
		try {
			unsigned long ret = instance->num_test_splits;
			return Ok(ret);
		} OCVRS_CATCH(Result<unsigned long>)
	}
	
	Result_void cv_face_FacemarkKazemi_Params_setNum_test_splits_unsigned_long(cv::face::FacemarkKazemi::Params* instance, unsigned long val) {
		try {
			instance->num_test_splits = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_face_FacemarkKazemi_Params_configfile_const(const cv::face::FacemarkKazemi::Params* instance) {
		try {
			cv::String ret = instance->configfile;
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_face_FacemarkKazemi_Params_setConfigfile_String(cv::face::FacemarkKazemi::Params* instance, char* val) {
		try {
			instance->configfile = std::string(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_FacemarkKazemi_Params_delete(cv::face::FacemarkKazemi::Params* instance) {
		delete instance;
	}
	Result<cv::face::FacemarkKazemi::Params*> cv_face_FacemarkKazemi_Params_Params() {
		try {
			cv::face::FacemarkKazemi::Params* ret = new cv::face::FacemarkKazemi::Params();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::face::FacemarkKazemi::Params*>)
	}
	
	Result<cv::Ptr<cv::face::FacemarkLBF>*> cv_face_FacemarkLBF_create_const_ParamsX(const cv::face::FacemarkLBF::Params* parameters) {
		try {
			cv::Ptr<cv::face::FacemarkLBF> ret = cv::face::FacemarkLBF::create(*parameters);
			return Ok(new cv::Ptr<cv::face::FacemarkLBF>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::face::FacemarkLBF>*>)
	}
	
	Result<double> cv_face_FacemarkLBF_Params_shape_offset_const(const cv::face::FacemarkLBF::Params* instance) {
		try {
			double ret = instance->shape_offset;
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_face_FacemarkLBF_Params_setShape_offset_double(cv::face::FacemarkLBF::Params* instance, double val) {
		try {
			instance->shape_offset = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_face_FacemarkLBF_Params_cascade_face_const(const cv::face::FacemarkLBF::Params* instance) {
		try {
			cv::String ret = instance->cascade_face;
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_face_FacemarkLBF_Params_setCascade_face_String(cv::face::FacemarkLBF::Params* instance, char* val) {
		try {
			instance->cascade_face = std::string(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_face_FacemarkLBF_Params_verbose_const(const cv::face::FacemarkLBF::Params* instance) {
		try {
			bool ret = instance->verbose;
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_face_FacemarkLBF_Params_setVerbose_bool(cv::face::FacemarkLBF::Params* instance, bool val) {
		try {
			instance->verbose = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_face_FacemarkLBF_Params_n_landmarks_const(const cv::face::FacemarkLBF::Params* instance) {
		try {
			int ret = instance->n_landmarks;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_face_FacemarkLBF_Params_setN_landmarks_int(cv::face::FacemarkLBF::Params* instance, int val) {
		try {
			instance->n_landmarks = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_face_FacemarkLBF_Params_initShape_n_const(const cv::face::FacemarkLBF::Params* instance) {
		try {
			int ret = instance->initShape_n;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_face_FacemarkLBF_Params_setInitShape_n_int(cv::face::FacemarkLBF::Params* instance, int val) {
		try {
			instance->initShape_n = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_face_FacemarkLBF_Params_stages_n_const(const cv::face::FacemarkLBF::Params* instance) {
		try {
			int ret = instance->stages_n;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_face_FacemarkLBF_Params_setStages_n_int(cv::face::FacemarkLBF::Params* instance, int val) {
		try {
			instance->stages_n = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_face_FacemarkLBF_Params_tree_n_const(const cv::face::FacemarkLBF::Params* instance) {
		try {
			int ret = instance->tree_n;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_face_FacemarkLBF_Params_setTree_n_int(cv::face::FacemarkLBF::Params* instance, int val) {
		try {
			instance->tree_n = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_face_FacemarkLBF_Params_tree_depth_const(const cv::face::FacemarkLBF::Params* instance) {
		try {
			int ret = instance->tree_depth;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_face_FacemarkLBF_Params_setTree_depth_int(cv::face::FacemarkLBF::Params* instance, int val) {
		try {
			instance->tree_depth = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_face_FacemarkLBF_Params_bagging_overlap_const(const cv::face::FacemarkLBF::Params* instance) {
		try {
			double ret = instance->bagging_overlap;
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_face_FacemarkLBF_Params_setBagging_overlap_double(cv::face::FacemarkLBF::Params* instance, double val) {
		try {
			instance->bagging_overlap = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_face_FacemarkLBF_Params_model_filename_const(const cv::face::FacemarkLBF::Params* instance) {
		try {
			std::string ret = instance->model_filename;
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_face_FacemarkLBF_Params_setModel_filename_string(cv::face::FacemarkLBF::Params* instance, char* val) {
		try {
			instance->model_filename = std::string(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_face_FacemarkLBF_Params_save_model_const(const cv::face::FacemarkLBF::Params* instance) {
		try {
			bool ret = instance->save_model;
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_face_FacemarkLBF_Params_setSave_model_bool(cv::face::FacemarkLBF::Params* instance, bool val) {
		try {
			instance->save_model = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<unsigned int> cv_face_FacemarkLBF_Params_seed_const(const cv::face::FacemarkLBF::Params* instance) {
		try {
			unsigned int ret = instance->seed;
			return Ok(ret);
		} OCVRS_CATCH(Result<unsigned int>)
	}
	
	Result_void cv_face_FacemarkLBF_Params_setSeed_unsigned_int(cv::face::FacemarkLBF::Params* instance, unsigned int val) {
		try {
			instance->seed = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<std::vector<int>*> cv_face_FacemarkLBF_Params_feats_m(cv::face::FacemarkLBF::Params* instance) {
		try {
			std::vector<int> ret = instance->feats_m;
			return Ok(new std::vector<int>(ret));
		} OCVRS_CATCH(Result<std::vector<int>*>)
	}
	
	Result_void cv_face_FacemarkLBF_Params_setFeats_m_vector_int_(cv::face::FacemarkLBF::Params* instance, std::vector<int>* val) {
		try {
			instance->feats_m = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<std::vector<double>*> cv_face_FacemarkLBF_Params_radius_m(cv::face::FacemarkLBF::Params* instance) {
		try {
			std::vector<double> ret = instance->radius_m;
			return Ok(new std::vector<double>(ret));
		} OCVRS_CATCH(Result<std::vector<double>*>)
	}
	
	Result_void cv_face_FacemarkLBF_Params_setRadius_m_vector_double_(cv::face::FacemarkLBF::Params* instance, std::vector<double>* val) {
		try {
			instance->radius_m = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Rect> cv_face_FacemarkLBF_Params_detectROI_const(const cv::face::FacemarkLBF::Params* instance) {
		try {
			cv::Rect ret = instance->detectROI;
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Rect>)
	}
	
	Result_void cv_face_FacemarkLBF_Params_setDetectROI_Rect(cv::face::FacemarkLBF::Params* instance, const cv::Rect* val) {
		try {
			instance->detectROI = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_FacemarkLBF_Params_delete(cv::face::FacemarkLBF::Params* instance) {
		delete instance;
	}
	Result<cv::face::FacemarkLBF::Params*> cv_face_FacemarkLBF_Params_Params() {
		try {
			cv::face::FacemarkLBF::Params* ret = new cv::face::FacemarkLBF::Params();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::face::FacemarkLBF::Params*>)
	}
	
	Result_void cv_face_FacemarkLBF_Params_read_const_FileNodeX(cv::face::FacemarkLBF::Params* instance, const cv::FileNode* unnamed) {
		try {
			instance->read(*unnamed);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_face_FacemarkLBF_Params_write_const_FileStorageX(const cv::face::FacemarkLBF::Params* instance, cv::FileStorage* unnamed) {
		try {
			instance->write(*unnamed);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_face_FacemarkTrain_addTrainingSample_const__InputArrayX_const__InputArrayX(cv::face::FacemarkTrain* instance, const cv::_InputArray* image, const cv::_InputArray* landmarks) {
		try {
			bool ret = instance->addTrainingSample(*image, *landmarks);
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_face_FacemarkTrain_training_voidX(cv::face::FacemarkTrain* instance, void* parameters) {
		try {
			instance->training(parameters);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_face_FacemarkTrain_setFaceDetector_FN_FaceDetector_voidX(cv::face::FacemarkTrain* instance, cv::face::FN_FaceDetector detector, void* userData) {
		try {
			bool ret = instance->setFaceDetector(detector, userData);
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_face_FacemarkTrain_getFaces_const__InputArrayX_const__OutputArrayX(cv::face::FacemarkTrain* instance, const cv::_InputArray* image, const cv::_OutputArray* faces) {
		try {
			bool ret = instance->getFaces(*image, *faces);
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_face_FacemarkTrain_getData_voidX(cv::face::FacemarkTrain* instance, void* items) {
		try {
			bool ret = instance->getData(items);
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<cv::Ptr<cv::face::FisherFaceRecognizer>*> cv_face_FisherFaceRecognizer_create_int_double(int num_components, double threshold) {
		try {
			cv::Ptr<cv::face::FisherFaceRecognizer> ret = cv::face::FisherFaceRecognizer::create(num_components, threshold);
			return Ok(new cv::Ptr<cv::face::FisherFaceRecognizer>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::face::FisherFaceRecognizer>*>)
	}
	
	Result<int> cv_face_LBPHFaceRecognizer_getGridX_const(const cv::face::LBPHFaceRecognizer* instance) {
		try {
			int ret = instance->getGridX();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_face_LBPHFaceRecognizer_setGridX_int(cv::face::LBPHFaceRecognizer* instance, int val) {
		try {
			instance->setGridX(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_face_LBPHFaceRecognizer_getGridY_const(const cv::face::LBPHFaceRecognizer* instance) {
		try {
			int ret = instance->getGridY();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_face_LBPHFaceRecognizer_setGridY_int(cv::face::LBPHFaceRecognizer* instance, int val) {
		try {
			instance->setGridY(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_face_LBPHFaceRecognizer_getRadius_const(const cv::face::LBPHFaceRecognizer* instance) {
		try {
			int ret = instance->getRadius();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_face_LBPHFaceRecognizer_setRadius_int(cv::face::LBPHFaceRecognizer* instance, int val) {
		try {
			instance->setRadius(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_face_LBPHFaceRecognizer_getNeighbors_const(const cv::face::LBPHFaceRecognizer* instance) {
		try {
			int ret = instance->getNeighbors();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_face_LBPHFaceRecognizer_setNeighbors_int(cv::face::LBPHFaceRecognizer* instance, int val) {
		try {
			instance->setNeighbors(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_face_LBPHFaceRecognizer_getThreshold_const(const cv::face::LBPHFaceRecognizer* instance) {
		try {
			double ret = instance->getThreshold();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_face_LBPHFaceRecognizer_setThreshold_double(cv::face::LBPHFaceRecognizer* instance, double val) {
		try {
			instance->setThreshold(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<std::vector<cv::Mat>*> cv_face_LBPHFaceRecognizer_getHistograms_const(const cv::face::LBPHFaceRecognizer* instance) {
		try {
			std::vector<cv::Mat> ret = instance->getHistograms();
			return Ok(new std::vector<cv::Mat>(ret));
		} OCVRS_CATCH(Result<std::vector<cv::Mat>*>)
	}
	
	Result<cv::Mat*> cv_face_LBPHFaceRecognizer_getLabels_const(const cv::face::LBPHFaceRecognizer* instance) {
		try {
			cv::Mat ret = instance->getLabels();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result<cv::Ptr<cv::face::LBPHFaceRecognizer>*> cv_face_LBPHFaceRecognizer_create_int_int_int_int_double(int radius, int neighbors, int grid_x, int grid_y, double threshold) {
		try {
			cv::Ptr<cv::face::LBPHFaceRecognizer> ret = cv::face::LBPHFaceRecognizer::create(radius, neighbors, grid_x, grid_y, threshold);
			return Ok(new cv::Ptr<cv::face::LBPHFaceRecognizer>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::face::LBPHFaceRecognizer>*>)
	}
	
	Result_void cv_face_MACE_salt_const_StringX(cv::face::MACE* instance, const char* passphrase) {
		try {
			instance->salt(std::string(passphrase));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_face_MACE_train_const__InputArrayX(cv::face::MACE* instance, const cv::_InputArray* images) {
		try {
			instance->train(*images);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_face_MACE_same_const_const__InputArrayX(const cv::face::MACE* instance, const cv::_InputArray* query) {
		try {
			bool ret = instance->same(*query);
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<cv::Ptr<cv::face::MACE>*> cv_face_MACE_load_const_StringX_const_StringX(const char* filename, const char* objname) {
		try {
			cv::Ptr<cv::face::MACE> ret = cv::face::MACE::load(std::string(filename), std::string(objname));
			return Ok(new cv::Ptr<cv::face::MACE>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::face::MACE>*>)
	}
	
	Result<cv::Ptr<cv::face::MACE>*> cv_face_MACE_create_int(int IMGSIZE) {
		try {
			cv::Ptr<cv::face::MACE> ret = cv::face::MACE::create(IMGSIZE);
			return Ok(new cv::Ptr<cv::face::MACE>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::face::MACE>*>)
	}
	
	Result_void cv_face_PredictCollector_init_size_t(cv::face::PredictCollector* instance, size_t size) {
		try {
			instance->init(size);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_face_PredictCollector_collect_int_double(cv::face::PredictCollector* instance, int label, double dist) {
		try {
			bool ret = instance->collect(label, dist);
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	void cv_StandardCollector_delete(cv::face::StandardCollector* instance) {
		delete instance;
	}
	Result<cv::face::StandardCollector*> cv_face_StandardCollector_StandardCollector_double(double threshold_) {
		try {
			cv::face::StandardCollector* ret = new cv::face::StandardCollector(threshold_);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::face::StandardCollector*>)
	}
	
	Result_void cv_face_StandardCollector_init_size_t(cv::face::StandardCollector* instance, size_t size) {
		try {
			instance->init(size);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_face_StandardCollector_collect_int_double(cv::face::StandardCollector* instance, int label, double dist) {
		try {
			bool ret = instance->collect(label, dist);
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<int> cv_face_StandardCollector_getMinLabel_const(const cv::face::StandardCollector* instance) {
		try {
			int ret = instance->getMinLabel();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<double> cv_face_StandardCollector_getMinDist_const(const cv::face::StandardCollector* instance) {
		try {
			double ret = instance->getMinDist();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result<cv::Ptr<cv::face::StandardCollector>*> cv_face_StandardCollector_create_double(double threshold) {
		try {
			cv::Ptr<cv::face::StandardCollector> ret = cv::face::StandardCollector::create(threshold);
			return Ok(new cv::Ptr<cv::face::StandardCollector>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::face::StandardCollector>*>)
	}
	
	Result<cv::face::StandardCollector::PredictResult> cv_face_StandardCollector_PredictResult_PredictResult_int_double(int label_, double distance_) {
		try {
			cv::face::StandardCollector::PredictResult ret(label_, distance_);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::face::StandardCollector::PredictResult>)
	}
	
}
