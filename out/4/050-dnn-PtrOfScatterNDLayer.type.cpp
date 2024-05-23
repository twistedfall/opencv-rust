extern "C" {
	// cv::Ptr<cv::dnn::ScatterNDLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::ScatterNDLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::ScatterNDLayer* cv_PtrLcv_dnn_ScatterNDLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::ScatterNDLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::ScatterNDLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::ScatterNDLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ScatterNDLayer* cv_PtrLcv_dnn_ScatterNDLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::ScatterNDLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::ScatterNDLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::ScatterNDLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::ScatterNDLayer>* cv_PtrLcv_dnn_ScatterNDLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::ScatterNDLayer>();
	}

	// cv::Ptr<cv::dnn::ScatterNDLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::ScatterNDLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_ScatterNDLayerG_delete(cv::Ptr<cv::dnn::ScatterNDLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::ScatterNDLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::ScatterNDLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_ScatterNDLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::ScatterNDLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::ScatterNDLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::ScatterNDLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_ScatterNDLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::ScatterNDLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::ScatterNDLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::ScatterNDLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::ScatterNDLayer"]), _)]),
	cv::Ptr<cv::dnn::ScatterNDLayer>* cv_PtrLcv_dnn_ScatterNDLayerG_new_const_ScatterNDLayer(cv::dnn::ScatterNDLayer* val) {
			return new cv::Ptr<cv::dnn::ScatterNDLayer>(val);
	}

}

