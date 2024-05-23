extern "C" {
	// cv::Ptr<cv::saliency::StaticSaliency>::getInnerPtr() generated
	// ("cv::Ptr<cv::saliency::StaticSaliency>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::saliency::StaticSaliency* cv_PtrLcv_saliency_StaticSaliencyG_getInnerPtr_const(const cv::Ptr<cv::saliency::StaticSaliency>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::saliency::StaticSaliency>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::saliency::StaticSaliency>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::saliency::StaticSaliency* cv_PtrLcv_saliency_StaticSaliencyG_getInnerPtrMut(cv::Ptr<cv::saliency::StaticSaliency>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::saliency::StaticSaliency>::new_null() generated
	// ("cv::Ptr<cv::saliency::StaticSaliency>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::saliency::StaticSaliency>* cv_PtrLcv_saliency_StaticSaliencyG_new_null_const() {
			return new cv::Ptr<cv::saliency::StaticSaliency>();
	}

	// cv::Ptr<cv::saliency::StaticSaliency>::delete() generated
	// ("cv::Ptr<cv::saliency::StaticSaliency>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_saliency_StaticSaliencyG_delete(cv::Ptr<cv::saliency::StaticSaliency>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::saliency::StaticSaliency>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::saliency::StaticSaliency>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_saliency_StaticSaliencyG_to_PtrOfAlgorithm(cv::Ptr<cv::saliency::StaticSaliency>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::saliency::StaticSaliency>::to_PtrOfSaliency() generated
	// ("cv::Ptr<cv::saliency::StaticSaliency>::to_PtrOfSaliency", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::saliency::Saliency>* cv_PtrLcv_saliency_StaticSaliencyG_to_PtrOfSaliency(cv::Ptr<cv::saliency::StaticSaliency>* instance) {
			return new cv::Ptr<cv::saliency::Saliency>(instance->dynamicCast<cv::saliency::Saliency>());
	}

}

