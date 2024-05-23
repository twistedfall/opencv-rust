extern "C" {
	// cv::Ptr<cv::dnn::ThresholdedReluLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::ThresholdedReluLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::ThresholdedReluLayer* cv_PtrLcv_dnn_ThresholdedReluLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::ThresholdedReluLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::ThresholdedReluLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::ThresholdedReluLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ThresholdedReluLayer* cv_PtrLcv_dnn_ThresholdedReluLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::ThresholdedReluLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::ThresholdedReluLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::ThresholdedReluLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::ThresholdedReluLayer>* cv_PtrLcv_dnn_ThresholdedReluLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::ThresholdedReluLayer>();
	}

	// cv::Ptr<cv::dnn::ThresholdedReluLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::ThresholdedReluLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_ThresholdedReluLayerG_delete(cv::Ptr<cv::dnn::ThresholdedReluLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::ThresholdedReluLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::ThresholdedReluLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_ThresholdedReluLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::ThresholdedReluLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::ThresholdedReluLayer>::to_PtrOfActivationLayer() generated
	// ("cv::Ptr<cv::dnn::ThresholdedReluLayer>::to_PtrOfActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::ActivationLayer>* cv_PtrLcv_dnn_ThresholdedReluLayerG_to_PtrOfActivationLayer(cv::Ptr<cv::dnn::ThresholdedReluLayer>* instance) {
			return new cv::Ptr<cv::dnn::ActivationLayer>(instance->dynamicCast<cv::dnn::ActivationLayer>());
	}

	// cv::Ptr<cv::dnn::ThresholdedReluLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::ThresholdedReluLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_ThresholdedReluLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::ThresholdedReluLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::ThresholdedReluLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::ThresholdedReluLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::ThresholdedReluLayer"]), _)]),
	cv::Ptr<cv::dnn::ThresholdedReluLayer>* cv_PtrLcv_dnn_ThresholdedReluLayerG_new_const_ThresholdedReluLayer(cv::dnn::ThresholdedReluLayer* val) {
			return new cv::Ptr<cv::dnn::ThresholdedReluLayer>(val);
	}

}

