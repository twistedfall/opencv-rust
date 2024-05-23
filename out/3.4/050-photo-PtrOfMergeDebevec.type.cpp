extern "C" {
	// cv::Ptr<cv::MergeDebevec>::getInnerPtr() generated
	// ("cv::Ptr<cv::MergeDebevec>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::MergeDebevec* cv_PtrLcv_MergeDebevecG_getInnerPtr_const(const cv::Ptr<cv::MergeDebevec>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::MergeDebevec>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::MergeDebevec>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::MergeDebevec* cv_PtrLcv_MergeDebevecG_getInnerPtrMut(cv::Ptr<cv::MergeDebevec>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::MergeDebevec>::new_null() generated
	// ("cv::Ptr<cv::MergeDebevec>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::MergeDebevec>* cv_PtrLcv_MergeDebevecG_new_null_const() {
			return new cv::Ptr<cv::MergeDebevec>();
	}

	// cv::Ptr<cv::MergeDebevec>::delete() generated
	// ("cv::Ptr<cv::MergeDebevec>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_MergeDebevecG_delete(cv::Ptr<cv::MergeDebevec>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::MergeDebevec>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::MergeDebevec>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_MergeDebevecG_to_PtrOfAlgorithm(cv::Ptr<cv::MergeDebevec>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::MergeDebevec>::to_PtrOfMergeExposures() generated
	// ("cv::Ptr<cv::MergeDebevec>::to_PtrOfMergeExposures", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::MergeExposures>* cv_PtrLcv_MergeDebevecG_to_PtrOfMergeExposures(cv::Ptr<cv::MergeDebevec>* instance) {
			return new cv::Ptr<cv::MergeExposures>(instance->dynamicCast<cv::MergeExposures>());
	}

}

