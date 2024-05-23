extern "C" {
	// cv::Ptr<cv::aruco::Board>::getInnerPtr() generated
	// ("cv::Ptr<cv::aruco::Board>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::aruco::Board* cv_PtrLcv_aruco_BoardG_getInnerPtr_const(const cv::Ptr<cv::aruco::Board>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::aruco::Board>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::aruco::Board>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::aruco::Board* cv_PtrLcv_aruco_BoardG_getInnerPtrMut(cv::Ptr<cv::aruco::Board>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::aruco::Board>::new_null() generated
	// ("cv::Ptr<cv::aruco::Board>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::aruco::Board>* cv_PtrLcv_aruco_BoardG_new_null_const() {
			return new cv::Ptr<cv::aruco::Board>();
	}

	// cv::Ptr<cv::aruco::Board>::delete() generated
	// ("cv::Ptr<cv::aruco::Board>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_aruco_BoardG_delete(cv::Ptr<cv::aruco::Board>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::aruco::Board>::new(TraitClass) generated
	// ("cv::Ptr<cv::aruco::Board>::new", vec![(pred!(const, ["val"], ["cv::aruco::Board"]), _)]),
	cv::Ptr<cv::aruco::Board>* cv_PtrLcv_aruco_BoardG_new_const_Board(cv::aruco::Board* val) {
			return new cv::Ptr<cv::aruco::Board>(val);
	}

}

