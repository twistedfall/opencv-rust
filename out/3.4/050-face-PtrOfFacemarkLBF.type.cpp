extern "C" {
	// cv::Ptr<cv::face::FacemarkLBF>::getInnerPtr() generated
	// ("cv::Ptr<cv::face::FacemarkLBF>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::face::FacemarkLBF* cv_PtrLcv_face_FacemarkLBFG_getInnerPtr_const(const cv::Ptr<cv::face::FacemarkLBF>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::face::FacemarkLBF>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::face::FacemarkLBF>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::face::FacemarkLBF* cv_PtrLcv_face_FacemarkLBFG_getInnerPtrMut(cv::Ptr<cv::face::FacemarkLBF>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::face::FacemarkLBF>::new_null() generated
	// ("cv::Ptr<cv::face::FacemarkLBF>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::face::FacemarkLBF>* cv_PtrLcv_face_FacemarkLBFG_new_null_const() {
			return new cv::Ptr<cv::face::FacemarkLBF>();
	}

	// cv::Ptr<cv::face::FacemarkLBF>::delete() generated
	// ("cv::Ptr<cv::face::FacemarkLBF>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_face_FacemarkLBFG_delete(cv::Ptr<cv::face::FacemarkLBF>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::face::FacemarkLBF>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::face::FacemarkLBF>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_face_FacemarkLBFG_to_PtrOfAlgorithm(cv::Ptr<cv::face::FacemarkLBF>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::face::FacemarkLBF>::to_PtrOfFacemark() generated
	// ("cv::Ptr<cv::face::FacemarkLBF>::to_PtrOfFacemark", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::face::Facemark>* cv_PtrLcv_face_FacemarkLBFG_to_PtrOfFacemark(cv::Ptr<cv::face::FacemarkLBF>* instance) {
			return new cv::Ptr<cv::face::Facemark>(instance->dynamicCast<cv::face::Facemark>());
	}

	// cv::Ptr<cv::face::FacemarkLBF>::to_PtrOfFacemarkTrain() generated
	// ("cv::Ptr<cv::face::FacemarkLBF>::to_PtrOfFacemarkTrain", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::face::FacemarkTrain>* cv_PtrLcv_face_FacemarkLBFG_to_PtrOfFacemarkTrain(cv::Ptr<cv::face::FacemarkLBF>* instance) {
			return new cv::Ptr<cv::face::FacemarkTrain>(instance->dynamicCast<cv::face::FacemarkTrain>());
	}

}

