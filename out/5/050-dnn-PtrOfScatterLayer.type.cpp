extern "C" {
	// cv::Ptr<cv::dnn::ScatterLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::ScatterLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::ScatterLayer* cv_PtrLcv_dnn_ScatterLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::ScatterLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::ScatterLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::ScatterLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ScatterLayer* cv_PtrLcv_dnn_ScatterLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::ScatterLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::ScatterLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::ScatterLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::ScatterLayer>* cv_PtrLcv_dnn_ScatterLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::ScatterLayer>();
	}

	// cv::Ptr<cv::dnn::ScatterLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::ScatterLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_ScatterLayerG_delete(cv::Ptr<cv::dnn::ScatterLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::ScatterLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::ScatterLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_ScatterLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::ScatterLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::ScatterLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::ScatterLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_ScatterLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::ScatterLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::ScatterLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::ScatterLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::ScatterLayer"]), _)]),
	cv::Ptr<cv::dnn::ScatterLayer>* cv_PtrLcv_dnn_ScatterLayerG_new_const_ScatterLayer(cv::dnn::ScatterLayer* val) {
			return new cv::Ptr<cv::dnn::ScatterLayer>(val);
	}

}

