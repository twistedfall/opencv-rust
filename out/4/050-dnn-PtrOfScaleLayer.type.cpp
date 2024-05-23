extern "C" {
	// cv::Ptr<cv::dnn::ScaleLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::ScaleLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::ScaleLayer* cv_PtrLcv_dnn_ScaleLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::ScaleLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::ScaleLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::ScaleLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ScaleLayer* cv_PtrLcv_dnn_ScaleLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::ScaleLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::ScaleLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::ScaleLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::ScaleLayer>* cv_PtrLcv_dnn_ScaleLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::ScaleLayer>();
	}

	// cv::Ptr<cv::dnn::ScaleLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::ScaleLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_ScaleLayerG_delete(cv::Ptr<cv::dnn::ScaleLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::ScaleLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::ScaleLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_ScaleLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::ScaleLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::ScaleLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::ScaleLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_ScaleLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::ScaleLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::ScaleLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::ScaleLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::ScaleLayer"]), _)]),
	cv::Ptr<cv::dnn::ScaleLayer>* cv_PtrLcv_dnn_ScaleLayerG_new_const_ScaleLayer(cv::dnn::ScaleLayer* val) {
			return new cv::Ptr<cv::dnn::ScaleLayer>(val);
	}

}

