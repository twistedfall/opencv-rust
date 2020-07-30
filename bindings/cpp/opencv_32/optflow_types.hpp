template struct Result<bool>;
template struct Result<const std::vector<cv::String>*>;
template struct Result<cv::Mat*>;
template struct Result<cv::Ptr<const cv::optflow::PCAPrior>*>;
template struct Result<cv::Ptr<cv::DenseOpticalFlow>*>;
template struct Result<cv::Ptr<cv::optflow::DISOpticalFlow>*>;
template struct Result<cv::Ptr<cv::optflow::GPCTrainingSamples>*>;
template struct Result<cv::Ptr<cv::optflow::GPCTree>*>;
template struct Result<cv::Ptr<cv::optflow::VariationalRefinement>*>;
template struct Result<cv::Vec<double, 18>>;
template struct Result<cv::optflow::GPCMatchingParams>;
template struct Result<cv::optflow::GPCPatchDescriptor*>;
template struct Result<cv::optflow::GPCTrainingParams>;
template struct Result<cv::optflow::OpticalFlowPCAFlow*>;
template struct Result<cv::optflow::PCAPrior*>;
template struct Result<double>;
template struct Result<float>;
template struct Result<int>;
template struct Result<std::vector<cv::Rect_<int>>*>;
template struct Result<std::vector<cv::optflow::GPCPatchDescriptor>*>;
template struct Result<unsigned int>;
template struct Result<unsigned long>;
extern "C" {
	void cv_PtrOfDISOpticalFlow_delete(cv::Ptr<cv::optflow::DISOpticalFlow>* instance) {
		delete instance;
	}

	cv::optflow::DISOpticalFlow* cv_PtrOfDISOpticalFlow_get_inner_ptr(cv::Ptr<cv::optflow::DISOpticalFlow>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::optflow::GPCTrainingSamples>* cv_PtrOfGPCTrainingSamples_new(cv::optflow::GPCTrainingSamples* val) {
		return new cv::Ptr<cv::optflow::GPCTrainingSamples>(val);
	}
	
	void cv_PtrOfGPCTrainingSamples_delete(cv::Ptr<cv::optflow::GPCTrainingSamples>* instance) {
		delete instance;
	}

	cv::optflow::GPCTrainingSamples* cv_PtrOfGPCTrainingSamples_get_inner_ptr(cv::Ptr<cv::optflow::GPCTrainingSamples>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<cv::optflow::GPCTree>* cv_PtrOfGPCTree_new(cv::optflow::GPCTree* val) {
		return new cv::Ptr<cv::optflow::GPCTree>(val);
	}
	
	void cv_PtrOfGPCTree_delete(cv::Ptr<cv::optflow::GPCTree>* instance) {
		delete instance;
	}

	cv::optflow::GPCTree* cv_PtrOfGPCTree_get_inner_ptr(cv::Ptr<cv::optflow::GPCTree>* instance) {
		return instance->get();
	}
}

extern "C" {
	cv::Ptr<const cv::optflow::PCAPrior>* cv_PtrOfPCAPrior_new(const cv::optflow::PCAPrior* val) {
		return new cv::Ptr<const cv::optflow::PCAPrior>(val);
	}
	
	void cv_PtrOfPCAPrior_delete(cv::Ptr<const cv::optflow::PCAPrior>* instance) {
		delete instance;
	}

	const cv::optflow::PCAPrior* cv_PtrOfPCAPrior_get_inner_ptr(cv::Ptr<const cv::optflow::PCAPrior>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfVariationalRefinement_delete(cv::Ptr<cv::optflow::VariationalRefinement>* instance) {
		delete instance;
	}

	cv::optflow::VariationalRefinement* cv_PtrOfVariationalRefinement_get_inner_ptr(cv::Ptr<cv::optflow::VariationalRefinement>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_VectorOfGPCPatchDescriptor_delete(std::vector<cv::optflow::GPCPatchDescriptor>* instance) {
		delete instance;
	}

	std::vector<cv::optflow::GPCPatchDescriptor>* cv_VectorOfGPCPatchDescriptor_new() {
		return new std::vector<cv::optflow::GPCPatchDescriptor>();
	}

	size_t cv_VectorOfGPCPatchDescriptor_len(const std::vector<cv::optflow::GPCPatchDescriptor>* instance) {
		return instance->size();
	}

	bool cv_VectorOfGPCPatchDescriptor_is_empty(const std::vector<cv::optflow::GPCPatchDescriptor>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfGPCPatchDescriptor_capacity(const std::vector<cv::optflow::GPCPatchDescriptor>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfGPCPatchDescriptor_shrink_to_fit(std::vector<cv::optflow::GPCPatchDescriptor>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfGPCPatchDescriptor_reserve(std::vector<cv::optflow::GPCPatchDescriptor>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfGPCPatchDescriptor_remove(std::vector<cv::optflow::GPCPatchDescriptor>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfGPCPatchDescriptor_swap(std::vector<cv::optflow::GPCPatchDescriptor>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfGPCPatchDescriptor_clear(std::vector<cv::optflow::GPCPatchDescriptor>* instance) {
		instance->clear();
	}

	void cv_VectorOfGPCPatchDescriptor_push(std::vector<cv::optflow::GPCPatchDescriptor>* instance, cv::optflow::GPCPatchDescriptor* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfGPCPatchDescriptor_insert(std::vector<cv::optflow::GPCPatchDescriptor>* instance, size_t index, cv::optflow::GPCPatchDescriptor* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<cv::optflow::GPCPatchDescriptor*> cv_VectorOfGPCPatchDescriptor_get(const std::vector<cv::optflow::GPCPatchDescriptor>* instance, size_t index) {
		return Ok<cv::optflow::GPCPatchDescriptor*>(new cv::optflow::GPCPatchDescriptor((*instance)[index]));
	}

	void cv_VectorOfGPCPatchDescriptor_set(std::vector<cv::optflow::GPCPatchDescriptor>* instance, size_t index, cv::optflow::GPCPatchDescriptor* val) {
		(*instance)[index] = *val;
	}

}


