extern "C" {
	// cv::Ptr<cv::CompressedRectilinearPortraitWarper>::getInnerPtr() generated
	// ("cv::Ptr<cv::CompressedRectilinearPortraitWarper>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::CompressedRectilinearPortraitWarper* cv_PtrLcv_CompressedRectilinearPortraitWarperG_getInnerPtr_const(const cv::Ptr<cv::CompressedRectilinearPortraitWarper>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::CompressedRectilinearPortraitWarper>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::CompressedRectilinearPortraitWarper>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::CompressedRectilinearPortraitWarper* cv_PtrLcv_CompressedRectilinearPortraitWarperG_getInnerPtrMut(cv::Ptr<cv::CompressedRectilinearPortraitWarper>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::CompressedRectilinearPortraitWarper>::new_null() generated
	// ("cv::Ptr<cv::CompressedRectilinearPortraitWarper>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::CompressedRectilinearPortraitWarper>* cv_PtrLcv_CompressedRectilinearPortraitWarperG_new_null_const() {
			return new cv::Ptr<cv::CompressedRectilinearPortraitWarper>();
	}

	// cv::Ptr<cv::CompressedRectilinearPortraitWarper>::delete() generated
	// ("cv::Ptr<cv::CompressedRectilinearPortraitWarper>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_CompressedRectilinearPortraitWarperG_delete(cv::Ptr<cv::CompressedRectilinearPortraitWarper>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::CompressedRectilinearPortraitWarper>::to_PtrOfWarperCreator() generated
	// ("cv::Ptr<cv::CompressedRectilinearPortraitWarper>::to_PtrOfWarperCreator", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::WarperCreator>* cv_PtrLcv_CompressedRectilinearPortraitWarperG_to_PtrOfWarperCreator(cv::Ptr<cv::CompressedRectilinearPortraitWarper>* instance) {
			return new cv::Ptr<cv::WarperCreator>(instance->dynamicCast<cv::WarperCreator>());
	}

	// cv::Ptr<cv::CompressedRectilinearPortraitWarper>::new(TraitClass) generated
	// ("cv::Ptr<cv::CompressedRectilinearPortraitWarper>::new", vec![(pred!(const, ["val"], ["cv::CompressedRectilinearPortraitWarper"]), _)]),
	cv::Ptr<cv::CompressedRectilinearPortraitWarper>* cv_PtrLcv_CompressedRectilinearPortraitWarperG_new_const_CompressedRectilinearPortraitWarper(cv::CompressedRectilinearPortraitWarper* val) {
			return new cv::Ptr<cv::CompressedRectilinearPortraitWarper>(val);
	}

}

