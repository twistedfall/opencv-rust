extern "C" {
	// cv::Ptr<cv::dnn::ELULayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::ELULayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::ELULayer* cv_PtrLcv_dnn_ELULayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::ELULayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::ELULayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::ELULayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ELULayer* cv_PtrLcv_dnn_ELULayerG_getInnerPtrMut(cv::Ptr<cv::dnn::ELULayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::ELULayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::ELULayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::ELULayer>* cv_PtrLcv_dnn_ELULayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::ELULayer>();
	}

	// cv::Ptr<cv::dnn::ELULayer>::delete() generated
	// ("cv::Ptr<cv::dnn::ELULayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_ELULayerG_delete(cv::Ptr<cv::dnn::ELULayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::ELULayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::ELULayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_ELULayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::ELULayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::ELULayer>::to_PtrOfActivationLayer() generated
	// ("cv::Ptr<cv::dnn::ELULayer>::to_PtrOfActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::ActivationLayer>* cv_PtrLcv_dnn_ELULayerG_to_PtrOfActivationLayer(cv::Ptr<cv::dnn::ELULayer>* instance) {
			return new cv::Ptr<cv::dnn::ActivationLayer>(instance->dynamicCast<cv::dnn::ActivationLayer>());
	}

	// cv::Ptr<cv::dnn::ELULayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::ELULayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_ELULayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::ELULayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::ELULayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::ELULayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::ELULayer"]), _)]),
	cv::Ptr<cv::dnn::ELULayer>* cv_PtrLcv_dnn_ELULayerG_new_const_ELULayer(cv::dnn::ELULayer* val) {
			return new cv::Ptr<cv::dnn::ELULayer>(val);
	}

}

