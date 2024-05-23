extern "C" {
	// cv::Ptr<cv::face::Facemark>::getInnerPtr() generated
	// ("cv::Ptr<cv::face::Facemark>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::face::Facemark* cv_PtrLcv_face_FacemarkG_getInnerPtr_const(const cv::Ptr<cv::face::Facemark>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::face::Facemark>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::face::Facemark>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::face::Facemark* cv_PtrLcv_face_FacemarkG_getInnerPtrMut(cv::Ptr<cv::face::Facemark>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::face::Facemark>::new_null() generated
	// ("cv::Ptr<cv::face::Facemark>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::face::Facemark>* cv_PtrLcv_face_FacemarkG_new_null_const() {
			return new cv::Ptr<cv::face::Facemark>();
	}

	// cv::Ptr<cv::face::Facemark>::delete() generated
	// ("cv::Ptr<cv::face::Facemark>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_face_FacemarkG_delete(cv::Ptr<cv::face::Facemark>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::face::Facemark>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::face::Facemark>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_face_FacemarkG_to_PtrOfAlgorithm(cv::Ptr<cv::face::Facemark>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

