extern "C" {
	// cv::Ptr<cv::PaniniPortraitWarper>::getInnerPtr() generated
	// ("cv::Ptr<cv::PaniniPortraitWarper>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::PaniniPortraitWarper* cv_PtrLcv_PaniniPortraitWarperG_getInnerPtr_const(const cv::Ptr<cv::PaniniPortraitWarper>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::PaniniPortraitWarper>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::PaniniPortraitWarper>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::PaniniPortraitWarper* cv_PtrLcv_PaniniPortraitWarperG_getInnerPtrMut(cv::Ptr<cv::PaniniPortraitWarper>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::PaniniPortraitWarper>::new_null() generated
	// ("cv::Ptr<cv::PaniniPortraitWarper>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::PaniniPortraitWarper>* cv_PtrLcv_PaniniPortraitWarperG_new_null_const() {
			return new cv::Ptr<cv::PaniniPortraitWarper>();
	}

	// cv::Ptr<cv::PaniniPortraitWarper>::delete() generated
	// ("cv::Ptr<cv::PaniniPortraitWarper>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_PaniniPortraitWarperG_delete(cv::Ptr<cv::PaniniPortraitWarper>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::PaniniPortraitWarper>::to_PtrOfWarperCreator() generated
	// ("cv::Ptr<cv::PaniniPortraitWarper>::to_PtrOfWarperCreator", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::WarperCreator>* cv_PtrLcv_PaniniPortraitWarperG_to_PtrOfWarperCreator(cv::Ptr<cv::PaniniPortraitWarper>* instance) {
			return new cv::Ptr<cv::WarperCreator>(instance->dynamicCast<cv::WarperCreator>());
	}

	// cv::Ptr<cv::PaniniPortraitWarper>::new(TraitClass) generated
	// ("cv::Ptr<cv::PaniniPortraitWarper>::new", vec![(pred!(const, ["val"], ["cv::PaniniPortraitWarper"]), _)]),
	cv::Ptr<cv::PaniniPortraitWarper>* cv_PtrLcv_PaniniPortraitWarperG_new_const_PaniniPortraitWarper(cv::PaniniPortraitWarper* val) {
			return new cv::Ptr<cv::PaniniPortraitWarper>(val);
	}

}

