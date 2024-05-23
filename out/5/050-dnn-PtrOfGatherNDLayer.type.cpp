extern "C" {
	// cv::Ptr<cv::dnn::GatherNDLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::GatherNDLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::GatherNDLayer* cv_PtrLcv_dnn_GatherNDLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::GatherNDLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::GatherNDLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::GatherNDLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::GatherNDLayer* cv_PtrLcv_dnn_GatherNDLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::GatherNDLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::GatherNDLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::GatherNDLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::GatherNDLayer>* cv_PtrLcv_dnn_GatherNDLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::GatherNDLayer>();
	}

	// cv::Ptr<cv::dnn::GatherNDLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::GatherNDLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_GatherNDLayerG_delete(cv::Ptr<cv::dnn::GatherNDLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::GatherNDLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::GatherNDLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_GatherNDLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::GatherNDLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::GatherNDLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::GatherNDLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_GatherNDLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::GatherNDLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::GatherNDLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::GatherNDLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::GatherNDLayer"]), _)]),
	cv::Ptr<cv::dnn::GatherNDLayer>* cv_PtrLcv_dnn_GatherNDLayerG_new_const_GatherNDLayer(cv::dnn::GatherNDLayer* val) {
			return new cv::Ptr<cv::dnn::GatherNDLayer>(val);
	}

}

