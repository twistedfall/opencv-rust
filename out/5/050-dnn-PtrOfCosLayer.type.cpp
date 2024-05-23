extern "C" {
	// cv::Ptr<cv::dnn::CosLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::CosLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::CosLayer* cv_PtrLcv_dnn_CosLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::CosLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::CosLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::CosLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::CosLayer* cv_PtrLcv_dnn_CosLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::CosLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::CosLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::CosLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::CosLayer>* cv_PtrLcv_dnn_CosLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::CosLayer>();
	}

	// cv::Ptr<cv::dnn::CosLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::CosLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_CosLayerG_delete(cv::Ptr<cv::dnn::CosLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::CosLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::CosLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_CosLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::CosLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::CosLayer>::to_PtrOfActivationLayer() generated
	// ("cv::Ptr<cv::dnn::CosLayer>::to_PtrOfActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::ActivationLayer>* cv_PtrLcv_dnn_CosLayerG_to_PtrOfActivationLayer(cv::Ptr<cv::dnn::CosLayer>* instance) {
			return new cv::Ptr<cv::dnn::ActivationLayer>(instance->dynamicCast<cv::dnn::ActivationLayer>());
	}

	// cv::Ptr<cv::dnn::CosLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::CosLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_CosLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::CosLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::CosLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::CosLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::CosLayer"]), _)]),
	cv::Ptr<cv::dnn::CosLayer>* cv_PtrLcv_dnn_CosLayerG_new_const_CosLayer(cv::dnn::CosLayer* val) {
			return new cv::Ptr<cv::dnn::CosLayer>(val);
	}

}

