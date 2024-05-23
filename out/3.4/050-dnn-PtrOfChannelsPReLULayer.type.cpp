extern "C" {
	// cv::Ptr<cv::dnn::ChannelsPReLULayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::ChannelsPReLULayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::ChannelsPReLULayer* cv_PtrLcv_dnn_ChannelsPReLULayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::ChannelsPReLULayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::ChannelsPReLULayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::ChannelsPReLULayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ChannelsPReLULayer* cv_PtrLcv_dnn_ChannelsPReLULayerG_getInnerPtrMut(cv::Ptr<cv::dnn::ChannelsPReLULayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::ChannelsPReLULayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::ChannelsPReLULayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::ChannelsPReLULayer>* cv_PtrLcv_dnn_ChannelsPReLULayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::ChannelsPReLULayer>();
	}

	// cv::Ptr<cv::dnn::ChannelsPReLULayer>::delete() generated
	// ("cv::Ptr<cv::dnn::ChannelsPReLULayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_ChannelsPReLULayerG_delete(cv::Ptr<cv::dnn::ChannelsPReLULayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::ChannelsPReLULayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::ChannelsPReLULayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_ChannelsPReLULayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::ChannelsPReLULayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::ChannelsPReLULayer>::to_PtrOfActivationLayer() generated
	// ("cv::Ptr<cv::dnn::ChannelsPReLULayer>::to_PtrOfActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::ActivationLayer>* cv_PtrLcv_dnn_ChannelsPReLULayerG_to_PtrOfActivationLayer(cv::Ptr<cv::dnn::ChannelsPReLULayer>* instance) {
			return new cv::Ptr<cv::dnn::ActivationLayer>(instance->dynamicCast<cv::dnn::ActivationLayer>());
	}

	// cv::Ptr<cv::dnn::ChannelsPReLULayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::ChannelsPReLULayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_ChannelsPReLULayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::ChannelsPReLULayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

}

