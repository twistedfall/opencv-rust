#include "face.hpp"
#include "face_types.hpp"

extern "C" {
	Result<void*> cv_face_createFacemarkAAM() {
		try {
			cv::Ptr<cv::face::Facemark> ret = cv::face::createFacemarkAAM();
			return Ok<void*>(new cv::Ptr<cv::face::Facemark>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_face_createFacemarkKazemi() {
		try {
			cv::Ptr<cv::face::Facemark> ret = cv::face::createFacemarkKazemi();
			return Ok<void*>(new cv::Ptr<cv::face::Facemark>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_face_createFacemarkLBF() {
		try {
			cv::Ptr<cv::face::Facemark> ret = cv::face::createFacemarkLBF();
			return Ok<void*>(new cv::Ptr<cv::face::Facemark>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_face_drawFacemarks_const__InputOutputArrayX_const__InputArrayX_Scalar(void* image, void* points, cv::Scalar color) {
		try {
			cv::face::drawFacemarks(*reinterpret_cast<const cv::_InputOutputArray*>(image), *reinterpret_cast<const cv::_InputArray*>(points), color);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_face_getFacesHAAR_const__InputArrayX_const__OutputArrayX_const_StringX(void* image, void* faces, const char* face_cascade_name) {
		try {
			bool ret = cv::face::getFacesHAAR(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<const cv::_OutputArray*>(faces), cv::String(face_cascade_name));
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_face_getFaces_const__InputArrayX_const__OutputArrayX_CParamsX(void* image, void* faces, void* params) {
		try {
			bool ret = cv::face::getFaces(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<const cv::_OutputArray*>(faces), reinterpret_cast<cv::face::CParams*>(params));
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_face_loadDatasetList_String_String_vector_String_X_vector_String_X(char* imageList, char* annotationList, void* images, void* annotations) {
		try {
			bool ret = cv::face::loadDatasetList(cv::String(imageList), cv::String(annotationList), *reinterpret_cast<std::vector<cv::String>*>(images), *reinterpret_cast<std::vector<cv::String>*>(annotations));
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_face_loadFacePoints_String_const__OutputArrayX_float(char* filename, void* points, float offset) {
		try {
			bool ret = cv::face::loadFacePoints(cv::String(filename), *reinterpret_cast<const cv::_OutputArray*>(points), offset);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_face_loadTrainingData_String_String_vector_String_X_const__OutputArrayX_float(char* imageList, char* groundTruth, void* images, void* facePoints, float offset) {
		try {
			bool ret = cv::face::loadTrainingData(cv::String(imageList), cv::String(groundTruth), *reinterpret_cast<std::vector<cv::String>*>(images), *reinterpret_cast<const cv::_OutputArray*>(facePoints), offset);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_face_loadTrainingData_String_vector_String_X_const__OutputArrayX_char_float(char* filename, void* images, void* facePoints, char delim, float offset) {
		try {
			bool ret = cv::face::loadTrainingData(cv::String(filename), *reinterpret_cast<std::vector<cv::String>*>(images), *reinterpret_cast<const cv::_OutputArray*>(facePoints), delim, offset);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_face_loadTrainingData_vector_String__vector_vector_Point2f__X_vector_String_X(void* filename, void* trainlandmarks, void* trainimages) {
		try {
			bool ret = cv::face::loadTrainingData(*reinterpret_cast<std::vector<cv::String>*>(filename), *reinterpret_cast<std::vector<std::vector<cv::Point2f>>*>(trainlandmarks), *reinterpret_cast<std::vector<cv::String>*>(trainimages));
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
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
	
	Result<void*> cv_face_BIF_create_int_int(int num_bands, int num_rotations) {
		try {
			cv::Ptr<cv::face::BIF> ret = cv::face::BIF::create(num_bands, num_rotations);
			return Ok<void*>(new cv::Ptr<cv::face::BIF>(ret));
		} OCVRS_CATCH(Result<void*>)
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
	
	Result_void cv_face_BasicFaceRecognizer_read_const_FileNodeX(void* instance, void* fn) {
		try {
			reinterpret_cast<cv::face::BasicFaceRecognizer*>(instance)->read(*reinterpret_cast<const cv::FileNode*>(fn));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_face_BasicFaceRecognizer_write_const_FileStorageX(void* instance, void* fs) {
		try {
			reinterpret_cast<cv::face::BasicFaceRecognizer*>(instance)->write(*reinterpret_cast<cv::FileStorage*>(fs));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_face_BasicFaceRecognizer_empty_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::face::BasicFaceRecognizer*>(instance)->empty();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<void*> cv_face_CParams_cascade_const(void* instance) {
		try {
			cv::String ret = reinterpret_cast<cv::face::CParams*>(instance)->cascade;
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_face_CParams_setCascade_String(void* instance, char* val) {
		try {
			reinterpret_cast<cv::face::CParams*>(instance)->cascade = cv::String(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_face_CParams_scaleFactor_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::face::CParams*>(instance)->scaleFactor;
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_face_CParams_setScaleFactor_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::face::CParams*>(instance)->scaleFactor = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_face_CParams_minNeighbors_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::face::CParams*>(instance)->minNeighbors;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_face_CParams_setMinNeighbors_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::face::CParams*>(instance)->minNeighbors = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Size> cv_face_CParams_minSize_const(void* instance) {
		try {
			cv::Size ret = reinterpret_cast<cv::face::CParams*>(instance)->minSize;
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_face_CParams_setMinSize_Size(void* instance, cv::Size val) {
		try {
			reinterpret_cast<cv::face::CParams*>(instance)->minSize = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Size> cv_face_CParams_maxSize_const(void* instance) {
		try {
			cv::Size ret = reinterpret_cast<cv::face::CParams*>(instance)->maxSize;
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_face_CParams_setMaxSize_Size(void* instance, cv::Size val) {
		try {
			reinterpret_cast<cv::face::CParams*>(instance)->maxSize = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_face_CParams_face_cascade(void* instance) {
		try {
			cv::CascadeClassifier ret = reinterpret_cast<cv::face::CParams*>(instance)->face_cascade;
			return Ok<void*>(new cv::CascadeClassifier(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_face_CParams_setFace_cascade_CascadeClassifier(void* instance, void* val) {
		try {
			reinterpret_cast<cv::face::CParams*>(instance)->face_cascade = *reinterpret_cast<cv::CascadeClassifier*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_CParams_delete(cv::face::CParams* instance) {
		delete instance;
	}
	Result<void*> cv_face_CParams_CParams_String_double_int_Size_Size(char* cascade_model, double sf, int minN, cv::Size minSz, cv::Size maxSz) {
		try {
			cv::face::CParams* ret = new cv::face::CParams(cv::String(cascade_model), sf, minN, minSz, maxSz);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_face_EigenFaceRecognizer_create_int_double(int num_components, double threshold) {
		try {
			cv::Ptr<cv::face::EigenFaceRecognizer> ret = cv::face::EigenFaceRecognizer::create(num_components, threshold);
			return Ok<void*>(new cv::Ptr<cv::face::EigenFaceRecognizer>(ret));
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
	
	Result_void cv_face_FaceRecognizer_write_const_const_StringX(void* instance, const char* filename) {
		try {
			reinterpret_cast<cv::face::FaceRecognizer*>(instance)->write(cv::String(filename));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_face_FaceRecognizer_read_const_StringX(void* instance, const char* filename) {
		try {
			reinterpret_cast<cv::face::FaceRecognizer*>(instance)->read(cv::String(filename));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_face_FaceRecognizer_write_const_FileStorageX(void* instance, void* fs) {
		try {
			reinterpret_cast<cv::face::FaceRecognizer*>(instance)->write(*reinterpret_cast<cv::FileStorage*>(fs));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_face_FaceRecognizer_read_const_FileNodeX(void* instance, void* fn) {
		try {
			reinterpret_cast<cv::face::FaceRecognizer*>(instance)->read(*reinterpret_cast<const cv::FileNode*>(fn));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_face_FaceRecognizer_empty_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::face::FaceRecognizer*>(instance)->empty();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
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
	
	Result_void cv_face_Facemark_loadModel_String(void* instance, char* model) {
		try {
			reinterpret_cast<cv::face::Facemark*>(instance)->loadModel(cv::String(model));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_face_Facemark_fit_const__InputArrayX_const__InputArrayX_const__OutputArrayX(void* instance, void* image, void* faces, void* landmarks) {
		try {
			bool ret = reinterpret_cast<cv::face::Facemark*>(instance)->fit(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<const cv::_InputArray*>(faces), *reinterpret_cast<const cv::_OutputArray*>(landmarks));
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_face_FacemarkAAM_fitConfig_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const_vector_Config_X(void* instance, void* image, void* roi, void* _landmarks, void* runtime_params) {
		try {
			bool ret = reinterpret_cast<cv::face::FacemarkAAM*>(instance)->fitConfig(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<const cv::_InputArray*>(roi), *reinterpret_cast<const cv::_OutputArray*>(_landmarks), *reinterpret_cast<const std::vector<cv::face::FacemarkAAM::Config>*>(runtime_params));
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<void*> cv_face_FacemarkAAM_create_const_ParamsX(void* parameters) {
		try {
			cv::Ptr<cv::face::FacemarkAAM> ret = cv::face::FacemarkAAM::create(*reinterpret_cast<const cv::face::FacemarkAAM::Params*>(parameters));
			return Ok<void*>(new cv::Ptr<cv::face::FacemarkAAM>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_face_FacemarkAAM_Config_R(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::face::FacemarkAAM::Config*>(instance)->R;
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_face_FacemarkAAM_Config_setR_Mat(void* instance, void* val) {
		try {
			reinterpret_cast<cv::face::FacemarkAAM::Config*>(instance)->R = *reinterpret_cast<cv::Mat*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Point2f> cv_face_FacemarkAAM_Config_t_const(void* instance) {
		try {
			cv::Point2f ret = reinterpret_cast<cv::face::FacemarkAAM::Config*>(instance)->t;
			return Ok<cv::Point2f>(ret);
		} OCVRS_CATCH(Result<cv::Point2f>)
	}
	
	Result_void cv_face_FacemarkAAM_Config_setT_Point2f(void* instance, cv::Point2f val) {
		try {
			reinterpret_cast<cv::face::FacemarkAAM::Config*>(instance)->t = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_face_FacemarkAAM_Config_scale_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::face::FacemarkAAM::Config*>(instance)->scale;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_face_FacemarkAAM_Config_setScale_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::face::FacemarkAAM::Config*>(instance)->scale = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_face_FacemarkAAM_Config_model_scale_idx_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::face::FacemarkAAM::Config*>(instance)->model_scale_idx;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_face_FacemarkAAM_Config_setModel_scale_idx_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::face::FacemarkAAM::Config*>(instance)->model_scale_idx = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_FacemarkAAM_Config_delete(cv::face::FacemarkAAM::Config* instance) {
		delete instance;
	}
	Result<void*> cv_face_FacemarkAAM_Config_Config_Mat_Point2f_float_int(void* rot, cv::Point2f trans, float scaling, int scale_id) {
		try {
			cv::face::FacemarkAAM::Config* ret = new cv::face::FacemarkAAM::Config(*reinterpret_cast<cv::Mat*>(rot), trans, scaling, scale_id);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_face_FacemarkAAM_Data_s0(void* instance) {
		try {
			std::vector<cv::Point2f> ret = reinterpret_cast<cv::face::FacemarkAAM::Data*>(instance)->s0;
			return Ok<void*>(new std::vector<cv::Point2f>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_face_FacemarkAAM_Data_setS0_vector_Point2f_(void* instance, void* val) {
		try {
			reinterpret_cast<cv::face::FacemarkAAM::Data*>(instance)->s0 = *reinterpret_cast<std::vector<cv::Point2f>*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_FacemarkAAM_Data_delete(cv::face::FacemarkAAM::Data* instance) {
		delete instance;
	}
	Result<void*> cv_face_FacemarkAAM_Model_scales(void* instance) {
		try {
			std::vector<float> ret = reinterpret_cast<cv::face::FacemarkAAM::Model*>(instance)->scales;
			return Ok<void*>(new std::vector<float>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_face_FacemarkAAM_Model_setScales_vector_float_(void* instance, void* val) {
		try {
			reinterpret_cast<cv::face::FacemarkAAM::Model*>(instance)->scales = *reinterpret_cast<std::vector<float>*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_face_FacemarkAAM_Model_triangles(void* instance) {
		try {
			std::vector<cv::Vec3i> ret = reinterpret_cast<cv::face::FacemarkAAM::Model*>(instance)->triangles;
			return Ok<void*>(new std::vector<cv::Vec3i>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_face_FacemarkAAM_Model_setTriangles_vector_Vec3i_(void* instance, void* val) {
		try {
			reinterpret_cast<cv::face::FacemarkAAM::Model*>(instance)->triangles = *reinterpret_cast<std::vector<cv::Vec3i>*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_face_FacemarkAAM_Model_textures(void* instance) {
		try {
			std::vector<cv::face::FacemarkAAM::Model::Texture> ret = reinterpret_cast<cv::face::FacemarkAAM::Model*>(instance)->textures;
			return Ok<void*>(new std::vector<cv::face::FacemarkAAM::Model::Texture>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_face_FacemarkAAM_Model_setTextures_vector_Texture_(void* instance, void* val) {
		try {
			reinterpret_cast<cv::face::FacemarkAAM::Model*>(instance)->textures = *reinterpret_cast<std::vector<cv::face::FacemarkAAM::Model::Texture>*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_face_FacemarkAAM_Model_s0(void* instance) {
		try {
			std::vector<cv::Point2f> ret = reinterpret_cast<cv::face::FacemarkAAM::Model*>(instance)->s0;
			return Ok<void*>(new std::vector<cv::Point2f>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_face_FacemarkAAM_Model_setS0_vector_Point2f_(void* instance, void* val) {
		try {
			reinterpret_cast<cv::face::FacemarkAAM::Model*>(instance)->s0 = *reinterpret_cast<std::vector<cv::Point2f>*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_face_FacemarkAAM_Model_S(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::face::FacemarkAAM::Model*>(instance)->S;
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_face_FacemarkAAM_Model_setS_Mat(void* instance, void* val) {
		try {
			reinterpret_cast<cv::face::FacemarkAAM::Model*>(instance)->S = *reinterpret_cast<cv::Mat*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_face_FacemarkAAM_Model_Q(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::face::FacemarkAAM::Model*>(instance)->Q;
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_face_FacemarkAAM_Model_setQ_Mat(void* instance, void* val) {
		try {
			reinterpret_cast<cv::face::FacemarkAAM::Model*>(instance)->Q = *reinterpret_cast<cv::Mat*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_FacemarkAAM_Model_delete(cv::face::FacemarkAAM::Model* instance) {
		delete instance;
	}
	Result<int> cv_face_FacemarkAAM_Model_Texture_max_m_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::face::FacemarkAAM::Model::Texture*>(instance)->max_m;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_face_FacemarkAAM_Model_Texture_setMax_m_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::face::FacemarkAAM::Model::Texture*>(instance)->max_m = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Rect> cv_face_FacemarkAAM_Model_Texture_resolution_const(void* instance) {
		try {
			cv::Rect ret = reinterpret_cast<cv::face::FacemarkAAM::Model::Texture*>(instance)->resolution;
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(Result<cv::Rect>)
	}
	
	Result_void cv_face_FacemarkAAM_Model_Texture_setResolution_Rect(void* instance, cv::Rect val) {
		try {
			reinterpret_cast<cv::face::FacemarkAAM::Model::Texture*>(instance)->resolution = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_face_FacemarkAAM_Model_Texture_A(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::face::FacemarkAAM::Model::Texture*>(instance)->A;
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_face_FacemarkAAM_Model_Texture_setA_Mat(void* instance, void* val) {
		try {
			reinterpret_cast<cv::face::FacemarkAAM::Model::Texture*>(instance)->A = *reinterpret_cast<cv::Mat*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_face_FacemarkAAM_Model_Texture_A0(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::face::FacemarkAAM::Model::Texture*>(instance)->A0;
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_face_FacemarkAAM_Model_Texture_setA0_Mat(void* instance, void* val) {
		try {
			reinterpret_cast<cv::face::FacemarkAAM::Model::Texture*>(instance)->A0 = *reinterpret_cast<cv::Mat*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_face_FacemarkAAM_Model_Texture_AA(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::face::FacemarkAAM::Model::Texture*>(instance)->AA;
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_face_FacemarkAAM_Model_Texture_setAA_Mat(void* instance, void* val) {
		try {
			reinterpret_cast<cv::face::FacemarkAAM::Model::Texture*>(instance)->AA = *reinterpret_cast<cv::Mat*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_face_FacemarkAAM_Model_Texture_AA0(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::face::FacemarkAAM::Model::Texture*>(instance)->AA0;
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_face_FacemarkAAM_Model_Texture_setAA0_Mat(void* instance, void* val) {
		try {
			reinterpret_cast<cv::face::FacemarkAAM::Model::Texture*>(instance)->AA0 = *reinterpret_cast<cv::Mat*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_face_FacemarkAAM_Model_Texture_textureIdx(void* instance) {
		try {
			std::vector<std::vector<cv::Point>> ret = reinterpret_cast<cv::face::FacemarkAAM::Model::Texture*>(instance)->textureIdx;
			return Ok<void*>(new std::vector<std::vector<cv::Point>>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_face_FacemarkAAM_Model_Texture_setTextureIdx_vector_vector_Point__(void* instance, void* val) {
		try {
			reinterpret_cast<cv::face::FacemarkAAM::Model::Texture*>(instance)->textureIdx = *reinterpret_cast<std::vector<std::vector<cv::Point>>*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_face_FacemarkAAM_Model_Texture_base_shape(void* instance) {
		try {
			std::vector<cv::Point2f> ret = reinterpret_cast<cv::face::FacemarkAAM::Model::Texture*>(instance)->base_shape;
			return Ok<void*>(new std::vector<cv::Point2f>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_face_FacemarkAAM_Model_Texture_setBase_shape_vector_Point2f_(void* instance, void* val) {
		try {
			reinterpret_cast<cv::face::FacemarkAAM::Model::Texture*>(instance)->base_shape = *reinterpret_cast<std::vector<cv::Point2f>*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_face_FacemarkAAM_Model_Texture_ind1(void* instance) {
		try {
			std::vector<int> ret = reinterpret_cast<cv::face::FacemarkAAM::Model::Texture*>(instance)->ind1;
			return Ok<void*>(new std::vector<int>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_face_FacemarkAAM_Model_Texture_setInd1_vector_int_(void* instance, void* val) {
		try {
			reinterpret_cast<cv::face::FacemarkAAM::Model::Texture*>(instance)->ind1 = *reinterpret_cast<std::vector<int>*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_face_FacemarkAAM_Model_Texture_ind2(void* instance) {
		try {
			std::vector<int> ret = reinterpret_cast<cv::face::FacemarkAAM::Model::Texture*>(instance)->ind2;
			return Ok<void*>(new std::vector<int>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_face_FacemarkAAM_Model_Texture_setInd2_vector_int_(void* instance, void* val) {
		try {
			reinterpret_cast<cv::face::FacemarkAAM::Model::Texture*>(instance)->ind2 = *reinterpret_cast<std::vector<int>*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_FacemarkAAM_Model_Texture_delete(cv::face::FacemarkAAM::Model::Texture* instance) {
		delete instance;
	}
	Result<void*> cv_face_FacemarkAAM_Params_model_filename_const(void* instance) {
		try {
			std::string ret = reinterpret_cast<cv::face::FacemarkAAM::Params*>(instance)->model_filename;
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_face_FacemarkAAM_Params_setModel_filename_string(void* instance, char* val) {
		try {
			reinterpret_cast<cv::face::FacemarkAAM::Params*>(instance)->model_filename = std::string(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_face_FacemarkAAM_Params_m_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::face::FacemarkAAM::Params*>(instance)->m;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_face_FacemarkAAM_Params_setM_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::face::FacemarkAAM::Params*>(instance)->m = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_face_FacemarkAAM_Params_n_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::face::FacemarkAAM::Params*>(instance)->n;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_face_FacemarkAAM_Params_setN_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::face::FacemarkAAM::Params*>(instance)->n = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_face_FacemarkAAM_Params_n_iter_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::face::FacemarkAAM::Params*>(instance)->n_iter;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_face_FacemarkAAM_Params_setN_iter_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::face::FacemarkAAM::Params*>(instance)->n_iter = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_face_FacemarkAAM_Params_verbose_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::face::FacemarkAAM::Params*>(instance)->verbose;
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_face_FacemarkAAM_Params_setVerbose_bool(void* instance, bool val) {
		try {
			reinterpret_cast<cv::face::FacemarkAAM::Params*>(instance)->verbose = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_face_FacemarkAAM_Params_save_model_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::face::FacemarkAAM::Params*>(instance)->save_model;
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_face_FacemarkAAM_Params_setSave_model_bool(void* instance, bool val) {
		try {
			reinterpret_cast<cv::face::FacemarkAAM::Params*>(instance)->save_model = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_face_FacemarkAAM_Params_max_m_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::face::FacemarkAAM::Params*>(instance)->max_m;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_face_FacemarkAAM_Params_setMax_m_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::face::FacemarkAAM::Params*>(instance)->max_m = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_face_FacemarkAAM_Params_max_n_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::face::FacemarkAAM::Params*>(instance)->max_n;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_face_FacemarkAAM_Params_setMax_n_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::face::FacemarkAAM::Params*>(instance)->max_n = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_face_FacemarkAAM_Params_texture_max_m_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::face::FacemarkAAM::Params*>(instance)->texture_max_m;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_face_FacemarkAAM_Params_setTexture_max_m_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::face::FacemarkAAM::Params*>(instance)->texture_max_m = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_face_FacemarkAAM_Params_scales(void* instance) {
		try {
			std::vector<float> ret = reinterpret_cast<cv::face::FacemarkAAM::Params*>(instance)->scales;
			return Ok<void*>(new std::vector<float>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_face_FacemarkAAM_Params_setScales_vector_float_(void* instance, void* val) {
		try {
			reinterpret_cast<cv::face::FacemarkAAM::Params*>(instance)->scales = *reinterpret_cast<std::vector<float>*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_FacemarkAAM_Params_delete(cv::face::FacemarkAAM::Params* instance) {
		delete instance;
	}
	Result<void*> cv_face_FacemarkAAM_Params_Params() {
		try {
			cv::face::FacemarkAAM::Params* ret = new cv::face::FacemarkAAM::Params();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_face_FacemarkAAM_Params_read_const_FileNodeX(void* instance, void* unnamed) {
		try {
			reinterpret_cast<cv::face::FacemarkAAM::Params*>(instance)->read(*reinterpret_cast<const cv::FileNode*>(unnamed));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_face_FacemarkAAM_Params_write_const_FileStorageX(void* instance, void* unnamed) {
		try {
			reinterpret_cast<cv::face::FacemarkAAM::Params*>(instance)->write(*reinterpret_cast<cv::FileStorage*>(unnamed));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_face_FacemarkKazemi_create_const_ParamsX(void* parameters) {
		try {
			cv::Ptr<cv::face::FacemarkKazemi> ret = cv::face::FacemarkKazemi::create(*reinterpret_cast<const cv::face::FacemarkKazemi::Params*>(parameters));
			return Ok<void*>(new cv::Ptr<cv::face::FacemarkKazemi>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<bool> cv_face_FacemarkKazemi_training_vector_Mat_X_vector_vector_Point2f__X_string_Size_string(void* instance, void* images, void* landmarks, char* configfile, cv::Size scale, char* modelFilename) {
		try {
			bool ret = reinterpret_cast<cv::face::FacemarkKazemi*>(instance)->training(*reinterpret_cast<std::vector<cv::Mat>*>(images), *reinterpret_cast<std::vector<std::vector<cv::Point2f>>*>(landmarks), std::string(configfile), scale, std::string(modelFilename));
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_face_FacemarkKazemi_getFaces_const__InputArrayX_const__OutputArrayX(void* instance, void* image, void* faces) {
		try {
			bool ret = reinterpret_cast<cv::face::FacemarkKazemi*>(instance)->getFaces(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<const cv::_OutputArray*>(faces));
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<unsigned long> cv_face_FacemarkKazemi_Params_cascade_depth_const(void* instance) {
		try {
			unsigned long ret = reinterpret_cast<cv::face::FacemarkKazemi::Params*>(instance)->cascade_depth;
			return Ok<unsigned long>(ret);
		} OCVRS_CATCH(Result<unsigned long>)
	}
	
	Result_void cv_face_FacemarkKazemi_Params_setCascade_depth_unsigned_long(void* instance, unsigned long val) {
		try {
			reinterpret_cast<cv::face::FacemarkKazemi::Params*>(instance)->cascade_depth = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<unsigned long> cv_face_FacemarkKazemi_Params_tree_depth_const(void* instance) {
		try {
			unsigned long ret = reinterpret_cast<cv::face::FacemarkKazemi::Params*>(instance)->tree_depth;
			return Ok<unsigned long>(ret);
		} OCVRS_CATCH(Result<unsigned long>)
	}
	
	Result_void cv_face_FacemarkKazemi_Params_setTree_depth_unsigned_long(void* instance, unsigned long val) {
		try {
			reinterpret_cast<cv::face::FacemarkKazemi::Params*>(instance)->tree_depth = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<unsigned long> cv_face_FacemarkKazemi_Params_num_trees_per_cascade_level_const(void* instance) {
		try {
			unsigned long ret = reinterpret_cast<cv::face::FacemarkKazemi::Params*>(instance)->num_trees_per_cascade_level;
			return Ok<unsigned long>(ret);
		} OCVRS_CATCH(Result<unsigned long>)
	}
	
	Result_void cv_face_FacemarkKazemi_Params_setNum_trees_per_cascade_level_unsigned_long(void* instance, unsigned long val) {
		try {
			reinterpret_cast<cv::face::FacemarkKazemi::Params*>(instance)->num_trees_per_cascade_level = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_face_FacemarkKazemi_Params_learning_rate_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::face::FacemarkKazemi::Params*>(instance)->learning_rate;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_face_FacemarkKazemi_Params_setLearning_rate_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::face::FacemarkKazemi::Params*>(instance)->learning_rate = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<unsigned long> cv_face_FacemarkKazemi_Params_oversampling_amount_const(void* instance) {
		try {
			unsigned long ret = reinterpret_cast<cv::face::FacemarkKazemi::Params*>(instance)->oversampling_amount;
			return Ok<unsigned long>(ret);
		} OCVRS_CATCH(Result<unsigned long>)
	}
	
	Result_void cv_face_FacemarkKazemi_Params_setOversampling_amount_unsigned_long(void* instance, unsigned long val) {
		try {
			reinterpret_cast<cv::face::FacemarkKazemi::Params*>(instance)->oversampling_amount = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<unsigned long> cv_face_FacemarkKazemi_Params_num_test_coordinates_const(void* instance) {
		try {
			unsigned long ret = reinterpret_cast<cv::face::FacemarkKazemi::Params*>(instance)->num_test_coordinates;
			return Ok<unsigned long>(ret);
		} OCVRS_CATCH(Result<unsigned long>)
	}
	
	Result_void cv_face_FacemarkKazemi_Params_setNum_test_coordinates_unsigned_long(void* instance, unsigned long val) {
		try {
			reinterpret_cast<cv::face::FacemarkKazemi::Params*>(instance)->num_test_coordinates = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_face_FacemarkKazemi_Params_lambda_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::face::FacemarkKazemi::Params*>(instance)->lambda;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_face_FacemarkKazemi_Params_setLambda_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::face::FacemarkKazemi::Params*>(instance)->lambda = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<unsigned long> cv_face_FacemarkKazemi_Params_num_test_splits_const(void* instance) {
		try {
			unsigned long ret = reinterpret_cast<cv::face::FacemarkKazemi::Params*>(instance)->num_test_splits;
			return Ok<unsigned long>(ret);
		} OCVRS_CATCH(Result<unsigned long>)
	}
	
	Result_void cv_face_FacemarkKazemi_Params_setNum_test_splits_unsigned_long(void* instance, unsigned long val) {
		try {
			reinterpret_cast<cv::face::FacemarkKazemi::Params*>(instance)->num_test_splits = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_face_FacemarkKazemi_Params_configfile_const(void* instance) {
		try {
			cv::String ret = reinterpret_cast<cv::face::FacemarkKazemi::Params*>(instance)->configfile;
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_face_FacemarkKazemi_Params_setConfigfile_String(void* instance, char* val) {
		try {
			reinterpret_cast<cv::face::FacemarkKazemi::Params*>(instance)->configfile = cv::String(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_FacemarkKazemi_Params_delete(cv::face::FacemarkKazemi::Params* instance) {
		delete instance;
	}
	Result<void*> cv_face_FacemarkKazemi_Params_Params() {
		try {
			cv::face::FacemarkKazemi::Params* ret = new cv::face::FacemarkKazemi::Params();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_face_FacemarkLBF_create_const_ParamsX(void* parameters) {
		try {
			cv::Ptr<cv::face::FacemarkLBF> ret = cv::face::FacemarkLBF::create(*reinterpret_cast<const cv::face::FacemarkLBF::Params*>(parameters));
			return Ok<void*>(new cv::Ptr<cv::face::FacemarkLBF>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<double> cv_face_FacemarkLBF_Params_shape_offset_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::face::FacemarkLBF::Params*>(instance)->shape_offset;
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_face_FacemarkLBF_Params_setShape_offset_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::face::FacemarkLBF::Params*>(instance)->shape_offset = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_face_FacemarkLBF_Params_cascade_face_const(void* instance) {
		try {
			cv::String ret = reinterpret_cast<cv::face::FacemarkLBF::Params*>(instance)->cascade_face;
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_face_FacemarkLBF_Params_setCascade_face_String(void* instance, char* val) {
		try {
			reinterpret_cast<cv::face::FacemarkLBF::Params*>(instance)->cascade_face = cv::String(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_face_FacemarkLBF_Params_verbose_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::face::FacemarkLBF::Params*>(instance)->verbose;
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_face_FacemarkLBF_Params_setVerbose_bool(void* instance, bool val) {
		try {
			reinterpret_cast<cv::face::FacemarkLBF::Params*>(instance)->verbose = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_face_FacemarkLBF_Params_n_landmarks_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::face::FacemarkLBF::Params*>(instance)->n_landmarks;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_face_FacemarkLBF_Params_setN_landmarks_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::face::FacemarkLBF::Params*>(instance)->n_landmarks = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_face_FacemarkLBF_Params_initShape_n_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::face::FacemarkLBF::Params*>(instance)->initShape_n;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_face_FacemarkLBF_Params_setInitShape_n_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::face::FacemarkLBF::Params*>(instance)->initShape_n = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_face_FacemarkLBF_Params_stages_n_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::face::FacemarkLBF::Params*>(instance)->stages_n;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_face_FacemarkLBF_Params_setStages_n_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::face::FacemarkLBF::Params*>(instance)->stages_n = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_face_FacemarkLBF_Params_tree_n_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::face::FacemarkLBF::Params*>(instance)->tree_n;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_face_FacemarkLBF_Params_setTree_n_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::face::FacemarkLBF::Params*>(instance)->tree_n = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_face_FacemarkLBF_Params_tree_depth_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::face::FacemarkLBF::Params*>(instance)->tree_depth;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_face_FacemarkLBF_Params_setTree_depth_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::face::FacemarkLBF::Params*>(instance)->tree_depth = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_face_FacemarkLBF_Params_bagging_overlap_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::face::FacemarkLBF::Params*>(instance)->bagging_overlap;
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_face_FacemarkLBF_Params_setBagging_overlap_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::face::FacemarkLBF::Params*>(instance)->bagging_overlap = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_face_FacemarkLBF_Params_model_filename_const(void* instance) {
		try {
			std::string ret = reinterpret_cast<cv::face::FacemarkLBF::Params*>(instance)->model_filename;
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_face_FacemarkLBF_Params_setModel_filename_string(void* instance, char* val) {
		try {
			reinterpret_cast<cv::face::FacemarkLBF::Params*>(instance)->model_filename = std::string(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_face_FacemarkLBF_Params_save_model_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::face::FacemarkLBF::Params*>(instance)->save_model;
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_face_FacemarkLBF_Params_setSave_model_bool(void* instance, bool val) {
		try {
			reinterpret_cast<cv::face::FacemarkLBF::Params*>(instance)->save_model = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<unsigned int> cv_face_FacemarkLBF_Params_seed_const(void* instance) {
		try {
			unsigned int ret = reinterpret_cast<cv::face::FacemarkLBF::Params*>(instance)->seed;
			return Ok<unsigned int>(ret);
		} OCVRS_CATCH(Result<unsigned int>)
	}
	
	Result_void cv_face_FacemarkLBF_Params_setSeed_unsigned_int(void* instance, unsigned int val) {
		try {
			reinterpret_cast<cv::face::FacemarkLBF::Params*>(instance)->seed = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_face_FacemarkLBF_Params_feats_m(void* instance) {
		try {
			std::vector<int> ret = reinterpret_cast<cv::face::FacemarkLBF::Params*>(instance)->feats_m;
			return Ok<void*>(new std::vector<int>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_face_FacemarkLBF_Params_setFeats_m_vector_int_(void* instance, void* val) {
		try {
			reinterpret_cast<cv::face::FacemarkLBF::Params*>(instance)->feats_m = *reinterpret_cast<std::vector<int>*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_face_FacemarkLBF_Params_radius_m(void* instance) {
		try {
			std::vector<double> ret = reinterpret_cast<cv::face::FacemarkLBF::Params*>(instance)->radius_m;
			return Ok<void*>(new std::vector<double>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_face_FacemarkLBF_Params_setRadius_m_vector_double_(void* instance, void* val) {
		try {
			reinterpret_cast<cv::face::FacemarkLBF::Params*>(instance)->radius_m = *reinterpret_cast<std::vector<double>*>(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Rect> cv_face_FacemarkLBF_Params_detectROI_const(void* instance) {
		try {
			cv::Rect ret = reinterpret_cast<cv::face::FacemarkLBF::Params*>(instance)->detectROI;
			return Ok<cv::Rect>(ret);
		} OCVRS_CATCH(Result<cv::Rect>)
	}
	
	Result_void cv_face_FacemarkLBF_Params_setDetectROI_Rect(void* instance, cv::Rect val) {
		try {
			reinterpret_cast<cv::face::FacemarkLBF::Params*>(instance)->detectROI = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_FacemarkLBF_Params_delete(cv::face::FacemarkLBF::Params* instance) {
		delete instance;
	}
	Result<void*> cv_face_FacemarkLBF_Params_Params() {
		try {
			cv::face::FacemarkLBF::Params* ret = new cv::face::FacemarkLBF::Params();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_face_FacemarkLBF_Params_read_const_FileNodeX(void* instance, void* unnamed) {
		try {
			reinterpret_cast<cv::face::FacemarkLBF::Params*>(instance)->read(*reinterpret_cast<const cv::FileNode*>(unnamed));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_face_FacemarkLBF_Params_write_const_FileStorageX(void* instance, void* unnamed) {
		try {
			reinterpret_cast<cv::face::FacemarkLBF::Params*>(instance)->write(*reinterpret_cast<cv::FileStorage*>(unnamed));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_face_FacemarkTrain_addTrainingSample_const__InputArrayX_const__InputArrayX(void* instance, void* image, void* landmarks) {
		try {
			bool ret = reinterpret_cast<cv::face::FacemarkTrain*>(instance)->addTrainingSample(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<const cv::_InputArray*>(landmarks));
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_face_FacemarkTrain_training_voidX(void* instance, void* parameters) {
		try {
			reinterpret_cast<cv::face::FacemarkTrain*>(instance)->training(parameters);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_face_FacemarkTrain_setFaceDetector_FN_FaceDetector_voidX(void* instance, cv::face::FN_FaceDetector detector, void* userData) {
		try {
			bool ret = reinterpret_cast<cv::face::FacemarkTrain*>(instance)->setFaceDetector(detector, userData);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_face_FacemarkTrain_getFaces_const__InputArrayX_const__OutputArrayX(void* instance, void* image, void* faces) {
		try {
			bool ret = reinterpret_cast<cv::face::FacemarkTrain*>(instance)->getFaces(*reinterpret_cast<const cv::_InputArray*>(image), *reinterpret_cast<const cv::_OutputArray*>(faces));
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_face_FacemarkTrain_getData_voidX(void* instance, void* items) {
		try {
			bool ret = reinterpret_cast<cv::face::FacemarkTrain*>(instance)->getData(items);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<void*> cv_face_FisherFaceRecognizer_create_int_double(int num_components, double threshold) {
		try {
			cv::Ptr<cv::face::FisherFaceRecognizer> ret = cv::face::FisherFaceRecognizer::create(num_components, threshold);
			return Ok<void*>(new cv::Ptr<cv::face::FisherFaceRecognizer>(ret));
		} OCVRS_CATCH(Result<void*>)
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
	
	Result<void*> cv_face_LBPHFaceRecognizer_create_int_int_int_int_double(int radius, int neighbors, int grid_x, int grid_y, double threshold) {
		try {
			cv::Ptr<cv::face::LBPHFaceRecognizer> ret = cv::face::LBPHFaceRecognizer::create(radius, neighbors, grid_x, grid_y, threshold);
			return Ok<void*>(new cv::Ptr<cv::face::LBPHFaceRecognizer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_face_MACE_salt_const_StringX(void* instance, const char* passphrase) {
		try {
			reinterpret_cast<cv::face::MACE*>(instance)->salt(cv::String(passphrase));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_face_MACE_train_const__InputArrayX(void* instance, void* images) {
		try {
			reinterpret_cast<cv::face::MACE*>(instance)->train(*reinterpret_cast<const cv::_InputArray*>(images));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_face_MACE_same_const_const__InputArrayX(void* instance, void* query) {
		try {
			bool ret = reinterpret_cast<cv::face::MACE*>(instance)->same(*reinterpret_cast<const cv::_InputArray*>(query));
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<void*> cv_face_MACE_load_const_StringX_const_StringX(const char* filename, const char* objname) {
		try {
			cv::Ptr<cv::face::MACE> ret = cv::face::MACE::load(cv::String(filename), cv::String(objname));
			return Ok<void*>(new cv::Ptr<cv::face::MACE>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_face_MACE_create_int(int IMGSIZE) {
		try {
			cv::Ptr<cv::face::MACE> ret = cv::face::MACE::create(IMGSIZE);
			return Ok<void*>(new cv::Ptr<cv::face::MACE>(ret));
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
