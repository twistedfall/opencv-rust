template struct Result<bool>;
template struct Result<const cv::Ptr<cv::Feature2D>*>;
template struct Result<const cv::Ptr<cv::LMSolver::Callback>*>;
template struct Result<cv::CirclesGridFinderParameters>;
template struct Result<cv::CirclesGridFinderParameters::GridType>;
template struct Result<cv::Mat*>;
template struct Result<cv::Ptr<cv::LMSolver>*>;
template struct Result<cv::Ptr<cv::StereoBM>*>;
template struct Result<cv::Ptr<cv::StereoSGBM>*>;
template struct Result<cv::Rect_<int>>;
template struct Result<cv::Scalar_<double>>;
template struct Result<cv::Size_<float>>;
template struct Result<cv::Vec<double, 3>>;
template struct Result<double>;
template struct Result<float>;
template struct Result<int>;
template struct Result<std::vector<cv::Point3_<double>>*>;
template struct Result<std::vector<cv::Point3_<int>>*>;
template struct Result<std::vector<std::vector<cv::Point3_<double>>>*>;
template struct Result<std::vector<std::vector<cv::Point3_<int>>>*>;
extern "C" {
	void cv_PtrOfLMSolver_delete(cv::Ptr<cv::LMSolver>* instance) {
		delete instance;
	}

	cv::LMSolver* cv_PtrOfLMSolver_get_inner_ptr(cv::Ptr<cv::LMSolver>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfLMSolver_Callback_delete(cv::Ptr<cv::LMSolver::Callback>* instance) {
		delete instance;
	}

	cv::LMSolver::Callback* cv_PtrOfLMSolver_Callback_get_inner_ptr(cv::Ptr<cv::LMSolver::Callback>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfStereoBM_delete(cv::Ptr<cv::StereoBM>* instance) {
		delete instance;
	}

	cv::StereoBM* cv_PtrOfStereoBM_get_inner_ptr(cv::Ptr<cv::StereoBM>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfStereoMatcher_delete(cv::Ptr<cv::StereoMatcher>* instance) {
		delete instance;
	}

	cv::StereoMatcher* cv_PtrOfStereoMatcher_get_inner_ptr(cv::Ptr<cv::StereoMatcher>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfStereoSGBM_delete(cv::Ptr<cv::StereoSGBM>* instance) {
		delete instance;
	}

	cv::StereoSGBM* cv_PtrOfStereoSGBM_get_inner_ptr(cv::Ptr<cv::StereoSGBM>* instance) {
		return instance->get();
	}
}

