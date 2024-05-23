extern "C" {
	// cv::Ptr<cv::aruco::CharucoDetector>::getInnerPtr() generated
	// ("cv::Ptr<cv::aruco::CharucoDetector>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::aruco::CharucoDetector* cv_PtrLcv_aruco_CharucoDetectorG_getInnerPtr_const(const cv::Ptr<cv::aruco::CharucoDetector>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::aruco::CharucoDetector>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::aruco::CharucoDetector>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::aruco::CharucoDetector* cv_PtrLcv_aruco_CharucoDetectorG_getInnerPtrMut(cv::Ptr<cv::aruco::CharucoDetector>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::aruco::CharucoDetector>::new_null() generated
	// ("cv::Ptr<cv::aruco::CharucoDetector>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::aruco::CharucoDetector>* cv_PtrLcv_aruco_CharucoDetectorG_new_null_const() {
			return new cv::Ptr<cv::aruco::CharucoDetector>();
	}

	// cv::Ptr<cv::aruco::CharucoDetector>::delete() generated
	// ("cv::Ptr<cv::aruco::CharucoDetector>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_aruco_CharucoDetectorG_delete(cv::Ptr<cv::aruco::CharucoDetector>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::aruco::CharucoDetector>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::aruco::CharucoDetector>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_aruco_CharucoDetectorG_to_PtrOfAlgorithm(cv::Ptr<cv::aruco::CharucoDetector>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::aruco::CharucoDetector>::new(TraitClass) generated
	// ("cv::Ptr<cv::aruco::CharucoDetector>::new", vec![(pred!(const, ["val"], ["cv::aruco::CharucoDetector"]), _)]),
	cv::Ptr<cv::aruco::CharucoDetector>* cv_PtrLcv_aruco_CharucoDetectorG_new_const_CharucoDetector(cv::aruco::CharucoDetector* val) {
			return new cv::Ptr<cv::aruco::CharucoDetector>(val);
	}

}

