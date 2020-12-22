template struct Result<bool>;
template struct Result<const cv::Mat*>;
template struct Result<cv::KalmanFilter*>;
template struct Result<cv::Mat*>;
template struct Result<cv::Ptr<cv::BackgroundSubtractorKNN>*>;
template struct Result<cv::Ptr<cv::BackgroundSubtractorMOG2>*>;
template struct Result<cv::Ptr<cv::DISOpticalFlow>*>;
template struct Result<cv::Ptr<cv::FarnebackOpticalFlow>*>;
template struct Result<cv::Ptr<cv::SparsePyrLKOpticalFlow>*>;
template struct Result<cv::Ptr<cv::TrackerGOTURN>*>;
template struct Result<cv::Ptr<cv::TrackerMIL>*>;
template struct Result<cv::Ptr<cv::VariationalRefinement>*>;
template struct Result<cv::RotatedRect*>;
template struct Result<cv::Size_<int>>;
template struct Result<cv::TermCriteria>;
template struct Result<cv::TrackerGOTURN::Params*>;
template struct Result<cv::TrackerMIL::Params>;
template struct Result<double>;
template struct Result<float>;
template struct Result<int>;
template struct Result<void*>;
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
	void cv_PtrOfDISOpticalFlow_delete(cv::Ptr<cv::DISOpticalFlow>* instance) {
		delete instance;
	}

	const cv::DISOpticalFlow* cv_PtrOfDISOpticalFlow_get_inner_ptr(const cv::Ptr<cv::DISOpticalFlow>* instance) {
		return instance->get();
	}

	cv::DISOpticalFlow* cv_PtrOfDISOpticalFlow_get_inner_ptr_mut(cv::Ptr<cv::DISOpticalFlow>* instance) {
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
	void cv_PtrOfSparseOpticalFlow_delete(cv::Ptr<cv::SparseOpticalFlow>* instance) {
		delete instance;
	}

	const cv::SparseOpticalFlow* cv_PtrOfSparseOpticalFlow_get_inner_ptr(const cv::Ptr<cv::SparseOpticalFlow>* instance) {
		return instance->get();
	}

	cv::SparseOpticalFlow* cv_PtrOfSparseOpticalFlow_get_inner_ptr_mut(cv::Ptr<cv::SparseOpticalFlow>* instance) {
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

extern "C" {
	void cv_PtrOfTrackerGOTURN_delete(cv::Ptr<cv::TrackerGOTURN>* instance) {
		delete instance;
	}

	const cv::TrackerGOTURN* cv_PtrOfTrackerGOTURN_get_inner_ptr(const cv::Ptr<cv::TrackerGOTURN>* instance) {
		return instance->get();
	}

	cv::TrackerGOTURN* cv_PtrOfTrackerGOTURN_get_inner_ptr_mut(cv::Ptr<cv::TrackerGOTURN>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfTrackerMIL_delete(cv::Ptr<cv::TrackerMIL>* instance) {
		delete instance;
	}

	const cv::TrackerMIL* cv_PtrOfTrackerMIL_get_inner_ptr(const cv::Ptr<cv::TrackerMIL>* instance) {
		return instance->get();
	}

	cv::TrackerMIL* cv_PtrOfTrackerMIL_get_inner_ptr_mut(cv::Ptr<cv::TrackerMIL>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfVariationalRefinement_delete(cv::Ptr<cv::VariationalRefinement>* instance) {
		delete instance;
	}

	const cv::VariationalRefinement* cv_PtrOfVariationalRefinement_get_inner_ptr(const cv::Ptr<cv::VariationalRefinement>* instance) {
		return instance->get();
	}

	cv::VariationalRefinement* cv_PtrOfVariationalRefinement_get_inner_ptr_mut(cv::Ptr<cv::VariationalRefinement>* instance) {
		return instance->get();
	}
}

