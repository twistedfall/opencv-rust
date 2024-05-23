extern "C" {
	// cv::Ptr<cv::aruco::GridBoard>::getInnerPtr() generated
	// ("cv::Ptr<cv::aruco::GridBoard>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::aruco::GridBoard* cv_PtrLcv_aruco_GridBoardG_getInnerPtr_const(const cv::Ptr<cv::aruco::GridBoard>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::aruco::GridBoard>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::aruco::GridBoard>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::aruco::GridBoard* cv_PtrLcv_aruco_GridBoardG_getInnerPtrMut(cv::Ptr<cv::aruco::GridBoard>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::aruco::GridBoard>::new_null() generated
	// ("cv::Ptr<cv::aruco::GridBoard>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::aruco::GridBoard>* cv_PtrLcv_aruco_GridBoardG_new_null_const() {
			return new cv::Ptr<cv::aruco::GridBoard>();
	}

	// cv::Ptr<cv::aruco::GridBoard>::delete() generated
	// ("cv::Ptr<cv::aruco::GridBoard>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_aruco_GridBoardG_delete(cv::Ptr<cv::aruco::GridBoard>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::aruco::GridBoard>::to_PtrOfBoard() generated
	// ("cv::Ptr<cv::aruco::GridBoard>::to_PtrOfBoard", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::aruco::Board>* cv_PtrLcv_aruco_GridBoardG_to_PtrOfBoard(cv::Ptr<cv::aruco::GridBoard>* instance) {
			return new cv::Ptr<cv::aruco::Board>(instance->dynamicCast<cv::aruco::Board>());
	}

	// cv::Ptr<cv::aruco::GridBoard>::new(TraitClass) generated
	// ("cv::Ptr<cv::aruco::GridBoard>::new", vec![(pred!(const, ["val"], ["cv::aruco::GridBoard"]), _)]),
	cv::Ptr<cv::aruco::GridBoard>* cv_PtrLcv_aruco_GridBoardG_new_const_GridBoard(cv::aruco::GridBoard* val) {
			return new cv::Ptr<cv::aruco::GridBoard>(val);
	}

}

