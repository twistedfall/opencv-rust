extern "C" {
	// cv::Ptr<cv::SphericalWarperGpu>::getInnerPtr() generated
	// ("cv::Ptr<cv::SphericalWarperGpu>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::SphericalWarperGpu* cv_PtrLcv_SphericalWarperGpuG_getInnerPtr_const(const cv::Ptr<cv::SphericalWarperGpu>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::SphericalWarperGpu>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::SphericalWarperGpu>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::SphericalWarperGpu* cv_PtrLcv_SphericalWarperGpuG_getInnerPtrMut(cv::Ptr<cv::SphericalWarperGpu>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::SphericalWarperGpu>::new_null() generated
	// ("cv::Ptr<cv::SphericalWarperGpu>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::SphericalWarperGpu>* cv_PtrLcv_SphericalWarperGpuG_new_null_const() {
			return new cv::Ptr<cv::SphericalWarperGpu>();
	}

	// cv::Ptr<cv::SphericalWarperGpu>::delete() generated
	// ("cv::Ptr<cv::SphericalWarperGpu>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_SphericalWarperGpuG_delete(cv::Ptr<cv::SphericalWarperGpu>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::SphericalWarperGpu>::to_PtrOfWarperCreator() generated
	// ("cv::Ptr<cv::SphericalWarperGpu>::to_PtrOfWarperCreator", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::WarperCreator>* cv_PtrLcv_SphericalWarperGpuG_to_PtrOfWarperCreator(cv::Ptr<cv::SphericalWarperGpu>* instance) {
			return new cv::Ptr<cv::WarperCreator>(instance->dynamicCast<cv::WarperCreator>());
	}

	// cv::Ptr<cv::SphericalWarperGpu>::new(TraitClass) generated
	// ("cv::Ptr<cv::SphericalWarperGpu>::new", vec![(pred!(const, ["val"], ["cv::SphericalWarperGpu"]), _)]),
	cv::Ptr<cv::SphericalWarperGpu>* cv_PtrLcv_SphericalWarperGpuG_new_const_SphericalWarperGpu(cv::SphericalWarperGpu* val) {
			return new cv::Ptr<cv::SphericalWarperGpu>(val);
	}

}

