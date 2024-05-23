extern "C" {
	// cv::Ptr<cv::dnn::SeluLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::SeluLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::SeluLayer* cv_PtrLcv_dnn_SeluLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::SeluLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::SeluLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::SeluLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::SeluLayer* cv_PtrLcv_dnn_SeluLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::SeluLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::SeluLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::SeluLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::SeluLayer>* cv_PtrLcv_dnn_SeluLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::SeluLayer>();
	}

	// cv::Ptr<cv::dnn::SeluLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::SeluLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_SeluLayerG_delete(cv::Ptr<cv::dnn::SeluLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::SeluLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::SeluLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_SeluLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::SeluLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::SeluLayer>::to_PtrOfActivationLayer() generated
	// ("cv::Ptr<cv::dnn::SeluLayer>::to_PtrOfActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::ActivationLayer>* cv_PtrLcv_dnn_SeluLayerG_to_PtrOfActivationLayer(cv::Ptr<cv::dnn::SeluLayer>* instance) {
			return new cv::Ptr<cv::dnn::ActivationLayer>(instance->dynamicCast<cv::dnn::ActivationLayer>());
	}

	// cv::Ptr<cv::dnn::SeluLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::SeluLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_SeluLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::SeluLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::SeluLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::SeluLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::SeluLayer"]), _)]),
	cv::Ptr<cv::dnn::SeluLayer>* cv_PtrLcv_dnn_SeluLayerG_new_const_SeluLayer(cv::dnn::SeluLayer* val) {
			return new cv::Ptr<cv::dnn::SeluLayer>(val);
	}

}

