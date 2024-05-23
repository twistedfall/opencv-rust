extern "C" {
	// cv::Ptr<cv::dnn::ErfLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::ErfLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::ErfLayer* cv_PtrLcv_dnn_ErfLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::ErfLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::ErfLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::ErfLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ErfLayer* cv_PtrLcv_dnn_ErfLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::ErfLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::ErfLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::ErfLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::ErfLayer>* cv_PtrLcv_dnn_ErfLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::ErfLayer>();
	}

	// cv::Ptr<cv::dnn::ErfLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::ErfLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_ErfLayerG_delete(cv::Ptr<cv::dnn::ErfLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::ErfLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::ErfLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_ErfLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::ErfLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::ErfLayer>::to_PtrOfActivationLayer() generated
	// ("cv::Ptr<cv::dnn::ErfLayer>::to_PtrOfActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::ActivationLayer>* cv_PtrLcv_dnn_ErfLayerG_to_PtrOfActivationLayer(cv::Ptr<cv::dnn::ErfLayer>* instance) {
			return new cv::Ptr<cv::dnn::ActivationLayer>(instance->dynamicCast<cv::dnn::ActivationLayer>());
	}

	// cv::Ptr<cv::dnn::ErfLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::ErfLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_ErfLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::ErfLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::ErfLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::ErfLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::ErfLayer"]), _)]),
	cv::Ptr<cv::dnn::ErfLayer>* cv_PtrLcv_dnn_ErfLayerG_new_const_ErfLayer(cv::dnn::ErfLayer* val) {
			return new cv::Ptr<cv::dnn::ErfLayer>(val);
	}

}

