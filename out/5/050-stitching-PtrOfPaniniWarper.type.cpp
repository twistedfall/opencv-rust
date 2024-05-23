extern "C" {
	// cv::Ptr<cv::PaniniWarper>::getInnerPtr() generated
	// ("cv::Ptr<cv::PaniniWarper>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::PaniniWarper* cv_PtrLcv_PaniniWarperG_getInnerPtr_const(const cv::Ptr<cv::PaniniWarper>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::PaniniWarper>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::PaniniWarper>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::PaniniWarper* cv_PtrLcv_PaniniWarperG_getInnerPtrMut(cv::Ptr<cv::PaniniWarper>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::PaniniWarper>::new_null() generated
	// ("cv::Ptr<cv::PaniniWarper>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::PaniniWarper>* cv_PtrLcv_PaniniWarperG_new_null_const() {
			return new cv::Ptr<cv::PaniniWarper>();
	}

	// cv::Ptr<cv::PaniniWarper>::delete() generated
	// ("cv::Ptr<cv::PaniniWarper>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_PaniniWarperG_delete(cv::Ptr<cv::PaniniWarper>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::PaniniWarper>::to_PtrOfWarperCreator() generated
	// ("cv::Ptr<cv::PaniniWarper>::to_PtrOfWarperCreator", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::WarperCreator>* cv_PtrLcv_PaniniWarperG_to_PtrOfWarperCreator(cv::Ptr<cv::PaniniWarper>* instance) {
			return new cv::Ptr<cv::WarperCreator>(instance->dynamicCast<cv::WarperCreator>());
	}

	// cv::Ptr<cv::PaniniWarper>::new(TraitClass) generated
	// ("cv::Ptr<cv::PaniniWarper>::new", vec![(pred!(const, ["val"], ["cv::PaniniWarper"]), _)]),
	cv::Ptr<cv::PaniniWarper>* cv_PtrLcv_PaniniWarperG_new_const_PaniniWarper(cv::PaniniWarper* val) {
			return new cv::Ptr<cv::PaniniWarper>(val);
	}

}

