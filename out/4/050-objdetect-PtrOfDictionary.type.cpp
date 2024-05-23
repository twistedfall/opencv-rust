extern "C" {
	// cv::Ptr<cv::aruco::Dictionary>::getInnerPtr() generated
	// ("cv::Ptr<cv::aruco::Dictionary>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::aruco::Dictionary* cv_PtrLcv_aruco_DictionaryG_getInnerPtr_const(const cv::Ptr<cv::aruco::Dictionary>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::aruco::Dictionary>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::aruco::Dictionary>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::aruco::Dictionary* cv_PtrLcv_aruco_DictionaryG_getInnerPtrMut(cv::Ptr<cv::aruco::Dictionary>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::aruco::Dictionary>::new_null() generated
	// ("cv::Ptr<cv::aruco::Dictionary>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::aruco::Dictionary>* cv_PtrLcv_aruco_DictionaryG_new_null_const() {
			return new cv::Ptr<cv::aruco::Dictionary>();
	}

	// cv::Ptr<cv::aruco::Dictionary>::delete() generated
	// ("cv::Ptr<cv::aruco::Dictionary>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_aruco_DictionaryG_delete(cv::Ptr<cv::aruco::Dictionary>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::aruco::Dictionary>::new(TraitClass) generated
	// ("cv::Ptr<cv::aruco::Dictionary>::new", vec![(pred!(const, ["val"], ["cv::aruco::Dictionary"]), _)]),
	cv::Ptr<cv::aruco::Dictionary>* cv_PtrLcv_aruco_DictionaryG_new_const_Dictionary(cv::aruco::Dictionary* val) {
			return new cv::Ptr<cv::aruco::Dictionary>(val);
	}

}

