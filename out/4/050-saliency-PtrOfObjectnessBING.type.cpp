extern "C" {
	// cv::Ptr<cv::saliency::ObjectnessBING>::getInnerPtr() generated
	// ("cv::Ptr<cv::saliency::ObjectnessBING>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::saliency::ObjectnessBING* cv_PtrLcv_saliency_ObjectnessBINGG_getInnerPtr_const(const cv::Ptr<cv::saliency::ObjectnessBING>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::saliency::ObjectnessBING>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::saliency::ObjectnessBING>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::saliency::ObjectnessBING* cv_PtrLcv_saliency_ObjectnessBINGG_getInnerPtrMut(cv::Ptr<cv::saliency::ObjectnessBING>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::saliency::ObjectnessBING>::new_null() generated
	// ("cv::Ptr<cv::saliency::ObjectnessBING>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::saliency::ObjectnessBING>* cv_PtrLcv_saliency_ObjectnessBINGG_new_null_const() {
			return new cv::Ptr<cv::saliency::ObjectnessBING>();
	}

	// cv::Ptr<cv::saliency::ObjectnessBING>::delete() generated
	// ("cv::Ptr<cv::saliency::ObjectnessBING>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_saliency_ObjectnessBINGG_delete(cv::Ptr<cv::saliency::ObjectnessBING>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::saliency::ObjectnessBING>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::saliency::ObjectnessBING>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_saliency_ObjectnessBINGG_to_PtrOfAlgorithm(cv::Ptr<cv::saliency::ObjectnessBING>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::saliency::ObjectnessBING>::to_PtrOfObjectness() generated
	// ("cv::Ptr<cv::saliency::ObjectnessBING>::to_PtrOfObjectness", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::saliency::Objectness>* cv_PtrLcv_saliency_ObjectnessBINGG_to_PtrOfObjectness(cv::Ptr<cv::saliency::ObjectnessBING>* instance) {
			return new cv::Ptr<cv::saliency::Objectness>(instance->dynamicCast<cv::saliency::Objectness>());
	}

	// cv::Ptr<cv::saliency::ObjectnessBING>::to_PtrOfSaliency() generated
	// ("cv::Ptr<cv::saliency::ObjectnessBING>::to_PtrOfSaliency", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::saliency::Saliency>* cv_PtrLcv_saliency_ObjectnessBINGG_to_PtrOfSaliency(cv::Ptr<cv::saliency::ObjectnessBING>* instance) {
			return new cv::Ptr<cv::saliency::Saliency>(instance->dynamicCast<cv::saliency::Saliency>());
	}

	// cv::Ptr<cv::saliency::ObjectnessBING>::new(TraitClass) generated
	// ("cv::Ptr<cv::saliency::ObjectnessBING>::new", vec![(pred!(const, ["val"], ["cv::saliency::ObjectnessBING"]), _)]),
	cv::Ptr<cv::saliency::ObjectnessBING>* cv_PtrLcv_saliency_ObjectnessBINGG_new_const_ObjectnessBING(cv::saliency::ObjectnessBING* val) {
			return new cv::Ptr<cv::saliency::ObjectnessBING>(val);
	}

}

