extern "C" {
	// cv::Ptr<cv::xfeatures2d::PCTSignatures>::getInnerPtr() generated
	// ("cv::Ptr<cv::xfeatures2d::PCTSignatures>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::xfeatures2d::PCTSignatures* cv_PtrLcv_xfeatures2d_PCTSignaturesG_getInnerPtr_const(const cv::Ptr<cv::xfeatures2d::PCTSignatures>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::xfeatures2d::PCTSignatures>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::xfeatures2d::PCTSignatures>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::xfeatures2d::PCTSignatures* cv_PtrLcv_xfeatures2d_PCTSignaturesG_getInnerPtrMut(cv::Ptr<cv::xfeatures2d::PCTSignatures>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::xfeatures2d::PCTSignatures>::new_null() generated
	// ("cv::Ptr<cv::xfeatures2d::PCTSignatures>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::xfeatures2d::PCTSignatures>* cv_PtrLcv_xfeatures2d_PCTSignaturesG_new_null_const() {
			return new cv::Ptr<cv::xfeatures2d::PCTSignatures>();
	}

	// cv::Ptr<cv::xfeatures2d::PCTSignatures>::delete() generated
	// ("cv::Ptr<cv::xfeatures2d::PCTSignatures>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_xfeatures2d_PCTSignaturesG_delete(cv::Ptr<cv::xfeatures2d::PCTSignatures>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::xfeatures2d::PCTSignatures>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::xfeatures2d::PCTSignatures>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_xfeatures2d_PCTSignaturesG_to_PtrOfAlgorithm(cv::Ptr<cv::xfeatures2d::PCTSignatures>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

