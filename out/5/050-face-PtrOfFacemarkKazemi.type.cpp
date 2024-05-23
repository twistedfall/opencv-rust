extern "C" {
	// cv::Ptr<cv::face::FacemarkKazemi>::getInnerPtr() generated
	// ("cv::Ptr<cv::face::FacemarkKazemi>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::face::FacemarkKazemi* cv_PtrLcv_face_FacemarkKazemiG_getInnerPtr_const(const cv::Ptr<cv::face::FacemarkKazemi>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::face::FacemarkKazemi>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::face::FacemarkKazemi>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::face::FacemarkKazemi* cv_PtrLcv_face_FacemarkKazemiG_getInnerPtrMut(cv::Ptr<cv::face::FacemarkKazemi>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::face::FacemarkKazemi>::new_null() generated
	// ("cv::Ptr<cv::face::FacemarkKazemi>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::face::FacemarkKazemi>* cv_PtrLcv_face_FacemarkKazemiG_new_null_const() {
			return new cv::Ptr<cv::face::FacemarkKazemi>();
	}

	// cv::Ptr<cv::face::FacemarkKazemi>::delete() generated
	// ("cv::Ptr<cv::face::FacemarkKazemi>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_face_FacemarkKazemiG_delete(cv::Ptr<cv::face::FacemarkKazemi>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::face::FacemarkKazemi>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::face::FacemarkKazemi>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_face_FacemarkKazemiG_to_PtrOfAlgorithm(cv::Ptr<cv::face::FacemarkKazemi>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::face::FacemarkKazemi>::to_PtrOfFacemark() generated
	// ("cv::Ptr<cv::face::FacemarkKazemi>::to_PtrOfFacemark", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::face::Facemark>* cv_PtrLcv_face_FacemarkKazemiG_to_PtrOfFacemark(cv::Ptr<cv::face::FacemarkKazemi>* instance) {
			return new cv::Ptr<cv::face::Facemark>(instance->dynamicCast<cv::face::Facemark>());
	}

}

