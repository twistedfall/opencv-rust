extern "C" {
	// cv::Ptr<cv::Feature2D>::getInnerPtr() generated
	// ("cv::Ptr<cv::Feature2D>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::Feature2D* cv_PtrLcv_Feature2DG_getInnerPtr_const(const cv::Ptr<cv::Feature2D>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::Feature2D>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::Feature2D>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::Feature2D* cv_PtrLcv_Feature2DG_getInnerPtrMut(cv::Ptr<cv::Feature2D>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::Feature2D>::new_null() generated
	// ("cv::Ptr<cv::Feature2D>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::Feature2D>* cv_PtrLcv_Feature2DG_new_null_const() {
			return new cv::Ptr<cv::Feature2D>();
	}

	// cv::Ptr<cv::Feature2D>::delete() generated
	// ("cv::Ptr<cv::Feature2D>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_Feature2DG_delete(cv::Ptr<cv::Feature2D>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::Feature2D>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::Feature2D>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_Feature2DG_to_PtrOfAlgorithm(cv::Ptr<cv::Feature2D>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::Feature2D>::new(TraitClass) generated
	// ("cv::Ptr<cv::Feature2D>::new", vec![(pred!(const, ["val"], ["cv::Feature2D"]), _)]),
	cv::Ptr<cv::Feature2D>* cv_PtrLcv_Feature2DG_new_const_Feature2D(cv::Feature2D* val) {
			return new cv::Ptr<cv::Feature2D>(val);
	}

}

