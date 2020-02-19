template struct Result<bool>;
template struct Result<const char*>;
template struct Result<const int*>;
template struct Result<cv::Size_<int>>;
template struct Result<cv::Vec<int, 4>>;
template struct Result<double>;
template struct Result<float*>;
template struct Result<int>;
template struct Result<int*>;
template struct Result<long>;
template struct Result<unsigned char*>;
template struct Result<unsigned long>;
template struct Result<void*>;
extern "C" void cv_PtrOfAbsLayer_delete(cv::Ptr<cv::dnn::AbsLayer>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfAbsLayer_get_inner_ptr(cv::Ptr<cv::dnn::AbsLayer>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfBNLLLayer_delete(cv::Ptr<cv::dnn::BNLLLayer>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfBNLLLayer_get_inner_ptr(cv::Ptr<cv::dnn::BNLLLayer>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfBaseConvolutionLayer_delete(cv::Ptr<cv::dnn::BaseConvolutionLayer>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfBaseConvolutionLayer_get_inner_ptr(cv::Ptr<cv::dnn::BaseConvolutionLayer>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfConcatLayer_delete(cv::Ptr<cv::dnn::ConcatLayer>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfConcatLayer_get_inner_ptr(cv::Ptr<cv::dnn::ConcatLayer>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfCropLayer_delete(cv::Ptr<cv::dnn::CropLayer>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfCropLayer_get_inner_ptr(cv::Ptr<cv::dnn::CropLayer>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfEltwiseLayer_delete(cv::Ptr<cv::dnn::EltwiseLayer>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfEltwiseLayer_get_inner_ptr(cv::Ptr<cv::dnn::EltwiseLayer>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfImporter_delete(cv::Ptr<cv::dnn::Importer>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfImporter_get_inner_ptr(cv::Ptr<cv::dnn::Importer>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfInnerProductLayer_delete(cv::Ptr<cv::dnn::InnerProductLayer>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfInnerProductLayer_get_inner_ptr(cv::Ptr<cv::dnn::InnerProductLayer>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfLRNLayer_delete(cv::Ptr<cv::dnn::LRNLayer>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfLRNLayer_get_inner_ptr(cv::Ptr<cv::dnn::LRNLayer>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfLSTMLayer_delete(cv::Ptr<cv::dnn::LSTMLayer>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfLSTMLayer_get_inner_ptr(cv::Ptr<cv::dnn::LSTMLayer>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfLayer_delete(cv::Ptr<cv::dnn::Layer>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfLayer_get_inner_ptr(cv::Ptr<cv::dnn::Layer>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfMVNLayer_delete(cv::Ptr<cv::dnn::MVNLayer>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfMVNLayer_get_inner_ptr(cv::Ptr<cv::dnn::MVNLayer>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfPoolingLayer_delete(cv::Ptr<cv::dnn::PoolingLayer>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfPoolingLayer_get_inner_ptr(cv::Ptr<cv::dnn::PoolingLayer>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfPowerLayer_delete(cv::Ptr<cv::dnn::PowerLayer>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfPowerLayer_get_inner_ptr(cv::Ptr<cv::dnn::PowerLayer>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfRNNLayer_delete(cv::Ptr<cv::dnn::RNNLayer>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfRNNLayer_get_inner_ptr(cv::Ptr<cv::dnn::RNNLayer>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfReLULayer_delete(cv::Ptr<cv::dnn::ReLULayer>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfReLULayer_get_inner_ptr(cv::Ptr<cv::dnn::ReLULayer>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfReshapeLayer_delete(cv::Ptr<cv::dnn::ReshapeLayer>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfReshapeLayer_get_inner_ptr(cv::Ptr<cv::dnn::ReshapeLayer>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfSigmoidLayer_delete(cv::Ptr<cv::dnn::SigmoidLayer>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfSigmoidLayer_get_inner_ptr(cv::Ptr<cv::dnn::SigmoidLayer>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfSliceLayer_delete(cv::Ptr<cv::dnn::SliceLayer>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfSliceLayer_get_inner_ptr(cv::Ptr<cv::dnn::SliceLayer>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfSoftmaxLayer_delete(cv::Ptr<cv::dnn::SoftmaxLayer>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfSoftmaxLayer_get_inner_ptr(cv::Ptr<cv::dnn::SoftmaxLayer>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfSplitLayer_delete(cv::Ptr<cv::dnn::SplitLayer>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfSplitLayer_get_inner_ptr(cv::Ptr<cv::dnn::SplitLayer>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfTanHLayer_delete(cv::Ptr<cv::dnn::TanHLayer>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfTanHLayer_get_inner_ptr(cv::Ptr<cv::dnn::TanHLayer>* instance) {
	return instance->get();
}

extern "C" {
	void cv_VectorOfBlob_delete(std::vector<cv::dnn::Blob>* instance) {
		delete instance;
	}

	void* cv_VectorOfBlob_new() {
		return new std::vector<cv::dnn::Blob>();
	}

	size_t cv_VectorOfBlob_len(const std::vector<cv::dnn::Blob>* instance) {
		return instance->size();
	}

	bool cv_VectorOfBlob_is_empty(const std::vector<cv::dnn::Blob>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfBlob_capacity(const std::vector<cv::dnn::Blob>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfBlob_shrink_to_fit(std::vector<cv::dnn::Blob>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfBlob_reserve(std::vector<cv::dnn::Blob>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfBlob_remove(std::vector<cv::dnn::Blob>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfBlob_swap(std::vector<cv::dnn::Blob>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfBlob_clear(std::vector<cv::dnn::Blob>* instance) {
		instance->clear();
	}

	void cv_VectorOfBlob_push(std::vector<cv::dnn::Blob>* instance, cv::dnn::Blob* val) {
		instance->push_back(*val);
	}
	
	void cv_VectorOfBlob_insert(std::vector<cv::dnn::Blob>* instance, size_t index, cv::dnn::Blob* val) {
		instance->insert(instance->begin() + index, *val);
	}
	
	Result<void*> cv_VectorOfBlob_get(const std::vector<cv::dnn::Blob>* instance, size_t index) {
		try {
			return Ok<void*>(new cv::dnn::Blob(instance->at(index)));
		} VEC_CATCH(Result<void*>)
	}
	
	void* cv_VectorOfBlob_get_unchecked(const std::vector<cv::dnn::Blob>* instance, size_t index) {
		return new cv::dnn::Blob((*instance)[index]);
	}
	
	Result_void cv_VectorOfBlob_set(std::vector<cv::dnn::Blob>* instance, size_t index, cv::dnn::Blob* val) {
		try {
			instance->at(index) = *val;
			return Ok();
		} VEC_CATCH(Result_void)
	}
	
	void cv_VectorOfBlob_set_unchecked(std::vector<cv::dnn::Blob>* instance, size_t index, cv::dnn::Blob* val) {
		(*instance)[index] = *val;
	}
	
}


extern "C" {
	void cv_VectorOfNet_LayerId_delete(std::vector<cv::dnn::Net::LayerId>* instance) {
		delete instance;
	}

	void* cv_VectorOfNet_LayerId_new() {
		return new std::vector<cv::dnn::Net::LayerId>();
	}

	size_t cv_VectorOfNet_LayerId_len(const std::vector<cv::dnn::Net::LayerId>* instance) {
		return instance->size();
	}

	bool cv_VectorOfNet_LayerId_is_empty(const std::vector<cv::dnn::Net::LayerId>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfNet_LayerId_capacity(const std::vector<cv::dnn::Net::LayerId>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfNet_LayerId_shrink_to_fit(std::vector<cv::dnn::Net::LayerId>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfNet_LayerId_reserve(std::vector<cv::dnn::Net::LayerId>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfNet_LayerId_remove(std::vector<cv::dnn::Net::LayerId>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfNet_LayerId_swap(std::vector<cv::dnn::Net::LayerId>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfNet_LayerId_clear(std::vector<cv::dnn::Net::LayerId>* instance) {
		instance->clear();
	}

	void cv_VectorOfNet_LayerId_push(std::vector<cv::dnn::Net::LayerId>* instance, cv::dnn::Net::LayerId* val) {
		instance->push_back(*val);
	}
	
	void cv_VectorOfNet_LayerId_insert(std::vector<cv::dnn::Net::LayerId>* instance, size_t index, cv::dnn::Net::LayerId* val) {
		instance->insert(instance->begin() + index, *val);
	}
	
	Result<void*> cv_VectorOfNet_LayerId_get(const std::vector<cv::dnn::Net::LayerId>* instance, size_t index) {
		try {
			return Ok<void*>(new cv::dnn::Net::LayerId(instance->at(index)));
		} VEC_CATCH(Result<void*>)
	}
	
	void* cv_VectorOfNet_LayerId_get_unchecked(const std::vector<cv::dnn::Net::LayerId>* instance, size_t index) {
		return new cv::dnn::Net::LayerId((*instance)[index]);
	}
	
	Result_void cv_VectorOfNet_LayerId_set(std::vector<cv::dnn::Net::LayerId>* instance, size_t index, cv::dnn::Net::LayerId* val) {
		try {
			instance->at(index) = *val;
			return Ok();
		} VEC_CATCH(Result_void)
	}
	
	void cv_VectorOfNet_LayerId_set_unchecked(std::vector<cv::dnn::Net::LayerId>* instance, size_t index, cv::dnn::Net::LayerId* val) {
		(*instance)[index] = *val;
	}
	
}


