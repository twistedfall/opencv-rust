extern "C" {
	// cv::Ptr<cv::dnn::RangeLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::RangeLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::RangeLayer* cv_PtrLcv_dnn_RangeLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::RangeLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::RangeLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::RangeLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::RangeLayer* cv_PtrLcv_dnn_RangeLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::RangeLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::RangeLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::RangeLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::RangeLayer>* cv_PtrLcv_dnn_RangeLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::RangeLayer>();
	}

	// cv::Ptr<cv::dnn::RangeLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::RangeLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_RangeLayerG_delete(cv::Ptr<cv::dnn::RangeLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::RangeLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::RangeLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_RangeLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::RangeLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::RangeLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::RangeLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_RangeLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::RangeLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::RangeLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::RangeLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::RangeLayer"]), _)]),
	cv::Ptr<cv::dnn::RangeLayer>* cv_PtrLcv_dnn_RangeLayerG_new_const_RangeLayer(cv::dnn::RangeLayer* val) {
			return new cv::Ptr<cv::dnn::RangeLayer>(val);
	}

}

