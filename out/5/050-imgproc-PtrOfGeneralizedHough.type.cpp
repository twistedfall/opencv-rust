extern "C" {
	// cv::Ptr<cv::GeneralizedHough>::getInnerPtr() generated
	// ("cv::Ptr<cv::GeneralizedHough>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::GeneralizedHough* cv_PtrLcv_GeneralizedHoughG_getInnerPtr_const(const cv::Ptr<cv::GeneralizedHough>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::GeneralizedHough>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::GeneralizedHough>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::GeneralizedHough* cv_PtrLcv_GeneralizedHoughG_getInnerPtrMut(cv::Ptr<cv::GeneralizedHough>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::GeneralizedHough>::new_null() generated
	// ("cv::Ptr<cv::GeneralizedHough>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::GeneralizedHough>* cv_PtrLcv_GeneralizedHoughG_new_null_const() {
			return new cv::Ptr<cv::GeneralizedHough>();
	}

	// cv::Ptr<cv::GeneralizedHough>::delete() generated
	// ("cv::Ptr<cv::GeneralizedHough>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_GeneralizedHoughG_delete(cv::Ptr<cv::GeneralizedHough>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::GeneralizedHough>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::GeneralizedHough>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_GeneralizedHoughG_to_PtrOfAlgorithm(cv::Ptr<cv::GeneralizedHough>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

