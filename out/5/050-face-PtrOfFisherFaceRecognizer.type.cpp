extern "C" {
	// cv::Ptr<cv::face::FisherFaceRecognizer>::getInnerPtr() generated
	// ("cv::Ptr<cv::face::FisherFaceRecognizer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::face::FisherFaceRecognizer* cv_PtrLcv_face_FisherFaceRecognizerG_getInnerPtr_const(const cv::Ptr<cv::face::FisherFaceRecognizer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::face::FisherFaceRecognizer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::face::FisherFaceRecognizer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::face::FisherFaceRecognizer* cv_PtrLcv_face_FisherFaceRecognizerG_getInnerPtrMut(cv::Ptr<cv::face::FisherFaceRecognizer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::face::FisherFaceRecognizer>::new_null() generated
	// ("cv::Ptr<cv::face::FisherFaceRecognizer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::face::FisherFaceRecognizer>* cv_PtrLcv_face_FisherFaceRecognizerG_new_null_const() {
			return new cv::Ptr<cv::face::FisherFaceRecognizer>();
	}

	// cv::Ptr<cv::face::FisherFaceRecognizer>::delete() generated
	// ("cv::Ptr<cv::face::FisherFaceRecognizer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_face_FisherFaceRecognizerG_delete(cv::Ptr<cv::face::FisherFaceRecognizer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::face::FisherFaceRecognizer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::face::FisherFaceRecognizer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_face_FisherFaceRecognizerG_to_PtrOfAlgorithm(cv::Ptr<cv::face::FisherFaceRecognizer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::face::FisherFaceRecognizer>::to_PtrOfBasicFaceRecognizer() generated
	// ("cv::Ptr<cv::face::FisherFaceRecognizer>::to_PtrOfBasicFaceRecognizer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::face::BasicFaceRecognizer>* cv_PtrLcv_face_FisherFaceRecognizerG_to_PtrOfBasicFaceRecognizer(cv::Ptr<cv::face::FisherFaceRecognizer>* instance) {
			return new cv::Ptr<cv::face::BasicFaceRecognizer>(instance->dynamicCast<cv::face::BasicFaceRecognizer>());
	}

	// cv::Ptr<cv::face::FisherFaceRecognizer>::to_PtrOfFaceRecognizer() generated
	// ("cv::Ptr<cv::face::FisherFaceRecognizer>::to_PtrOfFaceRecognizer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::face::FaceRecognizer>* cv_PtrLcv_face_FisherFaceRecognizerG_to_PtrOfFaceRecognizer(cv::Ptr<cv::face::FisherFaceRecognizer>* instance) {
			return new cv::Ptr<cv::face::FaceRecognizer>(instance->dynamicCast<cv::face::FaceRecognizer>());
	}

}

