extern "C" {
	// cv::Ptr<cv::PlaneWarperGpu>::getInnerPtr() generated
	// ("cv::Ptr<cv::PlaneWarperGpu>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::PlaneWarperGpu* cv_PtrLcv_PlaneWarperGpuG_getInnerPtr_const(const cv::Ptr<cv::PlaneWarperGpu>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::PlaneWarperGpu>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::PlaneWarperGpu>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::PlaneWarperGpu* cv_PtrLcv_PlaneWarperGpuG_getInnerPtrMut(cv::Ptr<cv::PlaneWarperGpu>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::PlaneWarperGpu>::new_null() generated
	// ("cv::Ptr<cv::PlaneWarperGpu>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::PlaneWarperGpu>* cv_PtrLcv_PlaneWarperGpuG_new_null_const() {
			return new cv::Ptr<cv::PlaneWarperGpu>();
	}

	// cv::Ptr<cv::PlaneWarperGpu>::delete() generated
	// ("cv::Ptr<cv::PlaneWarperGpu>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_PlaneWarperGpuG_delete(cv::Ptr<cv::PlaneWarperGpu>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::PlaneWarperGpu>::to_PtrOfWarperCreator() generated
	// ("cv::Ptr<cv::PlaneWarperGpu>::to_PtrOfWarperCreator", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::WarperCreator>* cv_PtrLcv_PlaneWarperGpuG_to_PtrOfWarperCreator(cv::Ptr<cv::PlaneWarperGpu>* instance) {
			return new cv::Ptr<cv::WarperCreator>(instance->dynamicCast<cv::WarperCreator>());
	}

	// cv::Ptr<cv::PlaneWarperGpu>::new(TraitClass) generated
	// ("cv::Ptr<cv::PlaneWarperGpu>::new", vec![(pred!(const, ["val"], ["cv::PlaneWarperGpu"]), _)]),
	cv::Ptr<cv::PlaneWarperGpu>* cv_PtrLcv_PlaneWarperGpuG_new_const_PlaneWarperGpu(cv::PlaneWarperGpu* val) {
			return new cv::Ptr<cv::PlaneWarperGpu>(val);
	}

}

