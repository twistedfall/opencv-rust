extern "C" {
	// cv::Ptr<cv::mcc::DetectorParameters>::getInnerPtr() generated
	// ("cv::Ptr<cv::mcc::DetectorParameters>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::mcc::DetectorParameters* cv_PtrLcv_mcc_DetectorParametersG_getInnerPtr_const(const cv::Ptr<cv::mcc::DetectorParameters>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::mcc::DetectorParameters>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::mcc::DetectorParameters>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::mcc::DetectorParameters* cv_PtrLcv_mcc_DetectorParametersG_getInnerPtrMut(cv::Ptr<cv::mcc::DetectorParameters>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::mcc::DetectorParameters>::new_null() generated
	// ("cv::Ptr<cv::mcc::DetectorParameters>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::mcc::DetectorParameters>* cv_PtrLcv_mcc_DetectorParametersG_new_null_const() {
			return new cv::Ptr<cv::mcc::DetectorParameters>();
	}

	// cv::Ptr<cv::mcc::DetectorParameters>::delete() generated
	// ("cv::Ptr<cv::mcc::DetectorParameters>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_mcc_DetectorParametersG_delete(cv::Ptr<cv::mcc::DetectorParameters>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::mcc::DetectorParameters>::new(TraitClass) generated
	// ("cv::Ptr<cv::mcc::DetectorParameters>::new", vec![(pred!(const, ["val"], ["cv::mcc::DetectorParameters"]), _)]),
	cv::Ptr<cv::mcc::DetectorParameters>* cv_PtrLcv_mcc_DetectorParametersG_new_const_DetectorParameters(cv::mcc::DetectorParameters* val) {
			return new cv::Ptr<cv::mcc::DetectorParameters>(val);
	}

}

