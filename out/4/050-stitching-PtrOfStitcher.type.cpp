extern "C" {
	// cv::Ptr<cv::Stitcher>::getInnerPtr() generated
	// ("cv::Ptr<cv::Stitcher>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::Stitcher* cv_PtrLcv_StitcherG_getInnerPtr_const(const cv::Ptr<cv::Stitcher>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::Stitcher>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::Stitcher>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::Stitcher* cv_PtrLcv_StitcherG_getInnerPtrMut(cv::Ptr<cv::Stitcher>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::Stitcher>::new_null() generated
	// ("cv::Ptr<cv::Stitcher>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::Stitcher>* cv_PtrLcv_StitcherG_new_null_const() {
			return new cv::Ptr<cv::Stitcher>();
	}

	// cv::Ptr<cv::Stitcher>::delete() generated
	// ("cv::Ptr<cv::Stitcher>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_StitcherG_delete(cv::Ptr<cv::Stitcher>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::Stitcher>::new(TraitClass) generated
	// ("cv::Ptr<cv::Stitcher>::new", vec![(pred!(const, ["val"], ["cv::Stitcher"]), _)]),
	cv::Ptr<cv::Stitcher>* cv_PtrLcv_StitcherG_new_const_Stitcher(cv::Stitcher* val) {
			return new cv::Ptr<cv::Stitcher>(val);
	}

}

