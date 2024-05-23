extern "C" {
	// cv::Ptr<cv::AffineWarper>::getInnerPtr() generated
	// ("cv::Ptr<cv::AffineWarper>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::AffineWarper* cv_PtrLcv_AffineWarperG_getInnerPtr_const(const cv::Ptr<cv::AffineWarper>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::AffineWarper>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::AffineWarper>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::AffineWarper* cv_PtrLcv_AffineWarperG_getInnerPtrMut(cv::Ptr<cv::AffineWarper>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::AffineWarper>::new_null() generated
	// ("cv::Ptr<cv::AffineWarper>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::AffineWarper>* cv_PtrLcv_AffineWarperG_new_null_const() {
			return new cv::Ptr<cv::AffineWarper>();
	}

	// cv::Ptr<cv::AffineWarper>::delete() generated
	// ("cv::Ptr<cv::AffineWarper>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_AffineWarperG_delete(cv::Ptr<cv::AffineWarper>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::AffineWarper>::to_PtrOfWarperCreator() generated
	// ("cv::Ptr<cv::AffineWarper>::to_PtrOfWarperCreator", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::WarperCreator>* cv_PtrLcv_AffineWarperG_to_PtrOfWarperCreator(cv::Ptr<cv::AffineWarper>* instance) {
			return new cv::Ptr<cv::WarperCreator>(instance->dynamicCast<cv::WarperCreator>());
	}

	// cv::Ptr<cv::AffineWarper>::new(TraitClass) generated
	// ("cv::Ptr<cv::AffineWarper>::new", vec![(pred!(const, ["val"], ["cv::AffineWarper"]), _)]),
	cv::Ptr<cv::AffineWarper>* cv_PtrLcv_AffineWarperG_new_const_AffineWarper(cv::AffineWarper* val) {
			return new cv::Ptr<cv::AffineWarper>(val);
	}

}

