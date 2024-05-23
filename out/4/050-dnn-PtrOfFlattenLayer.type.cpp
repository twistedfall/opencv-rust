extern "C" {
	// cv::Ptr<cv::dnn::FlattenLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::FlattenLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::FlattenLayer* cv_PtrLcv_dnn_FlattenLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::FlattenLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::FlattenLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::FlattenLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::FlattenLayer* cv_PtrLcv_dnn_FlattenLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::FlattenLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::FlattenLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::FlattenLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::FlattenLayer>* cv_PtrLcv_dnn_FlattenLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::FlattenLayer>();
	}

	// cv::Ptr<cv::dnn::FlattenLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::FlattenLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_FlattenLayerG_delete(cv::Ptr<cv::dnn::FlattenLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::FlattenLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::FlattenLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_FlattenLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::FlattenLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::FlattenLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::FlattenLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_FlattenLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::FlattenLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::FlattenLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::FlattenLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::FlattenLayer"]), _)]),
	cv::Ptr<cv::dnn::FlattenLayer>* cv_PtrLcv_dnn_FlattenLayerG_new_const_FlattenLayer(cv::dnn::FlattenLayer* val) {
			return new cv::Ptr<cv::dnn::FlattenLayer>(val);
	}

}

