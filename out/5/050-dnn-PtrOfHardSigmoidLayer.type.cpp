extern "C" {
	// cv::Ptr<cv::dnn::HardSigmoidLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::HardSigmoidLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::HardSigmoidLayer* cv_PtrLcv_dnn_HardSigmoidLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::HardSigmoidLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::HardSigmoidLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::HardSigmoidLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::HardSigmoidLayer* cv_PtrLcv_dnn_HardSigmoidLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::HardSigmoidLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::HardSigmoidLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::HardSigmoidLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::HardSigmoidLayer>* cv_PtrLcv_dnn_HardSigmoidLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::HardSigmoidLayer>();
	}

	// cv::Ptr<cv::dnn::HardSigmoidLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::HardSigmoidLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_HardSigmoidLayerG_delete(cv::Ptr<cv::dnn::HardSigmoidLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::HardSigmoidLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::HardSigmoidLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_HardSigmoidLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::HardSigmoidLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::HardSigmoidLayer>::to_PtrOfActivationLayer() generated
	// ("cv::Ptr<cv::dnn::HardSigmoidLayer>::to_PtrOfActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::ActivationLayer>* cv_PtrLcv_dnn_HardSigmoidLayerG_to_PtrOfActivationLayer(cv::Ptr<cv::dnn::HardSigmoidLayer>* instance) {
			return new cv::Ptr<cv::dnn::ActivationLayer>(instance->dynamicCast<cv::dnn::ActivationLayer>());
	}

	// cv::Ptr<cv::dnn::HardSigmoidLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::HardSigmoidLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_HardSigmoidLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::HardSigmoidLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::HardSigmoidLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::HardSigmoidLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::HardSigmoidLayer"]), _)]),
	cv::Ptr<cv::dnn::HardSigmoidLayer>* cv_PtrLcv_dnn_HardSigmoidLayerG_new_const_HardSigmoidLayer(cv::dnn::HardSigmoidLayer* val) {
			return new cv::Ptr<cv::dnn::HardSigmoidLayer>(val);
	}

}

