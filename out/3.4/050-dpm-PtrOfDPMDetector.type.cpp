extern "C" {
	// cv::Ptr<cv::dpm::DPMDetector>::getInnerPtr() generated
	// ("cv::Ptr<cv::dpm::DPMDetector>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dpm::DPMDetector* cv_PtrLcv_dpm_DPMDetectorG_getInnerPtr_const(const cv::Ptr<cv::dpm::DPMDetector>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dpm::DPMDetector>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dpm::DPMDetector>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dpm::DPMDetector* cv_PtrLcv_dpm_DPMDetectorG_getInnerPtrMut(cv::Ptr<cv::dpm::DPMDetector>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dpm::DPMDetector>::new_null() generated
	// ("cv::Ptr<cv::dpm::DPMDetector>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dpm::DPMDetector>* cv_PtrLcv_dpm_DPMDetectorG_new_null_const() {
			return new cv::Ptr<cv::dpm::DPMDetector>();
	}

	// cv::Ptr<cv::dpm::DPMDetector>::delete() generated
	// ("cv::Ptr<cv::dpm::DPMDetector>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dpm_DPMDetectorG_delete(cv::Ptr<cv::dpm::DPMDetector>* instance) {
			delete instance;
	}

}

