extern "C" {
	// cv::Ptr<cv::ximgproc::ContourFitting>::getInnerPtr() generated
	// ("cv::Ptr<cv::ximgproc::ContourFitting>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::ximgproc::ContourFitting* cv_PtrLcv_ximgproc_ContourFittingG_getInnerPtr_const(const cv::Ptr<cv::ximgproc::ContourFitting>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ximgproc::ContourFitting>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::ximgproc::ContourFitting>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::ximgproc::ContourFitting* cv_PtrLcv_ximgproc_ContourFittingG_getInnerPtrMut(cv::Ptr<cv::ximgproc::ContourFitting>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ximgproc::ContourFitting>::new_null() generated
	// ("cv::Ptr<cv::ximgproc::ContourFitting>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::ximgproc::ContourFitting>* cv_PtrLcv_ximgproc_ContourFittingG_new_null_const() {
			return new cv::Ptr<cv::ximgproc::ContourFitting>();
	}

	// cv::Ptr<cv::ximgproc::ContourFitting>::delete() generated
	// ("cv::Ptr<cv::ximgproc::ContourFitting>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_ximgproc_ContourFittingG_delete(cv::Ptr<cv::ximgproc::ContourFitting>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::ximgproc::ContourFitting>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::ximgproc::ContourFitting>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ximgproc_ContourFittingG_to_PtrOfAlgorithm(cv::Ptr<cv::ximgproc::ContourFitting>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::ximgproc::ContourFitting>::new(TraitClass) generated
	// ("cv::Ptr<cv::ximgproc::ContourFitting>::new", vec![(pred!(const, ["val"], ["cv::ximgproc::ContourFitting"]), _)]),
	cv::Ptr<cv::ximgproc::ContourFitting>* cv_PtrLcv_ximgproc_ContourFittingG_new_const_ContourFitting(cv::ximgproc::ContourFitting* val) {
			return new cv::Ptr<cv::ximgproc::ContourFitting>(val);
	}

}

