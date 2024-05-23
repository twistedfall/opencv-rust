extern "C" {
	// cv::Ptr<cv::dnn::GeluLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::GeluLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::GeluLayer* cv_PtrLcv_dnn_GeluLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::GeluLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::GeluLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::GeluLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::GeluLayer* cv_PtrLcv_dnn_GeluLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::GeluLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::GeluLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::GeluLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::GeluLayer>* cv_PtrLcv_dnn_GeluLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::GeluLayer>();
	}

	// cv::Ptr<cv::dnn::GeluLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::GeluLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_GeluLayerG_delete(cv::Ptr<cv::dnn::GeluLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::GeluLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::GeluLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_GeluLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::GeluLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::GeluLayer>::to_PtrOfActivationLayer() generated
	// ("cv::Ptr<cv::dnn::GeluLayer>::to_PtrOfActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::ActivationLayer>* cv_PtrLcv_dnn_GeluLayerG_to_PtrOfActivationLayer(cv::Ptr<cv::dnn::GeluLayer>* instance) {
			return new cv::Ptr<cv::dnn::ActivationLayer>(instance->dynamicCast<cv::dnn::ActivationLayer>());
	}

	// cv::Ptr<cv::dnn::GeluLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::GeluLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_GeluLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::GeluLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::GeluLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::GeluLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::GeluLayer"]), _)]),
	cv::Ptr<cv::dnn::GeluLayer>* cv_PtrLcv_dnn_GeluLayerG_new_const_GeluLayer(cv::dnn::GeluLayer* val) {
			return new cv::Ptr<cv::dnn::GeluLayer>(val);
	}

}

