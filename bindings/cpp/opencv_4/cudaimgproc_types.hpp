template struct Result<bool>;
template struct Result<cv::Ptr<cv::GeneralizedHoughBallard>*>;
template struct Result<cv::Ptr<cv::GeneralizedHoughGuil>*>;
template struct Result<cv::Ptr<cv::cuda::CLAHE>*>;
template struct Result<cv::Ptr<cv::cuda::CannyEdgeDetector>*>;
template struct Result<cv::Ptr<cv::cuda::CornernessCriteria>*>;
template struct Result<cv::Ptr<cv::cuda::CornersDetector>*>;
template struct Result<cv::Ptr<cv::cuda::HoughCirclesDetector>*>;
template struct Result<cv::Ptr<cv::cuda::HoughLinesDetector>*>;
template struct Result<cv::Ptr<cv::cuda::HoughSegmentDetector>*>;
template struct Result<cv::Ptr<cv::cuda::TemplateMatching>*>;
template struct Result<double>;
template struct Result<float>;
template struct Result<int>;
extern "C" {
	void cv_PtrOfCUDA_CLAHE_delete(cv::Ptr<cv::cuda::CLAHE>* instance) {
		delete instance;
	}

	const cv::cuda::CLAHE* cv_PtrOfCUDA_CLAHE_get_inner_ptr(const cv::Ptr<cv::cuda::CLAHE>* instance) {
		return instance->get();
	}

	cv::cuda::CLAHE* cv_PtrOfCUDA_CLAHE_get_inner_ptr_mut(cv::Ptr<cv::cuda::CLAHE>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfCUDA_CannyEdgeDetector_delete(cv::Ptr<cv::cuda::CannyEdgeDetector>* instance) {
		delete instance;
	}

	const cv::cuda::CannyEdgeDetector* cv_PtrOfCUDA_CannyEdgeDetector_get_inner_ptr(const cv::Ptr<cv::cuda::CannyEdgeDetector>* instance) {
		return instance->get();
	}

	cv::cuda::CannyEdgeDetector* cv_PtrOfCUDA_CannyEdgeDetector_get_inner_ptr_mut(cv::Ptr<cv::cuda::CannyEdgeDetector>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfCUDA_CornernessCriteria_delete(cv::Ptr<cv::cuda::CornernessCriteria>* instance) {
		delete instance;
	}

	const cv::cuda::CornernessCriteria* cv_PtrOfCUDA_CornernessCriteria_get_inner_ptr(const cv::Ptr<cv::cuda::CornernessCriteria>* instance) {
		return instance->get();
	}

	cv::cuda::CornernessCriteria* cv_PtrOfCUDA_CornernessCriteria_get_inner_ptr_mut(cv::Ptr<cv::cuda::CornernessCriteria>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfCUDA_CornersDetector_delete(cv::Ptr<cv::cuda::CornersDetector>* instance) {
		delete instance;
	}

	const cv::cuda::CornersDetector* cv_PtrOfCUDA_CornersDetector_get_inner_ptr(const cv::Ptr<cv::cuda::CornersDetector>* instance) {
		return instance->get();
	}

	cv::cuda::CornersDetector* cv_PtrOfCUDA_CornersDetector_get_inner_ptr_mut(cv::Ptr<cv::cuda::CornersDetector>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfCUDA_HoughCirclesDetector_delete(cv::Ptr<cv::cuda::HoughCirclesDetector>* instance) {
		delete instance;
	}

	const cv::cuda::HoughCirclesDetector* cv_PtrOfCUDA_HoughCirclesDetector_get_inner_ptr(const cv::Ptr<cv::cuda::HoughCirclesDetector>* instance) {
		return instance->get();
	}

	cv::cuda::HoughCirclesDetector* cv_PtrOfCUDA_HoughCirclesDetector_get_inner_ptr_mut(cv::Ptr<cv::cuda::HoughCirclesDetector>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfCUDA_HoughLinesDetector_delete(cv::Ptr<cv::cuda::HoughLinesDetector>* instance) {
		delete instance;
	}

	const cv::cuda::HoughLinesDetector* cv_PtrOfCUDA_HoughLinesDetector_get_inner_ptr(const cv::Ptr<cv::cuda::HoughLinesDetector>* instance) {
		return instance->get();
	}

	cv::cuda::HoughLinesDetector* cv_PtrOfCUDA_HoughLinesDetector_get_inner_ptr_mut(cv::Ptr<cv::cuda::HoughLinesDetector>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfCUDA_HoughSegmentDetector_delete(cv::Ptr<cv::cuda::HoughSegmentDetector>* instance) {
		delete instance;
	}

	const cv::cuda::HoughSegmentDetector* cv_PtrOfCUDA_HoughSegmentDetector_get_inner_ptr(const cv::Ptr<cv::cuda::HoughSegmentDetector>* instance) {
		return instance->get();
	}

	cv::cuda::HoughSegmentDetector* cv_PtrOfCUDA_HoughSegmentDetector_get_inner_ptr_mut(cv::Ptr<cv::cuda::HoughSegmentDetector>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfCUDA_TemplateMatching_delete(cv::Ptr<cv::cuda::TemplateMatching>* instance) {
		delete instance;
	}

	const cv::cuda::TemplateMatching* cv_PtrOfCUDA_TemplateMatching_get_inner_ptr(const cv::Ptr<cv::cuda::TemplateMatching>* instance) {
		return instance->get();
	}

	cv::cuda::TemplateMatching* cv_PtrOfCUDA_TemplateMatching_get_inner_ptr_mut(cv::Ptr<cv::cuda::TemplateMatching>* instance) {
		return instance->get();
	}
}

