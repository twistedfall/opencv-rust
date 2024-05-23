extern "C" {
	// cv::Ptr<cv::dnn::FloorLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::FloorLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::FloorLayer* cv_PtrLcv_dnn_FloorLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::FloorLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::FloorLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::FloorLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::FloorLayer* cv_PtrLcv_dnn_FloorLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::FloorLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::FloorLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::FloorLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::FloorLayer>* cv_PtrLcv_dnn_FloorLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::FloorLayer>();
	}

	// cv::Ptr<cv::dnn::FloorLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::FloorLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_FloorLayerG_delete(cv::Ptr<cv::dnn::FloorLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::FloorLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::FloorLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_FloorLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::FloorLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::FloorLayer>::to_PtrOfActivationLayer() generated
	// ("cv::Ptr<cv::dnn::FloorLayer>::to_PtrOfActivationLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::ActivationLayer>* cv_PtrLcv_dnn_FloorLayerG_to_PtrOfActivationLayer(cv::Ptr<cv::dnn::FloorLayer>* instance) {
			return new cv::Ptr<cv::dnn::ActivationLayer>(instance->dynamicCast<cv::dnn::ActivationLayer>());
	}

	// cv::Ptr<cv::dnn::FloorLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::FloorLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_FloorLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::FloorLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::FloorLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::FloorLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::FloorLayer"]), _)]),
	cv::Ptr<cv::dnn::FloorLayer>* cv_PtrLcv_dnn_FloorLayerG_new_const_FloorLayer(cv::dnn::FloorLayer* val) {
			return new cv::Ptr<cv::dnn::FloorLayer>(val);
	}

}

