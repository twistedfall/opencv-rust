extern "C" {
	// cv::Ptr<cv::cuda::SURF_CUDA>::getInnerPtr() generated
	// ("cv::Ptr<cv::cuda::SURF_CUDA>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::cuda::SURF_CUDA* cv_PtrLcv_cuda_SURF_CUDAG_getInnerPtr_const(const cv::Ptr<cv::cuda::SURF_CUDA>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::SURF_CUDA>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::cuda::SURF_CUDA>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::cuda::SURF_CUDA* cv_PtrLcv_cuda_SURF_CUDAG_getInnerPtrMut(cv::Ptr<cv::cuda::SURF_CUDA>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::SURF_CUDA>::new_null() generated
	// ("cv::Ptr<cv::cuda::SURF_CUDA>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::cuda::SURF_CUDA>* cv_PtrLcv_cuda_SURF_CUDAG_new_null_const() {
			return new cv::Ptr<cv::cuda::SURF_CUDA>();
	}

	// cv::Ptr<cv::cuda::SURF_CUDA>::delete() generated
	// ("cv::Ptr<cv::cuda::SURF_CUDA>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_cuda_SURF_CUDAG_delete(cv::Ptr<cv::cuda::SURF_CUDA>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::cuda::SURF_CUDA>::new(TraitClass) generated
	// ("cv::Ptr<cv::cuda::SURF_CUDA>::new", vec![(pred!(const, ["val"], ["cv::cuda::SURF_CUDA"]), _)]),
	cv::Ptr<cv::cuda::SURF_CUDA>* cv_PtrLcv_cuda_SURF_CUDAG_new_const_SURF_CUDA(cv::cuda::SURF_CUDA* val) {
			return new cv::Ptr<cv::cuda::SURF_CUDA>(val);
	}

}

