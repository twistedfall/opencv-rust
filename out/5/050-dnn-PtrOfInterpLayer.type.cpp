extern "C" {
	// cv::Ptr<cv::dnn::InterpLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::InterpLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::InterpLayer* cv_PtrLcv_dnn_InterpLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::InterpLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::InterpLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::InterpLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::InterpLayer* cv_PtrLcv_dnn_InterpLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::InterpLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::InterpLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::InterpLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::InterpLayer>* cv_PtrLcv_dnn_InterpLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::InterpLayer>();
	}

	// cv::Ptr<cv::dnn::InterpLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::InterpLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_InterpLayerG_delete(cv::Ptr<cv::dnn::InterpLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::InterpLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::InterpLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_InterpLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::InterpLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::InterpLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::InterpLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_InterpLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::InterpLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::InterpLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::InterpLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::InterpLayer"]), _)]),
	cv::Ptr<cv::dnn::InterpLayer>* cv_PtrLcv_dnn_InterpLayerG_new_const_InterpLayer(cv::dnn::InterpLayer* val) {
			return new cv::Ptr<cv::dnn::InterpLayer>(val);
	}

}

