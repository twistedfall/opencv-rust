extern "C" {
	// cv::Ptr<cv::TransverseMercatorWarper>::getInnerPtr() generated
	// ("cv::Ptr<cv::TransverseMercatorWarper>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::TransverseMercatorWarper* cv_PtrLcv_TransverseMercatorWarperG_getInnerPtr_const(const cv::Ptr<cv::TransverseMercatorWarper>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::TransverseMercatorWarper>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::TransverseMercatorWarper>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::TransverseMercatorWarper* cv_PtrLcv_TransverseMercatorWarperG_getInnerPtrMut(cv::Ptr<cv::TransverseMercatorWarper>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::TransverseMercatorWarper>::new_null() generated
	// ("cv::Ptr<cv::TransverseMercatorWarper>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::TransverseMercatorWarper>* cv_PtrLcv_TransverseMercatorWarperG_new_null_const() {
			return new cv::Ptr<cv::TransverseMercatorWarper>();
	}

	// cv::Ptr<cv::TransverseMercatorWarper>::delete() generated
	// ("cv::Ptr<cv::TransverseMercatorWarper>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_TransverseMercatorWarperG_delete(cv::Ptr<cv::TransverseMercatorWarper>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::TransverseMercatorWarper>::to_PtrOfWarperCreator() generated
	// ("cv::Ptr<cv::TransverseMercatorWarper>::to_PtrOfWarperCreator", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::WarperCreator>* cv_PtrLcv_TransverseMercatorWarperG_to_PtrOfWarperCreator(cv::Ptr<cv::TransverseMercatorWarper>* instance) {
			return new cv::Ptr<cv::WarperCreator>(instance->dynamicCast<cv::WarperCreator>());
	}

	// cv::Ptr<cv::TransverseMercatorWarper>::new(TraitClass) generated
	// ("cv::Ptr<cv::TransverseMercatorWarper>::new", vec![(pred!(const, ["val"], ["cv::TransverseMercatorWarper"]), _)]),
	cv::Ptr<cv::TransverseMercatorWarper>* cv_PtrLcv_TransverseMercatorWarperG_new_const_TransverseMercatorWarper(cv::TransverseMercatorWarper* val) {
			return new cv::Ptr<cv::TransverseMercatorWarper>(val);
	}

}

