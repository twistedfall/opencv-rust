extern "C" {
	// cv::Ptr<cv::saliency::Saliency>::getInnerPtr() generated
	// ("cv::Ptr<cv::saliency::Saliency>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::saliency::Saliency* cv_PtrLcv_saliency_SaliencyG_getInnerPtr_const(const cv::Ptr<cv::saliency::Saliency>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::saliency::Saliency>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::saliency::Saliency>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::saliency::Saliency* cv_PtrLcv_saliency_SaliencyG_getInnerPtrMut(cv::Ptr<cv::saliency::Saliency>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::saliency::Saliency>::new_null() generated
	// ("cv::Ptr<cv::saliency::Saliency>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::saliency::Saliency>* cv_PtrLcv_saliency_SaliencyG_new_null_const() {
			return new cv::Ptr<cv::saliency::Saliency>();
	}

	// cv::Ptr<cv::saliency::Saliency>::delete() generated
	// ("cv::Ptr<cv::saliency::Saliency>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_saliency_SaliencyG_delete(cv::Ptr<cv::saliency::Saliency>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::saliency::Saliency>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::saliency::Saliency>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_saliency_SaliencyG_to_PtrOfAlgorithm(cv::Ptr<cv::saliency::Saliency>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

