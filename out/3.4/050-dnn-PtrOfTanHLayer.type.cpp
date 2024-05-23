extern "C" {
	// cv::Ptr<cv::dnn::TanHLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::TanHLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::TanHLayer* cv_PtrLcv_dnn_TanHLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::TanHLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::TanHLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::TanHLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::TanHLayer* cv_PtrLcv_dnn_TanHLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::TanHLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::TanHLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::TanHLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::TanHLayer>* cv_PtrLcv_dnn_TanHLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::TanHLayer>();
	}

	// cv::Ptr<cv::dnn::TanHLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::TanHLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_TanHLayerG_delete(cv::Ptr<cv::dnn::TanHLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::TanHLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::TanHLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_TanHLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::TanHLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::TanHLayer>::to_PtrOfActivationLayer() generated
	// ("cv::Ptr<cv::dnn::TanHLayer>::to_PtrOfActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::ActivationLayer>* cv_PtrLcv_dnn_TanHLayerG_to_PtrOfActivationLayer(cv::Ptr<cv::dnn::TanHLayer>* instance) {
			return new cv::Ptr<cv::dnn::ActivationLayer>(instance->dynamicCast<cv::dnn::ActivationLayer>());
	}

	// cv::Ptr<cv::dnn::TanHLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::TanHLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_TanHLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::TanHLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

}

