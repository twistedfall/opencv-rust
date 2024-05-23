extern "C" {
	// cv::Ptr<cv::dnn::CorrelationLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::CorrelationLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::CorrelationLayer* cv_PtrLcv_dnn_CorrelationLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::CorrelationLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::CorrelationLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::CorrelationLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::CorrelationLayer* cv_PtrLcv_dnn_CorrelationLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::CorrelationLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::CorrelationLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::CorrelationLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::CorrelationLayer>* cv_PtrLcv_dnn_CorrelationLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::CorrelationLayer>();
	}

	// cv::Ptr<cv::dnn::CorrelationLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::CorrelationLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_CorrelationLayerG_delete(cv::Ptr<cv::dnn::CorrelationLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::CorrelationLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::CorrelationLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_CorrelationLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::CorrelationLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::CorrelationLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::CorrelationLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_CorrelationLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::CorrelationLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::CorrelationLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::CorrelationLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::CorrelationLayer"]), _)]),
	cv::Ptr<cv::dnn::CorrelationLayer>* cv_PtrLcv_dnn_CorrelationLayerG_new_const_CorrelationLayer(cv::dnn::CorrelationLayer* val) {
			return new cv::Ptr<cv::dnn::CorrelationLayer>(val);
	}

}

