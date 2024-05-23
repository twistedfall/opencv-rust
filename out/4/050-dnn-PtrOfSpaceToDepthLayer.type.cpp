extern "C" {
	// cv::Ptr<cv::dnn::SpaceToDepthLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::SpaceToDepthLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::SpaceToDepthLayer* cv_PtrLcv_dnn_SpaceToDepthLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::SpaceToDepthLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::SpaceToDepthLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::SpaceToDepthLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::SpaceToDepthLayer* cv_PtrLcv_dnn_SpaceToDepthLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::SpaceToDepthLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::SpaceToDepthLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::SpaceToDepthLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::SpaceToDepthLayer>* cv_PtrLcv_dnn_SpaceToDepthLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::SpaceToDepthLayer>();
	}

	// cv::Ptr<cv::dnn::SpaceToDepthLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::SpaceToDepthLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_SpaceToDepthLayerG_delete(cv::Ptr<cv::dnn::SpaceToDepthLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::SpaceToDepthLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::SpaceToDepthLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_SpaceToDepthLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::SpaceToDepthLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::SpaceToDepthLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::SpaceToDepthLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_SpaceToDepthLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::SpaceToDepthLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::SpaceToDepthLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::SpaceToDepthLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::SpaceToDepthLayer"]), _)]),
	cv::Ptr<cv::dnn::SpaceToDepthLayer>* cv_PtrLcv_dnn_SpaceToDepthLayerG_new_const_SpaceToDepthLayer(cv::dnn::SpaceToDepthLayer* val) {
			return new cv::Ptr<cv::dnn::SpaceToDepthLayer>(val);
	}

}

