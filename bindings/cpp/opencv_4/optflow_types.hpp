template struct Result<bool>;
template struct Result<cv::Ptr<cv::DenseOpticalFlow>*>;
template struct Result<cv::Ptr<cv::SparseOpticalFlow>*>;
template struct Result<cv::Ptr<cv::optflow::DenseRLOFOpticalFlow>*>;
template struct Result<cv::Ptr<cv::optflow::DualTVL1OpticalFlow>*>;
template struct Result<cv::Ptr<cv::optflow::GPCTrainingSamples>*>;
template struct Result<cv::Ptr<cv::optflow::GPCTree>*>;
template struct Result<cv::Ptr<cv::optflow::PCAPrior>*>;
template struct Result<cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>*>;
template struct Result<cv::Ptr<cv::optflow::SparseRLOFOpticalFlow>*>;
template struct Result<cv::Size_<int>>;
template struct Result<cv::Vec<double, 18>>;
template struct Result<cv::optflow::GPCMatchingParams>;
template struct Result<cv::optflow::GPCPatchDescriptor*>;
template struct Result<cv::optflow::GPCTrainingParams>;
template struct Result<cv::optflow::InterpolationType>;
template struct Result<cv::optflow::OpticalFlowPCAFlow*>;
template struct Result<cv::optflow::PCAPrior*>;
template struct Result<cv::optflow::RLOFOpticalFlowParameter*>;
template struct Result<cv::optflow::SolverType>;
template struct Result<cv::optflow::SupportRegionType>;
template struct Result<double>;
template struct Result<float>;
template struct Result<int>;
template struct Result<std::vector<cv::Rect_<int>>*>;
template struct Result<std::vector<cv::optflow::GPCPatchDescriptor>*>;
template struct Result<std::vector<std::string>*>;
template struct Result<unsigned int>;
template struct Result<unsigned long>;
extern "C" void cv_PtrOfDenseRLOFOpticalFlow_delete(cv::Ptr<cv::optflow::DenseRLOFOpticalFlow>* instance) {
	delete instance;
}

extern "C" cv::optflow::DenseRLOFOpticalFlow* cv_PtrOfDenseRLOFOpticalFlow_get_inner_ptr(cv::Ptr<cv::optflow::DenseRLOFOpticalFlow>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfDualTVL1OpticalFlow_delete(cv::Ptr<cv::optflow::DualTVL1OpticalFlow>* instance) {
	delete instance;
}

extern "C" cv::optflow::DualTVL1OpticalFlow* cv_PtrOfDualTVL1OpticalFlow_get_inner_ptr(cv::Ptr<cv::optflow::DualTVL1OpticalFlow>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfGPCTrainingSamples_delete(cv::Ptr<cv::optflow::GPCTrainingSamples>* instance) {
	delete instance;
}

extern "C" cv::optflow::GPCTrainingSamples* cv_PtrOfGPCTrainingSamples_get_inner_ptr(cv::Ptr<cv::optflow::GPCTrainingSamples>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfGPCTree_delete(cv::Ptr<cv::optflow::GPCTree>* instance) {
	delete instance;
}

extern "C" cv::optflow::GPCTree* cv_PtrOfGPCTree_get_inner_ptr(cv::Ptr<cv::optflow::GPCTree>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfPCAPrior_delete(cv::Ptr<cv::optflow::PCAPrior>* instance) {
	delete instance;
}

extern "C" cv::optflow::PCAPrior* cv_PtrOfPCAPrior_get_inner_ptr(cv::Ptr<cv::optflow::PCAPrior>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfRLOFOpticalFlowParameter_delete(cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>* instance) {
	delete instance;
}

extern "C" cv::optflow::RLOFOpticalFlowParameter* cv_PtrOfRLOFOpticalFlowParameter_get_inner_ptr(cv::Ptr<cv::optflow::RLOFOpticalFlowParameter>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfSparseRLOFOpticalFlow_delete(cv::Ptr<cv::optflow::SparseRLOFOpticalFlow>* instance) {
	delete instance;
}

extern "C" cv::optflow::SparseRLOFOpticalFlow* cv_PtrOfSparseRLOFOpticalFlow_get_inner_ptr(cv::Ptr<cv::optflow::SparseRLOFOpticalFlow>* instance) {
	return instance->get();
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


