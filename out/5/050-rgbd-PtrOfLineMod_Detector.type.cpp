extern "C" {
	// cv::Ptr<cv::linemod::Detector>::getInnerPtr() generated
	// ("cv::Ptr<cv::linemod::Detector>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::linemod::Detector* cv_PtrLcv_linemod_DetectorG_getInnerPtr_const(const cv::Ptr<cv::linemod::Detector>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::linemod::Detector>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::linemod::Detector>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::linemod::Detector* cv_PtrLcv_linemod_DetectorG_getInnerPtrMut(cv::Ptr<cv::linemod::Detector>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::linemod::Detector>::new_null() generated
	// ("cv::Ptr<cv::linemod::Detector>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::linemod::Detector>* cv_PtrLcv_linemod_DetectorG_new_null_const() {
			return new cv::Ptr<cv::linemod::Detector>();
	}

	// cv::Ptr<cv::linemod::Detector>::delete() generated
	// ("cv::Ptr<cv::linemod::Detector>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_linemod_DetectorG_delete(cv::Ptr<cv::linemod::Detector>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::linemod::Detector>::new(TraitClass) generated
	// ("cv::Ptr<cv::linemod::Detector>::new", vec![(pred!(const, ["val"], ["cv::linemod::Detector"]), _)]),
	cv::Ptr<cv::linemod::Detector>* cv_PtrLcv_linemod_DetectorG_new_const_Detector(cv::linemod::Detector* val) {
			return new cv::Ptr<cv::linemod::Detector>(val);
	}

}

