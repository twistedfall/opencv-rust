extern "C" {
	// cv::Ptr<cv::dnn::ReorgLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::ReorgLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::ReorgLayer* cv_PtrLcv_dnn_ReorgLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::ReorgLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::ReorgLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::ReorgLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ReorgLayer* cv_PtrLcv_dnn_ReorgLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::ReorgLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::ReorgLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::ReorgLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::ReorgLayer>* cv_PtrLcv_dnn_ReorgLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::ReorgLayer>();
	}

	// cv::Ptr<cv::dnn::ReorgLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::ReorgLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_ReorgLayerG_delete(cv::Ptr<cv::dnn::ReorgLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::ReorgLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::ReorgLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_ReorgLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::ReorgLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::ReorgLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::ReorgLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_ReorgLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::ReorgLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::ReorgLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::ReorgLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::ReorgLayer"]), _)]),
	cv::Ptr<cv::dnn::ReorgLayer>* cv_PtrLcv_dnn_ReorgLayerG_new_const_ReorgLayer(cv::dnn::ReorgLayer* val) {
			return new cv::Ptr<cv::dnn::ReorgLayer>(val);
	}

}

