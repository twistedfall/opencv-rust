extern "C" {
	// cv::Ptr<cv::quality::QualityMSE>::getInnerPtr() generated
	// ("cv::Ptr<cv::quality::QualityMSE>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::quality::QualityMSE* cv_PtrLcv_quality_QualityMSEG_getInnerPtr_const(const cv::Ptr<cv::quality::QualityMSE>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::quality::QualityMSE>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::quality::QualityMSE>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::quality::QualityMSE* cv_PtrLcv_quality_QualityMSEG_getInnerPtrMut(cv::Ptr<cv::quality::QualityMSE>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::quality::QualityMSE>::new_null() generated
	// ("cv::Ptr<cv::quality::QualityMSE>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::quality::QualityMSE>* cv_PtrLcv_quality_QualityMSEG_new_null_const() {
			return new cv::Ptr<cv::quality::QualityMSE>();
	}

	// cv::Ptr<cv::quality::QualityMSE>::delete() generated
	// ("cv::Ptr<cv::quality::QualityMSE>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_quality_QualityMSEG_delete(cv::Ptr<cv::quality::QualityMSE>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::quality::QualityMSE>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::quality::QualityMSE>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_quality_QualityMSEG_to_PtrOfAlgorithm(cv::Ptr<cv::quality::QualityMSE>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::quality::QualityMSE>::to_PtrOfQualityBase() generated
	// ("cv::Ptr<cv::quality::QualityMSE>::to_PtrOfQualityBase", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::quality::QualityBase>* cv_PtrLcv_quality_QualityMSEG_to_PtrOfQualityBase(cv::Ptr<cv::quality::QualityMSE>* instance) {
			return new cv::Ptr<cv::quality::QualityBase>(instance->dynamicCast<cv::quality::QualityBase>());
	}

	// cv::Ptr<cv::quality::QualityMSE>::new(TraitClass) generated
	// ("cv::Ptr<cv::quality::QualityMSE>::new", vec![(pred!(const, ["val"], ["cv::quality::QualityMSE"]), _)]),
	cv::Ptr<cv::quality::QualityMSE>* cv_PtrLcv_quality_QualityMSEG_new_const_QualityMSE(cv::quality::QualityMSE* val) {
			return new cv::Ptr<cv::quality::QualityMSE>(val);
	}

}

