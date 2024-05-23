extern "C" {
	// cv::Ptr<cv::SphericalWarper>::getInnerPtr() generated
	// ("cv::Ptr<cv::SphericalWarper>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::SphericalWarper* cv_PtrLcv_SphericalWarperG_getInnerPtr_const(const cv::Ptr<cv::SphericalWarper>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::SphericalWarper>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::SphericalWarper>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::SphericalWarper* cv_PtrLcv_SphericalWarperG_getInnerPtrMut(cv::Ptr<cv::SphericalWarper>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::SphericalWarper>::new_null() generated
	// ("cv::Ptr<cv::SphericalWarper>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::SphericalWarper>* cv_PtrLcv_SphericalWarperG_new_null_const() {
			return new cv::Ptr<cv::SphericalWarper>();
	}

	// cv::Ptr<cv::SphericalWarper>::delete() generated
	// ("cv::Ptr<cv::SphericalWarper>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_SphericalWarperG_delete(cv::Ptr<cv::SphericalWarper>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::SphericalWarper>::to_PtrOfWarperCreator() generated
	// ("cv::Ptr<cv::SphericalWarper>::to_PtrOfWarperCreator", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::WarperCreator>* cv_PtrLcv_SphericalWarperG_to_PtrOfWarperCreator(cv::Ptr<cv::SphericalWarper>* instance) {
			return new cv::Ptr<cv::WarperCreator>(instance->dynamicCast<cv::WarperCreator>());
	}

	// cv::Ptr<cv::SphericalWarper>::new(TraitClass) generated
	// ("cv::Ptr<cv::SphericalWarper>::new", vec![(pred!(const, ["val"], ["cv::SphericalWarper"]), _)]),
	cv::Ptr<cv::SphericalWarper>* cv_PtrLcv_SphericalWarperG_new_const_SphericalWarper(cv::SphericalWarper* val) {
			return new cv::Ptr<cv::SphericalWarper>(val);
	}

}

