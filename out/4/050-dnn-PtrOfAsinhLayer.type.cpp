extern "C" {
	// cv::Ptr<cv::dnn::AsinhLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::AsinhLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::AsinhLayer* cv_PtrLcv_dnn_AsinhLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::AsinhLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::AsinhLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::AsinhLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::AsinhLayer* cv_PtrLcv_dnn_AsinhLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::AsinhLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::AsinhLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::AsinhLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::AsinhLayer>* cv_PtrLcv_dnn_AsinhLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::AsinhLayer>();
	}

	// cv::Ptr<cv::dnn::AsinhLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::AsinhLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_AsinhLayerG_delete(cv::Ptr<cv::dnn::AsinhLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::AsinhLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::AsinhLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_AsinhLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::AsinhLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::AsinhLayer>::to_PtrOfActivationLayer() generated
	// ("cv::Ptr<cv::dnn::AsinhLayer>::to_PtrOfActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::ActivationLayer>* cv_PtrLcv_dnn_AsinhLayerG_to_PtrOfActivationLayer(cv::Ptr<cv::dnn::AsinhLayer>* instance) {
			return new cv::Ptr<cv::dnn::ActivationLayer>(instance->dynamicCast<cv::dnn::ActivationLayer>());
	}

	// cv::Ptr<cv::dnn::AsinhLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::AsinhLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_AsinhLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::AsinhLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::AsinhLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::AsinhLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::AsinhLayer"]), _)]),
	cv::Ptr<cv::dnn::AsinhLayer>* cv_PtrLcv_dnn_AsinhLayerG_new_const_AsinhLayer(cv::dnn::AsinhLayer* val) {
			return new cv::Ptr<cv::dnn::AsinhLayer>(val);
	}

}

