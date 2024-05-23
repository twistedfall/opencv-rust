extern "C" {
	// cv::Ptr<cv::FisheyeWarper>::getInnerPtr() generated
	// ("cv::Ptr<cv::FisheyeWarper>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::FisheyeWarper* cv_PtrLcv_FisheyeWarperG_getInnerPtr_const(const cv::Ptr<cv::FisheyeWarper>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::FisheyeWarper>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::FisheyeWarper>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::FisheyeWarper* cv_PtrLcv_FisheyeWarperG_getInnerPtrMut(cv::Ptr<cv::FisheyeWarper>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::FisheyeWarper>::new_null() generated
	// ("cv::Ptr<cv::FisheyeWarper>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::FisheyeWarper>* cv_PtrLcv_FisheyeWarperG_new_null_const() {
			return new cv::Ptr<cv::FisheyeWarper>();
	}

	// cv::Ptr<cv::FisheyeWarper>::delete() generated
	// ("cv::Ptr<cv::FisheyeWarper>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_FisheyeWarperG_delete(cv::Ptr<cv::FisheyeWarper>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::FisheyeWarper>::to_PtrOfWarperCreator() generated
	// ("cv::Ptr<cv::FisheyeWarper>::to_PtrOfWarperCreator", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::WarperCreator>* cv_PtrLcv_FisheyeWarperG_to_PtrOfWarperCreator(cv::Ptr<cv::FisheyeWarper>* instance) {
			return new cv::Ptr<cv::WarperCreator>(instance->dynamicCast<cv::WarperCreator>());
	}

	// cv::Ptr<cv::FisheyeWarper>::new(TraitClass) generated
	// ("cv::Ptr<cv::FisheyeWarper>::new", vec![(pred!(const, ["val"], ["cv::FisheyeWarper"]), _)]),
	cv::Ptr<cv::FisheyeWarper>* cv_PtrLcv_FisheyeWarperG_new_const_FisheyeWarper(cv::FisheyeWarper* val) {
			return new cv::Ptr<cv::FisheyeWarper>(val);
	}

}

