extern "C" {
	// cv::Ptr<cv::face::FacemarkAAM>::getInnerPtr() generated
	// ("cv::Ptr<cv::face::FacemarkAAM>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::face::FacemarkAAM* cv_PtrLcv_face_FacemarkAAMG_getInnerPtr_const(const cv::Ptr<cv::face::FacemarkAAM>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::face::FacemarkAAM>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::face::FacemarkAAM>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::face::FacemarkAAM* cv_PtrLcv_face_FacemarkAAMG_getInnerPtrMut(cv::Ptr<cv::face::FacemarkAAM>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::face::FacemarkAAM>::new_null() generated
	// ("cv::Ptr<cv::face::FacemarkAAM>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::face::FacemarkAAM>* cv_PtrLcv_face_FacemarkAAMG_new_null_const() {
			return new cv::Ptr<cv::face::FacemarkAAM>();
	}

	// cv::Ptr<cv::face::FacemarkAAM>::delete() generated
	// ("cv::Ptr<cv::face::FacemarkAAM>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_face_FacemarkAAMG_delete(cv::Ptr<cv::face::FacemarkAAM>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::face::FacemarkAAM>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::face::FacemarkAAM>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_face_FacemarkAAMG_to_PtrOfAlgorithm(cv::Ptr<cv::face::FacemarkAAM>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::face::FacemarkAAM>::to_PtrOfFacemark() generated
	// ("cv::Ptr<cv::face::FacemarkAAM>::to_PtrOfFacemark", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::face::Facemark>* cv_PtrLcv_face_FacemarkAAMG_to_PtrOfFacemark(cv::Ptr<cv::face::FacemarkAAM>* instance) {
			return new cv::Ptr<cv::face::Facemark>(instance->dynamicCast<cv::face::Facemark>());
	}

	// cv::Ptr<cv::face::FacemarkAAM>::to_PtrOfFacemarkTrain() generated
	// ("cv::Ptr<cv::face::FacemarkAAM>::to_PtrOfFacemarkTrain", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::face::FacemarkTrain>* cv_PtrLcv_face_FacemarkAAMG_to_PtrOfFacemarkTrain(cv::Ptr<cv::face::FacemarkAAM>* instance) {
			return new cv::Ptr<cv::face::FacemarkTrain>(instance->dynamicCast<cv::face::FacemarkTrain>());
	}

}

