extern "C" {
	// cv::Ptr<cv::dnn::ExpandLayer>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::ExpandLayer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::ExpandLayer* cv_PtrLcv_dnn_ExpandLayerG_getInnerPtr_const(const cv::Ptr<cv::dnn::ExpandLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::ExpandLayer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::ExpandLayer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::ExpandLayer* cv_PtrLcv_dnn_ExpandLayerG_getInnerPtrMut(cv::Ptr<cv::dnn::ExpandLayer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::ExpandLayer>::new_null() generated
	// ("cv::Ptr<cv::dnn::ExpandLayer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::ExpandLayer>* cv_PtrLcv_dnn_ExpandLayerG_new_null_const() {
			return new cv::Ptr<cv::dnn::ExpandLayer>();
	}

	// cv::Ptr<cv::dnn::ExpandLayer>::delete() generated
	// ("cv::Ptr<cv::dnn::ExpandLayer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_ExpandLayerG_delete(cv::Ptr<cv::dnn::ExpandLayer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::dnn::ExpandLayer>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::dnn::ExpandLayer>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_dnn_ExpandLayerG_to_PtrOfAlgorithm(cv::Ptr<cv::dnn::ExpandLayer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::dnn::ExpandLayer>::to_PtrOfLayer() generated
	// ("cv::Ptr<cv::dnn::ExpandLayer>::to_PtrOfLayer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::dnn::Layer>* cv_PtrLcv_dnn_ExpandLayerG_to_PtrOfLayer(cv::Ptr<cv::dnn::ExpandLayer>* instance) {
			return new cv::Ptr<cv::dnn::Layer>(instance->dynamicCast<cv::dnn::Layer>());
	}

	// cv::Ptr<cv::dnn::ExpandLayer>::new(TraitClass) generated
	// ("cv::Ptr<cv::dnn::ExpandLayer>::new", vec![(pred!(const, ["val"], ["cv::dnn::ExpandLayer"]), _)]),
	cv::Ptr<cv::dnn::ExpandLayer>* cv_PtrLcv_dnn_ExpandLayerG_new_const_ExpandLayer(cv::dnn::ExpandLayer* val) {
			return new cv::Ptr<cv::dnn::ExpandLayer>(val);
	}

}

