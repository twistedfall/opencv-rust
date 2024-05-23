extern "C" {
	// cv::Ptr<cv::dnn::FlowWarpLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::FlowWarpLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::FlowWarpLayer* cv_PtrLcv_dnn_FlowWarpLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::FlowWarpLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::FlowWarpLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::FlowWarpLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::FlowWarpLayer* cv_PtrLcv_dnn_FlowWarpLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::FlowWarpLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::FlowWarpLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::FlowWarpLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::FlowWarpLayer>* cv_PtrLcv_dnn_FlowWarpLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::FlowWarpLayer>();
	}

	// cv::Ptr<cv::dnn::FlowWarpLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::FlowWarpLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_FlowWarpLayerG_delete(cv::Ptr<cv::dnn::FlowWarpLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::FlowWarpLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::FlowWarpLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_FlowWarpLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::FlowWarpLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::FlowWarpLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::FlowWarpLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_FlowWarpLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::FlowWarpLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::FlowWarpLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::FlowWarpLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::FlowWarpLayer"]), _)]),
	cv::Ptr<cv::dnn::FlowWarpLayer>* cv_PtrLcv_dnn_FlowWarpLayerG_new_const_FlowWarpLayer(cv::dnn::FlowWarpLayer* val) {
			return new cv::Ptr<cv::dnn::FlowWarpLayer>(val);
	}

}

