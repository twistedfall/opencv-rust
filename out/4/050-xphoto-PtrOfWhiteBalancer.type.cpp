extern "C" {
	// cv::Ptr<cv::xphoto::WhiteBalancer>::getInnerPtr() generated
	// ("cv::Ptr<cv::xphoto::WhiteBalancer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::xphoto::WhiteBalancer* cv_PtrLcv_xphoto_WhiteBalancerG_getInnerPtr_const(const cv::Ptr<cv::xphoto::WhiteBalancer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::xphoto::WhiteBalancer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::xphoto::WhiteBalancer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::xphoto::WhiteBalancer* cv_PtrLcv_xphoto_WhiteBalancerG_getInnerPtrMut(cv::Ptr<cv::xphoto::WhiteBalancer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::xphoto::WhiteBalancer>::new_null() generated
	// ("cv::Ptr<cv::xphoto::WhiteBalancer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::xphoto::WhiteBalancer>* cv_PtrLcv_xphoto_WhiteBalancerG_new_null_const() {
			return new cv::Ptr<cv::xphoto::WhiteBalancer>();
	}

	// cv::Ptr<cv::xphoto::WhiteBalancer>::delete() generated
	// ("cv::Ptr<cv::xphoto::WhiteBalancer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_xphoto_WhiteBalancerG_delete(cv::Ptr<cv::xphoto::WhiteBalancer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::xphoto::WhiteBalancer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::xphoto::WhiteBalancer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_xphoto_WhiteBalancerG_to_PtrOfAlgorithm(cv::Ptr<cv::xphoto::WhiteBalancer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

