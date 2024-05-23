extern "C" {
	// cv::Ptr<cv::dnn::ShuffleChannelLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::ShuffleChannelLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::ShuffleChannelLayer* cv_PtrLcv_dnn_ShuffleChannelLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::ShuffleChannelLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::ShuffleChannelLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::ShuffleChannelLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ShuffleChannelLayer* cv_PtrLcv_dnn_ShuffleChannelLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::ShuffleChannelLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::ShuffleChannelLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::ShuffleChannelLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::ShuffleChannelLayer>* cv_PtrLcv_dnn_ShuffleChannelLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::ShuffleChannelLayer>();
	}

	// cv::Ptr<cv::dnn::ShuffleChannelLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::ShuffleChannelLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_ShuffleChannelLayerG_delete(cv::Ptr<cv::dnn::ShuffleChannelLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::ShuffleChannelLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::ShuffleChannelLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_ShuffleChannelLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::ShuffleChannelLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::ShuffleChannelLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::ShuffleChannelLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_ShuffleChannelLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::ShuffleChannelLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::ShuffleChannelLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::ShuffleChannelLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::ShuffleChannelLayer"]), _)]),
	cv::Ptr<cv::dnn::ShuffleChannelLayer>* cv_PtrLcv_dnn_ShuffleChannelLayerG_new_const_ShuffleChannelLayer(cv::dnn::ShuffleChannelLayer* val) {
			return new cv::Ptr<cv::dnn::ShuffleChannelLayer>(val);
	}

}

