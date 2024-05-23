extern "C" {
	// cv::Ptr<cv::FaceDetectorYN>::getInnerPtr() generated
	// ("cv::Ptr<cv::FaceDetectorYN>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::FaceDetectorYN* cv_PtrLcv_FaceDetectorYNG_getInnerPtr_const(const cv::Ptr<cv::FaceDetectorYN>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::FaceDetectorYN>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::FaceDetectorYN>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::FaceDetectorYN* cv_PtrLcv_FaceDetectorYNG_getInnerPtrMut(cv::Ptr<cv::FaceDetectorYN>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::FaceDetectorYN>::new_null() generated
	// ("cv::Ptr<cv::FaceDetectorYN>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::FaceDetectorYN>* cv_PtrLcv_FaceDetectorYNG_new_null_const() {
			return new cv::Ptr<cv::FaceDetectorYN>();
	}

	// cv::Ptr<cv::FaceDetectorYN>::delete() generated
	// ("cv::Ptr<cv::FaceDetectorYN>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_FaceDetectorYNG_delete(cv::Ptr<cv::FaceDetectorYN>* instance) {
			delete instance;
	}

}

