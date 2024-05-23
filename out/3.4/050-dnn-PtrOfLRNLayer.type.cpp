extern "C" {
	// cv::Ptr<cv::dnn::LRNLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::LRNLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::LRNLayer* cv_PtrLcv_dnn_LRNLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::LRNLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::LRNLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::LRNLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::LRNLayer* cv_PtrLcv_dnn_LRNLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::LRNLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::LRNLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::LRNLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::LRNLayer>* cv_PtrLcv_dnn_LRNLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::LRNLayer>();
	}

	// cv::Ptr<cv::dnn::LRNLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::LRNLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_LRNLayerG_delete(cv::Ptr<cv::dnn::LRNLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::LRNLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::LRNLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_LRNLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::LRNLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::LRNLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::LRNLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_LRNLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::LRNLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::LRNLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::LRNLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::LRNLayer"]), _)]),
	cv::Ptr<cv::dnn::LRNLayer>* cv_PtrLcv_dnn_LRNLayerG_new_const_LRNLayer(cv::dnn::LRNLayer* val) {
			return new cv::Ptr<cv::dnn::LRNLayer>(val);
	}

}

