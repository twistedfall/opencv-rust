extern "C" {
	// cv::Ptr<cv::saliency::MotionSaliency>::getInnerPtr() generated
	// ("cv::Ptr<cv::saliency::MotionSaliency>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::saliency::MotionSaliency* cv_PtrLcv_saliency_MotionSaliencyG_getInnerPtr_const(const cv::Ptr<cv::saliency::MotionSaliency>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::saliency::MotionSaliency>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::saliency::MotionSaliency>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::saliency::MotionSaliency* cv_PtrLcv_saliency_MotionSaliencyG_getInnerPtrMut(cv::Ptr<cv::saliency::MotionSaliency>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::saliency::MotionSaliency>::new_null() generated
	// ("cv::Ptr<cv::saliency::MotionSaliency>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::saliency::MotionSaliency>* cv_PtrLcv_saliency_MotionSaliencyG_new_null_const() {
			return new cv::Ptr<cv::saliency::MotionSaliency>();
	}

	// cv::Ptr<cv::saliency::MotionSaliency>::delete() generated
	// ("cv::Ptr<cv::saliency::MotionSaliency>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_saliency_MotionSaliencyG_delete(cv::Ptr<cv::saliency::MotionSaliency>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::saliency::MotionSaliency>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::saliency::MotionSaliency>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_saliency_MotionSaliencyG_to_PtrOfAlgorithm(cv::Ptr<cv::saliency::MotionSaliency>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::saliency::MotionSaliency>::to_PtrOfSaliency() generated
	// ("cv::Ptr<cv::saliency::MotionSaliency>::to_PtrOfSaliency", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::saliency::Saliency>* cv_PtrLcv_saliency_MotionSaliencyG_to_PtrOfSaliency(cv::Ptr<cv::saliency::MotionSaliency>* instance) {
			return new cv::Ptr<cv::saliency::Saliency>(instance->dynamicCast<cv::saliency::Saliency>());
	}

}

