extern "C" {
	// cv::Ptr<cv::structured_light::StructuredLightPattern>::getInnerPtr() generated
	// ("cv::Ptr<cv::structured_light::StructuredLightPattern>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::structured_light::StructuredLightPattern* cv_PtrLcv_structured_light_StructuredLightPatternG_getInnerPtr_const(const cv::Ptr<cv::structured_light::StructuredLightPattern>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::structured_light::StructuredLightPattern>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::structured_light::StructuredLightPattern>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::structured_light::StructuredLightPattern* cv_PtrLcv_structured_light_StructuredLightPatternG_getInnerPtrMut(cv::Ptr<cv::structured_light::StructuredLightPattern>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::structured_light::StructuredLightPattern>::new_null() generated
	// ("cv::Ptr<cv::structured_light::StructuredLightPattern>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::structured_light::StructuredLightPattern>* cv_PtrLcv_structured_light_StructuredLightPatternG_new_null_const() {
			return new cv::Ptr<cv::structured_light::StructuredLightPattern>();
	}

	// cv::Ptr<cv::structured_light::StructuredLightPattern>::delete() generated
	// ("cv::Ptr<cv::structured_light::StructuredLightPattern>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_structured_light_StructuredLightPatternG_delete(cv::Ptr<cv::structured_light::StructuredLightPattern>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::structured_light::StructuredLightPattern>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::structured_light::StructuredLightPattern>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_structured_light_StructuredLightPatternG_to_PtrOfAlgorithm(cv::Ptr<cv::structured_light::StructuredLightPattern>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

