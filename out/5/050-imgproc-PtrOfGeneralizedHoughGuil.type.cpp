extern "C" {
	// cv::Ptr<cv::GeneralizedHoughGuil>::getInnerPtr() generated
	// ("cv::Ptr<cv::GeneralizedHoughGuil>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::GeneralizedHoughGuil* cv_PtrLcv_GeneralizedHoughGuilG_getInnerPtr_const(const cv::Ptr<cv::GeneralizedHoughGuil>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::GeneralizedHoughGuil>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::GeneralizedHoughGuil>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::GeneralizedHoughGuil* cv_PtrLcv_GeneralizedHoughGuilG_getInnerPtrMut(cv::Ptr<cv::GeneralizedHoughGuil>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::GeneralizedHoughGuil>::new_null() generated
	// ("cv::Ptr<cv::GeneralizedHoughGuil>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::GeneralizedHoughGuil>* cv_PtrLcv_GeneralizedHoughGuilG_new_null_const() {
			return new cv::Ptr<cv::GeneralizedHoughGuil>();
	}

	// cv::Ptr<cv::GeneralizedHoughGuil>::delete() generated
	// ("cv::Ptr<cv::GeneralizedHoughGuil>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_GeneralizedHoughGuilG_delete(cv::Ptr<cv::GeneralizedHoughGuil>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::GeneralizedHoughGuil>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::GeneralizedHoughGuil>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_GeneralizedHoughGuilG_to_PtrOfAlgorithm(cv::Ptr<cv::GeneralizedHoughGuil>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::GeneralizedHoughGuil>::to_PtrOfGeneralizedHough() generated
	// ("cv::Ptr<cv::GeneralizedHoughGuil>::to_PtrOfGeneralizedHough", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::GeneralizedHough>* cv_PtrLcv_GeneralizedHoughGuilG_to_PtrOfGeneralizedHough(cv::Ptr<cv::GeneralizedHoughGuil>* instance) {
			return new cv::Ptr<cv::GeneralizedHough>(instance->dynamicCast<cv::GeneralizedHough>());
	}

}

