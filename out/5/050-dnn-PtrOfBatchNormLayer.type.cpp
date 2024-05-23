extern "C" {
	// cv::Ptr<cv::dnn::BatchNormLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::BatchNormLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::BatchNormLayer* cv_PtrLcv_dnn_BatchNormLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::BatchNormLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::BatchNormLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::BatchNormLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::BatchNormLayer* cv_PtrLcv_dnn_BatchNormLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::BatchNormLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::BatchNormLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::BatchNormLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::BatchNormLayer>* cv_PtrLcv_dnn_BatchNormLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::BatchNormLayer>();
	}

	// cv::Ptr<cv::dnn::BatchNormLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::BatchNormLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_BatchNormLayerG_delete(cv::Ptr<cv::dnn::BatchNormLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::BatchNormLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::BatchNormLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_BatchNormLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::BatchNormLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::BatchNormLayer>::to_PtrOfActivationLayer() generated
	// ("cv::Ptr<cv::dnn::BatchNormLayer>::to_PtrOfActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::ActivationLayer>* cv_PtrLcv_dnn_BatchNormLayerG_to_PtrOfActivationLayer(cv::Ptr<cv::dnn::BatchNormLayer>* instance) {
			return new cv::Ptr<cv::dnn::ActivationLayer>(instance->dynamicCast<cv::dnn::ActivationLayer>());
	}

	// cv::Ptr<cv::dnn::BatchNormLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::BatchNormLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_BatchNormLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::BatchNormLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::BatchNormLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::BatchNormLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::BatchNormLayer"]), _)]),
	cv::Ptr<cv::dnn::BatchNormLayer>* cv_PtrLcv_dnn_BatchNormLayerG_new_const_BatchNormLayer(cv::dnn::BatchNormLayer* val) {
			return new cv::Ptr<cv::dnn::BatchNormLayer>(val);
	}

}

