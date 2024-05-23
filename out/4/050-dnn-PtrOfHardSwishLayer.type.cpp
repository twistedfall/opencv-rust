extern "C" {
	// cv::Ptr<cv::dnn::HardSwishLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::HardSwishLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::HardSwishLayer* cv_PtrLcv_dnn_HardSwishLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::HardSwishLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::HardSwishLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::HardSwishLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::HardSwishLayer* cv_PtrLcv_dnn_HardSwishLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::HardSwishLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::HardSwishLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::HardSwishLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::HardSwishLayer>* cv_PtrLcv_dnn_HardSwishLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::HardSwishLayer>();
	}

	// cv::Ptr<cv::dnn::HardSwishLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::HardSwishLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_HardSwishLayerG_delete(cv::Ptr<cv::dnn::HardSwishLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::HardSwishLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::HardSwishLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_HardSwishLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::HardSwishLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::HardSwishLayer>::to_PtrOfActivationLayer() generated
	// ("cv::Ptr<cv::dnn::HardSwishLayer>::to_PtrOfActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::ActivationLayer>* cv_PtrLcv_dnn_HardSwishLayerG_to_PtrOfActivationLayer(cv::Ptr<cv::dnn::HardSwishLayer>* instance) {
			return new cv::Ptr<cv::dnn::ActivationLayer>(instance->dynamicCast<cv::dnn::ActivationLayer>());
	}

	// cv::Ptr<cv::dnn::HardSwishLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::HardSwishLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_HardSwishLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::HardSwishLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::HardSwishLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::HardSwishLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::HardSwishLayer"]), _)]),
	cv::Ptr<cv::dnn::HardSwishLayer>* cv_PtrLcv_dnn_HardSwishLayerG_new_const_HardSwishLayer(cv::dnn::HardSwishLayer* val) {
			return new cv::Ptr<cv::dnn::HardSwishLayer>(val);
	}

}

