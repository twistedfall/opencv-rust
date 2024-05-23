extern "C" {
	// cv::Ptr<cv::ShapeTransformer>::getInnerPtr() generated
	// ("cv::Ptr<cv::ShapeTransformer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::ShapeTransformer* cv_PtrLcv_ShapeTransformerG_getInnerPtr_const(const cv::Ptr<cv::ShapeTransformer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ShapeTransformer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::ShapeTransformer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::ShapeTransformer* cv_PtrLcv_ShapeTransformerG_getInnerPtrMut(cv::Ptr<cv::ShapeTransformer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ShapeTransformer>::new_null() generated
	// ("cv::Ptr<cv::ShapeTransformer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::ShapeTransformer>* cv_PtrLcv_ShapeTransformerG_new_null_const() {
			return new cv::Ptr<cv::ShapeTransformer>();
	}

	// cv::Ptr<cv::ShapeTransformer>::delete() generated
	// ("cv::Ptr<cv::ShapeTransformer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_ShapeTransformerG_delete(cv::Ptr<cv::ShapeTransformer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::ShapeTransformer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::ShapeTransformer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ShapeTransformerG_to_PtrOfAlgorithm(cv::Ptr<cv::ShapeTransformer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

