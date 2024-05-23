extern "C" {
	// cv::Ptr<cv::dnn::GroupNormLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::GroupNormLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::GroupNormLayer* cv_PtrLcv_dnn_GroupNormLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::GroupNormLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::GroupNormLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::GroupNormLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::GroupNormLayer* cv_PtrLcv_dnn_GroupNormLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::GroupNormLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::GroupNormLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::GroupNormLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::GroupNormLayer>* cv_PtrLcv_dnn_GroupNormLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::GroupNormLayer>();
	}

	// cv::Ptr<cv::dnn::GroupNormLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::GroupNormLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_GroupNormLayerG_delete(cv::Ptr<cv::dnn::GroupNormLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::GroupNormLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::GroupNormLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_GroupNormLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::GroupNormLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::GroupNormLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::GroupNormLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_GroupNormLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::GroupNormLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::GroupNormLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::GroupNormLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::GroupNormLayer"]), _)]),
	cv::Ptr<cv::dnn::GroupNormLayer>* cv_PtrLcv_dnn_GroupNormLayerG_new_const_GroupNormLayer(cv::dnn::GroupNormLayer* val) {
			return new cv::Ptr<cv::dnn::GroupNormLayer>(val);
	}

}

