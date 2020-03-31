template struct Result<bool>;
template struct Result<cv::Ptr<cv::dpm::DPMDetector>*>;
template struct Result<cv::Rect_<int>>;
template struct Result<cv::dpm::DPMDetector::ObjectDetection*>;
template struct Result<float>;
template struct Result<int>;
template struct Result<std::vector<cv::dpm::DPMDetector::ObjectDetection>*>;
template struct Result<std::vector<std::string>*>;
template struct Result<unsigned long>;
extern "C" void cv_PtrOfDPMDetector_delete(cv::Ptr<cv::dpm::DPMDetector>* instance) {
	delete instance;
}

extern "C" cv::dpm::DPMDetector* cv_PtrOfDPMDetector_get_inner_ptr(cv::Ptr<cv::dpm::DPMDetector>* instance) {
	return instance->get();
}

extern "C" {
	void cv_VectorOfDPMDetector_ObjectDetection_delete(std::vector<cv::dpm::DPMDetector::ObjectDetection>* instance) {
		delete instance;
	}

	std::vector<cv::dpm::DPMDetector::ObjectDetection>* cv_VectorOfDPMDetector_ObjectDetection_new() {
		return new std::vector<cv::dpm::DPMDetector::ObjectDetection>();
	}

	size_t cv_VectorOfDPMDetector_ObjectDetection_len(const std::vector<cv::dpm::DPMDetector::ObjectDetection>* instance) {
		return instance->size();
	}

	bool cv_VectorOfDPMDetector_ObjectDetection_is_empty(const std::vector<cv::dpm::DPMDetector::ObjectDetection>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfDPMDetector_ObjectDetection_capacity(const std::vector<cv::dpm::DPMDetector::ObjectDetection>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfDPMDetector_ObjectDetection_shrink_to_fit(std::vector<cv::dpm::DPMDetector::ObjectDetection>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfDPMDetector_ObjectDetection_reserve(std::vector<cv::dpm::DPMDetector::ObjectDetection>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfDPMDetector_ObjectDetection_remove(std::vector<cv::dpm::DPMDetector::ObjectDetection>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfDPMDetector_ObjectDetection_swap(std::vector<cv::dpm::DPMDetector::ObjectDetection>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfDPMDetector_ObjectDetection_clear(std::vector<cv::dpm::DPMDetector::ObjectDetection>* instance) {
		instance->clear();
	}

	void cv_VectorOfDPMDetector_ObjectDetection_push(std::vector<cv::dpm::DPMDetector::ObjectDetection>* instance, cv::dpm::DPMDetector::ObjectDetection* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfDPMDetector_ObjectDetection_insert(std::vector<cv::dpm::DPMDetector::ObjectDetection>* instance, size_t index, cv::dpm::DPMDetector::ObjectDetection* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<cv::dpm::DPMDetector::ObjectDetection*> cv_VectorOfDPMDetector_ObjectDetection_get(const std::vector<cv::dpm::DPMDetector::ObjectDetection>* instance, size_t index) {
		return Ok<cv::dpm::DPMDetector::ObjectDetection*>(new cv::dpm::DPMDetector::ObjectDetection((*instance)[index]));
	}

	void cv_VectorOfDPMDetector_ObjectDetection_set(std::vector<cv::dpm::DPMDetector::ObjectDetection>* instance, size_t index, cv::dpm::DPMDetector::ObjectDetection* val) {
		(*instance)[index] = *val;
	}

}


