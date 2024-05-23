extern "C" {
	// cv::Ptr<cv::PlaneWarper>::getInnerPtr() generated
	// ("cv::Ptr<cv::PlaneWarper>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::PlaneWarper* cv_PtrLcv_PlaneWarperG_getInnerPtr_const(const cv::Ptr<cv::PlaneWarper>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::PlaneWarper>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::PlaneWarper>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::PlaneWarper* cv_PtrLcv_PlaneWarperG_getInnerPtrMut(cv::Ptr<cv::PlaneWarper>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::PlaneWarper>::new_null() generated
	// ("cv::Ptr<cv::PlaneWarper>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::PlaneWarper>* cv_PtrLcv_PlaneWarperG_new_null_const() {
			return new cv::Ptr<cv::PlaneWarper>();
	}

	// cv::Ptr<cv::PlaneWarper>::delete() generated
	// ("cv::Ptr<cv::PlaneWarper>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_PlaneWarperG_delete(cv::Ptr<cv::PlaneWarper>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::PlaneWarper>::to_PtrOfWarperCreator() generated
	// ("cv::Ptr<cv::PlaneWarper>::to_PtrOfWarperCreator", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::WarperCreator>* cv_PtrLcv_PlaneWarperG_to_PtrOfWarperCreator(cv::Ptr<cv::PlaneWarper>* instance) {
			return new cv::Ptr<cv::WarperCreator>(instance->dynamicCast<cv::WarperCreator>());
	}

	// cv::Ptr<cv::PlaneWarper>::new(TraitClass) generated
	// ("cv::Ptr<cv::PlaneWarper>::new", vec![(pred!(const, ["val"], ["cv::PlaneWarper"]), _)]),
	cv::Ptr<cv::PlaneWarper>* cv_PtrLcv_PlaneWarperG_new_const_PlaneWarper(cv::PlaneWarper* val) {
			return new cv::Ptr<cv::PlaneWarper>(val);
	}

}

