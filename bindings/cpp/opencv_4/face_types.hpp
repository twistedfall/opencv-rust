template struct Result<bool>;
template struct Result<cv::CascadeClassifier*>;
template struct Result<cv::Mat*>;
template struct Result<cv::Point_<float>>;
template struct Result<cv::Ptr<cv::face::BIF>*>;
template struct Result<cv::Ptr<cv::face::EigenFaceRecognizer>*>;
template struct Result<cv::Ptr<cv::face::FacemarkAAM>*>;
template struct Result<cv::Ptr<cv::face::FacemarkKazemi>*>;
template struct Result<cv::Ptr<cv::face::FacemarkLBF>*>;
template struct Result<cv::Ptr<cv::face::Facemark>*>;
template struct Result<cv::Ptr<cv::face::FisherFaceRecognizer>*>;
template struct Result<cv::Ptr<cv::face::LBPHFaceRecognizer>*>;
template struct Result<cv::Ptr<cv::face::MACE>*>;
template struct Result<cv::Ptr<cv::face::PredictCollector>*>;
template struct Result<cv::Ptr<cv::face::StandardCollector>*>;
template struct Result<cv::Rect_<int>>;
template struct Result<cv::Size_<int>>;
template struct Result<cv::face::CParams*>;
template struct Result<cv::face::FacemarkAAM::Config*>;
template struct Result<cv::face::FacemarkAAM::Model::Texture*>;
template struct Result<cv::face::FacemarkAAM::Params*>;
template struct Result<cv::face::FacemarkKazemi::Params*>;
template struct Result<cv::face::FacemarkLBF::Params*>;
template struct Result<cv::face::StandardCollector*>;
template struct Result<cv::face::StandardCollector::PredictResult>;
template struct Result<double>;
template struct Result<float>;
template struct Result<int>;
template struct Result<std::vector<cv::Mat>*>;
template struct Result<std::vector<cv::Point_<float>>*>;
template struct Result<std::vector<cv::Point_<int>>*>;
template struct Result<std::vector<cv::Vec<int, 3>>*>;
template struct Result<std::vector<cv::face::FacemarkAAM::Config>*>;
template struct Result<std::vector<cv::face::FacemarkAAM::Model::Texture>*>;
template struct Result<std::vector<double>*>;
template struct Result<std::vector<float>*>;
template struct Result<std::vector<int>*>;
template struct Result<std::vector<int>(*)[2]>;
template struct Result<std::vector<std::string>*>;
template struct Result<std::vector<std::vector<cv::Point_<float>>>*>;
template struct Result<std::vector<std::vector<cv::Point_<int>>>*>;
template struct Result<unsigned int>;
template struct Result<unsigned long>;
template struct Result<void*>;
extern "C" void cv_PtrOfBIF_delete(cv::Ptr<cv::face::BIF>* instance) {
	delete instance;
}

