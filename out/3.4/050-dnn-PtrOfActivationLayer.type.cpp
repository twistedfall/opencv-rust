extern "C" {
	// cv::Ptr<cv::dnn::ActivationLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::ActivationLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::ActivationLayer* cv_PtrLcv_dnn_ActivationLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::ActivationLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::ActivationLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::ActivationLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ActivationLayer* cv_PtrLcv_dnn_ActivationLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::ActivationLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::ActivationLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::ActivationLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::ActivationLayer>* cv_PtrLcv_dnn_ActivationLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::ActivationLayer>();
	}

	// cv::Ptr<cv::dnn::ActivationLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::ActivationLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_ActivationLayerG_delete(cv::Ptr<cv::dnn::ActivationLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::ActivationLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::ActivationLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_ActivationLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::ActivationLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::ActivationLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::ActivationLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_ActivationLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::ActivationLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

}

