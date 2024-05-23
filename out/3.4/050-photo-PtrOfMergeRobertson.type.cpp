extern "C" {
	// cv::Ptr<cv::MergeRobertson>::getInnerPtr() generated
	// ("cv::Ptr<cv::MergeRobertson>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::MergeRobertson* cv_PtrLcv_MergeRobertsonG_getInnerPtr_const(const cv::Ptr<cv::MergeRobertson>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::MergeRobertson>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::MergeRobertson>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::MergeRobertson* cv_PtrLcv_MergeRobertsonG_getInnerPtrMut(cv::Ptr<cv::MergeRobertson>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::MergeRobertson>::new_null() generated
	// ("cv::Ptr<cv::MergeRobertson>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::MergeRobertson>* cv_PtrLcv_MergeRobertsonG_new_null_const() {
			return new cv::Ptr<cv::MergeRobertson>();
	}

	// cv::Ptr<cv::MergeRobertson>::delete() generated
	// ("cv::Ptr<cv::MergeRobertson>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_MergeRobertsonG_delete(cv::Ptr<cv::MergeRobertson>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::MergeRobertson>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::MergeRobertson>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_MergeRobertsonG_to_PtrOfAlgorithm(cv::Ptr<cv::MergeRobertson>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::MergeRobertson>::to_PtrOfMergeExposures() generated
	// ("cv::Ptr<cv::MergeRobertson>::to_PtrOfMergeExposures", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::MergeExposures>* cv_PtrLcv_MergeRobertsonG_to_PtrOfMergeExposures(cv::Ptr<cv::MergeRobertson>* instance) {
			return new cv::Ptr<cv::MergeExposures>(instance->dynamicCast<cv::MergeExposures>());
	}

}

