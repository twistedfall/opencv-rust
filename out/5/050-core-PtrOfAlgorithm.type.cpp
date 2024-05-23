extern "C" {
	// cv::Ptr<cv::Algorithm>::getInnerPtr() generated
	// ("cv::Ptr<cv::Algorithm>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::Algorithm* cv_PtrLcv_AlgorithmG_getInnerPtr_const(const cv::Ptr<cv::Algorithm>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::Algorithm>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::Algorithm>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_PtrLcv_AlgorithmG_getInnerPtrMut(cv::Ptr<cv::Algorithm>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::Algorithm>::new_null() generated
	// ("cv::Ptr<cv::Algorithm>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_AlgorithmG_new_null_const() {
			return new cv::Ptr<cv::Algorithm>();
	}

	// cv::Ptr<cv::Algorithm>::delete() generated
	// ("cv::Ptr<cv::Algorithm>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_AlgorithmG_delete(cv::Ptr<cv::Algorithm>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::Algorithm>::new(TraitClass) generated
	// ("cv::Ptr<cv::Algorithm>::new", vec![(pred!(const, ["val"], ["cv::Algorithm"]), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_AlgorithmG_new_const_Algorithm(cv::Algorithm* val) {
			return new cv::Ptr<cv::Algorithm>(val);
	}

}

