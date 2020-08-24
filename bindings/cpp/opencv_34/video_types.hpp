template struct Result<bool>;
template struct Result<const cv::Mat*>;
template struct Result<cv::KalmanFilter*>;
template struct Result<cv::Mat*>;
template struct Result<cv::Ptr<cv::BackgroundSubtractorKNN>*>;
template struct Result<cv::Ptr<cv::BackgroundSubtractorMOG2>*>;
template struct Result<cv::Ptr<cv::DualTVL1OpticalFlow>*>;
template struct Result<cv::Ptr<cv::FarnebackOpticalFlow>*>;
template struct Result<cv::Ptr<cv::SparsePyrLKOpticalFlow>*>;
template struct Result<cv::RotatedRect*>;
template struct Result<cv::Size_<int>>;
template struct Result<cv::TermCriteria>;
template struct Result<double>;
template struct Result<int>;
extern "C" {
	void cv_PtrOfBackgroundSubtractorKNN_delete(cv::Ptr<cv::BackgroundSubtractorKNN>* instance) {
		delete instance;
	}

	const cv::BackgroundSubtractorKNN* cv_PtrOfBackgroundSubtractorKNN_get_inner_ptr(const cv::Ptr<cv::BackgroundSubtractorKNN>* instance) {
		return instance->get();
	}

	cv::BackgroundSubtractorKNN* cv_PtrOfBackgroundSubtractorKNN_get_inner_ptr_mut(cv::Ptr<cv::BackgroundSubtractorKNN>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfBackgroundSubtractorMOG2_delete(cv::Ptr<cv::BackgroundSubtractorMOG2>* instance) {
		delete instance;
	}

	const cv::BackgroundSubtractorMOG2* cv_PtrOfBackgroundSubtractorMOG2_get_inner_ptr(const cv::Ptr<cv::BackgroundSubtractorMOG2>* instance) {
		return instance->get();
	}

	cv::BackgroundSubtractorMOG2* cv_PtrOfBackgroundSubtractorMOG2_get_inner_ptr_mut(cv::Ptr<cv::BackgroundSubtractorMOG2>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfDenseOpticalFlow_delete(cv::Ptr<cv::DenseOpticalFlow>* instance) {
		delete instance;
	}

	const cv::DenseOpticalFlow* cv_PtrOfDenseOpticalFlow_get_inner_ptr(const cv::Ptr<cv::DenseOpticalFlow>* instance) {
		return instance->get();
	}

	cv::DenseOpticalFlow* cv_PtrOfDenseOpticalFlow_get_inner_ptr_mut(cv::Ptr<cv::DenseOpticalFlow>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfDualTVL1OpticalFlow_delete(cv::Ptr<cv::DualTVL1OpticalFlow>* instance) {
		delete instance;
	}

	const cv::DualTVL1OpticalFlow* cv_PtrOfDualTVL1OpticalFlow_get_inner_ptr(const cv::Ptr<cv::DualTVL1OpticalFlow>* instance) {
		return instance->get();
	}

	cv::DualTVL1OpticalFlow* cv_PtrOfDualTVL1OpticalFlow_get_inner_ptr_mut(cv::Ptr<cv::DualTVL1OpticalFlow>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfFarnebackOpticalFlow_delete(cv::Ptr<cv::FarnebackOpticalFlow>* instance) {
		delete instance;
	}

	const cv::FarnebackOpticalFlow* cv_PtrOfFarnebackOpticalFlow_get_inner_ptr(const cv::Ptr<cv::FarnebackOpticalFlow>* instance) {
		return instance->get();
	}

	cv::FarnebackOpticalFlow* cv_PtrOfFarnebackOpticalFlow_get_inner_ptr_mut(cv::Ptr<cv::FarnebackOpticalFlow>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfSparsePyrLKOpticalFlow_delete(cv::Ptr<cv::SparsePyrLKOpticalFlow>* instance) {
		delete instance;
	}

	const cv::SparsePyrLKOpticalFlow* cv_PtrOfSparsePyrLKOpticalFlow_get_inner_ptr(const cv::Ptr<cv::SparsePyrLKOpticalFlow>* instance) {
		return instance->get();
	}

	cv::SparsePyrLKOpticalFlow* cv_PtrOfSparsePyrLKOpticalFlow_get_inner_ptr_mut(cv::Ptr<cv::SparsePyrLKOpticalFlow>* instance) {
		return instance->get();
	}
}

