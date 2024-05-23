extern "C" {
	// cv::Ptr<cv::saliency::Objectness>::getInnerPtr() generated
	// ("cv::Ptr<cv::saliency::Objectness>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::saliency::Objectness* cv_PtrLcv_saliency_ObjectnessG_getInnerPtr_const(const cv::Ptr<cv::saliency::Objectness>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::saliency::Objectness>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::saliency::Objectness>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::saliency::Objectness* cv_PtrLcv_saliency_ObjectnessG_getInnerPtrMut(cv::Ptr<cv::saliency::Objectness>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::saliency::Objectness>::new_null() generated
	// ("cv::Ptr<cv::saliency::Objectness>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::saliency::Objectness>* cv_PtrLcv_saliency_ObjectnessG_new_null_const() {
			return new cv::Ptr<cv::saliency::Objectness>();
	}

	// cv::Ptr<cv::saliency::Objectness>::delete() generated
	// ("cv::Ptr<cv::saliency::Objectness>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_saliency_ObjectnessG_delete(cv::Ptr<cv::saliency::Objectness>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::saliency::Objectness>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::saliency::Objectness>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_saliency_ObjectnessG_to_PtrOfAlgorithm(cv::Ptr<cv::saliency::Objectness>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::saliency::Objectness>::to_PtrOfSaliency() generated
	// ("cv::Ptr<cv::saliency::Objectness>::to_PtrOfSaliency", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::saliency::Saliency>* cv_PtrLcv_saliency_ObjectnessG_to_PtrOfSaliency(cv::Ptr<cv::saliency::Objectness>* instance) {
			return new cv::Ptr<cv::saliency::Saliency>(instance->dynamicCast<cv::saliency::Saliency>());
	}

}

