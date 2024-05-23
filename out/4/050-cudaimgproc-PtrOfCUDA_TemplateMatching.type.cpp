extern "C" {
	// cv::Ptr<cv::cuda::TemplateMatching>::getInnerPtr() generated
	// ("cv::Ptr<cv::cuda::TemplateMatching>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::cuda::TemplateMatching* cv_PtrLcv_cuda_TemplateMatchingG_getInnerPtr_const(const cv::Ptr<cv::cuda::TemplateMatching>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::TemplateMatching>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::cuda::TemplateMatching>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::cuda::TemplateMatching* cv_PtrLcv_cuda_TemplateMatchingG_getInnerPtrMut(cv::Ptr<cv::cuda::TemplateMatching>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::TemplateMatching>::new_null() generated
	// ("cv::Ptr<cv::cuda::TemplateMatching>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::cuda::TemplateMatching>* cv_PtrLcv_cuda_TemplateMatchingG_new_null_const() {
			return new cv::Ptr<cv::cuda::TemplateMatching>();
	}

	// cv::Ptr<cv::cuda::TemplateMatching>::delete() generated
	// ("cv::Ptr<cv::cuda::TemplateMatching>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_cuda_TemplateMatchingG_delete(cv::Ptr<cv::cuda::TemplateMatching>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::cuda::TemplateMatching>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::cuda::TemplateMatching>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_cuda_TemplateMatchingG_to_PtrOfAlgorithm(cv::Ptr<cv::cuda::TemplateMatching>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

