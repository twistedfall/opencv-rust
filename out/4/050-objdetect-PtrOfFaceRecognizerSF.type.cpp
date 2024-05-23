extern "C" {
	// cv::Ptr<cv::FaceRecognizerSF>::getInnerPtr() generated
	// ("cv::Ptr<cv::FaceRecognizerSF>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::FaceRecognizerSF* cv_PtrLcv_FaceRecognizerSFG_getInnerPtr_const(const cv::Ptr<cv::FaceRecognizerSF>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::FaceRecognizerSF>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::FaceRecognizerSF>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::FaceRecognizerSF* cv_PtrLcv_FaceRecognizerSFG_getInnerPtrMut(cv::Ptr<cv::FaceRecognizerSF>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::FaceRecognizerSF>::new_null() generated
	// ("cv::Ptr<cv::FaceRecognizerSF>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::FaceRecognizerSF>* cv_PtrLcv_FaceRecognizerSFG_new_null_const() {
			return new cv::Ptr<cv::FaceRecognizerSF>();
	}

	// cv::Ptr<cv::FaceRecognizerSF>::delete() generated
	// ("cv::Ptr<cv::FaceRecognizerSF>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_FaceRecognizerSFG_delete(cv::Ptr<cv::FaceRecognizerSF>* instance) {
			delete instance;
	}

}

