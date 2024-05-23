extern "C" {
	// cv::Ptr<cv::TonemapDrago>::getInnerPtr() generated
	// ("cv::Ptr<cv::TonemapDrago>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::TonemapDrago* cv_PtrLcv_TonemapDragoG_getInnerPtr_const(const cv::Ptr<cv::TonemapDrago>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::TonemapDrago>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::TonemapDrago>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::TonemapDrago* cv_PtrLcv_TonemapDragoG_getInnerPtrMut(cv::Ptr<cv::TonemapDrago>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::TonemapDrago>::new_null() generated
	// ("cv::Ptr<cv::TonemapDrago>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::TonemapDrago>* cv_PtrLcv_TonemapDragoG_new_null_const() {
			return new cv::Ptr<cv::TonemapDrago>();
	}

	// cv::Ptr<cv::TonemapDrago>::delete() generated
	// ("cv::Ptr<cv::TonemapDrago>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_TonemapDragoG_delete(cv::Ptr<cv::TonemapDrago>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::TonemapDrago>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::TonemapDrago>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_TonemapDragoG_to_PtrOfAlgorithm(cv::Ptr<cv::TonemapDrago>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::TonemapDrago>::to_PtrOfTonemap() generated
	// ("cv::Ptr<cv::TonemapDrago>::to_PtrOfTonemap", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Tonemap>* cv_PtrLcv_TonemapDragoG_to_PtrOfTonemap(cv::Ptr<cv::TonemapDrago>* instance) {
			return new cv::Ptr<cv::Tonemap>(instance->dynamicCast<cv::Tonemap>());
	}

}

