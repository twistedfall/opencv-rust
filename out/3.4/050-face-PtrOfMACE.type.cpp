extern "C" {
	// cv::Ptr<cv::face::MACE>::getInnerPtr() generated
	// ("cv::Ptr<cv::face::MACE>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::face::MACE* cv_PtrLcv_face_MACEG_getInnerPtr_const(const cv::Ptr<cv::face::MACE>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::face::MACE>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::face::MACE>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::face::MACE* cv_PtrLcv_face_MACEG_getInnerPtrMut(cv::Ptr<cv::face::MACE>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::face::MACE>::new_null() generated
	// ("cv::Ptr<cv::face::MACE>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::face::MACE>* cv_PtrLcv_face_MACEG_new_null_const() {
			return new cv::Ptr<cv::face::MACE>();
	}

	// cv::Ptr<cv::face::MACE>::delete() generated
	// ("cv::Ptr<cv::face::MACE>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_face_MACEG_delete(cv::Ptr<cv::face::MACE>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::face::MACE>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::face::MACE>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_face_MACEG_to_PtrOfAlgorithm(cv::Ptr<cv::face::MACE>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

