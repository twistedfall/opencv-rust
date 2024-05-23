extern "C" {
	// cv::Ptr<cv::xfeatures2d::BriefDescriptorExtractor>::getInnerPtr() generated
	// ("cv::Ptr<cv::xfeatures2d::BriefDescriptorExtractor>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::xfeatures2d::BriefDescriptorExtractor* cv_PtrLcv_xfeatures2d_BriefDescriptorExtractorG_getInnerPtr_const(const cv::Ptr<cv::xfeatures2d::BriefDescriptorExtractor>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::xfeatures2d::BriefDescriptorExtractor>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::xfeatures2d::BriefDescriptorExtractor>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::xfeatures2d::BriefDescriptorExtractor* cv_PtrLcv_xfeatures2d_BriefDescriptorExtractorG_getInnerPtrMut(cv::Ptr<cv::xfeatures2d::BriefDescriptorExtractor>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::xfeatures2d::BriefDescriptorExtractor>::new_null() generated
	// ("cv::Ptr<cv::xfeatures2d::BriefDescriptorExtractor>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::xfeatures2d::BriefDescriptorExtractor>* cv_PtrLcv_xfeatures2d_BriefDescriptorExtractorG_new_null_const() {
			return new cv::Ptr<cv::xfeatures2d::BriefDescriptorExtractor>();
	}

	// cv::Ptr<cv::xfeatures2d::BriefDescriptorExtractor>::delete() generated
	// ("cv::Ptr<cv::xfeatures2d::BriefDescriptorExtractor>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_xfeatures2d_BriefDescriptorExtractorG_delete(cv::Ptr<cv::xfeatures2d::BriefDescriptorExtractor>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::xfeatures2d::BriefDescriptorExtractor>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::xfeatures2d::BriefDescriptorExtractor>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_xfeatures2d_BriefDescriptorExtractorG_to_PtrOfAlgorithm(cv::Ptr<cv::xfeatures2d::BriefDescriptorExtractor>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::xfeatures2d::BriefDescriptorExtractor>::to_PtrOfFeature2D() generated
	// ("cv::Ptr<cv::xfeatures2d::BriefDescriptorExtractor>::to_PtrOfFeature2D", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Feature2D>* cv_PtrLcv_xfeatures2d_BriefDescriptorExtractorG_to_PtrOfFeature2D(cv::Ptr<cv::xfeatures2d::BriefDescriptorExtractor>* instance) {
			return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}

	// cv::Ptr<cv::xfeatures2d::BriefDescriptorExtractor>::new(TraitClass) generated
	// ("cv::Ptr<cv::xfeatures2d::BriefDescriptorExtractor>::new", vec![(pred!(const, ["val"], ["cv::xfeatures2d::BriefDescriptorExtractor"]), _)]),
	cv::Ptr<cv::xfeatures2d::BriefDescriptorExtractor>* cv_PtrLcv_xfeatures2d_BriefDescriptorExtractorG_new_const_BriefDescriptorExtractor(cv::xfeatures2d::BriefDescriptorExtractor* val) {
			return new cv::Ptr<cv::xfeatures2d::BriefDescriptorExtractor>(val);
	}

}

