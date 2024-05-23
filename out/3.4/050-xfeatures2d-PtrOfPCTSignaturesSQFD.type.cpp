extern "C" {
	// cv::Ptr<cv::xfeatures2d::PCTSignaturesSQFD>::getInnerPtr() generated
	// ("cv::Ptr<cv::xfeatures2d::PCTSignaturesSQFD>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::xfeatures2d::PCTSignaturesSQFD* cv_PtrLcv_xfeatures2d_PCTSignaturesSQFDG_getInnerPtr_const(const cv::Ptr<cv::xfeatures2d::PCTSignaturesSQFD>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::xfeatures2d::PCTSignaturesSQFD>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::xfeatures2d::PCTSignaturesSQFD>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::xfeatures2d::PCTSignaturesSQFD* cv_PtrLcv_xfeatures2d_PCTSignaturesSQFDG_getInnerPtrMut(cv::Ptr<cv::xfeatures2d::PCTSignaturesSQFD>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::xfeatures2d::PCTSignaturesSQFD>::new_null() generated
	// ("cv::Ptr<cv::xfeatures2d::PCTSignaturesSQFD>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::xfeatures2d::PCTSignaturesSQFD>* cv_PtrLcv_xfeatures2d_PCTSignaturesSQFDG_new_null_const() {
			return new cv::Ptr<cv::xfeatures2d::PCTSignaturesSQFD>();
	}

	// cv::Ptr<cv::xfeatures2d::PCTSignaturesSQFD>::delete() generated
	// ("cv::Ptr<cv::xfeatures2d::PCTSignaturesSQFD>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_xfeatures2d_PCTSignaturesSQFDG_delete(cv::Ptr<cv::xfeatures2d::PCTSignaturesSQFD>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::xfeatures2d::PCTSignaturesSQFD>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::xfeatures2d::PCTSignaturesSQFD>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_xfeatures2d_PCTSignaturesSQFDG_to_PtrOfAlgorithm(cv::Ptr<cv::xfeatures2d::PCTSignaturesSQFD>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

