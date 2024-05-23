extern "C" {
	// cv::Ptr<cv::bioinspired::RetinaFastToneMapping>::getInnerPtr() generated
	// ("cv::Ptr<cv::bioinspired::RetinaFastToneMapping>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::bioinspired::RetinaFastToneMapping* cv_PtrLcv_bioinspired_RetinaFastToneMappingG_getInnerPtr_const(const cv::Ptr<cv::bioinspired::RetinaFastToneMapping>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::bioinspired::RetinaFastToneMapping>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::bioinspired::RetinaFastToneMapping>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::bioinspired::RetinaFastToneMapping* cv_PtrLcv_bioinspired_RetinaFastToneMappingG_getInnerPtrMut(cv::Ptr<cv::bioinspired::RetinaFastToneMapping>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::bioinspired::RetinaFastToneMapping>::new_null() generated
	// ("cv::Ptr<cv::bioinspired::RetinaFastToneMapping>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::bioinspired::RetinaFastToneMapping>* cv_PtrLcv_bioinspired_RetinaFastToneMappingG_new_null_const() {
			return new cv::Ptr<cv::bioinspired::RetinaFastToneMapping>();
	}

	// cv::Ptr<cv::bioinspired::RetinaFastToneMapping>::delete() generated
	// ("cv::Ptr<cv::bioinspired::RetinaFastToneMapping>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_bioinspired_RetinaFastToneMappingG_delete(cv::Ptr<cv::bioinspired::RetinaFastToneMapping>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::bioinspired::RetinaFastToneMapping>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::bioinspired::RetinaFastToneMapping>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_bioinspired_RetinaFastToneMappingG_to_PtrOfAlgorithm(cv::Ptr<cv::bioinspired::RetinaFastToneMapping>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

