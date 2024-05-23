extern "C" {
	// cv::Ptr<cv::ThinPlateSplineShapeTransformer>::getInnerPtr() generated
	// ("cv::Ptr<cv::ThinPlateSplineShapeTransformer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::ThinPlateSplineShapeTransformer* cv_PtrLcv_ThinPlateSplineShapeTransformerG_getInnerPtr_const(const cv::Ptr<cv::ThinPlateSplineShapeTransformer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ThinPlateSplineShapeTransformer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::ThinPlateSplineShapeTransformer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::ThinPlateSplineShapeTransformer* cv_PtrLcv_ThinPlateSplineShapeTransformerG_getInnerPtrMut(cv::Ptr<cv::ThinPlateSplineShapeTransformer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ThinPlateSplineShapeTransformer>::new_null() generated
	// ("cv::Ptr<cv::ThinPlateSplineShapeTransformer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::ThinPlateSplineShapeTransformer>* cv_PtrLcv_ThinPlateSplineShapeTransformerG_new_null_const() {
			return new cv::Ptr<cv::ThinPlateSplineShapeTransformer>();
	}

	// cv::Ptr<cv::ThinPlateSplineShapeTransformer>::delete() generated
	// ("cv::Ptr<cv::ThinPlateSplineShapeTransformer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_ThinPlateSplineShapeTransformerG_delete(cv::Ptr<cv::ThinPlateSplineShapeTransformer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::ThinPlateSplineShapeTransformer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::ThinPlateSplineShapeTransformer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ThinPlateSplineShapeTransformerG_to_PtrOfAlgorithm(cv::Ptr<cv::ThinPlateSplineShapeTransformer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::ThinPlateSplineShapeTransformer>::to_PtrOfShapeTransformer() generated
	// ("cv::Ptr<cv::ThinPlateSplineShapeTransformer>::to_PtrOfShapeTransformer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::ShapeTransformer>* cv_PtrLcv_ThinPlateSplineShapeTransformerG_to_PtrOfShapeTransformer(cv::Ptr<cv::ThinPlateSplineShapeTransformer>* instance) {
			return new cv::Ptr<cv::ShapeTransformer>(instance->dynamicCast<cv::ShapeTransformer>());
	}

}

