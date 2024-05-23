extern "C" {
	// cv::Ptr<cv::structured_light::SinusoidalPattern>::getInnerPtr() generated
	// ("cv::Ptr<cv::structured_light::SinusoidalPattern>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::structured_light::SinusoidalPattern* cv_PtrLcv_structured_light_SinusoidalPatternG_getInnerPtr_const(const cv::Ptr<cv::structured_light::SinusoidalPattern>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::structured_light::SinusoidalPattern>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::structured_light::SinusoidalPattern>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::structured_light::SinusoidalPattern* cv_PtrLcv_structured_light_SinusoidalPatternG_getInnerPtrMut(cv::Ptr<cv::structured_light::SinusoidalPattern>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::structured_light::SinusoidalPattern>::new_null() generated
	// ("cv::Ptr<cv::structured_light::SinusoidalPattern>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::structured_light::SinusoidalPattern>* cv_PtrLcv_structured_light_SinusoidalPatternG_new_null_const() {
			return new cv::Ptr<cv::structured_light::SinusoidalPattern>();
	}

	// cv::Ptr<cv::structured_light::SinusoidalPattern>::delete() generated
	// ("cv::Ptr<cv::structured_light::SinusoidalPattern>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_structured_light_SinusoidalPatternG_delete(cv::Ptr<cv::structured_light::SinusoidalPattern>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::structured_light::SinusoidalPattern>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::structured_light::SinusoidalPattern>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_structured_light_SinusoidalPatternG_to_PtrOfAlgorithm(cv::Ptr<cv::structured_light::SinusoidalPattern>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::structured_light::SinusoidalPattern>::to_PtrOfStructuredLightPattern() generated
	// ("cv::Ptr<cv::structured_light::SinusoidalPattern>::to_PtrOfStructuredLightPattern", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::structured_light::StructuredLightPattern>* cv_PtrLcv_structured_light_SinusoidalPatternG_to_PtrOfStructuredLightPattern(cv::Ptr<cv::structured_light::SinusoidalPattern>* instance) {
			return new cv::Ptr<cv::structured_light::StructuredLightPattern>(instance->dynamicCast<cv::structured_light::StructuredLightPattern>());
	}

}

