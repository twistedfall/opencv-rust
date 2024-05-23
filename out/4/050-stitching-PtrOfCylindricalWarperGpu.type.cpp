extern "C" {
	// cv::Ptr<cv::CylindricalWarperGpu>::getInnerPtr() generated
	// ("cv::Ptr<cv::CylindricalWarperGpu>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::CylindricalWarperGpu* cv_PtrLcv_CylindricalWarperGpuG_getInnerPtr_const(const cv::Ptr<cv::CylindricalWarperGpu>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::CylindricalWarperGpu>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::CylindricalWarperGpu>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::CylindricalWarperGpu* cv_PtrLcv_CylindricalWarperGpuG_getInnerPtrMut(cv::Ptr<cv::CylindricalWarperGpu>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::CylindricalWarperGpu>::new_null() generated
	// ("cv::Ptr<cv::CylindricalWarperGpu>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::CylindricalWarperGpu>* cv_PtrLcv_CylindricalWarperGpuG_new_null_const() {
			return new cv::Ptr<cv::CylindricalWarperGpu>();
	}

	// cv::Ptr<cv::CylindricalWarperGpu>::delete() generated
	// ("cv::Ptr<cv::CylindricalWarperGpu>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_CylindricalWarperGpuG_delete(cv::Ptr<cv::CylindricalWarperGpu>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::CylindricalWarperGpu>::to_PtrOfWarperCreator() generated
	// ("cv::Ptr<cv::CylindricalWarperGpu>::to_PtrOfWarperCreator", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::WarperCreator>* cv_PtrLcv_CylindricalWarperGpuG_to_PtrOfWarperCreator(cv::Ptr<cv::CylindricalWarperGpu>* instance) {
			return new cv::Ptr<cv::WarperCreator>(instance->dynamicCast<cv::WarperCreator>());
	}

	// cv::Ptr<cv::CylindricalWarperGpu>::new(TraitClass) generated
	// ("cv::Ptr<cv::CylindricalWarperGpu>::new", vec![(pred!(const, ["val"], ["cv::CylindricalWarperGpu"]), _)]),
	cv::Ptr<cv::CylindricalWarperGpu>* cv_PtrLcv_CylindricalWarperGpuG_new_const_CylindricalWarperGpu(cv::CylindricalWarperGpu* val) {
			return new cv::Ptr<cv::CylindricalWarperGpu>(val);
	}

}

