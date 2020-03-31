template struct Result<bool>;
template struct Result<char>;
template struct Result<const cv::FileStorage**>;
template struct Result<const cv::Mat**>;
template struct Result<const cv::SparseMat**>;
template struct Result<const cv::SparseMat::Node**>;
template struct Result<const int*>;
template struct Result<const unsigned char*>;
template struct Result<const void*>;
template struct Result<cv::Algorithm*>;
template struct Result<cv::AsyncArray*>;
template struct Result<cv::AsyncPromise*>;
template struct Result<cv::CommandLineParser*>;
template struct Result<cv::DMatch>;
template struct Result<cv::Exception*>;
template struct Result<cv::FileNodeIterator*>;
template struct Result<cv::FileNode*>;
template struct Result<cv::FileStorage*>;
template struct Result<cv::KeyPoint>;
template struct Result<cv::LDA*>;
template struct Result<cv::MatConstIterator*>;
template struct Result<cv::MatExpr*>;
template struct Result<cv::MatSize*>;
template struct Result<cv::MatStep*>;
template struct Result<cv::Mat*>;
template struct Result<cv::Mat**>;
template struct Result<cv::Matx_AddOp*>;
template struct Result<cv::Matx_DivOp*>;
template struct Result<cv::Matx_MatMulOp*>;
template struct Result<cv::Matx_MulOp*>;
template struct Result<cv::Matx_ScaleOp*>;
template struct Result<cv::Matx_SubOp*>;
template struct Result<cv::Matx_TOp*>;
template struct Result<cv::Moments>;
template struct Result<cv::NormTypes>;
template struct Result<cv::PCA*>;
template struct Result<cv::Point3_<double>>;
template struct Result<cv::Point3_<float>>;
template struct Result<cv::Point3_<int>>;
template struct Result<cv::Point_<double>>;
template struct Result<cv::Point_<float>>;
template struct Result<cv::Point_<int>>;
template struct Result<cv::Ptr<cv::ConjGradSolver>*>;
template struct Result<cv::Ptr<cv::DownhillSolver>*>;
template struct Result<cv::Ptr<cv::FileStorage>*>;
template struct Result<cv::Ptr<cv::Formatted>*>;
template struct Result<cv::Ptr<cv::Formatter>*>;
template struct Result<cv::Ptr<cv::MinProblemSolver::Function>*>;
template struct Result<cv::RNG*>;
template struct Result<cv::RNG_MT19937*>;
template struct Result<cv::Range*>;
template struct Result<cv::Rect_<double>>;
template struct Result<cv::Rect_<float>>;
template struct Result<cv::Rect_<int>>;
template struct Result<cv::RotatedRect*>;
template struct Result<cv::SVD*>;
template struct Result<cv::Scalar_<double>>;
template struct Result<cv::Size_<float>>;
template struct Result<cv::Size_<int>>;
template struct Result<cv::SparseMatConstIterator*>;
template struct Result<cv::SparseMatIterator*>;
template struct Result<cv::SparseMat*>;
template struct Result<cv::SparseMat::Hdr*>;
template struct Result<cv::SparseMat::Hdr**>;
template struct Result<cv::SparseMat::Node**>;
template struct Result<cv::TermCriteria>;
template struct Result<cv::TickMeter*>;
template struct Result<cv::UMatData**>;
template struct Result<cv::UMatData::MemoryFlag>;
template struct Result<cv::UMatUsageFlags>;
template struct Result<cv::UMat*>;
template struct Result<cv::UMat**>;
template struct Result<cv::Vec<double, 3>>;
template struct Result<cv::Vec<float, 3>>;
template struct Result<cv::Vec<float, 4>>;
template struct Result<cv::Vec<float, 6>>;
template struct Result<cv::Vec<int, 2>>;
template struct Result<cv::Vec<int, 3>>;
template struct Result<cv::Vec<int, 4>>;
template struct Result<cv::_InputArray*>;
template struct Result<cv::_InputArray::KindFlag>;
template struct Result<cv::_InputOutputArray*>;
template struct Result<cv::_OutputArray*>;
template struct Result<cv::detail::TestOp>;
template struct Result<cv::instr::FLAGS>;
template struct Result<cv::instr::IMPL>;
template struct Result<cv::instr::NodeData*>;
template struct Result<cv::instr::TYPE>;
template struct Result<cv::internal::WriteStructContext*>;
template struct Result<cv::ocl::Context*>;
template struct Result<cv::ocl::Device*>;
template struct Result<cv::ocl::Image2D*>;
template struct Result<cv::ocl::KernelArg*>;
template struct Result<cv::ocl::Kernel*>;
template struct Result<cv::ocl::PlatformInfo*>;
template struct Result<cv::ocl::Platform*>;
template struct Result<cv::ocl::ProgramSource*>;
template struct Result<cv::ocl::Program*>;
template struct Result<cv::ocl::Queue*>;
template struct Result<cv::ocl::Timer*>;
template struct Result<cv::utils::logging::LogLevel>;
template struct Result<cv::utils::logging::LogTag*>;
template struct Result<cv::utils::logging::LogTag**>;
template struct Result<double>;
template struct Result<float>;
template struct Result<int>;
template struct Result<int*>;
template struct Result<int(*)[32]>;
template struct Result<long>;
template struct Result<short>;
template struct Result<signed char>;
template struct Result<std::vector<bool>*>;
template struct Result<std::vector<char>*>;
template struct Result<std::vector<cv::DMatch>*>;
template struct Result<std::vector<cv::KeyPoint>*>;
template struct Result<std::vector<cv::Mat>*>;
template struct Result<std::vector<cv::Point3_<double>>*>;
template struct Result<std::vector<cv::Point3_<float>>*>;
template struct Result<std::vector<cv::Point3_<int>>*>;
template struct Result<std::vector<cv::Point_<float>>*>;
template struct Result<std::vector<cv::Point_<int>>*>;
template struct Result<std::vector<cv::Range>*>;
template struct Result<std::vector<cv::UMat>*>;
template struct Result<std::vector<cv::Vec<int, 2>>*>;
template struct Result<std::vector<cv::ocl::PlatformInfo>*>;
template struct Result<std::vector<double>*>;
template struct Result<std::vector<int>*>;
template struct Result<std::vector<std::string>*>;
template struct Result<std::vector<std::vector<int>>*>;
template struct Result<std::vector<unsigned char>*>;
template struct Result<std::vector<unsigned long>*>;
template struct Result<unsigned char>;
template struct Result<unsigned char*>;
template struct Result<unsigned int>;
template struct Result<unsigned long>;
template struct Result<unsigned long*>;
template struct Result<unsigned long(*)[2]>;
template struct Result<unsigned long long>;
template struct Result<unsigned short>;
template struct Result<void*>;
extern "C" void cv_PtrOfConjGradSolver_delete(cv::Ptr<cv::ConjGradSolver>* instance) {
	delete instance;
}

