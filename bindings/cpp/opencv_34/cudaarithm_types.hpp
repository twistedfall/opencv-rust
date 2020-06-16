template struct Result<cv::Ptr<cv::cuda::Convolution>*>;
template struct Result<cv::Ptr<cv::cuda::DFT>*>;
template struct Result<cv::Ptr<cv::cuda::LookUpTable>*>;
template struct Result<cv::Scalar_<double>>;
template struct Result<double>;
template struct Result<int>;
template struct Result<std::vector<cv::cuda::GpuMat>*>;
extern "C" void cv_PtrOfConvolution_delete(cv::Ptr<cv::cuda::Convolution>* instance) {
	delete instance;
}

extern "C" cv::cuda::Convolution* cv_PtrOfConvolution_get_inner_ptr(cv::Ptr<cv::cuda::Convolution>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfDFT_delete(cv::Ptr<cv::cuda::DFT>* instance) {
	delete instance;
}

extern "C" cv::cuda::DFT* cv_PtrOfDFT_get_inner_ptr(cv::Ptr<cv::cuda::DFT>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfLookUpTable_delete(cv::Ptr<cv::cuda::LookUpTable>* instance) {
	delete instance;
}

extern "C" cv::cuda::LookUpTable* cv_PtrOfLookUpTable_get_inner_ptr(cv::Ptr<cv::cuda::LookUpTable>* instance) {
	return instance->get();
}

