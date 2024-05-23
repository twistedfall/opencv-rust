extern "C" {
	// cv::Ptr<cv::structured_light::GrayCodePattern>::getInnerPtr() generated
	// ("cv::Ptr<cv::structured_light::GrayCodePattern>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::structured_light::GrayCodePattern* cv_PtrLcv_structured_light_GrayCodePatternG_getInnerPtr_const(const cv::Ptr<cv::structured_light::GrayCodePattern>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::structured_light::GrayCodePattern>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::structured_light::GrayCodePattern>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::structured_light::GrayCodePattern* cv_PtrLcv_structured_light_GrayCodePatternG_getInnerPtrMut(cv::Ptr<cv::structured_light::GrayCodePattern>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::structured_light::GrayCodePattern>::new_null() generated
	// ("cv::Ptr<cv::structured_light::GrayCodePattern>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::structured_light::GrayCodePattern>* cv_PtrLcv_structured_light_GrayCodePatternG_new_null_const() {
			return new cv::Ptr<cv::structured_light::GrayCodePattern>();
	}

	// cv::Ptr<cv::structured_light::GrayCodePattern>::delete() generated
	// ("cv::Ptr<cv::structured_light::GrayCodePattern>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_structured_light_GrayCodePatternG_delete(cv::Ptr<cv::structured_light::GrayCodePattern>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::structured_light::GrayCodePattern>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::structured_light::GrayCodePattern>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_structured_light_GrayCodePatternG_to_PtrOfAlgorithm(cv::Ptr<cv::structured_light::GrayCodePattern>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::structured_light::GrayCodePattern>::to_PtrOfStructuredLightPattern() generated
	// ("cv::Ptr<cv::structured_light::GrayCodePattern>::to_PtrOfStructuredLightPattern", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::structured_light::StructuredLightPattern>* cv_PtrLcv_structured_light_GrayCodePatternG_to_PtrOfStructuredLightPattern(cv::Ptr<cv::structured_light::GrayCodePattern>* instance) {
			return new cv::Ptr<cv::structured_light::StructuredLightPattern>(instance->dynamicCast<cv::structured_light::StructuredLightPattern>());
	}

}

