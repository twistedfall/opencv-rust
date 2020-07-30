template struct Result<cv::Ptr<cv::rapid::OLSTracker>*>;
template struct Result<cv::Ptr<cv::rapid::Rapid>*>;
template struct Result<float>;
extern "C" {
	void cv_PtrOfOLSTracker_delete(cv::Ptr<cv::rapid::OLSTracker>* instance) {
		delete instance;
	}

	cv::rapid::OLSTracker* cv_PtrOfOLSTracker_get_inner_ptr(cv::Ptr<cv::rapid::OLSTracker>* instance) {
		return instance->get();
	}
}

extern "C" {
	void cv_PtrOfRapid_delete(cv::Ptr<cv::rapid::Rapid>* instance) {
		delete instance;
	}

	cv::rapid::Rapid* cv_PtrOfRapid_get_inner_ptr(cv::Ptr<cv::rapid::Rapid>* instance) {
		return instance->get();
	}
}

