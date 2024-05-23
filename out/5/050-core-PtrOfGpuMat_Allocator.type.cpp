extern "C" {
	// cv::Ptr<cv::cuda::GpuMat::Allocator>::getInnerPtr() generated
	// ("cv::Ptr<cv::cuda::GpuMat::Allocator>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::cuda::GpuMat::Allocator* cv_PtrLcv_cuda_GpuMat_AllocatorG_getInnerPtr_const(const cv::Ptr<cv::cuda::GpuMat::Allocator>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::GpuMat::Allocator>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::cuda::GpuMat::Allocator>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::cuda::GpuMat::Allocator* cv_PtrLcv_cuda_GpuMat_AllocatorG_getInnerPtrMut(cv::Ptr<cv::cuda::GpuMat::Allocator>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::cuda::GpuMat::Allocator>::new_null() generated
	// ("cv::Ptr<cv::cuda::GpuMat::Allocator>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::cuda::GpuMat::Allocator>* cv_PtrLcv_cuda_GpuMat_AllocatorG_new_null_const() {
			return new cv::Ptr<cv::cuda::GpuMat::Allocator>();
	}

	// cv::Ptr<cv::cuda::GpuMat::Allocator>::delete() generated
	// ("cv::Ptr<cv::cuda::GpuMat::Allocator>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_cuda_GpuMat_AllocatorG_delete(cv::Ptr<cv::cuda::GpuMat::Allocator>* instance) {
			delete instance;
	}

}

