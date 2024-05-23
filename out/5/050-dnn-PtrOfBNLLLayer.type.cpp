extern "C" {
	// cv::Ptr<cv::dnn::BNLLLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::BNLLLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::BNLLLayer* cv_PtrLcv_dnn_BNLLLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::BNLLLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::BNLLLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::BNLLLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::BNLLLayer* cv_PtrLcv_dnn_BNLLLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::BNLLLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::BNLLLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::BNLLLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::BNLLLayer>* cv_PtrLcv_dnn_BNLLLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::BNLLLayer>();
	}

	// cv::Ptr<cv::dnn::BNLLLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::BNLLLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_BNLLLayerG_delete(cv::Ptr<cv::dnn::BNLLLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::BNLLLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::BNLLLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_BNLLLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::BNLLLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::BNLLLayer>::to_PtrOfActivationLayer() generated
	// ("cv::Ptr<cv::dnn::BNLLLayer>::to_PtrOfActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::ActivationLayer>* cv_PtrLcv_dnn_BNLLLayerG_to_PtrOfActivationLayer(cv::Ptr<cv::dnn::BNLLLayer>* instance) {
			return new cv::Ptr<cv::dnn::ActivationLayer>(instance->dynamicCast<cv::dnn::ActivationLayer>());
	}

	// cv::Ptr<cv::dnn::BNLLLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::BNLLLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_BNLLLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::BNLLLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::BNLLLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::BNLLLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::BNLLLayer"]), _)]),
	cv::Ptr<cv::dnn::BNLLLayer>* cv_PtrLcv_dnn_BNLLLayerG_new_const_BNLLLayer(cv::dnn::BNLLLayer* val) {
			return new cv::Ptr<cv::dnn::BNLLLayer>(val);
	}

}

