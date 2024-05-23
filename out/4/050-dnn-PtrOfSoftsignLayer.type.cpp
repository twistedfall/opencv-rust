extern "C" {
	// cv::Ptr<cv::dnn::SoftsignLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::SoftsignLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::SoftsignLayer* cv_PtrLcv_dnn_SoftsignLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::SoftsignLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::SoftsignLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::SoftsignLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::SoftsignLayer* cv_PtrLcv_dnn_SoftsignLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::SoftsignLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::SoftsignLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::SoftsignLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::SoftsignLayer>* cv_PtrLcv_dnn_SoftsignLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::SoftsignLayer>();
	}

	// cv::Ptr<cv::dnn::SoftsignLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::SoftsignLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_SoftsignLayerG_delete(cv::Ptr<cv::dnn::SoftsignLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::SoftsignLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::SoftsignLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_SoftsignLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::SoftsignLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::SoftsignLayer>::to_PtrOfActivationLayer() generated
	// ("cv::Ptr<cv::dnn::SoftsignLayer>::to_PtrOfActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::ActivationLayer>* cv_PtrLcv_dnn_SoftsignLayerG_to_PtrOfActivationLayer(cv::Ptr<cv::dnn::SoftsignLayer>* instance) {
			return new cv::Ptr<cv::dnn::ActivationLayer>(instance->dynamicCast<cv::dnn::ActivationLayer>());
	}

	// cv::Ptr<cv::dnn::SoftsignLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::SoftsignLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_SoftsignLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::SoftsignLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::SoftsignLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::SoftsignLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::SoftsignLayer"]), _)]),
	cv::Ptr<cv::dnn::SoftsignLayer>* cv_PtrLcv_dnn_SoftsignLayerG_new_const_SoftsignLayer(cv::dnn::SoftsignLayer* val) {
			return new cv::Ptr<cv::dnn::SoftsignLayer>(val);
	}

}

