extern "C" {
	// cv::Ptr<cv::aruco::EstimateParameters>::getInnerPtr() generated
	// ("cv::Ptr<cv::aruco::EstimateParameters>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::aruco::EstimateParameters* cv_PtrLcv_aruco_EstimateParametersG_getInnerPtr_const(const cv::Ptr<cv::aruco::EstimateParameters>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::aruco::EstimateParameters>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::aruco::EstimateParameters>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::aruco::EstimateParameters* cv_PtrLcv_aruco_EstimateParametersG_getInnerPtrMut(cv::Ptr<cv::aruco::EstimateParameters>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::aruco::EstimateParameters>::new_null() generated
	// ("cv::Ptr<cv::aruco::EstimateParameters>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::aruco::EstimateParameters>* cv_PtrLcv_aruco_EstimateParametersG_new_null_const() {
			return new cv::Ptr<cv::aruco::EstimateParameters>();
	}

	// cv::Ptr<cv::aruco::EstimateParameters>::delete() generated
	// ("cv::Ptr<cv::aruco::EstimateParameters>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_aruco_EstimateParametersG_delete(cv::Ptr<cv::aruco::EstimateParameters>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::aruco::EstimateParameters>::new(TraitClass) generated
	// ("cv::Ptr<cv::aruco::EstimateParameters>::new", vec![(pred!(const, ["val"], ["cv::aruco::EstimateParameters"]), _)]),
	cv::Ptr<cv::aruco::EstimateParameters>* cv_PtrLcv_aruco_EstimateParametersG_new_const_EstimateParameters(cv::aruco::EstimateParameters* val) {
			return new cv::Ptr<cv::aruco::EstimateParameters>(val);
	}

}

