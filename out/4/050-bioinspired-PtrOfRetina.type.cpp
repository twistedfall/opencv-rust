extern "C" {
	// cv::Ptr<cv::bioinspired::Retina>::getInnerPtr() generated
	// ("cv::Ptr<cv::bioinspired::Retina>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::bioinspired::Retina* cv_PtrLcv_bioinspired_RetinaG_getInnerPtr_const(const cv::Ptr<cv::bioinspired::Retina>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::bioinspired::Retina>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::bioinspired::Retina>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::bioinspired::Retina* cv_PtrLcv_bioinspired_RetinaG_getInnerPtrMut(cv::Ptr<cv::bioinspired::Retina>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::bioinspired::Retina>::new_null() generated
	// ("cv::Ptr<cv::bioinspired::Retina>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::bioinspired::Retina>* cv_PtrLcv_bioinspired_RetinaG_new_null_const() {
			return new cv::Ptr<cv::bioinspired::Retina>();
	}

	// cv::Ptr<cv::bioinspired::Retina>::delete() generated
	// ("cv::Ptr<cv::bioinspired::Retina>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_bioinspired_RetinaG_delete(cv::Ptr<cv::bioinspired::Retina>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::bioinspired::Retina>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::bioinspired::Retina>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_bioinspired_RetinaG_to_PtrOfAlgorithm(cv::Ptr<cv::bioinspired::Retina>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

