template struct Result<bool>;
template struct Result<cv::DetectionBasedTracker::ObjectStatus>;
template struct Result<cv::Rect_<int>>;
template struct Result<cv::Size_<int>>;
template struct Result<double>;
template struct Result<float>;
template struct Result<int>;
template struct Result<unsigned long>;
template struct Result<void*>;
extern "C" void cv_PtrOfBaseCascadeClassifier_delete(cv::Ptr<cv::BaseCascadeClassifier>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfBaseCascadeClassifier_get_inner_ptr(cv::Ptr<cv::BaseCascadeClassifier>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfBaseCascadeClassifier_MaskGenerator_delete(cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfBaseCascadeClassifier_MaskGenerator_get_inner_ptr(cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfDetectionBasedTracker_IDetector_delete(cv::Ptr<cv::DetectionBasedTracker::IDetector>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfDetectionBasedTracker_IDetector_get_inner_ptr(cv::Ptr<cv::DetectionBasedTracker::IDetector>* instance) {
	return instance->get();
}

extern "C" {
	void cv_VectorOfDetectionBasedTracker_ExtObject_delete(std::vector<cv::DetectionBasedTracker::ExtObject>* instance) {
		delete instance;
	}

	void* cv_VectorOfDetectionBasedTracker_ExtObject_new() {
		return new std::vector<cv::DetectionBasedTracker::ExtObject>();
	}

	size_t cv_VectorOfDetectionBasedTracker_ExtObject_len(const std::vector<cv::DetectionBasedTracker::ExtObject>* instance) {
		return instance->size();
	}

	bool cv_VectorOfDetectionBasedTracker_ExtObject_is_empty(const std::vector<cv::DetectionBasedTracker::ExtObject>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfDetectionBasedTracker_ExtObject_capacity(const std::vector<cv::DetectionBasedTracker::ExtObject>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfDetectionBasedTracker_ExtObject_shrink_to_fit(std::vector<cv::DetectionBasedTracker::ExtObject>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfDetectionBasedTracker_ExtObject_reserve(std::vector<cv::DetectionBasedTracker::ExtObject>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfDetectionBasedTracker_ExtObject_remove(std::vector<cv::DetectionBasedTracker::ExtObject>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfDetectionBasedTracker_ExtObject_swap(std::vector<cv::DetectionBasedTracker::ExtObject>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfDetectionBasedTracker_ExtObject_clear(std::vector<cv::DetectionBasedTracker::ExtObject>* instance) {
		instance->clear();
	}

	void cv_VectorOfDetectionBasedTracker_ExtObject_push(std::vector<cv::DetectionBasedTracker::ExtObject>* instance, cv::DetectionBasedTracker::ExtObject* val) {
		instance->push_back(*val);
	}
	
	void cv_VectorOfDetectionBasedTracker_ExtObject_insert(std::vector<cv::DetectionBasedTracker::ExtObject>* instance, size_t index, cv::DetectionBasedTracker::ExtObject* val) {
		instance->insert(instance->begin() + index, *val);
	}
	
	Result<void*> cv_VectorOfDetectionBasedTracker_ExtObject_get(const std::vector<cv::DetectionBasedTracker::ExtObject>* instance, size_t index) {
		try {
			return Ok<void*>(new cv::DetectionBasedTracker::ExtObject(instance->at(index)));
		} VEC_CATCH(Result<void*>)
	}
	
	void* cv_VectorOfDetectionBasedTracker_ExtObject_get_unchecked(const std::vector<cv::DetectionBasedTracker::ExtObject>* instance, size_t index) {
		return new cv::DetectionBasedTracker::ExtObject((*instance)[index]);
	}
	
	Result_void cv_VectorOfDetectionBasedTracker_ExtObject_set(std::vector<cv::DetectionBasedTracker::ExtObject>* instance, size_t index, cv::DetectionBasedTracker::ExtObject* val) {
		try {
			instance->at(index) = *val;
			return Ok();
		} VEC_CATCH(Result_void)
	}
	
	void cv_VectorOfDetectionBasedTracker_ExtObject_set_unchecked(std::vector<cv::DetectionBasedTracker::ExtObject>* instance, size_t index, cv::DetectionBasedTracker::ExtObject* val) {
		(*instance)[index] = *val;
	}
	
}


extern "C" {
	void cv_VectorOfDetectionROI_delete(std::vector<cv::DetectionROI>* instance) {
		delete instance;
	}

	void* cv_VectorOfDetectionROI_new() {
		return new std::vector<cv::DetectionROI>();
	}

	size_t cv_VectorOfDetectionROI_len(const std::vector<cv::DetectionROI>* instance) {
		return instance->size();
	}

	bool cv_VectorOfDetectionROI_is_empty(const std::vector<cv::DetectionROI>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfDetectionROI_capacity(const std::vector<cv::DetectionROI>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfDetectionROI_shrink_to_fit(std::vector<cv::DetectionROI>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfDetectionROI_reserve(std::vector<cv::DetectionROI>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfDetectionROI_remove(std::vector<cv::DetectionROI>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfDetectionROI_swap(std::vector<cv::DetectionROI>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfDetectionROI_clear(std::vector<cv::DetectionROI>* instance) {
		instance->clear();
	}

	void cv_VectorOfDetectionROI_push(std::vector<cv::DetectionROI>* instance, cv::DetectionROI* val) {
		instance->push_back(*val);
	}
	
	void cv_VectorOfDetectionROI_insert(std::vector<cv::DetectionROI>* instance, size_t index, cv::DetectionROI* val) {
		instance->insert(instance->begin() + index, *val);
	}
	
	Result<void*> cv_VectorOfDetectionROI_get(const std::vector<cv::DetectionROI>* instance, size_t index) {
		try {
			return Ok<void*>(new cv::DetectionROI(instance->at(index)));
		} VEC_CATCH(Result<void*>)
	}
	
	void* cv_VectorOfDetectionROI_get_unchecked(const std::vector<cv::DetectionROI>* instance, size_t index) {
		return new cv::DetectionROI((*instance)[index]);
	}
	
	Result_void cv_VectorOfDetectionROI_set(std::vector<cv::DetectionROI>* instance, size_t index, cv::DetectionROI* val) {
		try {
			instance->at(index) = *val;
			return Ok();
		} VEC_CATCH(Result_void)
	}
	
	void cv_VectorOfDetectionROI_set_unchecked(std::vector<cv::DetectionROI>* instance, size_t index, cv::DetectionROI* val) {
		(*instance)[index] = *val;
	}
	
}


