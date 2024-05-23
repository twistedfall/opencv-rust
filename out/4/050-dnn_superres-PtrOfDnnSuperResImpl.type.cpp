extern "C" {
	// cv::Ptr<cv::dnn_superres::DnnSuperResImpl>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn_superres::DnnSuperResImpl>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn_superres::DnnSuperResImpl* cv_PtrLcv_dnn_superres_DnnSuperResImplG_getInnerPtr_const(const cv::Ptr<cv::dnn_superres::DnnSuperResImpl>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn_superres::DnnSuperResImpl>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn_superres::DnnSuperResImpl>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn_superres::DnnSuperResImpl* cv_PtrLcv_dnn_superres_DnnSuperResImplG_getInnerPtrMut(cv::Ptr<cv::dnn_superres::DnnSuperResImpl>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn_superres::DnnSuperResImpl>::new_null() generated
	// ("cv::Ptr<cv::dnn_superres::DnnSuperResImpl>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn_superres::DnnSuperResImpl>* cv_PtrLcv_dnn_superres_DnnSuperResImplG_new_null_const() {
			return new cv::Ptr<cv::dnn_superres::DnnSuperResImpl>();
	}

	// cv::Ptr<cv::dnn_superres::DnnSuperResImpl>::delete() generated
	// ("cv::Ptr<cv::dnn_superres::DnnSuperResImpl>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_superres_DnnSuperResImplG_delete(cv::Ptr<cv::dnn_superres::DnnSuperResImpl>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn_superres::DnnSuperResImpl>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn_superres::DnnSuperResImpl>::new", vec![(pred!(const, ["val"], ["cv::dnn_superres::DnnSuperResImpl"]), _)]),
	cv::Ptr<cv::dnn_superres::DnnSuperResImpl>* cv_PtrLcv_dnn_superres_DnnSuperResImplG_new_const_DnnSuperResImpl(cv::dnn_superres::DnnSuperResImpl* val) {
			return new cv::Ptr<cv::dnn_superres::DnnSuperResImpl>(val);
	}

}

