extern "C" {
	// cv::Ptr<cv::dnn::PriorBoxLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::PriorBoxLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::PriorBoxLayer* cv_PtrLcv_dnn_PriorBoxLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::PriorBoxLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::PriorBoxLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::PriorBoxLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::PriorBoxLayer* cv_PtrLcv_dnn_PriorBoxLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::PriorBoxLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::PriorBoxLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::PriorBoxLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::PriorBoxLayer>* cv_PtrLcv_dnn_PriorBoxLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::PriorBoxLayer>();
	}

	// cv::Ptr<cv::dnn::PriorBoxLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::PriorBoxLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_PriorBoxLayerG_delete(cv::Ptr<cv::dnn::PriorBoxLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::PriorBoxLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::PriorBoxLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_PriorBoxLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::PriorBoxLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::PriorBoxLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::PriorBoxLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_PriorBoxLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::PriorBoxLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::PriorBoxLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::PriorBoxLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::PriorBoxLayer"]), _)]),
	cv::Ptr<cv::dnn::PriorBoxLayer>* cv_PtrLcv_dnn_PriorBoxLayerG_new_const_PriorBoxLayer(cv::dnn::PriorBoxLayer* val) {
			return new cv::Ptr<cv::dnn::PriorBoxLayer>(val);
	}

}

