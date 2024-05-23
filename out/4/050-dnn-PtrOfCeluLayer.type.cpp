extern "C" {
	// cv::Ptr<cv::dnn::CeluLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::CeluLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::CeluLayer* cv_PtrLcv_dnn_CeluLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::CeluLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::CeluLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::CeluLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::CeluLayer* cv_PtrLcv_dnn_CeluLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::CeluLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::CeluLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::CeluLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::CeluLayer>* cv_PtrLcv_dnn_CeluLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::CeluLayer>();
	}

	// cv::Ptr<cv::dnn::CeluLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::CeluLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_CeluLayerG_delete(cv::Ptr<cv::dnn::CeluLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::CeluLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::CeluLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_CeluLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::CeluLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::CeluLayer>::to_PtrOfActivationLayer() generated
	// ("cv::Ptr<cv::dnn::CeluLayer>::to_PtrOfActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::ActivationLayer>* cv_PtrLcv_dnn_CeluLayerG_to_PtrOfActivationLayer(cv::Ptr<cv::dnn::CeluLayer>* instance) {
			return new cv::Ptr<cv::dnn::ActivationLayer>(instance->dynamicCast<cv::dnn::ActivationLayer>());
	}

	// cv::Ptr<cv::dnn::CeluLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::CeluLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_CeluLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::CeluLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::CeluLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::CeluLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::CeluLayer"]), _)]),
	cv::Ptr<cv::dnn::CeluLayer>* cv_PtrLcv_dnn_CeluLayerG_new_const_CeluLayer(cv::dnn::CeluLayer* val) {
			return new cv::Ptr<cv::dnn::CeluLayer>(val);
	}

}

