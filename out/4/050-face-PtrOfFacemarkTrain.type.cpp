extern "C" {
	// cv::Ptr<cv::face::FacemarkTrain>::getInnerPtr() generated
	// ("cv::Ptr<cv::face::FacemarkTrain>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::face::FacemarkTrain* cv_PtrLcv_face_FacemarkTrainG_getInnerPtr_const(const cv::Ptr<cv::face::FacemarkTrain>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::face::FacemarkTrain>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::face::FacemarkTrain>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::face::FacemarkTrain* cv_PtrLcv_face_FacemarkTrainG_getInnerPtrMut(cv::Ptr<cv::face::FacemarkTrain>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::face::FacemarkTrain>::new_null() generated
	// ("cv::Ptr<cv::face::FacemarkTrain>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::face::FacemarkTrain>* cv_PtrLcv_face_FacemarkTrainG_new_null_const() {
			return new cv::Ptr<cv::face::FacemarkTrain>();
	}

	// cv::Ptr<cv::face::FacemarkTrain>::delete() generated
	// ("cv::Ptr<cv::face::FacemarkTrain>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_face_FacemarkTrainG_delete(cv::Ptr<cv::face::FacemarkTrain>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::face::FacemarkTrain>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::face::FacemarkTrain>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_face_FacemarkTrainG_to_PtrOfAlgorithm(cv::Ptr<cv::face::FacemarkTrain>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::face::FacemarkTrain>::to_PtrOfFacemark() generated
	// ("cv::Ptr<cv::face::FacemarkTrain>::to_PtrOfFacemark", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::face::Facemark>* cv_PtrLcv_face_FacemarkTrainG_to_PtrOfFacemark(cv::Ptr<cv::face::FacemarkTrain>* instance) {
			return new cv::Ptr<cv::face::Facemark>(instance->dynamicCast<cv::face::Facemark>());
	}

}

