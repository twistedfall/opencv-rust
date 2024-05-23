extern "C" {
	// cv::Ptr<cv::dnn::CeilLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::CeilLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::CeilLayer* cv_PtrLcv_dnn_CeilLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::CeilLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::CeilLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::CeilLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::CeilLayer* cv_PtrLcv_dnn_CeilLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::CeilLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::CeilLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::CeilLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::CeilLayer>* cv_PtrLcv_dnn_CeilLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::CeilLayer>();
	}

	// cv::Ptr<cv::dnn::CeilLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::CeilLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_CeilLayerG_delete(cv::Ptr<cv::dnn::CeilLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::CeilLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::CeilLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_CeilLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::CeilLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::CeilLayer>::to_PtrOfActivationLayer() generated
	// ("cv::Ptr<cv::dnn::CeilLayer>::to_PtrOfActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::ActivationLayer>* cv_PtrLcv_dnn_CeilLayerG_to_PtrOfActivationLayer(cv::Ptr<cv::dnn::CeilLayer>* instance) {
			return new cv::Ptr<cv::dnn::ActivationLayer>(instance->dynamicCast<cv::dnn::ActivationLayer>());
	}

	// cv::Ptr<cv::dnn::CeilLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::CeilLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_CeilLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::CeilLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::CeilLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::CeilLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::CeilLayer"]), _)]),
	cv::Ptr<cv::dnn::CeilLayer>* cv_PtrLcv_dnn_CeilLayerG_new_const_CeilLayer(cv::dnn::CeilLayer* val) {
			return new cv::Ptr<cv::dnn::CeilLayer>(val);
	}

}

