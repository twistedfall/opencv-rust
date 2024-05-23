extern "C" {
	// cv::Ptr<cv::CalibrateDebevec>::getInnerPtr() generated
	// ("cv::Ptr<cv::CalibrateDebevec>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::CalibrateDebevec* cv_PtrLcv_CalibrateDebevecG_getInnerPtr_const(const cv::Ptr<cv::CalibrateDebevec>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::CalibrateDebevec>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::CalibrateDebevec>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::CalibrateDebevec* cv_PtrLcv_CalibrateDebevecG_getInnerPtrMut(cv::Ptr<cv::CalibrateDebevec>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::CalibrateDebevec>::new_null() generated
	// ("cv::Ptr<cv::CalibrateDebevec>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::CalibrateDebevec>* cv_PtrLcv_CalibrateDebevecG_new_null_const() {
			return new cv::Ptr<cv::CalibrateDebevec>();
	}

	// cv::Ptr<cv::CalibrateDebevec>::delete() generated
	// ("cv::Ptr<cv::CalibrateDebevec>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_CalibrateDebevecG_delete(cv::Ptr<cv::CalibrateDebevec>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::CalibrateDebevec>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::CalibrateDebevec>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_CalibrateDebevecG_to_PtrOfAlgorithm(cv::Ptr<cv::CalibrateDebevec>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::CalibrateDebevec>::to_PtrOfCalibrateCRF() generated
	// ("cv::Ptr<cv::CalibrateDebevec>::to_PtrOfCalibrateCRF", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::CalibrateCRF>* cv_PtrLcv_CalibrateDebevecG_to_PtrOfCalibrateCRF(cv::Ptr<cv::CalibrateDebevec>* instance) {
			return new cv::Ptr<cv::CalibrateCRF>(instance->dynamicCast<cv::CalibrateCRF>());
	}

}

