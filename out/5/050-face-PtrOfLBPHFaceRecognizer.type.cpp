extern "C" {
	// cv::Ptr<cv::face::LBPHFaceRecognizer>::getInnerPtr() generated
	// ("cv::Ptr<cv::face::LBPHFaceRecognizer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::face::LBPHFaceRecognizer* cv_PtrLcv_face_LBPHFaceRecognizerG_getInnerPtr_const(const cv::Ptr<cv::face::LBPHFaceRecognizer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::face::LBPHFaceRecognizer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::face::LBPHFaceRecognizer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::face::LBPHFaceRecognizer* cv_PtrLcv_face_LBPHFaceRecognizerG_getInnerPtrMut(cv::Ptr<cv::face::LBPHFaceRecognizer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::face::LBPHFaceRecognizer>::new_null() generated
	// ("cv::Ptr<cv::face::LBPHFaceRecognizer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::face::LBPHFaceRecognizer>* cv_PtrLcv_face_LBPHFaceRecognizerG_new_null_const() {
			return new cv::Ptr<cv::face::LBPHFaceRecognizer>();
	}

	// cv::Ptr<cv::face::LBPHFaceRecognizer>::delete() generated
	// ("cv::Ptr<cv::face::LBPHFaceRecognizer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_face_LBPHFaceRecognizerG_delete(cv::Ptr<cv::face::LBPHFaceRecognizer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::face::LBPHFaceRecognizer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::face::LBPHFaceRecognizer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_face_LBPHFaceRecognizerG_to_PtrOfAlgorithm(cv::Ptr<cv::face::LBPHFaceRecognizer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::face::LBPHFaceRecognizer>::to_PtrOfFaceRecognizer() generated
	// ("cv::Ptr<cv::face::LBPHFaceRecognizer>::to_PtrOfFaceRecognizer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::face::FaceRecognizer>* cv_PtrLcv_face_LBPHFaceRecognizerG_to_PtrOfFaceRecognizer(cv::Ptr<cv::face::LBPHFaceRecognizer>* instance) {
			return new cv::Ptr<cv::face::FaceRecognizer>(instance->dynamicCast<cv::face::FaceRecognizer>());
	}

}

