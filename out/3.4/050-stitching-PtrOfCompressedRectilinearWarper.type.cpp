extern "C" {
	// cv::Ptr<cv::CompressedRectilinearWarper>::getInnerPtr() generated
	// ("cv::Ptr<cv::CompressedRectilinearWarper>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::CompressedRectilinearWarper* cv_PtrLcv_CompressedRectilinearWarperG_getInnerPtr_const(const cv::Ptr<cv::CompressedRectilinearWarper>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::CompressedRectilinearWarper>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::CompressedRectilinearWarper>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::CompressedRectilinearWarper* cv_PtrLcv_CompressedRectilinearWarperG_getInnerPtrMut(cv::Ptr<cv::CompressedRectilinearWarper>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::CompressedRectilinearWarper>::new_null() generated
	// ("cv::Ptr<cv::CompressedRectilinearWarper>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::CompressedRectilinearWarper>* cv_PtrLcv_CompressedRectilinearWarperG_new_null_const() {
			return new cv::Ptr<cv::CompressedRectilinearWarper>();
	}

	// cv::Ptr<cv::CompressedRectilinearWarper>::delete() generated
	// ("cv::Ptr<cv::CompressedRectilinearWarper>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_CompressedRectilinearWarperG_delete(cv::Ptr<cv::CompressedRectilinearWarper>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::CompressedRectilinearWarper>::to_PtrOfWarperCreator() generated
	// ("cv::Ptr<cv::CompressedRectilinearWarper>::to_PtrOfWarperCreator", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::WarperCreator>* cv_PtrLcv_CompressedRectilinearWarperG_to_PtrOfWarperCreator(cv::Ptr<cv::CompressedRectilinearWarper>* instance) {
			return new cv::Ptr<cv::WarperCreator>(instance->dynamicCast<cv::WarperCreator>());
	}

	// cv::Ptr<cv::CompressedRectilinearWarper>::new(TraitClass) generated
	// ("cv::Ptr<cv::CompressedRectilinearWarper>::new", vec![(pred!(const, ["val"], ["cv::CompressedRectilinearWarper"]), _)]),
	cv::Ptr<cv::CompressedRectilinearWarper>* cv_PtrLcv_CompressedRectilinearWarperG_new_const_CompressedRectilinearWarper(cv::CompressedRectilinearWarper* val) {
			return new cv::Ptr<cv::CompressedRectilinearWarper>(val);
	}

}

