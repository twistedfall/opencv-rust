extern "C" {
	// cv::Ptr<cv::quality::QualityBRISQUE>::getInnerPtr() generated
	// ("cv::Ptr<cv::quality::QualityBRISQUE>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::quality::QualityBRISQUE* cv_PtrLcv_quality_QualityBRISQUEG_getInnerPtr_const(const cv::Ptr<cv::quality::QualityBRISQUE>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::quality::QualityBRISQUE>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::quality::QualityBRISQUE>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::quality::QualityBRISQUE* cv_PtrLcv_quality_QualityBRISQUEG_getInnerPtrMut(cv::Ptr<cv::quality::QualityBRISQUE>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::quality::QualityBRISQUE>::new_null() generated
	// ("cv::Ptr<cv::quality::QualityBRISQUE>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::quality::QualityBRISQUE>* cv_PtrLcv_quality_QualityBRISQUEG_new_null_const() {
			return new cv::Ptr<cv::quality::QualityBRISQUE>();
	}

	// cv::Ptr<cv::quality::QualityBRISQUE>::delete() generated
	// ("cv::Ptr<cv::quality::QualityBRISQUE>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_quality_QualityBRISQUEG_delete(cv::Ptr<cv::quality::QualityBRISQUE>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::quality::QualityBRISQUE>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::quality::QualityBRISQUE>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_quality_QualityBRISQUEG_to_PtrOfAlgorithm(cv::Ptr<cv::quality::QualityBRISQUE>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::quality::QualityBRISQUE>::to_PtrOfQualityBase() generated
	// ("cv::Ptr<cv::quality::QualityBRISQUE>::to_PtrOfQualityBase", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::quality::QualityBase>* cv_PtrLcv_quality_QualityBRISQUEG_to_PtrOfQualityBase(cv::Ptr<cv::quality::QualityBRISQUE>* instance) {
			return new cv::Ptr<cv::quality::QualityBase>(instance->dynamicCast<cv::quality::QualityBase>());
	}

	// cv::Ptr<cv::quality::QualityBRISQUE>::new(TraitClass) generated
	// ("cv::Ptr<cv::quality::QualityBRISQUE>::new", vec![(pred!(const, ["val"], ["cv::quality::QualityBRISQUE"]), _)]),
	cv::Ptr<cv::quality::QualityBRISQUE>* cv_PtrLcv_quality_QualityBRISQUEG_new_const_QualityBRISQUE(cv::quality::QualityBRISQUE* val) {
			return new cv::Ptr<cv::quality::QualityBRISQUE>(val);
	}

}

