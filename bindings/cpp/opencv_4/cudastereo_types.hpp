template struct Result<bool>;
template struct Result<cv::Ptr<cv::cuda::DisparityBilateralFilter>*>;
template struct Result<cv::Ptr<cv::cuda::StereoBM>*>;
template struct Result<cv::Ptr<cv::cuda::StereoBeliefPropagation>*>;
template struct Result<cv::Ptr<cv::cuda::StereoConstantSpaceBP>*>;
template struct Result<cv::Ptr<cv::cuda::StereoSGM>*>;
template struct Result<double>;
template struct Result<int>;
extern "C" {
	void cv_PtrOfCUDA_DisparityBilateralFilter_delete(cv::Ptr<cv::cuda::DisparityBilateralFilter>* instance) {
		delete instance;
	}

	const cv::cuda::DisparityBilateralFilter* cv_PtrOfCUDA_DisparityBilateralFilter_get_inner_ptr(const cv::Ptr<cv::cuda::DisparityBilateralFilter>* instance) {
		return instance->get();
	}

	cv::cuda::DisparityBilateralFilter* cv_PtrOfCUDA_DisparityBilateralFilter_get_inner_ptr_mut(cv::Ptr<cv::cuda::DisparityBilateralFilter>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfCUDA_StereoBM_delete(cv::Ptr<cv::cuda::StereoBM>* instance) {
		delete instance;
	}

	const cv::cuda::StereoBM* cv_PtrOfCUDA_StereoBM_get_inner_ptr(const cv::Ptr<cv::cuda::StereoBM>* instance) {
		return instance->get();
	}

	cv::cuda::StereoBM* cv_PtrOfCUDA_StereoBM_get_inner_ptr_mut(cv::Ptr<cv::cuda::StereoBM>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfCUDA_StereoBeliefPropagation_delete(cv::Ptr<cv::cuda::StereoBeliefPropagation>* instance) {
		delete instance;
	}

	const cv::cuda::StereoBeliefPropagation* cv_PtrOfCUDA_StereoBeliefPropagation_get_inner_ptr(const cv::Ptr<cv::cuda::StereoBeliefPropagation>* instance) {
		return instance->get();
	}

	cv::cuda::StereoBeliefPropagation* cv_PtrOfCUDA_StereoBeliefPropagation_get_inner_ptr_mut(cv::Ptr<cv::cuda::StereoBeliefPropagation>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfCUDA_StereoConstantSpaceBP_delete(cv::Ptr<cv::cuda::StereoConstantSpaceBP>* instance) {
		delete instance;
	}

	const cv::cuda::StereoConstantSpaceBP* cv_PtrOfCUDA_StereoConstantSpaceBP_get_inner_ptr(const cv::Ptr<cv::cuda::StereoConstantSpaceBP>* instance) {
		return instance->get();
	}

	cv::cuda::StereoConstantSpaceBP* cv_PtrOfCUDA_StereoConstantSpaceBP_get_inner_ptr_mut(cv::Ptr<cv::cuda::StereoConstantSpaceBP>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfCUDA_StereoSGM_delete(cv::Ptr<cv::cuda::StereoSGM>* instance) {
		delete instance;
	}

	const cv::cuda::StereoSGM* cv_PtrOfCUDA_StereoSGM_get_inner_ptr(const cv::Ptr<cv::cuda::StereoSGM>* instance) {
		return instance->get();
	}

	cv::cuda::StereoSGM* cv_PtrOfCUDA_StereoSGM_get_inner_ptr_mut(cv::Ptr<cv::cuda::StereoSGM>* instance) {
		return instance->get();
	}
}

