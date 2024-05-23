extern "C" {
	// cv::Ptr<cv::cuda::ORB>::getInnerPtr() generated
	// ("cv::Ptr<cv::cuda::ORB>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::cuda::ORB* cv_PtrLcv_cuda_ORBG_getInnerPtr_const(const cv::Ptr<cv::cuda::ORB>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::ORB>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::cuda::ORB>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::cuda::ORB* cv_PtrLcv_cuda_ORBG_getInnerPtrMut(cv::Ptr<cv::cuda::ORB>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::ORB>::new_null() generated
	// ("cv::Ptr<cv::cuda::ORB>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::cuda::ORB>* cv_PtrLcv_cuda_ORBG_new_null_const() {
			return new cv::Ptr<cv::cuda::ORB>();
	}

	// cv::Ptr<cv::cuda::ORB>::delete() generated
	// ("cv::Ptr<cv::cuda::ORB>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_cuda_ORBG_delete(cv::Ptr<cv::cuda::ORB>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::cuda::ORB>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::cuda::ORB>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_cuda_ORBG_to_PtrOfAlgorithm(cv::Ptr<cv::cuda::ORB>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::cuda::ORB>::to_PtrOfFeature2D() generated
	// ("cv::Ptr<cv::cuda::ORB>::to_PtrOfFeature2D", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Feature2D>* cv_PtrLcv_cuda_ORBG_to_PtrOfFeature2D(cv::Ptr<cv::cuda::ORB>* instance) {
			return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}

	// cv::Ptr<cv::cuda::ORB>::to_PtrOfCUDA_Feature2DAsync() generated
	// ("cv::Ptr<cv::cuda::ORB>::to_PtrOfCUDA_Feature2DAsync", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::cuda::Feature2DAsync>* cv_PtrLcv_cuda_ORBG_to_PtrOfCUDA_Feature2DAsync(cv::Ptr<cv::cuda::ORB>* instance) {
			return new cv::Ptr<cv::cuda::Feature2DAsync>(instance->dynamicCast<cv::cuda::Feature2DAsync>());
	}

}

