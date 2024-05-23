extern "C" {
	// cv::Ptr<cv::face::BIF>::getInnerPtr() generated
	// ("cv::Ptr<cv::face::BIF>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::face::BIF* cv_PtrLcv_face_BIFG_getInnerPtr_const(const cv::Ptr<cv::face::BIF>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::face::BIF>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::face::BIF>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::face::BIF* cv_PtrLcv_face_BIFG_getInnerPtrMut(cv::Ptr<cv::face::BIF>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::face::BIF>::new_null() generated
	// ("cv::Ptr<cv::face::BIF>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::face::BIF>* cv_PtrLcv_face_BIFG_new_null_const() {
			return new cv::Ptr<cv::face::BIF>();
	}

	// cv::Ptr<cv::face::BIF>::delete() generated
	// ("cv::Ptr<cv::face::BIF>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_face_BIFG_delete(cv::Ptr<cv::face::BIF>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::face::BIF>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::face::BIF>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_face_BIFG_to_PtrOfAlgorithm(cv::Ptr<cv::face::BIF>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

