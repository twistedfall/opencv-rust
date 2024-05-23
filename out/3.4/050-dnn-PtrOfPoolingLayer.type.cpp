extern "C" {
	// cv::Ptr<cv::dnn::PoolingLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::PoolingLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::PoolingLayer* cv_PtrLcv_dnn_PoolingLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::PoolingLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::PoolingLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::PoolingLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::PoolingLayer* cv_PtrLcv_dnn_PoolingLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::PoolingLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::PoolingLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::PoolingLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::PoolingLayer>* cv_PtrLcv_dnn_PoolingLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::PoolingLayer>();
	}

	// cv::Ptr<cv::dnn::PoolingLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::PoolingLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_PoolingLayerG_delete(cv::Ptr<cv::dnn::PoolingLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::PoolingLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::PoolingLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_PoolingLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::PoolingLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::PoolingLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::PoolingLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_PoolingLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::PoolingLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::PoolingLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::PoolingLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::PoolingLayer"]), _)]),
	cv::Ptr<cv::dnn::PoolingLayer>* cv_PtrLcv_dnn_PoolingLayerG_new_const_PoolingLayer(cv::dnn::PoolingLayer* val) {
			return new cv::Ptr<cv::dnn::PoolingLayer>(val);
	}

}

