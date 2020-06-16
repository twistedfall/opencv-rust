template struct Result<bool>;
template struct Result<cv::Ptr<cv::cuda::DisparityBilateralFilter>*>;
template struct Result<cv::Ptr<cv::cuda::StereoBM>*>;
template struct Result<cv::Ptr<cv::cuda::StereoBeliefPropagation>*>;
template struct Result<cv::Ptr<cv::cuda::StereoConstantSpaceBP>*>;
template struct Result<double>;
template struct Result<int>;
extern "C" void cv_PtrOfCUDA_DisparityBilateralFilter_delete(cv::Ptr<cv::cuda::DisparityBilateralFilter>* instance) {
	delete instance;
}

extern "C" cv::cuda::DisparityBilateralFilter* cv_PtrOfCUDA_DisparityBilateralFilter_get_inner_ptr(cv::Ptr<cv::cuda::DisparityBilateralFilter>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfCUDA_StereoBM_delete(cv::Ptr<cv::cuda::StereoBM>* instance) {
	delete instance;
}

extern "C" cv::cuda::StereoBM* cv_PtrOfCUDA_StereoBM_get_inner_ptr(cv::Ptr<cv::cuda::StereoBM>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfCUDA_StereoBeliefPropagation_delete(cv::Ptr<cv::cuda::StereoBeliefPropagation>* instance) {
	delete instance;
}

extern "C" cv::cuda::StereoBeliefPropagation* cv_PtrOfCUDA_StereoBeliefPropagation_get_inner_ptr(cv::Ptr<cv::cuda::StereoBeliefPropagation>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfCUDA_StereoConstantSpaceBP_delete(cv::Ptr<cv::cuda::StereoConstantSpaceBP>* instance) {
	delete instance;
}

extern "C" cv::cuda::StereoConstantSpaceBP* cv_PtrOfCUDA_StereoConstantSpaceBP_get_inner_ptr(cv::Ptr<cv::cuda::StereoConstantSpaceBP>* instance) {
	return instance->get();
}

