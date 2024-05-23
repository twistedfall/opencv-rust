extern "C" {
	// cv::Ptr<cv::face::StandardCollector>::getInnerPtr() generated
	// ("cv::Ptr<cv::face::StandardCollector>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::face::StandardCollector* cv_PtrLcv_face_StandardCollectorG_getInnerPtr_const(const cv::Ptr<cv::face::StandardCollector>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::face::StandardCollector>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::face::StandardCollector>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::face::StandardCollector* cv_PtrLcv_face_StandardCollectorG_getInnerPtrMut(cv::Ptr<cv::face::StandardCollector>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::face::StandardCollector>::new_null() generated
	// ("cv::Ptr<cv::face::StandardCollector>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::face::StandardCollector>* cv_PtrLcv_face_StandardCollectorG_new_null_const() {
			return new cv::Ptr<cv::face::StandardCollector>();
	}

	// cv::Ptr<cv::face::StandardCollector>::delete() generated
	// ("cv::Ptr<cv::face::StandardCollector>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_face_StandardCollectorG_delete(cv::Ptr<cv::face::StandardCollector>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::face::StandardCollector>::to_PtrOfPredictCollector() generated
	// ("cv::Ptr<cv::face::StandardCollector>::to_PtrOfPredictCollector", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::face::PredictCollector>* cv_PtrLcv_face_StandardCollectorG_to_PtrOfPredictCollector(cv::Ptr<cv::face::StandardCollector>* instance) {
			return new cv::Ptr<cv::face::PredictCollector>(instance->dynamicCast<cv::face::PredictCollector>());
	}

	// cv::Ptr<cv::face::StandardCollector>::new(TraitClass) generated
	// ("cv::Ptr<cv::face::StandardCollector>::new", vec![(pred!(const, ["val"], ["cv::face::StandardCollector"]), _)]),
	cv::Ptr<cv::face::StandardCollector>* cv_PtrLcv_face_StandardCollectorG_new_const_StandardCollector(cv::face::StandardCollector* val) {
			return new cv::Ptr<cv::face::StandardCollector>(val);
	}

}

