extern "C" {
	// cv::Ptr<cv::KeyPoint>::getInnerPtr() generated
	// ("cv::Ptr<cv::KeyPoint>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::KeyPoint* cv_PtrLcv_KeyPointG_getInnerPtr_const(const cv::Ptr<cv::KeyPoint>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::KeyPoint>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::KeyPoint>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::KeyPoint* cv_PtrLcv_KeyPointG_getInnerPtrMut(cv::Ptr<cv::KeyPoint>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::KeyPoint>::new_null() generated
	// ("cv::Ptr<cv::KeyPoint>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::KeyPoint>* cv_PtrLcv_KeyPointG_new_null_const() {
			return new cv::Ptr<cv::KeyPoint>();
	}

	// cv::Ptr<cv::KeyPoint>::delete() generated
	// ("cv::Ptr<cv::KeyPoint>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_KeyPointG_delete(cv::Ptr<cv::KeyPoint>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::KeyPoint>::new(TraitClass) generated
	// ("cv::Ptr<cv::KeyPoint>::new", vec![(pred!(const, ["val"], ["cv::KeyPoint"]), _)]),
	cv::Ptr<cv::KeyPoint>* cv_PtrLcv_KeyPointG_new_const_KeyPoint(cv::KeyPoint* val) {
			return new cv::Ptr<cv::KeyPoint>(val);
	}

}

