extern "C" {
	// cv::Ptr<cv::CalibrateCRF>::getInnerPtr() generated
	// ("cv::Ptr<cv::CalibrateCRF>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::CalibrateCRF* cv_PtrLcv_CalibrateCRFG_getInnerPtr_const(const cv::Ptr<cv::CalibrateCRF>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::CalibrateCRF>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::CalibrateCRF>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::CalibrateCRF* cv_PtrLcv_CalibrateCRFG_getInnerPtrMut(cv::Ptr<cv::CalibrateCRF>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::CalibrateCRF>::new_null() generated
	// ("cv::Ptr<cv::CalibrateCRF>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::CalibrateCRF>* cv_PtrLcv_CalibrateCRFG_new_null_const() {
			return new cv::Ptr<cv::CalibrateCRF>();
	}

	// cv::Ptr<cv::CalibrateCRF>::delete() generated
	// ("cv::Ptr<cv::CalibrateCRF>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_CalibrateCRFG_delete(cv::Ptr<cv::CalibrateCRF>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::CalibrateCRF>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::CalibrateCRF>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_CalibrateCRFG_to_PtrOfAlgorithm(cv::Ptr<cv::CalibrateCRF>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

