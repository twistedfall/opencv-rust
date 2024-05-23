extern "C" {
	// cv::Ptr<cv::text::ERFilter>::getInnerPtr() generated
	// ("cv::Ptr<cv::text::ERFilter>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::text::ERFilter* cv_PtrLcv_text_ERFilterG_getInnerPtr_const(const cv::Ptr<cv::text::ERFilter>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::text::ERFilter>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::text::ERFilter>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::text::ERFilter* cv_PtrLcv_text_ERFilterG_getInnerPtrMut(cv::Ptr<cv::text::ERFilter>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::text::ERFilter>::new_null() generated
	// ("cv::Ptr<cv::text::ERFilter>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::text::ERFilter>* cv_PtrLcv_text_ERFilterG_new_null_const() {
			return new cv::Ptr<cv::text::ERFilter>();
	}

	// cv::Ptr<cv::text::ERFilter>::delete() generated
	// ("cv::Ptr<cv::text::ERFilter>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_text_ERFilterG_delete(cv::Ptr<cv::text::ERFilter>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::text::ERFilter>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::text::ERFilter>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_text_ERFilterG_to_PtrOfAlgorithm(cv::Ptr<cv::text::ERFilter>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

