template struct Result<bool>;
template struct Result<cv::Mat*>;
template struct Result<cv::Ptr<cv::ml::ANN_MLP>*>;
template struct Result<cv::Ptr<cv::ml::Boost>*>;
template struct Result<cv::Ptr<cv::ml::DTrees>*>;
template struct Result<cv::Ptr<cv::ml::EM>*>;
template struct Result<cv::Ptr<cv::ml::KNearest>*>;
template struct Result<cv::Ptr<cv::ml::LogisticRegression>*>;
template struct Result<cv::Ptr<cv::ml::NormalBayesClassifier>*>;
template struct Result<cv::Ptr<cv::ml::ParamGrid>*>;
template struct Result<cv::Ptr<cv::ml::RTrees>*>;
template struct Result<cv::Ptr<cv::ml::SVMSGD>*>;
template struct Result<cv::Ptr<cv::ml::SVM::Kernel>*>;
template struct Result<cv::Ptr<cv::ml::SVM>*>;
template struct Result<cv::Ptr<cv::ml::TrainData>*>;
template struct Result<cv::TermCriteria>;
template struct Result<cv::Vec<double, 2>>;
template struct Result<cv::ml::DTrees::Node*>;
template struct Result<cv::ml::DTrees::Split*>;
template struct Result<cv::ml::ParamGrid*>;
template struct Result<double>;
template struct Result<float>;
template struct Result<int>;
template struct Result<std::vector<cv::Mat>*>;
template struct Result<std::vector<cv::ml::DTrees::Node>*>;
template struct Result<std::vector<cv::ml::DTrees::Split>*>;
template struct Result<std::vector<int>*>;
template struct Result<std::vector<std::string>*>;
extern "C" void cv_PtrOfANN_MLP_delete(cv::Ptr<cv::ml::ANN_MLP>* instance) {
	delete instance;
}

