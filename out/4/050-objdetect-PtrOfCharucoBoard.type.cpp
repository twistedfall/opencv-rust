extern "C" {
	// cv::Ptr<cv::aruco::CharucoBoard>::getInnerPtr() generated
	// ("cv::Ptr<cv::aruco::CharucoBoard>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::aruco::CharucoBoard* cv_PtrLcv_aruco_CharucoBoardG_getInnerPtr_const(const cv::Ptr<cv::aruco::CharucoBoard>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::aruco::CharucoBoard>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::aruco::CharucoBoard>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::aruco::CharucoBoard* cv_PtrLcv_aruco_CharucoBoardG_getInnerPtrMut(cv::Ptr<cv::aruco::CharucoBoard>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::aruco::CharucoBoard>::new_null() generated
	// ("cv::Ptr<cv::aruco::CharucoBoard>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::aruco::CharucoBoard>* cv_PtrLcv_aruco_CharucoBoardG_new_null_const() {
			return new cv::Ptr<cv::aruco::CharucoBoard>();
	}

	// cv::Ptr<cv::aruco::CharucoBoard>::delete() generated
	// ("cv::Ptr<cv::aruco::CharucoBoard>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_aruco_CharucoBoardG_delete(cv::Ptr<cv::aruco::CharucoBoard>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::aruco::CharucoBoard>::to_PtrOfBoard() generated
	// ("cv::Ptr<cv::aruco::CharucoBoard>::to_PtrOfBoard", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::aruco::Board>* cv_PtrLcv_aruco_CharucoBoardG_to_PtrOfBoard(cv::Ptr<cv::aruco::CharucoBoard>* instance) {
			return new cv::Ptr<cv::aruco::Board>(instance->dynamicCast<cv::aruco::Board>());
	}

	// cv::Ptr<cv::aruco::CharucoBoard>::new(TraitClass) generated
	// ("cv::Ptr<cv::aruco::CharucoBoard>::new", vec![(pred!(const, ["val"], ["cv::aruco::CharucoBoard"]), _)]),
	cv::Ptr<cv::aruco::CharucoBoard>* cv_PtrLcv_aruco_CharucoBoardG_new_const_CharucoBoard(cv::aruco::CharucoBoard* val) {
			return new cv::Ptr<cv::aruco::CharucoBoard>(val);
	}

}

