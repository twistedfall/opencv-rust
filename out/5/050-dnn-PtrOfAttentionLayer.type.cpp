extern "C" {
	// cv::Ptr<cv::dnn::AttentionLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::AttentionLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::AttentionLayer* cv_PtrLcv_dnn_AttentionLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::AttentionLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::AttentionLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::AttentionLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::AttentionLayer* cv_PtrLcv_dnn_AttentionLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::AttentionLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::AttentionLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::AttentionLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::AttentionLayer>* cv_PtrLcv_dnn_AttentionLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::AttentionLayer>();
	}

	// cv::Ptr<cv::dnn::AttentionLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::AttentionLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_AttentionLayerG_delete(cv::Ptr<cv::dnn::AttentionLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::AttentionLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::AttentionLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_AttentionLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::AttentionLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::AttentionLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::AttentionLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_AttentionLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::AttentionLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::AttentionLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::AttentionLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::AttentionLayer"]), _)]),
	cv::Ptr<cv::dnn::AttentionLayer>* cv_PtrLcv_dnn_AttentionLayerG_new_const_AttentionLayer(cv::dnn::AttentionLayer* val) {
			return new cv::Ptr<cv::dnn::AttentionLayer>(val);
	}

}

