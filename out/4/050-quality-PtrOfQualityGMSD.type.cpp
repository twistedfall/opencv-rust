extern "C" {
	// cv::Ptr<cv::quality::QualityGMSD>::getInnerPtr() generated
	// ("cv::Ptr<cv::quality::QualityGMSD>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::quality::QualityGMSD* cv_PtrLcv_quality_QualityGMSDG_getInnerPtr_const(const cv::Ptr<cv::quality::QualityGMSD>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::quality::QualityGMSD>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::quality::QualityGMSD>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::quality::QualityGMSD* cv_PtrLcv_quality_QualityGMSDG_getInnerPtrMut(cv::Ptr<cv::quality::QualityGMSD>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::quality::QualityGMSD>::new_null() generated
	// ("cv::Ptr<cv::quality::QualityGMSD>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::quality::QualityGMSD>* cv_PtrLcv_quality_QualityGMSDG_new_null_const() {
			return new cv::Ptr<cv::quality::QualityGMSD>();
	}

	// cv::Ptr<cv::quality::QualityGMSD>::delete() generated
	// ("cv::Ptr<cv::quality::QualityGMSD>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_quality_QualityGMSDG_delete(cv::Ptr<cv::quality::QualityGMSD>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::quality::QualityGMSD>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::quality::QualityGMSD>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_quality_QualityGMSDG_to_PtrOfAlgorithm(cv::Ptr<cv::quality::QualityGMSD>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::quality::QualityGMSD>::to_PtrOfQualityBase() generated
	// ("cv::Ptr<cv::quality::QualityGMSD>::to_PtrOfQualityBase", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::quality::QualityBase>* cv_PtrLcv_quality_QualityGMSDG_to_PtrOfQualityBase(cv::Ptr<cv::quality::QualityGMSD>* instance) {
			return new cv::Ptr<cv::quality::QualityBase>(instance->dynamicCast<cv::quality::QualityBase>());
	}

	// cv::Ptr<cv::quality::QualityGMSD>::new(TraitClass) generated
	// ("cv::Ptr<cv::quality::QualityGMSD>::new", vec![(pred!(const, ["val"], ["cv::quality::QualityGMSD"]), _)]),
	cv::Ptr<cv::quality::QualityGMSD>* cv_PtrLcv_quality_QualityGMSDG_new_const_QualityGMSD(cv::quality::QualityGMSD* val) {
			return new cv::Ptr<cv::quality::QualityGMSD>(val);
	}

}

