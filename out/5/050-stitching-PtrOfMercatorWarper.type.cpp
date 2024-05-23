extern "C" {
	// cv::Ptr<cv::MercatorWarper>::getInnerPtr() generated
	// ("cv::Ptr<cv::MercatorWarper>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::MercatorWarper* cv_PtrLcv_MercatorWarperG_getInnerPtr_const(const cv::Ptr<cv::MercatorWarper>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::MercatorWarper>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::MercatorWarper>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::MercatorWarper* cv_PtrLcv_MercatorWarperG_getInnerPtrMut(cv::Ptr<cv::MercatorWarper>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::MercatorWarper>::new_null() generated
	// ("cv::Ptr<cv::MercatorWarper>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::MercatorWarper>* cv_PtrLcv_MercatorWarperG_new_null_const() {
			return new cv::Ptr<cv::MercatorWarper>();
	}

	// cv::Ptr<cv::MercatorWarper>::delete() generated
	// ("cv::Ptr<cv::MercatorWarper>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_MercatorWarperG_delete(cv::Ptr<cv::MercatorWarper>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::MercatorWarper>::to_PtrOfWarperCreator() generated
	// ("cv::Ptr<cv::MercatorWarper>::to_PtrOfWarperCreator", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::WarperCreator>* cv_PtrLcv_MercatorWarperG_to_PtrOfWarperCreator(cv::Ptr<cv::MercatorWarper>* instance) {
			return new cv::Ptr<cv::WarperCreator>(instance->dynamicCast<cv::WarperCreator>());
	}

	// cv::Ptr<cv::MercatorWarper>::new(TraitClass) generated
	// ("cv::Ptr<cv::MercatorWarper>::new", vec![(pred!(const, ["val"], ["cv::MercatorWarper"]), _)]),
	cv::Ptr<cv::MercatorWarper>* cv_PtrLcv_MercatorWarperG_new_const_MercatorWarper(cv::MercatorWarper* val) {
			return new cv::Ptr<cv::MercatorWarper>(val);
	}

}

