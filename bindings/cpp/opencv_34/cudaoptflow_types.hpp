template struct Result<bool>;
template struct Result<cv::Ptr<cv::cuda::BroxOpticalFlow>*>;
template struct Result<cv::Ptr<cv::cuda::DensePyrLKOpticalFlow>*>;
template struct Result<cv::Ptr<cv::cuda::FarnebackOpticalFlow>*>;
template struct Result<cv::Ptr<cv::cuda::OpticalFlowDual_TVL1>*>;
template struct Result<cv::Ptr<cv::cuda::SparsePyrLKOpticalFlow>*>;
template struct Result<cv::Size_<int>>;
template struct Result<double>;
template struct Result<int>;
extern "C" {
	void cv_PtrOfCUDA_BroxOpticalFlow_delete(cv::Ptr<cv::cuda::BroxOpticalFlow>* instance) {
		delete instance;
	}

	const cv::cuda::BroxOpticalFlow* cv_PtrOfCUDA_BroxOpticalFlow_get_inner_ptr(const cv::Ptr<cv::cuda::BroxOpticalFlow>* instance) {
		return instance->get();
	}

	cv::cuda::BroxOpticalFlow* cv_PtrOfCUDA_BroxOpticalFlow_get_inner_ptr_mut(cv::Ptr<cv::cuda::BroxOpticalFlow>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfCUDA_DensePyrLKOpticalFlow_delete(cv::Ptr<cv::cuda::DensePyrLKOpticalFlow>* instance) {
		delete instance;
	}

	const cv::cuda::DensePyrLKOpticalFlow* cv_PtrOfCUDA_DensePyrLKOpticalFlow_get_inner_ptr(const cv::Ptr<cv::cuda::DensePyrLKOpticalFlow>* instance) {
		return instance->get();
	}

	cv::cuda::DensePyrLKOpticalFlow* cv_PtrOfCUDA_DensePyrLKOpticalFlow_get_inner_ptr_mut(cv::Ptr<cv::cuda::DensePyrLKOpticalFlow>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfCUDA_FarnebackOpticalFlow_delete(cv::Ptr<cv::cuda::FarnebackOpticalFlow>* instance) {
		delete instance;
	}

	const cv::cuda::FarnebackOpticalFlow* cv_PtrOfCUDA_FarnebackOpticalFlow_get_inner_ptr(const cv::Ptr<cv::cuda::FarnebackOpticalFlow>* instance) {
		return instance->get();
	}

	cv::cuda::FarnebackOpticalFlow* cv_PtrOfCUDA_FarnebackOpticalFlow_get_inner_ptr_mut(cv::Ptr<cv::cuda::FarnebackOpticalFlow>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfCUDA_OpticalFlowDual_TVL1_delete(cv::Ptr<cv::cuda::OpticalFlowDual_TVL1>* instance) {
		delete instance;
	}

	const cv::cuda::OpticalFlowDual_TVL1* cv_PtrOfCUDA_OpticalFlowDual_TVL1_get_inner_ptr(const cv::Ptr<cv::cuda::OpticalFlowDual_TVL1>* instance) {
		return instance->get();
	}

	cv::cuda::OpticalFlowDual_TVL1* cv_PtrOfCUDA_OpticalFlowDual_TVL1_get_inner_ptr_mut(cv::Ptr<cv::cuda::OpticalFlowDual_TVL1>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfCUDA_SparsePyrLKOpticalFlow_delete(cv::Ptr<cv::cuda::SparsePyrLKOpticalFlow>* instance) {
		delete instance;
	}

	const cv::cuda::SparsePyrLKOpticalFlow* cv_PtrOfCUDA_SparsePyrLKOpticalFlow_get_inner_ptr(const cv::Ptr<cv::cuda::SparsePyrLKOpticalFlow>* instance) {
		return instance->get();
	}

	cv::cuda::SparsePyrLKOpticalFlow* cv_PtrOfCUDA_SparsePyrLKOpticalFlow_get_inner_ptr_mut(cv::Ptr<cv::cuda::SparsePyrLKOpticalFlow>* instance) {
		return instance->get();
	}
}

