extern "C" {
	// cv::Ptr<cv::dnn::ReLULayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::ReLULayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::ReLULayer* cv_PtrLcv_dnn_ReLULayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::ReLULayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::ReLULayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::ReLULayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ReLULayer* cv_PtrLcv_dnn_ReLULayerG_getInnerPtrMut(cv::Ptr<cv::dnn::ReLULayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::ReLULayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::ReLULayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::ReLULayer>* cv_PtrLcv_dnn_ReLULayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::ReLULayer>();
	}

	// cv::Ptr<cv::dnn::ReLULayer>::delete() generated
	// ("cv::Ptr<cv::dnn::ReLULayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_ReLULayerG_delete(cv::Ptr<cv::dnn::ReLULayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::ReLULayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::ReLULayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_ReLULayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::ReLULayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::ReLULayer>::to_PtrOfActivationLayer() generated
	// ("cv::Ptr<cv::dnn::ReLULayer>::to_PtrOfActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::ActivationLayer>* cv_PtrLcv_dnn_ReLULayerG_to_PtrOfActivationLayer(cv::Ptr<cv::dnn::ReLULayer>* instance) {
			return new cv::Ptr<cv::dnn::ActivationLayer>(instance->dynamicCast<cv::dnn::ActivationLayer>());
	}

	// cv::Ptr<cv::dnn::ReLULayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::ReLULayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_ReLULayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::ReLULayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::ReLULayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::ReLULayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::ReLULayer"]), _)]),
	cv::Ptr<cv::dnn::ReLULayer>* cv_PtrLcv_dnn_ReLULayerG_new_const_ReLULayer(cv::dnn::ReLULayer* val) {
			return new cv::Ptr<cv::dnn::ReLULayer>(val);
	}

}

