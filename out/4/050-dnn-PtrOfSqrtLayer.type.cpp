extern "C" {
	// cv::Ptr<cv::dnn::SqrtLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::SqrtLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::SqrtLayer* cv_PtrLcv_dnn_SqrtLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::SqrtLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::SqrtLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::SqrtLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::SqrtLayer* cv_PtrLcv_dnn_SqrtLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::SqrtLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::SqrtLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::SqrtLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::SqrtLayer>* cv_PtrLcv_dnn_SqrtLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::SqrtLayer>();
	}

	// cv::Ptr<cv::dnn::SqrtLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::SqrtLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_SqrtLayerG_delete(cv::Ptr<cv::dnn::SqrtLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::SqrtLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::SqrtLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_SqrtLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::SqrtLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::SqrtLayer>::to_PtrOfActivationLayer() generated
	// ("cv::Ptr<cv::dnn::SqrtLayer>::to_PtrOfActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::ActivationLayer>* cv_PtrLcv_dnn_SqrtLayerG_to_PtrOfActivationLayer(cv::Ptr<cv::dnn::SqrtLayer>* instance) {
			return new cv::Ptr<cv::dnn::ActivationLayer>(instance->dynamicCast<cv::dnn::ActivationLayer>());
	}

	// cv::Ptr<cv::dnn::SqrtLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::SqrtLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_SqrtLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::SqrtLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::SqrtLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::SqrtLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::SqrtLayer"]), _)]),
	cv::Ptr<cv::dnn::SqrtLayer>* cv_PtrLcv_dnn_SqrtLayerG_new_const_SqrtLayer(cv::dnn::SqrtLayer* val) {
			return new cv::Ptr<cv::dnn::SqrtLayer>(val);
	}

}

