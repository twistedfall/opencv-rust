extern "C" {
	// cv::Ptr<cv::quality::QualitySSIM>::getInnerPtr() generated
	// ("cv::Ptr<cv::quality::QualitySSIM>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::quality::QualitySSIM* cv_PtrLcv_quality_QualitySSIMG_getInnerPtr_const(const cv::Ptr<cv::quality::QualitySSIM>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::quality::QualitySSIM>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::quality::QualitySSIM>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::quality::QualitySSIM* cv_PtrLcv_quality_QualitySSIMG_getInnerPtrMut(cv::Ptr<cv::quality::QualitySSIM>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::quality::QualitySSIM>::new_null() generated
	// ("cv::Ptr<cv::quality::QualitySSIM>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::quality::QualitySSIM>* cv_PtrLcv_quality_QualitySSIMG_new_null_const() {
			return new cv::Ptr<cv::quality::QualitySSIM>();
	}

	// cv::Ptr<cv::quality::QualitySSIM>::delete() generated
	// ("cv::Ptr<cv::quality::QualitySSIM>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_quality_QualitySSIMG_delete(cv::Ptr<cv::quality::QualitySSIM>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::quality::QualitySSIM>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::quality::QualitySSIM>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_quality_QualitySSIMG_to_PtrOfAlgorithm(cv::Ptr<cv::quality::QualitySSIM>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::quality::QualitySSIM>::to_PtrOfQualityBase() generated
	// ("cv::Ptr<cv::quality::QualitySSIM>::to_PtrOfQualityBase", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::quality::QualityBase>* cv_PtrLcv_quality_QualitySSIMG_to_PtrOfQualityBase(cv::Ptr<cv::quality::QualitySSIM>* instance) {
			return new cv::Ptr<cv::quality::QualityBase>(instance->dynamicCast<cv::quality::QualityBase>());
	}

	// cv::Ptr<cv::quality::QualitySSIM>::new(TraitClass) generated
	// ("cv::Ptr<cv::quality::QualitySSIM>::new", vec![(pred!(const, ["val"], ["cv::quality::QualitySSIM"]), _)]),
	cv::Ptr<cv::quality::QualitySSIM>* cv_PtrLcv_quality_QualitySSIMG_new_const_QualitySSIM(cv::quality::QualitySSIM* val) {
			return new cv::Ptr<cv::quality::QualitySSIM>(val);
	}

}

