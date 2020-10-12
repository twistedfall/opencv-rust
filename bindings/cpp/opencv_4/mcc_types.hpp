template struct Result<bool>;
template struct Result<cv::Mat*>;
template struct Result<cv::Point_<float>>;
template struct Result<cv::Ptr<cv::mcc::CCheckerDetector>*>;
template struct Result<cv::Ptr<cv::mcc::CCheckerDraw>*>;
template struct Result<cv::Ptr<cv::mcc::CChecker>*>;
template struct Result<cv::Ptr<cv::mcc::DetectorParameters>*>;
template struct Result<cv::Rect_<int>>;
template struct Result<cv::mcc::DetectorParameters*>;
template struct Result<cv::mcc::TYPECHART>;
template struct Result<double>;
template struct Result<float>;
template struct Result<int>;
template struct Result<std::vector<cv::Point_<float>>*>;
template struct Result<std::vector<cv::Ptr<cv::mcc::CChecker>>*>;
template struct Result<unsigned int>;
extern "C" {
	void cv_PtrOfMCC_CChecker_delete(cv::Ptr<cv::mcc::CChecker>* instance) {
		delete instance;
	}

	const cv::mcc::CChecker* cv_PtrOfMCC_CChecker_get_inner_ptr(const cv::Ptr<cv::mcc::CChecker>* instance) {
		return instance->get();
	}

	cv::mcc::CChecker* cv_PtrOfMCC_CChecker_get_inner_ptr_mut(cv::Ptr<cv::mcc::CChecker>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfMCC_CCheckerDetector_delete(cv::Ptr<cv::mcc::CCheckerDetector>* instance) {
		delete instance;
	}

	const cv::mcc::CCheckerDetector* cv_PtrOfMCC_CCheckerDetector_get_inner_ptr(const cv::Ptr<cv::mcc::CCheckerDetector>* instance) {
		return instance->get();
	}

	cv::mcc::CCheckerDetector* cv_PtrOfMCC_CCheckerDetector_get_inner_ptr_mut(cv::Ptr<cv::mcc::CCheckerDetector>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfMCC_CCheckerDraw_delete(cv::Ptr<cv::mcc::CCheckerDraw>* instance) {
		delete instance;
	}

	const cv::mcc::CCheckerDraw* cv_PtrOfMCC_CCheckerDraw_get_inner_ptr(const cv::Ptr<cv::mcc::CCheckerDraw>* instance) {
		return instance->get();
	}

	cv::mcc::CCheckerDraw* cv_PtrOfMCC_CCheckerDraw_get_inner_ptr_mut(cv::Ptr<cv::mcc::CCheckerDraw>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::mcc::DetectorParameters>* cv_PtrOfMCC_DetectorParameters_new(cv::mcc::DetectorParameters* val) {
		return new cv::Ptr<cv::mcc::DetectorParameters>(val);
	}
	
	void cv_PtrOfMCC_DetectorParameters_delete(cv::Ptr<cv::mcc::DetectorParameters>* instance) {
		delete instance;
	}

	const cv::mcc::DetectorParameters* cv_PtrOfMCC_DetectorParameters_get_inner_ptr(const cv::Ptr<cv::mcc::DetectorParameters>* instance) {
		return instance->get();
	}

	cv::mcc::DetectorParameters* cv_PtrOfMCC_DetectorParameters_get_inner_ptr_mut(cv::Ptr<cv::mcc::DetectorParameters>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_VectorOfPtrOfMCC_CChecker_delete(std::vector<cv::Ptr<cv::mcc::CChecker>>* instance) {
		delete instance;
	}

	std::vector<cv::Ptr<cv::mcc::CChecker>>* cv_VectorOfPtrOfMCC_CChecker_new() {
		return new std::vector<cv::Ptr<cv::mcc::CChecker>>();
	}

	size_t cv_VectorOfPtrOfMCC_CChecker_len(const std::vector<cv::Ptr<cv::mcc::CChecker>>* instance) {
		return instance->size();
	}

	bool cv_VectorOfPtrOfMCC_CChecker_is_empty(const std::vector<cv::Ptr<cv::mcc::CChecker>>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfPtrOfMCC_CChecker_capacity(const std::vector<cv::Ptr<cv::mcc::CChecker>>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfPtrOfMCC_CChecker_shrink_to_fit(std::vector<cv::Ptr<cv::mcc::CChecker>>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfPtrOfMCC_CChecker_reserve(std::vector<cv::Ptr<cv::mcc::CChecker>>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfPtrOfMCC_CChecker_remove(std::vector<cv::Ptr<cv::mcc::CChecker>>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfPtrOfMCC_CChecker_swap(std::vector<cv::Ptr<cv::mcc::CChecker>>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfPtrOfMCC_CChecker_clear(std::vector<cv::Ptr<cv::mcc::CChecker>>* instance) {
		instance->clear();
	}

	void cv_VectorOfPtrOfMCC_CChecker_push(std::vector<cv::Ptr<cv::mcc::CChecker>>* instance, cv::Ptr<cv::mcc::CChecker>* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfPtrOfMCC_CChecker_insert(std::vector<cv::Ptr<cv::mcc::CChecker>>* instance, size_t index, cv::Ptr<cv::mcc::CChecker>* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<cv::Ptr<cv::mcc::CChecker>*> cv_VectorOfPtrOfMCC_CChecker_get(const std::vector<cv::Ptr<cv::mcc::CChecker>>* instance, size_t index) {
		return Ok<cv::Ptr<cv::mcc::CChecker>*>(new cv::Ptr<cv::mcc::CChecker>((*instance)[index]));
	}

	void cv_VectorOfPtrOfMCC_CChecker_set(std::vector<cv::Ptr<cv::mcc::CChecker>>* instance, size_t index, cv::Ptr<cv::mcc::CChecker>* val) {
		(*instance)[index] = *val;
	}

}


