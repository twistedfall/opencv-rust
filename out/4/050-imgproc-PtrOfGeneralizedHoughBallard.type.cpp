extern "C" {
	// cv::Ptr<cv::GeneralizedHoughBallard>::getInnerPtr() generated
	// ("cv::Ptr<cv::GeneralizedHoughBallard>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::GeneralizedHoughBallard* cv_PtrLcv_GeneralizedHoughBallardG_getInnerPtr_const(const cv::Ptr<cv::GeneralizedHoughBallard>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::GeneralizedHoughBallard>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::GeneralizedHoughBallard>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::GeneralizedHoughBallard* cv_PtrLcv_GeneralizedHoughBallardG_getInnerPtrMut(cv::Ptr<cv::GeneralizedHoughBallard>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::GeneralizedHoughBallard>::new_null() generated
	// ("cv::Ptr<cv::GeneralizedHoughBallard>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::GeneralizedHoughBallard>* cv_PtrLcv_GeneralizedHoughBallardG_new_null_const() {
			return new cv::Ptr<cv::GeneralizedHoughBallard>();
	}

	// cv::Ptr<cv::GeneralizedHoughBallard>::delete() generated
	// ("cv::Ptr<cv::GeneralizedHoughBallard>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_GeneralizedHoughBallardG_delete(cv::Ptr<cv::GeneralizedHoughBallard>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::GeneralizedHoughBallard>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::GeneralizedHoughBallard>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_GeneralizedHoughBallardG_to_PtrOfAlgorithm(cv::Ptr<cv::GeneralizedHoughBallard>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::GeneralizedHoughBallard>::to_PtrOfGeneralizedHough() generated
	// ("cv::Ptr<cv::GeneralizedHoughBallard>::to_PtrOfGeneralizedHough", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::GeneralizedHough>* cv_PtrLcv_GeneralizedHoughBallardG_to_PtrOfGeneralizedHough(cv::Ptr<cv::GeneralizedHoughBallard>* instance) {
			return new cv::Ptr<cv::GeneralizedHough>(instance->dynamicCast<cv::GeneralizedHough>());
	}

}

