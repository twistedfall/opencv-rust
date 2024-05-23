extern "C" {
	// cv::Ptr<cv::face::EigenFaceRecognizer>::getInnerPtr() generated
	// ("cv::Ptr<cv::face::EigenFaceRecognizer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::face::EigenFaceRecognizer* cv_PtrLcv_face_EigenFaceRecognizerG_getInnerPtr_const(const cv::Ptr<cv::face::EigenFaceRecognizer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::face::EigenFaceRecognizer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::face::EigenFaceRecognizer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::face::EigenFaceRecognizer* cv_PtrLcv_face_EigenFaceRecognizerG_getInnerPtrMut(cv::Ptr<cv::face::EigenFaceRecognizer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::face::EigenFaceRecognizer>::new_null() generated
	// ("cv::Ptr<cv::face::EigenFaceRecognizer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::face::EigenFaceRecognizer>* cv_PtrLcv_face_EigenFaceRecognizerG_new_null_const() {
			return new cv::Ptr<cv::face::EigenFaceRecognizer>();
	}

	// cv::Ptr<cv::face::EigenFaceRecognizer>::delete() generated
	// ("cv::Ptr<cv::face::EigenFaceRecognizer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_face_EigenFaceRecognizerG_delete(cv::Ptr<cv::face::EigenFaceRecognizer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::face::EigenFaceRecognizer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::face::EigenFaceRecognizer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_face_EigenFaceRecognizerG_to_PtrOfAlgorithm(cv::Ptr<cv::face::EigenFaceRecognizer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::face::EigenFaceRecognizer>::to_PtrOfBasicFaceRecognizer() generated
	// ("cv::Ptr<cv::face::EigenFaceRecognizer>::to_PtrOfBasicFaceRecognizer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::face::BasicFaceRecognizer>* cv_PtrLcv_face_EigenFaceRecognizerG_to_PtrOfBasicFaceRecognizer(cv::Ptr<cv::face::EigenFaceRecognizer>* instance) {
			return new cv::Ptr<cv::face::BasicFaceRecognizer>(instance->dynamicCast<cv::face::BasicFaceRecognizer>());
	}

	// cv::Ptr<cv::face::EigenFaceRecognizer>::to_PtrOfFaceRecognizer() generated
	// ("cv::Ptr<cv::face::EigenFaceRecognizer>::to_PtrOfFaceRecognizer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::face::FaceRecognizer>* cv_PtrLcv_face_EigenFaceRecognizerG_to_PtrOfFaceRecognizer(cv::Ptr<cv::face::EigenFaceRecognizer>* instance) {
			return new cv::Ptr<cv::face::FaceRecognizer>(instance->dynamicCast<cv::face::FaceRecognizer>());
	}

}

