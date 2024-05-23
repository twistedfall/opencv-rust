extern "C" {
	// cv::Ptr<cv::aruco::DetectorParameters>::getInnerPtr() generated
	// ("cv::Ptr<cv::aruco::DetectorParameters>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::aruco::DetectorParameters* cv_PtrLcv_aruco_DetectorParametersG_getInnerPtr_const(const cv::Ptr<cv::aruco::DetectorParameters>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::aruco::DetectorParameters>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::aruco::DetectorParameters>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::aruco::DetectorParameters* cv_PtrLcv_aruco_DetectorParametersG_getInnerPtrMut(cv::Ptr<cv::aruco::DetectorParameters>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::aruco::DetectorParameters>::new_null() generated
	// ("cv::Ptr<cv::aruco::DetectorParameters>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::aruco::DetectorParameters>* cv_PtrLcv_aruco_DetectorParametersG_new_null_const() {
			return new cv::Ptr<cv::aruco::DetectorParameters>();
	}

	// cv::Ptr<cv::aruco::DetectorParameters>::delete() generated
	// ("cv::Ptr<cv::aruco::DetectorParameters>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_aruco_DetectorParametersG_delete(cv::Ptr<cv::aruco::DetectorParameters>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::aruco::DetectorParameters>::new(TraitClass) generated
	// ("cv::Ptr<cv::aruco::DetectorParameters>::new", vec![(pred!(const, ["val"], ["cv::aruco::DetectorParameters"]), _)]),
	cv::Ptr<cv::aruco::DetectorParameters>* cv_PtrLcv_aruco_DetectorParametersG_new_const_DetectorParameters(cv::aruco::DetectorParameters* val) {
			return new cv::Ptr<cv::aruco::DetectorParameters>(val);
	}

}

