extern "C" {
	// cv::Ptr<cv::utils::nested::OriginalClassName>::getInnerPtr() generated
	// ("cv::Ptr<cv::utils::nested::OriginalClassName>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::utils::nested::OriginalClassName* cv_PtrLcv_utils_nested_OriginalClassNameG_getInnerPtr_const(const cv::Ptr<cv::utils::nested::OriginalClassName>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::utils::nested::OriginalClassName>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::utils::nested::OriginalClassName>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::utils::nested::OriginalClassName* cv_PtrLcv_utils_nested_OriginalClassNameG_getInnerPtrMut(cv::Ptr<cv::utils::nested::OriginalClassName>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::utils::nested::OriginalClassName>::new_null() generated
	// ("cv::Ptr<cv::utils::nested::OriginalClassName>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::utils::nested::OriginalClassName>* cv_PtrLcv_utils_nested_OriginalClassNameG_new_null_const() {
			return new cv::Ptr<cv::utils::nested::OriginalClassName>();
	}

	// cv::Ptr<cv::utils::nested::OriginalClassName>::delete() generated
	// ("cv::Ptr<cv::utils::nested::OriginalClassName>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_utils_nested_OriginalClassNameG_delete(cv::Ptr<cv::utils::nested::OriginalClassName>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::utils::nested::OriginalClassName>::new(TraitClass) generated
	// ("cv::Ptr<cv::utils::nested::OriginalClassName>::new", vec![(pred!(const, ["val"], ["cv::utils::nested::OriginalClassName"]), _)]),
	cv::Ptr<cv::utils::nested::OriginalClassName>* cv_PtrLcv_utils_nested_OriginalClassNameG_new_const_OriginalClassName(cv::utils::nested::OriginalClassName* val) {
			return new cv::Ptr<cv::utils::nested::OriginalClassName>(val);
	}

}

