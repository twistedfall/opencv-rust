extern "C" {
	// cv::Ptr<cv::AffineTransformer>::getInnerPtr() generated
	// ("cv::Ptr<cv::AffineTransformer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::AffineTransformer* cv_PtrLcv_AffineTransformerG_getInnerPtr_const(const cv::Ptr<cv::AffineTransformer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::AffineTransformer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::AffineTransformer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::AffineTransformer* cv_PtrLcv_AffineTransformerG_getInnerPtrMut(cv::Ptr<cv::AffineTransformer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::AffineTransformer>::new_null() generated
	// ("cv::Ptr<cv::AffineTransformer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::AffineTransformer>* cv_PtrLcv_AffineTransformerG_new_null_const() {
			return new cv::Ptr<cv::AffineTransformer>();
	}

	// cv::Ptr<cv::AffineTransformer>::delete() generated
	// ("cv::Ptr<cv::AffineTransformer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_AffineTransformerG_delete(cv::Ptr<cv::AffineTransformer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::AffineTransformer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::AffineTransformer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_AffineTransformerG_to_PtrOfAlgorithm(cv::Ptr<cv::AffineTransformer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::AffineTransformer>::to_PtrOfShapeTransformer() generated
	// ("cv::Ptr<cv::AffineTransformer>::to_PtrOfShapeTransformer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::ShapeTransformer>* cv_PtrLcv_AffineTransformerG_to_PtrOfShapeTransformer(cv::Ptr<cv::AffineTransformer>* instance) {
			return new cv::Ptr<cv::ShapeTransformer>(instance->dynamicCast<cv::ShapeTransformer>());
	}

}

