extern "C" {
	// cv::Ptr<cv::MergeMertens>::getInnerPtr() generated
	// ("cv::Ptr<cv::MergeMertens>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::MergeMertens* cv_PtrLcv_MergeMertensG_getInnerPtr_const(const cv::Ptr<cv::MergeMertens>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::MergeMertens>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::MergeMertens>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::MergeMertens* cv_PtrLcv_MergeMertensG_getInnerPtrMut(cv::Ptr<cv::MergeMertens>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::MergeMertens>::new_null() generated
	// ("cv::Ptr<cv::MergeMertens>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::MergeMertens>* cv_PtrLcv_MergeMertensG_new_null_const() {
			return new cv::Ptr<cv::MergeMertens>();
	}

	// cv::Ptr<cv::MergeMertens>::delete() generated
	// ("cv::Ptr<cv::MergeMertens>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_MergeMertensG_delete(cv::Ptr<cv::MergeMertens>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::MergeMertens>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::MergeMertens>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_MergeMertensG_to_PtrOfAlgorithm(cv::Ptr<cv::MergeMertens>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::MergeMertens>::to_PtrOfMergeExposures() generated
	// ("cv::Ptr<cv::MergeMertens>::to_PtrOfMergeExposures", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::MergeExposures>* cv_PtrLcv_MergeMertensG_to_PtrOfMergeExposures(cv::Ptr<cv::MergeMertens>* instance) {
			return new cv::Ptr<cv::MergeExposures>(instance->dynamicCast<cv::MergeExposures>());
	}

}

