#include "face.hpp"
#include "face_types.hpp"

extern "C" {
	// createFacemarkAAM()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemark.hpp:83
	// ("cv::face::createFacemarkAAM", vec![(pred!(mut, [], []), _)]),
	void cv_face_createFacemarkAAM(Result<cv::Ptr<cv::face::Facemark>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::face::Facemark> ret = cv::face::createFacemarkAAM();
			Ok(new cv::Ptr<cv::face::Facemark>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createFacemarkKazemi()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemark.hpp:89
	// ("cv::face::createFacemarkKazemi", vec![(pred!(mut, [], []), _)]),
	void cv_face_createFacemarkKazemi(Result<cv::Ptr<cv::face::Facemark>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::face::Facemark> ret = cv::face::createFacemarkKazemi();
			Ok(new cv::Ptr<cv::face::Facemark>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createFacemarkLBF()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemark.hpp:86
	// ("cv::face::createFacemarkLBF", vec![(pred!(mut, [], []), _)]),
	void cv_face_createFacemarkLBF(Result<cv::Ptr<cv::face::Facemark>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::face::Facemark> ret = cv::face::createFacemarkLBF();
			Ok(new cv::Ptr<cv::face::Facemark>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::face::drawFacemarks(InputOutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemark_train.hpp:232
	// ("cv::face::drawFacemarks", vec![(pred!(mut, ["image", "points"], ["const cv::_InputOutputArray*", "const cv::_InputArray*"]), _)]),
	void cv_face_drawFacemarks_const__InputOutputArrayR_const__InputArrayR(const cv::_InputOutputArray* image, const cv::_InputArray* points, ResultVoid* ocvrs_return) {
		try {
			cv::face::drawFacemarks(*image, *points);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// drawFacemarks(InputOutputArray, InputArray, Scalar)(InputOutputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemark_train.hpp:232
	// ("cv::face::drawFacemarks", vec![(pred!(mut, ["image", "points", "color"], ["const cv::_InputOutputArray*", "const cv::_InputArray*", "cv::Scalar"]), _)]),
	void cv_face_drawFacemarks_const__InputOutputArrayR_const__InputArrayR_Scalar(const cv::_InputOutputArray* image, const cv::_InputArray* points, cv::Scalar* color, ResultVoid* ocvrs_return) {
		try {
			cv::face::drawFacemarks(*image, *points, *color);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getFacesHAAR(InputArray, OutputArray, const String &)(InputArray, OutputArray, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemark_train.hpp:76
	// ("cv::face::getFacesHAAR", vec![(pred!(mut, ["image", "faces", "face_cascade_name"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::String*"]), _)]),
	void cv_face_getFacesHAAR_const__InputArrayR_const__OutputArrayR_const_StringR(const cv::_InputArray* image, const cv::_OutputArray* faces, const char* face_cascade_name, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::face::getFacesHAAR(*image, *faces, cv::String(face_cascade_name));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getFaces(InputArray, OutputArray, CParams *)(InputArray, OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemark_train.hpp:74
	// ("cv::face::getFaces", vec![(pred!(mut, ["image", "faces", "params"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::face::CParams*"]), _)]),
	void cv_face_getFaces_const__InputArrayR_const__OutputArrayR_CParamsX(const cv::_InputArray* image, const cv::_OutputArray* faces, cv::face::CParams* params, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::face::getFaces(*image, *faces, params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// loadDatasetList(String, String, std::vector<String> &, std::vector<String> &)(InString, InString, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemark_train.hpp:93
	// ("cv::face::loadDatasetList", vec![(pred!(mut, ["imageList", "annotationList", "images", "annotations"], ["cv::String", "cv::String", "std::vector<cv::String>*", "std::vector<cv::String>*"]), _)]),
	void cv_face_loadDatasetList_String_String_vectorLStringGR_vectorLStringGR(const char* imageList, const char* annotationList, std::vector<cv::String>* images, std::vector<cv::String>* annotations, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::face::loadDatasetList(cv::String(imageList), cv::String(annotationList), *images, *annotations);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::face::loadFacePoints(InString, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemark_train.hpp:212
	// ("cv::face::loadFacePoints", vec![(pred!(mut, ["filename", "points"], ["cv::String", "const cv::_OutputArray*"]), _)]),
	void cv_face_loadFacePoints_String_const__OutputArrayR(const char* filename, const cv::_OutputArray* points, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::face::loadFacePoints(cv::String(filename), *points);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// loadFacePoints(String, OutputArray, float)(InString, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemark_train.hpp:212
	// ("cv::face::loadFacePoints", vec![(pred!(mut, ["filename", "points", "offset"], ["cv::String", "const cv::_OutputArray*", "float"]), _)]),
	void cv_face_loadFacePoints_String_const__OutputArrayR_float(const char* filename, const cv::_OutputArray* points, float offset, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::face::loadFacePoints(cv::String(filename), *points, offset);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::face::loadTrainingData(InString, InString, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemark_train.hpp:163
	// ("cv::face::loadTrainingData", vec![(pred!(mut, ["imageList", "groundTruth", "images", "facePoints"], ["cv::String", "cv::String", "std::vector<cv::String>*", "const cv::_OutputArray*"]), _)]),
	void cv_face_loadTrainingData_String_String_vectorLStringGR_const__OutputArrayR(const char* imageList, const char* groundTruth, std::vector<cv::String>* images, const cv::_OutputArray* facePoints, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::face::loadTrainingData(cv::String(imageList), cv::String(groundTruth), *images, *facePoints);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// loadTrainingData(String, String, std::vector<String> &, OutputArray, float)(InString, InString, CppPassByVoidPtr, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemark_train.hpp:163
	// ("cv::face::loadTrainingData", vec![(pred!(mut, ["imageList", "groundTruth", "images", "facePoints", "offset"], ["cv::String", "cv::String", "std::vector<cv::String>*", "const cv::_OutputArray*", "float"]), _)]),
	void cv_face_loadTrainingData_String_String_vectorLStringGR_const__OutputArrayR_float(const char* imageList, const char* groundTruth, std::vector<cv::String>* images, const cv::_OutputArray* facePoints, float offset, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::face::loadTrainingData(cv::String(imageList), cv::String(groundTruth), *images, *facePoints, offset);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::face::loadTrainingData(InString, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemark_train.hpp:123
	// ("cv::face::loadTrainingData", vec![(pred!(mut, ["filename", "images", "facePoints"], ["cv::String", "std::vector<cv::String>*", "const cv::_OutputArray*"]), _)]),
	void cv_face_loadTrainingData_String_vectorLStringGR_const__OutputArrayR(const char* filename, std::vector<cv::String>* images, const cv::_OutputArray* facePoints, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::face::loadTrainingData(cv::String(filename), *images, *facePoints);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// loadTrainingData(String, std::vector<String> &, OutputArray, char, float)(InString, CppPassByVoidPtr, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemark_train.hpp:123
	// ("cv::face::loadTrainingData", vec![(pred!(mut, ["filename", "images", "facePoints", "delim", "offset"], ["cv::String", "std::vector<cv::String>*", "const cv::_OutputArray*", "char", "float"]), _)]),
	void cv_face_loadTrainingData_String_vectorLStringGR_const__OutputArrayR_char_float(const char* filename, std::vector<cv::String>* images, const cv::_OutputArray* facePoints, char delim, float offset, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::face::loadTrainingData(cv::String(filename), *images, *facePoints, delim, offset);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// loadTrainingData(std::vector<String>, std::vector<std::vector<Point2f>> &, std::vector<String> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemark_train.hpp:184
	// ("cv::face::loadTrainingData", vec![(pred!(mut, ["filename", "trainlandmarks", "trainimages"], ["std::vector<cv::String>", "std::vector<std::vector<cv::Point2f>>*", "std::vector<cv::String>*"]), _)]),
	void cv_face_loadTrainingData_vectorLStringG_vectorLvectorLPoint2fGGR_vectorLStringGR(std::vector<cv::String>* filename, std::vector<std::vector<cv::Point2f>>* trainlandmarks, std::vector<cv::String>* trainimages, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::face::loadTrainingData(*filename, *trainlandmarks, *trainimages);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNumBands()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/bif.hpp:60
	// ("cv::face::BIF::getNumBands", vec![(pred!(const, [], []), _)]),
	void cv_face_BIF_getNumBands_const(const cv::face::BIF* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNumBands();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNumRotations()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/bif.hpp:63
	// ("cv::face::BIF::getNumRotations", vec![(pred!(const, [], []), _)]),
	void cv_face_BIF_getNumRotations_const(const cv::face::BIF* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNumRotations();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// compute(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/bif.hpp:69
	// ("cv::face::BIF::compute", vec![(pred!(const, ["image", "features"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_face_BIF_compute_const_const__InputArrayR_const__OutputArrayR(const cv::face::BIF* instance, const cv::_InputArray* image, const cv::_OutputArray* features, ResultVoid* ocvrs_return) {
		try {
			instance->compute(*image, *features);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/bif.hpp:77
	// ("cv::face::BIF::create", vec![(pred!(mut, ["num_bands", "num_rotations"], ["int", "int"]), _)]),
	void cv_face_BIF_create_int_int(int num_bands, int num_rotations, Result<cv::Ptr<cv::face::BIF>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::face::BIF> ret = cv::face::BIF::create(num_bands, num_rotations);
			Ok(new cv::Ptr<cv::face::BIF>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::face::BIF::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/bif.hpp:77
	// ("cv::face::BIF::create", vec![(pred!(mut, [], []), _)]),
	void cv_face_BIF_create(Result<cv::Ptr<cv::face::BIF>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::face::BIF> ret = cv::face::BIF::create();
			Ok(new cv::Ptr<cv::face::BIF>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::face::BIF::to_Algorithm() generated
	// ("cv::face::BIF::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_face_BIF_to_Algorithm(cv::face::BIF* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::face::BIF::delete() generated
	// ("cv::face::BIF::delete", vec![(pred!(mut, [], []), _)]),
	void cv_face_BIF_delete(cv::face::BIF* instance) {
			delete instance;
	}

	// getNumComponents()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facerec.hpp:24
	// ("cv::face::BasicFaceRecognizer::getNumComponents", vec![(pred!(const, [], []), _)]),
	void cv_face_BasicFaceRecognizer_getNumComponents_const(const cv::face::BasicFaceRecognizer* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNumComponents();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNumComponents(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facerec.hpp:26
	// ("cv::face::BasicFaceRecognizer::setNumComponents", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_face_BasicFaceRecognizer_setNumComponents_int(cv::face::BasicFaceRecognizer* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setNumComponents(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getThreshold()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facerec.hpp:28
	// ("cv::face::BasicFaceRecognizer::getThreshold", vec![(pred!(const, [], []), _)]),
	void cv_face_BasicFaceRecognizer_getThreshold_const(const cv::face::BasicFaceRecognizer* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facerec.hpp:30
	// ("cv::face::BasicFaceRecognizer::setThreshold", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_face_BasicFaceRecognizer_setThreshold_double(cv::face::BasicFaceRecognizer* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setThreshold(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getProjections()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facerec.hpp:31
	// ("cv::face::BasicFaceRecognizer::getProjections", vec![(pred!(const, [], []), _)]),
	void cv_face_BasicFaceRecognizer_getProjections_const(const cv::face::BasicFaceRecognizer* instance, Result<std::vector<cv::Mat>*>* ocvrs_return) {
		try {
			std::vector<cv::Mat> ret = instance->getProjections();
			Ok(new std::vector<cv::Mat>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLabels()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facerec.hpp:32
	// ("cv::face::BasicFaceRecognizer::getLabels", vec![(pred!(const, [], []), _)]),
	void cv_face_BasicFaceRecognizer_getLabels_const(const cv::face::BasicFaceRecognizer* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getLabels();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getEigenValues()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facerec.hpp:33
	// ("cv::face::BasicFaceRecognizer::getEigenValues", vec![(pred!(const, [], []), _)]),
	void cv_face_BasicFaceRecognizer_getEigenValues_const(const cv::face::BasicFaceRecognizer* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getEigenValues();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getEigenVectors()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facerec.hpp:34
	// ("cv::face::BasicFaceRecognizer::getEigenVectors", vec![(pred!(const, [], []), _)]),
	void cv_face_BasicFaceRecognizer_getEigenVectors_const(const cv::face::BasicFaceRecognizer* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getEigenVectors();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMean()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facerec.hpp:35
	// ("cv::face::BasicFaceRecognizer::getMean", vec![(pred!(const, [], []), _)]),
	void cv_face_BasicFaceRecognizer_getMean_const(const cv::face::BasicFaceRecognizer* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getMean();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facerec.hpp:37
	// ("cv::face::BasicFaceRecognizer::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	void cv_face_BasicFaceRecognizer_read_const_FileNodeR(cv::face::BasicFaceRecognizer* instance, const cv::FileNode* fn, ResultVoid* ocvrs_return) {
		try {
			instance->read(*fn);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facerec.hpp:38
	// ("cv::face::BasicFaceRecognizer::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	void cv_face_BasicFaceRecognizer_write_const_FileStorageR(const cv::face::BasicFaceRecognizer* instance, cv::FileStorage* fs, ResultVoid* ocvrs_return) {
		try {
			instance->write(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// empty()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facerec.hpp:39
	// ("cv::face::BasicFaceRecognizer::empty", vec![(pred!(const, [], []), _)]),
	void cv_face_BasicFaceRecognizer_empty_const(const cv::face::BasicFaceRecognizer* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::face::BasicFaceRecognizer::to_EigenFaceRecognizer() generated
	// ("cv::face::BasicFaceRecognizer::to_EigenFaceRecognizer", vec![(pred!(mut, [], []), _)]),
	cv::face::EigenFaceRecognizer* cv_face_BasicFaceRecognizer_to_EigenFaceRecognizer(cv::face::BasicFaceRecognizer* instance) {
			return dynamic_cast<cv::face::EigenFaceRecognizer*>(instance);
	}

	// cv::face::BasicFaceRecognizer::to_FisherFaceRecognizer() generated
	// ("cv::face::BasicFaceRecognizer::to_FisherFaceRecognizer", vec![(pred!(mut, [], []), _)]),
	cv::face::FisherFaceRecognizer* cv_face_BasicFaceRecognizer_to_FisherFaceRecognizer(cv::face::BasicFaceRecognizer* instance) {
			return dynamic_cast<cv::face::FisherFaceRecognizer*>(instance);
	}

	// cv::face::BasicFaceRecognizer::to_Algorithm() generated
	// ("cv::face::BasicFaceRecognizer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_face_BasicFaceRecognizer_to_Algorithm(cv::face::BasicFaceRecognizer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::face::BasicFaceRecognizer::to_FaceRecognizer() generated
	// ("cv::face::BasicFaceRecognizer::to_FaceRecognizer", vec![(pred!(mut, [], []), _)]),
	cv::face::FaceRecognizer* cv_face_BasicFaceRecognizer_to_FaceRecognizer(cv::face::BasicFaceRecognizer* instance) {
			return dynamic_cast<cv::face::FaceRecognizer*>(instance);
	}

	// cv::face::BasicFaceRecognizer::delete() generated
	// ("cv::face::BasicFaceRecognizer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_face_BasicFaceRecognizer_delete(cv::face::BasicFaceRecognizer* instance) {
			delete instance;
	}

	// CParams(String, double, int, Size, Size)(InString, Primitive, Primitive, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemark_train.hpp:42
	// ("cv::face::CParams::CParams", vec![(pred!(mut, ["cascade_model", "sf", "minN", "minSz", "maxSz"], ["cv::String", "double", "int", "cv::Size", "cv::Size"]), _)]),
	void cv_face_CParams_CParams_String_double_int_Size_Size(const char* cascade_model, double sf, int minN, cv::Size* minSz, cv::Size* maxSz, Result<cv::face::CParams*>* ocvrs_return) {
		try {
			cv::face::CParams* ret = new cv::face::CParams(cv::String(cascade_model), sf, minN, *minSz, *maxSz);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::face::CParams::CParams(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemark_train.hpp:42
	// ("cv::face::CParams::CParams", vec![(pred!(mut, ["cascade_model"], ["cv::String"]), _)]),
	void cv_face_CParams_CParams_String(const char* cascade_model, Result<cv::face::CParams*>* ocvrs_return) {
		try {
			cv::face::CParams* ret = new cv::face::CParams(cv::String(cascade_model));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::face::CParams::cascade() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemark_train.hpp:36
	// ("cv::face::CParams::cascade", vec![(pred!(const, [], []), _)]),
	void* cv_face_CParams_propCascade_const(const cv::face::CParams* instance) {
			cv::String ret = instance->cascade;
			return ocvrs_create_string(ret.c_str());
	}

	// cv::face::CParams::setCascade(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemark_train.hpp:36
	// ("cv::face::CParams::setCascade", vec![(pred!(mut, ["val"], ["const cv::String"]), _)]),
	void cv_face_CParams_propCascade_const_String(cv::face::CParams* instance, const char* val) {
			instance->cascade = cv::String(val);
	}

	// cv::face::CParams::scaleFactor() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemark_train.hpp:37
	// ("cv::face::CParams::scaleFactor", vec![(pred!(const, [], []), _)]),
	double cv_face_CParams_propScaleFactor_const(const cv::face::CParams* instance) {
			double ret = instance->scaleFactor;
			return ret;
	}

	// cv::face::CParams::setScaleFactor(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemark_train.hpp:37
	// ("cv::face::CParams::setScaleFactor", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_face_CParams_propScaleFactor_const_double(cv::face::CParams* instance, const double val) {
			instance->scaleFactor = val;
	}

	// cv::face::CParams::minNeighbors() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemark_train.hpp:38
	// ("cv::face::CParams::minNeighbors", vec![(pred!(const, [], []), _)]),
	int cv_face_CParams_propMinNeighbors_const(const cv::face::CParams* instance) {
			int ret = instance->minNeighbors;
			return ret;
	}

	// cv::face::CParams::setMinNeighbors(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemark_train.hpp:38
	// ("cv::face::CParams::setMinNeighbors", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_face_CParams_propMinNeighbors_const_int(cv::face::CParams* instance, const int val) {
			instance->minNeighbors = val;
	}

	// cv::face::CParams::minSize() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemark_train.hpp:39
	// ("cv::face::CParams::minSize", vec![(pred!(const, [], []), _)]),
	void cv_face_CParams_propMinSize_const(const cv::face::CParams* instance, cv::Size* ocvrs_return) {
			cv::Size ret = instance->minSize;
			*ocvrs_return = ret;
	}

	// cv::face::CParams::setMinSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemark_train.hpp:39
	// ("cv::face::CParams::setMinSize", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	void cv_face_CParams_propMinSize_const_Size(cv::face::CParams* instance, const cv::Size* val) {
			instance->minSize = *val;
	}

	// cv::face::CParams::maxSize() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemark_train.hpp:40
	// ("cv::face::CParams::maxSize", vec![(pred!(const, [], []), _)]),
	void cv_face_CParams_propMaxSize_const(const cv::face::CParams* instance, cv::Size* ocvrs_return) {
			cv::Size ret = instance->maxSize;
			*ocvrs_return = ret;
	}

	// cv::face::CParams::setMaxSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemark_train.hpp:40
	// ("cv::face::CParams::setMaxSize", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	void cv_face_CParams_propMaxSize_const_Size(cv::face::CParams* instance, const cv::Size* val) {
			instance->maxSize = *val;
	}

	// cv::face::CParams::face_cascade() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemark_train.hpp:50
	// ("cv::face::CParams::face_cascade", vec![(pred!(const, [], []), _)]),
	cv::CascadeClassifier* cv_face_CParams_propFace_cascade_const(const cv::face::CParams* instance) {
			cv::CascadeClassifier ret = instance->face_cascade;
			return new cv::CascadeClassifier(ret);
	}

	// cv::face::CParams::setFace_cascade(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemark_train.hpp:50
	// ("cv::face::CParams::setFace_cascade", vec![(pred!(mut, ["val"], ["const cv::CascadeClassifier"]), _)]),
	void cv_face_CParams_propFace_cascade_const_CascadeClassifier(cv::face::CParams* instance, const cv::CascadeClassifier* val) {
			instance->face_cascade = *val;
	}

	// cv::face::CParams::delete() generated
	// ("cv::face::CParams::delete", vec![(pred!(mut, [], []), _)]),
	void cv_face_CParams_delete(cv::face::CParams* instance) {
			delete instance;
	}

	// create(int, double)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facerec.hpp:86
	// ("cv::face::EigenFaceRecognizer::create", vec![(pred!(mut, ["num_components", "threshold"], ["int", "double"]), _)]),
	void cv_face_EigenFaceRecognizer_create_int_double(int num_components, double threshold, Result<cv::Ptr<cv::face::EigenFaceRecognizer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::face::EigenFaceRecognizer> ret = cv::face::EigenFaceRecognizer::create(num_components, threshold);
			Ok(new cv::Ptr<cv::face::EigenFaceRecognizer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::face::EigenFaceRecognizer::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facerec.hpp:86
	// ("cv::face::EigenFaceRecognizer::create", vec![(pred!(mut, [], []), _)]),
	void cv_face_EigenFaceRecognizer_create(Result<cv::Ptr<cv::face::EigenFaceRecognizer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::face::EigenFaceRecognizer> ret = cv::face::EigenFaceRecognizer::create();
			Ok(new cv::Ptr<cv::face::EigenFaceRecognizer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::face::EigenFaceRecognizer::to_Algorithm() generated
	// ("cv::face::EigenFaceRecognizer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_face_EigenFaceRecognizer_to_Algorithm(cv::face::EigenFaceRecognizer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::face::EigenFaceRecognizer::to_BasicFaceRecognizer() generated
	// ("cv::face::EigenFaceRecognizer::to_BasicFaceRecognizer", vec![(pred!(mut, [], []), _)]),
	cv::face::BasicFaceRecognizer* cv_face_EigenFaceRecognizer_to_BasicFaceRecognizer(cv::face::EigenFaceRecognizer* instance) {
			return dynamic_cast<cv::face::BasicFaceRecognizer*>(instance);
	}

	// cv::face::EigenFaceRecognizer::to_FaceRecognizer() generated
	// ("cv::face::EigenFaceRecognizer::to_FaceRecognizer", vec![(pred!(mut, [], []), _)]),
	cv::face::FaceRecognizer* cv_face_EigenFaceRecognizer_to_FaceRecognizer(cv::face::EigenFaceRecognizer* instance) {
			return dynamic_cast<cv::face::FaceRecognizer*>(instance);
	}

	// cv::face::EigenFaceRecognizer::delete() generated
	// ("cv::face::EigenFaceRecognizer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_face_EigenFaceRecognizer_delete(cv::face::EigenFaceRecognizer* instance) {
			delete instance;
	}

	// train(InputArrayOfArrays, InputArray)(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face.hpp:209
	// ("cv::face::FaceRecognizer::train", vec![(pred!(mut, ["src", "labels"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_face_FaceRecognizer_train_const__InputArrayR_const__InputArrayR(cv::face::FaceRecognizer* instance, const cv::_InputArray* src, const cv::_InputArray* labels, ResultVoid* ocvrs_return) {
		try {
			instance->train(*src, *labels);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// update(InputArrayOfArrays, InputArray)(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face.hpp:258
	// ("cv::face::FaceRecognizer::update", vec![(pred!(mut, ["src", "labels"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_face_FaceRecognizer_update_const__InputArrayR_const__InputArrayR(cv::face::FaceRecognizer* instance, const cv::_InputArray* src, const cv::_InputArray* labels, ResultVoid* ocvrs_return) {
		try {
			instance->update(*src, *labels);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// predict(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face.hpp:261
	// ("cv::face::FaceRecognizer::predict", vec![(pred!(const, ["src"], ["const cv::_InputArray*"]), _)]),
	void cv_face_FaceRecognizer_predict_const_const__InputArrayR(const cv::face::FaceRecognizer* instance, const cv::_InputArray* src, Result<int>* ocvrs_return) {
		try {
			int ret = instance->predict(*src);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// predict(InputArray, int &, double &)(InputArray, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face.hpp:299
	// ("cv::face::FaceRecognizer::predict", vec![(pred!(const, ["src", "label", "confidence"], ["const cv::_InputArray*", "int*", "double*"]), _)]),
	void cv_face_FaceRecognizer_predict_const_const__InputArrayR_intR_doubleR(const cv::face::FaceRecognizer* instance, const cv::_InputArray* src, int* label, double* confidence, ResultVoid* ocvrs_return) {
		try {
			instance->predict(*src, *label, *confidence);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// predict(InputArray, Ptr<PredictCollector>)(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face.hpp:309
	// ("cv::face::FaceRecognizer::predict", vec![(pred!(const, ["src", "collector"], ["const cv::_InputArray*", "cv::Ptr<cv::face::PredictCollector>"]), _)]),
	void cv_face_FaceRecognizer_predict_const_const__InputArrayR_PtrLPredictCollectorG(const cv::face::FaceRecognizer* instance, const cv::_InputArray* src, cv::Ptr<cv::face::PredictCollector>* collector, ResultVoid* ocvrs_return) {
		try {
			instance->predict(*src, *collector);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face.hpp:323
	// ("cv::face::FaceRecognizer::write", vec![(pred!(const, ["filename"], ["const cv::String*"]), _)]),
	void cv_face_FaceRecognizer_write_const_const_StringR(const cv::face::FaceRecognizer* instance, const char* filename, ResultVoid* ocvrs_return) {
		try {
			instance->write(cv::String(filename));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face.hpp:332
	// ("cv::face::FaceRecognizer::read", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
	void cv_face_FaceRecognizer_read_const_StringR(cv::face::FaceRecognizer* instance, const char* filename, ResultVoid* ocvrs_return) {
		try {
			instance->read(cv::String(filename));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face.hpp:338
	// ("cv::face::FaceRecognizer::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	void cv_face_FaceRecognizer_write_const_FileStorageR(const cv::face::FaceRecognizer* instance, cv::FileStorage* fs, ResultVoid* ocvrs_return) {
		try {
			instance->write(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face.hpp:341
	// ("cv::face::FaceRecognizer::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	void cv_face_FaceRecognizer_read_const_FileNodeR(cv::face::FaceRecognizer* instance, const cv::FileNode* fn, ResultVoid* ocvrs_return) {
		try {
			instance->read(*fn);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// empty()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face.hpp:344
	// ("cv::face::FaceRecognizer::empty", vec![(pred!(const, [], []), _)]),
	void cv_face_FaceRecognizer_empty_const(const cv::face::FaceRecognizer* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setLabelInfo(int, const String &)(Primitive, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face.hpp:350
	// ("cv::face::FaceRecognizer::setLabelInfo", vec![(pred!(mut, ["label", "strInfo"], ["int", "const cv::String*"]), _)]),
	void cv_face_FaceRecognizer_setLabelInfo_int_const_StringR(cv::face::FaceRecognizer* instance, int label, const char* strInfo, ResultVoid* ocvrs_return) {
		try {
			instance->setLabelInfo(label, cv::String(strInfo));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLabelInfo(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face.hpp:357
	// ("cv::face::FaceRecognizer::getLabelInfo", vec![(pred!(const, ["label"], ["int"]), _)]),
	void cv_face_FaceRecognizer_getLabelInfo_const_int(const cv::face::FaceRecognizer* instance, int label, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getLabelInfo(label);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLabelsByString(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face.hpp:364
	// ("cv::face::FaceRecognizer::getLabelsByString", vec![(pred!(const, ["str"], ["const cv::String*"]), _)]),
	void cv_face_FaceRecognizer_getLabelsByString_const_const_StringR(const cv::face::FaceRecognizer* instance, const char* str, Result<std::vector<int>*>* ocvrs_return) {
		try {
			std::vector<int> ret = instance->getLabelsByString(cv::String(str));
			Ok(new std::vector<int>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getThreshold()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face.hpp:366
	// ("cv::face::FaceRecognizer::getThreshold", vec![(pred!(const, [], []), _)]),
	void cv_face_FaceRecognizer_getThreshold_const(const cv::face::FaceRecognizer* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face.hpp:368
	// ("cv::face::FaceRecognizer::setThreshold", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_face_FaceRecognizer_setThreshold_double(cv::face::FaceRecognizer* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setThreshold(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::face::FaceRecognizer::to_BasicFaceRecognizer() generated
	// ("cv::face::FaceRecognizer::to_BasicFaceRecognizer", vec![(pred!(mut, [], []), _)]),
	cv::face::BasicFaceRecognizer* cv_face_FaceRecognizer_to_BasicFaceRecognizer(cv::face::FaceRecognizer* instance) {
			return dynamic_cast<cv::face::BasicFaceRecognizer*>(instance);
	}

	// cv::face::FaceRecognizer::to_EigenFaceRecognizer() generated
	// ("cv::face::FaceRecognizer::to_EigenFaceRecognizer", vec![(pred!(mut, [], []), _)]),
	cv::face::EigenFaceRecognizer* cv_face_FaceRecognizer_to_EigenFaceRecognizer(cv::face::FaceRecognizer* instance) {
			return dynamic_cast<cv::face::EigenFaceRecognizer*>(instance);
	}

	// cv::face::FaceRecognizer::to_FisherFaceRecognizer() generated
	// ("cv::face::FaceRecognizer::to_FisherFaceRecognizer", vec![(pred!(mut, [], []), _)]),
	cv::face::FisherFaceRecognizer* cv_face_FaceRecognizer_to_FisherFaceRecognizer(cv::face::FaceRecognizer* instance) {
			return dynamic_cast<cv::face::FisherFaceRecognizer*>(instance);
	}

	// cv::face::FaceRecognizer::to_LBPHFaceRecognizer() generated
	// ("cv::face::FaceRecognizer::to_LBPHFaceRecognizer", vec![(pred!(mut, [], []), _)]),
	cv::face::LBPHFaceRecognizer* cv_face_FaceRecognizer_to_LBPHFaceRecognizer(cv::face::FaceRecognizer* instance) {
			return dynamic_cast<cv::face::LBPHFaceRecognizer*>(instance);
	}

	// cv::face::FaceRecognizer::to_Algorithm() generated
	// ("cv::face::FaceRecognizer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_face_FaceRecognizer_to_Algorithm(cv::face::FaceRecognizer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::face::FaceRecognizer::delete() generated
	// ("cv::face::FaceRecognizer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_face_FaceRecognizer_delete(cv::face::FaceRecognizer* instance) {
			delete instance;
	}

	// loadModel(String)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemark.hpp:59
	// ("cv::face::Facemark::loadModel", vec![(pred!(mut, ["model"], ["cv::String"]), _)]),
	void cv_face_Facemark_loadModel_String(cv::face::Facemark* instance, const char* model, ResultVoid* ocvrs_return) {
		try {
			instance->loadModel(cv::String(model));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// fit(InputArray, InputArray, OutputArrayOfArrays)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemark.hpp:76
	// ("cv::face::Facemark::fit", vec![(pred!(mut, ["image", "faces", "landmarks"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_face_Facemark_fit_const__InputArrayR_const__InputArrayR_const__OutputArrayR(cv::face::Facemark* instance, const cv::_InputArray* image, const cv::_InputArray* faces, const cv::_OutputArray* landmarks, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->fit(*image, *faces, *landmarks);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::face::Facemark::to_FacemarkAAM() generated
	// ("cv::face::Facemark::to_FacemarkAAM", vec![(pred!(mut, [], []), _)]),
	cv::face::FacemarkAAM* cv_face_Facemark_to_FacemarkAAM(cv::face::Facemark* instance) {
			return dynamic_cast<cv::face::FacemarkAAM*>(instance);
	}

	// cv::face::Facemark::to_FacemarkKazemi() generated
	// ("cv::face::Facemark::to_FacemarkKazemi", vec![(pred!(mut, [], []), _)]),
	cv::face::FacemarkKazemi* cv_face_Facemark_to_FacemarkKazemi(cv::face::Facemark* instance) {
			return dynamic_cast<cv::face::FacemarkKazemi*>(instance);
	}

	// cv::face::Facemark::to_FacemarkLBF() generated
	// ("cv::face::Facemark::to_FacemarkLBF", vec![(pred!(mut, [], []), _)]),
	cv::face::FacemarkLBF* cv_face_Facemark_to_FacemarkLBF(cv::face::Facemark* instance) {
			return dynamic_cast<cv::face::FacemarkLBF*>(instance);
	}

	// cv::face::Facemark::to_FacemarkTrain() generated
	// ("cv::face::Facemark::to_FacemarkTrain", vec![(pred!(mut, [], []), _)]),
	cv::face::FacemarkTrain* cv_face_Facemark_to_FacemarkTrain(cv::face::Facemark* instance) {
			return dynamic_cast<cv::face::FacemarkTrain*>(instance);
	}

	// cv::face::Facemark::to_Algorithm() generated
	// ("cv::face::Facemark::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_face_Facemark_to_Algorithm(cv::face::Facemark* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::face::Facemark::delete() generated
	// ("cv::face::Facemark::delete", vec![(pred!(mut, [], []), _)]),
	void cv_face_Facemark_delete(cv::face::Facemark* instance) {
			delete instance;
	}

	// fitConfig(InputArray, InputArray, OutputArrayOfArrays, const std::vector<Config> &)(InputArray, InputArray, OutputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:149
	// ("cv::face::FacemarkAAM::fitConfig", vec![(pred!(mut, ["image", "roi", "_landmarks", "runtime_params"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const std::vector<cv::face::FacemarkAAM::Config>*"]), _)]),
	void cv_face_FacemarkAAM_fitConfig_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const_vectorLConfigGR(cv::face::FacemarkAAM* instance, const cv::_InputArray* image, const cv::_InputArray* roi, const cv::_OutputArray* _landmarks, const std::vector<cv::face::FacemarkAAM::Config>* runtime_params, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->fitConfig(*image, *roi, *_landmarks, *runtime_params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(const FacemarkAAM::Params &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:153
	// ("cv::face::FacemarkAAM::create", vec![(pred!(mut, ["parameters"], ["const cv::face::FacemarkAAM::Params*"]), _)]),
	void cv_face_FacemarkAAM_create_const_ParamsR(const cv::face::FacemarkAAM::Params* parameters, Result<cv::Ptr<cv::face::FacemarkAAM>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::face::FacemarkAAM> ret = cv::face::FacemarkAAM::create(*parameters);
			Ok(new cv::Ptr<cv::face::FacemarkAAM>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::face::FacemarkAAM::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:153
	// ("cv::face::FacemarkAAM::create", vec![(pred!(mut, [], []), _)]),
	void cv_face_FacemarkAAM_create(Result<cv::Ptr<cv::face::FacemarkAAM>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::face::FacemarkAAM> ret = cv::face::FacemarkAAM::create();
			Ok(new cv::Ptr<cv::face::FacemarkAAM>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::face::FacemarkAAM::to_Algorithm() generated
	// ("cv::face::FacemarkAAM::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_face_FacemarkAAM_to_Algorithm(cv::face::FacemarkAAM* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::face::FacemarkAAM::to_Facemark() generated
	// ("cv::face::FacemarkAAM::to_Facemark", vec![(pred!(mut, [], []), _)]),
	cv::face::Facemark* cv_face_FacemarkAAM_to_Facemark(cv::face::FacemarkAAM* instance) {
			return dynamic_cast<cv::face::Facemark*>(instance);
	}

	// cv::face::FacemarkAAM::to_FacemarkTrain() generated
	// ("cv::face::FacemarkAAM::to_FacemarkTrain", vec![(pred!(mut, [], []), _)]),
	cv::face::FacemarkTrain* cv_face_FacemarkAAM_to_FacemarkTrain(cv::face::FacemarkAAM* instance) {
			return dynamic_cast<cv::face::FacemarkTrain*>(instance);
	}

	// cv::face::FacemarkAAM::delete() generated
	// ("cv::face::FacemarkAAM::delete", vec![(pred!(mut, [], []), _)]),
	void cv_face_FacemarkAAM_delete(cv::face::FacemarkAAM* instance) {
			delete instance;
	}

	// Config(Mat, Point2f, float, int)(TraitClass, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:82
	// ("cv::face::FacemarkAAM::Config::Config", vec![(pred!(mut, ["rot", "trans", "scaling", "scale_id"], ["cv::Mat", "cv::Point2f", "float", "int"]), _)]),
	void cv_face_FacemarkAAM_Config_Config_Mat_Point2f_float_int(cv::Mat* rot, cv::Point2f* trans, float scaling, int scale_id, Result<cv::face::FacemarkAAM::Config*>* ocvrs_return) {
		try {
			cv::face::FacemarkAAM::Config* ret = new cv::face::FacemarkAAM::Config(*rot, *trans, scaling, scale_id);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::face::FacemarkAAM::Config::Config() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:82
	// ("cv::face::FacemarkAAM::Config::Config", vec![(pred!(mut, [], []), _)]),
	void cv_face_FacemarkAAM_Config_Config(Result<cv::face::FacemarkAAM::Config*>* ocvrs_return) {
		try {
			cv::face::FacemarkAAM::Config* ret = new cv::face::FacemarkAAM::Config();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::face::FacemarkAAM::Config::R() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:88
	// ("cv::face::FacemarkAAM::Config::R", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_face_FacemarkAAM_Config_propR_const(const cv::face::FacemarkAAM::Config* instance) {
			cv::Mat ret = instance->R;
			return new cv::Mat(ret);
	}

	// cv::face::FacemarkAAM::Config::setR(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:88
	// ("cv::face::FacemarkAAM::Config::setR", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_face_FacemarkAAM_Config_propR_const_Mat(cv::face::FacemarkAAM::Config* instance, const cv::Mat* val) {
			instance->R = *val;
	}

	// cv::face::FacemarkAAM::Config::t() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:89
	// ("cv::face::FacemarkAAM::Config::t", vec![(pred!(const, [], []), _)]),
	void cv_face_FacemarkAAM_Config_propT_const(const cv::face::FacemarkAAM::Config* instance, cv::Point2f* ocvrs_return) {
			cv::Point2f ret = instance->t;
			*ocvrs_return = ret;
	}

	// cv::face::FacemarkAAM::Config::setT(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:89
	// ("cv::face::FacemarkAAM::Config::setT", vec![(pred!(mut, ["val"], ["const cv::Point2f"]), _)]),
	void cv_face_FacemarkAAM_Config_propT_const_Point2f(cv::face::FacemarkAAM::Config* instance, const cv::Point2f* val) {
			instance->t = *val;
	}

	// cv::face::FacemarkAAM::Config::scale() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:90
	// ("cv::face::FacemarkAAM::Config::scale", vec![(pred!(const, [], []), _)]),
	float cv_face_FacemarkAAM_Config_propScale_const(const cv::face::FacemarkAAM::Config* instance) {
			float ret = instance->scale;
			return ret;
	}

	// cv::face::FacemarkAAM::Config::setScale(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:90
	// ("cv::face::FacemarkAAM::Config::setScale", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_face_FacemarkAAM_Config_propScale_const_float(cv::face::FacemarkAAM::Config* instance, const float val) {
			instance->scale = val;
	}

	// cv::face::FacemarkAAM::Config::model_scale_idx() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:91
	// ("cv::face::FacemarkAAM::Config::model_scale_idx", vec![(pred!(const, [], []), _)]),
	int cv_face_FacemarkAAM_Config_propModel_scale_idx_const(const cv::face::FacemarkAAM::Config* instance) {
			int ret = instance->model_scale_idx;
			return ret;
	}

	// cv::face::FacemarkAAM::Config::setModel_scale_idx(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:91
	// ("cv::face::FacemarkAAM::Config::setModel_scale_idx", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_face_FacemarkAAM_Config_propModel_scale_idx_const_int(cv::face::FacemarkAAM::Config* instance, const int val) {
			instance->model_scale_idx = val;
	}

	// cv::face::FacemarkAAM::Config::delete() generated
	// ("cv::face::FacemarkAAM::Config::delete", vec![(pred!(mut, [], []), _)]),
	void cv_face_FacemarkAAM_Config_delete(cv::face::FacemarkAAM::Config* instance) {
			delete instance;
	}

	// cv::face::FacemarkAAM::Data::defaultNew() generated
	// ("cv::face::FacemarkAAM::Data::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::face::FacemarkAAM::Data* cv_face_FacemarkAAM_Data_defaultNew_const() {
			cv::face::FacemarkAAM::Data* ret = new cv::face::FacemarkAAM::Data();
			return ret;
	}

	// cv::face::FacemarkAAM::Data::s0() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:100
	// ("cv::face::FacemarkAAM::Data::s0", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Point2f>* cv_face_FacemarkAAM_Data_propS0_const(const cv::face::FacemarkAAM::Data* instance) {
			std::vector<cv::Point2f> ret = instance->s0;
			return new std::vector<cv::Point2f>(ret);
	}

	// cv::face::FacemarkAAM::Data::setS0(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:100
	// ("cv::face::FacemarkAAM::Data::setS0", vec![(pred!(mut, ["val"], ["const std::vector<cv::Point2f>"]), _)]),
	void cv_face_FacemarkAAM_Data_propS0_const_vectorLPoint2fG(cv::face::FacemarkAAM::Data* instance, const std::vector<cv::Point2f>* val) {
			instance->s0 = *val;
	}

	// cv::face::FacemarkAAM::Data::delete() generated
	// ("cv::face::FacemarkAAM::Data::delete", vec![(pred!(mut, [], []), _)]),
	void cv_face_FacemarkAAM_Data_delete(cv::face::FacemarkAAM::Data* instance) {
			delete instance;
	}

	// cv::face::FacemarkAAM::Model::defaultNew() generated
	// ("cv::face::FacemarkAAM::Model::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::face::FacemarkAAM::Model* cv_face_FacemarkAAM_Model_defaultNew_const() {
			cv::face::FacemarkAAM::Model* ret = new cv::face::FacemarkAAM::Model();
			return ret;
	}

	// cv::face::FacemarkAAM::Model::scales() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:108
	// ("cv::face::FacemarkAAM::Model::scales", vec![(pred!(const, [], []), _)]),
	std::vector<float>* cv_face_FacemarkAAM_Model_propScales_const(const cv::face::FacemarkAAM::Model* instance) {
			std::vector<float> ret = instance->scales;
			return new std::vector<float>(ret);
	}

	// cv::face::FacemarkAAM::Model::setScales(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:108
	// ("cv::face::FacemarkAAM::Model::setScales", vec![(pred!(mut, ["val"], ["const std::vector<float>"]), _)]),
	void cv_face_FacemarkAAM_Model_propScales_const_vectorLfloatG(cv::face::FacemarkAAM::Model* instance, const std::vector<float>* val) {
			instance->scales = *val;
	}

	// cv::face::FacemarkAAM::Model::triangles() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:112
	// ("cv::face::FacemarkAAM::Model::triangles", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Vec3i>* cv_face_FacemarkAAM_Model_propTriangles_const(const cv::face::FacemarkAAM::Model* instance) {
			std::vector<cv::Vec3i> ret = instance->triangles;
			return new std::vector<cv::Vec3i>(ret);
	}

	// cv::face::FacemarkAAM::Model::setTriangles(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:112
	// ("cv::face::FacemarkAAM::Model::setTriangles", vec![(pred!(mut, ["val"], ["const std::vector<cv::Vec3i>"]), _)]),
	void cv_face_FacemarkAAM_Model_propTriangles_const_vectorLVec3iG(cv::face::FacemarkAAM::Model* instance, const std::vector<cv::Vec3i>* val) {
			instance->triangles = *val;
	}

	// cv::face::FacemarkAAM::Model::textures() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:137
	// ("cv::face::FacemarkAAM::Model::textures", vec![(pred!(const, [], []), _)]),
	std::vector<cv::face::FacemarkAAM::Model::Texture>* cv_face_FacemarkAAM_Model_propTextures_const(const cv::face::FacemarkAAM::Model* instance) {
			std::vector<cv::face::FacemarkAAM::Model::Texture> ret = instance->textures;
			return new std::vector<cv::face::FacemarkAAM::Model::Texture>(ret);
	}

	// cv::face::FacemarkAAM::Model::setTextures(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:137
	// ("cv::face::FacemarkAAM::Model::setTextures", vec![(pred!(mut, ["val"], ["const std::vector<cv::face::FacemarkAAM::Model::Texture>"]), _)]),
	void cv_face_FacemarkAAM_Model_propTextures_const_vectorLTextureG(cv::face::FacemarkAAM::Model* instance, const std::vector<cv::face::FacemarkAAM::Model::Texture>* val) {
			instance->textures = *val;
	}

	// cv::face::FacemarkAAM::Model::s0() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:141
	// ("cv::face::FacemarkAAM::Model::s0", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Point2f>* cv_face_FacemarkAAM_Model_propS0_const(const cv::face::FacemarkAAM::Model* instance) {
			std::vector<cv::Point2f> ret = instance->s0;
			return new std::vector<cv::Point2f>(ret);
	}

	// cv::face::FacemarkAAM::Model::setS0(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:141
	// ("cv::face::FacemarkAAM::Model::setS0", vec![(pred!(mut, ["val"], ["const std::vector<cv::Point2f>"]), _)]),
	void cv_face_FacemarkAAM_Model_propS0_const_vectorLPoint2fG(cv::face::FacemarkAAM::Model* instance, const std::vector<cv::Point2f>* val) {
			instance->s0 = *val;
	}

	// cv::face::FacemarkAAM::Model::S() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:143
	// ("cv::face::FacemarkAAM::Model::S", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_face_FacemarkAAM_Model_propS_const(const cv::face::FacemarkAAM::Model* instance) {
			cv::Mat ret = instance->S;
			return new cv::Mat(ret);
	}

	// cv::face::FacemarkAAM::Model::setS(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:143
	// ("cv::face::FacemarkAAM::Model::setS", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_face_FacemarkAAM_Model_propS_const_Mat(cv::face::FacemarkAAM::Model* instance, const cv::Mat* val) {
			instance->S = *val;
	}

	// cv::face::FacemarkAAM::Model::Q() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:143
	// ("cv::face::FacemarkAAM::Model::Q", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_face_FacemarkAAM_Model_propQ_const(const cv::face::FacemarkAAM::Model* instance) {
			cv::Mat ret = instance->Q;
			return new cv::Mat(ret);
	}

	// cv::face::FacemarkAAM::Model::setQ(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:143
	// ("cv::face::FacemarkAAM::Model::setQ", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_face_FacemarkAAM_Model_propQ_const_Mat(cv::face::FacemarkAAM::Model* instance, const cv::Mat* val) {
			instance->Q = *val;
	}

	// cv::face::FacemarkAAM::Model::delete() generated
	// ("cv::face::FacemarkAAM::Model::delete", vec![(pred!(mut, [], []), _)]),
	void cv_face_FacemarkAAM_Model_delete(cv::face::FacemarkAAM::Model* instance) {
			delete instance;
	}

	// cv::face::FacemarkAAM::Model::Texture::defaultNew() generated
	// ("cv::face::FacemarkAAM::Model::Texture::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::face::FacemarkAAM::Model::Texture* cv_face_FacemarkAAM_Model_Texture_defaultNew_const() {
			cv::face::FacemarkAAM::Model::Texture* ret = new cv::face::FacemarkAAM::Model::Texture();
			return ret;
	}

	// cv::face::FacemarkAAM::Model::Texture::max_m() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:116
	// ("cv::face::FacemarkAAM::Model::Texture::max_m", vec![(pred!(const, [], []), _)]),
	int cv_face_FacemarkAAM_Model_Texture_propMax_m_const(const cv::face::FacemarkAAM::Model::Texture* instance) {
			int ret = instance->max_m;
			return ret;
	}

	// cv::face::FacemarkAAM::Model::Texture::setMax_m(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:116
	// ("cv::face::FacemarkAAM::Model::Texture::setMax_m", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_face_FacemarkAAM_Model_Texture_propMax_m_const_int(cv::face::FacemarkAAM::Model::Texture* instance, const int val) {
			instance->max_m = val;
	}

	// cv::face::FacemarkAAM::Model::Texture::resolution() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:117
	// ("cv::face::FacemarkAAM::Model::Texture::resolution", vec![(pred!(const, [], []), _)]),
	void cv_face_FacemarkAAM_Model_Texture_propResolution_const(const cv::face::FacemarkAAM::Model::Texture* instance, cv::Rect* ocvrs_return) {
			cv::Rect ret = instance->resolution;
			*ocvrs_return = ret;
	}

	// cv::face::FacemarkAAM::Model::Texture::setResolution(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:117
	// ("cv::face::FacemarkAAM::Model::Texture::setResolution", vec![(pred!(mut, ["val"], ["const cv::Rect"]), _)]),
	void cv_face_FacemarkAAM_Model_Texture_propResolution_const_Rect(cv::face::FacemarkAAM::Model::Texture* instance, const cv::Rect* val) {
			instance->resolution = *val;
	}

	// cv::face::FacemarkAAM::Model::Texture::A() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:119
	// ("cv::face::FacemarkAAM::Model::Texture::A", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_face_FacemarkAAM_Model_Texture_propA_const(const cv::face::FacemarkAAM::Model::Texture* instance) {
			cv::Mat ret = instance->A;
			return new cv::Mat(ret);
	}

	// cv::face::FacemarkAAM::Model::Texture::setA(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:119
	// ("cv::face::FacemarkAAM::Model::Texture::setA", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_face_FacemarkAAM_Model_Texture_propA_const_Mat(cv::face::FacemarkAAM::Model::Texture* instance, const cv::Mat* val) {
			instance->A = *val;
	}

	// cv::face::FacemarkAAM::Model::Texture::A0() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:121
	// ("cv::face::FacemarkAAM::Model::Texture::A0", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_face_FacemarkAAM_Model_Texture_propA0_const(const cv::face::FacemarkAAM::Model::Texture* instance) {
			cv::Mat ret = instance->A0;
			return new cv::Mat(ret);
	}

	// cv::face::FacemarkAAM::Model::Texture::setA0(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:121
	// ("cv::face::FacemarkAAM::Model::Texture::setA0", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_face_FacemarkAAM_Model_Texture_propA0_const_Mat(cv::face::FacemarkAAM::Model::Texture* instance, const cv::Mat* val) {
			instance->A0 = *val;
	}

	// cv::face::FacemarkAAM::Model::Texture::AA() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:123
	// ("cv::face::FacemarkAAM::Model::Texture::AA", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_face_FacemarkAAM_Model_Texture_propAA_const(const cv::face::FacemarkAAM::Model::Texture* instance) {
			cv::Mat ret = instance->AA;
			return new cv::Mat(ret);
	}

	// cv::face::FacemarkAAM::Model::Texture::setAA(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:123
	// ("cv::face::FacemarkAAM::Model::Texture::setAA", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_face_FacemarkAAM_Model_Texture_propAA_const_Mat(cv::face::FacemarkAAM::Model::Texture* instance, const cv::Mat* val) {
			instance->AA = *val;
	}

	// cv::face::FacemarkAAM::Model::Texture::AA0() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:125
	// ("cv::face::FacemarkAAM::Model::Texture::AA0", vec![(pred!(const, [], []), _)]),
	cv::Mat* cv_face_FacemarkAAM_Model_Texture_propAA0_const(const cv::face::FacemarkAAM::Model::Texture* instance) {
			cv::Mat ret = instance->AA0;
			return new cv::Mat(ret);
	}

	// cv::face::FacemarkAAM::Model::Texture::setAA0(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:125
	// ("cv::face::FacemarkAAM::Model::Texture::setAA0", vec![(pred!(mut, ["val"], ["const cv::Mat"]), _)]),
	void cv_face_FacemarkAAM_Model_Texture_propAA0_const_Mat(cv::face::FacemarkAAM::Model::Texture* instance, const cv::Mat* val) {
			instance->AA0 = *val;
	}

	// cv::face::FacemarkAAM::Model::Texture::textureIdx() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:128
	// ("cv::face::FacemarkAAM::Model::Texture::textureIdx", vec![(pred!(const, [], []), _)]),
	std::vector<std::vector<cv::Point>>* cv_face_FacemarkAAM_Model_Texture_propTextureIdx_const(const cv::face::FacemarkAAM::Model::Texture* instance) {
			std::vector<std::vector<cv::Point>> ret = instance->textureIdx;
			return new std::vector<std::vector<cv::Point>>(ret);
	}

	// cv::face::FacemarkAAM::Model::Texture::setTextureIdx(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:128
	// ("cv::face::FacemarkAAM::Model::Texture::setTextureIdx", vec![(pred!(mut, ["val"], ["const std::vector<std::vector<cv::Point>>"]), _)]),
	void cv_face_FacemarkAAM_Model_Texture_propTextureIdx_const_vectorLvectorLPointGG(cv::face::FacemarkAAM::Model::Texture* instance, const std::vector<std::vector<cv::Point>>* val) {
			instance->textureIdx = *val;
	}

	// cv::face::FacemarkAAM::Model::Texture::base_shape() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:130
	// ("cv::face::FacemarkAAM::Model::Texture::base_shape", vec![(pred!(const, [], []), _)]),
	std::vector<cv::Point2f>* cv_face_FacemarkAAM_Model_Texture_propBase_shape_const(const cv::face::FacemarkAAM::Model::Texture* instance) {
			std::vector<cv::Point2f> ret = instance->base_shape;
			return new std::vector<cv::Point2f>(ret);
	}

	// cv::face::FacemarkAAM::Model::Texture::setBase_shape(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:130
	// ("cv::face::FacemarkAAM::Model::Texture::setBase_shape", vec![(pred!(mut, ["val"], ["const std::vector<cv::Point2f>"]), _)]),
	void cv_face_FacemarkAAM_Model_Texture_propBase_shape_const_vectorLPoint2fG(cv::face::FacemarkAAM::Model::Texture* instance, const std::vector<cv::Point2f>* val) {
			instance->base_shape = *val;
	}

	// cv::face::FacemarkAAM::Model::Texture::ind1() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:132
	// ("cv::face::FacemarkAAM::Model::Texture::ind1", vec![(pred!(const, [], []), _)]),
	std::vector<int>* cv_face_FacemarkAAM_Model_Texture_propInd1_const(const cv::face::FacemarkAAM::Model::Texture* instance) {
			std::vector<int> ret = instance->ind1;
			return new std::vector<int>(ret);
	}

	// cv::face::FacemarkAAM::Model::Texture::setInd1(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:132
	// ("cv::face::FacemarkAAM::Model::Texture::setInd1", vec![(pred!(mut, ["val"], ["const std::vector<int>"]), _)]),
	void cv_face_FacemarkAAM_Model_Texture_propInd1_const_vectorLintG(cv::face::FacemarkAAM::Model::Texture* instance, const std::vector<int>* val) {
			instance->ind1 = *val;
	}

	// cv::face::FacemarkAAM::Model::Texture::ind2() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:134
	// ("cv::face::FacemarkAAM::Model::Texture::ind2", vec![(pred!(const, [], []), _)]),
	std::vector<int>* cv_face_FacemarkAAM_Model_Texture_propInd2_const(const cv::face::FacemarkAAM::Model::Texture* instance) {
			std::vector<int> ret = instance->ind2;
			return new std::vector<int>(ret);
	}

	// cv::face::FacemarkAAM::Model::Texture::setInd2(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:134
	// ("cv::face::FacemarkAAM::Model::Texture::setInd2", vec![(pred!(mut, ["val"], ["const std::vector<int>"]), _)]),
	void cv_face_FacemarkAAM_Model_Texture_propInd2_const_vectorLintG(cv::face::FacemarkAAM::Model::Texture* instance, const std::vector<int>* val) {
			instance->ind2 = *val;
	}

	// cv::face::FacemarkAAM::Model::Texture::delete() generated
	// ("cv::face::FacemarkAAM::Model::Texture::delete", vec![(pred!(mut, [], []), _)]),
	void cv_face_FacemarkAAM_Model_Texture_delete(cv::face::FacemarkAAM::Model::Texture* instance) {
			delete instance;
	}

	// Params()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:55
	// ("cv::face::FacemarkAAM::Params::Params", vec![(pred!(mut, [], []), _)]),
	void cv_face_FacemarkAAM_Params_Params(Result<cv::face::FacemarkAAM::Params*>* ocvrs_return) {
		try {
			cv::face::FacemarkAAM::Params* ret = new cv::face::FacemarkAAM::Params();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:60
	// ("cv::face::FacemarkAAM::Params::read", vec![(pred!(mut, ["unnamed"], ["const cv::FileNode*"]), _)]),
	void cv_face_FacemarkAAM_Params_read_const_FileNodeR(cv::face::FacemarkAAM::Params* instance, const cv::FileNode* unnamed, ResultVoid* ocvrs_return) {
		try {
			instance->read(*unnamed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:65
	// ("cv::face::FacemarkAAM::Params::write", vec![(pred!(const, ["unnamed"], ["cv::FileStorage*"]), _)]),
	void cv_face_FacemarkAAM_Params_write_const_FileStorageR(const cv::face::FacemarkAAM::Params* instance, cv::FileStorage* unnamed, ResultVoid* ocvrs_return) {
		try {
			instance->write(*unnamed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::face::FacemarkAAM::Params::model_filename() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:67
	// ("cv::face::FacemarkAAM::Params::model_filename", vec![(pred!(const, [], []), _)]),
	void* cv_face_FacemarkAAM_Params_propModel_filename_const(const cv::face::FacemarkAAM::Params* instance) {
			std::string ret = instance->model_filename;
			return ocvrs_create_string(ret.c_str());
	}

	// cv::face::FacemarkAAM::Params::setModel_filename(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:67
	// ("cv::face::FacemarkAAM::Params::setModel_filename", vec![(pred!(mut, ["val"], ["const std::string"]), _)]),
	void cv_face_FacemarkAAM_Params_propModel_filename_const_string(cv::face::FacemarkAAM::Params* instance, const char* val) {
			instance->model_filename = std::string(val);
	}

	// cv::face::FacemarkAAM::Params::m() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:68
	// ("cv::face::FacemarkAAM::Params::m", vec![(pred!(const, [], []), _)]),
	int cv_face_FacemarkAAM_Params_propM_const(const cv::face::FacemarkAAM::Params* instance) {
			int ret = instance->m;
			return ret;
	}

	// cv::face::FacemarkAAM::Params::setM(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:68
	// ("cv::face::FacemarkAAM::Params::setM", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_face_FacemarkAAM_Params_propM_const_int(cv::face::FacemarkAAM::Params* instance, const int val) {
			instance->m = val;
	}

	// cv::face::FacemarkAAM::Params::n() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:69
	// ("cv::face::FacemarkAAM::Params::n", vec![(pred!(const, [], []), _)]),
	int cv_face_FacemarkAAM_Params_propN_const(const cv::face::FacemarkAAM::Params* instance) {
			int ret = instance->n;
			return ret;
	}

	// cv::face::FacemarkAAM::Params::setN(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:69
	// ("cv::face::FacemarkAAM::Params::setN", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_face_FacemarkAAM_Params_propN_const_int(cv::face::FacemarkAAM::Params* instance, const int val) {
			instance->n = val;
	}

	// cv::face::FacemarkAAM::Params::n_iter() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:70
	// ("cv::face::FacemarkAAM::Params::n_iter", vec![(pred!(const, [], []), _)]),
	int cv_face_FacemarkAAM_Params_propN_iter_const(const cv::face::FacemarkAAM::Params* instance) {
			int ret = instance->n_iter;
			return ret;
	}

	// cv::face::FacemarkAAM::Params::setN_iter(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:70
	// ("cv::face::FacemarkAAM::Params::setN_iter", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_face_FacemarkAAM_Params_propN_iter_const_int(cv::face::FacemarkAAM::Params* instance, const int val) {
			instance->n_iter = val;
	}

	// cv::face::FacemarkAAM::Params::verbose() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:71
	// ("cv::face::FacemarkAAM::Params::verbose", vec![(pred!(const, [], []), _)]),
	bool cv_face_FacemarkAAM_Params_propVerbose_const(const cv::face::FacemarkAAM::Params* instance) {
			bool ret = instance->verbose;
			return ret;
	}

	// cv::face::FacemarkAAM::Params::setVerbose(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:71
	// ("cv::face::FacemarkAAM::Params::setVerbose", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_face_FacemarkAAM_Params_propVerbose_const_bool(cv::face::FacemarkAAM::Params* instance, const bool val) {
			instance->verbose = val;
	}

	// cv::face::FacemarkAAM::Params::save_model() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:72
	// ("cv::face::FacemarkAAM::Params::save_model", vec![(pred!(const, [], []), _)]),
	bool cv_face_FacemarkAAM_Params_propSave_model_const(const cv::face::FacemarkAAM::Params* instance) {
			bool ret = instance->save_model;
			return ret;
	}

	// cv::face::FacemarkAAM::Params::setSave_model(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:72
	// ("cv::face::FacemarkAAM::Params::setSave_model", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_face_FacemarkAAM_Params_propSave_model_const_bool(cv::face::FacemarkAAM::Params* instance, const bool val) {
			instance->save_model = val;
	}

	// cv::face::FacemarkAAM::Params::max_m() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:73
	// ("cv::face::FacemarkAAM::Params::max_m", vec![(pred!(const, [], []), _)]),
	int cv_face_FacemarkAAM_Params_propMax_m_const(const cv::face::FacemarkAAM::Params* instance) {
			int ret = instance->max_m;
			return ret;
	}

	// cv::face::FacemarkAAM::Params::setMax_m(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:73
	// ("cv::face::FacemarkAAM::Params::setMax_m", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_face_FacemarkAAM_Params_propMax_m_const_int(cv::face::FacemarkAAM::Params* instance, const int val) {
			instance->max_m = val;
	}

	// cv::face::FacemarkAAM::Params::max_n() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:73
	// ("cv::face::FacemarkAAM::Params::max_n", vec![(pred!(const, [], []), _)]),
	int cv_face_FacemarkAAM_Params_propMax_n_const(const cv::face::FacemarkAAM::Params* instance) {
			int ret = instance->max_n;
			return ret;
	}

	// cv::face::FacemarkAAM::Params::setMax_n(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:73
	// ("cv::face::FacemarkAAM::Params::setMax_n", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_face_FacemarkAAM_Params_propMax_n_const_int(cv::face::FacemarkAAM::Params* instance, const int val) {
			instance->max_n = val;
	}

	// cv::face::FacemarkAAM::Params::texture_max_m() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:73
	// ("cv::face::FacemarkAAM::Params::texture_max_m", vec![(pred!(const, [], []), _)]),
	int cv_face_FacemarkAAM_Params_propTexture_max_m_const(const cv::face::FacemarkAAM::Params* instance) {
			int ret = instance->texture_max_m;
			return ret;
	}

	// cv::face::FacemarkAAM::Params::setTexture_max_m(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:73
	// ("cv::face::FacemarkAAM::Params::setTexture_max_m", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_face_FacemarkAAM_Params_propTexture_max_m_const_int(cv::face::FacemarkAAM::Params* instance, const int val) {
			instance->texture_max_m = val;
	}

	// cv::face::FacemarkAAM::Params::scales() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:74
	// ("cv::face::FacemarkAAM::Params::scales", vec![(pred!(const, [], []), _)]),
	std::vector<float>* cv_face_FacemarkAAM_Params_propScales_const(const cv::face::FacemarkAAM::Params* instance) {
			std::vector<float> ret = instance->scales;
			return new std::vector<float>(ret);
	}

	// cv::face::FacemarkAAM::Params::setScales(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkAAM.hpp:74
	// ("cv::face::FacemarkAAM::Params::setScales", vec![(pred!(mut, ["val"], ["const std::vector<float>"]), _)]),
	void cv_face_FacemarkAAM_Params_propScales_const_vectorLfloatG(cv::face::FacemarkAAM::Params* instance, const std::vector<float>* val) {
			instance->scales = *val;
	}

	// cv::face::FacemarkAAM::Params::delete() generated
	// ("cv::face::FacemarkAAM::Params::delete", vec![(pred!(mut, [], []), _)]),
	void cv_face_FacemarkAAM_Params_delete(cv::face::FacemarkAAM::Params* instance) {
			delete instance;
	}

	// create(const FacemarkKazemi::Params &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/face_alignment.hpp:39
	// ("cv::face::FacemarkKazemi::create", vec![(pred!(mut, ["parameters"], ["const cv::face::FacemarkKazemi::Params*"]), _)]),
	void cv_face_FacemarkKazemi_create_const_ParamsR(const cv::face::FacemarkKazemi::Params* parameters, Result<cv::Ptr<cv::face::FacemarkKazemi>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::face::FacemarkKazemi> ret = cv::face::FacemarkKazemi::create(*parameters);
			Ok(new cv::Ptr<cv::face::FacemarkKazemi>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::face::FacemarkKazemi::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/face_alignment.hpp:39
	// ("cv::face::FacemarkKazemi::create", vec![(pred!(mut, [], []), _)]),
	void cv_face_FacemarkKazemi_create(Result<cv::Ptr<cv::face::FacemarkKazemi>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::face::FacemarkKazemi> ret = cv::face::FacemarkKazemi::create();
			Ok(new cv::Ptr<cv::face::FacemarkKazemi>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// training(std::vector<Mat> &, std::vector<std::vector<Point2f>> &, std::string, Size, std::string)(CppPassByVoidPtr, CppPassByVoidPtr, InString, SimpleClass, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/face_alignment.hpp:51
	// ("cv::face::FacemarkKazemi::training", vec![(pred!(mut, ["images", "landmarks", "configfile", "scale", "modelFilename"], ["std::vector<cv::Mat>*", "std::vector<std::vector<cv::Point2f>>*", "std::string", "cv::Size", "std::string"]), _)]),
	void cv_face_FacemarkKazemi_training_vectorLMatGR_vectorLvectorLPoint2fGGR_string_Size_string(cv::face::FacemarkKazemi* instance, std::vector<cv::Mat>* images, std::vector<std::vector<cv::Point2f>>* landmarks, const char* configfile, cv::Size* scale, const char* modelFilename, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->training(*images, *landmarks, std::string(configfile), *scale, std::string(modelFilename));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::face::FacemarkKazemi::training(CppPassByVoidPtr, CppPassByVoidPtr, InString, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/face_alignment.hpp:51
	// ("cv::face::FacemarkKazemi::training", vec![(pred!(mut, ["images", "landmarks", "configfile", "scale"], ["std::vector<cv::Mat>*", "std::vector<std::vector<cv::Point2f>>*", "std::string", "cv::Size"]), _)]),
	void cv_face_FacemarkKazemi_training_vectorLMatGR_vectorLvectorLPoint2fGGR_string_Size(cv::face::FacemarkKazemi* instance, std::vector<cv::Mat>* images, std::vector<std::vector<cv::Point2f>>* landmarks, const char* configfile, cv::Size* scale, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->training(*images, *landmarks, std::string(configfile), *scale);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setFaceDetector(bool (*)(InputArray, OutputArray, void *), void *)(Function, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/face_alignment.hpp:54
	// ("cv::face::FacemarkKazemi::setFaceDetector", vec![(pred!(mut, ["f", "userData"], ["bool (*)(const cv::_InputArray&, const cv::_OutputArray&, void*)", "void*"]), _)]),
	void cv_face_FacemarkKazemi_setFaceDetector_bool__X__const_cv__InputArrayR__const_cv__OutputArrayR__voidX__voidX(cv::face::FacemarkKazemi* instance, bool (*f)(const cv::_InputArray&, const cv::_OutputArray&, void*), void* userData, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->setFaceDetector(f, userData);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getFaces(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/face_alignment.hpp:56
	// ("cv::face::FacemarkKazemi::getFaces", vec![(pred!(mut, ["image", "faces"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_face_FacemarkKazemi_getFaces_const__InputArrayR_const__OutputArrayR(cv::face::FacemarkKazemi* instance, const cv::_InputArray* image, const cv::_OutputArray* faces, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getFaces(*image, *faces);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::face::FacemarkKazemi::to_Algorithm() generated
	// ("cv::face::FacemarkKazemi::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_face_FacemarkKazemi_to_Algorithm(cv::face::FacemarkKazemi* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::face::FacemarkKazemi::to_Facemark() generated
	// ("cv::face::FacemarkKazemi::to_Facemark", vec![(pred!(mut, [], []), _)]),
	cv::face::Facemark* cv_face_FacemarkKazemi_to_Facemark(cv::face::FacemarkKazemi* instance) {
			return dynamic_cast<cv::face::Facemark*>(instance);
	}

	// cv::face::FacemarkKazemi::delete() generated
	// ("cv::face::FacemarkKazemi::delete", vec![(pred!(mut, [], []), _)]),
	void cv_face_FacemarkKazemi_delete(cv::face::FacemarkKazemi* instance) {
			delete instance;
	}

	// Params()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/face_alignment.hpp:19
	// ("cv::face::FacemarkKazemi::Params::Params", vec![(pred!(mut, [], []), _)]),
	void cv_face_FacemarkKazemi_Params_Params(Result<cv::face::FacemarkKazemi::Params*>* ocvrs_return) {
		try {
			cv::face::FacemarkKazemi::Params* ret = new cv::face::FacemarkKazemi::Params();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::face::FacemarkKazemi::Params::cascade_depth() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/face_alignment.hpp:21
	// ("cv::face::FacemarkKazemi::Params::cascade_depth", vec![(pred!(const, [], []), _)]),
	unsigned long cv_face_FacemarkKazemi_Params_propCascade_depth_const(const cv::face::FacemarkKazemi::Params* instance) {
			unsigned long ret = instance->cascade_depth;
			return ret;
	}

	// cv::face::FacemarkKazemi::Params::setCascade_depth(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/face_alignment.hpp:21
	// ("cv::face::FacemarkKazemi::Params::setCascade_depth", vec![(pred!(mut, ["val"], ["const unsigned long"]), _)]),
	void cv_face_FacemarkKazemi_Params_propCascade_depth_const_unsigned_long(cv::face::FacemarkKazemi::Params* instance, const unsigned long val) {
			instance->cascade_depth = val;
	}

	// cv::face::FacemarkKazemi::Params::tree_depth() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/face_alignment.hpp:23
	// ("cv::face::FacemarkKazemi::Params::tree_depth", vec![(pred!(const, [], []), _)]),
	unsigned long cv_face_FacemarkKazemi_Params_propTree_depth_const(const cv::face::FacemarkKazemi::Params* instance) {
			unsigned long ret = instance->tree_depth;
			return ret;
	}

	// cv::face::FacemarkKazemi::Params::setTree_depth(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/face_alignment.hpp:23
	// ("cv::face::FacemarkKazemi::Params::setTree_depth", vec![(pred!(mut, ["val"], ["const unsigned long"]), _)]),
	void cv_face_FacemarkKazemi_Params_propTree_depth_const_unsigned_long(cv::face::FacemarkKazemi::Params* instance, const unsigned long val) {
			instance->tree_depth = val;
	}

	// cv::face::FacemarkKazemi::Params::num_trees_per_cascade_level() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/face_alignment.hpp:25
	// ("cv::face::FacemarkKazemi::Params::num_trees_per_cascade_level", vec![(pred!(const, [], []), _)]),
	unsigned long cv_face_FacemarkKazemi_Params_propNum_trees_per_cascade_level_const(const cv::face::FacemarkKazemi::Params* instance) {
			unsigned long ret = instance->num_trees_per_cascade_level;
			return ret;
	}

	// cv::face::FacemarkKazemi::Params::setNum_trees_per_cascade_level(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/face_alignment.hpp:25
	// ("cv::face::FacemarkKazemi::Params::setNum_trees_per_cascade_level", vec![(pred!(mut, ["val"], ["const unsigned long"]), _)]),
	void cv_face_FacemarkKazemi_Params_propNum_trees_per_cascade_level_const_unsigned_long(cv::face::FacemarkKazemi::Params* instance, const unsigned long val) {
			instance->num_trees_per_cascade_level = val;
	}

	// cv::face::FacemarkKazemi::Params::learning_rate() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/face_alignment.hpp:27
	// ("cv::face::FacemarkKazemi::Params::learning_rate", vec![(pred!(const, [], []), _)]),
	float cv_face_FacemarkKazemi_Params_propLearning_rate_const(const cv::face::FacemarkKazemi::Params* instance) {
			float ret = instance->learning_rate;
			return ret;
	}

	// cv::face::FacemarkKazemi::Params::setLearning_rate(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/face_alignment.hpp:27
	// ("cv::face::FacemarkKazemi::Params::setLearning_rate", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_face_FacemarkKazemi_Params_propLearning_rate_const_float(cv::face::FacemarkKazemi::Params* instance, const float val) {
			instance->learning_rate = val;
	}

	// cv::face::FacemarkKazemi::Params::oversampling_amount() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/face_alignment.hpp:29
	// ("cv::face::FacemarkKazemi::Params::oversampling_amount", vec![(pred!(const, [], []), _)]),
	unsigned long cv_face_FacemarkKazemi_Params_propOversampling_amount_const(const cv::face::FacemarkKazemi::Params* instance) {
			unsigned long ret = instance->oversampling_amount;
			return ret;
	}

	// cv::face::FacemarkKazemi::Params::setOversampling_amount(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/face_alignment.hpp:29
	// ("cv::face::FacemarkKazemi::Params::setOversampling_amount", vec![(pred!(mut, ["val"], ["const unsigned long"]), _)]),
	void cv_face_FacemarkKazemi_Params_propOversampling_amount_const_unsigned_long(cv::face::FacemarkKazemi::Params* instance, const unsigned long val) {
			instance->oversampling_amount = val;
	}

	// cv::face::FacemarkKazemi::Params::num_test_coordinates() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/face_alignment.hpp:31
	// ("cv::face::FacemarkKazemi::Params::num_test_coordinates", vec![(pred!(const, [], []), _)]),
	unsigned long cv_face_FacemarkKazemi_Params_propNum_test_coordinates_const(const cv::face::FacemarkKazemi::Params* instance) {
			unsigned long ret = instance->num_test_coordinates;
			return ret;
	}

	// cv::face::FacemarkKazemi::Params::setNum_test_coordinates(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/face_alignment.hpp:31
	// ("cv::face::FacemarkKazemi::Params::setNum_test_coordinates", vec![(pred!(mut, ["val"], ["const unsigned long"]), _)]),
	void cv_face_FacemarkKazemi_Params_propNum_test_coordinates_const_unsigned_long(cv::face::FacemarkKazemi::Params* instance, const unsigned long val) {
			instance->num_test_coordinates = val;
	}

	// cv::face::FacemarkKazemi::Params::lambda() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/face_alignment.hpp:33
	// ("cv::face::FacemarkKazemi::Params::lambda", vec![(pred!(const, [], []), _)]),
	float cv_face_FacemarkKazemi_Params_propLambda_const(const cv::face::FacemarkKazemi::Params* instance) {
			float ret = instance->lambda;
			return ret;
	}

	// cv::face::FacemarkKazemi::Params::setLambda(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/face_alignment.hpp:33
	// ("cv::face::FacemarkKazemi::Params::setLambda", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_face_FacemarkKazemi_Params_propLambda_const_float(cv::face::FacemarkKazemi::Params* instance, const float val) {
			instance->lambda = val;
	}

	// cv::face::FacemarkKazemi::Params::num_test_splits() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/face_alignment.hpp:35
	// ("cv::face::FacemarkKazemi::Params::num_test_splits", vec![(pred!(const, [], []), _)]),
	unsigned long cv_face_FacemarkKazemi_Params_propNum_test_splits_const(const cv::face::FacemarkKazemi::Params* instance) {
			unsigned long ret = instance->num_test_splits;
			return ret;
	}

	// cv::face::FacemarkKazemi::Params::setNum_test_splits(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/face_alignment.hpp:35
	// ("cv::face::FacemarkKazemi::Params::setNum_test_splits", vec![(pred!(mut, ["val"], ["const unsigned long"]), _)]),
	void cv_face_FacemarkKazemi_Params_propNum_test_splits_const_unsigned_long(cv::face::FacemarkKazemi::Params* instance, const unsigned long val) {
			instance->num_test_splits = val;
	}

	// cv::face::FacemarkKazemi::Params::configfile() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/face_alignment.hpp:37
	// ("cv::face::FacemarkKazemi::Params::configfile", vec![(pred!(const, [], []), _)]),
	void* cv_face_FacemarkKazemi_Params_propConfigfile_const(const cv::face::FacemarkKazemi::Params* instance) {
			cv::String ret = instance->configfile;
			return ocvrs_create_string(ret.c_str());
	}

	// cv::face::FacemarkKazemi::Params::setConfigfile(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/face_alignment.hpp:37
	// ("cv::face::FacemarkKazemi::Params::setConfigfile", vec![(pred!(mut, ["val"], ["const cv::String"]), _)]),
	void cv_face_FacemarkKazemi_Params_propConfigfile_const_String(cv::face::FacemarkKazemi::Params* instance, const char* val) {
			instance->configfile = cv::String(val);
	}

	// cv::face::FacemarkKazemi::Params::delete() generated
	// ("cv::face::FacemarkKazemi::Params::delete", vec![(pred!(mut, [], []), _)]),
	void cv_face_FacemarkKazemi_Params_delete(cv::face::FacemarkKazemi::Params* instance) {
			delete instance;
	}

	// create(const FacemarkLBF::Params &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkLBF.hpp:111
	// ("cv::face::FacemarkLBF::create", vec![(pred!(mut, ["parameters"], ["const cv::face::FacemarkLBF::Params*"]), _)]),
	void cv_face_FacemarkLBF_create_const_ParamsR(const cv::face::FacemarkLBF::Params* parameters, Result<cv::Ptr<cv::face::FacemarkLBF>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::face::FacemarkLBF> ret = cv::face::FacemarkLBF::create(*parameters);
			Ok(new cv::Ptr<cv::face::FacemarkLBF>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::face::FacemarkLBF::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkLBF.hpp:111
	// ("cv::face::FacemarkLBF::create", vec![(pred!(mut, [], []), _)]),
	void cv_face_FacemarkLBF_create(Result<cv::Ptr<cv::face::FacemarkLBF>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::face::FacemarkLBF> ret = cv::face::FacemarkLBF::create();
			Ok(new cv::Ptr<cv::face::FacemarkLBF>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::face::FacemarkLBF::to_Algorithm() generated
	// ("cv::face::FacemarkLBF::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_face_FacemarkLBF_to_Algorithm(cv::face::FacemarkLBF* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::face::FacemarkLBF::to_Facemark() generated
	// ("cv::face::FacemarkLBF::to_Facemark", vec![(pred!(mut, [], []), _)]),
	cv::face::Facemark* cv_face_FacemarkLBF_to_Facemark(cv::face::FacemarkLBF* instance) {
			return dynamic_cast<cv::face::Facemark*>(instance);
	}

	// cv::face::FacemarkLBF::to_FacemarkTrain() generated
	// ("cv::face::FacemarkLBF::to_FacemarkTrain", vec![(pred!(mut, [], []), _)]),
	cv::face::FacemarkTrain* cv_face_FacemarkLBF_to_FacemarkTrain(cv::face::FacemarkLBF* instance) {
			return dynamic_cast<cv::face::FacemarkTrain*>(instance);
	}

	// cv::face::FacemarkLBF::delete() generated
	// ("cv::face::FacemarkLBF::delete", vec![(pred!(mut, [], []), _)]),
	void cv_face_FacemarkLBF_delete(cv::face::FacemarkLBF* instance) {
			delete instance;
	}

	// Params()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkLBF.hpp:56
	// ("cv::face::FacemarkLBF::Params::Params", vec![(pred!(mut, [], []), _)]),
	void cv_face_FacemarkLBF_Params_Params(Result<cv::face::FacemarkLBF::Params*>* ocvrs_return) {
		try {
			cv::face::FacemarkLBF::Params* ret = new cv::face::FacemarkLBF::Params();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkLBF.hpp:91
	// ("cv::face::FacemarkLBF::Params::read", vec![(pred!(mut, ["unnamed"], ["const cv::FileNode*"]), _)]),
	void cv_face_FacemarkLBF_Params_read_const_FileNodeR(cv::face::FacemarkLBF::Params* instance, const cv::FileNode* unnamed, ResultVoid* ocvrs_return) {
		try {
			instance->read(*unnamed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkLBF.hpp:92
	// ("cv::face::FacemarkLBF::Params::write", vec![(pred!(const, ["unnamed"], ["cv::FileStorage*"]), _)]),
	void cv_face_FacemarkLBF_Params_write_const_FileStorageR(const cv::face::FacemarkLBF::Params* instance, cv::FileStorage* unnamed, ResultVoid* ocvrs_return) {
		try {
			instance->write(*unnamed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::face::FacemarkLBF::Params::shape_offset() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkLBF.hpp:58
	// ("cv::face::FacemarkLBF::Params::shape_offset", vec![(pred!(const, [], []), _)]),
	double cv_face_FacemarkLBF_Params_propShape_offset_const(const cv::face::FacemarkLBF::Params* instance) {
			double ret = instance->shape_offset;
			return ret;
	}

	// cv::face::FacemarkLBF::Params::setShape_offset(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkLBF.hpp:58
	// ("cv::face::FacemarkLBF::Params::setShape_offset", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_face_FacemarkLBF_Params_propShape_offset_const_double(cv::face::FacemarkLBF::Params* instance, const double val) {
			instance->shape_offset = val;
	}

	// cv::face::FacemarkLBF::Params::cascade_face() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkLBF.hpp:60
	// ("cv::face::FacemarkLBF::Params::cascade_face", vec![(pred!(const, [], []), _)]),
	void* cv_face_FacemarkLBF_Params_propCascade_face_const(const cv::face::FacemarkLBF::Params* instance) {
			cv::String ret = instance->cascade_face;
			return ocvrs_create_string(ret.c_str());
	}

	// cv::face::FacemarkLBF::Params::setCascade_face(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkLBF.hpp:60
	// ("cv::face::FacemarkLBF::Params::setCascade_face", vec![(pred!(mut, ["val"], ["const cv::String"]), _)]),
	void cv_face_FacemarkLBF_Params_propCascade_face_const_String(cv::face::FacemarkLBF::Params* instance, const char* val) {
			instance->cascade_face = cv::String(val);
	}

	// cv::face::FacemarkLBF::Params::verbose() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkLBF.hpp:62
	// ("cv::face::FacemarkLBF::Params::verbose", vec![(pred!(const, [], []), _)]),
	bool cv_face_FacemarkLBF_Params_propVerbose_const(const cv::face::FacemarkLBF::Params* instance) {
			bool ret = instance->verbose;
			return ret;
	}

	// cv::face::FacemarkLBF::Params::setVerbose(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkLBF.hpp:62
	// ("cv::face::FacemarkLBF::Params::setVerbose", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_face_FacemarkLBF_Params_propVerbose_const_bool(cv::face::FacemarkLBF::Params* instance, const bool val) {
			instance->verbose = val;
	}

	// cv::face::FacemarkLBF::Params::n_landmarks() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkLBF.hpp:65
	// ("cv::face::FacemarkLBF::Params::n_landmarks", vec![(pred!(const, [], []), _)]),
	int cv_face_FacemarkLBF_Params_propN_landmarks_const(const cv::face::FacemarkLBF::Params* instance) {
			int ret = instance->n_landmarks;
			return ret;
	}

	// cv::face::FacemarkLBF::Params::setN_landmarks(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkLBF.hpp:65
	// ("cv::face::FacemarkLBF::Params::setN_landmarks", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_face_FacemarkLBF_Params_propN_landmarks_const_int(cv::face::FacemarkLBF::Params* instance, const int val) {
			instance->n_landmarks = val;
	}

	// cv::face::FacemarkLBF::Params::initShape_n() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkLBF.hpp:67
	// ("cv::face::FacemarkLBF::Params::initShape_n", vec![(pred!(const, [], []), _)]),
	int cv_face_FacemarkLBF_Params_propInitShape_n_const(const cv::face::FacemarkLBF::Params* instance) {
			int ret = instance->initShape_n;
			return ret;
	}

	// cv::face::FacemarkLBF::Params::setInitShape_n(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkLBF.hpp:67
	// ("cv::face::FacemarkLBF::Params::setInitShape_n", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_face_FacemarkLBF_Params_propInitShape_n_const_int(cv::face::FacemarkLBF::Params* instance, const int val) {
			instance->initShape_n = val;
	}

	// cv::face::FacemarkLBF::Params::stages_n() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkLBF.hpp:70
	// ("cv::face::FacemarkLBF::Params::stages_n", vec![(pred!(const, [], []), _)]),
	int cv_face_FacemarkLBF_Params_propStages_n_const(const cv::face::FacemarkLBF::Params* instance) {
			int ret = instance->stages_n;
			return ret;
	}

	// cv::face::FacemarkLBF::Params::setStages_n(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkLBF.hpp:70
	// ("cv::face::FacemarkLBF::Params::setStages_n", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_face_FacemarkLBF_Params_propStages_n_const_int(cv::face::FacemarkLBF::Params* instance, const int val) {
			instance->stages_n = val;
	}

	// cv::face::FacemarkLBF::Params::tree_n() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkLBF.hpp:72
	// ("cv::face::FacemarkLBF::Params::tree_n", vec![(pred!(const, [], []), _)]),
	int cv_face_FacemarkLBF_Params_propTree_n_const(const cv::face::FacemarkLBF::Params* instance) {
			int ret = instance->tree_n;
			return ret;
	}

	// cv::face::FacemarkLBF::Params::setTree_n(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkLBF.hpp:72
	// ("cv::face::FacemarkLBF::Params::setTree_n", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_face_FacemarkLBF_Params_propTree_n_const_int(cv::face::FacemarkLBF::Params* instance, const int val) {
			instance->tree_n = val;
	}

	// cv::face::FacemarkLBF::Params::tree_depth() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkLBF.hpp:74
	// ("cv::face::FacemarkLBF::Params::tree_depth", vec![(pred!(const, [], []), _)]),
	int cv_face_FacemarkLBF_Params_propTree_depth_const(const cv::face::FacemarkLBF::Params* instance) {
			int ret = instance->tree_depth;
			return ret;
	}

	// cv::face::FacemarkLBF::Params::setTree_depth(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkLBF.hpp:74
	// ("cv::face::FacemarkLBF::Params::setTree_depth", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_face_FacemarkLBF_Params_propTree_depth_const_int(cv::face::FacemarkLBF::Params* instance, const int val) {
			instance->tree_depth = val;
	}

	// cv::face::FacemarkLBF::Params::bagging_overlap() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkLBF.hpp:76
	// ("cv::face::FacemarkLBF::Params::bagging_overlap", vec![(pred!(const, [], []), _)]),
	double cv_face_FacemarkLBF_Params_propBagging_overlap_const(const cv::face::FacemarkLBF::Params* instance) {
			double ret = instance->bagging_overlap;
			return ret;
	}

	// cv::face::FacemarkLBF::Params::setBagging_overlap(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkLBF.hpp:76
	// ("cv::face::FacemarkLBF::Params::setBagging_overlap", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_face_FacemarkLBF_Params_propBagging_overlap_const_double(cv::face::FacemarkLBF::Params* instance, const double val) {
			instance->bagging_overlap = val;
	}

	// cv::face::FacemarkLBF::Params::model_filename() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkLBF.hpp:79
	// ("cv::face::FacemarkLBF::Params::model_filename", vec![(pred!(const, [], []), _)]),
	void* cv_face_FacemarkLBF_Params_propModel_filename_const(const cv::face::FacemarkLBF::Params* instance) {
			std::string ret = instance->model_filename;
			return ocvrs_create_string(ret.c_str());
	}

	// cv::face::FacemarkLBF::Params::setModel_filename(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkLBF.hpp:79
	// ("cv::face::FacemarkLBF::Params::setModel_filename", vec![(pred!(mut, ["val"], ["const std::string"]), _)]),
	void cv_face_FacemarkLBF_Params_propModel_filename_const_string(cv::face::FacemarkLBF::Params* instance, const char* val) {
			instance->model_filename = std::string(val);
	}

	// cv::face::FacemarkLBF::Params::save_model() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkLBF.hpp:81
	// ("cv::face::FacemarkLBF::Params::save_model", vec![(pred!(const, [], []), _)]),
	bool cv_face_FacemarkLBF_Params_propSave_model_const(const cv::face::FacemarkLBF::Params* instance) {
			bool ret = instance->save_model;
			return ret;
	}

	// cv::face::FacemarkLBF::Params::setSave_model(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkLBF.hpp:81
	// ("cv::face::FacemarkLBF::Params::setSave_model", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_face_FacemarkLBF_Params_propSave_model_const_bool(cv::face::FacemarkLBF::Params* instance, const bool val) {
			instance->save_model = val;
	}

	// cv::face::FacemarkLBF::Params::seed() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkLBF.hpp:82
	// ("cv::face::FacemarkLBF::Params::seed", vec![(pred!(const, [], []), _)]),
	unsigned int cv_face_FacemarkLBF_Params_propSeed_const(const cv::face::FacemarkLBF::Params* instance) {
			unsigned int ret = instance->seed;
			return ret;
	}

	// cv::face::FacemarkLBF::Params::setSeed(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkLBF.hpp:82
	// ("cv::face::FacemarkLBF::Params::setSeed", vec![(pred!(mut, ["val"], ["const unsigned int"]), _)]),
	void cv_face_FacemarkLBF_Params_propSeed_const_unsigned_int(cv::face::FacemarkLBF::Params* instance, const unsigned int val) {
			instance->seed = val;
	}

	// cv::face::FacemarkLBF::Params::feats_m() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkLBF.hpp:84
	// ("cv::face::FacemarkLBF::Params::feats_m", vec![(pred!(const, [], []), _)]),
	std::vector<int>* cv_face_FacemarkLBF_Params_propFeats_m_const(const cv::face::FacemarkLBF::Params* instance) {
			std::vector<int> ret = instance->feats_m;
			return new std::vector<int>(ret);
	}

	// cv::face::FacemarkLBF::Params::setFeats_m(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkLBF.hpp:84
	// ("cv::face::FacemarkLBF::Params::setFeats_m", vec![(pred!(mut, ["val"], ["const std::vector<int>"]), _)]),
	void cv_face_FacemarkLBF_Params_propFeats_m_const_vectorLintG(cv::face::FacemarkLBF::Params* instance, const std::vector<int>* val) {
			instance->feats_m = *val;
	}

	// cv::face::FacemarkLBF::Params::radius_m() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkLBF.hpp:85
	// ("cv::face::FacemarkLBF::Params::radius_m", vec![(pred!(const, [], []), _)]),
	std::vector<double>* cv_face_FacemarkLBF_Params_propRadius_m_const(const cv::face::FacemarkLBF::Params* instance) {
			std::vector<double> ret = instance->radius_m;
			return new std::vector<double>(ret);
	}

	// cv::face::FacemarkLBF::Params::setRadius_m(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkLBF.hpp:85
	// ("cv::face::FacemarkLBF::Params::setRadius_m", vec![(pred!(mut, ["val"], ["const std::vector<double>"]), _)]),
	void cv_face_FacemarkLBF_Params_propRadius_m_const_vectorLdoubleG(cv::face::FacemarkLBF::Params* instance, const std::vector<double>* val) {
			instance->radius_m = *val;
	}

	// cv::face::FacemarkLBF::Params::detectROI() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkLBF.hpp:89
	// ("cv::face::FacemarkLBF::Params::detectROI", vec![(pred!(const, [], []), _)]),
	void cv_face_FacemarkLBF_Params_propDetectROI_const(const cv::face::FacemarkLBF::Params* instance, cv::Rect* ocvrs_return) {
			cv::Rect ret = instance->detectROI;
			*ocvrs_return = ret;
	}

	// cv::face::FacemarkLBF::Params::setDetectROI(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemarkLBF.hpp:89
	// ("cv::face::FacemarkLBF::Params::setDetectROI", vec![(pred!(mut, ["val"], ["const cv::Rect"]), _)]),
	void cv_face_FacemarkLBF_Params_propDetectROI_const_Rect(cv::face::FacemarkLBF::Params* instance, const cv::Rect* val) {
			instance->detectROI = *val;
	}

	// cv::face::FacemarkLBF::Params::delete() generated
	// ("cv::face::FacemarkLBF::Params::delete", vec![(pred!(mut, [], []), _)]),
	void cv_face_FacemarkLBF_Params_delete(cv::face::FacemarkLBF::Params* instance) {
			delete instance;
	}

	// addTrainingSample(InputArray, InputArray)(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemark_train.hpp:308
	// ("cv::face::FacemarkTrain::addTrainingSample", vec![(pred!(mut, ["image", "landmarks"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_face_FacemarkTrain_addTrainingSample_const__InputArrayR_const__InputArrayR(cv::face::FacemarkTrain* instance, const cv::_InputArray* image, const cv::_InputArray* landmarks, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->addTrainingSample(*image, *landmarks);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// training(void *)(Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemark_train.hpp:328
	// ("cv::face::FacemarkTrain::training", vec![(pred!(mut, ["parameters"], ["void*"]), _)]),
	void cv_face_FacemarkTrain_training_voidX(cv::face::FacemarkTrain* instance, void* parameters, ResultVoid* ocvrs_return) {
		try {
			instance->training(parameters);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::face::FacemarkTrain::training() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemark_train.hpp:328
	// ("cv::face::FacemarkTrain::training", vec![(pred!(mut, [], []), _)]),
	void cv_face_FacemarkTrain_training(cv::face::FacemarkTrain* instance, ResultVoid* ocvrs_return) {
		try {
			instance->training();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setFaceDetector(FN_FaceDetector, void *)(Function, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemark_train.hpp:351
	// ("cv::face::FacemarkTrain::setFaceDetector", vec![(pred!(mut, ["detector", "userData"], ["cv::face::FN_FaceDetector", "void*"]), _)]),
	void cv_face_FacemarkTrain_setFaceDetector_FN_FaceDetector_voidX(cv::face::FacemarkTrain* instance, cv::face::FN_FaceDetector detector, void* userData, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->setFaceDetector(detector, userData);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getFaces(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemark_train.hpp:368
	// ("cv::face::FacemarkTrain::getFaces", vec![(pred!(mut, ["image", "faces"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_face_FacemarkTrain_getFaces_const__InputArrayR_const__OutputArrayR(cv::face::FacemarkTrain* instance, const cv::_InputArray* image, const cv::_OutputArray* faces, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getFaces(*image, *faces);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getData(void *)(Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemark_train.hpp:386
	// ("cv::face::FacemarkTrain::getData", vec![(pred!(mut, ["items"], ["void*"]), _)]),
	void cv_face_FacemarkTrain_getData_voidX(cv::face::FacemarkTrain* instance, void* items, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getData(items);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::face::FacemarkTrain::getData() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facemark_train.hpp:386
	// ("cv::face::FacemarkTrain::getData", vec![(pred!(mut, [], []), _)]),
	void cv_face_FacemarkTrain_getData(cv::face::FacemarkTrain* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getData();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::face::FacemarkTrain::to_FacemarkAAM() generated
	// ("cv::face::FacemarkTrain::to_FacemarkAAM", vec![(pred!(mut, [], []), _)]),
	cv::face::FacemarkAAM* cv_face_FacemarkTrain_to_FacemarkAAM(cv::face::FacemarkTrain* instance) {
			return dynamic_cast<cv::face::FacemarkAAM*>(instance);
	}

	// cv::face::FacemarkTrain::to_FacemarkLBF() generated
	// ("cv::face::FacemarkTrain::to_FacemarkLBF", vec![(pred!(mut, [], []), _)]),
	cv::face::FacemarkLBF* cv_face_FacemarkTrain_to_FacemarkLBF(cv::face::FacemarkTrain* instance) {
			return dynamic_cast<cv::face::FacemarkLBF*>(instance);
	}

	// cv::face::FacemarkTrain::to_Algorithm() generated
	// ("cv::face::FacemarkTrain::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_face_FacemarkTrain_to_Algorithm(cv::face::FacemarkTrain* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::face::FacemarkTrain::to_Facemark() generated
	// ("cv::face::FacemarkTrain::to_Facemark", vec![(pred!(mut, [], []), _)]),
	cv::face::Facemark* cv_face_FacemarkTrain_to_Facemark(cv::face::FacemarkTrain* instance) {
			return dynamic_cast<cv::face::Facemark*>(instance);
	}

	// cv::face::FacemarkTrain::delete() generated
	// ("cv::face::FacemarkTrain::delete", vec![(pred!(mut, [], []), _)]),
	void cv_face_FacemarkTrain_delete(cv::face::FacemarkTrain* instance) {
			delete instance;
	}

	// create(int, double)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facerec.hpp:122
	// ("cv::face::FisherFaceRecognizer::create", vec![(pred!(mut, ["num_components", "threshold"], ["int", "double"]), _)]),
	void cv_face_FisherFaceRecognizer_create_int_double(int num_components, double threshold, Result<cv::Ptr<cv::face::FisherFaceRecognizer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::face::FisherFaceRecognizer> ret = cv::face::FisherFaceRecognizer::create(num_components, threshold);
			Ok(new cv::Ptr<cv::face::FisherFaceRecognizer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::face::FisherFaceRecognizer::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facerec.hpp:122
	// ("cv::face::FisherFaceRecognizer::create", vec![(pred!(mut, [], []), _)]),
	void cv_face_FisherFaceRecognizer_create(Result<cv::Ptr<cv::face::FisherFaceRecognizer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::face::FisherFaceRecognizer> ret = cv::face::FisherFaceRecognizer::create();
			Ok(new cv::Ptr<cv::face::FisherFaceRecognizer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::face::FisherFaceRecognizer::to_Algorithm() generated
	// ("cv::face::FisherFaceRecognizer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_face_FisherFaceRecognizer_to_Algorithm(cv::face::FisherFaceRecognizer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::face::FisherFaceRecognizer::to_BasicFaceRecognizer() generated
	// ("cv::face::FisherFaceRecognizer::to_BasicFaceRecognizer", vec![(pred!(mut, [], []), _)]),
	cv::face::BasicFaceRecognizer* cv_face_FisherFaceRecognizer_to_BasicFaceRecognizer(cv::face::FisherFaceRecognizer* instance) {
			return dynamic_cast<cv::face::BasicFaceRecognizer*>(instance);
	}

	// cv::face::FisherFaceRecognizer::to_FaceRecognizer() generated
	// ("cv::face::FisherFaceRecognizer::to_FaceRecognizer", vec![(pred!(mut, [], []), _)]),
	cv::face::FaceRecognizer* cv_face_FisherFaceRecognizer_to_FaceRecognizer(cv::face::FisherFaceRecognizer* instance) {
			return dynamic_cast<cv::face::FaceRecognizer*>(instance);
	}

	// cv::face::FisherFaceRecognizer::delete() generated
	// ("cv::face::FisherFaceRecognizer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_face_FisherFaceRecognizer_delete(cv::face::FisherFaceRecognizer* instance) {
			delete instance;
	}

	// getGridX()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facerec.hpp:130
	// ("cv::face::LBPHFaceRecognizer::getGridX", vec![(pred!(const, [], []), _)]),
	void cv_face_LBPHFaceRecognizer_getGridX_const(const cv::face::LBPHFaceRecognizer* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getGridX();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setGridX(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facerec.hpp:132
	// ("cv::face::LBPHFaceRecognizer::setGridX", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_face_LBPHFaceRecognizer_setGridX_int(cv::face::LBPHFaceRecognizer* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setGridX(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getGridY()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facerec.hpp:134
	// ("cv::face::LBPHFaceRecognizer::getGridY", vec![(pred!(const, [], []), _)]),
	void cv_face_LBPHFaceRecognizer_getGridY_const(const cv::face::LBPHFaceRecognizer* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getGridY();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setGridY(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facerec.hpp:136
	// ("cv::face::LBPHFaceRecognizer::setGridY", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_face_LBPHFaceRecognizer_setGridY_int(cv::face::LBPHFaceRecognizer* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setGridY(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getRadius()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facerec.hpp:138
	// ("cv::face::LBPHFaceRecognizer::getRadius", vec![(pred!(const, [], []), _)]),
	void cv_face_LBPHFaceRecognizer_getRadius_const(const cv::face::LBPHFaceRecognizer* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getRadius();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setRadius(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facerec.hpp:140
	// ("cv::face::LBPHFaceRecognizer::setRadius", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_face_LBPHFaceRecognizer_setRadius_int(cv::face::LBPHFaceRecognizer* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setRadius(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNeighbors()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facerec.hpp:142
	// ("cv::face::LBPHFaceRecognizer::getNeighbors", vec![(pred!(const, [], []), _)]),
	void cv_face_LBPHFaceRecognizer_getNeighbors_const(const cv::face::LBPHFaceRecognizer* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNeighbors();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNeighbors(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facerec.hpp:144
	// ("cv::face::LBPHFaceRecognizer::setNeighbors", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_face_LBPHFaceRecognizer_setNeighbors_int(cv::face::LBPHFaceRecognizer* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setNeighbors(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getThreshold()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facerec.hpp:146
	// ("cv::face::LBPHFaceRecognizer::getThreshold", vec![(pred!(const, [], []), _)]),
	void cv_face_LBPHFaceRecognizer_getThreshold_const(const cv::face::LBPHFaceRecognizer* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facerec.hpp:148
	// ("cv::face::LBPHFaceRecognizer::setThreshold", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_face_LBPHFaceRecognizer_setThreshold_double(cv::face::LBPHFaceRecognizer* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setThreshold(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getHistograms()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facerec.hpp:149
	// ("cv::face::LBPHFaceRecognizer::getHistograms", vec![(pred!(const, [], []), _)]),
	void cv_face_LBPHFaceRecognizer_getHistograms_const(const cv::face::LBPHFaceRecognizer* instance, Result<std::vector<cv::Mat>*>* ocvrs_return) {
		try {
			std::vector<cv::Mat> ret = instance->getHistograms();
			Ok(new std::vector<cv::Mat>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLabels()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facerec.hpp:150
	// ("cv::face::LBPHFaceRecognizer::getLabels", vec![(pred!(const, [], []), _)]),
	void cv_face_LBPHFaceRecognizer_getLabels_const(const cv::face::LBPHFaceRecognizer* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getLabels();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(int, int, int, int, double)(Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facerec.hpp:184
	// ("cv::face::LBPHFaceRecognizer::create", vec![(pred!(mut, ["radius", "neighbors", "grid_x", "grid_y", "threshold"], ["int", "int", "int", "int", "double"]), _)]),
	void cv_face_LBPHFaceRecognizer_create_int_int_int_int_double(int radius, int neighbors, int grid_x, int grid_y, double threshold, Result<cv::Ptr<cv::face::LBPHFaceRecognizer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::face::LBPHFaceRecognizer> ret = cv::face::LBPHFaceRecognizer::create(radius, neighbors, grid_x, grid_y, threshold);
			Ok(new cv::Ptr<cv::face::LBPHFaceRecognizer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::face::LBPHFaceRecognizer::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/facerec.hpp:184
	// ("cv::face::LBPHFaceRecognizer::create", vec![(pred!(mut, [], []), _)]),
	void cv_face_LBPHFaceRecognizer_create(Result<cv::Ptr<cv::face::LBPHFaceRecognizer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::face::LBPHFaceRecognizer> ret = cv::face::LBPHFaceRecognizer::create();
			Ok(new cv::Ptr<cv::face::LBPHFaceRecognizer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::face::LBPHFaceRecognizer::to_Algorithm() generated
	// ("cv::face::LBPHFaceRecognizer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_face_LBPHFaceRecognizer_to_Algorithm(cv::face::LBPHFaceRecognizer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::face::LBPHFaceRecognizer::to_FaceRecognizer() generated
	// ("cv::face::LBPHFaceRecognizer::to_FaceRecognizer", vec![(pred!(mut, [], []), _)]),
	cv::face::FaceRecognizer* cv_face_LBPHFaceRecognizer_to_FaceRecognizer(cv::face::LBPHFaceRecognizer* instance) {
			return dynamic_cast<cv::face::FaceRecognizer*>(instance);
	}

	// cv::face::LBPHFaceRecognizer::delete() generated
	// ("cv::face::LBPHFaceRecognizer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_face_LBPHFaceRecognizer_delete(cv::face::LBPHFaceRecognizer* instance) {
			delete instance;
	}

	// salt(const cv::String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/mace.hpp:78
	// ("cv::face::MACE::salt", vec![(pred!(mut, ["passphrase"], ["const cv::String*"]), _)]),
	void cv_face_MACE_salt_const_StringR(cv::face::MACE* instance, const char* passphrase, ResultVoid* ocvrs_return) {
		try {
			instance->salt(cv::String(passphrase));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// train(cv::InputArrayOfArrays)(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/mace.hpp:86
	// ("cv::face::MACE::train", vec![(pred!(mut, ["images"], ["const cv::_InputArray*"]), _)]),
	void cv_face_MACE_train_const__InputArrayR(cv::face::MACE* instance, const cv::_InputArray* images, ResultVoid* ocvrs_return) {
		try {
			instance->train(*images);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// same(cv::InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/mace.hpp:92
	// ("cv::face::MACE::same", vec![(pred!(const, ["query"], ["const cv::_InputArray*"]), _)]),
	void cv_face_MACE_same_const_const__InputArrayR(const cv::face::MACE* instance, const cv::_InputArray* query, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->same(*query);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// load(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/mace.hpp:100
	// ("cv::face::MACE::load", vec![(pred!(mut, ["filename", "objname"], ["const cv::String*", "const cv::String*"]), _)]),
	void cv_face_MACE_load_const_StringR_const_StringR(const char* filename, const char* objname, Result<cv::Ptr<cv::face::MACE>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::face::MACE> ret = cv::face::MACE::load(cv::String(filename), cv::String(objname));
			Ok(new cv::Ptr<cv::face::MACE>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::face::MACE::load(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/mace.hpp:100
	// ("cv::face::MACE::load", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
	void cv_face_MACE_load_const_StringR(const char* filename, Result<cv::Ptr<cv::face::MACE>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::face::MACE> ret = cv::face::MACE::load(cv::String(filename));
			Ok(new cv::Ptr<cv::face::MACE>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/mace.hpp:106
	// ("cv::face::MACE::create", vec![(pred!(mut, ["IMGSIZE"], ["int"]), _)]),
	void cv_face_MACE_create_int(int IMGSIZE, Result<cv::Ptr<cv::face::MACE>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::face::MACE> ret = cv::face::MACE::create(IMGSIZE);
			Ok(new cv::Ptr<cv::face::MACE>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::face::MACE::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/mace.hpp:106
	// ("cv::face::MACE::create", vec![(pred!(mut, [], []), _)]),
	void cv_face_MACE_create(Result<cv::Ptr<cv::face::MACE>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::face::MACE> ret = cv::face::MACE::create();
			Ok(new cv::Ptr<cv::face::MACE>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::face::MACE::to_Algorithm() generated
	// ("cv::face::MACE::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_face_MACE_to_Algorithm(cv::face::MACE* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::face::MACE::delete() generated
	// ("cv::face::MACE::delete", vec![(pred!(mut, [], []), _)]),
	void cv_face_MACE_delete(cv::face::MACE* instance) {
			delete instance;
	}

	// init(size_t)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/predict_collector.hpp:69
	// ("cv::face::PredictCollector::init", vec![(pred!(mut, ["size"], ["size_t"]), _)]),
	void cv_face_PredictCollector_init_size_t(cv::face::PredictCollector* instance, size_t size, ResultVoid* ocvrs_return) {
		try {
			instance->init(size);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// collect(int, double)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/predict_collector.hpp:75
	// ("cv::face::PredictCollector::collect", vec![(pred!(mut, ["label", "dist"], ["int", "double"]), _)]),
	void cv_face_PredictCollector_collect_int_double(cv::face::PredictCollector* instance, int label, double dist, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->collect(label, dist);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::face::PredictCollector::to_StandardCollector() generated
	// ("cv::face::PredictCollector::to_StandardCollector", vec![(pred!(mut, [], []), _)]),
	cv::face::StandardCollector* cv_face_PredictCollector_to_StandardCollector(cv::face::PredictCollector* instance) {
			return dynamic_cast<cv::face::StandardCollector*>(instance);
	}

	// cv::face::PredictCollector::delete() generated
	// ("cv::face::PredictCollector::delete", vec![(pred!(mut, [], []), _)]),
	void cv_face_PredictCollector_delete(cv::face::PredictCollector* instance) {
			delete instance;
	}

	// StandardCollector(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/predict_collector.hpp:99
	// ("cv::face::StandardCollector::StandardCollector", vec![(pred!(mut, ["threshold_"], ["double"]), _)]),
	void cv_face_StandardCollector_StandardCollector_double(double threshold_, Result<cv::face::StandardCollector*>* ocvrs_return) {
		try {
			cv::face::StandardCollector* ret = new cv::face::StandardCollector(threshold_);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::face::StandardCollector::StandardCollector() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/predict_collector.hpp:99
	// ("cv::face::StandardCollector::StandardCollector", vec![(pred!(mut, [], []), _)]),
	void cv_face_StandardCollector_StandardCollector(Result<cv::face::StandardCollector*>* ocvrs_return) {
		try {
			cv::face::StandardCollector* ret = new cv::face::StandardCollector();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// init(size_t)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/predict_collector.hpp:101
	// ("cv::face::StandardCollector::init", vec![(pred!(mut, ["size"], ["size_t"]), _)]),
	void cv_face_StandardCollector_init_size_t(cv::face::StandardCollector* instance, size_t size, ResultVoid* ocvrs_return) {
		try {
			instance->init(size);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// collect(int, double)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/predict_collector.hpp:103
	// ("cv::face::StandardCollector::collect", vec![(pred!(mut, ["label", "dist"], ["int", "double"]), _)]),
	void cv_face_StandardCollector_collect_int_double(cv::face::StandardCollector* instance, int label, double dist, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->collect(label, dist);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMinLabel()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/predict_collector.hpp:105
	// ("cv::face::StandardCollector::getMinLabel", vec![(pred!(const, [], []), _)]),
	void cv_face_StandardCollector_getMinLabel_const(const cv::face::StandardCollector* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMinLabel();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMinDist()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/predict_collector.hpp:107
	// ("cv::face::StandardCollector::getMinDist", vec![(pred!(const, [], []), _)]),
	void cv_face_StandardCollector_getMinDist_const(const cv::face::StandardCollector* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMinDist();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getResults(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/predict_collector.hpp:112
	// ("cv::face::StandardCollector::getResults", vec![(pred!(const, ["sorted"], ["bool"]), _)]),
	void cv_face_StandardCollector_getResults_const_bool(const cv::face::StandardCollector* instance, bool sorted, Result<std::vector<std::pair<int, double>>*>* ocvrs_return) {
		try {
			std::vector<std::pair<int, double>> ret = instance->getResults(sorted);
			Ok(new std::vector<std::pair<int, double>>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::face::StandardCollector::getResults() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/predict_collector.hpp:112
	// ("cv::face::StandardCollector::getResults", vec![(pred!(const, [], []), _)]),
	void cv_face_StandardCollector_getResults_const(const cv::face::StandardCollector* instance, Result<std::vector<std::pair<int, double>>*>* ocvrs_return) {
		try {
			std::vector<std::pair<int, double>> ret = instance->getResults();
			Ok(new std::vector<std::pair<int, double>>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/predict_collector.hpp:120
	// ("cv::face::StandardCollector::create", vec![(pred!(mut, ["threshold"], ["double"]), _)]),
	void cv_face_StandardCollector_create_double(double threshold, Result<cv::Ptr<cv::face::StandardCollector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::face::StandardCollector> ret = cv::face::StandardCollector::create(threshold);
			Ok(new cv::Ptr<cv::face::StandardCollector>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::face::StandardCollector::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/predict_collector.hpp:120
	// ("cv::face::StandardCollector::create", vec![(pred!(mut, [], []), _)]),
	void cv_face_StandardCollector_create(Result<cv::Ptr<cv::face::StandardCollector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::face::StandardCollector> ret = cv::face::StandardCollector::create();
			Ok(new cv::Ptr<cv::face::StandardCollector>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::face::StandardCollector::to_PredictCollector() generated
	// ("cv::face::StandardCollector::to_PredictCollector", vec![(pred!(mut, [], []), _)]),
	cv::face::PredictCollector* cv_face_StandardCollector_to_PredictCollector(cv::face::StandardCollector* instance) {
			return dynamic_cast<cv::face::PredictCollector*>(instance);
	}

	// cv::face::StandardCollector::delete() generated
	// ("cv::face::StandardCollector::delete", vec![(pred!(mut, [], []), _)]),
	void cv_face_StandardCollector_delete(cv::face::StandardCollector* instance) {
			delete instance;
	}

	// PredictResult(int, double)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/predict_collector.hpp:89
	// ("cv::face::StandardCollector::PredictResult::PredictResult", vec![(pred!(mut, ["label_", "distance_"], ["int", "double"]), _)]),
	void cv_face_StandardCollector_PredictResult_PredictResult_int_double(int label_, double distance_, Result<cv::face::StandardCollector::PredictResult>* ocvrs_return) {
		try {
			cv::face::StandardCollector::PredictResult ret(label_, distance_);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::face::StandardCollector::PredictResult::PredictResult() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/face/predict_collector.hpp:89
	// ("cv::face::StandardCollector::PredictResult::PredictResult", vec![(pred!(mut, [], []), _)]),
	void cv_face_StandardCollector_PredictResult_PredictResult(Result<cv::face::StandardCollector::PredictResult>* ocvrs_return) {
		try {
			cv::face::StandardCollector::PredictResult ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

}
