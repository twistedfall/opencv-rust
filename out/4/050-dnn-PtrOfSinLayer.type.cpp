extern "C" {
	// cv::Ptr<cv::dnn::SinLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::SinLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::SinLayer* cv_PtrLcv_dnn_SinLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::SinLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::SinLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::SinLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::SinLayer* cv_PtrLcv_dnn_SinLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::SinLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::SinLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::SinLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::SinLayer>* cv_PtrLcv_dnn_SinLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::SinLayer>();
	}

	// cv::Ptr<cv::dnn::SinLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::SinLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_SinLayerG_delete(cv::Ptr<cv::dnn::SinLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::SinLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::SinLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_SinLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::SinLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::SinLayer>::to_PtrOfActivationLayer() generated
	// ("cv::Ptr<cv::dnn::SinLayer>::to_PtrOfActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::ActivationLayer>* cv_PtrLcv_dnn_SinLayerG_to_PtrOfActivationLayer(cv::Ptr<cv::dnn::SinLayer>* instance) {
			return new cv::Ptr<cv::dnn::ActivationLayer>(instance->dynamicCast<cv::dnn::ActivationLayer>());
	}

	// cv::Ptr<cv::dnn::SinLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::SinLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_SinLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::SinLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::SinLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::SinLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::SinLayer"]), _)]),
	cv::Ptr<cv::dnn::SinLayer>* cv_PtrLcv_dnn_SinLayerG_new_const_SinLayer(cv::dnn::SinLayer* val) {
			return new cv::Ptr<cv::dnn::SinLayer>(val);
	}

}

