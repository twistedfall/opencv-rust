extern "C" {
	// cv::Ptr<cv::dnn::SliceLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::SliceLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::SliceLayer* cv_PtrLcv_dnn_SliceLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::SliceLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::SliceLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::SliceLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::SliceLayer* cv_PtrLcv_dnn_SliceLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::SliceLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::SliceLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::SliceLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::SliceLayer>* cv_PtrLcv_dnn_SliceLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::SliceLayer>();
	}

	// cv::Ptr<cv::dnn::SliceLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::SliceLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_SliceLayerG_delete(cv::Ptr<cv::dnn::SliceLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::SliceLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::SliceLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_SliceLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::SliceLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::SliceLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::SliceLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_SliceLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::SliceLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::SliceLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::SliceLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::SliceLayer"]), _)]),
	cv::Ptr<cv::dnn::SliceLayer>* cv_PtrLcv_dnn_SliceLayerG_new_const_SliceLayer(cv::dnn::SliceLayer* val) {
			return new cv::Ptr<cv::dnn::SliceLayer>(val);
	}

}

