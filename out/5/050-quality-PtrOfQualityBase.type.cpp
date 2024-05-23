extern "C" {
	// cv::Ptr<cv::quality::QualityBase>::getInnerPtr() generated
	// ("cv::Ptr<cv::quality::QualityBase>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::quality::QualityBase* cv_PtrLcv_quality_QualityBaseG_getInnerPtr_const(const cv::Ptr<cv::quality::QualityBase>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::quality::QualityBase>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::quality::QualityBase>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::quality::QualityBase* cv_PtrLcv_quality_QualityBaseG_getInnerPtrMut(cv::Ptr<cv::quality::QualityBase>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::quality::QualityBase>::new_null() generated
	// ("cv::Ptr<cv::quality::QualityBase>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::quality::QualityBase>* cv_PtrLcv_quality_QualityBaseG_new_null_const() {
			return new cv::Ptr<cv::quality::QualityBase>();
	}

	// cv::Ptr<cv::quality::QualityBase>::delete() generated
	// ("cv::Ptr<cv::quality::QualityBase>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_quality_QualityBaseG_delete(cv::Ptr<cv::quality::QualityBase>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::quality::QualityBase>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::quality::QualityBase>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_quality_QualityBaseG_to_PtrOfAlgorithm(cv::Ptr<cv::quality::QualityBase>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

