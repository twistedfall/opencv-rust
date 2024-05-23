extern "C" {
	// cv::Ptr<cv::quality::QualityPSNR>::getInnerPtr() generated
	// ("cv::Ptr<cv::quality::QualityPSNR>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::quality::QualityPSNR* cv_PtrLcv_quality_QualityPSNRG_getInnerPtr_const(const cv::Ptr<cv::quality::QualityPSNR>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::quality::QualityPSNR>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::quality::QualityPSNR>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::quality::QualityPSNR* cv_PtrLcv_quality_QualityPSNRG_getInnerPtrMut(cv::Ptr<cv::quality::QualityPSNR>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::quality::QualityPSNR>::new_null() generated
	// ("cv::Ptr<cv::quality::QualityPSNR>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::quality::QualityPSNR>* cv_PtrLcv_quality_QualityPSNRG_new_null_const() {
			return new cv::Ptr<cv::quality::QualityPSNR>();
	}

	// cv::Ptr<cv::quality::QualityPSNR>::delete() generated
	// ("cv::Ptr<cv::quality::QualityPSNR>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_quality_QualityPSNRG_delete(cv::Ptr<cv::quality::QualityPSNR>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::quality::QualityPSNR>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::quality::QualityPSNR>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_quality_QualityPSNRG_to_PtrOfAlgorithm(cv::Ptr<cv::quality::QualityPSNR>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::quality::QualityPSNR>::to_PtrOfQualityBase() generated
	// ("cv::Ptr<cv::quality::QualityPSNR>::to_PtrOfQualityBase", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::quality::QualityBase>* cv_PtrLcv_quality_QualityPSNRG_to_PtrOfQualityBase(cv::Ptr<cv::quality::QualityPSNR>* instance) {
			return new cv::Ptr<cv::quality::QualityBase>(instance->dynamicCast<cv::quality::QualityBase>());
	}

	// cv::Ptr<cv::quality::QualityPSNR>::new(TraitClass) generated
	// ("cv::Ptr<cv::quality::QualityPSNR>::new", vec![(pred!(const, ["val"], ["cv::quality::QualityPSNR"]), _)]),
	cv::Ptr<cv::quality::QualityPSNR>* cv_PtrLcv_quality_QualityPSNRG_new_const_QualityPSNR(cv::quality::QualityPSNR* val) {
			return new cv::Ptr<cv::quality::QualityPSNR>(val);
	}

}

