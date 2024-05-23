extern "C" {
	// cv::Ptr<cv::optflow::GPCTree>::getInnerPtr() generated
	// ("cv::Ptr<cv::optflow::GPCTree>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::optflow::GPCTree* cv_PtrLcv_optflow_GPCTreeG_getInnerPtr_const(const cv::Ptr<cv::optflow::GPCTree>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::optflow::GPCTree>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::optflow::GPCTree>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::optflow::GPCTree* cv_PtrLcv_optflow_GPCTreeG_getInnerPtrMut(cv::Ptr<cv::optflow::GPCTree>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::optflow::GPCTree>::new_null() generated
	// ("cv::Ptr<cv::optflow::GPCTree>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::optflow::GPCTree>* cv_PtrLcv_optflow_GPCTreeG_new_null_const() {
			return new cv::Ptr<cv::optflow::GPCTree>();
	}

	// cv::Ptr<cv::optflow::GPCTree>::delete() generated
	// ("cv::Ptr<cv::optflow::GPCTree>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_optflow_GPCTreeG_delete(cv::Ptr<cv::optflow::GPCTree>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::optflow::GPCTree>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::optflow::GPCTree>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_optflow_GPCTreeG_to_PtrOfAlgorithm(cv::Ptr<cv::optflow::GPCTree>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::optflow::GPCTree>::new(TraitClass) generated
	// ("cv::Ptr<cv::optflow::GPCTree>::new", vec![(pred!(const, ["val"], ["cv::optflow::GPCTree"]), _)]),
	cv::Ptr<cv::optflow::GPCTree>* cv_PtrLcv_optflow_GPCTreeG_new_const_GPCTree(cv::optflow::GPCTree* val) {
			return new cv::Ptr<cv::optflow::GPCTree>(val);
	}

}

