extern "C" {
	// cv::Ptr<cv::StereographicWarper>::getInnerPtr() generated
	// ("cv::Ptr<cv::StereographicWarper>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::StereographicWarper* cv_PtrLcv_StereographicWarperG_getInnerPtr_const(const cv::Ptr<cv::StereographicWarper>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::StereographicWarper>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::StereographicWarper>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::StereographicWarper* cv_PtrLcv_StereographicWarperG_getInnerPtrMut(cv::Ptr<cv::StereographicWarper>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::StereographicWarper>::new_null() generated
	// ("cv::Ptr<cv::StereographicWarper>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::StereographicWarper>* cv_PtrLcv_StereographicWarperG_new_null_const() {
			return new cv::Ptr<cv::StereographicWarper>();
	}

	// cv::Ptr<cv::StereographicWarper>::delete() generated
	// ("cv::Ptr<cv::StereographicWarper>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_StereographicWarperG_delete(cv::Ptr<cv::StereographicWarper>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::StereographicWarper>::to_PtrOfWarperCreator() generated
	// ("cv::Ptr<cv::StereographicWarper>::to_PtrOfWarperCreator", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::WarperCreator>* cv_PtrLcv_StereographicWarperG_to_PtrOfWarperCreator(cv::Ptr<cv::StereographicWarper>* instance) {
			return new cv::Ptr<cv::WarperCreator>(instance->dynamicCast<cv::WarperCreator>());
	}

	// cv::Ptr<cv::StereographicWarper>::new(TraitClass) generated
	// ("cv::Ptr<cv::StereographicWarper>::new", vec![(pred!(const, ["val"], ["cv::StereographicWarper"]), _)]),
	cv::Ptr<cv::StereographicWarper>* cv_PtrLcv_StereographicWarperG_new_const_StereographicWarper(cv::StereographicWarper* val) {
			return new cv::Ptr<cv::StereographicWarper>(val);
	}

}

