template struct Result<cv::Ptr<cv::cuda::Convolution>*>;
template struct Result<cv::Ptr<cv::cuda::DFT>*>;
template struct Result<cv::Ptr<cv::cuda::LookUpTable>*>;
template struct Result<cv::Scalar_<double>>;
template struct Result<cv::cuda::GpuMat*>;
template struct Result<double>;
template struct Result<int>;
extern "C" {
	void cv_PtrOfConvolution_delete(cv::Ptr<cv::cuda::Convolution>* instance) {
		delete instance;
	}

	const cv::cuda::Convolution* cv_PtrOfConvolution_get_inner_ptr(const cv::Ptr<cv::cuda::Convolution>* instance) {
		return instance->get();
	}

	cv::cuda::Convolution* cv_PtrOfConvolution_get_inner_ptr_mut(cv::Ptr<cv::cuda::Convolution>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfDFT_delete(cv::Ptr<cv::cuda::DFT>* instance) {
		delete instance;
	}

	const cv::cuda::DFT* cv_PtrOfDFT_get_inner_ptr(const cv::Ptr<cv::cuda::DFT>* instance) {
		return instance->get();
	}

	cv::cuda::DFT* cv_PtrOfDFT_get_inner_ptr_mut(cv::Ptr<cv::cuda::DFT>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfLookUpTable_delete(cv::Ptr<cv::cuda::LookUpTable>* instance) {
		delete instance;
	}

	const cv::cuda::LookUpTable* cv_PtrOfLookUpTable_get_inner_ptr(const cv::Ptr<cv::cuda::LookUpTable>* instance) {
		return instance->get();
	}

	cv::cuda::LookUpTable* cv_PtrOfLookUpTable_get_inner_ptr_mut(cv::Ptr<cv::cuda::LookUpTable>* instance) {
		return instance->get();
	}
}

