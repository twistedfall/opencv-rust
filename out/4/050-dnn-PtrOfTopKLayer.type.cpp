extern "C" {
	// cv::Ptr<cv::dnn::TopKLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::TopKLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::TopKLayer* cv_PtrLcv_dnn_TopKLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::TopKLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::TopKLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::TopKLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::TopKLayer* cv_PtrLcv_dnn_TopKLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::TopKLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::TopKLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::TopKLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::TopKLayer>* cv_PtrLcv_dnn_TopKLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::TopKLayer>();
	}

	// cv::Ptr<cv::dnn::TopKLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::TopKLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_TopKLayerG_delete(cv::Ptr<cv::dnn::TopKLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::TopKLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::TopKLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_TopKLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::TopKLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::TopKLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::TopKLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_TopKLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::TopKLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::TopKLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::TopKLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::TopKLayer"]), _)]),
	cv::Ptr<cv::dnn::TopKLayer>* cv_PtrLcv_dnn_TopKLayerG_new_const_TopKLayer(cv::dnn::TopKLayer* val) {
			return new cv::Ptr<cv::dnn::TopKLayer>(val);
	}

}

