extern "C" {
	// cv::Ptr<cv::dnn::SigmoidLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::SigmoidLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::SigmoidLayer* cv_PtrLcv_dnn_SigmoidLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::SigmoidLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::SigmoidLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::SigmoidLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::SigmoidLayer* cv_PtrLcv_dnn_SigmoidLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::SigmoidLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::SigmoidLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::SigmoidLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::SigmoidLayer>* cv_PtrLcv_dnn_SigmoidLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::SigmoidLayer>();
	}

	// cv::Ptr<cv::dnn::SigmoidLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::SigmoidLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_SigmoidLayerG_delete(cv::Ptr<cv::dnn::SigmoidLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::SigmoidLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::SigmoidLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_SigmoidLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::SigmoidLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::SigmoidLayer>::to_PtrOfActivationLayer() generated
	// ("cv::Ptr<cv::dnn::SigmoidLayer>::to_PtrOfActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::ActivationLayer>* cv_PtrLcv_dnn_SigmoidLayerG_to_PtrOfActivationLayer(cv::Ptr<cv::dnn::SigmoidLayer>* instance) {
			return new cv::Ptr<cv::dnn::ActivationLayer>(instance->dynamicCast<cv::dnn::ActivationLayer>());
	}

	// cv::Ptr<cv::dnn::SigmoidLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::SigmoidLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_SigmoidLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::SigmoidLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

}