extern "C" cv::ConjGradSolver* cv_PtrOfConjGradSolver_get_inner_ptr(cv::Ptr<cv::ConjGradSolver>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfDownhillSolver_delete(cv::Ptr<cv::DownhillSolver>* instance) {
	delete instance;
}

extern "C" cv::DownhillSolver* cv_PtrOfDownhillSolver_get_inner_ptr(cv::Ptr<cv::DownhillSolver>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfFileStorage_delete(cv::Ptr<cv::FileStorage>* instance) {
	delete instance;
}

extern "C" cv::FileStorage* cv_PtrOfFileStorage_get_inner_ptr(cv::Ptr<cv::FileStorage>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfFormatted_delete(cv::Ptr<cv::Formatted>* instance) {
	delete instance;
}

extern "C" cv::Formatted* cv_PtrOfFormatted_get_inner_ptr(cv::Ptr<cv::Formatted>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfFormatter_delete(cv::Ptr<cv::Formatter>* instance) {
	delete instance;
}

extern "C" cv::Formatter* cv_PtrOfFormatter_get_inner_ptr(cv::Ptr<cv::Formatter>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfMinProblemSolver_Function_delete(cv::Ptr<cv::MinProblemSolver::Function>* instance) {
	delete instance;
}

extern "C" cv::MinProblemSolver::Function* cv_PtrOfMinProblemSolver_Function_get_inner_ptr(cv::Ptr<cv::MinProblemSolver::Function>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOff32_delete(cv::Ptr<float>* instance) {
	delete instance;
}

extern "C" float* cv_PtrOff32_get_inner_ptr(cv::Ptr<float>* instance) {
	return instance->get();
}

extern "C" {
	void cv_VectorOfDMatch_delete(std::vector<cv::DMatch>* instance) {
		delete instance;
	}

	std::vector<cv::DMatch>* cv_VectorOfDMatch_new() {
		return new std::vector<cv::DMatch>();
	}

	size_t cv_VectorOfDMatch_len(const std::vector<cv::DMatch>* instance) {
		return instance->size();
	}

	bool cv_VectorOfDMatch_is_empty(const std::vector<cv::DMatch>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfDMatch_capacity(const std::vector<cv::DMatch>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfDMatch_shrink_to_fit(std::vector<cv::DMatch>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfDMatch_reserve(std::vector<cv::DMatch>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfDMatch_remove(std::vector<cv::DMatch>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfDMatch_swap(std::vector<cv::DMatch>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfDMatch_clear(std::vector<cv::DMatch>* instance) {
		instance->clear();
	}

	void cv_VectorOfDMatch_push(std::vector<cv::DMatch>* instance, const cv::DMatch* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfDMatch_insert(std::vector<cv::DMatch>* instance, size_t index, const cv::DMatch* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<cv::DMatch> cv_VectorOfDMatch_get(const std::vector<cv::DMatch>* instance, size_t index) {
		return Ok<cv::DMatch>((*instance)[index]);
	}

	void cv_VectorOfDMatch_set(std::vector<cv::DMatch>* instance, size_t index, const cv::DMatch* val) {
		(*instance)[index] = *val;
	}

	const cv::DMatch* cv_VectorOfDMatch_data(std::vector<cv::DMatch>* instance) {
		return instance->data();
	}
	
}


extern "C" {
	void cv_VectorOfKeyPoint_delete(std::vector<cv::KeyPoint>* instance) {
		delete instance;
	}

	std::vector<cv::KeyPoint>* cv_VectorOfKeyPoint_new() {
		return new std::vector<cv::KeyPoint>();
	}

	size_t cv_VectorOfKeyPoint_len(const std::vector<cv::KeyPoint>* instance) {
		return instance->size();
	}

	bool cv_VectorOfKeyPoint_is_empty(const std::vector<cv::KeyPoint>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfKeyPoint_capacity(const std::vector<cv::KeyPoint>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfKeyPoint_shrink_to_fit(std::vector<cv::KeyPoint>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfKeyPoint_reserve(std::vector<cv::KeyPoint>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfKeyPoint_remove(std::vector<cv::KeyPoint>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfKeyPoint_swap(std::vector<cv::KeyPoint>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfKeyPoint_clear(std::vector<cv::KeyPoint>* instance) {
		instance->clear();
	}

	void cv_VectorOfKeyPoint_push(std::vector<cv::KeyPoint>* instance, const cv::KeyPoint* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfKeyPoint_insert(std::vector<cv::KeyPoint>* instance, size_t index, const cv::KeyPoint* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<cv::KeyPoint> cv_VectorOfKeyPoint_get(const std::vector<cv::KeyPoint>* instance, size_t index) {
		return Ok<cv::KeyPoint>((*instance)[index]);
	}

	void cv_VectorOfKeyPoint_set(std::vector<cv::KeyPoint>* instance, size_t index, const cv::KeyPoint* val) {
		(*instance)[index] = *val;
	}

	const cv::KeyPoint* cv_VectorOfKeyPoint_data(std::vector<cv::KeyPoint>* instance) {
		return instance->data();
	}
	
}


extern "C" {
	void cv_VectorOfMat_delete(std::vector<cv::Mat>* instance) {
		delete instance;
	}

	std::vector<cv::Mat>* cv_VectorOfMat_new() {
		return new std::vector<cv::Mat>();
	}

	size_t cv_VectorOfMat_len(const std::vector<cv::Mat>* instance) {
		return instance->size();
	}

	bool cv_VectorOfMat_is_empty(const std::vector<cv::Mat>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfMat_capacity(const std::vector<cv::Mat>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfMat_shrink_to_fit(std::vector<cv::Mat>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfMat_reserve(std::vector<cv::Mat>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfMat_remove(std::vector<cv::Mat>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfMat_swap(std::vector<cv::Mat>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfMat_clear(std::vector<cv::Mat>* instance) {
		instance->clear();
	}

	void cv_VectorOfMat_push(std::vector<cv::Mat>* instance, cv::Mat* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfMat_insert(std::vector<cv::Mat>* instance, size_t index, cv::Mat* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<cv::Mat*> cv_VectorOfMat_get(const std::vector<cv::Mat>* instance, size_t index) {
		return Ok<cv::Mat*>(new cv::Mat((*instance)[index]));
	}

	void cv_VectorOfMat_set(std::vector<cv::Mat>* instance, size_t index, cv::Mat* val) {
		(*instance)[index] = *val;
	}

}


extern "C" {
	void cv_VectorOfPlatformInfo_delete(std::vector<cv::ocl::PlatformInfo>* instance) {
		delete instance;
	}

	std::vector<cv::ocl::PlatformInfo>* cv_VectorOfPlatformInfo_new() {
		return new std::vector<cv::ocl::PlatformInfo>();
	}

	size_t cv_VectorOfPlatformInfo_len(const std::vector<cv::ocl::PlatformInfo>* instance) {
		return instance->size();
	}

	bool cv_VectorOfPlatformInfo_is_empty(const std::vector<cv::ocl::PlatformInfo>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfPlatformInfo_capacity(const std::vector<cv::ocl::PlatformInfo>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfPlatformInfo_shrink_to_fit(std::vector<cv::ocl::PlatformInfo>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfPlatformInfo_reserve(std::vector<cv::ocl::PlatformInfo>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfPlatformInfo_remove(std::vector<cv::ocl::PlatformInfo>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfPlatformInfo_swap(std::vector<cv::ocl::PlatformInfo>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfPlatformInfo_clear(std::vector<cv::ocl::PlatformInfo>* instance) {
		instance->clear();
	}

	void cv_VectorOfPlatformInfo_push(std::vector<cv::ocl::PlatformInfo>* instance, cv::ocl::PlatformInfo* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfPlatformInfo_insert(std::vector<cv::ocl::PlatformInfo>* instance, size_t index, cv::ocl::PlatformInfo* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<cv::ocl::PlatformInfo*> cv_VectorOfPlatformInfo_get(const std::vector<cv::ocl::PlatformInfo>* instance, size_t index) {
		return Ok<cv::ocl::PlatformInfo*>(new cv::ocl::PlatformInfo((*instance)[index]));
	}

	void cv_VectorOfPlatformInfo_set(std::vector<cv::ocl::PlatformInfo>* instance, size_t index, cv::ocl::PlatformInfo* val) {
		(*instance)[index] = *val;
	}

}


extern "C" {
	void cv_VectorOfPoint_delete(std::vector<cv::Point>* instance) {
		delete instance;
	}

	std::vector<cv::Point>* cv_VectorOfPoint_new() {
		return new std::vector<cv::Point>();
	}

	size_t cv_VectorOfPoint_len(const std::vector<cv::Point>* instance) {
		return instance->size();
	}

	bool cv_VectorOfPoint_is_empty(const std::vector<cv::Point>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfPoint_capacity(const std::vector<cv::Point>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfPoint_shrink_to_fit(std::vector<cv::Point>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfPoint_reserve(std::vector<cv::Point>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfPoint_remove(std::vector<cv::Point>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfPoint_swap(std::vector<cv::Point>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfPoint_clear(std::vector<cv::Point>* instance) {
		instance->clear();
	}

	void cv_VectorOfPoint_push(std::vector<cv::Point>* instance, const cv::Point* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfPoint_insert(std::vector<cv::Point>* instance, size_t index, const cv::Point* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<cv::Point> cv_VectorOfPoint_get(const std::vector<cv::Point>* instance, size_t index) {
		return Ok<cv::Point>((*instance)[index]);
	}

	void cv_VectorOfPoint_set(std::vector<cv::Point>* instance, size_t index, const cv::Point* val) {
		(*instance)[index] = *val;
	}

	const cv::Point* cv_VectorOfPoint_data(std::vector<cv::Point>* instance) {
		return instance->data();
	}
	
	Result<cv::_InputArray*> cv_VectorOfPoint_input_array(std::vector<cv::Point>* instance) {
		try {
			return Ok(new cv::_InputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputArray*>)
	}
	
	Result<cv::_OutputArray*> cv_VectorOfPoint_output_array(std::vector<cv::Point>* instance) {
		try {
			return Ok(new cv::_OutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_OutputArray*>)
	}
	
	Result<cv::_InputOutputArray*> cv_VectorOfPoint_input_output_array(std::vector<cv::Point>* instance) {
		try {
			return Ok(new cv::_InputOutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputOutputArray*>)
	}
	
}


extern "C" {
	void cv_VectorOfPoint2d_delete(std::vector<cv::Point2d>* instance) {
		delete instance;
	}

	std::vector<cv::Point2d>* cv_VectorOfPoint2d_new() {
		return new std::vector<cv::Point2d>();
	}

	size_t cv_VectorOfPoint2d_len(const std::vector<cv::Point2d>* instance) {
		return instance->size();
	}

	bool cv_VectorOfPoint2d_is_empty(const std::vector<cv::Point2d>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfPoint2d_capacity(const std::vector<cv::Point2d>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfPoint2d_shrink_to_fit(std::vector<cv::Point2d>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfPoint2d_reserve(std::vector<cv::Point2d>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfPoint2d_remove(std::vector<cv::Point2d>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfPoint2d_swap(std::vector<cv::Point2d>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfPoint2d_clear(std::vector<cv::Point2d>* instance) {
		instance->clear();
	}

	void cv_VectorOfPoint2d_push(std::vector<cv::Point2d>* instance, const cv::Point2d* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfPoint2d_insert(std::vector<cv::Point2d>* instance, size_t index, const cv::Point2d* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<cv::Point2d> cv_VectorOfPoint2d_get(const std::vector<cv::Point2d>* instance, size_t index) {
		return Ok<cv::Point2d>((*instance)[index]);
	}

	void cv_VectorOfPoint2d_set(std::vector<cv::Point2d>* instance, size_t index, const cv::Point2d* val) {
		(*instance)[index] = *val;
	}

	const cv::Point2d* cv_VectorOfPoint2d_data(std::vector<cv::Point2d>* instance) {
		return instance->data();
	}
	
	Result<cv::_InputArray*> cv_VectorOfPoint2d_input_array(std::vector<cv::Point2d>* instance) {
		try {
			return Ok(new cv::_InputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputArray*>)
	}
	
	Result<cv::_OutputArray*> cv_VectorOfPoint2d_output_array(std::vector<cv::Point2d>* instance) {
		try {
			return Ok(new cv::_OutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_OutputArray*>)
	}
	
	Result<cv::_InputOutputArray*> cv_VectorOfPoint2d_input_output_array(std::vector<cv::Point2d>* instance) {
		try {
			return Ok(new cv::_InputOutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputOutputArray*>)
	}
	
}


extern "C" {
	void cv_VectorOfPoint2f_delete(std::vector<cv::Point2f>* instance) {
		delete instance;
	}

	std::vector<cv::Point2f>* cv_VectorOfPoint2f_new() {
		return new std::vector<cv::Point2f>();
	}

	size_t cv_VectorOfPoint2f_len(const std::vector<cv::Point2f>* instance) {
		return instance->size();
	}

	bool cv_VectorOfPoint2f_is_empty(const std::vector<cv::Point2f>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfPoint2f_capacity(const std::vector<cv::Point2f>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfPoint2f_shrink_to_fit(std::vector<cv::Point2f>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfPoint2f_reserve(std::vector<cv::Point2f>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfPoint2f_remove(std::vector<cv::Point2f>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfPoint2f_swap(std::vector<cv::Point2f>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfPoint2f_clear(std::vector<cv::Point2f>* instance) {
		instance->clear();
	}

	void cv_VectorOfPoint2f_push(std::vector<cv::Point2f>* instance, const cv::Point2f* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfPoint2f_insert(std::vector<cv::Point2f>* instance, size_t index, const cv::Point2f* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<cv::Point2f> cv_VectorOfPoint2f_get(const std::vector<cv::Point2f>* instance, size_t index) {
		return Ok<cv::Point2f>((*instance)[index]);
	}

	void cv_VectorOfPoint2f_set(std::vector<cv::Point2f>* instance, size_t index, const cv::Point2f* val) {
		(*instance)[index] = *val;
	}

	const cv::Point2f* cv_VectorOfPoint2f_data(std::vector<cv::Point2f>* instance) {
		return instance->data();
	}
	
	Result<cv::_InputArray*> cv_VectorOfPoint2f_input_array(std::vector<cv::Point2f>* instance) {
		try {
			return Ok(new cv::_InputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputArray*>)
	}
	
	Result<cv::_OutputArray*> cv_VectorOfPoint2f_output_array(std::vector<cv::Point2f>* instance) {
		try {
			return Ok(new cv::_OutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_OutputArray*>)
	}
	
	Result<cv::_InputOutputArray*> cv_VectorOfPoint2f_input_output_array(std::vector<cv::Point2f>* instance) {
		try {
			return Ok(new cv::_InputOutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputOutputArray*>)
	}
	
}


extern "C" {
	void cv_VectorOfPoint3d_delete(std::vector<cv::Point3d>* instance) {
		delete instance;
	}

	std::vector<cv::Point3d>* cv_VectorOfPoint3d_new() {
		return new std::vector<cv::Point3d>();
	}

	size_t cv_VectorOfPoint3d_len(const std::vector<cv::Point3d>* instance) {
		return instance->size();
	}

	bool cv_VectorOfPoint3d_is_empty(const std::vector<cv::Point3d>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfPoint3d_capacity(const std::vector<cv::Point3d>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfPoint3d_shrink_to_fit(std::vector<cv::Point3d>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfPoint3d_reserve(std::vector<cv::Point3d>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfPoint3d_remove(std::vector<cv::Point3d>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfPoint3d_swap(std::vector<cv::Point3d>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfPoint3d_clear(std::vector<cv::Point3d>* instance) {
		instance->clear();
	}

	void cv_VectorOfPoint3d_push(std::vector<cv::Point3d>* instance, const cv::Point3d* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfPoint3d_insert(std::vector<cv::Point3d>* instance, size_t index, const cv::Point3d* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<cv::Point3d> cv_VectorOfPoint3d_get(const std::vector<cv::Point3d>* instance, size_t index) {
		return Ok<cv::Point3d>((*instance)[index]);
	}

	void cv_VectorOfPoint3d_set(std::vector<cv::Point3d>* instance, size_t index, const cv::Point3d* val) {
		(*instance)[index] = *val;
	}

	const cv::Point3d* cv_VectorOfPoint3d_data(std::vector<cv::Point3d>* instance) {
		return instance->data();
	}
	
	Result<cv::_InputArray*> cv_VectorOfPoint3d_input_array(std::vector<cv::Point3d>* instance) {
		try {
			return Ok(new cv::_InputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputArray*>)
	}
	
	Result<cv::_OutputArray*> cv_VectorOfPoint3d_output_array(std::vector<cv::Point3d>* instance) {
		try {
			return Ok(new cv::_OutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_OutputArray*>)
	}
	
	Result<cv::_InputOutputArray*> cv_VectorOfPoint3d_input_output_array(std::vector<cv::Point3d>* instance) {
		try {
			return Ok(new cv::_InputOutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputOutputArray*>)
	}
	
}


extern "C" {
	void cv_VectorOfPoint3f_delete(std::vector<cv::Point3f>* instance) {
		delete instance;
	}

	std::vector<cv::Point3f>* cv_VectorOfPoint3f_new() {
		return new std::vector<cv::Point3f>();
	}

	size_t cv_VectorOfPoint3f_len(const std::vector<cv::Point3f>* instance) {
		return instance->size();
	}

	bool cv_VectorOfPoint3f_is_empty(const std::vector<cv::Point3f>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfPoint3f_capacity(const std::vector<cv::Point3f>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfPoint3f_shrink_to_fit(std::vector<cv::Point3f>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfPoint3f_reserve(std::vector<cv::Point3f>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfPoint3f_remove(std::vector<cv::Point3f>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfPoint3f_swap(std::vector<cv::Point3f>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfPoint3f_clear(std::vector<cv::Point3f>* instance) {
		instance->clear();
	}

	void cv_VectorOfPoint3f_push(std::vector<cv::Point3f>* instance, const cv::Point3f* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfPoint3f_insert(std::vector<cv::Point3f>* instance, size_t index, const cv::Point3f* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<cv::Point3f> cv_VectorOfPoint3f_get(const std::vector<cv::Point3f>* instance, size_t index) {
		return Ok<cv::Point3f>((*instance)[index]);
	}

	void cv_VectorOfPoint3f_set(std::vector<cv::Point3f>* instance, size_t index, const cv::Point3f* val) {
		(*instance)[index] = *val;
	}

	const cv::Point3f* cv_VectorOfPoint3f_data(std::vector<cv::Point3f>* instance) {
		return instance->data();
	}
	
	Result<cv::_InputArray*> cv_VectorOfPoint3f_input_array(std::vector<cv::Point3f>* instance) {
		try {
			return Ok(new cv::_InputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputArray*>)
	}
	
	Result<cv::_OutputArray*> cv_VectorOfPoint3f_output_array(std::vector<cv::Point3f>* instance) {
		try {
			return Ok(new cv::_OutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_OutputArray*>)
	}
	
	Result<cv::_InputOutputArray*> cv_VectorOfPoint3f_input_output_array(std::vector<cv::Point3f>* instance) {
		try {
			return Ok(new cv::_InputOutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputOutputArray*>)
	}
	
}


extern "C" {
	void cv_VectorOfPoint3i_delete(std::vector<cv::Point3i>* instance) {
		delete instance;
	}

	std::vector<cv::Point3i>* cv_VectorOfPoint3i_new() {
		return new std::vector<cv::Point3i>();
	}

	size_t cv_VectorOfPoint3i_len(const std::vector<cv::Point3i>* instance) {
		return instance->size();
	}

	bool cv_VectorOfPoint3i_is_empty(const std::vector<cv::Point3i>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfPoint3i_capacity(const std::vector<cv::Point3i>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfPoint3i_shrink_to_fit(std::vector<cv::Point3i>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfPoint3i_reserve(std::vector<cv::Point3i>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfPoint3i_remove(std::vector<cv::Point3i>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfPoint3i_swap(std::vector<cv::Point3i>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfPoint3i_clear(std::vector<cv::Point3i>* instance) {
		instance->clear();
	}

	void cv_VectorOfPoint3i_push(std::vector<cv::Point3i>* instance, const cv::Point3i* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfPoint3i_insert(std::vector<cv::Point3i>* instance, size_t index, const cv::Point3i* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<cv::Point3i> cv_VectorOfPoint3i_get(const std::vector<cv::Point3i>* instance, size_t index) {
		return Ok<cv::Point3i>((*instance)[index]);
	}

	void cv_VectorOfPoint3i_set(std::vector<cv::Point3i>* instance, size_t index, const cv::Point3i* val) {
		(*instance)[index] = *val;
	}

	const cv::Point3i* cv_VectorOfPoint3i_data(std::vector<cv::Point3i>* instance) {
		return instance->data();
	}
	
	Result<cv::_InputArray*> cv_VectorOfPoint3i_input_array(std::vector<cv::Point3i>* instance) {
		try {
			return Ok(new cv::_InputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputArray*>)
	}
	
	Result<cv::_OutputArray*> cv_VectorOfPoint3i_output_array(std::vector<cv::Point3i>* instance) {
		try {
			return Ok(new cv::_OutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_OutputArray*>)
	}
	
	Result<cv::_InputOutputArray*> cv_VectorOfPoint3i_input_output_array(std::vector<cv::Point3i>* instance) {
		try {
			return Ok(new cv::_InputOutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputOutputArray*>)
	}
	
}


extern "C" {
	void cv_VectorOfRange_delete(std::vector<cv::Range>* instance) {
		delete instance;
	}

	std::vector<cv::Range>* cv_VectorOfRange_new() {
		return new std::vector<cv::Range>();
	}

	size_t cv_VectorOfRange_len(const std::vector<cv::Range>* instance) {
		return instance->size();
	}

	bool cv_VectorOfRange_is_empty(const std::vector<cv::Range>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfRange_capacity(const std::vector<cv::Range>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfRange_shrink_to_fit(std::vector<cv::Range>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfRange_reserve(std::vector<cv::Range>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfRange_remove(std::vector<cv::Range>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfRange_swap(std::vector<cv::Range>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfRange_clear(std::vector<cv::Range>* instance) {
		instance->clear();
	}

	void cv_VectorOfRange_push(std::vector<cv::Range>* instance, cv::Range* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfRange_insert(std::vector<cv::Range>* instance, size_t index, cv::Range* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<cv::Range*> cv_VectorOfRange_get(const std::vector<cv::Range>* instance, size_t index) {
		return Ok<cv::Range*>(new cv::Range((*instance)[index]));
	}

	void cv_VectorOfRange_set(std::vector<cv::Range>* instance, size_t index, cv::Range* val) {
		(*instance)[index] = *val;
	}

}


extern "C" {
	void cv_VectorOfRect_delete(std::vector<cv::Rect>* instance) {
		delete instance;
	}

	std::vector<cv::Rect>* cv_VectorOfRect_new() {
		return new std::vector<cv::Rect>();
	}

	size_t cv_VectorOfRect_len(const std::vector<cv::Rect>* instance) {
		return instance->size();
	}

	bool cv_VectorOfRect_is_empty(const std::vector<cv::Rect>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfRect_capacity(const std::vector<cv::Rect>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfRect_shrink_to_fit(std::vector<cv::Rect>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfRect_reserve(std::vector<cv::Rect>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfRect_remove(std::vector<cv::Rect>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfRect_swap(std::vector<cv::Rect>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfRect_clear(std::vector<cv::Rect>* instance) {
		instance->clear();
	}

	void cv_VectorOfRect_push(std::vector<cv::Rect>* instance, const cv::Rect* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfRect_insert(std::vector<cv::Rect>* instance, size_t index, const cv::Rect* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<cv::Rect> cv_VectorOfRect_get(const std::vector<cv::Rect>* instance, size_t index) {
		return Ok<cv::Rect>((*instance)[index]);
	}

	void cv_VectorOfRect_set(std::vector<cv::Rect>* instance, size_t index, const cv::Rect* val) {
		(*instance)[index] = *val;
	}

	const cv::Rect* cv_VectorOfRect_data(std::vector<cv::Rect>* instance) {
		return instance->data();
	}
	
	Result<cv::_InputArray*> cv_VectorOfRect_input_array(std::vector<cv::Rect>* instance) {
		try {
			return Ok(new cv::_InputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputArray*>)
	}
	
	Result<cv::_OutputArray*> cv_VectorOfRect_output_array(std::vector<cv::Rect>* instance) {
		try {
			return Ok(new cv::_OutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_OutputArray*>)
	}
	
	Result<cv::_InputOutputArray*> cv_VectorOfRect_input_output_array(std::vector<cv::Rect>* instance) {
		try {
			return Ok(new cv::_InputOutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputOutputArray*>)
	}
	
}


extern "C" {
	void cv_VectorOfRect2d_delete(std::vector<cv::Rect2d>* instance) {
		delete instance;
	}

	std::vector<cv::Rect2d>* cv_VectorOfRect2d_new() {
		return new std::vector<cv::Rect2d>();
	}

	size_t cv_VectorOfRect2d_len(const std::vector<cv::Rect2d>* instance) {
		return instance->size();
	}

	bool cv_VectorOfRect2d_is_empty(const std::vector<cv::Rect2d>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfRect2d_capacity(const std::vector<cv::Rect2d>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfRect2d_shrink_to_fit(std::vector<cv::Rect2d>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfRect2d_reserve(std::vector<cv::Rect2d>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfRect2d_remove(std::vector<cv::Rect2d>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfRect2d_swap(std::vector<cv::Rect2d>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfRect2d_clear(std::vector<cv::Rect2d>* instance) {
		instance->clear();
	}

	void cv_VectorOfRect2d_push(std::vector<cv::Rect2d>* instance, const cv::Rect2d* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfRect2d_insert(std::vector<cv::Rect2d>* instance, size_t index, const cv::Rect2d* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<cv::Rect2d> cv_VectorOfRect2d_get(const std::vector<cv::Rect2d>* instance, size_t index) {
		return Ok<cv::Rect2d>((*instance)[index]);
	}

	void cv_VectorOfRect2d_set(std::vector<cv::Rect2d>* instance, size_t index, const cv::Rect2d* val) {
		(*instance)[index] = *val;
	}

	const cv::Rect2d* cv_VectorOfRect2d_data(std::vector<cv::Rect2d>* instance) {
		return instance->data();
	}
	
	Result<cv::_InputArray*> cv_VectorOfRect2d_input_array(std::vector<cv::Rect2d>* instance) {
		try {
			return Ok(new cv::_InputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputArray*>)
	}
	
	Result<cv::_OutputArray*> cv_VectorOfRect2d_output_array(std::vector<cv::Rect2d>* instance) {
		try {
			return Ok(new cv::_OutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_OutputArray*>)
	}
	
	Result<cv::_InputOutputArray*> cv_VectorOfRect2d_input_output_array(std::vector<cv::Rect2d>* instance) {
		try {
			return Ok(new cv::_InputOutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputOutputArray*>)
	}
	
}


extern "C" {
	void cv_VectorOfRotatedRect_delete(std::vector<cv::RotatedRect>* instance) {
		delete instance;
	}

	std::vector<cv::RotatedRect>* cv_VectorOfRotatedRect_new() {
		return new std::vector<cv::RotatedRect>();
	}

	size_t cv_VectorOfRotatedRect_len(const std::vector<cv::RotatedRect>* instance) {
		return instance->size();
	}

	bool cv_VectorOfRotatedRect_is_empty(const std::vector<cv::RotatedRect>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfRotatedRect_capacity(const std::vector<cv::RotatedRect>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfRotatedRect_shrink_to_fit(std::vector<cv::RotatedRect>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfRotatedRect_reserve(std::vector<cv::RotatedRect>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfRotatedRect_remove(std::vector<cv::RotatedRect>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfRotatedRect_swap(std::vector<cv::RotatedRect>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfRotatedRect_clear(std::vector<cv::RotatedRect>* instance) {
		instance->clear();
	}

	void cv_VectorOfRotatedRect_push(std::vector<cv::RotatedRect>* instance, cv::RotatedRect* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfRotatedRect_insert(std::vector<cv::RotatedRect>* instance, size_t index, cv::RotatedRect* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<cv::RotatedRect*> cv_VectorOfRotatedRect_get(const std::vector<cv::RotatedRect>* instance, size_t index) {
		return Ok<cv::RotatedRect*>(new cv::RotatedRect((*instance)[index]));
	}

	void cv_VectorOfRotatedRect_set(std::vector<cv::RotatedRect>* instance, size_t index, cv::RotatedRect* val) {
		(*instance)[index] = *val;
	}

}


extern "C" {
	void cv_VectorOfScalar_delete(std::vector<cv::Scalar>* instance) {
		delete instance;
	}

	std::vector<cv::Scalar>* cv_VectorOfScalar_new() {
		return new std::vector<cv::Scalar>();
	}

	size_t cv_VectorOfScalar_len(const std::vector<cv::Scalar>* instance) {
		return instance->size();
	}

	bool cv_VectorOfScalar_is_empty(const std::vector<cv::Scalar>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfScalar_capacity(const std::vector<cv::Scalar>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfScalar_shrink_to_fit(std::vector<cv::Scalar>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfScalar_reserve(std::vector<cv::Scalar>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfScalar_remove(std::vector<cv::Scalar>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfScalar_swap(std::vector<cv::Scalar>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfScalar_clear(std::vector<cv::Scalar>* instance) {
		instance->clear();
	}

	void cv_VectorOfScalar_push(std::vector<cv::Scalar>* instance, const cv::Scalar* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfScalar_insert(std::vector<cv::Scalar>* instance, size_t index, const cv::Scalar* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<cv::Scalar> cv_VectorOfScalar_get(const std::vector<cv::Scalar>* instance, size_t index) {
		return Ok<cv::Scalar>((*instance)[index]);
	}

	void cv_VectorOfScalar_set(std::vector<cv::Scalar>* instance, size_t index, const cv::Scalar* val) {
		(*instance)[index] = *val;
	}

	const cv::Scalar* cv_VectorOfScalar_data(std::vector<cv::Scalar>* instance) {
		return instance->data();
	}
	
	Result<cv::_InputArray*> cv_VectorOfScalar_input_array(std::vector<cv::Scalar>* instance) {
		try {
			return Ok(new cv::_InputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputArray*>)
	}
	
	Result<cv::_OutputArray*> cv_VectorOfScalar_output_array(std::vector<cv::Scalar>* instance) {
		try {
			return Ok(new cv::_OutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_OutputArray*>)
	}
	
	Result<cv::_InputOutputArray*> cv_VectorOfScalar_input_output_array(std::vector<cv::Scalar>* instance) {
		try {
			return Ok(new cv::_InputOutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputOutputArray*>)
	}
	
}


extern "C" {
	void cv_VectorOfSize_delete(std::vector<cv::Size>* instance) {
		delete instance;
	}

	std::vector<cv::Size>* cv_VectorOfSize_new() {
		return new std::vector<cv::Size>();
	}

	size_t cv_VectorOfSize_len(const std::vector<cv::Size>* instance) {
		return instance->size();
	}

	bool cv_VectorOfSize_is_empty(const std::vector<cv::Size>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfSize_capacity(const std::vector<cv::Size>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfSize_shrink_to_fit(std::vector<cv::Size>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfSize_reserve(std::vector<cv::Size>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfSize_remove(std::vector<cv::Size>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfSize_swap(std::vector<cv::Size>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfSize_clear(std::vector<cv::Size>* instance) {
		instance->clear();
	}

	void cv_VectorOfSize_push(std::vector<cv::Size>* instance, const cv::Size* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfSize_insert(std::vector<cv::Size>* instance, size_t index, const cv::Size* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<cv::Size> cv_VectorOfSize_get(const std::vector<cv::Size>* instance, size_t index) {
		return Ok<cv::Size>((*instance)[index]);
	}

	void cv_VectorOfSize_set(std::vector<cv::Size>* instance, size_t index, const cv::Size* val) {
		(*instance)[index] = *val;
	}

	const cv::Size* cv_VectorOfSize_data(std::vector<cv::Size>* instance) {
		return instance->data();
	}
	
	Result<cv::_InputArray*> cv_VectorOfSize_input_array(std::vector<cv::Size>* instance) {
		try {
			return Ok(new cv::_InputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputArray*>)
	}
	
	Result<cv::_OutputArray*> cv_VectorOfSize_output_array(std::vector<cv::Size>* instance) {
		try {
			return Ok(new cv::_OutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_OutputArray*>)
	}
	
	Result<cv::_InputOutputArray*> cv_VectorOfSize_input_output_array(std::vector<cv::Size>* instance) {
		try {
			return Ok(new cv::_InputOutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputOutputArray*>)
	}
	
}


extern "C" {
	void cv_VectorOfString_delete(std::vector<cv::String>* instance) {
		delete instance;
	}

	std::vector<cv::String>* cv_VectorOfString_new() {
		return new std::vector<cv::String>();
	}

	size_t cv_VectorOfString_len(const std::vector<cv::String>* instance) {
		return instance->size();
	}

	bool cv_VectorOfString_is_empty(const std::vector<cv::String>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfString_capacity(const std::vector<cv::String>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfString_shrink_to_fit(std::vector<cv::String>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfString_reserve(std::vector<cv::String>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfString_remove(std::vector<cv::String>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfString_swap(std::vector<cv::String>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfString_clear(std::vector<cv::String>* instance) {
		instance->clear();
	}

	void cv_VectorOfString_push(std::vector<cv::String>* instance, char* val) {
		instance->push_back(std::string(val));
	}

	void cv_VectorOfString_insert(std::vector<cv::String>* instance, size_t index, char* val) {
		instance->insert(instance->begin() + index, std::string(val));
	}

	Result<void*> cv_VectorOfString_get(const std::vector<cv::String>* instance, size_t index) {
		return Ok<void*>(ocvrs_create_string((*instance)[index].c_str()));
	}

	void cv_VectorOfString_set(std::vector<cv::String>* instance, size_t index, char* val) {
		(*instance)[index] = std::string(val);
	}

}


extern "C" {
	void cv_VectorOfUMat_delete(std::vector<cv::UMat>* instance) {
		delete instance;
	}

	std::vector<cv::UMat>* cv_VectorOfUMat_new() {
		return new std::vector<cv::UMat>();
	}

	size_t cv_VectorOfUMat_len(const std::vector<cv::UMat>* instance) {
		return instance->size();
	}

	bool cv_VectorOfUMat_is_empty(const std::vector<cv::UMat>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfUMat_capacity(const std::vector<cv::UMat>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfUMat_shrink_to_fit(std::vector<cv::UMat>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfUMat_reserve(std::vector<cv::UMat>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfUMat_remove(std::vector<cv::UMat>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfUMat_swap(std::vector<cv::UMat>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfUMat_clear(std::vector<cv::UMat>* instance) {
		instance->clear();
	}

	void cv_VectorOfUMat_push(std::vector<cv::UMat>* instance, cv::UMat* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfUMat_insert(std::vector<cv::UMat>* instance, size_t index, cv::UMat* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<cv::UMat*> cv_VectorOfUMat_get(const std::vector<cv::UMat>* instance, size_t index) {
		return Ok<cv::UMat*>(new cv::UMat((*instance)[index]));
	}

	void cv_VectorOfUMat_set(std::vector<cv::UMat>* instance, size_t index, cv::UMat* val) {
		(*instance)[index] = *val;
	}

}


extern "C" {
	void cv_VectorOfVec2i_delete(std::vector<cv::Vec2i>* instance) {
		delete instance;
	}

	std::vector<cv::Vec2i>* cv_VectorOfVec2i_new() {
		return new std::vector<cv::Vec2i>();
	}

	size_t cv_VectorOfVec2i_len(const std::vector<cv::Vec2i>* instance) {
		return instance->size();
	}

	bool cv_VectorOfVec2i_is_empty(const std::vector<cv::Vec2i>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfVec2i_capacity(const std::vector<cv::Vec2i>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfVec2i_shrink_to_fit(std::vector<cv::Vec2i>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfVec2i_reserve(std::vector<cv::Vec2i>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfVec2i_remove(std::vector<cv::Vec2i>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfVec2i_swap(std::vector<cv::Vec2i>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfVec2i_clear(std::vector<cv::Vec2i>* instance) {
		instance->clear();
	}

	void cv_VectorOfVec2i_push(std::vector<cv::Vec2i>* instance, const cv::Vec2i* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfVec2i_insert(std::vector<cv::Vec2i>* instance, size_t index, const cv::Vec2i* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<cv::Vec2i> cv_VectorOfVec2i_get(const std::vector<cv::Vec2i>* instance, size_t index) {
		return Ok<cv::Vec2i>((*instance)[index]);
	}

	void cv_VectorOfVec2i_set(std::vector<cv::Vec2i>* instance, size_t index, const cv::Vec2i* val) {
		(*instance)[index] = *val;
	}

	const cv::Vec2i* cv_VectorOfVec2i_data(std::vector<cv::Vec2i>* instance) {
		return instance->data();
	}
	
	Result<cv::_InputArray*> cv_VectorOfVec2i_input_array(std::vector<cv::Vec2i>* instance) {
		try {
			return Ok(new cv::_InputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputArray*>)
	}
	
	Result<cv::_OutputArray*> cv_VectorOfVec2i_output_array(std::vector<cv::Vec2i>* instance) {
		try {
			return Ok(new cv::_OutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_OutputArray*>)
	}
	
	Result<cv::_InputOutputArray*> cv_VectorOfVec2i_input_output_array(std::vector<cv::Vec2i>* instance) {
		try {
			return Ok(new cv::_InputOutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputOutputArray*>)
	}
	
}


extern "C" {
	void cv_VectorOfVec3d_delete(std::vector<cv::Vec3d>* instance) {
		delete instance;
	}

	std::vector<cv::Vec3d>* cv_VectorOfVec3d_new() {
		return new std::vector<cv::Vec3d>();
	}

	size_t cv_VectorOfVec3d_len(const std::vector<cv::Vec3d>* instance) {
		return instance->size();
	}

	bool cv_VectorOfVec3d_is_empty(const std::vector<cv::Vec3d>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfVec3d_capacity(const std::vector<cv::Vec3d>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfVec3d_shrink_to_fit(std::vector<cv::Vec3d>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfVec3d_reserve(std::vector<cv::Vec3d>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfVec3d_remove(std::vector<cv::Vec3d>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfVec3d_swap(std::vector<cv::Vec3d>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfVec3d_clear(std::vector<cv::Vec3d>* instance) {
		instance->clear();
	}

	void cv_VectorOfVec3d_push(std::vector<cv::Vec3d>* instance, const cv::Vec3d* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfVec3d_insert(std::vector<cv::Vec3d>* instance, size_t index, const cv::Vec3d* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<cv::Vec3d> cv_VectorOfVec3d_get(const std::vector<cv::Vec3d>* instance, size_t index) {
		return Ok<cv::Vec3d>((*instance)[index]);
	}

	void cv_VectorOfVec3d_set(std::vector<cv::Vec3d>* instance, size_t index, const cv::Vec3d* val) {
		(*instance)[index] = *val;
	}

	const cv::Vec3d* cv_VectorOfVec3d_data(std::vector<cv::Vec3d>* instance) {
		return instance->data();
	}
	
	Result<cv::_InputArray*> cv_VectorOfVec3d_input_array(std::vector<cv::Vec3d>* instance) {
		try {
			return Ok(new cv::_InputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputArray*>)
	}
	
	Result<cv::_OutputArray*> cv_VectorOfVec3d_output_array(std::vector<cv::Vec3d>* instance) {
		try {
			return Ok(new cv::_OutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_OutputArray*>)
	}
	
	Result<cv::_InputOutputArray*> cv_VectorOfVec3d_input_output_array(std::vector<cv::Vec3d>* instance) {
		try {
			return Ok(new cv::_InputOutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputOutputArray*>)
	}
	
}


extern "C" {
	void cv_VectorOfVec3f_delete(std::vector<cv::Vec3f>* instance) {
		delete instance;
	}

	std::vector<cv::Vec3f>* cv_VectorOfVec3f_new() {
		return new std::vector<cv::Vec3f>();
	}

	size_t cv_VectorOfVec3f_len(const std::vector<cv::Vec3f>* instance) {
		return instance->size();
	}

	bool cv_VectorOfVec3f_is_empty(const std::vector<cv::Vec3f>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfVec3f_capacity(const std::vector<cv::Vec3f>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfVec3f_shrink_to_fit(std::vector<cv::Vec3f>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfVec3f_reserve(std::vector<cv::Vec3f>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfVec3f_remove(std::vector<cv::Vec3f>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfVec3f_swap(std::vector<cv::Vec3f>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfVec3f_clear(std::vector<cv::Vec3f>* instance) {
		instance->clear();
	}

	void cv_VectorOfVec3f_push(std::vector<cv::Vec3f>* instance, const cv::Vec3f* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfVec3f_insert(std::vector<cv::Vec3f>* instance, size_t index, const cv::Vec3f* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<cv::Vec3f> cv_VectorOfVec3f_get(const std::vector<cv::Vec3f>* instance, size_t index) {
		return Ok<cv::Vec3f>((*instance)[index]);
	}

	void cv_VectorOfVec3f_set(std::vector<cv::Vec3f>* instance, size_t index, const cv::Vec3f* val) {
		(*instance)[index] = *val;
	}

	const cv::Vec3f* cv_VectorOfVec3f_data(std::vector<cv::Vec3f>* instance) {
		return instance->data();
	}
	
	Result<cv::_InputArray*> cv_VectorOfVec3f_input_array(std::vector<cv::Vec3f>* instance) {
		try {
			return Ok(new cv::_InputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputArray*>)
	}
	
	Result<cv::_OutputArray*> cv_VectorOfVec3f_output_array(std::vector<cv::Vec3f>* instance) {
		try {
			return Ok(new cv::_OutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_OutputArray*>)
	}
	
	Result<cv::_InputOutputArray*> cv_VectorOfVec3f_input_output_array(std::vector<cv::Vec3f>* instance) {
		try {
			return Ok(new cv::_InputOutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputOutputArray*>)
	}
	
}


extern "C" {
	void cv_VectorOfVec3i_delete(std::vector<cv::Vec3i>* instance) {
		delete instance;
	}

	std::vector<cv::Vec3i>* cv_VectorOfVec3i_new() {
		return new std::vector<cv::Vec3i>();
	}

	size_t cv_VectorOfVec3i_len(const std::vector<cv::Vec3i>* instance) {
		return instance->size();
	}

	bool cv_VectorOfVec3i_is_empty(const std::vector<cv::Vec3i>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfVec3i_capacity(const std::vector<cv::Vec3i>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfVec3i_shrink_to_fit(std::vector<cv::Vec3i>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfVec3i_reserve(std::vector<cv::Vec3i>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfVec3i_remove(std::vector<cv::Vec3i>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfVec3i_swap(std::vector<cv::Vec3i>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfVec3i_clear(std::vector<cv::Vec3i>* instance) {
		instance->clear();
	}

	void cv_VectorOfVec3i_push(std::vector<cv::Vec3i>* instance, const cv::Vec3i* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfVec3i_insert(std::vector<cv::Vec3i>* instance, size_t index, const cv::Vec3i* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<cv::Vec3i> cv_VectorOfVec3i_get(const std::vector<cv::Vec3i>* instance, size_t index) {
		return Ok<cv::Vec3i>((*instance)[index]);
	}

	void cv_VectorOfVec3i_set(std::vector<cv::Vec3i>* instance, size_t index, const cv::Vec3i* val) {
		(*instance)[index] = *val;
	}

	const cv::Vec3i* cv_VectorOfVec3i_data(std::vector<cv::Vec3i>* instance) {
		return instance->data();
	}
	
	Result<cv::_InputArray*> cv_VectorOfVec3i_input_array(std::vector<cv::Vec3i>* instance) {
		try {
			return Ok(new cv::_InputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputArray*>)
	}
	
	Result<cv::_OutputArray*> cv_VectorOfVec3i_output_array(std::vector<cv::Vec3i>* instance) {
		try {
			return Ok(new cv::_OutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_OutputArray*>)
	}
	
	Result<cv::_InputOutputArray*> cv_VectorOfVec3i_input_output_array(std::vector<cv::Vec3i>* instance) {
		try {
			return Ok(new cv::_InputOutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputOutputArray*>)
	}
	
}


extern "C" {
	void cv_VectorOfVec4f_delete(std::vector<cv::Vec4f>* instance) {
		delete instance;
	}

	std::vector<cv::Vec4f>* cv_VectorOfVec4f_new() {
		return new std::vector<cv::Vec4f>();
	}

	size_t cv_VectorOfVec4f_len(const std::vector<cv::Vec4f>* instance) {
		return instance->size();
	}

	bool cv_VectorOfVec4f_is_empty(const std::vector<cv::Vec4f>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfVec4f_capacity(const std::vector<cv::Vec4f>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfVec4f_shrink_to_fit(std::vector<cv::Vec4f>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfVec4f_reserve(std::vector<cv::Vec4f>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfVec4f_remove(std::vector<cv::Vec4f>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfVec4f_swap(std::vector<cv::Vec4f>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfVec4f_clear(std::vector<cv::Vec4f>* instance) {
		instance->clear();
	}

	void cv_VectorOfVec4f_push(std::vector<cv::Vec4f>* instance, const cv::Vec4f* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfVec4f_insert(std::vector<cv::Vec4f>* instance, size_t index, const cv::Vec4f* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<cv::Vec4f> cv_VectorOfVec4f_get(const std::vector<cv::Vec4f>* instance, size_t index) {
		return Ok<cv::Vec4f>((*instance)[index]);
	}

	void cv_VectorOfVec4f_set(std::vector<cv::Vec4f>* instance, size_t index, const cv::Vec4f* val) {
		(*instance)[index] = *val;
	}

	const cv::Vec4f* cv_VectorOfVec4f_data(std::vector<cv::Vec4f>* instance) {
		return instance->data();
	}
	
	Result<cv::_InputArray*> cv_VectorOfVec4f_input_array(std::vector<cv::Vec4f>* instance) {
		try {
			return Ok(new cv::_InputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputArray*>)
	}
	
	Result<cv::_OutputArray*> cv_VectorOfVec4f_output_array(std::vector<cv::Vec4f>* instance) {
		try {
			return Ok(new cv::_OutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_OutputArray*>)
	}
	
	Result<cv::_InputOutputArray*> cv_VectorOfVec4f_input_output_array(std::vector<cv::Vec4f>* instance) {
		try {
			return Ok(new cv::_InputOutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputOutputArray*>)
	}
	
}


extern "C" {
	void cv_VectorOfVec4i_delete(std::vector<cv::Vec4i>* instance) {
		delete instance;
	}

	std::vector<cv::Vec4i>* cv_VectorOfVec4i_new() {
		return new std::vector<cv::Vec4i>();
	}

	size_t cv_VectorOfVec4i_len(const std::vector<cv::Vec4i>* instance) {
		return instance->size();
	}

	bool cv_VectorOfVec4i_is_empty(const std::vector<cv::Vec4i>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfVec4i_capacity(const std::vector<cv::Vec4i>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfVec4i_shrink_to_fit(std::vector<cv::Vec4i>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfVec4i_reserve(std::vector<cv::Vec4i>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfVec4i_remove(std::vector<cv::Vec4i>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfVec4i_swap(std::vector<cv::Vec4i>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfVec4i_clear(std::vector<cv::Vec4i>* instance) {
		instance->clear();
	}

	void cv_VectorOfVec4i_push(std::vector<cv::Vec4i>* instance, const cv::Vec4i* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfVec4i_insert(std::vector<cv::Vec4i>* instance, size_t index, const cv::Vec4i* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<cv::Vec4i> cv_VectorOfVec4i_get(const std::vector<cv::Vec4i>* instance, size_t index) {
		return Ok<cv::Vec4i>((*instance)[index]);
	}

	void cv_VectorOfVec4i_set(std::vector<cv::Vec4i>* instance, size_t index, const cv::Vec4i* val) {
		(*instance)[index] = *val;
	}

	const cv::Vec4i* cv_VectorOfVec4i_data(std::vector<cv::Vec4i>* instance) {
		return instance->data();
	}
	
	Result<cv::_InputArray*> cv_VectorOfVec4i_input_array(std::vector<cv::Vec4i>* instance) {
		try {
			return Ok(new cv::_InputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputArray*>)
	}
	
	Result<cv::_OutputArray*> cv_VectorOfVec4i_output_array(std::vector<cv::Vec4i>* instance) {
		try {
			return Ok(new cv::_OutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_OutputArray*>)
	}
	
	Result<cv::_InputOutputArray*> cv_VectorOfVec4i_input_output_array(std::vector<cv::Vec4i>* instance) {
		try {
			return Ok(new cv::_InputOutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputOutputArray*>)
	}
	
}


extern "C" {
	void cv_VectorOfVec6f_delete(std::vector<cv::Vec6f>* instance) {
		delete instance;
	}

	std::vector<cv::Vec6f>* cv_VectorOfVec6f_new() {
		return new std::vector<cv::Vec6f>();
	}

	size_t cv_VectorOfVec6f_len(const std::vector<cv::Vec6f>* instance) {
		return instance->size();
	}

	bool cv_VectorOfVec6f_is_empty(const std::vector<cv::Vec6f>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfVec6f_capacity(const std::vector<cv::Vec6f>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfVec6f_shrink_to_fit(std::vector<cv::Vec6f>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfVec6f_reserve(std::vector<cv::Vec6f>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfVec6f_remove(std::vector<cv::Vec6f>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfVec6f_swap(std::vector<cv::Vec6f>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfVec6f_clear(std::vector<cv::Vec6f>* instance) {
		instance->clear();
	}

	void cv_VectorOfVec6f_push(std::vector<cv::Vec6f>* instance, const cv::Vec6f* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfVec6f_insert(std::vector<cv::Vec6f>* instance, size_t index, const cv::Vec6f* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<cv::Vec6f> cv_VectorOfVec6f_get(const std::vector<cv::Vec6f>* instance, size_t index) {
		return Ok<cv::Vec6f>((*instance)[index]);
	}

	void cv_VectorOfVec6f_set(std::vector<cv::Vec6f>* instance, size_t index, const cv::Vec6f* val) {
		(*instance)[index] = *val;
	}

	const cv::Vec6f* cv_VectorOfVec6f_data(std::vector<cv::Vec6f>* instance) {
		return instance->data();
	}
	
	Result<cv::_InputArray*> cv_VectorOfVec6f_input_array(std::vector<cv::Vec6f>* instance) {
		try {
			return Ok(new cv::_InputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputArray*>)
	}
	
	Result<cv::_OutputArray*> cv_VectorOfVec6f_output_array(std::vector<cv::Vec6f>* instance) {
		try {
			return Ok(new cv::_OutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_OutputArray*>)
	}
	
	Result<cv::_InputOutputArray*> cv_VectorOfVec6f_input_output_array(std::vector<cv::Vec6f>* instance) {
		try {
			return Ok(new cv::_InputOutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputOutputArray*>)
	}
	
}


extern "C" {
	void cv_VectorOfVectorOfDMatch_delete(std::vector<std::vector<cv::DMatch>>* instance) {
		delete instance;
	}

	std::vector<std::vector<cv::DMatch>>* cv_VectorOfVectorOfDMatch_new() {
		return new std::vector<std::vector<cv::DMatch>>();
	}

	size_t cv_VectorOfVectorOfDMatch_len(const std::vector<std::vector<cv::DMatch>>* instance) {
		return instance->size();
	}

	bool cv_VectorOfVectorOfDMatch_is_empty(const std::vector<std::vector<cv::DMatch>>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfVectorOfDMatch_capacity(const std::vector<std::vector<cv::DMatch>>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfVectorOfDMatch_shrink_to_fit(std::vector<std::vector<cv::DMatch>>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfVectorOfDMatch_reserve(std::vector<std::vector<cv::DMatch>>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfVectorOfDMatch_remove(std::vector<std::vector<cv::DMatch>>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfVectorOfDMatch_swap(std::vector<std::vector<cv::DMatch>>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfVectorOfDMatch_clear(std::vector<std::vector<cv::DMatch>>* instance) {
		instance->clear();
	}

	void cv_VectorOfVectorOfDMatch_push(std::vector<std::vector<cv::DMatch>>* instance, std::vector<cv::DMatch>* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfVectorOfDMatch_insert(std::vector<std::vector<cv::DMatch>>* instance, size_t index, std::vector<cv::DMatch>* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<std::vector<cv::DMatch>*> cv_VectorOfVectorOfDMatch_get(const std::vector<std::vector<cv::DMatch>>* instance, size_t index) {
		return Ok<std::vector<cv::DMatch>*>(new std::vector<cv::DMatch>((*instance)[index]));
	}

	void cv_VectorOfVectorOfDMatch_set(std::vector<std::vector<cv::DMatch>>* instance, size_t index, std::vector<cv::DMatch>* val) {
		(*instance)[index] = *val;
	}

}


extern "C" {
	void cv_VectorOfVectorOfKeyPoint_delete(std::vector<std::vector<cv::KeyPoint>>* instance) {
		delete instance;
	}

	std::vector<std::vector<cv::KeyPoint>>* cv_VectorOfVectorOfKeyPoint_new() {
		return new std::vector<std::vector<cv::KeyPoint>>();
	}

	size_t cv_VectorOfVectorOfKeyPoint_len(const std::vector<std::vector<cv::KeyPoint>>* instance) {
		return instance->size();
	}

	bool cv_VectorOfVectorOfKeyPoint_is_empty(const std::vector<std::vector<cv::KeyPoint>>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfVectorOfKeyPoint_capacity(const std::vector<std::vector<cv::KeyPoint>>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfVectorOfKeyPoint_shrink_to_fit(std::vector<std::vector<cv::KeyPoint>>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfVectorOfKeyPoint_reserve(std::vector<std::vector<cv::KeyPoint>>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfVectorOfKeyPoint_remove(std::vector<std::vector<cv::KeyPoint>>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfVectorOfKeyPoint_swap(std::vector<std::vector<cv::KeyPoint>>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfVectorOfKeyPoint_clear(std::vector<std::vector<cv::KeyPoint>>* instance) {
		instance->clear();
	}

	void cv_VectorOfVectorOfKeyPoint_push(std::vector<std::vector<cv::KeyPoint>>* instance, std::vector<cv::KeyPoint>* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfVectorOfKeyPoint_insert(std::vector<std::vector<cv::KeyPoint>>* instance, size_t index, std::vector<cv::KeyPoint>* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<std::vector<cv::KeyPoint>*> cv_VectorOfVectorOfKeyPoint_get(const std::vector<std::vector<cv::KeyPoint>>* instance, size_t index) {
		return Ok<std::vector<cv::KeyPoint>*>(new std::vector<cv::KeyPoint>((*instance)[index]));
	}

	void cv_VectorOfVectorOfKeyPoint_set(std::vector<std::vector<cv::KeyPoint>>* instance, size_t index, std::vector<cv::KeyPoint>* val) {
		(*instance)[index] = *val;
	}

}


extern "C" {
	void cv_VectorOfVectorOfMat_delete(std::vector<std::vector<cv::Mat>>* instance) {
		delete instance;
	}

	std::vector<std::vector<cv::Mat>>* cv_VectorOfVectorOfMat_new() {
		return new std::vector<std::vector<cv::Mat>>();
	}

	size_t cv_VectorOfVectorOfMat_len(const std::vector<std::vector<cv::Mat>>* instance) {
		return instance->size();
	}

	bool cv_VectorOfVectorOfMat_is_empty(const std::vector<std::vector<cv::Mat>>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfVectorOfMat_capacity(const std::vector<std::vector<cv::Mat>>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfVectorOfMat_shrink_to_fit(std::vector<std::vector<cv::Mat>>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfVectorOfMat_reserve(std::vector<std::vector<cv::Mat>>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfVectorOfMat_remove(std::vector<std::vector<cv::Mat>>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfVectorOfMat_swap(std::vector<std::vector<cv::Mat>>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfVectorOfMat_clear(std::vector<std::vector<cv::Mat>>* instance) {
		instance->clear();
	}

	void cv_VectorOfVectorOfMat_push(std::vector<std::vector<cv::Mat>>* instance, std::vector<cv::Mat>* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfVectorOfMat_insert(std::vector<std::vector<cv::Mat>>* instance, size_t index, std::vector<cv::Mat>* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<std::vector<cv::Mat>*> cv_VectorOfVectorOfMat_get(const std::vector<std::vector<cv::Mat>>* instance, size_t index) {
		return Ok<std::vector<cv::Mat>*>(new std::vector<cv::Mat>((*instance)[index]));
	}

	void cv_VectorOfVectorOfMat_set(std::vector<std::vector<cv::Mat>>* instance, size_t index, std::vector<cv::Mat>* val) {
		(*instance)[index] = *val;
	}

}


extern "C" {
	void cv_VectorOfVectorOfPoint_delete(std::vector<std::vector<cv::Point>>* instance) {
		delete instance;
	}

	std::vector<std::vector<cv::Point>>* cv_VectorOfVectorOfPoint_new() {
		return new std::vector<std::vector<cv::Point>>();
	}

	size_t cv_VectorOfVectorOfPoint_len(const std::vector<std::vector<cv::Point>>* instance) {
		return instance->size();
	}

	bool cv_VectorOfVectorOfPoint_is_empty(const std::vector<std::vector<cv::Point>>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfVectorOfPoint_capacity(const std::vector<std::vector<cv::Point>>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfVectorOfPoint_shrink_to_fit(std::vector<std::vector<cv::Point>>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfVectorOfPoint_reserve(std::vector<std::vector<cv::Point>>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfVectorOfPoint_remove(std::vector<std::vector<cv::Point>>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfVectorOfPoint_swap(std::vector<std::vector<cv::Point>>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfVectorOfPoint_clear(std::vector<std::vector<cv::Point>>* instance) {
		instance->clear();
	}

	void cv_VectorOfVectorOfPoint_push(std::vector<std::vector<cv::Point>>* instance, std::vector<cv::Point>* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfVectorOfPoint_insert(std::vector<std::vector<cv::Point>>* instance, size_t index, std::vector<cv::Point>* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<std::vector<cv::Point>*> cv_VectorOfVectorOfPoint_get(const std::vector<std::vector<cv::Point>>* instance, size_t index) {
		return Ok<std::vector<cv::Point>*>(new std::vector<cv::Point>((*instance)[index]));
	}

	void cv_VectorOfVectorOfPoint_set(std::vector<std::vector<cv::Point>>* instance, size_t index, std::vector<cv::Point>* val) {
		(*instance)[index] = *val;
	}

	Result<cv::_InputArray*> cv_VectorOfVectorOfPoint_input_array(std::vector<std::vector<cv::Point>>* instance) {
		try {
			return Ok(new cv::_InputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputArray*>)
	}
	
	Result<cv::_OutputArray*> cv_VectorOfVectorOfPoint_output_array(std::vector<std::vector<cv::Point>>* instance) {
		try {
			return Ok(new cv::_OutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_OutputArray*>)
	}
	
	Result<cv::_InputOutputArray*> cv_VectorOfVectorOfPoint_input_output_array(std::vector<std::vector<cv::Point>>* instance) {
		try {
			return Ok(new cv::_InputOutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputOutputArray*>)
	}
	
}


extern "C" {
	void cv_VectorOfVectorOfPoint2f_delete(std::vector<std::vector<cv::Point2f>>* instance) {
		delete instance;
	}

	std::vector<std::vector<cv::Point2f>>* cv_VectorOfVectorOfPoint2f_new() {
		return new std::vector<std::vector<cv::Point2f>>();
	}

	size_t cv_VectorOfVectorOfPoint2f_len(const std::vector<std::vector<cv::Point2f>>* instance) {
		return instance->size();
	}

	bool cv_VectorOfVectorOfPoint2f_is_empty(const std::vector<std::vector<cv::Point2f>>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfVectorOfPoint2f_capacity(const std::vector<std::vector<cv::Point2f>>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfVectorOfPoint2f_shrink_to_fit(std::vector<std::vector<cv::Point2f>>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfVectorOfPoint2f_reserve(std::vector<std::vector<cv::Point2f>>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfVectorOfPoint2f_remove(std::vector<std::vector<cv::Point2f>>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfVectorOfPoint2f_swap(std::vector<std::vector<cv::Point2f>>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfVectorOfPoint2f_clear(std::vector<std::vector<cv::Point2f>>* instance) {
		instance->clear();
	}

	void cv_VectorOfVectorOfPoint2f_push(std::vector<std::vector<cv::Point2f>>* instance, std::vector<cv::Point2f>* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfVectorOfPoint2f_insert(std::vector<std::vector<cv::Point2f>>* instance, size_t index, std::vector<cv::Point2f>* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<std::vector<cv::Point2f>*> cv_VectorOfVectorOfPoint2f_get(const std::vector<std::vector<cv::Point2f>>* instance, size_t index) {
		return Ok<std::vector<cv::Point2f>*>(new std::vector<cv::Point2f>((*instance)[index]));
	}

	void cv_VectorOfVectorOfPoint2f_set(std::vector<std::vector<cv::Point2f>>* instance, size_t index, std::vector<cv::Point2f>* val) {
		(*instance)[index] = *val;
	}

	Result<cv::_InputArray*> cv_VectorOfVectorOfPoint2f_input_array(std::vector<std::vector<cv::Point2f>>* instance) {
		try {
			return Ok(new cv::_InputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputArray*>)
	}
	
	Result<cv::_OutputArray*> cv_VectorOfVectorOfPoint2f_output_array(std::vector<std::vector<cv::Point2f>>* instance) {
		try {
			return Ok(new cv::_OutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_OutputArray*>)
	}
	
	Result<cv::_InputOutputArray*> cv_VectorOfVectorOfPoint2f_input_output_array(std::vector<std::vector<cv::Point2f>>* instance) {
		try {
			return Ok(new cv::_InputOutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputOutputArray*>)
	}
	
}


extern "C" {
	void cv_VectorOfVectorOfPoint3d_delete(std::vector<std::vector<cv::Point3d>>* instance) {
		delete instance;
	}

	std::vector<std::vector<cv::Point3d>>* cv_VectorOfVectorOfPoint3d_new() {
		return new std::vector<std::vector<cv::Point3d>>();
	}

	size_t cv_VectorOfVectorOfPoint3d_len(const std::vector<std::vector<cv::Point3d>>* instance) {
		return instance->size();
	}

	bool cv_VectorOfVectorOfPoint3d_is_empty(const std::vector<std::vector<cv::Point3d>>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfVectorOfPoint3d_capacity(const std::vector<std::vector<cv::Point3d>>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfVectorOfPoint3d_shrink_to_fit(std::vector<std::vector<cv::Point3d>>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfVectorOfPoint3d_reserve(std::vector<std::vector<cv::Point3d>>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfVectorOfPoint3d_remove(std::vector<std::vector<cv::Point3d>>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfVectorOfPoint3d_swap(std::vector<std::vector<cv::Point3d>>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfVectorOfPoint3d_clear(std::vector<std::vector<cv::Point3d>>* instance) {
		instance->clear();
	}

	void cv_VectorOfVectorOfPoint3d_push(std::vector<std::vector<cv::Point3d>>* instance, std::vector<cv::Point3d>* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfVectorOfPoint3d_insert(std::vector<std::vector<cv::Point3d>>* instance, size_t index, std::vector<cv::Point3d>* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<std::vector<cv::Point3d>*> cv_VectorOfVectorOfPoint3d_get(const std::vector<std::vector<cv::Point3d>>* instance, size_t index) {
		return Ok<std::vector<cv::Point3d>*>(new std::vector<cv::Point3d>((*instance)[index]));
	}

	void cv_VectorOfVectorOfPoint3d_set(std::vector<std::vector<cv::Point3d>>* instance, size_t index, std::vector<cv::Point3d>* val) {
		(*instance)[index] = *val;
	}

	Result<cv::_InputArray*> cv_VectorOfVectorOfPoint3d_input_array(std::vector<std::vector<cv::Point3d>>* instance) {
		try {
			return Ok(new cv::_InputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputArray*>)
	}
	
	Result<cv::_OutputArray*> cv_VectorOfVectorOfPoint3d_output_array(std::vector<std::vector<cv::Point3d>>* instance) {
		try {
			return Ok(new cv::_OutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_OutputArray*>)
	}
	
	Result<cv::_InputOutputArray*> cv_VectorOfVectorOfPoint3d_input_output_array(std::vector<std::vector<cv::Point3d>>* instance) {
		try {
			return Ok(new cv::_InputOutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputOutputArray*>)
	}
	
}


extern "C" {
	void cv_VectorOfVectorOfPoint3f_delete(std::vector<std::vector<cv::Point3f>>* instance) {
		delete instance;
	}

	std::vector<std::vector<cv::Point3f>>* cv_VectorOfVectorOfPoint3f_new() {
		return new std::vector<std::vector<cv::Point3f>>();
	}

	size_t cv_VectorOfVectorOfPoint3f_len(const std::vector<std::vector<cv::Point3f>>* instance) {
		return instance->size();
	}

	bool cv_VectorOfVectorOfPoint3f_is_empty(const std::vector<std::vector<cv::Point3f>>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfVectorOfPoint3f_capacity(const std::vector<std::vector<cv::Point3f>>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfVectorOfPoint3f_shrink_to_fit(std::vector<std::vector<cv::Point3f>>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfVectorOfPoint3f_reserve(std::vector<std::vector<cv::Point3f>>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfVectorOfPoint3f_remove(std::vector<std::vector<cv::Point3f>>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfVectorOfPoint3f_swap(std::vector<std::vector<cv::Point3f>>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfVectorOfPoint3f_clear(std::vector<std::vector<cv::Point3f>>* instance) {
		instance->clear();
	}

	void cv_VectorOfVectorOfPoint3f_push(std::vector<std::vector<cv::Point3f>>* instance, std::vector<cv::Point3f>* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfVectorOfPoint3f_insert(std::vector<std::vector<cv::Point3f>>* instance, size_t index, std::vector<cv::Point3f>* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<std::vector<cv::Point3f>*> cv_VectorOfVectorOfPoint3f_get(const std::vector<std::vector<cv::Point3f>>* instance, size_t index) {
		return Ok<std::vector<cv::Point3f>*>(new std::vector<cv::Point3f>((*instance)[index]));
	}

	void cv_VectorOfVectorOfPoint3f_set(std::vector<std::vector<cv::Point3f>>* instance, size_t index, std::vector<cv::Point3f>* val) {
		(*instance)[index] = *val;
	}

	Result<cv::_InputArray*> cv_VectorOfVectorOfPoint3f_input_array(std::vector<std::vector<cv::Point3f>>* instance) {
		try {
			return Ok(new cv::_InputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputArray*>)
	}
	
	Result<cv::_OutputArray*> cv_VectorOfVectorOfPoint3f_output_array(std::vector<std::vector<cv::Point3f>>* instance) {
		try {
			return Ok(new cv::_OutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_OutputArray*>)
	}
	
	Result<cv::_InputOutputArray*> cv_VectorOfVectorOfPoint3f_input_output_array(std::vector<std::vector<cv::Point3f>>* instance) {
		try {
			return Ok(new cv::_InputOutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputOutputArray*>)
	}
	
}


extern "C" {
	void cv_VectorOfVectorOfPoint3i_delete(std::vector<std::vector<cv::Point3i>>* instance) {
		delete instance;
	}

	std::vector<std::vector<cv::Point3i>>* cv_VectorOfVectorOfPoint3i_new() {
		return new std::vector<std::vector<cv::Point3i>>();
	}

	size_t cv_VectorOfVectorOfPoint3i_len(const std::vector<std::vector<cv::Point3i>>* instance) {
		return instance->size();
	}

	bool cv_VectorOfVectorOfPoint3i_is_empty(const std::vector<std::vector<cv::Point3i>>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfVectorOfPoint3i_capacity(const std::vector<std::vector<cv::Point3i>>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfVectorOfPoint3i_shrink_to_fit(std::vector<std::vector<cv::Point3i>>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfVectorOfPoint3i_reserve(std::vector<std::vector<cv::Point3i>>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfVectorOfPoint3i_remove(std::vector<std::vector<cv::Point3i>>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfVectorOfPoint3i_swap(std::vector<std::vector<cv::Point3i>>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfVectorOfPoint3i_clear(std::vector<std::vector<cv::Point3i>>* instance) {
		instance->clear();
	}

	void cv_VectorOfVectorOfPoint3i_push(std::vector<std::vector<cv::Point3i>>* instance, std::vector<cv::Point3i>* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfVectorOfPoint3i_insert(std::vector<std::vector<cv::Point3i>>* instance, size_t index, std::vector<cv::Point3i>* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<std::vector<cv::Point3i>*> cv_VectorOfVectorOfPoint3i_get(const std::vector<std::vector<cv::Point3i>>* instance, size_t index) {
		return Ok<std::vector<cv::Point3i>*>(new std::vector<cv::Point3i>((*instance)[index]));
	}

	void cv_VectorOfVectorOfPoint3i_set(std::vector<std::vector<cv::Point3i>>* instance, size_t index, std::vector<cv::Point3i>* val) {
		(*instance)[index] = *val;
	}

	Result<cv::_InputArray*> cv_VectorOfVectorOfPoint3i_input_array(std::vector<std::vector<cv::Point3i>>* instance) {
		try {
			return Ok(new cv::_InputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputArray*>)
	}
	
	Result<cv::_OutputArray*> cv_VectorOfVectorOfPoint3i_output_array(std::vector<std::vector<cv::Point3i>>* instance) {
		try {
			return Ok(new cv::_OutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_OutputArray*>)
	}
	
	Result<cv::_InputOutputArray*> cv_VectorOfVectorOfPoint3i_input_output_array(std::vector<std::vector<cv::Point3i>>* instance) {
		try {
			return Ok(new cv::_InputOutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputOutputArray*>)
	}
	
}


extern "C" {
	void cv_VectorOfVectorOfRange_delete(std::vector<std::vector<cv::Range>>* instance) {
		delete instance;
	}

	std::vector<std::vector<cv::Range>>* cv_VectorOfVectorOfRange_new() {
		return new std::vector<std::vector<cv::Range>>();
	}

	size_t cv_VectorOfVectorOfRange_len(const std::vector<std::vector<cv::Range>>* instance) {
		return instance->size();
	}

	bool cv_VectorOfVectorOfRange_is_empty(const std::vector<std::vector<cv::Range>>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfVectorOfRange_capacity(const std::vector<std::vector<cv::Range>>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfVectorOfRange_shrink_to_fit(std::vector<std::vector<cv::Range>>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfVectorOfRange_reserve(std::vector<std::vector<cv::Range>>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfVectorOfRange_remove(std::vector<std::vector<cv::Range>>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfVectorOfRange_swap(std::vector<std::vector<cv::Range>>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfVectorOfRange_clear(std::vector<std::vector<cv::Range>>* instance) {
		instance->clear();
	}

	void cv_VectorOfVectorOfRange_push(std::vector<std::vector<cv::Range>>* instance, std::vector<cv::Range>* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfVectorOfRange_insert(std::vector<std::vector<cv::Range>>* instance, size_t index, std::vector<cv::Range>* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<std::vector<cv::Range>*> cv_VectorOfVectorOfRange_get(const std::vector<std::vector<cv::Range>>* instance, size_t index) {
		return Ok<std::vector<cv::Range>*>(new std::vector<cv::Range>((*instance)[index]));
	}

	void cv_VectorOfVectorOfRange_set(std::vector<std::vector<cv::Range>>* instance, size_t index, std::vector<cv::Range>* val) {
		(*instance)[index] = *val;
	}

}


extern "C" {
	void cv_VectorOfVectorOfVec2i_delete(std::vector<std::vector<cv::Vec2i>>* instance) {
		delete instance;
	}

	std::vector<std::vector<cv::Vec2i>>* cv_VectorOfVectorOfVec2i_new() {
		return new std::vector<std::vector<cv::Vec2i>>();
	}

	size_t cv_VectorOfVectorOfVec2i_len(const std::vector<std::vector<cv::Vec2i>>* instance) {
		return instance->size();
	}

	bool cv_VectorOfVectorOfVec2i_is_empty(const std::vector<std::vector<cv::Vec2i>>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfVectorOfVec2i_capacity(const std::vector<std::vector<cv::Vec2i>>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfVectorOfVec2i_shrink_to_fit(std::vector<std::vector<cv::Vec2i>>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfVectorOfVec2i_reserve(std::vector<std::vector<cv::Vec2i>>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfVectorOfVec2i_remove(std::vector<std::vector<cv::Vec2i>>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfVectorOfVec2i_swap(std::vector<std::vector<cv::Vec2i>>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfVectorOfVec2i_clear(std::vector<std::vector<cv::Vec2i>>* instance) {
		instance->clear();
	}

	void cv_VectorOfVectorOfVec2i_push(std::vector<std::vector<cv::Vec2i>>* instance, std::vector<cv::Vec2i>* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfVectorOfVec2i_insert(std::vector<std::vector<cv::Vec2i>>* instance, size_t index, std::vector<cv::Vec2i>* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<std::vector<cv::Vec2i>*> cv_VectorOfVectorOfVec2i_get(const std::vector<std::vector<cv::Vec2i>>* instance, size_t index) {
		return Ok<std::vector<cv::Vec2i>*>(new std::vector<cv::Vec2i>((*instance)[index]));
	}

	void cv_VectorOfVectorOfVec2i_set(std::vector<std::vector<cv::Vec2i>>* instance, size_t index, std::vector<cv::Vec2i>* val) {
		(*instance)[index] = *val;
	}

	Result<cv::_InputArray*> cv_VectorOfVectorOfVec2i_input_array(std::vector<std::vector<cv::Vec2i>>* instance) {
		try {
			return Ok(new cv::_InputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputArray*>)
	}
	
	Result<cv::_OutputArray*> cv_VectorOfVectorOfVec2i_output_array(std::vector<std::vector<cv::Vec2i>>* instance) {
		try {
			return Ok(new cv::_OutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_OutputArray*>)
	}
	
	Result<cv::_InputOutputArray*> cv_VectorOfVectorOfVec2i_input_output_array(std::vector<std::vector<cv::Vec2i>>* instance) {
		try {
			return Ok(new cv::_InputOutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputOutputArray*>)
	}
	
}


extern "C" {
	void cv_VectorOfVectorOff64_delete(std::vector<std::vector<double>>* instance) {
		delete instance;
	}

	std::vector<std::vector<double>>* cv_VectorOfVectorOff64_new() {
		return new std::vector<std::vector<double>>();
	}

	size_t cv_VectorOfVectorOff64_len(const std::vector<std::vector<double>>* instance) {
		return instance->size();
	}

	bool cv_VectorOfVectorOff64_is_empty(const std::vector<std::vector<double>>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfVectorOff64_capacity(const std::vector<std::vector<double>>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfVectorOff64_shrink_to_fit(std::vector<std::vector<double>>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfVectorOff64_reserve(std::vector<std::vector<double>>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfVectorOff64_remove(std::vector<std::vector<double>>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfVectorOff64_swap(std::vector<std::vector<double>>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfVectorOff64_clear(std::vector<std::vector<double>>* instance) {
		instance->clear();
	}

	void cv_VectorOfVectorOff64_push(std::vector<std::vector<double>>* instance, std::vector<double>* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfVectorOff64_insert(std::vector<std::vector<double>>* instance, size_t index, std::vector<double>* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<std::vector<double>*> cv_VectorOfVectorOff64_get(const std::vector<std::vector<double>>* instance, size_t index) {
		return Ok<std::vector<double>*>(new std::vector<double>((*instance)[index]));
	}

	void cv_VectorOfVectorOff64_set(std::vector<std::vector<double>>* instance, size_t index, std::vector<double>* val) {
		(*instance)[index] = *val;
	}

	Result<cv::_InputArray*> cv_VectorOfVectorOff64_input_array(std::vector<std::vector<double>>* instance) {
		try {
			return Ok(new cv::_InputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputArray*>)
	}
	
	Result<cv::_OutputArray*> cv_VectorOfVectorOff64_output_array(std::vector<std::vector<double>>* instance) {
		try {
			return Ok(new cv::_OutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_OutputArray*>)
	}
	
	Result<cv::_InputOutputArray*> cv_VectorOfVectorOff64_input_output_array(std::vector<std::vector<double>>* instance) {
		try {
			return Ok(new cv::_InputOutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputOutputArray*>)
	}
	
}


extern "C" {
	void cv_VectorOfVectorOfi32_delete(std::vector<std::vector<int>>* instance) {
		delete instance;
	}

	std::vector<std::vector<int>>* cv_VectorOfVectorOfi32_new() {
		return new std::vector<std::vector<int>>();
	}

	size_t cv_VectorOfVectorOfi32_len(const std::vector<std::vector<int>>* instance) {
		return instance->size();
	}

	bool cv_VectorOfVectorOfi32_is_empty(const std::vector<std::vector<int>>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfVectorOfi32_capacity(const std::vector<std::vector<int>>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfVectorOfi32_shrink_to_fit(std::vector<std::vector<int>>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfVectorOfi32_reserve(std::vector<std::vector<int>>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfVectorOfi32_remove(std::vector<std::vector<int>>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfVectorOfi32_swap(std::vector<std::vector<int>>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfVectorOfi32_clear(std::vector<std::vector<int>>* instance) {
		instance->clear();
	}

	void cv_VectorOfVectorOfi32_push(std::vector<std::vector<int>>* instance, std::vector<int>* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfVectorOfi32_insert(std::vector<std::vector<int>>* instance, size_t index, std::vector<int>* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<std::vector<int>*> cv_VectorOfVectorOfi32_get(const std::vector<std::vector<int>>* instance, size_t index) {
		return Ok<std::vector<int>*>(new std::vector<int>((*instance)[index]));
	}

	void cv_VectorOfVectorOfi32_set(std::vector<std::vector<int>>* instance, size_t index, std::vector<int>* val) {
		(*instance)[index] = *val;
	}

	Result<cv::_InputArray*> cv_VectorOfVectorOfi32_input_array(std::vector<std::vector<int>>* instance) {
		try {
			return Ok(new cv::_InputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputArray*>)
	}
	
	Result<cv::_OutputArray*> cv_VectorOfVectorOfi32_output_array(std::vector<std::vector<int>>* instance) {
		try {
			return Ok(new cv::_OutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_OutputArray*>)
	}
	
	Result<cv::_InputOutputArray*> cv_VectorOfVectorOfi32_input_output_array(std::vector<std::vector<int>>* instance) {
		try {
			return Ok(new cv::_InputOutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputOutputArray*>)
	}
	
}


extern "C" {
	void cv_VectorOfVectorOfi8_delete(std::vector<std::vector<char>>* instance) {
		delete instance;
	}

	std::vector<std::vector<char>>* cv_VectorOfVectorOfi8_new() {
		return new std::vector<std::vector<char>>();
	}

	size_t cv_VectorOfVectorOfi8_len(const std::vector<std::vector<char>>* instance) {
		return instance->size();
	}

	bool cv_VectorOfVectorOfi8_is_empty(const std::vector<std::vector<char>>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfVectorOfi8_capacity(const std::vector<std::vector<char>>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfVectorOfi8_shrink_to_fit(std::vector<std::vector<char>>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfVectorOfi8_reserve(std::vector<std::vector<char>>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfVectorOfi8_remove(std::vector<std::vector<char>>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfVectorOfi8_swap(std::vector<std::vector<char>>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfVectorOfi8_clear(std::vector<std::vector<char>>* instance) {
		instance->clear();
	}

	void cv_VectorOfVectorOfi8_push(std::vector<std::vector<char>>* instance, std::vector<char>* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfVectorOfi8_insert(std::vector<std::vector<char>>* instance, size_t index, std::vector<char>* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<std::vector<char>*> cv_VectorOfVectorOfi8_get(const std::vector<std::vector<char>>* instance, size_t index) {
		return Ok<std::vector<char>*>(new std::vector<char>((*instance)[index]));
	}

	void cv_VectorOfVectorOfi8_set(std::vector<std::vector<char>>* instance, size_t index, std::vector<char>* val) {
		(*instance)[index] = *val;
	}

	Result<cv::_InputArray*> cv_VectorOfVectorOfi8_input_array(std::vector<std::vector<char>>* instance) {
		try {
			return Ok(new cv::_InputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputArray*>)
	}
	
	Result<cv::_OutputArray*> cv_VectorOfVectorOfi8_output_array(std::vector<std::vector<char>>* instance) {
		try {
			return Ok(new cv::_OutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_OutputArray*>)
	}
	
	Result<cv::_InputOutputArray*> cv_VectorOfVectorOfi8_input_output_array(std::vector<std::vector<char>>* instance) {
		try {
			return Ok(new cv::_InputOutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputOutputArray*>)
	}
	
}


extern "C" {
	void cv_VectorOfVectorOfu8_delete(std::vector<std::vector<unsigned char>>* instance) {
		delete instance;
	}

	std::vector<std::vector<unsigned char>>* cv_VectorOfVectorOfu8_new() {
		return new std::vector<std::vector<unsigned char>>();
	}

	size_t cv_VectorOfVectorOfu8_len(const std::vector<std::vector<unsigned char>>* instance) {
		return instance->size();
	}

	bool cv_VectorOfVectorOfu8_is_empty(const std::vector<std::vector<unsigned char>>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfVectorOfu8_capacity(const std::vector<std::vector<unsigned char>>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfVectorOfu8_shrink_to_fit(std::vector<std::vector<unsigned char>>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfVectorOfu8_reserve(std::vector<std::vector<unsigned char>>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfVectorOfu8_remove(std::vector<std::vector<unsigned char>>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfVectorOfu8_swap(std::vector<std::vector<unsigned char>>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfVectorOfu8_clear(std::vector<std::vector<unsigned char>>* instance) {
		instance->clear();
	}

	void cv_VectorOfVectorOfu8_push(std::vector<std::vector<unsigned char>>* instance, std::vector<unsigned char>* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfVectorOfu8_insert(std::vector<std::vector<unsigned char>>* instance, size_t index, std::vector<unsigned char>* val) {
		instance->insert(instance->begin() + index, *val);
	}

	Result<std::vector<unsigned char>*> cv_VectorOfVectorOfu8_get(const std::vector<std::vector<unsigned char>>* instance, size_t index) {
		return Ok<std::vector<unsigned char>*>(new std::vector<unsigned char>((*instance)[index]));
	}

	void cv_VectorOfVectorOfu8_set(std::vector<std::vector<unsigned char>>* instance, size_t index, std::vector<unsigned char>* val) {
		(*instance)[index] = *val;
	}

	Result<cv::_InputArray*> cv_VectorOfVectorOfu8_input_array(std::vector<std::vector<unsigned char>>* instance) {
		try {
			return Ok(new cv::_InputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputArray*>)
	}
	
	Result<cv::_OutputArray*> cv_VectorOfVectorOfu8_output_array(std::vector<std::vector<unsigned char>>* instance) {
		try {
			return Ok(new cv::_OutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_OutputArray*>)
	}
	
	Result<cv::_InputOutputArray*> cv_VectorOfVectorOfu8_input_output_array(std::vector<std::vector<unsigned char>>* instance) {
		try {
			return Ok(new cv::_InputOutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputOutputArray*>)
	}
	
}


extern "C" {
	void cv_VectorOfbool_delete(std::vector<bool>* instance) {
		delete instance;
	}

	std::vector<bool>* cv_VectorOfbool_new() {
		return new std::vector<bool>();
	}

	size_t cv_VectorOfbool_len(const std::vector<bool>* instance) {
		return instance->size();
	}

	bool cv_VectorOfbool_is_empty(const std::vector<bool>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfbool_capacity(const std::vector<bool>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfbool_shrink_to_fit(std::vector<bool>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfbool_reserve(std::vector<bool>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfbool_remove(std::vector<bool>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfbool_swap(std::vector<bool>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfbool_clear(std::vector<bool>* instance) {
		instance->clear();
	}

	void cv_VectorOfbool_push(std::vector<bool>* instance, bool val) {
		instance->push_back(val);
	}

	void cv_VectorOfbool_insert(std::vector<bool>* instance, size_t index, bool val) {
		instance->insert(instance->begin() + index, val);
	}

	Result<bool> cv_VectorOfbool_get(const std::vector<bool>* instance, size_t index) {
		return Ok<bool>((*instance)[index]);
	}

	void cv_VectorOfbool_set(std::vector<bool>* instance, size_t index, bool val) {
		(*instance)[index] = val;
	}

}


extern "C" {
	void cv_VectorOff32_delete(std::vector<float>* instance) {
		delete instance;
	}

	std::vector<float>* cv_VectorOff32_new() {
		return new std::vector<float>();
	}

	size_t cv_VectorOff32_len(const std::vector<float>* instance) {
		return instance->size();
	}

	bool cv_VectorOff32_is_empty(const std::vector<float>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOff32_capacity(const std::vector<float>* instance) {
		return instance->capacity();
	}

	void cv_VectorOff32_shrink_to_fit(std::vector<float>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOff32_reserve(std::vector<float>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOff32_remove(std::vector<float>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOff32_swap(std::vector<float>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOff32_clear(std::vector<float>* instance) {
		instance->clear();
	}

	void cv_VectorOff32_push(std::vector<float>* instance, float val) {
		instance->push_back(val);
	}

	void cv_VectorOff32_insert(std::vector<float>* instance, size_t index, float val) {
		instance->insert(instance->begin() + index, val);
	}

	Result<float> cv_VectorOff32_get(const std::vector<float>* instance, size_t index) {
		return Ok<float>((*instance)[index]);
	}

	void cv_VectorOff32_set(std::vector<float>* instance, size_t index, float val) {
		(*instance)[index] = val;
	}

	const float* cv_VectorOff32_data(std::vector<float>* instance) {
		return instance->data();
	}
	
	Result<cv::_InputArray*> cv_VectorOff32_input_array(std::vector<float>* instance) {
		try {
			return Ok(new cv::_InputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputArray*>)
	}
	
	Result<cv::_OutputArray*> cv_VectorOff32_output_array(std::vector<float>* instance) {
		try {
			return Ok(new cv::_OutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_OutputArray*>)
	}
	
	Result<cv::_InputOutputArray*> cv_VectorOff32_input_output_array(std::vector<float>* instance) {
		try {
			return Ok(new cv::_InputOutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputOutputArray*>)
	}
	
}


extern "C" {
	void cv_VectorOff64_delete(std::vector<double>* instance) {
		delete instance;
	}

	std::vector<double>* cv_VectorOff64_new() {
		return new std::vector<double>();
	}

	size_t cv_VectorOff64_len(const std::vector<double>* instance) {
		return instance->size();
	}

	bool cv_VectorOff64_is_empty(const std::vector<double>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOff64_capacity(const std::vector<double>* instance) {
		return instance->capacity();
	}

	void cv_VectorOff64_shrink_to_fit(std::vector<double>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOff64_reserve(std::vector<double>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOff64_remove(std::vector<double>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOff64_swap(std::vector<double>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOff64_clear(std::vector<double>* instance) {
		instance->clear();
	}

	void cv_VectorOff64_push(std::vector<double>* instance, double val) {
		instance->push_back(val);
	}

	void cv_VectorOff64_insert(std::vector<double>* instance, size_t index, double val) {
		instance->insert(instance->begin() + index, val);
	}

	Result<double> cv_VectorOff64_get(const std::vector<double>* instance, size_t index) {
		return Ok<double>((*instance)[index]);
	}

	void cv_VectorOff64_set(std::vector<double>* instance, size_t index, double val) {
		(*instance)[index] = val;
	}

	const double* cv_VectorOff64_data(std::vector<double>* instance) {
		return instance->data();
	}
	
	Result<cv::_InputArray*> cv_VectorOff64_input_array(std::vector<double>* instance) {
		try {
			return Ok(new cv::_InputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputArray*>)
	}
	
	Result<cv::_OutputArray*> cv_VectorOff64_output_array(std::vector<double>* instance) {
		try {
			return Ok(new cv::_OutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_OutputArray*>)
	}
	
	Result<cv::_InputOutputArray*> cv_VectorOff64_input_output_array(std::vector<double>* instance) {
		try {
			return Ok(new cv::_InputOutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputOutputArray*>)
	}
	
}


extern "C" {
	void cv_VectorOfi32_delete(std::vector<int>* instance) {
		delete instance;
	}

	std::vector<int>* cv_VectorOfi32_new() {
		return new std::vector<int>();
	}

	size_t cv_VectorOfi32_len(const std::vector<int>* instance) {
		return instance->size();
	}

	bool cv_VectorOfi32_is_empty(const std::vector<int>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfi32_capacity(const std::vector<int>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfi32_shrink_to_fit(std::vector<int>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfi32_reserve(std::vector<int>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfi32_remove(std::vector<int>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfi32_swap(std::vector<int>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfi32_clear(std::vector<int>* instance) {
		instance->clear();
	}

	void cv_VectorOfi32_push(std::vector<int>* instance, int val) {
		instance->push_back(val);
	}

	void cv_VectorOfi32_insert(std::vector<int>* instance, size_t index, int val) {
		instance->insert(instance->begin() + index, val);
	}

	Result<int> cv_VectorOfi32_get(const std::vector<int>* instance, size_t index) {
		return Ok<int>((*instance)[index]);
	}

	void cv_VectorOfi32_set(std::vector<int>* instance, size_t index, int val) {
		(*instance)[index] = val;
	}

	const int* cv_VectorOfi32_data(std::vector<int>* instance) {
		return instance->data();
	}
	
	Result<cv::_InputArray*> cv_VectorOfi32_input_array(std::vector<int>* instance) {
		try {
			return Ok(new cv::_InputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputArray*>)
	}
	
	Result<cv::_OutputArray*> cv_VectorOfi32_output_array(std::vector<int>* instance) {
		try {
			return Ok(new cv::_OutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_OutputArray*>)
	}
	
	Result<cv::_InputOutputArray*> cv_VectorOfi32_input_output_array(std::vector<int>* instance) {
		try {
			return Ok(new cv::_InputOutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputOutputArray*>)
	}
	
}


extern "C" {
	void cv_VectorOfi8_delete(std::vector<char>* instance) {
		delete instance;
	}

	std::vector<char>* cv_VectorOfi8_new() {
		return new std::vector<char>();
	}

	size_t cv_VectorOfi8_len(const std::vector<char>* instance) {
		return instance->size();
	}

	bool cv_VectorOfi8_is_empty(const std::vector<char>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfi8_capacity(const std::vector<char>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfi8_shrink_to_fit(std::vector<char>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfi8_reserve(std::vector<char>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfi8_remove(std::vector<char>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfi8_swap(std::vector<char>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfi8_clear(std::vector<char>* instance) {
		instance->clear();
	}

	void cv_VectorOfi8_push(std::vector<char>* instance, char val) {
		instance->push_back(val);
	}

	void cv_VectorOfi8_insert(std::vector<char>* instance, size_t index, char val) {
		instance->insert(instance->begin() + index, val);
	}

	Result<char> cv_VectorOfi8_get(const std::vector<char>* instance, size_t index) {
		return Ok<char>((*instance)[index]);
	}

	void cv_VectorOfi8_set(std::vector<char>* instance, size_t index, char val) {
		(*instance)[index] = val;
	}

	const char* cv_VectorOfi8_data(std::vector<char>* instance) {
		return instance->data();
	}
	
	Result<cv::_InputArray*> cv_VectorOfi8_input_array(std::vector<char>* instance) {
		try {
			return Ok(new cv::_InputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputArray*>)
	}
	
	Result<cv::_OutputArray*> cv_VectorOfi8_output_array(std::vector<char>* instance) {
		try {
			return Ok(new cv::_OutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_OutputArray*>)
	}
	
	Result<cv::_InputOutputArray*> cv_VectorOfi8_input_output_array(std::vector<char>* instance) {
		try {
			return Ok(new cv::_InputOutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputOutputArray*>)
	}
	
}


extern "C" {
	void cv_VectorOfsize_t_delete(std::vector<size_t>* instance) {
		delete instance;
	}

	std::vector<size_t>* cv_VectorOfsize_t_new() {
		return new std::vector<size_t>();
	}

	size_t cv_VectorOfsize_t_len(const std::vector<size_t>* instance) {
		return instance->size();
	}

	bool cv_VectorOfsize_t_is_empty(const std::vector<size_t>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfsize_t_capacity(const std::vector<size_t>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfsize_t_shrink_to_fit(std::vector<size_t>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfsize_t_reserve(std::vector<size_t>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfsize_t_remove(std::vector<size_t>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfsize_t_swap(std::vector<size_t>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfsize_t_clear(std::vector<size_t>* instance) {
		instance->clear();
	}

	void cv_VectorOfsize_t_push(std::vector<size_t>* instance, size_t val) {
		instance->push_back(val);
	}

	void cv_VectorOfsize_t_insert(std::vector<size_t>* instance, size_t index, size_t val) {
		instance->insert(instance->begin() + index, val);
	}

	Result<size_t> cv_VectorOfsize_t_get(const std::vector<size_t>* instance, size_t index) {
		return Ok<size_t>((*instance)[index]);
	}

	void cv_VectorOfsize_t_set(std::vector<size_t>* instance, size_t index, size_t val) {
		(*instance)[index] = val;
	}

	const size_t* cv_VectorOfsize_t_data(std::vector<size_t>* instance) {
		return instance->data();
	}
	
}


extern "C" {
	void cv_VectorOfu8_delete(std::vector<unsigned char>* instance) {
		delete instance;
	}

	std::vector<unsigned char>* cv_VectorOfu8_new() {
		return new std::vector<unsigned char>();
	}

	size_t cv_VectorOfu8_len(const std::vector<unsigned char>* instance) {
		return instance->size();
	}

	bool cv_VectorOfu8_is_empty(const std::vector<unsigned char>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfu8_capacity(const std::vector<unsigned char>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfu8_shrink_to_fit(std::vector<unsigned char>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfu8_reserve(std::vector<unsigned char>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfu8_remove(std::vector<unsigned char>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfu8_swap(std::vector<unsigned char>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfu8_clear(std::vector<unsigned char>* instance) {
		instance->clear();
	}

	void cv_VectorOfu8_push(std::vector<unsigned char>* instance, unsigned char val) {
		instance->push_back(val);
	}

	void cv_VectorOfu8_insert(std::vector<unsigned char>* instance, size_t index, unsigned char val) {
		instance->insert(instance->begin() + index, val);
	}

	Result<unsigned char> cv_VectorOfu8_get(const std::vector<unsigned char>* instance, size_t index) {
		return Ok<unsigned char>((*instance)[index]);
	}

	void cv_VectorOfu8_set(std::vector<unsigned char>* instance, size_t index, unsigned char val) {
		(*instance)[index] = val;
	}

	const unsigned char* cv_VectorOfu8_data(std::vector<unsigned char>* instance) {
		return instance->data();
	}
	
	Result<cv::_InputArray*> cv_VectorOfu8_input_array(std::vector<unsigned char>* instance) {
		try {
			return Ok(new cv::_InputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputArray*>)
	}
	
	Result<cv::_OutputArray*> cv_VectorOfu8_output_array(std::vector<unsigned char>* instance) {
		try {
			return Ok(new cv::_OutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_OutputArray*>)
	}
	
	Result<cv::_InputOutputArray*> cv_VectorOfu8_input_output_array(std::vector<unsigned char>* instance) {
		try {
			return Ok(new cv::_InputOutputArray(*instance));
		} OCVRS_CATCH(Result<cv::_InputOutputArray*>)
	}
	
}


