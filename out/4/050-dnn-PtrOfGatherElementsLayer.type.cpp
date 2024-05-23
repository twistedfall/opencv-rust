extern "C" {
	// cv::Ptr<cv::dnn::GatherElementsLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::GatherElementsLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::GatherElementsLayer* cv_PtrLcv_dnn_GatherElementsLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::GatherElementsLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::GatherElementsLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::GatherElementsLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::GatherElementsLayer* cv_PtrLcv_dnn_GatherElementsLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::GatherElementsLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::GatherElementsLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::GatherElementsLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::GatherElementsLayer>* cv_PtrLcv_dnn_GatherElementsLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::GatherElementsLayer>();
	}

	// cv::Ptr<cv::dnn::GatherElementsLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::GatherElementsLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_GatherElementsLayerG_delete(cv::Ptr<cv::dnn::GatherElementsLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::GatherElementsLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::GatherElementsLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_GatherElementsLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::GatherElementsLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::GatherElementsLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::GatherElementsLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_GatherElementsLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::GatherElementsLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::GatherElementsLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::GatherElementsLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::GatherElementsLayer"]), _)]),
	cv::Ptr<cv::dnn::GatherElementsLayer>* cv_PtrLcv_dnn_GatherElementsLayerG_new_const_GatherElementsLayer(cv::dnn::GatherElementsLayer* val) {
			return new cv::Ptr<cv::dnn::GatherElementsLayer>(val);
	}

}

