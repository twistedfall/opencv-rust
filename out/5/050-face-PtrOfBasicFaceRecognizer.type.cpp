extern "C" {
	// cv::Ptr<cv::face::BasicFaceRecognizer>::getInnerPtr() generated
	// ("cv::Ptr<cv::face::BasicFaceRecognizer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::face::BasicFaceRecognizer* cv_PtrLcv_face_BasicFaceRecognizerG_getInnerPtr_const(const cv::Ptr<cv::face::BasicFaceRecognizer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::face::BasicFaceRecognizer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::face::BasicFaceRecognizer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::face::BasicFaceRecognizer* cv_PtrLcv_face_BasicFaceRecognizerG_getInnerPtrMut(cv::Ptr<cv::face::BasicFaceRecognizer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::face::BasicFaceRecognizer>::new_null() generated
	// ("cv::Ptr<cv::face::BasicFaceRecognizer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::face::BasicFaceRecognizer>* cv_PtrLcv_face_BasicFaceRecognizerG_new_null_const() {
			return new cv::Ptr<cv::face::BasicFaceRecognizer>();
	}

	// cv::Ptr<cv::face::BasicFaceRecognizer>::delete() generated
	// ("cv::Ptr<cv::face::BasicFaceRecognizer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_face_BasicFaceRecognizerG_delete(cv::Ptr<cv::face::BasicFaceRecognizer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::face::BasicFaceRecognizer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::face::BasicFaceRecognizer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_face_BasicFaceRecognizerG_to_PtrOfAlgorithm(cv::Ptr<cv::face::BasicFaceRecognizer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::face::BasicFaceRecognizer>::to_PtrOfFaceRecognizer() generated
	// ("cv::Ptr<cv::face::BasicFaceRecognizer>::to_PtrOfFaceRecognizer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::face::FaceRecognizer>* cv_PtrLcv_face_BasicFaceRecognizerG_to_PtrOfFaceRecognizer(cv::Ptr<cv::face::BasicFaceRecognizer>* instance) {
			return new cv::Ptr<cv::face::FaceRecognizer>(instance->dynamicCast<cv::face::FaceRecognizer>());
	}

}

