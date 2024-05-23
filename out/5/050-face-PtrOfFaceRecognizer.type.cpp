extern "C" {
	// cv::Ptr<cv::face::FaceRecognizer>::getInnerPtr() generated
	// ("cv::Ptr<cv::face::FaceRecognizer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::face::FaceRecognizer* cv_PtrLcv_face_FaceRecognizerG_getInnerPtr_const(const cv::Ptr<cv::face::FaceRecognizer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::face::FaceRecognizer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::face::FaceRecognizer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::face::FaceRecognizer* cv_PtrLcv_face_FaceRecognizerG_getInnerPtrMut(cv::Ptr<cv::face::FaceRecognizer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::face::FaceRecognizer>::new_null() generated
	// ("cv::Ptr<cv::face::FaceRecognizer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::face::FaceRecognizer>* cv_PtrLcv_face_FaceRecognizerG_new_null_const() {
			return new cv::Ptr<cv::face::FaceRecognizer>();
	}

	// cv::Ptr<cv::face::FaceRecognizer>::delete() generated
	// ("cv::Ptr<cv::face::FaceRecognizer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_face_FaceRecognizerG_delete(cv::Ptr<cv::face::FaceRecognizer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::face::FaceRecognizer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::face::FaceRecognizer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_face_FaceRecognizerG_to_PtrOfAlgorithm(cv::Ptr<cv::face::FaceRecognizer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

