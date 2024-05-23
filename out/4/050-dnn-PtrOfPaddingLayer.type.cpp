extern "C" {
	// cv::Ptr<cv::dnn::PaddingLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::PaddingLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::PaddingLayer* cv_PtrLcv_dnn_PaddingLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::PaddingLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::PaddingLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::PaddingLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::PaddingLayer* cv_PtrLcv_dnn_PaddingLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::PaddingLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::PaddingLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::PaddingLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::PaddingLayer>* cv_PtrLcv_dnn_PaddingLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::PaddingLayer>();
	}

	// cv::Ptr<cv::dnn::PaddingLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::PaddingLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_PaddingLayerG_delete(cv::Ptr<cv::dnn::PaddingLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::PaddingLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::PaddingLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_PaddingLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::PaddingLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::PaddingLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::PaddingLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_PaddingLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::PaddingLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::PaddingLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::PaddingLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::PaddingLayer"]), _)]),
	cv::Ptr<cv::dnn::PaddingLayer>* cv_PtrLcv_dnn_PaddingLayerG_new_const_PaddingLayer(cv::dnn::PaddingLayer* val) {
			return new cv::Ptr<cv::dnn::PaddingLayer>(val);
	}

}

