extern "C" {
	// cv::Ptr<cv::aruco::ArucoDetector>::getInnerPtr() generated
	// ("cv::Ptr<cv::aruco::ArucoDetector>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::aruco::ArucoDetector* cv_PtrLcv_aruco_ArucoDetectorG_getInnerPtr_const(const cv::Ptr<cv::aruco::ArucoDetector>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::aruco::ArucoDetector>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::aruco::ArucoDetector>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::aruco::ArucoDetector* cv_PtrLcv_aruco_ArucoDetectorG_getInnerPtrMut(cv::Ptr<cv::aruco::ArucoDetector>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::aruco::ArucoDetector>::new_null() generated
	// ("cv::Ptr<cv::aruco::ArucoDetector>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::aruco::ArucoDetector>* cv_PtrLcv_aruco_ArucoDetectorG_new_null_const() {
			return new cv::Ptr<cv::aruco::ArucoDetector>();
	}

	// cv::Ptr<cv::aruco::ArucoDetector>::delete() generated
	// ("cv::Ptr<cv::aruco::ArucoDetector>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_aruco_ArucoDetectorG_delete(cv::Ptr<cv::aruco::ArucoDetector>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::aruco::ArucoDetector>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::aruco::ArucoDetector>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_aruco_ArucoDetectorG_to_PtrOfAlgorithm(cv::Ptr<cv::aruco::ArucoDetector>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::aruco::ArucoDetector>::new(TraitClass) generated
	// ("cv::Ptr<cv::aruco::ArucoDetector>::new", vec![(pred!(const, ["val"], ["cv::aruco::ArucoDetector"]), _)]),
	cv::Ptr<cv::aruco::ArucoDetector>* cv_PtrLcv_aruco_ArucoDetectorG_new_const_ArucoDetector(cv::aruco::ArucoDetector* val) {
			return new cv::Ptr<cv::aruco::ArucoDetector>(val);
	}

}