extern "C" cv::face::BIF* cv_PtrOfBIF_get_inner_ptr(cv::Ptr<cv::face::BIF>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfEigenFaceRecognizer_delete(cv::Ptr<cv::face::EigenFaceRecognizer>* instance) {
	delete instance;
}

extern "C" cv::face::EigenFaceRecognizer* cv_PtrOfEigenFaceRecognizer_get_inner_ptr(cv::Ptr<cv::face::EigenFaceRecognizer>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfFacemark_delete(cv::Ptr<cv::face::Facemark>* instance) {
	delete instance;
}

extern "C" cv::face::Facemark* cv_PtrOfFacemark_get_inner_ptr(cv::Ptr<cv::face::Facemark>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfFacemarkAAM_delete(cv::Ptr<cv::face::FacemarkAAM>* instance) {
	delete instance;
}

extern "C" cv::face::FacemarkAAM* cv_PtrOfFacemarkAAM_get_inner_ptr(cv::Ptr<cv::face::FacemarkAAM>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfFacemarkKazemi_delete(cv::Ptr<cv::face::FacemarkKazemi>* instance) {
	delete instance;
}

extern "C" cv::face::FacemarkKazemi* cv_PtrOfFacemarkKazemi_get_inner_ptr(cv::Ptr<cv::face::FacemarkKazemi>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfFacemarkLBF_delete(cv::Ptr<cv::face::FacemarkLBF>* instance) {
	delete instance;
}

extern "C" cv::face::FacemarkLBF* cv_PtrOfFacemarkLBF_get_inner_ptr(cv::Ptr<cv::face::FacemarkLBF>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfFisherFaceRecognizer_delete(cv::Ptr<cv::face::FisherFaceRecognizer>* instance) {
	delete instance;
}

extern "C" cv::face::FisherFaceRecognizer* cv_PtrOfFisherFaceRecognizer_get_inner_ptr(cv::Ptr<cv::face::FisherFaceRecognizer>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfLBPHFaceRecognizer_delete(cv::Ptr<cv::face::LBPHFaceRecognizer>* instance) {
	delete instance;
}

extern "C" cv::face::LBPHFaceRecognizer* cv_PtrOfLBPHFaceRecognizer_get_inner_ptr(cv::Ptr<cv::face::LBPHFaceRecognizer>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfMACE_delete(cv::Ptr<cv::face::MACE>* instance) {
	delete instance;
}

extern "C" cv::face::MACE* cv_PtrOfMACE_get_inner_ptr(cv::Ptr<cv::face::MACE>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfPredictCollector_delete(cv::Ptr<cv::face::PredictCollector>* instance) {
	delete instance;
}

extern "C" cv::face::PredictCollector* cv_PtrOfPredictCollector_get_inner_ptr(cv::Ptr<cv::face::PredictCollector>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfStandardCollector_delete(cv::Ptr<cv::face::StandardCollector>* instance) {
	delete instance;
}

extern "C" cv::face::StandardCollector* cv_PtrOfStandardCollector_get_inner_ptr(cv::Ptr<cv::face::StandardCollector>* instance) {
	return instance->get();
}

extern "C" {
	void cv_VectorOfFacemarkAAM_Config_delete(std::vector<cv::face::FacemarkAAM::Config>* instance) {
		delete instance;
	}

	std::vector<cv::face::FacemarkAAM::Config>* cv_VectorOfFacemarkAAM_Config_new() {
		return new std::vector<cv::face::FacemarkAAM::Config>();
	}

	size_t cv_VectorOfFacemarkAAM_Config_len(const std::vector<cv::face::FacemarkAAM::Config>* instance) {
		return instance->size();
	}

	bool cv_VectorOfFacemarkAAM_Config_is_empty(const std::vector<cv::face::FacemarkAAM::Config>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfFacemarkAAM_Config_capacity(const std::vector<cv::face::FacemarkAAM::Config>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfFacemarkAAM_Config_shrink_to_fit(std::vector<cv::face::FacemarkAAM::Config>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfFacemarkAAM_Config_reserve(std::vector<cv::face::FacemarkAAM::Config>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfFacemarkAAM_Config_remove(std::vector<cv::face::FacemarkAAM::Config>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfFacemarkAAM_Config_swap(std::vector<cv::face::FacemarkAAM::Config>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfFacemarkAAM_Config_clear(std::vector<cv::face::FacemarkAAM::Config>* instance) {
		instance->clear();
	}

	void cv_VectorOfFacemarkAAM_Config_push(std::vector<cv::face::FacemarkAAM::Config>* instance, cv::face::FacemarkAAM::Config* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfFacemarkAAM_Config_insert(std::vector<cv::face::FacemarkAAM::Config>* instance, size_t index, cv::face::FacemarkAAM::Config* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<cv::face::FacemarkAAM::Config*> cv_VectorOfFacemarkAAM_Config_get(const std::vector<cv::face::FacemarkAAM::Config>* instance, size_t index) {
		return Ok<cv::face::FacemarkAAM::Config*>(new cv::face::FacemarkAAM::Config((*instance)[index]));
	}

	void cv_VectorOfFacemarkAAM_Config_set(std::vector<cv::face::FacemarkAAM::Config>* instance, size_t index, cv::face::FacemarkAAM::Config* val) {
		(*instance)[index] = *val;
	}

}


extern "C" {
	void cv_VectorOfFacemarkAAM_Model_Texture_delete(std::vector<cv::face::FacemarkAAM::Model::Texture>* instance) {
		delete instance;
	}

	std::vector<cv::face::FacemarkAAM::Model::Texture>* cv_VectorOfFacemarkAAM_Model_Texture_new() {
		return new std::vector<cv::face::FacemarkAAM::Model::Texture>();
	}

	size_t cv_VectorOfFacemarkAAM_Model_Texture_len(const std::vector<cv::face::FacemarkAAM::Model::Texture>* instance) {
		return instance->size();
	}

	bool cv_VectorOfFacemarkAAM_Model_Texture_is_empty(const std::vector<cv::face::FacemarkAAM::Model::Texture>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfFacemarkAAM_Model_Texture_capacity(const std::vector<cv::face::FacemarkAAM::Model::Texture>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfFacemarkAAM_Model_Texture_shrink_to_fit(std::vector<cv::face::FacemarkAAM::Model::Texture>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfFacemarkAAM_Model_Texture_reserve(std::vector<cv::face::FacemarkAAM::Model::Texture>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfFacemarkAAM_Model_Texture_remove(std::vector<cv::face::FacemarkAAM::Model::Texture>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfFacemarkAAM_Model_Texture_swap(std::vector<cv::face::FacemarkAAM::Model::Texture>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfFacemarkAAM_Model_Texture_clear(std::vector<cv::face::FacemarkAAM::Model::Texture>* instance) {
		instance->clear();
	}

	void cv_VectorOfFacemarkAAM_Model_Texture_push(std::vector<cv::face::FacemarkAAM::Model::Texture>* instance, cv::face::FacemarkAAM::Model::Texture* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfFacemarkAAM_Model_Texture_insert(std::vector<cv::face::FacemarkAAM::Model::Texture>* instance, size_t index, cv::face::FacemarkAAM::Model::Texture* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<cv::face::FacemarkAAM::Model::Texture*> cv_VectorOfFacemarkAAM_Model_Texture_get(const std::vector<cv::face::FacemarkAAM::Model::Texture>* instance, size_t index) {
		return Ok<cv::face::FacemarkAAM::Model::Texture*>(new cv::face::FacemarkAAM::Model::Texture((*instance)[index]));
	}

	void cv_VectorOfFacemarkAAM_Model_Texture_set(std::vector<cv::face::FacemarkAAM::Model::Texture>* instance, size_t index, cv::face::FacemarkAAM::Model::Texture* val) {
		(*instance)[index] = *val;
	}

}


