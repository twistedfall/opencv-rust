extern "C" {
	// cv::Ptr<cv::saliency::StaticSaliencyFineGrained>::getInnerPtr() generated
	// ("cv::Ptr<cv::saliency::StaticSaliencyFineGrained>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::saliency::StaticSaliencyFineGrained* cv_PtrLcv_saliency_StaticSaliencyFineGrainedG_getInnerPtr_const(const cv::Ptr<cv::saliency::StaticSaliencyFineGrained>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::saliency::StaticSaliencyFineGrained>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::saliency::StaticSaliencyFineGrained>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::saliency::StaticSaliencyFineGrained* cv_PtrLcv_saliency_StaticSaliencyFineGrainedG_getInnerPtrMut(cv::Ptr<cv::saliency::StaticSaliencyFineGrained>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::saliency::StaticSaliencyFineGrained>::new_null() generated
	// ("cv::Ptr<cv::saliency::StaticSaliencyFineGrained>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::saliency::StaticSaliencyFineGrained>* cv_PtrLcv_saliency_StaticSaliencyFineGrainedG_new_null_const() {
			return new cv::Ptr<cv::saliency::StaticSaliencyFineGrained>();
	}

	// cv::Ptr<cv::saliency::StaticSaliencyFineGrained>::delete() generated
	// ("cv::Ptr<cv::saliency::StaticSaliencyFineGrained>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_saliency_StaticSaliencyFineGrainedG_delete(cv::Ptr<cv::saliency::StaticSaliencyFineGrained>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::saliency::StaticSaliencyFineGrained>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::saliency::StaticSaliencyFineGrained>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_saliency_StaticSaliencyFineGrainedG_to_PtrOfAlgorithm(cv::Ptr<cv::saliency::StaticSaliencyFineGrained>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::saliency::StaticSaliencyFineGrained>::to_PtrOfSaliency() generated
	// ("cv::Ptr<cv::saliency::StaticSaliencyFineGrained>::to_PtrOfSaliency", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::saliency::Saliency>* cv_PtrLcv_saliency_StaticSaliencyFineGrainedG_to_PtrOfSaliency(cv::Ptr<cv::saliency::StaticSaliencyFineGrained>* instance) {
			return new cv::Ptr<cv::saliency::Saliency>(instance->dynamicCast<cv::saliency::Saliency>());
	}

	// cv::Ptr<cv::saliency::StaticSaliencyFineGrained>::to_PtrOfStaticSaliency() generated
	// ("cv::Ptr<cv::saliency::StaticSaliencyFineGrained>::to_PtrOfStaticSaliency", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::saliency::StaticSaliency>* cv_PtrLcv_saliency_StaticSaliencyFineGrainedG_to_PtrOfStaticSaliency(cv::Ptr<cv::saliency::StaticSaliencyFineGrained>* instance) {
			return new cv::Ptr<cv::saliency::StaticSaliency>(instance->dynamicCast<cv::saliency::StaticSaliency>());
	}

	// cv::Ptr<cv::saliency::StaticSaliencyFineGrained>::new(TraitClass) generated
	// ("cv::Ptr<cv::saliency::StaticSaliencyFineGrained>::new", vec![(pred!(const, ["val"], ["cv::saliency::StaticSaliencyFineGrained"]), _)]),
	cv::Ptr<cv::saliency::StaticSaliencyFineGrained>* cv_PtrLcv_saliency_StaticSaliencyFineGrainedG_new_const_StaticSaliencyFineGrained(cv::saliency::StaticSaliencyFineGrained* val) {
			return new cv::Ptr<cv::saliency::StaticSaliencyFineGrained>(val);
	}

}