extern "C" cv::ml::ANN_MLP* cv_PtrOfANN_MLP_get_inner_ptr(cv::Ptr<cv::ml::ANN_MLP>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfBoost_delete(cv::Ptr<cv::ml::Boost>* instance) {
	delete instance;
}

extern "C" cv::ml::Boost* cv_PtrOfBoost_get_inner_ptr(cv::Ptr<cv::ml::Boost>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfDTrees_delete(cv::Ptr<cv::ml::DTrees>* instance) {
	delete instance;
}

extern "C" cv::ml::DTrees* cv_PtrOfDTrees_get_inner_ptr(cv::Ptr<cv::ml::DTrees>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfEM_delete(cv::Ptr<cv::ml::EM>* instance) {
	delete instance;
}

extern "C" cv::ml::EM* cv_PtrOfEM_get_inner_ptr(cv::Ptr<cv::ml::EM>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfKNearest_delete(cv::Ptr<cv::ml::KNearest>* instance) {
	delete instance;
}

extern "C" cv::ml::KNearest* cv_PtrOfKNearest_get_inner_ptr(cv::Ptr<cv::ml::KNearest>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfLogisticRegression_delete(cv::Ptr<cv::ml::LogisticRegression>* instance) {
	delete instance;
}

extern "C" cv::ml::LogisticRegression* cv_PtrOfLogisticRegression_get_inner_ptr(cv::Ptr<cv::ml::LogisticRegression>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfNormalBayesClassifier_delete(cv::Ptr<cv::ml::NormalBayesClassifier>* instance) {
	delete instance;
}

extern "C" cv::ml::NormalBayesClassifier* cv_PtrOfNormalBayesClassifier_get_inner_ptr(cv::Ptr<cv::ml::NormalBayesClassifier>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfParamGrid_delete(cv::Ptr<cv::ml::ParamGrid>* instance) {
	delete instance;
}

extern "C" cv::ml::ParamGrid* cv_PtrOfParamGrid_get_inner_ptr(cv::Ptr<cv::ml::ParamGrid>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfRTrees_delete(cv::Ptr<cv::ml::RTrees>* instance) {
	delete instance;
}

extern "C" cv::ml::RTrees* cv_PtrOfRTrees_get_inner_ptr(cv::Ptr<cv::ml::RTrees>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfSVM_delete(cv::Ptr<cv::ml::SVM>* instance) {
	delete instance;
}

extern "C" cv::ml::SVM* cv_PtrOfSVM_get_inner_ptr(cv::Ptr<cv::ml::SVM>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfSVMSGD_delete(cv::Ptr<cv::ml::SVMSGD>* instance) {
	delete instance;
}

extern "C" cv::ml::SVMSGD* cv_PtrOfSVMSGD_get_inner_ptr(cv::Ptr<cv::ml::SVMSGD>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfSVM_Kernel_delete(cv::Ptr<cv::ml::SVM::Kernel>* instance) {
	delete instance;
}

extern "C" cv::ml::SVM::Kernel* cv_PtrOfSVM_Kernel_get_inner_ptr(cv::Ptr<cv::ml::SVM::Kernel>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfTrainData_delete(cv::Ptr<cv::ml::TrainData>* instance) {
	delete instance;
}

extern "C" cv::ml::TrainData* cv_PtrOfTrainData_get_inner_ptr(cv::Ptr<cv::ml::TrainData>* instance) {
	return instance->get();
}

extern "C" {
	void cv_VectorOfDTrees_Node_delete(std::vector<cv::ml::DTrees::Node>* instance) {
		delete instance;
	}

	std::vector<cv::ml::DTrees::Node>* cv_VectorOfDTrees_Node_new() {
		return new std::vector<cv::ml::DTrees::Node>();
	}

	size_t cv_VectorOfDTrees_Node_len(const std::vector<cv::ml::DTrees::Node>* instance) {
		return instance->size();
	}

	bool cv_VectorOfDTrees_Node_is_empty(const std::vector<cv::ml::DTrees::Node>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfDTrees_Node_capacity(const std::vector<cv::ml::DTrees::Node>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfDTrees_Node_shrink_to_fit(std::vector<cv::ml::DTrees::Node>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfDTrees_Node_reserve(std::vector<cv::ml::DTrees::Node>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfDTrees_Node_remove(std::vector<cv::ml::DTrees::Node>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfDTrees_Node_swap(std::vector<cv::ml::DTrees::Node>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfDTrees_Node_clear(std::vector<cv::ml::DTrees::Node>* instance) {
		instance->clear();
	}

	void cv_VectorOfDTrees_Node_push(std::vector<cv::ml::DTrees::Node>* instance, cv::ml::DTrees::Node* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfDTrees_Node_insert(std::vector<cv::ml::DTrees::Node>* instance, size_t index, cv::ml::DTrees::Node* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<cv::ml::DTrees::Node*> cv_VectorOfDTrees_Node_get(const std::vector<cv::ml::DTrees::Node>* instance, size_t index) {
		return Ok<cv::ml::DTrees::Node*>(new cv::ml::DTrees::Node((*instance)[index]));
	}

	void cv_VectorOfDTrees_Node_set(std::vector<cv::ml::DTrees::Node>* instance, size_t index, cv::ml::DTrees::Node* val) {
		(*instance)[index] = *val;
	}

}


extern "C" {
	void cv_VectorOfDTrees_Split_delete(std::vector<cv::ml::DTrees::Split>* instance) {
		delete instance;
	}

	std::vector<cv::ml::DTrees::Split>* cv_VectorOfDTrees_Split_new() {
		return new std::vector<cv::ml::DTrees::Split>();
	}

	size_t cv_VectorOfDTrees_Split_len(const std::vector<cv::ml::DTrees::Split>* instance) {
		return instance->size();
	}

	bool cv_VectorOfDTrees_Split_is_empty(const std::vector<cv::ml::DTrees::Split>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfDTrees_Split_capacity(const std::vector<cv::ml::DTrees::Split>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfDTrees_Split_shrink_to_fit(std::vector<cv::ml::DTrees::Split>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfDTrees_Split_reserve(std::vector<cv::ml::DTrees::Split>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfDTrees_Split_remove(std::vector<cv::ml::DTrees::Split>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfDTrees_Split_swap(std::vector<cv::ml::DTrees::Split>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfDTrees_Split_clear(std::vector<cv::ml::DTrees::Split>* instance) {
		instance->clear();
	}

	void cv_VectorOfDTrees_Split_push(std::vector<cv::ml::DTrees::Split>* instance, cv::ml::DTrees::Split* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfDTrees_Split_insert(std::vector<cv::ml::DTrees::Split>* instance, size_t index, cv::ml::DTrees::Split* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<cv::ml::DTrees::Split*> cv_VectorOfDTrees_Split_get(const std::vector<cv::ml::DTrees::Split>* instance, size_t index) {
		return Ok<cv::ml::DTrees::Split*>(new cv::ml::DTrees::Split((*instance)[index]));
	}

	void cv_VectorOfDTrees_Split_set(std::vector<cv::ml::DTrees::Split>* instance, size_t index, cv::ml::DTrees::Split* val) {
		(*instance)[index] = *val;
	}

}


