extern "C" {
	// cv::Ptr<cv::dnn::MishLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::MishLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::MishLayer* cv_PtrLcv_dnn_MishLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::MishLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::MishLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::MishLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::MishLayer* cv_PtrLcv_dnn_MishLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::MishLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::MishLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::MishLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::MishLayer>* cv_PtrLcv_dnn_MishLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::MishLayer>();
	}

	// cv::Ptr<cv::dnn::MishLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::MishLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_MishLayerG_delete(cv::Ptr<cv::dnn::MishLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::MishLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::MishLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_MishLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::MishLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::MishLayer>::to_PtrOfActivationLayer() generated
	// ("cv::Ptr<cv::dnn::MishLayer>::to_PtrOfActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::ActivationLayer>* cv_PtrLcv_dnn_MishLayerG_to_PtrOfActivationLayer(cv::Ptr<cv::dnn::MishLayer>* instance) {
			return new cv::Ptr<cv::dnn::ActivationLayer>(instance->dynamicCast<cv::dnn::ActivationLayer>());
	}

	// cv::Ptr<cv::dnn::MishLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::MishLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_MishLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::MishLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

}

