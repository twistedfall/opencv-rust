extern "C" {
	// cv::Ptr<cv::ximgproc::EdgeDrawing>::getInnerPtr() generated
	// ("cv::Ptr<cv::ximgproc::EdgeDrawing>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::ximgproc::EdgeDrawing* cv_PtrLcv_ximgproc_EdgeDrawingG_getInnerPtr_const(const cv::Ptr<cv::ximgproc::EdgeDrawing>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ximgproc::EdgeDrawing>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::ximgproc::EdgeDrawing>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::ximgproc::EdgeDrawing* cv_PtrLcv_ximgproc_EdgeDrawingG_getInnerPtrMut(cv::Ptr<cv::ximgproc::EdgeDrawing>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ximgproc::EdgeDrawing>::new_null() generated
	// ("cv::Ptr<cv::ximgproc::EdgeDrawing>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::ximgproc::EdgeDrawing>* cv_PtrLcv_ximgproc_EdgeDrawingG_new_null_const() {
			return new cv::Ptr<cv::ximgproc::EdgeDrawing>();
	}

	// cv::Ptr<cv::ximgproc::EdgeDrawing>::delete() generated
	// ("cv::Ptr<cv::ximgproc::EdgeDrawing>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_ximgproc_EdgeDrawingG_delete(cv::Ptr<cv::ximgproc::EdgeDrawing>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::ximgproc::EdgeDrawing>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::ximgproc::EdgeDrawing>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ximgproc_EdgeDrawingG_to_PtrOfAlgorithm(cv::Ptr<cv::ximgproc::EdgeDrawing>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

