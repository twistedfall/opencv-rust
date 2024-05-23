extern "C" {
	// cv::Ptr<cv::dnn::SignLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::SignLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::SignLayer* cv_PtrLcv_dnn_SignLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::SignLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::SignLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::SignLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::SignLayer* cv_PtrLcv_dnn_SignLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::SignLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::SignLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::SignLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::SignLayer>* cv_PtrLcv_dnn_SignLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::SignLayer>();
	}

	// cv::Ptr<cv::dnn::SignLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::SignLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_SignLayerG_delete(cv::Ptr<cv::dnn::SignLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::SignLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::SignLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_SignLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::SignLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::SignLayer>::to_PtrOfActivationLayer() generated
	// ("cv::Ptr<cv::dnn::SignLayer>::to_PtrOfActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::ActivationLayer>* cv_PtrLcv_dnn_SignLayerG_to_PtrOfActivationLayer(cv::Ptr<cv::dnn::SignLayer>* instance) {
			return new cv::Ptr<cv::dnn::ActivationLayer>(instance->dynamicCast<cv::dnn::ActivationLayer>());
	}

	// cv::Ptr<cv::dnn::SignLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::SignLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_SignLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::SignLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::SignLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::SignLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::SignLayer"]), _)]),
	cv::Ptr<cv::dnn::SignLayer>* cv_PtrLcv_dnn_SignLayerG_new_const_SignLayer(cv::dnn::SignLayer* val) {
			return new cv::Ptr<cv::dnn::SignLayer>(val);
	}

}

